//! Organization routes — Org CRUD, OrgUnits, Settings, Dashboard, Bootstrap.

use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::ToSchema;
use uuid::Uuid;

use super::extractors::AuthContext;
use super::{ApiError, ApiResponse};
use crate::state::AppState;
use orbis_domain::entities::academic_policy::{RankingScope, RoundingMode, TiePolicy};
use orbis_domain::entities::settings::OrganizationSettings;
use rust_decimal::Decimal;

// ── DTOs ────────────────────────────────────────────────────────

#[derive(Deserialize, ToSchema)]
pub struct CreateOrgRequest {
    #[schema(example = "Orbis High School")]
    pub name: String,
    #[schema(example = "orbis-high")]
    pub slug: String,
    #[schema(example = "Asia/Riyadh")]
    pub timezone: String,
    #[schema(example = "ar-SA")]
    pub locale: String,
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateOrgRequest {
    pub name: Option<String>,
    pub name_ar: Option<String>,
    pub code: Option<String>,
    pub timezone: Option<String>,
    pub locale: Option<String>,
    pub date_format: Option<String>,
    pub format_locale: Option<String>,
    pub time_format: Option<String>,
    pub is_hijri_enabled: Option<bool>,
    pub base_currency_code: Option<String>,
    pub primary_color: Option<String>,
    pub logo: Option<String>,
    pub favicon: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Serialize, ToSchema)]
