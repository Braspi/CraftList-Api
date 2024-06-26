use actix_web::{web, HttpResponse, Responder};
use sea_orm::{DatabaseConnection, EntityTrait};
use serde_json::json;
use std::sync::Arc;

use crate::{entities::versions, error::AppError};

#[utoipa::path(
    get,
    path = "/api/versions",
    tag = "Versions",
    responses(
        (status = 200, description = "List of versions", body = Vec<crate::entities::versions::Model>, example = json!([{"id": 1, "name": "1.7", "protocol": 3}, {"id": 3, "name": "1.8", "protocol": 47}])),
        (status = 500, description = "Server error"),
    )
)]
pub async fn list_versions(
    db: web::Data<Arc<DatabaseConnection>>,
) -> Result<impl Responder, AppError> {
    let versions = versions::Entity::find().all(db.get_ref().as_ref()).await?;

    Ok(HttpResponse::Ok().json(json! {versions}))
}
