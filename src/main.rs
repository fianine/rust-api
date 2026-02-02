mod config;
mod routes;
mod state;
mod errors;
mod models;

use axum::{routing::get, Router};
use std::net::SocketAddr;
use crate::state::AppState;

#[tokio::main]
async fn main() {
    let config = config::AppConfig::from_env();
    let state = AppState::new();

    let app = Router::new()
        .route("/health", get(health_check))
        .nest("/auth", routes::auth_routes())
        .nest("/posts", routes::post_routes())
        .nest("/social", routes::social_routes())
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    println!("Starting server on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("server failed");
}

async fn health_check() -> &'static str {
    "OK"
}
