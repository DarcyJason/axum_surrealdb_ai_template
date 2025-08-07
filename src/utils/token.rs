use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

use crate::{config::Config, errors::{jwt::JwtError, Result}, models::token::{Claims, TokenType}};

pub fn generate_access_token(config: Config, user_id: String) -> Result<String> {
    let exp = Utc::now()
        .checked_add_signed(Duration::seconds(config.jwt.access_token_expires_in))
        .unwrap()
        .timestamp() as usize;
    let claims = Claims {
        sub: user_id,
        exp,
        token_type: TokenType::Access,
    };
    Ok(encode(&Header::default(), &claims, &EncodingKey::from_secret(config.jwt.access_token_secret.as_bytes()))?)
}

pub fn generate_refresh_token(config: Config, user_id: String) -> Result<String> {
    let exp = Utc::now()
        .checked_add_signed(Duration::seconds(config.jwt.refresh_token_expires_in))
        .unwrap()
        .timestamp() as usize;
    let claims = Claims {
        sub: user_id,
        exp,
        token_type: TokenType::Refresh,
    };
    Ok(encode(&Header::default(), &claims, &EncodingKey::from_secret(config.jwt.refresh_token_secret.as_bytes()))?)
}

pub fn verify_access_token(config: Config, token: &str) -> Result<Claims> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(config.jwt.access_token_secret.as_bytes()),
        &Validation::default(),
    ).map_err(JwtError::from)?;

    if token_data.claims.token_type != TokenType::Access {
        return Err(JwtError::InvalidToken.into());
    }

    Ok(token_data.claims)
}

pub fn verify_refresh_token(config: Config, token: &str) -> Result<Claims> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(config.jwt.refresh_token_secret.as_bytes()),
        &Validation::default(),
    ).map_err(JwtError::from)?;
    if token_data.claims.token_type != TokenType::Refresh {
        return Err(JwtError::InvalidToken.into())
    }
    Ok(token_data.claims)
}

pub fn generate_token_pair(config: Config, user_id: String) -> Result<(String, String)> {
    let access_token = generate_access_token(config.clone(), user_id.clone())?;
    let refresh_token = generate_refresh_token(config, user_id)?;
    Ok((access_token, refresh_token))
}

pub fn refresh_access_token(config: Config, refresh_token: &str) -> Result<String> {
    let claims = verify_refresh_token(config.clone(), refresh_token)?;
    generate_access_token(config, claims.sub)
}

pub fn extract_token_from_header(header: &str) -> Result<&str> {
    if let Some(token) = header.strip_prefix("Bearer ") {
        Ok(token)
    } else {
        Err(JwtError::InvalidToken.into())
    }
}
