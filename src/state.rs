use surrealdb::{Surreal, engine::remote::ws::Client};

use crate::config::Config;

#[derive(Debug, Clone)]
pub struct AppState {
    pub config: Config,
    pub db: Surreal<Client>,
    pub redis_client: redis::Client,
}

impl AppState {
    pub fn new(config: Config, db: Surreal<Client>, redis_client: redis::Client) -> Self {
        AppState {
            config,
            db,
            redis_client,
        }
    }
}
