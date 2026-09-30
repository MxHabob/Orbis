//! Onboarding Service — central orchestrator for comprehensive organization onboarding.
//!
//! Owns the validation and execution of the authoritative onboarding contract.
//! Replaces scattered, piecemeal frontend API loops with an atomic, transactional operation.

use std::sync::Arc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use tracing::info;
use orbis_domain::errors::DomainError;

use crate::ports::onboarding::{
    OnboardingPort, ValidatedOnboardingData, ValidatedOnboardingYear,
    ValidatedOnboardingPeriod, ValidatedOnboardingClass, ValidatedOnboardingSubject,
    ValidatedOnboardingTimeSlot, ValidatedOnboardingBuilding, ValidatedOnboardingRoom,
    ValidatedOnboardingFee, ValidatedOnboardingDiscount,
};
use crate::ports::repositories::SystemRepository;
use crate::services::system_service::{OrganizationReadiness, OrgLifecycleState};

// ── Public Request / Response Types ─────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompleteOrganizationOnboardingRequest {
    #[serde(default, alias = "org_id")]
    pub org_id: Option<String>,
    pub profile: OnboardingProfileDto,
    pub calendar: OnboardingCalendarDto,
    pub academics: OnboardingAcademicsDto,
    #[serde(default)]
    pub facilities: OnboardingFacilitiesDto,
    #[serde(default)]
    pub finance: OnboardingFinanceDto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OnboardingProfileDto {
    #[serde(alias = "institution_type")]
    pub institution_type: String,
    pub name: String,
    #[serde(default, alias = "name_ar")]
    pub name_ar: Option<String>,
    #[serde(alias = "base_currency_code")]
    pub base_currency_code: String,
    pub timezone: String,
    pub locale: String,
    #[serde(default, alias = "student_number_pattern")]
    pub student_number_pattern: Option<String>,
    #[serde(default, alias = "required_fields")]
    pub required_fields: Option<serde_json::Value>,
    #[serde(default, alias = "active_fields")]
    pub active_fields: Option<serde_json::Value>,
    #[serde(default, alias = "prevent_duplicates")]
    pub prevent_duplicates: Option<serde_json::Value>,
    #[serde(default, alias = "required_documents")]
    pub required_documents: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OnboardingCalendarDto {
    #[serde(alias = "academic_year_name")]
    pub academic_year_name: String,
    #[serde(default, alias = "academic_year_name_ar")]
    pub academic_year_name_ar: Option<String>,
    #[serde(alias = "start_date")]
    pub start_date: String,
    #[serde(alias = "end_date")]
    pub end_date: String,
    pub periods: Vec<OnboardingPeriodItemDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OnboardingPeriodItemDto {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default, alias = "parent_id")]
    pub parent_id: Option<String>,
    pub name: String,
    #[serde(default, alias = "name_ar")]
    pub name_ar: Option<String>,
    #[serde(alias = "period_type")]
    pub period_type: String,
    #[serde(default = "default_teaching", alias = "period_kind")]
    pub period_kind: String,
    #[serde(alias = "start_date")]
    pub start_date: String,
    #[serde(alias = "end_date")]
    pub end_date: String,
}

