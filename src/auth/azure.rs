use std::sync::Arc;

use axum::{extract::FromRequestParts, http::HeaderValue};
use azure_jwt::{AzureAuth, AzureJwtClaims};
use tokio::sync::Mutex;

use crate::{error::AppError, state::AppState};

pub async fn build_azure_auth(client_id: &str) -> anyhow::Result<Arc<Mutex<AzureAuth>>> {
    let auth = AzureAuth::new_async(client_id).await?;
    Ok(Arc::new(Mutex::new(auth)))
}

pub struct AzureClaims(pub AzureJwtClaims);

fn bearer_token(h: &HeaderValue) -> Option<&str> {
    let s = h.to_str().ok()?;
    let s = s.strip_prefix("Bearer ")?;
    Some(s.trim())
}

impl FromRequestParts<AppState> for AzureClaims {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let authz = parts
            .headers
            .get(axum::http::header::AUTHORIZATION)
            .ok_or(AppError::Unauthorized)?;

        let token = bearer_token(authz).ok_or(AppError::Unauthorized)?;

        let decoded = {
            let mut auth = state.azure.lock().await;
            auth.validate_token_async(token)
                .await
                .map_err(|_| AppError::Unauthorized)?
        };

        Ok(AzureClaims(decoded.claims))
    }
}
