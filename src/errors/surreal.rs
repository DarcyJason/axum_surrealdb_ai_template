use thiserror::Error;

use crate::errors::http::HttpError;

#[derive(Debug, Error)]
pub enum SurrealDBError {
    #[error("SurrealDB connection failed: {0}")]
    ConnectionFailed(String),
    #[error("SurrealDB query failed: {0}")]
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
                HttpError::internal_server_error("SurrealDB connection failed".to_string())
            }
            SurrealDBError::QueryFailed(_) => {
                HttpError::internal_server_error("SurrealDB query failed".to_string())
            }
            SurrealDBError::NotFound => HttpError::not_found("Record not found".to_string()),
            SurrealDBError::Duplicate => HttpError::conflict("Duplicate record".to_string()),
        }
    }
}

impl From<surrealdb::Error> for SurrealDBError {
    fn from(err: surrealdb::Error) -> Self {
        use surrealdb::Error;

        match err {
            Error::Db(msg) => SurrealDBError::QueryFailed(msg.to_string()),
            Error::Api(msg) => SurrealDBError::ConnectionFailed(msg.to_string()),
        }
    }
}
