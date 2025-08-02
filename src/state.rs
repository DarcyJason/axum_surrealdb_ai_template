use surrealdb::{Surreal, engine::remote::ws::Client};

use crate::config::Config;

#[derive(Debug, Clone)]
pub struct AppState {
    pub config: Config,
    pub db: Surreal<Client>,
}

impl AppState {
    pub fn new(config: Config, db: Surreal<Client>) -> Self {
        AppState { config, db }
    }
}
