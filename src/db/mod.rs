pub mod seed;
use std::time::Duration;

use sqlx::{PgPool, postgres::PgPoolOptions};

/// Creates a new `PostgreSQL` connection pool.
///
/// # Errors
///
/// Returns an error if the connection pool cannot be established.
pub async fn new_pool(url: &str, max_connections: u32) -> anyhow::Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(max_connections)
        .acquire_timeout(Duration::from_secs(5))
        .connect(url)
        .await?;

    Ok(pool)
}
