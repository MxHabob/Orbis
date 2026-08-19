//! Organization write commands.

use serde::{Deserialize, Serialize};

/// Create a new organization (during initial setup).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOrganizationCmd {
    pub name: String,
    pub slug: String,
    pub timezone: String,
    pub locale: String,
}

/// Update organization basic info.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateOrganizationCmd {
    pub org_id: String,
    pub name: Option<String>,
    pub name_ar: Option<String>,
    pub timezone: Option<String>,
    pub locale: Option<String>,
    pub currency: Option<String>,
    pub date_format: Option<String>,
    pub hijri_enabled: Option<bool>,
    pub logo: Option<String>,
    pub favicon: Option<String>,
    pub primary_color: Option<String>,
}

/// Update registration settings for an organization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateRegistrationSettingsCmd {
    pub org_id: String,
    pub settings: shared_kernel::settings::RegistrationSettings,
}

/// Update finance settings for an organization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFinanceSettingsCmd {
    pub org_id: String,
    pub settings: shared_kernel::settings::FinanceSettings,
}

/// Update grading settings for an organization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateGradingSettingsCmd {
    pub org_id: String,
    pub settings: shared_kernel::settings::GradingSettings,
}
