use actix_web::{web, HttpRequest, HttpResponse, Responder};
use sea_orm::DatabaseConnection;
use serde_json::json;
use std::sync::Arc;

use crate::{
    error::AppError,
    utils::{create_access_token, validate_refresh_token},
    Config,
};

#[utoipa::path(
    get,
    path = "/auth/refresh",
    tag = "Auth",
    responses(
        (status = 200, description = "JwtToken", body = LoginResponse),
        (status = 401, description = "Invalid refresh token provided"),
        (status = 500, description = "Database error"),
    )
)]
pub async fn refresh(
    db: web::Data<Arc<DatabaseConnection>>,
    config: web::Data<Config>,
    req: HttpRequest,
) -> Result<impl Responder, AppError> {
    let cookie = match req.cookie("refresh_token") {
        Some(v) => v,
        None => return Err(actix_web::error::ErrorUnauthorized("Missing refresh token").into()),
    };

    let claims =
        match validate_refresh_token(&db, cookie.value(), config.json_token.as_bytes()).await {
            Ok(v) => v,
            Err(_) => return Err(actix_web::error::ErrorUnauthorized("Invalid token").into()),
        };

    let token = create_access_token(claims.sub, config.json_token.as_bytes());

    Ok(HttpResponse::Ok().json(json!({"jwt_token": token})))
}
