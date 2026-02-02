pub mod auth;
pub mod posts;
pub mod social;

use axum::Router;
use crate::state::AppState;

pub fn auth_routes() -> Router<AppState> {
    auth::router()
}

pub fn post_routes() -> Router<AppState> {
    posts::router()
}

pub fn social_routes() -> Router<AppState> {
    social::router()
}
