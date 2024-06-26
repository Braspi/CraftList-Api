use actix_web::{
    cookie::Cookie, error::ErrorUnauthorized, web, HttpRequest, HttpResponse, Responder,
};
use argon2::{self, Argon2, PasswordHash, PasswordVerifier};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use std::sync::Arc;

use crate::{
    entities::users,
    error::AppError,
    utils::{create_access_token, create_refresh_token, validate_refresh_token},
    Config,
};

use super::{LoginRequest, LoginResponse};

#[utoipa::path(
    post,
    path = "/auth/login",
    tag = "Auth",
    request_body(content = LoginRequest, description = "Credentials data", content_type = "application/json"),
    responses(
        (status = 200, description = "Successfully logged in", body = LoginResponse),
        (status = 500, description = "Database error"),
    ),
    security(
        ("Authorization" = [])
    )
)]
pub async fn login(
    db: web::Data<Arc<DatabaseConnection>>,
    config: web::Data<Config>,
    data: web::Json<LoginRequest>,
    req: HttpRequest,
) -> Result<impl Responder, AppError> {
    let user = users::Entity::find()
        .filter(users::Column::Email.eq(data.email.clone()))
        .one(db.get_ref().as_ref())
        .await?;

    if let Some(user) = user {
        let argon2 = Argon2::default();
        if argon2
            .verify_password(
                data.password.as_bytes(),
                &PasswordHash::new(&user.password).unwrap(),
            )
            .is_ok()
        {
            let cookie = req.cookie("refresh_token").map(|v| v.value().to_owned());
            let token = create_access_token(user.id, config.json_token.as_bytes());
            let refresh_token = if cookie.is_some()
                && validate_refresh_token(
                    &db,
                    cookie.clone().unwrap().as_str(),
                    config.json_token.as_bytes(),
                )
                .await
                .is_ok()
            {
                cookie.unwrap()
            } else {
                create_refresh_token(db.get_ref(), user.id, config.json_token.as_bytes()).await
            };

            let cookie = Cookie::build("refresh_token", refresh_token)
                // TODO: Add valid domain url to unwrap
                .domain(req.uri().host().unwrap_or(""))
                .path("/")
                .http_only(true)
                .finish();

            return Ok(HttpResponse::Ok()
                .cookie(cookie)
                .json(LoginResponse { jwt_token: token }));
        }
    }

    Err(ErrorUnauthorized("Invalid Credentials").into())
}
