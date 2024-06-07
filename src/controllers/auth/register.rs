use actix_web::{web, HttpResponse, Responder};
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use std::{ops::Deref, sync::Arc};

use crate::{entities::users, error::AppError};

use super::RegisterRequest;

// TODO: Validate if user already exists
#[utoipa::path(
    post,
    path = "/auth/register",
    tag = "Auth",
    request_body(content = RegisterRequest, description = "Credentials data", content_type = "application/json"),
    responses(
        (status = 200, description = "Successfully registered"),
        (status = 401, description = "Invalid refresh_token provided"),
        (status = 500, description = "Database error"),
    )
)]
pub async fn register(
    db: web::Data<Arc<DatabaseConnection>>,
    data: web::Json<RegisterRequest>,
) -> Result<impl Responder, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(data.password.as_bytes(), &salt)
        .unwrap();

    let new_user = users::ActiveModel {
        email: Set(data.email.clone()),
        username: Set(data.username.clone()),
        password: Set(password_hash.to_string()),
        ..Default::default()
    };

    new_user.insert(db.get_ref().deref()).await?;

    Ok(HttpResponse::Created().finish())
}
