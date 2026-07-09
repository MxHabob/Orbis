use std::sync::Arc;
use crate::repository::LifecycleQueryRepo;
use shared_kernel::errors::AppError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrgSettings {
    #[serde(default = "default_prefix")]
    pub student_number_prefix: String,
    #[serde(default = "default_digits")]
    pub student_number_digits: usize,
    #[serde(default = "default_required_fields")]
    pub required_registration_fields: Vec<String>,
}

fn default_prefix() -> String {
    "ST".to_string()
}

fn default_digits() -> usize {
    6
}

fn default_required_fields() -> Vec<String> {
    vec!["email".to_string(), "phone".to_string()]
}

pub struct OrganizationSettingsService {
    pub query: Arc<LifecycleQueryRepo>,
}

impl OrganizationSettingsService {
    pub fn new(query: Arc<LifecycleQueryRepo>) -> Self {
        Self { query }
    }

    /// Retrieve the settings for an organization.
    pub async fn get_settings(&self, org_id: &str) -> Result<OrgSettings, AppError> {
        let org = self.query.get_organization_by_id(org_id).await?
            .ok_or_else(|| AppError::not_found("organization", org_id))?;

        if let Some(settings_str) = org.settings {
            if let Ok(settings) = serde_json::from_str::<OrgSettings>(&settings_str) {
                return Ok(settings);
            }
        }

        Ok(OrgSettings {
            student_number_prefix: default_prefix(),
            student_number_digits: default_digits(),
            required_registration_fields: default_required_fields(),
        })
    }

    /// Format student number based on organization settings.
    pub async fn format_student_number(&self, org_id: &str, sequence: u64) -> Result<String, AppError> {
        let settings = self.get_settings(org_id).await?;
        let prefix = settings.student_number_prefix;
        let digits = settings.student_number_digits;
        Ok(format!("{}{:0width$}", prefix, sequence, width = digits))
    }
}