fn default_teaching() -> String { "teaching".to_string() }

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OnboardingAcademicsDto {
    pub classes: Vec<OnboardingClassItemDto>,
    pub subjects: Vec<OnboardingSubjectItemDto>,
    #[serde(default, alias = "time_slots")]
    pub time_slots: Vec<OnboardingTimeSlotItemDto>,
    #[serde(default, alias = "working_days")]
    pub working_days: Vec<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OnboardingClassItemDto {
    pub name: String,
    #[serde(default, alias = "name_ar")]
    pub name_ar: Option<String>,
    pub level: String,
    #[serde(default)]
    pub division: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OnboardingSubjectItemDto {
    pub code: String,
    pub name: String,
    #[serde(default, alias = "name_ar")]
    pub name_ar: Option<String>,
    #[serde(default = "default_true", alias = "is_core")]
    pub is_core: bool,
}

fn default_true() -> bool { true }

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OnboardingTimeSlotItemDto {
    pub name: String,
    #[serde(default, alias = "name_ar")]
    pub name_ar: Option<String>,
    #[serde(alias = "start_time")]
    pub start_time: String,
    #[serde(alias = "end_time")]
    pub end_time: String,
    #[serde(default, alias = "day_of_week")]
    pub day_of_week: Option<i32>,
    #[serde(default, alias = "is_break")]
    pub is_break: bool,
    #[serde(default, alias = "break_type")]
    pub break_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct OnboardingFacilitiesDto {
    #[serde(default)]
    pub buildings: Vec<String>,
    #[serde(default)]
    pub rooms: Vec<OnboardingRoomItemDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OnboardingRoomItemDto {
    pub name: String,
    #[serde(default, alias = "name_ar")]
    pub name_ar: Option<String>,
    #[serde(alias = "room_type")]
    pub room_type: String,
    pub capacity: i32,
    #[serde(default)]
    pub floor: Option<i32>,
    #[serde(default)]
    pub building: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct OnboardingFinanceDto {
    #[serde(default = "default_true", alias = "initialize_chart_of_accounts")]
    pub initialize_chart_of_accounts: bool,
    #[serde(default, alias = "tax_rate")]
    pub tax_rate: Option<f64>,
    #[serde(default, alias = "fee_templates")]
    pub fee_templates: Vec<OnboardingFeeItemDto>,
    #[serde(default, alias = "discount_templates")]
    pub discount_templates: Vec<OnboardingDiscountItemDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OnboardingFeeItemDto {
    pub code: String,
    pub name: String,
    #[serde(default, alias = "name_ar")]
    pub name_ar: Option<String>,
    pub category: String,
    #[serde(alias = "default_amount")]
    pub default_amount: f64,
    #[serde(default, alias = "applicable_levels")]
    pub applicable_levels: Option<Vec<String>>,
    pub frequency: String,
    #[serde(default = "default_true", alias = "is_mandatory")]
    pub is_mandatory: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OnboardingDiscountItemDto {
    pub code: String,
    pub name: String,
    #[serde(default, alias = "name_ar")]
    pub name_ar: Option<String>,
    #[serde(alias = "discount_type")]
    pub discount_type: String,
    pub value: f64,
    #[serde(default, alias = "is_approval_required")]
    pub is_approval_required: bool,
    #[serde(default, alias = "is_auto_apply")]
    pub is_auto_apply: bool,
    #[serde(default, alias = "is_stackable")]
    pub is_stackable: bool,
    #[serde(default)]
    pub priority: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OnboardingResult {
    pub success: bool,
    pub org_id: String,
    pub readiness: OrganizationReadiness,
}

// ── Onboarding Service ──────────────────────────────────────────────────────

pub struct OnboardingService {
    pub onboarding_port: Arc<dyn OnboardingPort>,
    pub system_repo: Arc<dyn SystemRepository>,
}

impl OnboardingService {
    pub fn new(
        onboarding_port: Arc<dyn OnboardingPort>,
        system_repo: Arc<dyn SystemRepository>,
    ) -> Self {
        Self {
            onboarding_port,
            system_repo,
        }
    }

    /// Complete full organization onboarding atomically.
    pub async fn complete_onboarding(
        &self,
        target_org_id: Uuid,
        request: CompleteOrganizationOnboardingRequest,
    ) -> Result<OnboardingResult, DomainError> {
        info!("Completing onboarding for org_id={}", target_org_id);

        // 1. Verify organization exists
        if !self.system_repo.has_organization().await? {
            return Err(DomainError::NotFound("Organization shell does not exist. Run bootstrap first.".into()));
        }

        // 2. Validate input constraints
        self.validate_request(&request)?;

        // 3. Build unified structured settings JSON
        let settings = serde_json::json!({
            "academic": {
                "default_grading_system": match request.profile.institution_type.as_str() {
                    "university" => "gpa_4_0",
                    "institute" => "pass_fail",
                    _ => "percentage",
                },
                "working_days": request.academics.working_days,
            },
            "enrollment": {
                "registration_enabled": true,
                "student_number_pattern": request.profile.student_number_pattern.clone().unwrap_or_else(|| "STD-{YYYY}-{####}".into()),
                "required_fields": request.profile.required_fields.clone().unwrap_or_else(|| serde_json::json!({})),
                "active_fields": request.profile.active_fields.clone().unwrap_or_else(|| serde_json::json!({})),
                "prevent_duplicates": request.profile.prevent_duplicates.clone().unwrap_or_else(|| serde_json::json!({})),
                "required_documents": request.profile.required_documents.clone().unwrap_or_default(),
            },
            "finance": {
                "base_currency": request.profile.base_currency_code.clone(),
                "tax_rate": request.finance.tax_rate.unwrap_or(0.15),
                "invoice_due_days": 14,
            },
            "notifications": {
                "notify_parents_on_absence": true,
                "notify_parents_on_grade_publish": true,
            },
            "autopilot": {
                "enabled": true,
                "mode": "automatic",
                "max_retries": 3,
            }
        });

        // 4. Materialize Validated Data Structures
        let year_id = Uuid::new_v4();
        let validated_year = ValidatedOnboardingYear {
            id: year_id,
            org_id: target_org_id,
            name: request.calendar.academic_year_name.clone(),
            name_ar: request.calendar.academic_year_name_ar.clone(),
            start_date: request.calendar.start_date.clone(),
            end_date: request.calendar.end_date.clone(),
        };

        let mut period_id_map: std::collections::HashMap<String, Uuid> = std::collections::HashMap::new();
        for (idx, p) in request.calendar.periods.iter().enumerate() {
            let key = p.id.clone().unwrap_or_else(|| format!("p_{idx}"));
            let assigned_id = p.id.as_deref().and_then(|s| Uuid::parse_str(s).ok()).unwrap_or_else(Uuid::new_v4);
            period_id_map.insert(key, assigned_id);
        }

        let validated_periods = request.calendar.periods.iter().enumerate().map(|(idx, p)| {
            let key = p.id.clone().unwrap_or_else(|| format!("p_{idx}"));
            let pid = *period_id_map.get(&key).unwrap();
            let parent_uuid = p.parent_id.as_deref().and_then(|pid_ref| {
                period_id_map.get(pid_ref).copied().or_else(|| Uuid::parse_str(pid_ref).ok())
            });

            ValidatedOnboardingPeriod {
                id: pid,
                org_id: target_org_id,
                year_id,
                parent_id: parent_uuid,
                name: p.name.clone(),
                name_ar: p.name_ar.clone(),
                period_type: p.period_type.clone(),
                period_kind: p.period_kind.clone(),
                start_date: p.start_date.clone(),
                end_date: p.end_date.clone(),
            }
        }).collect();

        let validated_classes = request.academics.classes.iter().map(|c| {
            ValidatedOnboardingClass {
                id: Uuid::new_v4(),
                org_id: target_org_id,
                name: c.name.clone(),
                name_ar: c.name_ar.clone(),
                level: c.level.clone(),
                division: c.division.clone(),
            }
        }).collect();

        let validated_subjects = request.academics.subjects.iter().map(|s| {
            ValidatedOnboardingSubject {
                id: Uuid::new_v4(),
                org_id: target_org_id,
                code: s.code.clone(),
                name: s.name.clone(),
                name_ar: s.name_ar.clone(),
                is_core: s.is_core,
            }
        }).collect();

        let validated_time_slots = request.academics.time_slots.iter().map(|ts| {
            ValidatedOnboardingTimeSlot {
                id: Uuid::new_v4(),
                org_id: target_org_id,
                name: ts.name.clone(),
                name_ar: ts.name_ar.clone(),
                start_time: ts.start_time.clone(),
                end_time: ts.end_time.clone(),
                day_of_week: ts.day_of_week.unwrap_or(0),
                is_break: ts.is_break,
                break_type: ts.break_type.clone(),
            }
        }).collect();

        let mut bld_map = std::collections::HashMap::new();
        let mut validated_buildings = Vec::new();
        for (idx, bld_name) in request.facilities.buildings.iter().enumerate() {
            let b_id = Uuid::new_v4();
            bld_map.insert(bld_name.clone(), b_id);
            validated_buildings.push(ValidatedOnboardingBuilding {
                id: b_id,
                org_id: target_org_id,
                parent_unit_id: None,
                name: bld_name.clone(),
                name_ar: Some(bld_name.clone()),
                code: format!("BLD{}", idx + 1),
            });
        }

        let validated_rooms = request.facilities.rooms.iter().map(|rm| {
            let parent_id = rm.building.as_ref().and_then(|b| bld_map.get(b).copied());
            ValidatedOnboardingRoom {
                id: Uuid::new_v4(),
                org_id: target_org_id,
                building_unit_id: parent_id,
                name: rm.name.clone(),
                name_ar: rm.name_ar.clone(),
                room_type: rm.room_type.clone(),
                capacity: rm.capacity,
                floor: rm.floor,
            }
        }).collect();

        let validated_fees = request.finance.fee_templates.iter().map(|f| {
            ValidatedOnboardingFee {
                id: Uuid::new_v4(),
                org_id: target_org_id,
                code: f.code.clone(),
                name: f.name.clone(),
                name_ar: f.name_ar.clone().unwrap_or_else(|| f.name.clone()),
                category: f.category.clone(),
                default_amount: f.default_amount,
                currency_code: request.profile.base_currency_code.clone(),
                applicable_levels: f.applicable_levels.clone(),
                frequency: f.frequency.clone(),
                is_mandatory: f.is_mandatory,
            }
        }).collect();

        let validated_discounts = request.finance.discount_templates.iter().map(|d| {
            ValidatedOnboardingDiscount {
                id: Uuid::new_v4(),
                org_id: target_org_id,
                code: d.code.clone(),
                name: d.name.clone(),
                name_ar: d.name_ar.clone().unwrap_or_else(|| d.name.clone()),
                discount_type: d.discount_type.clone(),
                value: d.value,
                is_approval_required: d.is_approval_required,
                is_auto_apply: d.is_auto_apply,
                is_stackable: d.is_stackable,
                priority: d.priority,
            }
        }).collect();

        let validated_data = ValidatedOnboardingData {
            org_id: target_org_id,
            org_type: request.profile.institution_type.clone(),
            base_currency_code: request.profile.base_currency_code.clone(),
            timezone: request.profile.timezone.clone(),
            locale: request.profile.locale.clone(),
            settings,
            academic_year: validated_year,
            periods: validated_periods,
            classes: validated_classes,
            subjects: validated_subjects,
            time_slots: validated_time_slots,
            buildings: validated_buildings,
            rooms: validated_rooms,
            fee_templates: validated_fees,
            discount_templates: validated_discounts,
            initialize_chart_of_accounts: request.finance.initialize_chart_of_accounts,
        };

        // 5. Delegate to Infrastructure for transactional execution
        self.onboarding_port.persist_onboarding(validated_data).await?;

        info!("Onboarding completed successfully for org_id={}", target_org_id);

        Ok(OnboardingResult {
            success: true,
            org_id: target_org_id.to_string(),
            readiness: OrganizationReadiness {
                state: OrgLifecycleState::Operational,
                org_id: Some(target_org_id.to_string()),
                org_name: Some(request.profile.name),
                has_organization: true,
                has_admin: true,
                has_academic_year: true,
                has_classes: true,
                has_subjects: true,
                pending_setup_items: vec![],
            },
        })
    }

    fn validate_request(&self, req: &CompleteOrganizationOnboardingRequest) -> Result<(), DomainError> {
        if req.calendar.academic_year_name.trim().is_empty() {
            return Err(DomainError::Validation("Academic Year Name is required".into()));
        }
        if req.calendar.start_date.trim().is_empty() || req.calendar.end_date.trim().is_empty() {
            return Err(DomainError::Validation("Academic Year Start and End dates are required".into()));
        }
        if req.calendar.periods.is_empty() {
            return Err(DomainError::Validation("At least one academic period/semester is required".into()));
        }
        if req.academics.classes.is_empty() {
            return Err(DomainError::Validation("At least one class is required".into()));
        }
        if req.academics.subjects.is_empty() {
            return Err(DomainError::Validation("At least one subject is required".into()));
        }
        Ok(())
    }
}
