use actix_web::{web, HttpResponse, Responder};
use sea_orm::{DatabaseConnection, EntityTrait};
use serde_json::json;
use std::sync::Arc;

use crate::{entities::categories, error::AppError};

#[utoipa::path(
    get,
    path = "/api/categories",
    tag = "Categories",
    responses(
        (status = 200, description = "List of categories", body = Vec<crate::entities::categories::Model>, example = json!([{"id": 1, "name": "Survival"}, {"id": 3, "name": "Vanilla"}])),
        (status = 500, description = "Server error"),
    )
)]
pub async fn list_categories(
    db: web::Data<Arc<DatabaseConnection>>,
) -> Result<impl Responder, AppError> {
    let categories = categories::Entity::find()
        .all(db.get_ref().as_ref())
        .await?;

    Ok(HttpResponse::Ok().json(json! {categories}))
}
