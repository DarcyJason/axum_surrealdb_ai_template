use figment::{
    Figment,
    providers::{Format, Toml},
};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Backend {
    pub port: u16,
}

impl Backend {
    pub fn init() -> Result<Self, figment::Error> {
        Figment::new()
            .merge(Toml::file("settings.toml").nested())
            .select("backend")
            .extract()
    }
}
