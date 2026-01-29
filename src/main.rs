use axum_web::{build_app, config::Settings, telemetry};
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let settings = Settings::load()?;
    telemetry::init(&settings);

    let app = build_app(&settings).await?;

    info!(addr = %settings.addr, "starting server");
    let listener = tokio::net::TcpListener::bind(settings.addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
