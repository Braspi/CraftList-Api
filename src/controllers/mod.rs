use actix_web::web::{self, ServiceConfig};

pub mod auth;

pub fn configure() -> impl FnOnce(&mut ServiceConfig) {
    |config: &mut ServiceConfig| {
        config.service(web::scope("/auth").configure(auth::configure()));
    }
}