pub struct OrgDto {
    pub id: Uuid,
    pub name: String,
    pub name_ar: Option<String>,
    pub slug: String,
    pub code: Option<String>,
    pub timezone: String,
    pub locale: String,
    pub date_format: Option<String>,
    pub format_locale: Option<String>,
    pub time_format: Option<String>,
    pub is_hijri_enabled: bool,
    pub base_currency_code: Option<String>,
    pub primary_color: Option<String>,
    pub logo: Option<String>,
    pub favicon: Option<String>,
    pub current_period_id: Option<String>,
    pub is_active: bool,
    pub is_verified: bool,
    pub settings: Option<serde_json::Value>,
    pub features: Option<serde_json::Value>,
    pub institution_type: String,
    pub metadata: Option<serde_json::Value>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub version: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

fn org_to_dto(org: orbis_domain::entities::organization::Organization) -> OrgDto {
    let institution_type = org
        .metadata
        .as_ref()
        .and_then(|m| m.get("institution_type"))
        .and_then(|v| v.as_str())
        .unwrap_or("school")
        .to_string();

    OrgDto {
        id: org.id,
        name: org.name,
        name_ar: org.name_ar,
        slug: org.slug,
        code: org.code,
        timezone: org.timezone,
        locale: org.locale,
        date_format: org.date_format,
        format_locale: org.format_locale,
        time_format: org.time_format,
        is_hijri_enabled: org.is_hijri_enabled,
        base_currency_code: org.base_currency_code,
        primary_color: org.primary_color,
        logo: org.logo,
        favicon: org.favicon,
        current_period_id: org.current_period_id,
        is_active: org.is_active,
        is_verified: org.is_verified,
        settings: org.settings,
        features: org.features,
        institution_type,
        metadata: org.metadata,
        created_by: org.created_by,
        updated_by: org.updated_by,
        version: org.version,
        created_at: org.created_at,
        updated_at: org.updated_at,
    }
}

#[derive(Deserialize, ToSchema)]
pub struct CreateOrgUnitRequest {
    #[schema(example = "Main Campus")]
    pub name: String,
    #[schema(example = "الفرع الرئيسي")]
    pub name_ar: Option<String>,
    #[schema(example = "BR-01")]
    pub code: Option<String>,
    #[schema(example = "branch")]
    pub unit_type: String,
    pub parent_id: Option<Uuid>,
    pub head_id: Option<Uuid>,
    pub capacity: Option<i32>,
    pub building: Option<String>,
    pub floor: Option<i32>,
    pub room_number: Option<String>,
    pub settings: Option<serde_json::Value>,
    pub local_policies: Option<serde_json::Value>,
    #[schema(example = "shared")]
    pub data_isolation: Option<String>,
    pub sort_order: Option<i32>,
    pub is_active: Option<bool>,
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateOrgUnitRequest {
    pub name: Option<String>,
    pub name_ar: Option<String>,
    pub code: Option<String>,
    pub unit_type: Option<String>,
    pub parent_id: Option<Uuid>,
    pub head_id: Option<Uuid>,
    pub capacity: Option<i32>,
    pub building: Option<String>,
    pub floor: Option<i32>,
    pub room_number: Option<String>,
    pub settings: Option<serde_json::Value>,
    pub local_policies: Option<serde_json::Value>,
    pub data_isolation: Option<String>,
    pub sort_order: Option<i32>,
    pub is_active: Option<bool>,
}

#[derive(Serialize, ToSchema)]
pub struct OrgUnitDto {
    pub id: Uuid,
    pub org_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub path: String,
    pub depth: i32,
    pub name: String,
    pub name_ar: Option<String>,
    pub code: Option<String>,
    pub unit_type: String,
    pub head_id: Option<Uuid>,
    pub capacity: Option<i32>,
    pub building: Option<String>,
    pub floor: Option<i32>,
    pub room_number: Option<String>,
    pub settings: Option<serde_json::Value>,
    pub local_policies: Option<serde_json::Value>,
    pub data_isolation: String,
    pub children_count: i32,
    pub descendant_count: i32,
    pub sort_order: Option<i32>,
    pub is_active: bool,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

fn unit_to_dto(u: orbis_domain::entities::org_unit::OrgUnit) -> OrgUnitDto {
    OrgUnitDto {
        id: u.id,
        org_id: u.org_id,
        parent_id: u.parent_id,
        path: u.path,
        depth: u.depth,
        name: u.name,
        name_ar: u.name_ar,
        code: u.code,
        unit_type: u.unit_type,
        head_id: u.head_id,
        capacity: u.capacity,
        building: u.building,
        floor: u.floor,
        room_number: u.room_number,
        settings: u.settings,
        local_policies: u.local_policies,
        data_isolation: u.data_isolation,
        children_count: u.children_count,
        descendant_count: u.descendant_count,
        sort_order: u.sort_order,
        is_active: u.is_active,
        created_by: u.created_by,
        updated_by: u.updated_by,
        created_at: u.created_at,
        updated_at: u.updated_at,
    }
}

// ── Handlers ────────────────────────────────────────────────────

#[utoipa::path(post, path = "/api/v1/organizations", request_body = CreateOrgRequest,
    responses((status = 201, description = "Organization created", body = OrgDto)),
    tag = "Organization")]
async fn create_org(
    State(state): State<AppState>,
    Json(payload): Json<CreateOrgRequest>,
) -> Result<Json<ApiResponse<OrgDto>>, ApiError> {
    let ctx = orbis_app::context::RequestContext {
        tenant_id: Uuid::new_v4(),
        actor: orbis_domain::iam::actor::Actor::System,
        request_id: Uuid::new_v4().to_string(),
        source: orbis_app::context::ActionSource::Api,
        org_unit_path: None,
    };

    let org = state
        .organization_service
        .create_organization(
            &ctx,
            payload.name,
            payload.slug,
            payload.timezone,
            payload.locale,
        )
        .await?;

    Ok(ApiResponse::ok(org_to_dto(org)))
}

#[utoipa::path(get, path = "/api/v1/organizations/current",
    responses((status = 200, description = "Current organization", body = OrgDto)),
    security(("bearer" = [])), tag = "Organization")]
async fn get_current_org(
    ctx: AuthContext,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<OrgDto>>, ApiError> {
    let org = state
        .organization_service
        .get_organization(ctx.tenant_id)
        .await?
        .ok_or_else(|| ApiError::not_found("Organization not found"))?;

    Ok(ApiResponse::ok(org_to_dto(org)))
}

#[utoipa::path(put, path = "/api/v1/organizations/current", request_body = UpdateOrgRequest,
    responses((status = 200, description = "Organization updated", body = OrgDto)),
    security(("bearer" = [])), tag = "Organization")]
async fn update_current_org(
    ctx: AuthContext,
    State(state): State<AppState>,
    Json(payload): Json<UpdateOrgRequest>,
) -> Result<Json<ApiResponse<OrgDto>>, ApiError> {
    let org = state
        .organization_service
        .update_organization(
            &ctx,
            ctx.tenant_id,
            orbis_app::services::UpdateOrgRequest {
                name: payload.name,
                name_ar: payload.name_ar,
                code: payload.code,
                timezone: payload.timezone,
                locale: payload.locale,
                date_format: payload.date_format,
                format_locale: payload.format_locale,
                time_format: payload.time_format,
                is_hijri_enabled: payload.is_hijri_enabled,
                base_currency_code: payload.base_currency_code,
                primary_color: payload.primary_color,
                logo: payload.logo,
                favicon: payload.favicon,
                metadata: payload.metadata,
            },
        )
        .await?;

    Ok(ApiResponse::ok(org_to_dto(org)))
}

