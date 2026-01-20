use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::Serialize;

use crate::auth::azure::AzureClaims;

const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[derive(Serialize)]
struct RootResponse {
    name: &'static str,
    version: &'static str,
}

pub async fn root(_claims: AzureClaims) -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(RootResponse {
            name: "Endava Cloud Certification - API Server",
            version: APP_VERSION,
        }),
    )
}
