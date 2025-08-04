use axum::http::{
    HeaderValue, Method,
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
};
use tower_http::cors::CorsLayer;

use crate::config::Config;

pub fn create_cors_layer(config: &Config) -> CorsLayer {
    let frontend_url = config.frontend.url.clone();
    
    CorsLayer::new()
        .allow_origin(frontend_url.parse::<HeaderValue>().unwrap())
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE])
        .allow_credentials(true)
        .allow_methods([Method::GET, Method::POST, Method::PUT])
}
