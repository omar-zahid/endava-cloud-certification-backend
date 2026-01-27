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
    pub role: Option<String>,
}

pub async fn get_certificate(
    State(state): State<AppState>,
    Query(filter): Query<CertificateFilter>,
) -> Result<Json<Vec<Certificate>>, AppError> {
    let certificates = sqlx::query_as::<_, Certificate>(
        "SELECT * FROM certificates WHERE ($1::certificate_vendor IS NULL OR vendor = $1) AND ($2::TEXT IS NULL OR role = $2)",
    )
    .bind(filter.vendor)
    .bind(filter.role)
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(certificates))
}

pub async fn get_certificate_roles(
    State(state): State<AppState>,
) -> Result<Json<Vec<String>>, AppError> {
    let roles = sqlx::query_as::<_, (String,)>("SELECT DISTINCT role FROM certificates")
        .fetch_all(&state.pool)
        .await?
        .into_iter()
        .map(|(role,)| role)
        .collect();
    Ok(Json(roles))
}
