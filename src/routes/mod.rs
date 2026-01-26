use axum::{Router, routing::get};

use crate::state::AppState;

pub mod certificate;
pub mod health;
pub mod root;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/", get(root::root))
        .route("/health", get(health::health))
        .route("/ready", get(health::ready))
        .route("/certificates", get(certificate::get_certificate))
        .with_state(state)
}
