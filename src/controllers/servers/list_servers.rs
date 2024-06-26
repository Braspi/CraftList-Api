use actix_web::{web, HttpResponse, Responder};
use sea_orm::DatabaseConnection;
use serde_json::json;
use std::sync::Arc;

use crate::error::AppError;

use super::utils;

#[utoipa::path(
    get,
    path = "/api/servers",
    tag = "Servers",
    responses(
        (status = 200, description = "Server object", body = Vec<Server>),
        (status = 500, description = "Server error"),
    ),
)]
pub async fn list_servers(
    db: web::Data<Arc<DatabaseConnection>>,
) -> Result<impl Responder, AppError> {
    let servers = utils::get_server()
        .into_json()
        .all(db.get_ref().as_ref())
        .await?;

    Ok(HttpResponse::Ok().json(json! {servers}))
}
