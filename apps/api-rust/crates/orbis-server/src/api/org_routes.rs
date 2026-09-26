//! Organization routes — Org CRUD, OrgUnits, Settings, Dashboard, Bootstrap.

use axum::{extract::{State, Path}, routing::{get, post}, Json, Router};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use super::{ApiError, ApiResponse};
use super::extractors::AuthContext;
use crate::state::AppState;
use orbis_domain::entities::settings::OrganizationSettings;

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
    pub timezone: Option<String>,
    pub locale: Option<String>,
    pub primary_color: Option<String>,
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
}

fn org_to_dto(org: orbis_domain::entities::organization::Organization) -> OrgDto {
    OrgDto {
        id: org.id,
        name: org.name,
        name_ar: org.name_ar,
        slug: org.slug,
        code: org.code,
        timezone: org.timezone,
        locale: org.locale,
        date_format: org.date_format,
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
    }
}

#[derive(Deserialize, ToSchema)]
pub struct CreateOrgUnitRequest {
    pub name: String,
    pub unit_type: String,
    pub parent_id: Option<Uuid>,
    pub data_isolation: String,
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
    pub capacity: Option<i32>,
    pub building: Option<String>,
    pub floor: Option<i32>,
    pub room_number: Option<String>,
    pub sort_order: Option<i32>,
    pub is_active: bool,
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
        capacity: u.capacity,
        building: u.building,
        floor: u.floor,
        room_number: u.room_number,
        sort_order: u.sort_order,
        is_active: u.is_active,
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

    let org = state.organization_service.create_organization(
        &ctx, payload.name, payload.slug, payload.timezone, payload.locale
    ).await?;

    Ok(ApiResponse::ok(org_to_dto(org)))
}

#[utoipa::path(get, path = "/api/v1/organizations/current",
    responses((status = 200, description = "Current organization", body = OrgDto)),
    security(("bearer" = [])), tag = "Organization")]
async fn get_current_org(
    ctx: AuthContext,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<OrgDto>>, ApiError> {
    let org = state.organization_service.get_organization(ctx.tenant_id).await?
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
    let org = state.organization_service.update_organization(
        &ctx, ctx.tenant_id, payload.name, payload.timezone, payload.locale, payload.primary_color
    ).await?;

    Ok(ApiResponse::ok(org_to_dto(org)))
}

#[utoipa::path(post, path = "/api/v1/bootstrap",
    request_body = serde_json::Value,
    responses((status = 200, description = "Organization bootstrapped")),
    tag = "Organization")]
pub async fn bootstrap_org_endpoint(
    State(state): State<AppState>,
    Json(payload): Json<orbis_app::services::BootstrapRequest>,
) -> Result<Json<ApiResponse<orbis_app::services::BootstrapResult>>, ApiError> {
    let res = state.bootstrap_service.bootstrap_organization(payload).await
        .map_err(|e| ApiError::bad_request(e.to_string()))?;
    Ok(ApiResponse::ok(res))
}

#[utoipa::path(post, path = "/api/v1/bootstrap/initial-setup",
    request_body = serde_json::Value,
    responses((status = 200, description = "Initial academic setup completed")),
    tag = "Organization")]
pub async fn bootstrap_initial_setup_endpoint(
    ctx: AuthContext,
    State(state): State<AppState>,
    Json(payload): Json<orbis_app::services::InitialSetupRequest>,
) -> Result<Json<ApiResponse<orbis_app::services::OrganizationReadiness>>, ApiError> {
    let res = state.bootstrap_service.complete_initial_setup(ctx.tenant_id, payload).await
        .map_err(|e| ApiError::bad_request(e.to_string()))?;
    Ok(ApiResponse::ok(res))
}

#[utoipa::path(post, path = "/api/v1/onboarding/complete",
    request_body = serde_json::Value,
    responses((status = 200, description = "Organization onboarding completed")),
    security(("bearer" = [])), tag = "Organization")]
pub async fn onboarding_complete_endpoint(
    ctx: AuthContext,
    State(state): State<AppState>,
    Json(payload): Json<orbis_app::services::CompleteOrganizationOnboardingRequest>,
) -> Result<Json<ApiResponse<orbis_app::services::OnboardingResult>>, ApiError> {
    let res = state.onboarding_service.complete_onboarding(ctx.tenant_id, payload).await
        .map_err(|e| ApiError::bad_request(e.to_string()))?;
    Ok(ApiResponse::ok(res))
}

#[utoipa::path(get, path = "/api/v1/system/readiness",
    responses((status = 200, description = "Get system readiness")),
    tag = "System")]
pub async fn get_system_readiness_endpoint(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<orbis_app::services::OrganizationReadiness>>, ApiError> {
    let res = state.system_service.get_readiness().await
        .map_err(|e| ApiError::internal(e.to_string()))?;
    Ok(ApiResponse::ok(res))
}

#[utoipa::path(get, path = "/api/v1/system/has-organization",
    responses((status = 200, description = "Check if organization exists")),
    tag = "System")]
