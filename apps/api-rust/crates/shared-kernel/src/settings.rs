//! Organization settings types — shared across all microservices.
//!
//! These types are the Single Source of Truth for org-level configuration.
//! `svc-organization` populates them; every other service reads them via `SettingsProvider`.

use serde::{Deserialize, Serialize};

// ── Registration Settings ───────────────────────────────────────

/// Controls every aspect of student registration across the platform.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistrationSettings {
    /// Institution type ("school", "university", "institute", "training_center", "custom").
    #[serde(default = "default_institution_type")]
    pub institution_type: String,

    /// Custom student numbering pattern template (e.g. "STU-{YEAR}-{SEQ}").
    #[serde(default = "default_number_pattern")]
    pub student_number_pattern: String,

    /// Academic stages list.
    #[serde(default = "default_stages")]
    pub stages: Vec<String>,

    /// Fields active on registration form.
    #[serde(default = "default_active_fields")]
    pub active_fields: Vec<String>,

    /// Required fields for registration.
    #[serde(default = "default_required_fields")]
    pub required_fields: Vec<String>,

    /// Fields that prevent duplicate registration.
    #[serde(default = "default_prevent_duplicates")]
    pub prevent_duplicates: Vec<String>,

    /// List of required document names.
    #[serde(default = "default_required_documents")]
    pub required_documents: Vec<String>,

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
            institution_type: default_institution_type(),
            student_number_pattern: default_number_pattern(),
            stages: default_stages(),
            active_fields: default_active_fields(),
            required_fields: default_required_fields(),
            prevent_duplicates: default_prevent_duplicates(),
            required_documents: default_required_documents(),
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

    /// Whether grade entry is allowed after period lock.
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
    #[serde(default)]
    pub reporting: ReportingSettings,
}

// ── Reporting Settings ──────────────────────────────────────────

/// Controls report generation behavior for an organization.
///
/// Stored as part of `OrgSettings` in `organizations.settings` JSON.
/// Read by `SettingsProvider` and used when resolving report `OrgContext`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportingSettings {
    /// Header style: "standard" (logo + name + address), "minimal" (name only),
    /// "custom" (user-defined template component).
    #[serde(default = "default_header_style")]
    pub header_style: String,

    /// Whether to show the organization logo in report headers.
    #[serde(default = "default_true")]
    pub show_logo: bool,

    /// Whether to show the address in report headers.
    #[serde(default)]
    pub show_address: bool,

    /// Whether to show contact info (phone/email) in report headers.
    #[serde(default)]
    pub show_contact: bool,

    /// Organization address (English).
    #[serde(default)]
    pub address: Option<String>,

    /// Organization address (Arabic).
    #[serde(default)]
    pub address_ar: Option<String>,

    /// Phone number for report headers.
    #[serde(default)]
    pub phone: Option<String>,

    /// Email for report headers.
    #[serde(default)]
    pub email: Option<String>,

    /// Website for report headers.
    #[serde(default)]
    pub website: Option<String>,

    /// Tax identification number.
    #[serde(default)]
    pub tax_id: Option<String>,

    /// Registration / license number.
    #[serde(default)]
    pub registration_number: Option<String>,

    /// Asset ID of the official stamp image.
    #[serde(default)]
    pub stamp_asset_id: Option<String>,

    /// Asset ID of the authorized signature image.
    #[serde(default)]
    pub signature_asset_id: Option<String>,

    /// Default report locale: "ar", "en", "ar-en", "en-ar".
    #[serde(default = "default_report_locale")]
    pub default_locale: String,

    /// Number format: "arabic-indic" (٠١٢٣) or "western" (0123).
    #[serde(default = "default_number_format")]
    pub number_format: String,

    /// Date display format.
    #[serde(default = "default_date_format")]
    pub date_format: String,
}

impl Default for ReportingSettings {
    fn default() -> Self {
        Self {
            header_style: default_header_style(),
            show_logo: true,
            show_address: false,
            show_contact: false,
            address: None,
            address_ar: None,
            phone: None,
            email: None,
            website: None,
            tax_id: None,
            registration_number: None,
            stamp_asset_id: None,
            signature_asset_id: None,
            default_locale: default_report_locale(),
            number_format: default_number_format(),
            date_format: default_date_format(),
        }
    }
}

// ── Default Helpers ─────────────────────────────────────────────

fn default_institution_type() -> String {
    "school".to_string()
}

fn default_number_pattern() -> String {
    "STU-{YEAR}-{SEQ}".to_string()
}

fn default_stages() -> Vec<String> {
    vec!["primary".to_string(), "middle".to_string(), "high".to_string()]
}

fn default_active_fields() -> Vec<String> {
    vec![
        "national_id".to_string(),
        "email".to_string(),
        "phone".to_string(),
        "birth_date".to_string(),
        "sex".to_string(),
        "address".to_string(),
        "nationality".to_string(),
        "place_of_birth".to_string(),
        "passport_number".to_string(),
        "blood_type".to_string(),
        "emergency_contact".to_string(),
        "medical_notes".to_string(),
        "previous_school".to_string(),
        "documents".to_string(),
    ]
}

fn default_required_fields() -> Vec<String> {
    vec![
        "national_id".to_string(),
        "phone".to_string(),
        "birth_date".to_string(),
        "sex".to_string(),
    ]
}

fn default_prevent_duplicates() -> Vec<String> {
    vec!["national_id".to_string(), "phone".to_string()]
}

fn default_required_documents() -> Vec<String> {
    vec![
        "Birth Certificate".to_string(),
        "National ID or Passport".to_string(),
    ]
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

fn default_header_style() -> String {
    "standard".to_string()
}

fn default_report_locale() -> String {
    "ar".to_string()
}

fn default_number_format() -> String {
    "western".to_string()
}

fn default_date_format() -> String {
    "YYYY/MM/DD".to_string()
}

