use figment::{
    Figment,
    providers::{Format, Toml},
};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Frontend {
    pub url: String,
}

impl Frontend {
    pub fn init() -> Result<Self, figment::Error> {
        Figment::new()
            .merge(Toml::file("settings.toml").nested())
            .select("frontend")
            .extract()
    }
}
