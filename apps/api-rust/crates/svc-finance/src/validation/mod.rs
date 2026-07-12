//! Business rule validation for the Finance domain.
//! 
//! Provides a highly professional, reusable validation engine
//! ensuring consistency across the finance microservice.

use shared_kernel::errors::AppError;

/// Trait for data objects that require business rule validation before processing.
pub trait Validate {
    fn validate(&self) -> Result<(), AppError>;
}

/// Helper to validate strings.
pub fn validate_string_length(val: &str, min: usize, max: usize, field_name: &str) -> Result<(), AppError> {
    let len = val.trim().chars().count();
    if len < min || len > max {
        return Err(AppError::validation(format!(
            "{} must be between {} and {} characters",
            field_name, min, max
        )));
    }
    Ok(())
}

/// Helper to validate amounts.
pub fn validate_positive_amount(amount: f64, field_name: &str) -> Result<(), AppError> {
    if amount <= 0.0 {
        return Err(AppError::validation(format!(
            "{} must be greater than zero",
            field_name
        )));
    }
    Ok(())
}

// ── Implementations for DTOs ────────────────────────────────────

use crate::contracts::commands::*;

impl Validate for CreateFeeTemplateCmd {
    fn validate(&self) -> Result<(), AppError> {
        validate_string_length(&self.name, 3, 100, "Fee Template Name")?;
        validate_positive_amount(self.default_amount, "Fee Template Amount")?;
        Ok(())
    }
}

impl Validate for UpdateFeeTemplateCmd {
    fn validate(&self) -> Result<(), AppError> {
        if let Some(ref n) = self.name {
            validate_string_length(n, 3, 100, "Fee Template Name")?;
        }
        if let Some(ref a) = self.default_amount {
            validate_positive_amount(*a, "Fee Template Amount")?;
        }
        Ok(())
    }
}

impl Validate for CreateDiscountTemplateCmd {
    fn validate(&self) -> Result<(), AppError> {
        validate_string_length(&self.name, 3, 100, "Discount Template Name")?;
        validate_positive_amount(self.amount, "Discount Template Amount")?;
        if self.is_percentage && self.amount > 100.0 {
            return Err(AppError::validation("Percentage discount cannot exceed 100%"));
        }
        Ok(())
    }
}

impl Validate for AssignFeeCmd {
    fn validate(&self) -> Result<(), AppError> {
        if self.fee_template_id.trim().is_empty() {
            return Err(AppError::validation("Template ID is required"));
        }
        if self.enrollment_id.trim().is_empty() {
            return Err(AppError::validation("Enrollment ID is required"));
        }
        if let Some(amt) = self.amount_override {
            validate_positive_amount(amt, "Assigned Fee Amount")?;
        }
        Ok(())
    }
}

impl Validate for RecordPaymentCmd {
    fn validate(&self) -> Result<(), AppError> {
        if self.enrollment_id.as_deref().unwrap_or("").trim().is_empty() && self.fee_id.as_deref().unwrap_or("").trim().is_empty() {
            return Err(AppError::validation("Enrollment ID or Fee ID is required"));
        }
        validate_positive_amount(self.amount, "Payment Amount")?;
        Ok(())
    }
}

impl Validate for CreateTreasuryCmd {
    fn validate(&self) -> Result<(), AppError> {
        validate_string_length(&self.name, 3, 100, "Treasury Name")?;
        if self.treasury_type != "bank" && self.treasury_type != "cash" {
            return Err(AppError::validation("Treasury type must be 'bank' or 'cash'"));
        }
        Ok(())
    }
}
