use std::sync::Arc;

use axum::{Extension, Router};
use tower_http::trace::TraceLayer;

use crate::{
    middlewares::cors::create_cors_layer,
    routes::{protected::protected_routes, public::public_routes},
    state::AppState,
};

pub mod protected;
pub mod public;
pub mod auth;

pub fn create_routes(app_state: Arc<AppState>) -> Router {
    let cors = create_cors_layer(&app_state.config);
    let api_route = Router::new()
        .merge(protected_routes())
        .merge(public_routes())
        .layer(Extension(app_state));
    Router::new()
        .nest("/v0/api", api_route)
        .layer(cors)
        .layer(TraceLayer::new_for_http())
}
