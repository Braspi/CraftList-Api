use actix_web::web::{self, ServiceConfig};
use serde::{Deserialize, Serialize};

mod login;
mod register;

#[derive(Deserialize)]
struct RegisterRequest {
    email: String,
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

pub fn configure() -> impl FnOnce(&mut ServiceConfig) {
    |config: &mut ServiceConfig| {
        config
            .service(web::resource("/register").route(web::post().to(register::register)))
            .service(web::resource("/login").route(web::post().to(login::login)));
    }
}
