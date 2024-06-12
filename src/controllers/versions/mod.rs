use actix_web::web::{self, ServiceConfig};
use actix_web_lab::middleware::from_fn;
use serde::Deserialize;
use utoipa::ToSchema;

use crate::utils::admin_auth_middleware;

pub mod add_version;
pub mod list_versions;
pub mod remove_version;
pub mod update_version;

#[derive(Deserialize, ToSchema)]
pub struct Version {
    name: String,
    protocol: i32,
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateVersion {
    id: i32,
    name: Option<String>,
    protocol: Option<i32>,
}

#[derive(Deserialize, ToSchema)]
pub struct DeleteVersion {
    id: Option<i32>,
    name: String,
}

pub fn configure() -> impl FnOnce(&mut ServiceConfig) {
    |config: &mut ServiceConfig| {
        config.service(
            web::resource("/versions")
                .route(
                    web::post()
                        .to(add_version::add_version)
                        .wrap(from_fn(admin_auth_middleware)),
                )
                .route(
                    web::delete()
                        .to(remove_version::remove_version)
                        .wrap(from_fn(admin_auth_middleware)),
                )
                .route(
                    web::put()
                        .to(update_version::update_version)
                        .wrap(from_fn(admin_auth_middleware)),
                )
                .get(list_versions::list_versions),
        );
    }
}
