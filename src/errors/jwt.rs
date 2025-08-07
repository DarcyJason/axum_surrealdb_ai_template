use thiserror::Error;

use crate::errors::http::HttpError;

#[derive(Debug, Error)]
pub enum JwtError {
    #[error("Token creation failed: {0}")]
    TokenCreation(String),
    #[error("Token decoding failed: {0}")]
    TokenDecoding(String),
    #[error("Token expired")]
    TokenExpired,
    #[error("Invalid token")]
    InvalidToken,
}

impl JwtError {
    pub fn to_http_error(&self) -> HttpError {
        match self {
            JwtError::TokenCreation(msg) => {
                HttpError::internal_server_error(format!("Failed to create token: {msg}"))
            }
            JwtError::TokenDecoding(msg) => {
                HttpError::unauthorized(format!("Failed to decode token: {msg}"))
            }
            JwtError::TokenExpired => HttpError::unauthorized("Token expired".to_string()),
            JwtError::InvalidToken => HttpError::unauthorized("Invalid token".to_string()),
        }
    }
}

impl From<jsonwebtoken::errors::Error> for JwtError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        use jsonwebtoken::errors::ErrorKind;

        match err.kind() {
            ErrorKind::InvalidToken => JwtError::InvalidToken,
            ErrorKind::ExpiredSignature => JwtError::TokenExpired,
            ErrorKind::InvalidIssuer => JwtError::TokenDecoding("Invalid issuer".to_string()),
            ErrorKind::InvalidAudience => JwtError::TokenDecoding("Invalid audience".to_string()),
            ErrorKind::InvalidSubject => JwtError::TokenDecoding("Invalid subject".to_string()),
            ErrorKind::InvalidAlgorithm => JwtError::TokenDecoding("Invalid algorithm".to_string()),
            ErrorKind::MissingRequiredClaim(claim) => {
                JwtError::TokenDecoding(format!("Missing required claim: {claim}"))
            }
            ErrorKind::ImmatureSignature => JwtError::TokenDecoding("Immature signature".to_string()),
            ErrorKind::InvalidSignature => JwtError::InvalidToken,
            ErrorKind::Base64(_) => JwtError::TokenDecoding("Base64 decoding error".to_string()),
            ErrorKind::Json(_) => JwtError::TokenDecoding("JSON decoding error".to_string()),
            ErrorKind::Utf8(_) => JwtError::TokenDecoding("UTF-8 decoding error".to_string()),
            _ => JwtError::TokenDecoding("Unknown JWT error".to_string()),
        }
    }
}