#[utoipa::path(get, path = "/api/v1/system/readiness",
    responses((status = 200, description = "Get system readiness")),
    tag = "System")]
pub async fn get_system_readiness_endpoint(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<orbis_app::services::OrganizationReadiness>>, ApiError> {
    let res = state
        .system_service
        .get_readiness()
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?;
    Ok(ApiResponse::ok(res))
}

#[utoipa::path(get, path = "/api/v1/system/has-organization",
    responses((status = 200, description = "Check if organization exists")),
    tag = "System")]
pub async fn get_system_has_org_endpoint(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<serde_json::Value>>, ApiError> {
    let has_org = state
        .system_service
        .has_organization()
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?;
    Ok(ApiResponse::ok(
        serde_json::json!({ "has_organization": has_org }),
    ))
}

// ── Settings DTOs ───────────────────────────────────────────────

#[derive(Serialize, Deserialize, ToSchema, Default)]
pub struct OrganizationSettingsDto {
    #[serde(default)]
    pub academic: AcademicSettingsDto,
    #[serde(default)]
    pub enrollment: EnrollmentSettingsDto,
    #[serde(default, alias = "registration")]
    pub admission: AdmissionConfigurationDto,
    #[serde(default)]
    pub finance: FinanceSettingsDto,
    #[serde(default)]
    pub notifications: NotificationSettingsDto,
    #[serde(default)]
    pub autopilot: AutopilotSettingsDto,
}

#[derive(Serialize, Deserialize, ToSchema)]
#[serde(default)]
pub struct AcademicSettingsDto {
    pub default_grading_system: String,
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
    pub enrollment_auto_close: bool,
    pub allow_late_enrollment: bool,
    pub late_enrollment_approval: bool,
    pub grade_entry_auto_close: bool,
    pub allow_grade_edit_after_close: bool,
    pub grade_edit_approval: bool,
    pub auto_graduate_final_level: bool,
    pub exclude_withdrawn: bool,
}

impl Default for AcademicSettingsDto {
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

#[derive(Serialize, Deserialize, ToSchema, Default)]
pub struct EnrollmentSettingsDto {
    pub registration_enabled: bool,
    pub require_admin_approval: bool,
    pub allow_waitlist: bool,
    pub sibling_priority_enabled: bool,
    pub max_class_size_default: i32,
    pub allow_late_enrollment: bool,
    #[serde(default)]
    pub is_configured: bool,
}

#[derive(Serialize, Deserialize, ToSchema, Default)]
pub struct NumberingSchemaDto {
    pub prefix: String,
    pub year_format: String,
    pub sequence_digits: u8,
    pub separator: String,
}

#[derive(Serialize, Deserialize, ToSchema, Default)]
pub struct AdmissionPolicyDto {
    pub requires_approval: bool,
    pub allow_no_guardian: bool,
    pub allow_duplicate_national_id: bool,
    pub auto_assign_fees: bool,
    pub max_class_capacity: i32,
    pub allow_late_enrollment: bool,
    pub allow_waitlist: bool,
    pub sibling_priority_enabled: bool,
}

#[derive(Serialize, Deserialize, ToSchema, Default)]
pub struct FormFieldConfigDto {
    pub key: String,
    pub name_ar: String,
    pub name_en: String,
    pub is_active: bool,
    pub is_required: bool,
    pub prevent_duplicate: bool,
    pub supports_duplicate: bool,
}

#[derive(Serialize, Deserialize, ToSchema, Default)]
pub struct AdmissionConfigurationDto {
    #[serde(default)]
    pub numbering: NumberingSchemaDto,
    #[serde(default)]
    pub policy: AdmissionPolicyDto,
    #[serde(default)]
    pub fields: Vec<FormFieldConfigDto>,
}

#[derive(Serialize, Deserialize, ToSchema, Default)]
pub struct FinanceSettingsDto {
    pub currency: String,
    pub tax_rate: f64,
    pub invoice_due_days: i32,
    pub late_fee_type: String,
    pub late_fee_value: f64,
    pub allow_partial_payments: bool,
    pub primary_payment_gateway_id: Option<Uuid>,
    pub invoicing_integration_id: Option<Uuid>,
}

#[derive(Serialize, Deserialize, ToSchema, Default)]
pub struct NotificationSettingsDto {
    #[serde(default)]
    pub default_integration_ids: HashMap<String, Uuid>,
    pub email_integration_id: Option<Uuid>,
    pub sms_integration_id: Option<Uuid>,
    pub whatsapp_integration_id: Option<Uuid>,
    pub notify_parents_on_absence: bool,
    pub notify_parents_on_grade_publish: bool,
    pub finance_reminder_days: Vec<i32>,
}

#[derive(Serialize, Deserialize, ToSchema, Default)]
pub struct AutopilotSettingsDto {
    pub enabled: bool,
    pub mode: String,
    pub auto_open_enrollment: bool,
    pub auto_close_enrollment: bool,
    pub auto_record_attendance: bool,
    pub auto_fee_assignment: bool,
    pub auto_grade_entry_reminder: bool,
    pub auto_period_lock: bool,
    pub auto_promotion: bool,
    pub auto_exam_scheduling: bool,
    pub auto_report_generation: bool,
    pub grace_days: i32,
    pub notify_before_deadline_hours: i32,
    pub max_retries: i32,
    pub auto_advance_periods: bool,
    pub auto_lock_grades: bool,
    pub grade_entry_auto_close_days: i32,
    pub allow_grade_edit_after_close: bool,
}

#[derive(Serialize, Deserialize, ToSchema, Default)]
pub struct DashboardCountsDto {
    pub total_students: u64,
    pub total_teachers: u64,
    pub total_classes: u64,
    pub total_enrollments: u64,
    pub total_subjects: u64,
    pub active_academic_year: Option<String>,
    pub total_payments_amount: f64,
    pub pending_fees_amount: f64,
}

impl From<OrganizationSettings> for OrganizationSettingsDto {
    fn from(s: OrganizationSettings) -> Self {
        Self {
            academic: AcademicSettingsDto {
                default_grading_system: s.academic.default_grading_system,
                default_passing_score: s.academic.default_passing_score,
                default_max_score: s.academic.default_max_score,
                passing_threshold_percentage: s.academic.passing_threshold_percentage,
                honors_threshold_percentage: s.academic.honors_threshold_percentage,
                minimum_final_exam_score: s.academic.minimum_final_exam_score,
                max_failed_subjects_for_conditional_pass: s
                    .academic
                    .max_failed_subjects_for_conditional_pass,
                min_attendance_rate: s.academic.min_attendance_rate,
                ranking_enabled: s.academic.ranking_enabled,
                max_displayed_rank: s.academic.max_displayed_rank,
                ranking_scope: s.academic.ranking_scope,
                tie_policy: s.academic.tie_policy,
                rounding_mode: s.academic.rounding_mode,
                decimal_places: s.academic.decimal_places,
                enrollment_auto_close: s.academic.enrollment_auto_close,
                allow_late_enrollment: s.academic.allow_late_enrollment,
                late_enrollment_approval: s.academic.late_enrollment_approval,
                grade_entry_auto_close: s.academic.grade_entry_auto_close,
                allow_grade_edit_after_close: s.academic.allow_grade_edit_after_close,
                grade_edit_approval: s.academic.grade_edit_approval,
                auto_graduate_final_level: s.academic.auto_graduate_final_level,
                exclude_withdrawn: s.academic.exclude_withdrawn,
            },
            enrollment: EnrollmentSettingsDto {
                registration_enabled: s.enrollment.registration_enabled,
                require_admin_approval: s.enrollment.require_admin_approval,
                allow_waitlist: s.enrollment.allow_waitlist,
                sibling_priority_enabled: s.enrollment.sibling_priority_enabled,
                max_class_size_default: s.enrollment.max_class_size_default,
                allow_late_enrollment: s.enrollment.allow_late_enrollment,
                is_configured: s.enrollment.is_configured,
            },
            admission: AdmissionConfigurationDto {
                numbering: NumberingSchemaDto {
                    prefix: s.admission.numbering.prefix,
                    year_format: match s.admission.numbering.year_format {
                        orbis_domain::entities::admission::YearFormat::Full => "full".to_string(),
                        orbis_domain::entities::admission::YearFormat::Short => "short".to_string(),
                        orbis_domain::entities::admission::YearFormat::None => "none".to_string(),
                    },
                    sequence_digits: s.admission.numbering.sequence_digits,
                    separator: s.admission.numbering.separator,
                },
                policy: AdmissionPolicyDto {
                    requires_approval: s.admission.policy.requires_approval,
                    allow_no_guardian: s.admission.policy.allow_no_guardian,
                    allow_duplicate_national_id: s.admission.policy.allow_duplicate_national_id,
                    auto_assign_fees: s.admission.policy.auto_assign_fees,
                    max_class_capacity: s.admission.policy.max_class_capacity,
                    allow_late_enrollment: s.admission.policy.allow_late_enrollment,
                    allow_waitlist: s.admission.policy.allow_waitlist,
                    sibling_priority_enabled: s.admission.policy.sibling_priority_enabled,
                },
                fields: s
                    .admission
                    .fields
                    .into_iter()
                    .map(|f| FormFieldConfigDto {
                        key: f.key,
                        name_ar: f.name_ar,
                        name_en: f.name_en,
                        is_active: f.is_active,
                        is_required: f.is_required,
                        prevent_duplicate: f.prevent_duplicate,
                        supports_duplicate: f.supports_duplicate,
                    })
                    .collect(),
            },
            finance: FinanceSettingsDto {
                currency: s.finance.currency,
                tax_rate: s.finance.tax_rate,
                invoice_due_days: s.finance.invoice_due_days,
                late_fee_type: s.finance.late_fee_type,
                late_fee_value: s.finance.late_fee_value,
                allow_partial_payments: s.finance.allow_partial_payments,
                primary_payment_gateway_id: s.finance.primary_payment_gateway_id,
                invoicing_integration_id: s.finance.invoicing_integration_id,
            },
            notifications: NotificationSettingsDto {
                default_integration_ids: s.notifications.default_integration_ids,
                email_integration_id: s.notifications.email_integration_id,
                sms_integration_id: s.notifications.sms_integration_id,
                whatsapp_integration_id: s.notifications.whatsapp_integration_id,
                notify_parents_on_absence: s.notifications.notify_parents_on_absence,
                notify_parents_on_grade_publish: s.notifications.notify_parents_on_grade_publish,
                finance_reminder_days: s.notifications.finance_reminder_days,
            },
            autopilot: AutopilotSettingsDto {
                enabled: s.autopilot.enabled,
                mode: s.autopilot.mode,
                auto_open_enrollment: s.autopilot.auto_open_enrollment,
                auto_close_enrollment: s.autopilot.auto_close_enrollment,
                auto_record_attendance: s.autopilot.auto_record_attendance,
                auto_fee_assignment: s.autopilot.auto_fee_assignment,
                auto_grade_entry_reminder: s.autopilot.auto_grade_entry_reminder,
                auto_period_lock: s.autopilot.auto_period_lock,
                auto_promotion: s.autopilot.auto_promotion,
                auto_exam_scheduling: s.autopilot.auto_exam_scheduling,
                auto_report_generation: s.autopilot.auto_report_generation,
                grace_days: s.autopilot.grace_days,
                notify_before_deadline_hours: s.autopilot.notify_before_deadline_hours,
                max_retries: s.autopilot.max_retries,
                auto_advance_periods: s.autopilot.auto_advance_periods,
                auto_lock_grades: s.autopilot.auto_lock_grades,
                grade_entry_auto_close_days: s.autopilot.grade_entry_auto_close_days,
                allow_grade_edit_after_close: s.autopilot.allow_grade_edit_after_close,
            },
        }
    }
}

impl From<OrganizationSettingsDto> for OrganizationSettings {
    fn from(s: OrganizationSettingsDto) -> Self {
        Self {
            academic: orbis_domain::entities::settings::AcademicSettings {
                default_grading_system: s.academic.default_grading_system,
                default_passing_score: s.academic.default_passing_score,
                default_max_score: s.academic.default_max_score,
                passing_threshold_percentage: s.academic.passing_threshold_percentage,
                honors_threshold_percentage: s.academic.honors_threshold_percentage,
                minimum_final_exam_score: s.academic.minimum_final_exam_score,
                max_failed_subjects_for_conditional_pass: s
                    .academic
                    .max_failed_subjects_for_conditional_pass,
                min_attendance_rate: s.academic.min_attendance_rate,
                ranking_enabled: s.academic.ranking_enabled,
                max_displayed_rank: s.academic.max_displayed_rank,
                ranking_scope: s.academic.ranking_scope,
                tie_policy: s.academic.tie_policy,
                rounding_mode: s.academic.rounding_mode,
                decimal_places: s.academic.decimal_places,
                enrollment_auto_close: s.academic.enrollment_auto_close,
                allow_late_enrollment: s.academic.allow_late_enrollment,
                late_enrollment_approval: s.academic.late_enrollment_approval,
                grade_entry_auto_close: s.academic.grade_entry_auto_close,
                allow_grade_edit_after_close: s.academic.allow_grade_edit_after_close,
                grade_edit_approval: s.academic.grade_edit_approval,
                auto_graduate_final_level: s.academic.auto_graduate_final_level,
                exclude_withdrawn: s.academic.exclude_withdrawn,
            },
            enrollment: orbis_domain::entities::settings::EnrollmentSettings {
                registration_enabled: s.enrollment.registration_enabled,
                require_admin_approval: s.enrollment.require_admin_approval,
                allow_waitlist: s.enrollment.allow_waitlist,
                sibling_priority_enabled: s.enrollment.sibling_priority_enabled,
                max_class_size_default: s.enrollment.max_class_size_default,
                allow_late_enrollment: s.enrollment.allow_late_enrollment,
                is_configured: s.enrollment.is_configured,
            },
            admission: orbis_domain::entities::admission::AdmissionConfiguration {
                numbering: orbis_domain::entities::admission::NumberingSchema {
                    prefix: s.admission.numbering.prefix,
                    year_format: match s.admission.numbering.year_format.to_lowercase().as_str() {
                        "short" => orbis_domain::entities::admission::YearFormat::Short,
                        "none" => orbis_domain::entities::admission::YearFormat::None,
                        _ => orbis_domain::entities::admission::YearFormat::Full,
                    },
                    sequence_digits: s.admission.numbering.sequence_digits,
                    separator: s.admission.numbering.separator,
                },
                policy: orbis_domain::entities::admission::AdmissionPolicy {
                    requires_approval: s.admission.policy.requires_approval,
                    allow_no_guardian: s.admission.policy.allow_no_guardian,
                    allow_duplicate_national_id: s.admission.policy.allow_duplicate_national_id,
                    auto_assign_fees: s.admission.policy.auto_assign_fees,
                    max_class_capacity: s.admission.policy.max_class_capacity,
                    allow_late_enrollment: s.admission.policy.allow_late_enrollment,
                    allow_waitlist: s.admission.policy.allow_waitlist,
                    sibling_priority_enabled: s.admission.policy.sibling_priority_enabled,
                },
                fields: s
                    .admission
                    .fields
                    .into_iter()
                    .map(|f| orbis_domain::entities::admission::FormFieldConfig {
                        key: f.key,
                        name_ar: f.name_ar,
                        name_en: f.name_en,
                        is_active: f.is_active,
                        is_required: f.is_required,
                        prevent_duplicate: f.prevent_duplicate,
                        supports_duplicate: f.supports_duplicate,
                    })
                    .collect(),
            },
            finance: orbis_domain::entities::settings::FinanceSettings {
                currency: s.finance.currency,
                tax_rate: s.finance.tax_rate,
                invoice_due_days: s.finance.invoice_due_days,
                late_fee_type: s.finance.late_fee_type,
                late_fee_value: s.finance.late_fee_value,
                allow_partial_payments: s.finance.allow_partial_payments,
                primary_payment_gateway_id: s.finance.primary_payment_gateway_id,
                invoicing_integration_id: s.finance.invoicing_integration_id,
            },
            notifications: orbis_domain::entities::settings::NotificationSettings {
                default_integration_ids: s.notifications.default_integration_ids,
                email_integration_id: s.notifications.email_integration_id,
                sms_integration_id: s.notifications.sms_integration_id,
                whatsapp_integration_id: s.notifications.whatsapp_integration_id,
                notify_parents_on_absence: s.notifications.notify_parents_on_absence,
                notify_parents_on_grade_publish: s.notifications.notify_parents_on_grade_publish,
                finance_reminder_days: s.notifications.finance_reminder_days,
            },
            autopilot: orbis_domain::entities::settings::AutopilotSettings {
                enabled: s.autopilot.enabled,
                mode: s.autopilot.mode,
                auto_open_enrollment: s.autopilot.auto_open_enrollment,
                auto_close_enrollment: s.autopilot.auto_close_enrollment,
                auto_record_attendance: s.autopilot.auto_record_attendance,
                auto_fee_assignment: s.autopilot.auto_fee_assignment,
                auto_grade_entry_reminder: s.autopilot.auto_grade_entry_reminder,
                auto_period_lock: s.autopilot.auto_period_lock,
                auto_promotion: s.autopilot.auto_promotion,
                auto_exam_scheduling: s.autopilot.auto_exam_scheduling,
                auto_report_generation: s.autopilot.auto_report_generation,
                grace_days: s.autopilot.grace_days,
                notify_before_deadline_hours: s.autopilot.notify_before_deadline_hours,
                max_retries: s.autopilot.max_retries,
                auto_advance_periods: s.autopilot.auto_advance_periods,
                auto_lock_grades: s.autopilot.auto_lock_grades,
                grade_entry_auto_close_days: s.autopilot.grade_entry_auto_close_days,
                allow_grade_edit_after_close: s.autopilot.allow_grade_edit_after_close,
            },
        }
    }
}

// ── Settings Handlers ───────────────────────────────────────────

#[utoipa::path(get, path = "/api/v1/organizations/current/settings",
    responses((status = 200, description = "Organization settings", body = OrganizationSettingsDto)),
    security(("bearer" = [])), tag = "Organization")]
