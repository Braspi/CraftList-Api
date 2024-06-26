mod client_update;
mod controllers;
mod docs;
mod entities;
mod error;
mod sender;
mod tasks;
mod utils;

use std::{fs::File, io::Read, sync::Arc};

use actix_cors::Cors;
use actix_web::{
    http::header,
    middleware,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};
use docs::ApiDoc;
use error::AppError;
use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, Database};
use sender::Broadcaster;
use serde::Deserialize;
use tasks::spawn;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(Deserialize, Clone)]
struct Config {
    addr: String,
    port: u16,
    threads: usize,
    database_table: String,
    database_url: String,
    log: u32,
    json_token: String,
}

fn load_config() -> Config {
    let mut file = File::open("config.json").unwrap();
    let mut str = String::new();
    file.read_to_string(&mut str).unwrap();
    serde_json::from_str(&str).unwrap()
}

#[actix_web::main]
async fn main() -> Result<(), AppError> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let config: Config = load_config();

    let mut opt = ConnectOptions::new(format!("{}/{}", config.database_url, config.database_table));
    opt.sqlx_logging_level(log::LevelFilter::Debug);
    let conn = Arc::new(Database::connect(opt).await.unwrap());

    Migrator::up(&*conn, None).await.unwrap();

    let broadcaster = Broadcaster::create();

    // Spawn Tasks
    let broadcaster_clone = Arc::clone(&broadcaster);
    spawn(broadcaster_clone, Arc::clone(&conn));

    let openapi = ApiDoc::openapi();

    let config_clone = config.clone();
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors)
            .app_data(Data::new(Arc::clone(&conn)))
            .app_data(Data::new(config_clone.clone()))
            .app_data(Data::new(Arc::clone(&broadcaster)))
            .wrap(middleware::Logger::default().log_target("CraftList"))
            .configure(controllers::configure())
            .route("/events", web::get().to(sse_client))
            .route("/send", web::get().to(send))
            .route(
                "/docs",
                web::get().to(|| async {
                    HttpResponse::Found()
                        .insert_header((header::LOCATION, "/docs/"))
                        .finish()
                }),
            )
            .service(SwaggerUi::new("/docs/{_:.*}").url("/api-doc/openapi.json", openapi.clone()))
    })
    .workers(config.threads)
    .bind((config.addr, config.port))?
    .run()
    .await?;

    Ok(())
}

pub async fn sse_client(broadcaster: web::Data<Arc<Broadcaster>>) -> impl Responder {
    broadcaster.new_client().await
}

pub async fn send(broadcaster: web::Data<Arc<Broadcaster>>) -> impl Responder {
    broadcaster.broadcast("Hello").await;
    HttpResponse::Ok().finish()
}
