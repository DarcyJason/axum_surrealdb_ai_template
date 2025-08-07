use std::sync::Arc;

use axum::{Extension, Json};

use crate::{dtos::token::{RefreshTokenRequest, TokenResponse}, errors::Result, state::AppState, utils::token::refresh_access_token};

use serde::Deserialize;

pub async fn refresh_access_token_handler(
    Extension(app_state): Extension<Arc<AppState>>,
    Json(payload): Json<RefreshTokenRequest>
) -> Result<Json<TokenResponse>> {
    let new_access_token = refresh_access_token(app_state.config.clone(), &payload.refresh_token)?;
    Ok(Json(
        TokenResponse { access_token: new_access_token }
    ))
}