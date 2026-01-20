use std::{env, net::SocketAddr};

pub struct Settings {
    pub addr: SocketAddr,
    pub database_url: String,
    pub database_max_connections: u32,
    pub azure_client_id: String,
}

impl Settings {
    pub fn load() -> anyhow::Result<Self> {
        let addr: SocketAddr = env::var("APP_ADDR")
            .unwrap_or_else(|_| "0.0.0.0:3000".into())
            .parse()?;

        let database_url = env::var("DATABASE_URL")?;

        let database_max_connections = env::var("DATABASE_MAX_CONNECTIONS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(10);

        let azure_client_id = env::var("AZURE_CLIENT_ID")?;

        Ok(Self {
            addr,
            database_url,
            database_max_connections,
            azure_client_id,
        })
    }
}
