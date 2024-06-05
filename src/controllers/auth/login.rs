use actix_web::{cookie::Cookie, web, Error, HttpRequest, HttpResponse, Responder};
use argon2::{self, Argon2, PasswordHash, PasswordVerifier};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use std::sync::Arc;

use crate::{entities::users, utils::create_token, Config};

use super::LoginRequest;

pub async fn login(
    db: web::Data<Arc<DatabaseConnection>>,
    config: web::Data<Config>,
    data: web::Json<LoginRequest>,
    req: HttpRequest,
) -> Result<impl Responder, Error> {
    let argon2 = Argon2::default();

    let user = users::Entity::find()
        .filter(users::Column::Email.eq(data.email.clone()))
        .one(db.get_ref().as_ref())
        .await
        .unwrap();

    if let Some(user) = user {
        if argon2
            .verify_password(
                data.password.as_bytes(),
                &PasswordHash::new(&user.password).unwrap(),
            )
            .is_ok()
        {
            let token = create_token(db.get_ref(), user.id, config.json_token.as_bytes()).await;
            let cookie = Cookie::build("jwt_token", token)
                .domain(req.uri().host().unwrap_or(""))
                .path("/")
                .finish();
            return Ok(HttpResponse::Ok().cookie(cookie).finish());
        }
    }

    Ok(HttpResponse::Unauthorized().finish())
}
