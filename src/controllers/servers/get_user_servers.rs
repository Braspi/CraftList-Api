use actix_web::{web, HttpResponse, Responder};
use sea_orm::{ColumnTrait, DatabaseConnection, QueryFilter};
use serde_json::{json, Value};
use std::sync::Arc;

use crate::{entities::servers, error::AppError};

use super::{utils, Server};

#[utoipa::path(
    get,
    path = "/api/servers/user/{id}",
    tag = "Servers",
    params(
        ("id" = i32, Path, description = "Id of the user")
    ),
    responses(
        (status = 200, description = "Server object", body = Vec<Server>),
        (status = 500, description = "Server error"),
    ),
)]
pub async fn get_user_servers(
    db: web::Data<Arc<DatabaseConnection>>,
    body: web::Path<i32>,
) -> Result<impl Responder, AppError> {
    let servers = utils::get_server()
        .filter(servers::Column::UserId.eq(body.into_inner()))
        .into_json()
        .all(db.get_ref().as_ref())
        .await?;

    Ok(HttpResponse::Ok()
        .json(json! {serde_json::from_value::<Vec<Server>>(Value::Array(servers))?}))
}
