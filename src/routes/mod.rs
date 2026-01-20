use axum::{Router, routing::get};

use crate::state::AppState;

pub mod health;
pub mod root;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health::health))
        .route("/ready", get(health::ready))
        .route("/", get(root::root))
        .with_state(state)
}
