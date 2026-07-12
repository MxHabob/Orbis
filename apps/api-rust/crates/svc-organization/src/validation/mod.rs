//! Lifeperiod validation  - business rule enforcement.
//!
//! Validates commands before execution in ops.

use shared_kernel::errors::AppError;
use crate::contracts::commands::{CreateOrganizationCmd, UpdateOrganizationCmd};

/// Validate CreateOrganizationCmd.
pub fn validate_create_organization(cmd: &CreateOrganizationCmd) -> Result<(), AppError> {
    if cmd.name.trim().is_empty() {
        return Err(AppError::validation("Organization name cannot be empty"));
    }
    if cmd.slug.trim().is_empty() {
        return Err(AppError::validation("Organization slug cannot be empty"));
    }
    if !cmd.slug.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
        return Err(AppError::validation("Slug can only contain alphanumeric characters, dashes, and underscores"));
    }
    if cmd.timezone.trim().is_empty() {
        return Err(AppError::validation("Timezone cannot be empty"));
    }
    Ok(())
}

/// Validate UpdateOrganizationCmd.
pub fn validate_update_organization(cmd: &UpdateOrganizationCmd) -> Result<(), AppError> {
    if let Some(ref name) = cmd.name {
        if name.trim().is_empty() {
            return Err(AppError::validation("Organization name cannot be empty"));
        }
    }
    Ok(())
}
