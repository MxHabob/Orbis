//! Employee routes — Registrations, Profiles, Status updates, Account provisioning, Group assignments.

use axum::{extract::{State, Path, Query}, routing::{get, post, patch}, Json, Router};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use chrono::{DateTime, Utc, NaiveDate};
use uuid::Uuid;

use super::{ApiError, ApiResponse};
use super::extractors::AuthContext;
use crate::state::AppState;
use orbis_domain::entities::employee::{EmployeeDetail, EmployeeStatus, EmploymentType};
use orbis_domain::entities::identity_provisioning::{DeliveryChannel, AccountProvisioningResult};
use orbis_app::ports::repositories::EmployeeFilter;
use orbis_app::services::employee_service::{
    RegisterEmployeeInput, UpdateEmployeeInput, AccountProvisioningConfig
};

#[derive(Serialize, Deserialize, ToSchema)]
pub struct EmployeeDetailDto {
    pub id: Uuid,
    pub org_id: Uuid,
    pub person_id: Uuid,
    pub user_id: Option<Uuid>,
    pub employee_number: String,
    pub employee_type: String,
    pub job_title: Option<String>,
    pub department: Option<String>,
    pub org_unit_id: Option<Uuid>,
    pub org_unit_name: Option<String>,
    pub hire_date: NaiveDate,
    pub status: String,
    pub employment_type: String,
    pub work_shift: Option<String>,
    pub notes: Option<String>,

    // Person fields
    pub national_id: Option<String>,
    pub passport_number: Option<String>,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub first_name_ar: Option<String>,
    pub middle_name_ar: Option<String>,
    pub last_name_ar: Option<String>,
    pub display_name: Option<String>,
    pub full_name: String,
    pub full_name_ar: Option<String>,
    pub gender: Option<String>,
    pub birth_date: Option<NaiveDate>,
    pub place_of_birth: Option<String>,
    pub nationality: Option<String>,
    pub blood_type: Option<String>,
    pub medical_notes: Option<String>,
    pub special_needs_notes: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub avatar_url: Option<String>,

