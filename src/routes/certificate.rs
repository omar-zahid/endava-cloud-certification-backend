use axum::{
    Json,
    extract::{Query, State},
};
use serde::Deserialize;

use crate::{
    error::AppError,
    models::certificate::{Certificate, CertificateVendor},
    state::AppState,
};

#[derive(Debug, Deserialize)]
pub struct CertificateFilter {
    pub vendor: Option<CertificateVendor>,
}

pub async fn get_certificate(
    State(state): State<AppState>,
    Query(filter): Query<CertificateFilter>,
) -> Result<Json<Vec<Certificate>>, AppError> {
    let certificates = sqlx::query_as::<_, Certificate>(
        "SELECT * FROM certificates WHERE ($1::certificate_vendor IS NULL OR vendor = $1)",
    )
    .bind(filter.vendor)
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(certificates))
}
