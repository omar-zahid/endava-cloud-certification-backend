use sqlx::Type;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Type)]
#[sqlx(type_name = "certificate_vendor", rename_all = "PascalCase")]
pub enum CertificateVendor {
    Azure,
    AWS,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Type)]
#[sqlx(type_name = "certificate_level", rename_all = "PascalCase")]
pub enum CertificateLevel {
    Associate,
    Professional,
}