    // User account summary
    pub username: Option<String>,
    pub user_status: Option<String>,
    pub roles: Vec<String>,
    pub groups: Vec<String>,
    pub effective_permissions: Vec<String>,

    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<EmployeeDetail> for EmployeeDetailDto {
    fn from(d: EmployeeDetail) -> Self {
        let e = d.employee;
        let p = d.person;
        let full_name = p.full_name();
        let full_name_ar = p.full_name_ar();
        let username = d.user.as_ref().and_then(|u| u.email.clone());
        let user_status = d.user.as_ref().map(|u| u.status.clone());
        let groups = d.groups.into_iter().map(|g| g.name).collect();

        Self {
            id: e.id,
            org_id: e.org_id,
            person_id: e.person_id,
            user_id: e.user_id,
            employee_number: e.employee_number,
            employee_type: e.employee_type,
            job_title: e.job_title,
            department: e.department,
            org_unit_id: e.org_unit_id,
            org_unit_name: d.org_unit_name,
            hire_date: e.hire_date,
            status: e.status.to_string(),
            employment_type: e.employment_type.to_string(),
            work_shift: e.work_shift,
            notes: e.notes,

            national_id: p.national_id,
            passport_number: p.passport_number,
            first_name: p.first_name,
            middle_name: p.middle_name,
            last_name: p.last_name,
            first_name_ar: p.first_name_ar,
            middle_name_ar: p.middle_name_ar,
            last_name_ar: p.last_name_ar,
            display_name: p.display_name,
            full_name,
            full_name_ar,
            gender: p.gender,
            birth_date: p.birth_date,
            place_of_birth: p.place_of_birth,
            nationality: p.nationality,
            blood_type: p.blood_type,
            medical_notes: p.medical_notes,
            special_needs_notes: p.special_needs_notes,
            phone: p.phone,
            email: p.email,
            address: p.address,
            avatar_url: p.avatar_url,

            username,
            user_status,
            roles: d.roles,
            groups,
            effective_permissions: d.effective_permissions,

            created_by: e.created_by,
            updated_by: e.updated_by,
            created_at: e.created_at,
            updated_at: e.updated_at,
        }
    }
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct AccountProvisioningResultDto {
    pub user_id: Uuid,
    pub username: String,
    pub temporary_password: Option<String>,
    pub must_change_password: bool,
    pub delivery_channel: String,
    pub delivery_status: String,
}

impl From<AccountProvisioningResult> for AccountProvisioningResultDto {
    fn from(r: AccountProvisioningResult) -> Self {
        Self {
            user_id: r.user_id,
            username: r.username,
            temporary_password: r.temporary_password,
            must_change_password: r.must_change_password,
            delivery_channel: r.delivery_channel.to_string(),
            delivery_status: r.delivery_status.to_string(),
        }
    }
}

impl From<orbis_app::services::identity_provisioning_service::ResetPasswordResult> for AccountProvisioningResultDto {
    fn from(r: orbis_app::services::identity_provisioning_service::ResetPasswordResult) -> Self {
        Self {
            user_id: r.user_id,
            username: r.username,
            temporary_password: r.temporary_password,
            must_change_password: true,
            delivery_channel: r.delivery_channel.to_string(),
            delivery_status: r.delivery_status.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct RegisterEmployeeResponseDto {
    pub employee: EmployeeDetailDto,
    pub provisioning: Option<AccountProvisioningResultDto>,
}

#[derive(Deserialize, ToSchema)]
pub struct AccountProvisioningConfigRequest {
    pub enabled: bool,
    pub preferred_username: Option<String>,
    pub role: Option<String>,
    #[serde(default)]
    pub group_ids: Vec<Uuid>,
    pub delivery_channel: Option<String>,
    pub custom_password: Option<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct RegisterEmployeeRequest {
    pub person_id: Option<Uuid>,
    pub employee_number: Option<String>,
    pub employee_type: String,
    pub job_title: Option<String>,
    pub department: Option<String>,
    pub org_unit_id: Option<Uuid>,
    pub hire_date: Option<NaiveDate>,
    pub status: Option<String>,
    pub employment_type: Option<String>,
    pub work_shift: Option<String>,
    pub notes: Option<String>,

    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub first_name_ar: Option<String>,
    pub middle_name_ar: Option<String>,
    pub last_name_ar: Option<String>,
    pub display_name: Option<String>,
    pub national_id: Option<String>,
    pub passport_number: Option<String>,
    pub gender: Option<String>,
    pub birth_date: Option<NaiveDate>,
    pub place_of_birth: Option<String>,
    pub nationality: Option<String>,
    pub blood_type: Option<String>,
    pub medical_notes: Option<String>,
    pub special_needs_notes: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub avatar_url: Option<String>,

    pub account: Option<AccountProvisioningConfigRequest>,
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateEmployeeRequest {
    pub employee_type: Option<String>,
    pub job_title: Option<String>,
    pub department: Option<String>,
    pub org_unit_id: Option<Uuid>,
    pub hire_date: Option<NaiveDate>,
    pub status: Option<String>,
    pub employment_type: Option<String>,
    pub work_shift: Option<String>,
    pub notes: Option<String>,

    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub first_name_ar: Option<String>,
    pub middle_name_ar: Option<String>,
    pub last_name_ar: Option<String>,
    pub display_name: Option<String>,
    pub national_id: Option<String>,
    pub passport_number: Option<String>,
    pub gender: Option<String>,
    pub birth_date: Option<NaiveDate>,
    pub place_of_birth: Option<String>,
    pub nationality: Option<String>,
    pub blood_type: Option<String>,
    pub medical_notes: Option<String>,
    pub special_needs_notes: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub avatar_url: Option<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateEmployeeStatusRequest {
    pub status: String,
}

#[derive(Deserialize, ToSchema)]
pub struct ProvisionEmployeeAccountRequest {
    pub preferred_username: Option<String>,
    pub role: Option<String>,
    #[serde(default)]
    pub group_ids: Vec<Uuid>,
    pub delivery_channel: Option<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct AssignEmployeeGroupsRequest {
    pub group_ids: Vec<Uuid>,
}

#[derive(Deserialize, Default)]
pub struct EmployeeFilterQuery {
    pub query: Option<String>,
    pub employee_type: Option<String>,
    pub department: Option<String>,
    pub status: Option<String>,
    pub org_unit_id: Option<Uuid>,
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

#[derive(Serialize, ToSchema)]
pub struct PaginatedEmployeesDto {
    pub items: Vec<EmployeeDetailDto>,
    pub total_count: u64,
    pub page: u64,
    pub page_size: u64,
    pub total_pages: u64,
}

fn parse_delivery_channel(s: Option<&str>) -> Option<DeliveryChannel> {
    match s {
        Some("email") => Some(DeliveryChannel::Email),
        Some("sms") => Some(DeliveryChannel::Sms),
        Some("whatsapp") => Some(DeliveryChannel::WhatsApp),
        Some("manual") => Some(DeliveryChannel::Manual),
        _ => Some(DeliveryChannel::Auto),
    }
}

// ── Handlers ─────────────────────────────────────────────────────

#[utoipa::path(post, path = "/api/v1/employees",
    request_body = RegisterEmployeeRequest,
    responses((status = 200, body = RegisterEmployeeResponseDto)),
    security(("bearer" = [])), tag = "Employee")]
pub async fn register_employee(
    ctx: AuthContext,
    State(state): State<AppState>,
    Json(p): Json<RegisterEmployeeRequest>,
) -> Result<Json<ApiResponse<RegisterEmployeeResponseDto>>, ApiError> {
    let org_id = ctx.tenant_id;

    let account_cfg = p.account.map(|acc| AccountProvisioningConfig {
        enabled: acc.enabled,
        preferred_username: acc.preferred_username,
        role: acc.role,
        group_ids: acc.group_ids,
        delivery_channel: parse_delivery_channel(acc.delivery_channel.as_deref()),
        custom_password: acc.custom_password,
    });

    let status = p.status.as_deref().and_then(|s| match s {
        "active" => Some(EmployeeStatus::Active),
        "on_leave" => Some(EmployeeStatus::OnLeave),
        "suspended" => Some(EmployeeStatus::Suspended),
        "terminated" => Some(EmployeeStatus::Terminated),
        "resigned" => Some(EmployeeStatus::Resigned),
        _ => None,
    });

    let employment_type = p.employment_type.as_deref().and_then(|e| match e {
        "full_time" => Some(EmploymentType::FullTime),
        "part_time" => Some(EmploymentType::PartTime),
        "contract" => Some(EmploymentType::Contract),
        "hourly" => Some(EmploymentType::Hourly),
        _ => None,
    });

    let input = RegisterEmployeeInput {
        person_id: p.person_id,
        employee_number: p.employee_number,
        employee_type: p.employee_type,
        job_title: p.job_title,
        department: p.department,
        org_unit_id: p.org_unit_id,
        hire_date: p.hire_date,
        status,
        employment_type,
        work_shift: p.work_shift,
        notes: p.notes,
        first_name: p.first_name,
        middle_name: p.middle_name,
        last_name: p.last_name,
        first_name_ar: p.first_name_ar,
        middle_name_ar: p.middle_name_ar,
        last_name_ar: p.last_name_ar,
        display_name: p.display_name,
        national_id: p.national_id,
        passport_number: p.passport_number,
        gender: p.gender,
        birth_date: p.birth_date,
        place_of_birth: p.place_of_birth,
        nationality: p.nationality,
        blood_type: p.blood_type,
        medical_notes: p.medical_notes,
        special_needs_notes: p.special_needs_notes,
        phone: p.phone,
        email: p.email,
        address: p.address,
        avatar_url: p.avatar_url,
        account: account_cfg,
    };

    let result = state.employee_service.register_employee(&ctx, org_id, input).await?;
    let res_dto = RegisterEmployeeResponseDto {
        employee: result.detail.into(),
        provisioning: result.provisioning.map(Into::into),
    };
    Ok(ApiResponse::ok(res_dto))
}

#[utoipa::path(get, path = "/api/v1/employees",
    responses((status = 200, body = PaginatedEmployeesDto)),
    security(("bearer" = [])), tag = "Employee")]
pub async fn list_employees(
    ctx: AuthContext,
    State(state): State<AppState>,
    Query(q): Query<EmployeeFilterQuery>,
) -> Result<Json<ApiResponse<PaginatedEmployeesDto>>, ApiError> {
    let org_id = ctx.tenant_id;
    let page = q.page.unwrap_or(1);
    let page_size = q.page_size.unwrap_or(20);
    let offset = (page.saturating_sub(1)) * page_size;

    let filter = EmployeeFilter {
        search: q.query,
        employee_type: q.employee_type,
        department: q.department,
        org_unit_id: q.org_unit_id,
        status: q.status,
        limit: Some(page_size),
        offset: Some(offset),
    };

    let list = state.employee_service.list_employees(&ctx, org_id, filter).await?;
    let dto = PaginatedEmployeesDto {
        items: list.items.into_iter().map(Into::into).collect(),
        total_count: list.total_count,
        page: list.page,
        page_size: list.page_size,
        total_pages: list.total_pages,
    };
    Ok(ApiResponse::ok(dto))
}

#[utoipa::path(get, path = "/api/v1/employees/stats",
    responses((status = 200, body = std::collections::HashMap<String, i64>)),
    security(("bearer" = [])), tag = "Employee")]
pub async fn get_employee_stats(
    ctx: AuthContext,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<std::collections::HashMap<String, i64>>>, ApiError> {
    let org_id = ctx.tenant_id;
    let stats = state.employee_service.count_employees_by_status(&ctx, org_id).await?;
    Ok(ApiResponse::ok(stats))
}

#[utoipa::path(get, path = "/api/v1/employees/{id}",
    params(("id" = Uuid, Path, description = "Employee ID")),
    responses((status = 200, body = EmployeeDetailDto)),
    security(("bearer" = [])), tag = "Employee")]
pub async fn get_employee(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<EmployeeDetailDto>>, ApiError> {
    let detail = state.employee_service.get_employee(&ctx, id).await?
        .ok_or_else(|| ApiError::not_found(format!("Employee {} not found", id)))?;
    Ok(ApiResponse::ok(detail.into()))
}

#[utoipa::path(patch, path = "/api/v1/employees/{id}",
    params(("id" = Uuid, Path, description = "Employee ID")),
    request_body = UpdateEmployeeRequest,
    responses((status = 200, body = EmployeeDetailDto)),
    security(("bearer" = [])), tag = "Employee")]
pub async fn update_employee(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(p): Json<UpdateEmployeeRequest>,
) -> Result<Json<ApiResponse<EmployeeDetailDto>>, ApiError> {
    let status = p.status.as_deref().and_then(|s| match s {
        "active" => Some(EmployeeStatus::Active),
        "on_leave" => Some(EmployeeStatus::OnLeave),
        "suspended" => Some(EmployeeStatus::Suspended),
        "terminated" => Some(EmployeeStatus::Terminated),
        "resigned" => Some(EmployeeStatus::Resigned),
        _ => None,
    });

    let employment_type = p.employment_type.as_deref().and_then(|e| match e {
        "full_time" => Some(EmploymentType::FullTime),
        "part_time" => Some(EmploymentType::PartTime),
        "contract" => Some(EmploymentType::Contract),
        "hourly" => Some(EmploymentType::Hourly),
        _ => None,
    });

    let input = UpdateEmployeeInput {
        employee_type: p.employee_type,
        job_title: p.job_title,
        department: p.department,
        org_unit_id: p.org_unit_id,
        hire_date: p.hire_date,
        status,
        employment_type,
        work_shift: p.work_shift,
        notes: p.notes,
        first_name: p.first_name,
        middle_name: p.middle_name,
        last_name: p.last_name,
        first_name_ar: p.first_name_ar,
        middle_name_ar: p.middle_name_ar,
        last_name_ar: p.last_name_ar,
        display_name: p.display_name,
        national_id: p.national_id,
        passport_number: p.passport_number,
        gender: p.gender,
        birth_date: p.birth_date,
        place_of_birth: p.place_of_birth,
        nationality: p.nationality,
        blood_type: p.blood_type,
        medical_notes: p.medical_notes,
        special_needs_notes: p.special_needs_notes,
        phone: p.phone,
        email: p.email,
        address: p.address,
        avatar_url: p.avatar_url,
    };

    let updated = state.employee_service.update_employee(&ctx, id, input).await?;
    Ok(ApiResponse::ok(updated.into()))
}

#[utoipa::path(patch, path = "/api/v1/employees/{id}/status",
    params(("id" = Uuid, Path, description = "Employee ID")),
    request_body = UpdateEmployeeStatusRequest,
    responses((status = 200, body = EmployeeDetailDto)),
    security(("bearer" = [])), tag = "Employee")]