async fn get_settings(
    ctx: AuthContext,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<OrganizationSettingsDto>>, ApiError> {
    let settings = state
        .organization_service
        .get_settings(ctx.tenant_id)
        .await?;
    Ok(ApiResponse::ok(settings.into()))
}

#[utoipa::path(put, path = "/api/v1/organizations/current/settings",
    request_body = OrganizationSettingsDto,
    responses((status = 200, description = "Settings updated", body = OrganizationSettingsDto)),
    security(("bearer" = [])), tag = "Organization")]
async fn update_settings(
    ctx: AuthContext,
    State(state): State<AppState>,
    Json(payload): Json<OrganizationSettingsDto>,
) -> Result<Json<ApiResponse<OrganizationSettingsDto>>, ApiError> {
    let settings = state
        .organization_service
        .update_settings(&ctx, ctx.tenant_id, payload.into())
        .await?;
    Ok(ApiResponse::ok(settings.into()))
}

// ── Dashboard ───────────────────────────────────────────────────

#[utoipa::path(get, path = "/api/v1/organizations/current/dashboard",
    responses((status = 200, description = "Dashboard statistics", body = DashboardCountsDto)),
    security(("bearer" = [])), tag = "Organization")]
async fn get_dashboard(
    ctx: AuthContext,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<DashboardCountsDto>>, ApiError> {
    let counts = state
        .organization_service
        .get_dashboard_counts(ctx.tenant_id)
        .await?;
    Ok(ApiResponse::ok(DashboardCountsDto {
        total_students: counts.total_students,
        total_teachers: counts.total_teachers,
        total_classes: counts.total_classes,
        total_enrollments: counts.total_enrollments,
        total_subjects: counts.total_subjects,
        active_academic_year: counts.active_academic_year,
        total_payments_amount: counts.total_payments_amount,
        pending_fees_amount: counts.pending_fees_amount,
    }))
}

// ── OrgUnits ────────────────────────────────────────────────────

#[utoipa::path(post, path = "/api/v1/org-units", request_body = CreateOrgUnitRequest,
    responses((status = 201, description = "OrgUnit created", body = OrgUnitDto)),
    security(("bearer" = [])), tag = "OrgUnits")]
