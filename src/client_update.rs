use crate::entities;
use actix_web::http::StatusCode;
use sea_orm::{DatabaseConnection, EntityTrait};
use serde::Serialize;
use serde_json::json;
use serde_json::Value;

use crate::error::AppError;

#[derive(Serialize, Debug)]
pub enum UpdateEventType {
    PlayersGraph,
    Servers,

    Error,
}

#[derive(Serialize, Debug)]
pub struct UpdateResponseBody {
    pub code: u16,
    pub message: String,
    pub data: Option<Value>,
    pub event: UpdateEventType,
}

impl UpdateResponseBody {
    pub fn new(
        code: StatusCode,
        message: &str,
        data: Option<Value>,
        event: UpdateEventType,
    ) -> Self {
        Self {
            code: code.as_u16(),
            message: message.to_owned(),
            data,
            event,
        }
    }

    pub fn err(e: &AppError) -> Self {
        Self {
            code: e.status_code().as_u16(),
            message: e.to_string(),
            data: None,
            event: UpdateEventType::Error,
        }
    }
}

macro_rules! s {
    ($name:ident, $type:expr) => {
        pub async fn $name(conn: &DatabaseConnection) -> Result<UpdateResponseBody, AppError> {
            let $name = entities::$name::Entity::find().all(conn).await?;

            Ok(UpdateResponseBody::new(
                StatusCode::OK,
                "Ok",
                Some(json! {$name}),
                $type,
            ))
        }
    };
}

s!(servers, UpdateEventType::Servers);
s!(players_graph, UpdateEventType::PlayersGraph);
