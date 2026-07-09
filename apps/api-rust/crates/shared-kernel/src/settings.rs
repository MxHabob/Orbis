//! Organization settings types — shared across all microservices.
//!
//! These types are the Single Source of Truth for org-level configuration.
//! `svc-organization` populates them; every other service reads them via `SettingsProvider`.

use serde::{Deserialize, Serialize};

// ── Registration Settings ───────────────────────────────────────

/// Controls every aspect of student registration across the platform.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistrationSettings {
    /// Required fields for registration (email, phone, national_id, guardian, birth_date…).
    #[serde(default = "default_required_fields")]
    pub required_fields: Vec<String>,

    /// Student number prefix (ST, STU, …).
    #[serde(default = "default_prefix")]
    pub student_number_prefix: String,

    /// Number of digits in student number (e.g. 6 → ST000042).
    #[serde(default = "default_digits")]
    pub student_number_digits: usize,

    /// Whether student registration requires admin approval.
    #[serde(default)]
    pub requires_approval: bool,

    /// Maximum number of students allowed per class.
    #[serde(default)]
    pub max_class_capacity: Option<u32>,

    /// Whether a student can be registered without a guardian.
    #[serde(default = "default_true")]
    pub allow_no_guardian: bool,

    /// Whether duplicate national IDs are allowed.
    #[serde(default)]
    pub allow_duplicate_national_id: bool,

    /// Whether fees are automatically assigned on enrollment.
    #[serde(default = "default_true")]
    pub auto_assign_fees: bool,

    /// Default fee template IDs to assign on enrollment.
    #[serde(default)]
    pub default_fee_template_ids: Vec<String>,
}

impl Default for RegistrationSettings {
    fn default() -> Self {
        Self {
            required_fields: default_required_fields(),
            student_number_prefix: default_prefix(),
            student_number_digits: default_digits(),
            requires_approval: false,
            max_class_capacity: None,
            allow_no_guardian: true,
            allow_duplicate_national_id: false,
            auto_assign_fees: true,
            default_fee_template_ids: vec![],
        }
    }
}

// ── Finance Settings ────────────────────────────────────────────

/// Controls the financial system behaviour for an organization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinanceSettings {
    /// Default currency code (SAR, USD, EGP, …).
    #[serde(default = "default_currency")]
    pub default_currency: String,

    /// Whether partial payments are accepted.
    #[serde(default = "default_true")]
    pub allow_partial_payment: bool,

    /// Whether late payments are accepted (after due date).
    #[serde(default = "default_true")]
    pub allow_late_payment: bool,

    /// Late fee percentage applied after grace period.
    #[serde(default)]
    pub late_fee_percentage: Option<f64>,

    /// Grace period in days before late fee applies.
    #[serde(default)]
    pub grace_period_days: Option<u32>,

    /// Whether discounts are allowed.
    #[serde(default = "default_true")]
    pub allow_discounts: bool,

    /// Maximum allowed discount percentage.
    #[serde(default)]
    pub max_discount_percentage: Option<f64>,

    /// Default ledger revenue account ID.
    #[serde(default)]
    pub default_revenue_account_id: Option<String>,

    /// Default ledger receivable account ID.
    #[serde(default)]
    pub default_receivable_account_id: Option<String>,
}

impl Default for FinanceSettings {
    fn default() -> Self {
        Self {
            default_currency: default_currency(),
            allow_partial_payment: true,
            allow_late_payment: true,
            late_fee_percentage: None,
            grace_period_days: None,
            allow_discounts: true,
            max_discount_percentage: None,
            default_revenue_account_id: None,
            default_receivable_account_id: None,
        }
    }
}

// ── Grading Settings ────────────────────────────────────────────

/// Controls grade entry and evaluation rules.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GradingSettings {
    /// Grading system: "percentage", "gpa", "letter", "points".
    #[serde(default = "default_grading_system")]
    pub grading_system: String,

    /// Minimum passing grade.
    #[serde(default = "default_passing_grade")]
    pub passing_grade: f64,

    /// Maximum grade value.
    #[serde(default = "default_max_grade")]
    pub max_grade: f64,

    /// Whether grade entry is allowed after cycle lock.
    #[serde(default)]
    pub allow_grade_entry_after_lock: bool,

    /// Whether grades can be edited after publishing.
    #[serde(default)]
    pub allow_grade_edit_after_publish: bool,
}

impl Default for GradingSettings {
    fn default() -> Self {
        Self {
            grading_system: default_grading_system(),
            passing_grade: default_passing_grade(),
            max_grade: default_max_grade(),
            allow_grade_entry_after_lock: false,
            allow_grade_edit_after_publish: false,
        }
    }
}

// ── Aggregate Org Settings ──────────────────────────────────────

/// All organization settings combined.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OrgSettings {
    pub registration: RegistrationSettings,
    pub finance: FinanceSettings,
    pub grading: GradingSettings,
}

// ── Default Helpers ─────────────────────────────────────────────

fn default_required_fields() -> Vec<String> {
    vec!["email".to_string(), "phone".to_string()]
}

fn default_prefix() -> String {
    "ST".to_string()
}

fn default_digits() -> usize {
    6
}

fn default_true() -> bool {
    true
}

fn default_currency() -> String {
    "SAR".to_string()
}

fn default_grading_system() -> String {
    "percentage".to_string()
}

fn default_passing_grade() -> f64 {
    50.0
}

fn default_max_grade() -> f64 {
    100.0
}
