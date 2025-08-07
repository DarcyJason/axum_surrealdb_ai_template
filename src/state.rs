use surrealdb::{Surreal, engine::remote::ws::Client};
use redis::aio::MultiplexedConnection;
use crate::config::Config;

#[derive(Debug, Clone)]
pub struct AppState {
    pub config: Config,
    pub db: Surreal<Client>,
    pub redis_client: MultiplexedConnection,
}

impl AppState {
    pub fn new(config: Config, db: Surreal<Client>, redis_client: MultiplexedConnection) -> Self {
        AppState {
            config,
            db,
            redis_client,
        }
    }
}
