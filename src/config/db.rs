use figment::{
    Figment,
    providers::{Format, Toml},
};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct DB {
    pub surrealdb_host: String,
    pub surrealdb_root_name: String,
    pub surrealdb_root_password: String,
    pub surrealdb_namespace: String,
    pub surrealdb_database: String,
}

impl DB {
    pub fn init() -> Result<Self, figment::Error> {
        Figment::new()
            .merge(Toml::file("settings.toml").nested())
            .select("db")
            .extract()
    }
}