pub async fn update_employee_status(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(p): Json<UpdateEmployeeStatusRequest>,
) -> Result<Json<ApiResponse<EmployeeDetailDto>>, ApiError> {
    let new_status = match p.status.as_str() {
        "active" => EmployeeStatus::Active,
        "on_leave" => EmployeeStatus::OnLeave,
        "suspended" => EmployeeStatus::Suspended,
        "terminated" => EmployeeStatus::Terminated,
        "resigned" => EmployeeStatus::Resigned,
        other => return Err(ApiError::bad_request(format!("Invalid employee status: {}", other))),
    };

    let updated = state.employee_service.update_employee_status(&ctx, id, new_status).await?;
    Ok(ApiResponse::ok(updated.into()))
}

#[utoipa::path(post, path = "/api/v1/employees/{id}/account",
    params(("id" = Uuid, Path, description = "Employee ID")),
    request_body = ProvisionEmployeeAccountRequest,
    responses((status = 200, body = AccountProvisioningResultDto)),
    security(("bearer" = [])), tag = "Employee")]
pub async fn provision_account(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(p): Json<ProvisionEmployeeAccountRequest>,
) -> Result<Json<ApiResponse<AccountProvisioningResultDto>>, ApiError> {
    let delivery_channel = parse_delivery_channel(p.delivery_channel.as_deref());
    let res = state.employee_service.provision_employee_account(
        &ctx,
        id,
        p.preferred_username,
        p.role,
        p.group_ids,
        delivery_channel,
    ).await?;
    Ok(ApiResponse::ok(res.into()))
}

#[utoipa::path(post, path = "/api/v1/employees/{id}/groups",
    params(("id" = Uuid, Path, description = "Employee ID")),
    request_body = AssignEmployeeGroupsRequest,
    responses((status = 200, body = bool)),
    security(("bearer" = [])), tag = "Employee")]
pub async fn assign_groups(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(p): Json<AssignEmployeeGroupsRequest>,
) -> Result<Json<ApiResponse<bool>>, ApiError> {
    state.employee_service.assign_employee_groups(&ctx, id, p.group_ids).await?;
    Ok(ApiResponse::ok(true))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/v1/employees", post(register_employee).get(list_employees))
        .route("/api/v1/employees/stats", get(get_employee_stats))
        .route("/api/v1/employees/:id", get(get_employee).patch(update_employee))
        .route("/api/v1/employees/:id/status", patch(update_employee_status))
        .route("/api/v1/employees/:id/account", post(provision_account))
        .route("/api/v1/employees/:id/groups", post(assign_groups))
}
