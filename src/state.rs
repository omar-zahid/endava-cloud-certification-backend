use std::sync::Arc;

use azure_jwt::AzureAuth;
use sqlx::PgPool;
use tokio::sync::Mutex;

pub type SharedAzureAuth = Arc<Mutex<AzureAuth>>;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub azure: SharedAzureAuth,
    pub azure_client_id: String,
}
