use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::academic_calculation::{CalculationPlan, OutcomePolicy};
use super::academic_policy::{AcademicPolicy, RankingScope, RoundingMode, TiePolicy};
use super::admission::AdmissionConfiguration;
use rust_decimal::Decimal;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OrganizationSettings {
    #[serde(default)]
    pub academic: AcademicSettings,
    #[serde(default)]
    pub enrollment: EnrollmentSettings,
    #[serde(default, alias = "registration")]
    pub admission: AdmissionConfiguration,
    #[serde(default)]
    pub finance: FinanceSettings,
    #[serde(default)]
    pub notifications: NotificationSettings,
    #[serde(default)]
    pub autopilot: AutopilotSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AcademicSettings {
    pub default_grading_system: String, // e.g., "percentage", "letter", "gpa"
    pub default_passing_score: Decimal,
    pub default_max_score: Decimal,
    pub passing_threshold_percentage: Decimal,
    pub honors_threshold_percentage: Decimal,
    pub minimum_final_exam_score: Option<Decimal>,
    pub max_failed_subjects_for_conditional_pass: usize,
    pub min_attendance_rate: Decimal,
    pub ranking_enabled: bool,
    pub max_displayed_rank: Option<u32>,
    pub ranking_scope: RankingScope,
    pub tie_policy: TiePolicy,
    pub rounding_mode: RoundingMode,
    pub decimal_places: u32,
    #[serde(default)]
    pub calculation_plan: Option<CalculationPlan>,
    #[serde(default)]
    pub outcome_policy: Option<OutcomePolicy>,
    pub enrollment_auto_close: bool,
    pub allow_late_enrollment: bool,
    pub late_enrollment_approval: bool,
    pub grade_entry_auto_close: bool,
    pub allow_grade_edit_after_close: bool,
    pub grade_edit_approval: bool,
    pub auto_graduate_final_level: bool,
    pub exclude_withdrawn: bool,
}

impl Default for AcademicSettings {
    fn default() -> Self {
        Self {
            default_grading_system: "percentage".to_string(),
            default_passing_score: Decimal::from(50),
            default_max_score: Decimal::from(100),
            passing_threshold_percentage: Decimal::from(50),
            honors_threshold_percentage: Decimal::from(85),
            minimum_final_exam_score: Some(Decimal::from(10)),
            max_failed_subjects_for_conditional_pass: 2,
            min_attendance_rate: Decimal::from(75),
            ranking_enabled: true,
            max_displayed_rank: None,
            ranking_scope: RankingScope::Class,
            tie_policy: TiePolicy::StandardCompetition,
            rounding_mode: RoundingMode::RoundHalfUp,
            decimal_places: 2,
            calculation_plan: None,
            outcome_policy: None,
            enrollment_auto_close: true,
            allow_late_enrollment: false,
            late_enrollment_approval: true,
            grade_entry_auto_close: true,
            allow_grade_edit_after_close: false,
            grade_edit_approval: true,
            auto_graduate_final_level: false,
            exclude_withdrawn: true,
        }
    }
}

impl AcademicSettings {
    pub fn to_academic_policy(&self, org_id: Uuid) -> AcademicPolicy {
        AcademicPolicy {
            org_id,
            passing_threshold_percentage: self.passing_threshold_percentage,
            honors_threshold_percentage: self.honors_threshold_percentage,
            minimum_final_exam_score: self.minimum_final_exam_score,
            max_failed_subjects_for_conditional_pass: self.max_failed_subjects_for_conditional_pass,
            rounding_mode: self.rounding_mode,
            decimal_places: self.decimal_places,
            ranking_scope: self.ranking_scope,
            tie_policy: self.tie_policy,
            ranking_enabled: self.ranking_enabled,
            max_displayed_rank: self.max_displayed_rank,
            coursework_max_score: Decimal::from(20),
            exam_max_score: Decimal::from(30),
            term_max_score: Decimal::from(50),
            annual_max_score: Decimal::from(100),
            certificate_exam_max_score: Decimal::from(80),
            certificate_cumulative_max_score: Decimal::from(20),
            certificate_cumulative_year_count: 3,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct EnrollmentSettings {
    pub registration_enabled: bool,
    pub require_admin_approval: bool,
    pub allow_waitlist: bool,
    pub sibling_priority_enabled: bool,
    pub max_class_size_default: i32,
    pub allow_late_enrollment: bool,
    pub is_configured: bool,
}

impl Default for EnrollmentSettings {
    fn default() -> Self {
        Self {
            registration_enabled: true,
            require_admin_approval: true,
            allow_waitlist: true,
            sibling_priority_enabled: true,
            max_class_size_default: 25,
            allow_late_enrollment: false,
            is_configured: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct FinanceSettings {
    pub currency: String,
    pub tax_rate: f64,
    pub invoice_due_days: i32,
    pub late_fee_type: String, // "fixed" or "percentage"
    pub late_fee_value: f64,
    pub allow_partial_payments: bool,
    /// Integration UUID for the primary payment gateway (e.g. Mada, Stripe, Moyasar)
    pub primary_payment_gateway_id: Option<Uuid>,
    /// Integration UUID for B2B/B2C invoicing (e.g. Orbis Finance, Xero)
    pub invoicing_integration_id: Option<Uuid>,
}

impl Default for FinanceSettings {
    fn default() -> Self {
        Self {
            currency: "SAR".to_string(),
            tax_rate: 0.15,
            invoice_due_days: 14,
            late_fee_type: "fixed".to_string(),
            late_fee_value: 0.0,
            allow_partial_payments: false,
            primary_payment_gateway_id: None,
            invoicing_integration_id: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct NotificationSettings {
    #[serde(default)]
    pub default_integration_ids: HashMap<String, Uuid>,
    /// Default Integration UUID for Email delivery (e.g. SendGrid, Amazon SES)
    pub email_integration_id: Option<Uuid>,
    /// Default Integration UUID for SMS delivery (e.g. Twilio, Unifonic)
    pub sms_integration_id: Option<Uuid>,
    /// Default Integration UUID for WhatsApp delivery (e.g. WhatsApp Business API, Infobip)
    pub whatsapp_integration_id: Option<Uuid>,

    pub notify_parents_on_absence: bool,
    pub notify_parents_on_grade_publish: bool,
    pub finance_reminder_days: Vec<i32>,
}

impl Default for NotificationSettings {
    fn default() -> Self {
        Self {
            default_integration_ids: HashMap::new(),
            email_integration_id: None,
            sms_integration_id: None,
            whatsapp_integration_id: None,
            notify_parents_on_absence: true,
            notify_parents_on_grade_publish: true,
            finance_reminder_days: vec![7, 3, 1],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AutopilotSettings {
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default = "default_mode")]
    pub mode: String, // "automatic", "suggested", "manual"
    #[serde(default)]
    pub auto_open_enrollment: bool,
    #[serde(default = "default_true")]
    pub auto_close_enrollment: bool,
    #[serde(default = "default_true")]
    pub auto_record_attendance: bool,
    #[serde(default = "default_true")]
    pub auto_fee_assignment: bool,
    #[serde(default = "default_true")]
    pub auto_grade_entry_reminder: bool,
    #[serde(default)]
    pub auto_period_lock: bool,
    #[serde(default)]
    pub auto_promotion: bool,
    #[serde(default)]
    pub auto_exam_scheduling: bool,
    #[serde(default = "default_true")]
    pub auto_report_generation: bool,
    #[serde(default = "default_grace_days")]
    pub grace_days: i32,
    #[serde(default = "default_lead_hours")]
    pub notify_before_deadline_hours: i32,
    #[serde(default = "default_max_retries")]
    pub max_retries: i32,
    #[serde(default)]
    pub auto_advance_periods: bool,
    #[serde(default)]
    pub auto_lock_grades: bool,
    #[serde(default = "default_grade_entry_auto_close_days")]
    pub grade_entry_auto_close_days: i32,
    #[serde(default)]
    pub allow_grade_edit_after_close: bool,
}

fn default_true() -> bool {
    true
}

fn default_mode() -> String {
    "suggested".to_string()
}

fn default_grace_days() -> i32 {
    3
}

fn default_lead_hours() -> i32 {
    24
}

fn default_max_retries() -> i32 {
    3
}

fn default_grade_entry_auto_close_days() -> i32 {
    7
}

impl Default for AutopilotSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            mode: "suggested".to_string(),
            auto_open_enrollment: false,
            auto_close_enrollment: true,
            auto_record_attendance: true,
            auto_fee_assignment: true,
            auto_grade_entry_reminder: true,
            auto_period_lock: false,
            auto_promotion: false,
            auto_exam_scheduling: false,
            auto_report_generation: true,
            grace_days: 3,
            notify_before_deadline_hours: 24,
            max_retries: 3,
            auto_advance_periods: false,
            auto_lock_grades: false,
            grade_entry_auto_close_days: 7,
            allow_grade_edit_after_close: false,
        }
    }
}
