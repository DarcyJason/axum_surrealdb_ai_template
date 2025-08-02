use std::sync::Arc;

use axum::{Extension, Router};

use crate::{
    routes::{protected::protected_routes, public::public_routes},
    state::AppState,
};

pub mod protected;
pub mod public;

pub fn create_routes(app_state: Arc<AppState>) -> Router {
    let api_route = Router::new()
        .merge(protected_routes())
        .merge(public_routes())
        .layer(Extension(app_state));
    Router::new().nest("/v0/api", api_route)
}
