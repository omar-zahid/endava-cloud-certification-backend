use axum::{Json, extract::State};

use crate::{
    auth::azure::AzureClaims, error::AppError, models::certificate::Certificate, state::AppState,
};

pub async fn get_certificate(
    State(state): State<AppState>,
    _: AzureClaims,
) -> Result<Json<Vec<Certificate>>, AppError> {
    let certificates = sqlx::query_as::<_, Certificate>("SELECT * FROM certificates")
        .fetch_all(&state.pool)
        .await?;

    Ok(Json(certificates))
}
