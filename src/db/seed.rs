use std::fs;

use serde::Deserialize;
use sqlx::{PgPool, query};

use crate::models::certificate::{CertificateLevel, CertificateVendor};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CertificateJson {
    name: String,
    description: String,
    badge_url: String,
    vendor: CertificateVendor,
    level: CertificateLevel,
    role: String,
    subject: String,
    external_link: String,
}

/// Seeds certificates from a JSON file.
///
/// # Errors
///
/// Returns an error if the file cannot be read, parsed, or inserted into the database.
pub async fn seed_certificates_from_json(pool: &PgPool, path: &str) -> anyhow::Result<()> {
    let json = fs::read_to_string(path)?;
    let certs: Vec<CertificateJson> = serde_json::from_str(&json)?;

    for cert in certs {
        query(
            r"
            INSERT INTO certificates
            (name, description, badge_url, vendor, level, role, subject, external_link)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            ",
        )
        .bind(cert.name)
        .bind(cert.description)
        .bind(cert.badge_url)
        .bind(cert.vendor)
        .bind(cert.level)
        .bind(cert.role)
        .bind(cert.subject)
        .bind(cert.external_link)
        .execute(pool)
        .await?;
    }

    Ok(())
}
