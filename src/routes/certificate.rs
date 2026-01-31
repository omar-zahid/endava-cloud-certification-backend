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

pub async fn get_certificate_by_id(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Certificate>, AppError> {
    let certificate = sqlx::query_as::<_, Certificate>("SELECT * FROM certificates WHERE id = $1")
        .bind(id)
        .fetch_optional(&state.pool)
        .await?;

    match certificate {
        Some(certificate) => Ok(Json(certificate)),
        None => Err(AppError::NotFound),
    }
}

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
