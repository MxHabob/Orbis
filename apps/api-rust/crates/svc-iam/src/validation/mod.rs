//! IAM validation — business rules.

use shared_kernel::errors::AppError;
use crate::contracts::commands::CreateUserCmd;

/// Validate a CreateUserCmd.
pub fn validate_create_user(cmd: &CreateUserCmd) -> Result<(), AppError> {
    if cmd.username.trim().is_empty() {
        return Err(AppError::validation("Username cannot be empty"));
    }
    if cmd.email.trim().is_empty() || !cmd.email.contains('@') {
        return Err(AppError::validation("Valid email is required"));
    }
    if cmd.password.len() < 8 {
        return Err(AppError::validation("Password must be at least 8 characters"));
    }
    let valid_roles = ["super_admin", "admin", "teacher", "student", "parent", "staff"];
    if !valid_roles.contains(&cmd.primary_role.to_lowercase().as_str()) {
        return Err(AppError::validation(format!("Invalid role: {}", cmd.primary_role)));
    }
    Ok(())
}