async fn create_org_unit(
    ctx: AuthContext,
    State(state): State<AppState>,
    Json(payload): Json<CreateOrgUnitRequest>,
) -> Result<Json<ApiResponse<OrgUnitDto>>, ApiError> {
    let unit = state
        .organization_service
        .create_org_unit(
            &ctx,
            ctx.tenant_id,
            orbis_app::services::CreateOrgUnitInput {
                name: payload.name,
                name_ar: payload.name_ar,
                code: payload.code,
                unit_type: payload.unit_type,
                parent_id: payload.parent_id,
                head_id: payload.head_id,
                capacity: payload.capacity,
                building: payload.building,
                floor: payload.floor,
                room_number: payload.room_number,
                settings: payload.settings,
                local_policies: payload.local_policies,
                data_isolation: payload.data_isolation,
                sort_order: payload.sort_order,
                is_active: payload.is_active,
            },
        )
        .await?;

    Ok(ApiResponse::ok(unit_to_dto(unit)))
}

#[utoipa::path(get, path = "/api/v1/org-units",
    responses((status = 200, description = "List of org units", body = Vec<OrgUnitDto>)),
    security(("bearer" = [])), tag = "OrgUnits")]
async fn list_org_units(
    ctx: AuthContext,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<OrgUnitDto>>>, ApiError> {
    let units = state
        .organization_service
        .list_org_units(ctx.tenant_id)
        .await?;
    let dtos: Vec<OrgUnitDto> = units.into_iter().map(unit_to_dto).collect();
    Ok(ApiResponse::ok(dtos))
}

#[utoipa::path(get, path = "/api/v1/org-units/{id}",
    params(("id" = Uuid, Path, description = "OrgUnit ID")),
    responses((status = 200, description = "OrgUnit details", body = OrgUnitDto)),
    security(("bearer" = [])), tag = "OrgUnits")]
