use actix_web::web::{self, ServiceConfig};

pub mod auth;
pub mod categories;
pub mod servers;
pub mod versions;

pub fn configure() -> impl FnOnce(&mut ServiceConfig) {
    |config: &mut ServiceConfig| {
        config.service(web::scope("/auth").configure(auth::configure()));
        config.service(
            web::scope("/api")
                .configure(servers::configure())
                .configure(categories::configure())
                .configure(versions::configure()),
        );
    }
}
