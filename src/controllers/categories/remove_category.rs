use actix_web::{error::ErrorNotFound, web, HttpResponse, Responder};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter};
use serde_json::json;
use std::sync::Arc;

use crate::{entities::categories, error::AppError};

use super::DeleteCategory;

#[utoipa::path(
    delete,
    path = "/api/categories",
    tag = "Categories",
    params(
        ("Authentication" = String, Header, description = "JWT access token"),
    ),
    request_body(content = DeleteCategory, description = "Category Data", content_type = "application/json", examples(
        ("Full" = (value = json!({"id": 3, "name": "Vanilla"}))),
        ("No Id" = (value = json!({"name": "Vanilla"})))
    )),
    responses(
        (status = 200, description = "Successfully deleted category", body = None, example = json!({"message": "Success"})),
        (status = 404, description = "Category does not exist", body = None, example = json!({"message": "No such category exist"})),
        (status = 500, description = "Server error"),
    ),
    security(
        ("Authorization" = [])
    )
)]
pub async fn remove_category(
    db: web::Data<Arc<DatabaseConnection>>,
    data: web::Json<DeleteCategory>,
) -> Result<impl Responder, AppError> {
    let category = if let Some(id) = data.id {
        categories::Entity::find().filter(categories::Column::Id.eq(id))
    } else {
        categories::Entity::find().filter(categories::Column::Name.eq(&data.name))
    }
    .one(db.get_ref().as_ref())
    .await?;

    if let Some(category) = category {
        category.delete(db.get_ref().as_ref()).await?;
        return Ok(HttpResponse::Ok().json(json!({"message": "Success"})));
    }

    Err(ErrorNotFound("No such category exist").into())
}
