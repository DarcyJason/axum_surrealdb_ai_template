use thiserror::Error;

use crate::errors::http::HttpError;

#[derive(Debug, Error)]
pub enum DbError {
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),
    #[error("Query failed: {0}")]
    QueryFailed(String),
    #[error("Record not found")]
    NotFound,
    #[error("Duplicate record")]
    Duplicate,
}

impl DbError {
    pub fn to_http_error(self) -> HttpError {
        match self {
            DbError::ConnectionFailed(_) => {
                HttpError::internal_server_error("Database connection failed".to_string())
            }
            DbError::QueryFailed(_) => {
                HttpError::internal_server_error("Database query failed".to_string())
            }
            DbError::NotFound => HttpError::not_found("Record not found".to_string()),
            DbError::Duplicate => HttpError::conflict("Duplicate record".to_string()),
        }
    }
}

impl From<surrealdb::Error> for DbError {
    fn from(err: surrealdb::Error) -> Self {
        DbError::QueryFailed(err.to_string())
    }
}
