use std::sync::Arc;

use tokio::signal::ctrl_c;
use tracing::{error, info};

use crate::{
    config::Config,
    db::{
        init::init_surrealdb,
        prepare::{prepare_redis, prepare_surrealdb},
    },
    errors::{redis::RedisError, AppError, Result},
    log::log_init,
    routes::create_routes,
    state::AppState,
};

mod config;
mod db;
mod dtos;
mod errors;
mod handlers;
mod log;
mod middlewares;
mod models;
mod routes;
mod state;
mod utils;
mod services;

pub async fn run() -> Result<()> {
    log_init();

    let config = Config::init();

    let port = config.backend.port;

    let db = prepare_surrealdb(config.clone()).await?;
    info!("✅ Connect to SurrealDB successfully!!!");
    init_surrealdb(db.clone()).await?;

    let redis_client = prepare_redis(config.clone()).await.map_err(|e| { 
        error!("❌ Failed to connect to Redis: {}", e);
        AppError::Redis(RedisError::from(e)) 
    })?;
    info!("✅ Connect to Redis successfully!!!");

    let app_state = AppState::new(config.clone(), db, redis_client);

    let app_routes = create_routes(Arc::new(app_state));

    let listener = match tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await {
        Ok(listener) => listener,
        Err(e) => {
            error!("❌ Failed to bind to port {}: {}", port, e);
            std::process::exit(1);
        }
    };

    info!("🚀 The server is running on http://localhost:{}", port);
    if let Err(e) = axum::serve(listener, app_routes)
        .with_graceful_shutdown(async {
            match ctrl_c().await {
                Ok(()) => {
                    println!();
                    info!("✅ The server has been shut down gracefully by Ctrl + C.");
                }
                Err(e) => {
                    println!();
                    error!("❌ The Server has been shut down by accident: {}", e);
                }
            }
        })
        .await
    {
        error!("❌ Server error: {}", e);
        std::process::exit(1);
    };
    Ok(())
}
