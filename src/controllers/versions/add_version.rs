use actix_web::{error::ErrorConflict, web, HttpResponse, Responder};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection, EntityTrait};
use serde_json::json;
use std::sync::Arc;

use crate::{entities::versions, error::AppError};

use super::Version;

#[utoipa::path(
    post,
    path = "/api/versions",
    tag = "Versions",
    params(
        ("Authentication" = String, Header, description = "JWT access token"),
    ),
    request_body(content = Version, description = "Version Data", content_type = "application/json", example = json!({"name": "1.8", "protocol": 47})),
    responses(
        (status = 201, description = "Created new version", body = None, example = json!({"message": "Success", "id": 3})),
        (status = 409, description = "Version already exists", body = None, example = json!({"message": "Version already exists"})),
        (status = 500, description = "Server error"),
    ),
    security(
        ("Authorization" = [])
    )
)]
pub async fn add_version(
    db: web::Data<Arc<DatabaseConnection>>,
    data: web::Json<Version>,
) -> Result<impl Responder, AppError> {
    let versions: Vec<String> = versions::Entity::find()
        .all(db.get_ref().as_ref())
        .await?
        .iter()
        .map(|v| v.name.to_lowercase())
        .collect();

    if versions.contains(&data.name.to_lowercase()) {
        return Err(ErrorConflict("Version already exists").into());
    }

    let model = versions::ActiveModel {
        name: Set(data.name.clone()),
        protocol: Set(data.protocol),
        ..Default::default()
    };

    let model_i = model.insert(db.get_ref().as_ref()).await?;
    let last_id = model_i.id;

    Ok(HttpResponse::Created().json(json!({"message": "Success", "id": last_id})))
}
