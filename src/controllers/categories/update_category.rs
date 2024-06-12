use actix_web::{error::ErrorNotFound, web, HttpResponse, Responder};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde_json::json;
use std::sync::Arc;

use crate::{entities::categories, error::AppError};

use super::UpdateCategory;

#[utoipa::path(
    put,
    path = "/api/categories",
    tag = "Categories",
    params(
        ("Authentication" = String, Header, description = "JWT access token"),
    ),
    request_body(content = UpdateCategory, description = "Category Data", content_type = "application/json", example = json!({"id": 3, "name": "Vanilla"})),
    responses(
        (status = 200, description = "Successfully updated category", body = None, example = json!({"message": "Success"})),
        (status = 404, description = "Category does not exist", body = None, example = json!({"message": "No such category exist"})),
        (status = 500, description = "Server error"),
    ),
    security(
        ("Authorization" = [])
    )
)]
pub async fn update_category(
    db: web::Data<Arc<DatabaseConnection>>,
    data: web::Json<UpdateCategory>,
) -> Result<impl Responder, AppError> {
    let category = categories::Entity::find()
        .filter(categories::Column::Id.eq(data.id))
        .one(db.get_ref().as_ref())
        .await?;

    if let Some(category) = category {
        let mut new_category: categories::ActiveModel = category.into();
        new_category.name = Set(data.name.clone());
        new_category.update(db.get_ref().as_ref()).await?;

        return Ok(HttpResponse::Ok().json(json!({"message": "Success"})));
    }

    Err(ErrorNotFound("No such category exist").into())
}
