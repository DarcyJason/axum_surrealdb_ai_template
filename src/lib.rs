use std::sync::Arc;

use axum::http::{
    HeaderValue, Method,
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
};
use tokio::signal::ctrl_c;
use tower_http::cors::CorsLayer;
use tracing::{error, info};

use crate::{
    config::Config, db::prepare::prepare_surrealdb, log::log_init, routes::create_routes,
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
mod services;
mod state;
mod utils;

pub async fn run() {
    log_init();

    let config = Config::init();

    let port = config.backend.port;

    let frontend_url = config.frontend.url.clone();
    let cors = CorsLayer::new()
        .allow_origin(frontend_url.parse::<HeaderValue>().unwrap())
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE])
        .allow_credentials(true)
        .allow_methods([Method::GET, Method::POST, Method::PUT]);

    let db = prepare_surrealdb(config.clone()).await;

    let app_state = AppState::new(config.clone(), db);

    let app_routes = create_routes(Arc::new(app_state)).layer(cors);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();

    info!("🚀 The server is running on http://localhost:{}", port);
    axum::serve(listener, app_routes)
        .with_graceful_shutdown(async {
            match ctrl_c().await {
                Ok(()) => {
                    println!();
                    info!("✅ The server has been shut down gracefully by Ctrl + C.");
                }
                Err(e) => {
                    println!();
                    error!("🔥 The Server has been shut down by accident: {}", e);
                }
            }
        })
        .await
        .unwrap();
}
