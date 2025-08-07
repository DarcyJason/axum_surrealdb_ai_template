use thiserror::Error;

use crate::errors::http::HttpError;

#[derive(Debug, Error)]
pub enum SurrealDBError {
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),
    #[error("Query failed: {0}")]
    QueryFailed(String),
    #[error("Record not found")]
    NotFound,
    #[error("Duplicate record")]
    Duplicate,
}

impl SurrealDBError {
    pub fn to_http_error(&self) -> HttpError {
        match self {
            SurrealDBError::ConnectionFailed(_) => {
                HttpError::internal_server_error("Database connection failed".to_string())
            }
            SurrealDBError::QueryFailed(_) => {
                HttpError::internal_server_error("Database query failed".to_string())
            }
            SurrealDBError::NotFound => HttpError::not_found("Record not found".to_string()),
            SurrealDBError::Duplicate => HttpError::conflict("Duplicate record".to_string()),
        }
    }
}

impl From<surrealdb::Error> for SurrealDBError {
    fn from(err: surrealdb::Error) -> Self {
        SurrealDBError::QueryFailed(err.to_string())
    }
}
