use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OrganizationSettings {
    #[serde(default)]
    pub academic: AcademicSettings,
    #[serde(default)]
    pub enrollment: EnrollmentSettings,
    #[serde(default)]
    pub finance: FinanceSettings,
    #[serde(default)]
    pub notifications: NotificationSettings,
    #[serde(default)]
    pub autopilot: AutopilotSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcademicSettings {
    pub default_grading_system: String, // e.g., "percentage", "letter", "gpa"
}

impl Default for AcademicSettings {
    fn default() -> Self {
        Self {
            default_grading_system: "percentage".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnrollmentSettings {
    pub registration_enabled: bool,
    pub require_admin_approval: bool,
    pub allow_waitlist: bool,
    pub sibling_priority_enabled: bool,
    pub max_class_size_default: i32,
    pub allow_late_enrollment: bool,
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
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
pub struct NotificationSettings {
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
pub struct AutopilotSettings {
    pub enabled: bool,
    pub mode: String, // "automatic", "suggested", "manual"
    pub max_retries: i32,
    pub auto_advance_periods: bool,
    pub auto_lock_grades: bool,
    pub grade_entry_auto_close_days: i32,
    pub allow_grade_edit_after_close: bool,
}

impl Default for AutopilotSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            mode: "automatic".to_string(),
            max_retries: 3,
            auto_advance_periods: false,
            auto_lock_grades: false,
            grade_entry_auto_close_days: 7,
            allow_grade_edit_after_close: false,
        }
    }
}
