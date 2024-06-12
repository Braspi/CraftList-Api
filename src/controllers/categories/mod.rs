use actix_web::web::{self, ServiceConfig};
use actix_web_lab::middleware::from_fn;
use serde::Deserialize;
use utoipa::ToSchema;

use crate::utils::admin_auth_middleware;

pub mod add_category;
pub mod list_categories;
pub mod remove_category;
pub mod update_category;

#[derive(Deserialize, ToSchema)]
pub struct Category {
    name: String,
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateCategory {
    id: i32,
    name: String,
}

#[derive(Deserialize, ToSchema)]
pub struct DeleteCategory {
    id: Option<i32>,
    name: String,
}

pub fn configure() -> impl FnOnce(&mut ServiceConfig) {
    |config: &mut ServiceConfig| {
        config.service(
            web::resource("/categories")
                .route(
                    // NOTE: .wrap() must be in the end
                    web::post()
                        .to(add_category::add_category)
                        .wrap(from_fn(admin_auth_middleware)),
                )
                .route(
                    web::delete()
                        .to(remove_category::remove_category)
                        .wrap(from_fn(admin_auth_middleware)),
                )
                .route(
                    web::put()
                        .to(update_category::update_category)
                        .wrap(from_fn(admin_auth_middleware)),
                )
                .get(list_categories::list_categories),
        );
    }
}
