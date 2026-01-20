use axum::response::IntoResponse;

use crate::auth::azure::AzureClaims;

pub async fn root(_claims: AzureClaims) -> impl IntoResponse {
    "Endava Cloud Certification API Server"
}
