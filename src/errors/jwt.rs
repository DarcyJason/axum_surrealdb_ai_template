use thiserror::Error;

use crate::errors::http::HttpError;

#[derive(Debug, Error)]
pub enum JwtError {
    #[error("Invalid token")]
    InvalidToken,
    #[error("Token expired")]
    Expired,
    #[error("Missing authorization header")]
    MissingHeader,
    #[error("Token generation failed")]
    GenerationFailed,
}

impl JwtError {
    pub fn to_http_error(&self) -> HttpError {
        match self {
            JwtError::InvalidToken => HttpError::unauthorized("Invalid token".to_string()),
            JwtError::Expired => HttpError::unauthorized("Token expired".to_string()),
            JwtError::MissingHeader => {
                HttpError::unauthorized("Missing authorization header".to_string())
            }
            JwtError::GenerationFailed => {
                HttpError::internal_server_error("Token generation failed".to_string())
            }
        }
    }
}

impl From<jsonwebtoken::errors::Error> for JwtError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        match err.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => JwtError::Expired,
            _ => JwtError::InvalidToken,
        }
    }
}
