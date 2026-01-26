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
    vendor: String,
    level: String,
    role: String,
    subject: String,
    external_link: String,
}

impl CertificateLevel {
    fn from_json(value: &str) -> anyhow::Result<Self> {
        match value {
            "Associate" => Ok(Self::Associate),
            "Professional" => Ok(Self::Professional),
            _ => Err(anyhow::anyhow!("Invalid certificate level: {}", value)),
        }
    }
}
impl CertificateVendor {
    fn from_json(value: &str) -> anyhow::Result<Self> {
        match value {
            "Azure" => Ok(Self::Azure),
            "AWS" => Ok(Self::AWS),
            _ => Err(anyhow::anyhow!("Invalid certificate vendor: {}", value)),
        }
    }
}

// pub async fn seed_certificates_from_json(pool: &PgPool, path: &str) -> anyhow::Result<()> {
//     let json = fs::read_to_string(path)?;
//     let certs: Vec<CertificateJson> = serde_json::from_str(&json)?;
//
//     for cert in certs {
//         let vendor = CertificateVendor::from_json(&cert.vendor)?;
//         let level = CertificateLevel::from_json(&cert.level)?;
//
//         query!(
//             r#"
//             INSERT INTO certificates
//             (name, description, badge_url, vendor, level, role, subject, external_link)
//             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
//             ON CONFLICT (name) DO NOTHING
//             "#,
//             cert.name,
//             cert.description,
//             cert.badge_url,
//             vendor as CertificateVendor,
//             level as CertificateLevel,
//             cert.role,
//             cert.subject,
//             cert.external_link
//         )
//         .execute(pool)
//         .await?;
//     }
//
//     Ok(())
// }
