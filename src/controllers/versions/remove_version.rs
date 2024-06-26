use actix_web::{error::ErrorNotFound, web, HttpResponse, Responder};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter};
use serde_json::json;
use std::sync::Arc;

use crate::{entities::versions, error::AppError};

use super::DeleteVersion;

#[utoipa::path(
    delete,
    path = "/api/versions",
    tag = "Versions",
    params(
        ("Authentication" = String, Header, description = "JWT access token"),
    ),
    request_body(content = DeleteVersion, description = "Version Data", content_type = "application/json", examples(
        ("Full" = (value = json!({"id": 3, "name": "1.8"}))),
        ("No Id" = (value = json!({"name": "1.8"})))
    )),
    responses(
        (status = 200, description = "Successfully deleted version", body = None, example = json!({"message": "Success"})),
        (status = 404, description = "version does not exist", body = None, example = json!({"message": "No such version exist"})),
        (status = 500, description = "Server error"),
    ),
    security(
        ("Authorization" = [])
    )
)]
pub async fn remove_version(
    db: web::Data<Arc<DatabaseConnection>>,
    data: web::Json<DeleteVersion>,
) -> Result<impl Responder, AppError> {
    let version = if let Some(id) = data.id {
        versions::Entity::find().filter(versions::Column::Id.eq(id))
    } else {
        versions::Entity::find().filter(versions::Column::Name.eq(&data.name))
    }
    .one(db.get_ref().as_ref())
    .await?;

    if let Some(version) = version {
        version.delete(db.get_ref().as_ref()).await?;
        return Ok(HttpResponse::Ok().json(json!({"message": "Success"})));
    }

    Err(ErrorNotFound("No such version exist").into())
}
