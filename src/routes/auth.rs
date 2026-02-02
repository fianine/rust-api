use axum::{extract::State, routing::post, Json, Router};
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::errors::ApiError;
use crate::models::user::{AuthResponse, LoginRequest, RegisterRequest, User};
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
}

async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, ApiError> {
    let mut users = state.users.write().await;

    if users.values().any(|u| u.username == payload.username) {
        return Err(ApiError::BadRequest("username already taken".into()));
    }

    let user = User {
        id: Uuid::new_v4(),
        username: payload.username,
        hashed_password: payload.password, // NOTE: plain for demo only
    };

    let user_id = user.id;
    users.insert(user.id, user);

    let response = AuthResponse {
        user_id,
        token: format!("fake-token-{}", user_id),
    };

    Ok(Json(response))
}

async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, ApiError> {
    let users = state.users.read().await;
    let user = users
        .values()
        .find(|u| u.username == payload.username && u.hashed_password == payload.password)
        .ok_or(ApiError::Unauthorized)?;

    let response = AuthResponse {
        user_id: user.id,
        token: format!("fake-token-{}", user.id),
    };

    Ok(Json(response))
}
