use std::collections::HashMap;
use std::fmt::Debug;
use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;

use actix_web::http::StatusCode;
use actix_web::rt::time::interval;
use futures::lock::Mutex;
use sea_orm::DatabaseConnection;
use serde::Deserialize;

use crate::client_update::{players_graph, servers, UpdateResponseBody};
use crate::entities;
use crate::error::AppError;
use crate::sender::Broadcaster;
use crate::utils::validate;

const UPDATE_INTERVAL: Duration = Duration::from_secs(6);
type FetchReturn<'a> =
    Pin<Box<dyn Future<Output = Result<UpdateResponseBody, AppError>> + Send + 'a>>;
type FetchFn = fn(&DatabaseConnection) -> FetchReturn;

trait TaskTrait {
    fn run(self: Box<Self>) -> Pin<Box<dyn Future<Output = ()> + Send>>;
}

macro_rules! add_task {
    ($task_manager:expr, $fetch_fn:ident) => {
        $task_manager.add_task::<entities::$fetch_fn::Model>(|conn| {
            Box::pin(async move { $fetch_fn(conn).await })
        });
    };
}

pub fn spawn(broadcaster: Arc<Broadcaster>, conn: Arc<DatabaseConnection>) {
    let mut task_manager = TaskManager::new(broadcaster, conn);
    add_task!(task_manager, players_graph);
    add_task!(task_manager, servers);
    task_manager.start();
}

pub struct TaskManager {
    broadcaster: Arc<Broadcaster>,
    conn: Arc<DatabaseConnection>,
    cache: Arc<Mutex<HashMap<&'static str, serde_json::Value>>>,
    tasks: Vec<Box<dyn TaskTrait>>,
}

impl TaskManager {
    pub fn new(broadcaster: Arc<Broadcaster>, conn: Arc<DatabaseConnection>) -> Self {
        Self {
            broadcaster,
            conn,
            cache: Arc::new(Mutex::new(HashMap::new())),
            tasks: Vec::new(),
        }
    }

    pub fn add_task<T>(&mut self, fetch_fn: FetchFn)
    where
        T: 'static
            + for<'a> Deserialize<'a>
            + PartialEq
            + Debug
            + std::marker::Send
            + std::clone::Clone
            + serde::Serialize,
    {
        let task = Box::new(Task::<T> {
            broadcaster: Arc::clone(&self.broadcaster),
            conn: Arc::clone(&self.conn),
            cache: Arc::clone(&self.cache),
            cache_key: std::any::type_name::<T>(),
            interval: interval(UPDATE_INTERVAL),
            fetch_fn,
            _marker: PhantomData,
        });
        self.tasks.push(task);
    }

    pub fn start(self) {
        for task in self.tasks {
            actix_web::rt::spawn(task.run());
        }
    }
}

pub struct Task<T> {
    broadcaster: Arc<Broadcaster>,
    conn: Arc<DatabaseConnection>,
    cache: Arc<Mutex<HashMap<&'static str, serde_json::Value>>>,
    cache_key: &'static str,
    interval: actix_web::rt::time::Interval,
    fetch_fn: FetchFn,
    _marker: PhantomData<T>,
}

impl<
        T: Debug
            + PartialEq
            + for<'b> serde::Deserialize<'b>
            + std::marker::Send
            + std::clone::Clone
            + serde::Serialize,
    > TaskTrait for Task<T>
{
    fn run(self: Box<Self>) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        let conn = self.conn.clone();
        let cache = self.cache.clone();
        let cache_key = self.cache_key;
        let fetch_fn = self.fetch_fn;
        let broadcaster = self.broadcaster.clone();
        let mut interval = self.interval;

        Box::pin(async move {
            add_to_cache(&conn, Arc::clone(&cache), cache_key, fetch_fn).await;

            loop {
                interval.tick().await;

                check_from_db::<T>(
                    Arc::clone(&broadcaster),
                    &conn,
                    Arc::clone(&cache),
                    cache_key,
                    fetch_fn,
                )
                .await;
            }
        })
    }
}

async fn add_to_cache<'a>(
    conn: &DatabaseConnection,
    cache: Arc<Mutex<HashMap<&'a str, serde_json::Value>>>,
    cache_key: &'a str,
    fetch_fn: (impl Fn(&DatabaseConnection) -> FetchReturn + Send + '_),
) {
    let res = fetch_fn(conn).await.unwrap();
    cache.lock().await.insert(cache_key, res.data.unwrap());
}

async fn check_from_db<'a, T>(
    broadcaster: Arc<Broadcaster>,
    conn: &DatabaseConnection,
    cache: Arc<Mutex<HashMap<&'a str, serde_json::Value>>>,
    cache_key: &'a str,
    fetch_fn: fn(&DatabaseConnection) -> FetchReturn,
) where
    T: for<'b> Deserialize<'b> + PartialEq + Debug + std::clone::Clone + serde::Serialize,
{
    let res = fetch_fn(conn)
        .await
        .unwrap_or_else(|e| UpdateResponseBody::err(&e));

    if let Some(data) = res.data.clone() {
        let data = validate::<Vec<T>>(data).unwrap();
        let cached =
            validate::<Vec<T>>(cache.lock().await.get(cache_key).unwrap().clone()).unwrap();

        let matches = data.iter().zip(&cached).filter(|&(a, b)| a != b);
        if matches.count() > 0 || data.len() != cached.len() {
            cache
                .lock()
                .await
                .insert(cache_key, res.data.clone().unwrap());
            let result_vec: Vec<T> = data
                .iter()
                .filter(|&x| !cached.contains(x))
                .chain(cached.iter().filter(|&x| !data.contains(x)))
                .cloned()
                .collect();
            let new = UpdateResponseBody::new(
                StatusCode::from_u16(res.code).unwrap(),
                &res.message,
                Some(serde_json::to_value(result_vec).unwrap()),
                res.event,
            );
            broadcaster.broadcast(&sjts(&new)).await;
        }
    } else {
        broadcaster.broadcast(&sjts(&res)).await;
    }
}

// serde_json.to_string
fn sjts(v: &UpdateResponseBody) -> String {
    serde_json::to_string(&v).unwrap_or_else(|_| {
        r#"{"code":500,"message":"Failed to serialize message","data":null,"event":"Error"}"#
            .to_owned()
    })
}
