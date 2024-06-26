use actix_web::http::header::ContentType;
use actix_web::{body, http::StatusCode, HttpResponse};
use serde_json::json;
use thiserror::Error;
use utoipa::ToSchema;

#[derive(Debug, Error, ToSchema)]
pub enum AppError {
    #[error("Database Error: {0}")]
    Db(#[from] sea_orm::DbErr),

    #[error("Io Error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Error while serializing or deserializing JSON: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("{0}")]
    Actix(#[from] actix_web::error::Error),

    #[error("{0}")]
    JsonWebToken(#[from] jsonwebtoken::errors::Error),
}

impl AppError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::Db(_) | Self::Io(_) | Self::JsonWebToken(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Actix(e) => e.as_response_error().status_code(),
            Self::Serde(_) => StatusCode::BAD_REQUEST,
        }
    }
}

impl actix_web::error::ResponseError for AppError {
    fn error_response(&self) -> HttpResponse<body::BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .json(json!({
                "code": self.status_code().as_u16(),
                "message": self.to_string()
            }))
    }

    fn status_code(&self) -> StatusCode {
        match self {
            Self::Db(_) | Self::Io(_) | Self::JsonWebToken(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Actix(e) => e.as_response_error().status_code(),
            Self::Serde(_) => StatusCode::BAD_REQUEST,
        }
    }
}
