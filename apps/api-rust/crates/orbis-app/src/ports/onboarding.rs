//! Onboarding port — Application-layer contract for complete organization onboarding persistence.
//!
//! Owns the transactional materialization of administrator decisions made during the
//! Onboarding Wizard (Calendar, Academics, Facilities, Finance, Registration & Policies).

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use orbis_domain::errors::DomainError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatedOnboardingData {
    pub org_id: Uuid,
    pub org_type: String,
    pub base_currency_code: String,
    pub timezone: String,
    pub locale: String,
    pub settings: serde_json::Value,
    pub academic_year: ValidatedOnboardingYear,
    pub periods: Vec<ValidatedOnboardingPeriod>,
    pub classes: Vec<ValidatedOnboardingClass>,
    pub subjects: Vec<ValidatedOnboardingSubject>,
    pub time_slots: Vec<ValidatedOnboardingTimeSlot>,
    pub buildings: Vec<ValidatedOnboardingBuilding>,
    pub rooms: Vec<ValidatedOnboardingRoom>,
    pub fee_templates: Vec<ValidatedOnboardingFee>,
    pub discount_templates: Vec<ValidatedOnboardingDiscount>,
    pub initialize_chart_of_accounts: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatedOnboardingYear {
    pub id: Uuid,
    pub org_id: Uuid,
    pub name: String,
    pub name_ar: Option<String>,
    pub start_date: String,
    pub end_date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatedOnboardingPeriod {
    pub id: Uuid,
    pub org_id: Uuid,
    pub year_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub name: String,
    pub name_ar: Option<String>,
    pub period_type: String,
    pub start_date: String,
    pub end_date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatedOnboardingClass {
    pub id: Uuid,
    pub org_id: Uuid,
    pub name: String,
    pub name_ar: Option<String>,
    pub level: String,
    pub division: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatedOnboardingSubject {
    pub id: Uuid,
    pub org_id: Uuid,
    pub code: String,
    pub name: String,
    pub name_ar: Option<String>,
    pub is_core: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatedOnboardingTimeSlot {
    pub id: Uuid,
    pub org_id: Uuid,
    pub name: String,
    pub name_ar: Option<String>,
    pub start_time: String,
    pub end_time: String,
    pub day_of_week: i32,
    pub is_break: bool,
    pub break_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatedOnboardingBuilding {
    pub id: Uuid,
    pub org_id: Uuid,
    pub parent_unit_id: Option<Uuid>,
    pub name: String,
    pub name_ar: Option<String>,
    pub code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatedOnboardingRoom {
    pub id: Uuid,
    pub org_id: Uuid,
    pub building_unit_id: Option<Uuid>,
    pub name: String,
    pub name_ar: Option<String>,
    pub room_type: String,
    pub capacity: i32,
    pub floor: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatedOnboardingFee {
    pub id: Uuid,
    pub org_id: Uuid,
    pub code: String,
    pub name: String,
    pub name_ar: String,
    pub category: String,
    pub default_amount: f64,
    pub currency_code: String,
    pub applicable_levels: Option<Vec<String>>,
    pub frequency: String,
    pub is_mandatory: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatedOnboardingDiscount {
    pub id: Uuid,
    pub org_id: Uuid,
    pub code: String,
    pub name: String,
    pub name_ar: String,
    pub discount_type: String,
    pub value: f64,
    pub is_approval_required: bool,
    pub is_auto_apply: bool,
    pub is_stackable: bool,
    pub priority: i32,
}

#[async_trait]
pub trait OnboardingPort: Send + Sync {
    async fn persist_onboarding(
        &self,
        data: ValidatedOnboardingData,
    ) -> Result<(), DomainError>;
}