async fn get_org_unit(
    _ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<OrgUnitDto>>, ApiError> {
    let u = state
        .organization_service
        .get_org_unit(id)
        .await?
        .ok_or_else(|| ApiError::not_found(format!("OrgUnit {} not found", id)))?;

    Ok(ApiResponse::ok(unit_to_dto(u)))
}

#[utoipa::path(patch, path = "/api/v1/org-units/{id}", request_body = UpdateOrgUnitRequest,
    params(("id" = Uuid, Path, description = "OrgUnit ID")),
    responses((status = 200, description = "OrgUnit updated", body = OrgUnitDto)),
    security(("bearer" = [])), tag = "OrgUnits")]
async fn update_org_unit(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateOrgUnitRequest>,
) -> Result<Json<ApiResponse<OrgUnitDto>>, ApiError> {
    let unit = state
        .organization_service
        .update_org_unit(
            &ctx,
            id,
            orbis_app::services::UpdateOrgUnitRequest {
                name: payload.name,
                name_ar: payload.name_ar,
                code: payload.code,
                unit_type: payload.unit_type,
                parent_id: payload.parent_id,
                head_id: payload.head_id,
                capacity: payload.capacity,
                building: payload.building,
                floor: payload.floor,
                room_number: payload.room_number,
                settings: payload.settings,
                local_policies: payload.local_policies,
                data_isolation: payload.data_isolation,
                sort_order: payload.sort_order,
                is_active: payload.is_active,
            },
        )
        .await?;

    Ok(ApiResponse::ok(unit_to_dto(unit)))
}

