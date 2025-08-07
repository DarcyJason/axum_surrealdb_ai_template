use axum::{routing::post, Router};

use crate::handlers::auth::refresh_access_token_handler;

pub fn auth_routes() -> Router {
    Router::new()
        .route("/auth/refresh", post(refresh_access_token_handler))
}
