use std::sync::Arc;

use axum::{extract::FromRequestParts, http::HeaderValue};
use azure_jwt::{AzureAuth, AzureJwtClaims};
use jsonwebtoken::{Algorithm, Validation};
use tokio::sync::Mutex;
use tracing::{debug, warn};

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
        let authz = match parts.headers.get(axum::http::header::AUTHORIZATION) {
            Some(h) => {
                debug!("Authorization header found.");
                h
            }
            None => {
                warn!("Missing authorization header,");
                return Err(AppError::Unauthorized);
            }
        };

        let token = match bearer_token(authz) {
            Some(t) => {
                debug!("Bearer token extracted (len={})", t.len());
                t
            }
            None => {
                warn!("Auth header is not a valid Bearer token");
                return Err(AppError::Unauthorized);
            }
        };

        let decoded = {
            debug!("Validating Azure JWT");

            let mut auth = state.azure.lock().await;
            let mut validator = Validation::new(Algorithm::RS256);

            validator.set_audience(&[format!("api://{}", state.azure_client_id)]);

            match auth
                .validate_custom_async::<AzureJwtClaims>(token, &validator)
                .await
            {
                Ok(d) => {
                    debug!(
                        aud = ?d.claims.aud,
                        iss = %d.claims.iss,
                        scp = ?d.claims.scp,
                        tid = %d.claims.tid,
                    "Azure JWT validated successfully");
                    d
                }
                Err(e) => {
                    warn!(error = ?e, "Azure JWT validation failed");
                    return Err(AppError::Unauthorized);
                }
            }
        };

        Ok(AzureClaims(decoded.claims))
    }
}
