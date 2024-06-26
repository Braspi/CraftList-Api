use actix_web::web::{self, ServiceConfig};
use actix_web_lab::middleware::from_fn;
use serde::{Deserialize, Serialize, Serializer};
use utoipa::ToSchema;

use crate::utils::auth_middleware;

pub mod add_server;
pub mod get_server;
pub mod get_user_servers;
pub mod list_servers;
mod utils;

#[derive(Deserialize, ToSchema)]
pub struct ServerData {
    name: String,
    description: String,
    address: String,
    port: u16,
    categories: Vec<String>,
    min_version: String,
    max_version: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Category {
    id: i32,
    name: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Server {
    id: i32,
    address: String,
    name: String,
    min_version: String,
    max_version: String,
    #[serde(serialize_with = "int_to_bool")]
    is_premium: i32,
    user_id: i32,
    description: String,
    created_at: String,
    categories: Vec<Category>,
}

fn int_to_bool<S>(value: &i32, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_bool(*value != 0)
}

pub fn configure() -> impl FnOnce(&mut ServiceConfig) {
    |config: &mut ServiceConfig| {
        config
            .service(
                web::resource("/servers")
                    .route(
                        web::post()
                            .to(add_server::add_server)
                            .wrap(from_fn(auth_middleware)),
                    )
                    .get(list_servers::list_servers),
            )
            .service(web::resource("/servers/{id}").get(get_server::get_server))
            .service(web::resource("/servers/user/{id}").get(get_user_servers::get_user_servers));
    }
}
