//! Organization response types.

use serde::Serialize;
use shared_kernel::settings::OrgSettings;

/// Full organization response with parsed settings.
#[derive(Debug, Clone, Serialize)]
pub struct OrganizationResponse {
    pub id: String,
    pub name: String,
    pub name_ar: Option<String>,
    pub slug: String,
    pub timezone: String,
    pub locale: String,
    pub date_format: Option<String>,
    pub hijri_enabled: bool,
    pub currency: String,
    pub logo: Option<String>,
    pub favicon: Option<String>,
    pub primary_color: Option<String>,
    pub settings: OrgSettings,
    pub is_active: bool,
    pub is_verified: bool,
}
