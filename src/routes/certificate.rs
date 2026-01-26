use axum::{Json, extract::State};

use crate::{error::AppError, models::certificate::Certificate, state::AppState};

pub async fn get_certificate(
    State(state): State<AppState>,
) -> Result<Json<Vec<Certificate>>, AppError> {
    let certificates = sqlx::query_as::<_, Certificate>("SELECT * FROM certificates")
        .fetch_all(&state.pool)
        .await?;

    Ok(Json(certificates))
}
