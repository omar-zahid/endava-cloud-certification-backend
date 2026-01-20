pub mod auth;
pub mod config;
pub mod db;
pub mod error;
pub mod routes;
pub mod state;
pub mod telemetry;

use axum::Router;
use config::Settings;
use state::AppState;

pub async fn build_app(settings: &Settings) -> anyhow::Result<Router> {
    let pool = db::new_pool(&settings.database_url, settings.database_max_connections).await?;
    let azure = auth::azure::build_azure_auth(&settings.azure_client_id).await?;
    let azure_client_id = settings.azure_client_id.clone();

    let state = AppState {
        pool,
        azure,
        azure_client_id,
    };

    Ok(routes::router(state))
}
