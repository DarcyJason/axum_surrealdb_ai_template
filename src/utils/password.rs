use argon2::{password_hash::{rand_core::OsRng, PasswordHasher, SaltString}, Argon2, PasswordHash, PasswordVerifier};

use crate::errors::{password::PasswordErr, Result};

const MAX_PASSWORD_LENGTH: usize = 64;

pub fn password_hash(password: String) -> Result<String> {
    if password.is_empty() {
        return Err(PasswordErr::EmptyPassword.into());
    }
    if password.len() > MAX_PASSWORD_LENGTH {
        return Err(PasswordErr::ExceededMaxPasswordLength(MAX_PASSWORD_LENGTH).into());
    }
    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| PasswordErr::PasswordHashingError)?
        .to_string();
    Ok(hashed_password)
}

pub fn compare(password: &str, hashed_password: &str) -> Result<bool> {
    if password.is_empty() {
        return Err(PasswordErr::EmptyPassword.into());
    }
    if password.len() > MAX_PASSWORD_LENGTH {
        return Err(PasswordErr::ExceededMaxPasswordLength(MAX_PASSWORD_LENGTH).into());
    }
    let parsed_hash = PasswordHash::new(hashed_password)
    .map_err(|_| PasswordErr::InvalidHashFormat)?;
    let password_matched = Argon2::default()
    .verify_password(password.as_bytes(), &parsed_hash)
    .map_or(false, |_| true);
    Ok(password_matched)
}