use actix_web::{error::ErrorNotFound, web, HttpResponse, Responder};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde_json::json;
use std::sync::Arc;

use crate::{entities::versions, error::AppError};

use super::UpdateVersion;

#[utoipa::path(
    put,
    path = "/api/versions",
    tag = "versions",
    params(
        ("Authentication" = String, Header, description = "JWT access token"),
    ),
    request_body(content = UpdateVersion, description = "Version Data", content_type = "application/json", example = json!({"id": 3, "name": "1.8", "protocol": 47})),
    responses(
        (status = 200, description = "Successfully updated version", body = None, example = json!({"message": "Success"})),
        (status = 404, description = "Version does not exist", body = None, example = json!({"message": "No such version exist"})),
        (status = 500, description = "Server error"),
    ),
    security(
        ("Authorization" = [])
    )
)]
pub async fn update_version(
    db: web::Data<Arc<DatabaseConnection>>,
    data: web::Json<UpdateVersion>,
) -> Result<impl Responder, AppError> {
    let version = versions::Entity::find()
        .filter(versions::Column::Id.eq(data.id))
        .one(db.get_ref().as_ref())
        .await?;

    if let Some(version) = version {
        let mut new_version: versions::ActiveModel = version.into();
        if let Some(name) = &data.name {
            new_version.name = Set(name.to_owned());
        }
        if let Some(protocol) = data.protocol {
            new_version.protocol = Set(protocol);
        }
        new_version.update(db.get_ref().as_ref()).await?;

        return Ok(HttpResponse::Ok().json(json!({"message": "Success"})));
    }

    Err(ErrorNotFound("No such version exist").into())
}
