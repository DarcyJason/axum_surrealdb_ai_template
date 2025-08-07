use thiserror::Error;

use crate::errors::http::HttpError;

#[derive(Debug, Error)]
pub enum PasswordErr {
    #[error("Password cannot be empty")]
    EmptyPassword,
    #[error("Password exceeds max length of {0}")]
    ExceededMaxPasswordLength(usize),
    #[error("Failed to hash password")]
    PasswordHashingError,
    #[error("Invalid password hash format")]
    InvalidHashFormat,
}

impl PasswordErr {
    pub fn to_http_error(&self) -> HttpError {
        match self {
            PasswordErr::EmptyPassword => HttpError::bad_request("Password cannot be empty".to_string()),
            PasswordErr::ExceededMaxPasswordLength(max) => HttpError::bad_request(format!("Password exceeds max length of {max}")),
            PasswordErr::PasswordHashingError => HttpError::internal_server_error("Failed to hash password".to_string()),
            PasswordErr::InvalidHashFormat => HttpError::bad_request("Invalid password hash format".to_string()),
        }
    }
}

