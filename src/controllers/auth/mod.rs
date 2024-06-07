use actix_web::web::{self, ServiceConfig};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub mod login;
pub mod refresh;
pub mod register;

#[derive(Deserialize, ToSchema)]
pub struct RegisterRequest {
    email: String,
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct LoginResponse {
    jwt_token: String,
}

pub fn configure() -> impl FnOnce(&mut ServiceConfig) {
    |config: &mut ServiceConfig| {
        config
            .service(web::resource("/register").route(web::post().to(register::register)))
            .service(web::resource("/login").route(web::post().to(login::login)))
            .service(web::resource("/refresh").route(web::get().to(refresh::refresh)));
    }
}
