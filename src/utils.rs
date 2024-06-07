use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    web,
};
use actix_web_lab::middleware::Next;
use chrono::{Duration, NaiveDateTime, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
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
    pub sub: i32,
    exp: i64,
}

fn create_token(user_id: i32, expiration: NaiveDateTime, secret_key: &[u8]) -> String {
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

    token
}

pub fn create_access_token(user_id: i32, secret_key: &[u8]) -> String {
    let now = Utc::now().naive_utc();
    let expiration = now
        .checked_add_signed(Duration::minutes(10))
        .expect("valid timestamp");

    create_token(user_id, expiration, secret_key)
}

pub async fn create_refresh_token(
    db: &DatabaseConnection,
    user_id: i32,
    secret_key: &[u8],
) -> String {
    let now = Utc::now().naive_utc();
    let expiration = now
        .checked_add_signed(Duration::days(1))
        .expect("valid timestamp");

    let token = create_token(user_id, expiration, secret_key);

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

pub async fn validate_refresh_token(
    db: &DatabaseConnection,
    refresh_token: &str,
    secret_key: &[u8],
) -> Result<Claims, jsonwebtoken::errors::Error> {
    let token_data = decode::<Claims>(
        refresh_token,
        &DecodingKey::from_secret(secret_key),
        &Validation::new(Algorithm::HS256),
    )?;
    let claims = token_data.claims;

    if claims.exp < Utc::now().timestamp() {
        return Err(jsonwebtoken::errors::ErrorKind::ExpiredSignature.into());
    }

    let stored_token = auth::Entity::find()
        .filter(auth::Column::Token.eq(refresh_token))
        .filter(auth::Column::ExpiresAt.gt(Utc::now().naive_utc()))
        .one(db)
        .await
        .unwrap();

    match stored_token {
        Some(_) => Ok(claims),
        None => Err(jsonwebtoken::errors::ErrorKind::InvalidToken.into()),
    }
}

pub async fn validate_token(
    access_token: &str,
    secret_key: &[u8],
) -> Result<Claims, jsonwebtoken::errors::Error> {
    let token_data = decode::<Claims>(
        access_token,
        &DecodingKey::from_secret(secret_key),
        &Validation::new(Algorithm::HS256),
    )?;
    let claims = token_data.claims;

    if claims.exp < Utc::now().timestamp() {
        return Err(jsonwebtoken::errors::ErrorKind::ExpiredSignature.into());
    }

    Ok(claims)
}

pub async fn auth_middleware(
    config: web::Data<Config>,
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
    let token = req
        .headers()
        .get("Authentication")
        .and_then(|value| value.to_str().ok())
        .map(str::to_owned);

    let Some(token_v) = token else {
        return Err(actix_web::error::ErrorUnauthorized(""));
    };

    match validate_token(&token_v, config.json_token.as_bytes()).await {
        Ok(_) => next.call(req).await,
        Err(_) => Err(actix_web::error::ErrorUnauthorized("")),
    }
}
