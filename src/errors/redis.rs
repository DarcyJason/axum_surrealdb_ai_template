use crate::errors::http::HttpError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RedisError {
    #[error("Redis connection failed: {0}")]
    ConnectionFailed(String),
    #[error("Redis command failed: {0}")]
    CommandFailed(String),
    #[error("Key not found")]
    KeyNotFound,
    #[error("Duplicate key")]
    DuplicateKey,
}

impl RedisError {
    pub fn to_http_error(&self) -> HttpError {
        match self {
            RedisError::ConnectionFailed(_) => {
                HttpError::internal_server_error("Redis connection failed".to_string())
            }
            RedisError::CommandFailed(_) => {
                HttpError::internal_server_error("Redis command failed".to_string())
            }
            RedisError::KeyNotFound => HttpError::not_found("Key not found".to_string()),
            RedisError::DuplicateKey => HttpError::conflict("Duplicate key".to_string()),
        }
    }
}

impl From<redis::RedisError> for RedisError {
    fn from(err: redis::RedisError) -> Self {
        if err.is_connection_refusal() {
            RedisError::ConnectionFailed(err.to_string())
        } else if err.is_timeout() {
            RedisError::ConnectionFailed(err.to_string())
        } else if err.is_io_error() {
            RedisError::ConnectionFailed(err.to_string())
        } else {
            let error_msg = err.to_string();
            if error_msg.contains("key not found") || error_msg.contains("nil") {
                RedisError::KeyNotFound
            } else {
                RedisError::CommandFailed(error_msg)
            }
        }
    }
}
