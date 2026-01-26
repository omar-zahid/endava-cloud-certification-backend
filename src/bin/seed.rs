use sqlx::PgPool;
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL")?;
    let pool = PgPool::connect(&database_url).await?;

    axum_web::db::seed::seed_certificates_from_json(&pool, "data/certificates.json").await?;

    println!("✅ Database seeded successfully");

    Ok(())
}
