//! Assessment domain validation rules.
//!
//! Centralizes all validation logic for the assessment domain to ensure
//! data integrity before writes reach the database.

use shared_kernel::errors::AppError;

// ── Grade Validation ────────────────────────────────────────────

/// Validate that a grade score is within acceptable bounds.
pub fn validate_grade_score(score: f64, max_score: f64) -> Result<(), AppError> {
    if score < 0.0 {
        return Err(AppError::validation("Score cannot be negative"));
    }
    if max_score <= 0.0 {
        return Err(AppError::validation("Max score must be greater than zero"));
    }
    if score > max_score {
        return Err(AppError::validation(format!(
            "Score ({}) cannot exceed max score ({})", score, max_score
        )));
    }
    Ok(())
}

/// Validate that a grade weight is between 0 and 1 (exclusive 0).
pub fn validate_grade_weight(weight: f64) -> Result<(), AppError> {
    if weight <= 0.0 || weight > 1.0 {
        return Err(AppError::validation(format!(
            "Grade weight must be between 0 (exclusive) and 1 (inclusive), got {}", weight
        )));
    }
    Ok(())
}

/// Validate exam type is one of the allowed values.
pub fn validate_exam_type(exam_type: &str) -> Result<(), AppError> {
    const VALID_TYPES: &[&str] = &[
        "assignment", "quiz", "midterm", "final",
        "project", "participation", "homework",
        "practical", "oral", "presentation",
    ];
    if !VALID_TYPES.contains(&exam_type) {
        return Err(AppError::validation(format!(
            "Invalid exam type '{}'. Valid types: {:?}", exam_type, VALID_TYPES
        )));
    }
    Ok(())
}

// ── Attendance Validation ───────────────────────────────────────

/// Validate attendance status.
pub fn validate_attendance_status(status: &str) -> Result<(), AppError> {
    const VALID: &[&str] = &["present", "absent", "late", "excused", "early_leave"];
    if !VALID.contains(&status) {
        return Err(AppError::validation(format!(
            "Invalid attendance status '{}'. Valid statuses: {:?}", status, VALID
        )));
    }
    Ok(())
}

/// Validate date format (YYYY-MM-DD).
pub fn validate_date_format(date: &str) -> Result<(), AppError> {
    if chrono::NaiveDate::parse_from_str(date, "%Y-%m-%d").is_err() {
        return Err(AppError::validation(format!(
            "Invalid date format '{}'. Expected YYYY-MM-DD", date
        )));
    }
    Ok(())
}

// ── Behavior Validation ─────────────────────────────────────────

/// Validate behavior type.
pub fn validate_behavior_type(behavior_type: &str) -> Result<(), AppError> {
    const VALID: &[&str] = &["positive", "negative", "neutral", "observation"];
    if !VALID.contains(&behavior_type) {
        return Err(AppError::validation(format!(
            "Invalid behavior type '{}'. Valid types: {:?}", behavior_type, VALID
        )));
    }
    Ok(())
}

/// Validate behavior severity level.
pub fn validate_behavior_severity(severity: &str) -> Result<(), AppError> {
    const VALID: &[&str] = &["minor", "moderate", "major", "critical"];
    if !VALID.contains(&severity) {
        return Err(AppError::validation(format!(
            "Invalid severity '{}'. Valid levels: {:?}", severity, VALID
        )));
    }
    Ok(())
}

// ── Assessment Scheme Validation ────────────────────────────────

/// Validate that component weights in a scheme sum to 1.0 (±tolerance).
pub fn validate_scheme_weights(weights: &[f64]) -> Result<(), AppError> {
    if weights.is_empty() {
        return Err(AppError::validation("Assessment scheme must have at least one component"));
    }
    let sum: f64 = weights.iter().sum();
    if (sum - 1.0).abs() > 0.01 {
        return Err(AppError::validation(format!(
            "Component weights must sum to 1.0, got {:.4}", sum
        )));
    }
    Ok(())
}

// ── Calculation Policy Validation ───────────────────────────────

/// Validate rounding policy value.
pub fn validate_rounding_policy(policy: &str) -> Result<(), AppError> {
    const VALID: &[&str] = &["exact", "round_up", "round_down", "round_half_up"];
    if !VALID.contains(&policy) {
        return Err(AppError::validation(format!(
            "Invalid rounding policy '{}'. Valid policies: {:?}", policy, VALID
        )));
    }
    Ok(())
}

/// Validate a formula expression can be parsed as a valid AST.
pub fn validate_formula_expression(json: &str) -> Result<(), AppError> {
    use crate::ops::calc_engine::AstNode;
    serde_json::from_str::<AstNode>(json).map_err(|e| {
        AppError::validation(format!("Invalid formula expression: {}", e))
    })?;
    Ok(())
}
