use actix_web::{web, HttpResponse, Responder};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection, EntityTrait};
use serde_json::json;
use std::sync::Arc;

use crate::{entities::categories, error::AppError};

use super::Category;

#[utoipa::path(
    post,
    path = "/api/categories",
    tag = "Categories",
    request_body(content = Category, description = "Category Data", content_type = "application/json", example = json!({"name": "Vanilla"})),
    responses(
        (status = 201, description = "Created new category", body = None, example = json!({"message": "Success", "id": 3})),
        (status = 409, description = "Category already exists", body = None, example = json!({"message": "Category already exists"})),
        (status = 500, description = "Server error"),
    ),
    security(
        ("Authorization" = [])
    )
)]
pub async fn add_category(
    db: web::Data<Arc<DatabaseConnection>>,
    data: web::Json<Category>,
) -> Result<impl Responder, AppError> {
    let categories: Vec<String> = categories::Entity::find()
        .all(db.get_ref().as_ref())
        .await?
        .iter()
        .map(|v| v.name.to_lowercase())
        .collect();

    if categories.contains(&data.name.to_lowercase()) {
        return Ok(HttpResponse::Conflict().json(json!({"message": "Category already exists"})));
    }

    let model = categories::ActiveModel {
        name: Set(data.name.clone()),
        ..Default::default()
    };

    let model_i = model.insert(db.get_ref().as_ref()).await?;
    let last_id = model_i.id;

    Ok(HttpResponse::Created().json(json!({"message": "Success", "id": last_id})))
}