#[utoipa::path(delete, path = "/api/v1/org-units/{id}",
    params(("id" = Uuid, Path, description = "OrgUnit ID")),
    responses((status = 204, description = "OrgUnit deleted")),
    security(("bearer" = [])), tag = "OrgUnits")]
async fn delete_org_unit(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<axum::http::StatusCode, ApiError> {
    state.organization_service.delete_org_unit(&ctx, id).await?;
    Ok(axum::http::StatusCode::NO_CONTENT)
}

// ── Router ──────────────────────────────────────────────────────

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/api/v1/system/readiness",
            get(get_system_readiness_endpoint),
        )
        .route(
            "/api/v1/system/has-organization",
            get(get_system_has_org_endpoint),
        )
        .route("/api/v1/organizations", post(create_org))
        .route(
            "/api/v1/organizations/current",
            get(get_current_org).put(update_current_org),
        )
        .route(
            "/api/v1/organizations/current/settings",
            get(get_settings).put(update_settings),
        )
        .route(
            "/api/v1/organizations/current/dashboard",
            get(get_dashboard),
        )
        .route(
            "/api/v1/org-units",
            post(create_org_unit).get(list_org_units),
        )
        .route(
            "/api/v1/org-units/:id",
            get(get_org_unit)
                .patch(update_org_unit)
                .delete(delete_org_unit),
        )
}
