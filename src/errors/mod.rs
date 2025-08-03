use axum::{
    Json,
    response::{IntoResponse, Response},
};
use serde_json::json;
use thiserror::Error;

use crate::errors::{db::DbError, jwt::JwtError};

pub mod db;
pub mod http;
pub mod jwt;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] DbError),
    #[error("JWT error: {0}")]
    Jwt(#[from] JwtError),
}

pub type Result<T> = std::result::Result<T, AppError>;

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let http_error = match self {
            AppError::Database(err) => err.to_http_error(),
            AppError::Jwt(err) => err.to_http_error(),
        };
        let body = Json(json!({
            "error": {
                "message": http_error.message,
                "status": http_error.status.as_u16()
            }
        }));
        (http_error.status, body).into_response()
    }
}

impl From<surrealdb::Error> for AppError {
    fn from(err: surrealdb::Error) -> Self {
        AppError::Database(DbError::from(err))
    }
}

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        AppError::Jwt(JwtError::from(err))
    }
}
