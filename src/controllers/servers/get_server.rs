use actix_web::{error::ErrorNotFound, web, HttpResponse, Responder};
use sea_orm::{ColumnTrait, DatabaseConnection, QueryFilter};
use serde_json::json;
use std::sync::Arc;

use crate::{entities::servers, error::AppError};

use super::{utils, Server};

#[utoipa::path(
    get,
    path = "/api/servers/{id}",
    tag = "Servers",
    params(
        ("id" = i32, Path, description = "Id of the server")
    ),
    responses(
        (status = 200, description = "Server object", body = Server),
        (status = 500, description = "Server error"),
    ),
)]
pub async fn get_server(
    db: web::Data<Arc<DatabaseConnection>>,
    body: web::Path<i32>,
) -> Result<impl Responder, AppError> {
    let server = utils::get_server()
        .filter(servers::Column::Id.eq(body.into_inner()))
        .into_json()
        .one(db.get_ref().as_ref())
        .await?;

    if let Some(server) = server {
        return Ok(HttpResponse::Ok().json(json! {serde_json::from_value::<Server>(server)?}));
    }

    Err(ErrorNotFound("No such server exists").into())
}
