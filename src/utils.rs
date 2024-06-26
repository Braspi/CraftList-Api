use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    error::{ErrorBadRequest, ErrorUnauthorized},
    http::header,
    web, HttpRequest,
};
use actix_web_lab::middleware::Next;
use chrono::{Duration, NaiveDateTime, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;

use crate::{
    entities::{auth, sea_orm_active_enums::Role, users},
    error::AppError,
    Config,
};

pub fn validate<T>(body: Value) -> Result<T, AppError>
where
    T: for<'a> Deserialize<'a>,
{
    let data: Result<T, serde_json::Error> = serde_json::from_value(body);
    let req: T = match data {
        Ok(v) => v,
        Err(e) => {
            return Err(AppError::Serde(e));
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

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret_key),
    )
    .expect("token creation failed")
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
        return Err(ErrorUnauthorized("Missing JWT token"));
    };

    match validate_token(&token_v, config.json_token.as_bytes()).await {
        Ok(_) => next.call(req).await,
        Err(_) => Err(ErrorUnauthorized("Invalid token")),
    }
}

pub async fn admin_auth_middleware(
    db: web::Data<Arc<DatabaseConnection>>,
    config: web::Data<Config>,
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
    let Ok(claims) = validate_token(req.get_token()?, config.json_token.as_bytes()).await else {
        return Err(ErrorUnauthorized("Invalid token"));
    };

    let user = users::Entity::find()
        .filter(users::Column::Id.eq(claims.sub))
        .filter(users::Column::Role.eq(Role::Admin))
        .one(db.get_ref().as_ref())
        .await
        .unwrap();

    if user.is_some() {
        return next.call(req).await;
    }

    Err(ErrorUnauthorized(
        "You are not authorized to access this resource",
    ))
}

pub trait RequestUtils {
    fn get_user_id(&self) -> Result<i32, AppError>;
    fn get_token(&self) -> Result<&str, actix_web::Error>;
}

macro_rules! impl_request_utils {
    ($type:ty) => {
        impl RequestUtils for $type {
            fn get_user_id(&self) -> Result<i32, AppError> {
                let config = self.app_data::<Config>().unwrap();

                let token_data = decode::<Claims>(
                    self.get_token()?,
                    &DecodingKey::from_secret(config.json_token.as_bytes()),
                    &Validation::new(Algorithm::HS256),
                )?;

                Ok(token_data.claims.sub)
            }

            fn get_token(&self) -> Result<&str, actix_web::Error> {
                self.headers()
                    .get(header::AUTHORIZATION)
                    .and_then(|value| value.to_str().ok())
                    .ok_or(ErrorBadRequest("Missing JWT token"))
            }
        }
    };
}

impl_request_utils!(HttpRequest);
impl_request_utils!(ServiceRequest);