pub async fn get_system_has_org_endpoint(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<serde_json::Value>>, ApiError> {
    let has_org = state.system_service.has_organization().await
        .map_err(|e| ApiError::internal(e.to_string()))?;
    Ok(ApiResponse::ok(serde_json::json!({ "has_organization": has_org })))
}

#[utoipa::path(get, path = "/api/v1/system/entitlement",
    responses((status = 200, description = "Get system entitlement")),
    tag = "System")]
pub async fn get_system_entitlement_endpoint(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<orbis_domain::entities::entitlement::EntitlementResult>>, ApiError> {
    let res = state.bootstrap_service.entitlement_provider
        .check_entitlement("orbis-server", "default")
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?;
    Ok(ApiResponse::ok(res))
}

// ── Settings DTOs ───────────────────────────────────────────────

#[derive(Serialize, Deserialize, ToSchema, Default)]
pub struct OrganizationSettingsDto {
    #[serde(default)]
    pub academic: AcademicSettingsDto,
    #[serde(default)]
    pub enrollment: EnrollmentSettingsDto,
    #[serde(default)]
    pub finance: FinanceSettingsDto,
    #[serde(default)]
    pub notifications: NotificationSettingsDto,
    #[serde(default)]
    pub autopilot: AutopilotSettingsDto,
}

#[derive(Serialize, Deserialize, ToSchema, Default)]
pub struct AcademicSettingsDto {
    pub default_grading_system: String,
}

#[derive(Serialize, Deserialize, ToSchema, Default)]
pub struct EnrollmentSettingsDto {
    pub registration_enabled: bool,
    pub require_admin_approval: bool,
    pub allow_waitlist: bool,
    pub sibling_priority_enabled: bool,
    pub max_class_size_default: i32,
    pub allow_late_enrollment: bool,
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
            academic: AcademicSettingsDto { default_grading_system: s.academic.default_grading_system },
            enrollment: EnrollmentSettingsDto {
                registration_enabled: s.enrollment.registration_enabled,
                require_admin_approval: s.enrollment.require_admin_approval,
                allow_waitlist: s.enrollment.allow_waitlist,
                sibling_priority_enabled: s.enrollment.sibling_priority_enabled,
                max_class_size_default: s.enrollment.max_class_size_default,
                allow_late_enrollment: s.enrollment.allow_late_enrollment,
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
            academic: orbis_domain::entities::settings::AcademicSettings { default_grading_system: s.academic.default_grading_system },
            enrollment: orbis_domain::entities::settings::EnrollmentSettings {
                registration_enabled: s.enrollment.registration_enabled,
                require_admin_approval: s.enrollment.require_admin_approval,
                allow_waitlist: s.enrollment.allow_waitlist,
                sibling_priority_enabled: s.enrollment.sibling_priority_enabled,
                max_class_size_default: s.enrollment.max_class_size_default,
                allow_late_enrollment: s.enrollment.allow_late_enrollment,
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
            admission: Default::default(),
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
    let settings = state.organization_service.get_settings(ctx.tenant_id).await?;
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
    let settings = state.organization_service.update_settings(&ctx, ctx.tenant_id, payload.into()).await?;
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
    let counts = state.organization_service.get_dashboard_counts(ctx.tenant_id).await?;
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
    let unit = state.organization_service.create_org_unit(&ctx, ctx.tenant_id, payload.name, payload.unit_type, payload.parent_id, payload.data_isolation).await?;

    Ok(ApiResponse::ok(unit_to_dto(unit)))
}

#[utoipa::path(get, path = "/api/v1/org-units",
    responses((status = 200, description = "List of org units", body = Vec<OrgUnitDto>)),
    security(("bearer" = [])), tag = "OrgUnits")]
async fn list_org_units(
    ctx: AuthContext,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<OrgUnitDto>>>, ApiError> {
    let units = state.organization_service.list_org_units(ctx.tenant_id).await?;
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
    let u = state.organization_service.get_org_unit(id).await?
        .ok_or_else(|| ApiError::not_found(format!("OrgUnit {} not found", id)))?;

    Ok(ApiResponse::ok(unit_to_dto(u)))
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
        .route("/api/v1/bootstrap", post(bootstrap_org_endpoint))
        .route("/api/v1/bootstrap/initial-setup", post(bootstrap_initial_setup_endpoint))
        .route("/api/v1/onboarding/complete", post(onboarding_complete_endpoint))
        .route("/api/v1/organizations/bootstrap", post(bootstrap_org_endpoint))
        .route("/api/v1/system/readiness", get(get_system_readiness_endpoint))
        .route("/api/v1/system/has-organization", get(get_system_has_org_endpoint))
        .route("/api/v1/system/entitlement", get(get_system_entitlement_endpoint))
        .route("/api/v1/organizations", post(create_org))
        .route("/api/v1/organizations/current", get(get_current_org).put(update_current_org))
        .route("/api/v1/organizations/current/settings", get(get_settings).put(update_settings))
        .route("/api/v1/organizations/current/dashboard", get(get_dashboard))
        .route("/api/v1/org-units", post(create_org_unit).get(list_org_units))
        .route("/api/v1/org-units/:id", get(get_org_unit).delete(delete_org_unit))
}
