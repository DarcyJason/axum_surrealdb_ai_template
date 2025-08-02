use figment::{
    Figment,
    providers::{Format, Toml},
};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Jwt {
    pub access_token_secret: String,
    pub refresh_token_secret: String,
    pub access_token_expires_in: u64,
    pub refresh_token_expires_in: u64,
    pub token_cleanup_interval: u64,
}

impl Jwt {
    pub fn init() -> Result<Self, figment::Error> {
        Figment::new()
            .merge(Toml::file("settings.toml").nested())
            .select("jwt")
            .extract()
    }
}
