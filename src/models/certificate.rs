use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::Type;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Type)]
#[serde(rename_all = "PascalCase")]
#[sqlx(type_name = "certificate_vendor", rename_all = "PascalCase")]
pub enum CertificateVendor {
    Azure,
    #[serde(rename = "AWS")]
    #[sqlx(rename = "AWS")]
    AWS,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Type)]
#[serde(rename_all = "PascalCase")]
#[sqlx(type_name = "certificate_level", rename_all = "PascalCase")]
pub enum CertificateLevel {
    Associate,
    Professional,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Certificate {
    pub id: Uuid,
    pub vendor: CertificateVendor,
    pub code: Option<String>,
    pub name: String,
    pub description: String,
    pub badge_url: String,
    pub level: CertificateLevel,
    pub role: String,
    pub subject: String,
    pub external_link: String,
    pub expires: DateTime<Utc>,
    pub validity_days: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
