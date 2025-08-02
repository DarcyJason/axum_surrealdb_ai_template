use serde::Deserialize;

use crate::config::{backend::Backend, db::DB, frontend::Frontend, jwt::Jwt};

pub mod backend;
pub mod db;
pub mod frontend;
pub mod jwt;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub frontend: Frontend,
    pub backend: Backend,
    pub db: DB,
    pub jwt: Jwt,
}

impl Config {
    pub fn init() -> Self {
        Config {
            frontend: Frontend::init().expect("Failed to load frontend config"),
            backend: Backend::init().expect("Failed to load backend config"),
            db: DB::init().expect("Failed to load db config"),
            jwt: Jwt::init().expect("Failed to load jwt config"),
        }
    }
}
