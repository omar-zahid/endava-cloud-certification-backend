use axum::{
    Json,
    extract::{Path, Query, State},
};
use serde::Deserialize;
use uuid::Uuid;

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

/// Returns certificates filtered by query parameters.
///
/// # Errors
///
/// Returns an error if the database query fails.
pub async fn get_certificate(
    State(state): State<AppState>,
    Query(filter): Query<CertificateFilter>,
) -> Result<Json<Vec<Certificate>>, AppError> {
    let certificates = sqlx::query_as::<_, Certificate>(
        "SELECT * FROM certificates
        WHERE ($1::certificate_vendor IS NULL OR vendor = $1)
        AND ($2::TEXT IS NULL OR role = $2)
        ORDER BY created_at DESC",
    )
    .bind(filter.vendor)
    .bind(filter.role)
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(certificates))
}

/// Returns a certificate by its id.
///
/// # Errors
///
/// Returns an error if the certificate is not found or the database query fails.
pub async fn get_certificate_by_id(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Certificate>, AppError> {
    let certificate = sqlx::query_as::<_, Certificate>("SELECT * FROM certificates WHERE id = $1")
        .bind(id)
        .fetch_optional(&state.pool)
        .await?;

    certificate.map_or_else(
        || Err(AppError::NotFound),
        |certificate| Ok(Json(certificate)),
    )
}

/// Returns distinct certificate roles.
///
/// # Errors
///
/// Returns an error if the database query fails.
pub async fn get_certificate_roles(
    State(state): State<AppState>,
) -> Result<Json<Vec<String>>, AppError> {
    let roles =
        sqlx::query_as::<_, (String,)>("SELECT DISTINCT role FROM certificates ORDER BY role ASC")
            .fetch_all(&state.pool)
            .await?
            .into_iter()
            .map(|(role,)| role)
            .collect();

    Ok(Json(roles))
}
