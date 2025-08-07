use axum::Router;

use crate::routes::auth::auth_routes;

pub fn public_routes() -> Router {
    Router::new()
        .merge(auth_routes())
}
