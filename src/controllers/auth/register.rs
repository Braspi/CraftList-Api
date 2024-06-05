use actix_web::{web, HttpResponse, Responder};
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use std::{ops::Deref, sync::Arc};

use crate::{entities::users, error::AppError};

use super::RegisterRequest;

pub async fn register(
    db: web::Data<Arc<DatabaseConnection>>,
    req: web::Json<RegisterRequest>,
) -> Result<impl Responder, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(req.password.as_bytes(), &salt)
        .unwrap();

    let new_user = users::ActiveModel {
        email: Set(req.email.clone()),
        username: Set(req.username.clone()),
        password: Set(password_hash.to_string()),
        ..Default::default()
    };

    new_user.insert(db.get_ref().deref()).await?;

    Ok(HttpResponse::Created().finish())
}
