use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    web,
};
use actix_web_lab::middleware::Next;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{entities::auth, error::AppError, Config};

pub fn validate<T>(body: Value) -> Result<T, AppError>
where
    T: for<'a> Deserialize<'a>,
{
    let data: Result<T, serde_json::Error> = serde_json::from_value(body);
    let req: T = match data {
        Ok(v) => v,
        Err(e) => {
            return Err(AppError::SerdeError(e));
        }
    };
    Ok(req)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: i32,
    exp: i64,
}

pub async fn create_token(db: &DatabaseConnection, user_id: i32, secret_key: &[u8]) -> String {
    let now = Utc::now().naive_utc();
    let expiration = now
        .checked_add_signed(Duration::hours(1))
        .expect("valid timestamp");

    let claims = Claims {
        sub: user_id,
        exp: expiration.and_utc().timestamp(),
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret_key),
    )
    .expect("token creation failed");

    let new_token = auth::ActiveModel {
        token: Set(token.clone()),
        user_id: Set(user_id),
        created_at: Set(Some(now)),
        expires_at: Set(expiration),
        ..Default::default()
    };

    new_token.insert(db).await.unwrap();

    token
}

pub async fn validate_token(
    // db: &DatabaseConnection,
    token: &str,
    secret_key: &[u8],
) -> Result<Claims, jsonwebtoken::errors::Error> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret_key),
        &Validation::new(Algorithm::HS256),
    )?;
    let claims = token_data.claims;

    if claims.exp < Utc::now().timestamp() {
        return Err(jsonwebtoken::errors::ErrorKind::ExpiredSignature.into());
    }

    // let stored_token = auth::Entity::find()
    //     .filter(auth::Column::Token.eq(token))
    //     .filter(auth::Column::ExpiresAt.gt(Utc::now().naive_utc()))
    //     .one(db)
    //     .await
    //     .unwrap();

    // match stored_token {
    //     Some(_) => Ok(claims),
    //     None => Err(jsonwebtoken::errors::ErrorKind::InvalidToken.into()),
    // }
    Ok(claims)
}

pub async fn auth_middleware(
    // db: web::Data<Arc<DatabaseConnection>>,
    config: web::Data<Config>,
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
    let cookie = match req.cookie("jwt_token") {
        Some(v) => v,
        None => return Err(actix_web::error::ErrorUnauthorized("")),
    };

    match validate_token(
        /* db.get_ref(), */ cookie.value(),
        config.json_token.as_bytes(),
    )
    .await
    {
        Ok(_) => next.call(req).await,
        Err(_) => Err(actix_web::error::ErrorUnauthorized("")),
    }
}
