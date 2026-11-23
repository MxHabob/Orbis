//! Tauri IPC commands — Employee Affairs (Registration, Profiles, Status updates, Account Provisioning, Groups).

use super::make_ctx;
use super::org_cmds::ApiResponse;
use crate::state::AppState;
use chrono::NaiveDate;
use orbis_app::ports::repositories::EmployeeFilter;
use orbis_app::services::employee_service::{
    AccountProvisioningConfig, RegisterEmployeeInput, UpdateEmployeeInput,
};
use orbis_domain::entities::employee::{EmployeeDetail, EmployeeStatus, EmploymentType};
use orbis_domain::entities::identity_provisioning::{AccountProvisioningResult, DeliveryChannel};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tauri::State;
use tokio::sync::RwLock;
use uuid::Uuid;

fn ok<T: Serialize>(data: T) -> ApiResponse<T> {
    ApiResponse { data }
}

// ── DTOs ─────────────────────────────────────────────────────────

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EmployeeDetailDto {
    pub id: String,
    pub org_id: String,
    pub person_id: String,
    pub user_id: Option<String>,
    pub employee_number: String,
    pub employee_type: String,
    pub job_title: Option<String>,
    pub department: Option<String>,
    pub org_unit_id: Option<String>,
    pub org_unit_name: Option<String>,
    pub hire_date: String,
    pub status: String,
    pub employment_type: String,
    pub work_shift: Option<String>,
    pub notes: Option<String>,

    // Person demographic fields
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
    pub birth_date: Option<String>,
    pub place_of_birth: Option<String>,
    pub nationality: Option<String>,
    pub blood_type: Option<String>,
    pub marital_status: Option<String>,
    pub medical_notes: Option<String>,
    pub special_needs_notes: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub avatar_url: Option<String>,

    // Linked user account
    pub username: Option<String>,
    pub user_status: Option<String>,
    pub roles: Vec<String>,
    pub groups: Vec<String>,
    pub effective_permissions: Vec<String>,

    pub created_at: String,
    pub updated_at: String,
}

pub fn employee_detail_to_dto(d: EmployeeDetail) -> EmployeeDetailDto {
    let e = d.employee;
    let p = d.person;
    let full_name = p.full_name();
    let full_name_ar = p.full_name_ar();
    let username = d.user.as_ref().and_then(|u| u.email.clone());
    let user_status = d.user.as_ref().map(|u| u.status.clone());
    let groups = d.groups.into_iter().map(|g| g.name).collect();

    EmployeeDetailDto {
        id: e.id.to_string(),
        org_id: e.org_id.to_string(),
        person_id: e.person_id.to_string(),
        user_id: e.user_id.map(|u| u.to_string()),
        employee_number: e.employee_number,
        employee_type: e.employee_type,
        job_title: e.job_title,
        department: e.department,
        org_unit_id: e.org_unit_id.map(|o| o.to_string()),
        org_unit_name: d.org_unit_name,
        hire_date: e.hire_date.to_string(),
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
        birth_date: p.birth_date.map(|b| b.to_string()),
        place_of_birth: p.place_of_birth,
        nationality: p.nationality,
        blood_type: p.blood_type,
        marital_status: p.marital_status,
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

        created_at: e.created_at.to_rfc3339(),
        updated_at: e.updated_at.to_rfc3339(),
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AccountProvisioningResultDto {
    pub user_id: String,
    pub username: String,
    pub temporary_password: Option<String>,
    pub must_change_password: bool,
    pub delivery_channel: String,
    pub delivery_status: String,
}

pub fn provisioning_to_dto(r: AccountProvisioningResult) -> AccountProvisioningResultDto {
    AccountProvisioningResultDto {
        user_id: r.user_id.to_string(),
        username: r.username,
        temporary_password: r.temporary_password,
        must_change_password: r.must_change_password,
        delivery_channel: r.delivery_channel.to_string(),
        delivery_status: r.delivery_status.to_string(),
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RegisterEmployeeResponseDto {
    pub employee: EmployeeDetailDto,
    pub provisioning: Option<AccountProvisioningResultDto>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PaginatedEmployeesDto {
    pub items: Vec<EmployeeDetailDto>,
    pub total_count: u64,
    pub page: u64,
    pub page_size: u64,
    pub total_pages: u64,
}

// ── Request Inputs ───────────────────────────────────────────────

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AccountProvisioningConfigInput {
    pub enabled: bool,
    pub preferred_username: Option<String>,
    pub role: Option<String>,
    #[serde(default)]
    pub group_ids: Vec<String>,
    pub delivery_channel: Option<String>,
    pub custom_password: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RegisterEmployeeRequest {
    pub person_id: Option<String>,
    pub employee_number: Option<String>,
    pub employee_type: String,
    pub job_title: Option<String>,
    pub department: Option<String>,
    pub org_unit_id: Option<String>,
    pub hire_date: Option<String>,
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
    pub birth_date: Option<String>,
    pub place_of_birth: Option<String>,
    pub nationality: Option<String>,
    pub blood_type: Option<String>,
    pub marital_status: Option<String>,
    pub medical_notes: Option<String>,
    pub special_needs_notes: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub avatar_url: Option<String>,

    pub account: Option<AccountProvisioningConfigInput>,
}

#[derive(Deserialize, Debug)]
pub struct UpdateEmployeeRequest {
    pub employee_type: Option<String>,
    pub job_title: Option<String>,
    pub department: Option<String>,
    pub org_unit_id: Option<String>,
    pub hire_date: Option<String>,
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
    pub birth_date: Option<String>,
    pub place_of_birth: Option<String>,
    pub nationality: Option<String>,
    pub blood_type: Option<String>,
    pub marital_status: Option<String>,
    pub medical_notes: Option<String>,
    pub special_needs_notes: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub avatar_url: Option<String>,
}

#[derive(Deserialize, Default, Debug)]
pub struct EmployeeFilterInput {
    pub query: Option<String>,
    pub employee_type: Option<String>,
    pub department: Option<String>,
    pub status: Option<String>,
    pub org_unit_id: Option<String>,
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

#[derive(Deserialize, Debug)]
pub struct ProvisionEmployeeAccountInput {
    pub preferred_username: Option<String>,
    pub role: Option<String>,
    #[serde(default)]
    pub group_ids: Vec<String>,
    pub delivery_channel: Option<String>,
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

// ── Commands ─────────────────────────────────────────────────────

#[tauri::command]
pub async fn employee_register_employee(
    request: RegisterEmployeeRequest,
    state: State<'_, AppState>,
) -> Result<ApiResponse<RegisterEmployeeResponseDto>, String> {
    let ctx = make_ctx(&state);
    let org_id = ctx.tenant_id;

    let person_id = request.person_id.and_then(|id| Uuid::parse_str(&id).ok());
    let org_unit_id = request.org_unit_id.and_then(|id| Uuid::parse_str(&id).ok());
    let hire_date = request
        .hire_date
        .and_then(|d| NaiveDate::parse_from_str(&d, "%Y-%m-%d").ok());
    let birth_date = request
        .birth_date
        .and_then(|d| NaiveDate::parse_from_str(&d, "%Y-%m-%d").ok());

    let status = request.status.as_deref().and_then(|s| match s {
        "active" => Some(EmployeeStatus::Active),
        "on_leave" => Some(EmployeeStatus::OnLeave),
        "suspended" => Some(EmployeeStatus::Suspended),
        "terminated" => Some(EmployeeStatus::Terminated),
        "resigned" => Some(EmployeeStatus::Resigned),
        _ => None,
    });

    let employment_type = request.employment_type.as_deref().and_then(|e| match e {
        "full_time" => Some(EmploymentType::FullTime),
        "part_time" => Some(EmploymentType::PartTime),
        "contract" => Some(EmploymentType::Contract),
        "hourly" => Some(EmploymentType::Hourly),
        "volunteer" => Some(EmploymentType::Volunteer),
        _ => None,
    });

    let account_cfg = request.account.map(|acc| {
        let group_ids: Vec<Uuid> = acc
            .group_ids
            .into_iter()
            .filter_map(|g| Uuid::parse_str(&g).ok())
            .collect();
        AccountProvisioningConfig {
            enabled: acc.enabled,
            preferred_username: acc.preferred_username,
            role: acc.role,
            group_ids,
            delivery_channel: parse_delivery_channel(acc.delivery_channel.as_deref()),
            custom_password: acc.custom_password,
        }
    });

    let input = RegisterEmployeeInput {
        person_id,
        employee_number: request.employee_number,
        employee_type: request.employee_type,
        job_title: request.job_title,
        department: request.department,
        org_unit_id,
        hire_date,
        status,
        employment_type,
        work_shift: request.work_shift,
        notes: request.notes,
        first_name: request.first_name,
        middle_name: request.middle_name,
        last_name: request.last_name,
        first_name_ar: request.first_name_ar,
        middle_name_ar: request.middle_name_ar,
        last_name_ar: request.last_name_ar,
        display_name: request.display_name,
        national_id: request.national_id,
        passport_number: request.passport_number,
        gender: request.gender,
        birth_date,
        place_of_birth: request.place_of_birth,
        nationality: request.nationality,
        blood_type: request.blood_type,
        marital_status: request.marital_status,
        medical_notes: request.medical_notes,
        special_needs_notes: request.special_needs_notes,
        phone: request.phone,
        email: request.email,
        address: request.address,
        avatar_url: request.avatar_url,
        account: account_cfg,
    };

    let result = state
        .employee_service
        .register_employee(&ctx, org_id, input)
        .await
        .map_err(|e| e.to_string())?;

    let res_dto = RegisterEmployeeResponseDto {
        employee: employee_detail_to_dto(result.detail),
        provisioning: result.provisioning.map(provisioning_to_dto),
    };

    Ok(ok(res_dto))
}

#[tauri::command]
pub async fn employee_list_employees(
    filter: EmployeeFilterInput,
    state: State<'_, AppState>,
) -> Result<ApiResponse<PaginatedEmployeesDto>, String> {
    let ctx = make_ctx(&state);
    let org_id = ctx.tenant_id;
    let org_unit_id = filter.org_unit_id.and_then(|id| Uuid::parse_str(&id).ok());
    let page = filter.page.unwrap_or(1).max(1);
    let page_size = filter.page_size.unwrap_or(20).clamp(1, 100);
    let offset = (page.saturating_sub(1)) * page_size;

    let domain_filter = EmployeeFilter {
        search: filter.query,
        employee_type: filter.employee_type,
        department: filter.department,
        org_unit_id,
        status: filter.status,
        limit: Some(page_size),
        offset: Some(offset),
    };

    let list = state
        .employee_service
        .list_employees(&ctx, org_id, domain_filter)
        .await
        .map_err(|e| e.to_string())?;

    let dto = PaginatedEmployeesDto {
        items: list.items.into_iter().map(employee_detail_to_dto).collect(),
        total_count: list.total_count,
        page: list.page,
        page_size: list.page_size,
        total_pages: list.total_pages,
    };

    Ok(ok(dto))
}

#[tauri::command]
pub async fn employee_get_employee(
    id: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<EmployeeDetailDto>, String> {
    let ctx = make_ctx(&state);
    let employee_id = Uuid::parse_str(&id).map_err(|e| e.to_string())?;

    let detail = state
        .employee_service
        .get_employee(&ctx, employee_id)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("Employee {} not found", id))?;

    Ok(ok(employee_detail_to_dto(detail)))
}

#[tauri::command]
pub async fn employee_get_employee_stats(
    state: State<'_, AppState>,
) -> Result<ApiResponse<std::collections::HashMap<String, i64>>, String> {
    let ctx = make_ctx(&state);
    let org_id = ctx.tenant_id;

    let stats = state
        .employee_service
        .count_employees_by_status(&ctx, org_id)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(stats))
}

#[tauri::command]
pub async fn employee_update_employee(
    id: String,
    request: UpdateEmployeeRequest,
    state: State<'_, AppState>,
) -> Result<ApiResponse<EmployeeDetailDto>, String> {
    let ctx = make_ctx(&state);
    let employee_id = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    let org_unit_id = request.org_unit_id.and_then(|i| Uuid::parse_str(&i).ok());
    let hire_date = request
        .hire_date
        .and_then(|d| NaiveDate::parse_from_str(&d, "%Y-%m-%d").ok());
    let birth_date = request
        .birth_date
        .and_then(|d| NaiveDate::parse_from_str(&d, "%Y-%m-%d").ok());

    let status = request.status.as_deref().and_then(|s| match s {
        "active" => Some(EmployeeStatus::Active),
        "on_leave" => Some(EmployeeStatus::OnLeave),
        "suspended" => Some(EmployeeStatus::Suspended),
        "terminated" => Some(EmployeeStatus::Terminated),
        "resigned" => Some(EmployeeStatus::Resigned),
        _ => None,
    });

    let employment_type = request.employment_type.as_deref().and_then(|e| match e {
        "full_time" => Some(EmploymentType::FullTime),
        "part_time" => Some(EmploymentType::PartTime),
        "contract" => Some(EmploymentType::Contract),
        "hourly" => Some(EmploymentType::Hourly),
        "volunteer" => Some(EmploymentType::Volunteer),
        _ => None,
    });

    let input = UpdateEmployeeInput {
        employee_type: request.employee_type,
        job_title: request.job_title,
        department: request.department,
        org_unit_id,
        hire_date,
        status,
        employment_type,
        work_shift: request.work_shift,
        notes: request.notes,
        first_name: request.first_name,
        middle_name: request.middle_name,
        last_name: request.last_name,
        first_name_ar: request.first_name_ar,
        middle_name_ar: request.middle_name_ar,
        last_name_ar: request.last_name_ar,
        display_name: request.display_name,
        national_id: request.national_id,
        passport_number: request.passport_number,
        gender: request.gender,
        birth_date,
        place_of_birth: request.place_of_birth,
        nationality: request.nationality,
        blood_type: request.blood_type,
        marital_status: request.marital_status,
        medical_notes: request.medical_notes,
        special_needs_notes: request.special_needs_notes,
        phone: request.phone,
        email: request.email,
        address: request.address,
        avatar_url: request.avatar_url,
    };

    let updated = state
        .employee_service
        .update_employee(&ctx, employee_id, input)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(employee_detail_to_dto(updated)))
}

#[tauri::command]
pub async fn employee_update_employee_status(
    id: String,
    status: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<EmployeeDetailDto>, String> {
    let ctx = make_ctx(&state);
    let employee_id = Uuid::parse_str(&id).map_err(|e| e.to_string())?;

    let new_status = match status.as_str() {
        "active" => EmployeeStatus::Active,
        "on_leave" => EmployeeStatus::OnLeave,
        "suspended" => EmployeeStatus::Suspended,
        "terminated" => EmployeeStatus::Terminated,
        "resigned" => EmployeeStatus::Resigned,
        other => return Err(format!("Invalid employee status: {}", other)),
    };

    let updated = state
        .employee_service
        .update_employee_status(&ctx, employee_id, new_status)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(employee_detail_to_dto(updated)))
}

#[tauri::command]
pub async fn employee_provision_account(
    id: String,
    request: ProvisionEmployeeAccountInput,
    state: State<'_, AppState>,
) -> Result<ApiResponse<AccountProvisioningResultDto>, String> {
    let ctx = make_ctx(&state);
    let employee_id = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    let group_ids: Vec<Uuid> = request
        .group_ids
        .into_iter()
        .filter_map(|g| Uuid::parse_str(&g).ok())
        .collect();
    let delivery_channel = parse_delivery_channel(request.delivery_channel.as_deref());

    let res = state
        .employee_service
        .provision_employee_account(
            &ctx,
            employee_id,
            request.preferred_username,
            request.role,
            group_ids,
            delivery_channel,
        )
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(provisioning_to_dto(res)))
}

#[tauri::command]
pub async fn employee_assign_groups(
    id: String,
    group_ids: Vec<String>,
    state: State<'_, AppState>,
) -> Result<ApiResponse<bool>, String> {
    let ctx = make_ctx(&state);
    let employee_id = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    let parsed_group_ids: Vec<Uuid> = group_ids
        .into_iter()
        .filter_map(|g| Uuid::parse_str(&g).ok())
        .collect();

    state
        .employee_service
        .assign_employee_groups(&ctx, employee_id, parsed_group_ids)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(true))
}

// ── Bulk Registration Command ───────────────────────────────────

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BulkRegisterEmployeeCommandItem {
    pub row_index: usize,
    pub registration: RegisterEmployeeRequest,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BulkRegisterEmployeeCommandResultDto {
    pub total: usize,
    pub succeeded: usize,
    pub failed: usize,
    pub items: Vec<BulkRegisterEmployeeItemCommandResultDto>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BulkRegisterEmployeeItemCommandResultDto {
    pub row_index: usize,
    pub success: bool,
    pub employee_id: Option<String>,
    pub employee_number: Option<String>,
    pub user_id: Option<String>,
    pub error: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BulkRegisterEmployeeJobDto {
    pub id: String,
    pub status: String,
    pub total: usize,
    pub processed: usize,
    pub succeeded: usize,
    pub failed: usize,
    pub result: Option<BulkRegisterEmployeeCommandResultDto>,
    pub error: Option<String>,
}

#[derive(Clone, Default)]
pub struct BulkRegisterEmployeeJobRegistry {
    pub jobs: Arc<RwLock<HashMap<String, BulkRegisterEmployeeJobDto>>>,
}

fn to_domain_employee_items(
    items: Vec<BulkRegisterEmployeeCommandItem>,
) -> Vec<orbis_app::services::employee_service::BulkRegisterEmployeeItemInput> {
    items
        .into_iter()
        .map(|item| {
            let registration = item.registration;
            let input = orbis_app::services::employee_service::RegisterEmployeeInput {
                person_id: registration
                    .person_id
                    .as_deref()
                    .and_then(|id| Uuid::parse_str(id).ok()),
                employee_number: registration
                    .employee_number
                    .as_deref()
                    .map(str::trim)
                    .filter(|value| !value.is_empty())
                    .map(str::to_string),
                employee_type: registration.employee_type,
                job_title: registration.job_title,
                department: registration.department,
                org_unit_id: registration
                    .org_unit_id
                    .as_deref()
                    .and_then(|id| Uuid::parse_str(id).ok()),
                hire_date: registration
                    .hire_date
                    .as_deref()
                    .and_then(|date| NaiveDate::parse_from_str(date, "%Y-%m-%d").ok()),
                status: Some(EmployeeStatus::Active),
                employment_type: Some(EmploymentType::FullTime),
                work_shift: registration.work_shift,
                notes: registration.notes,
                first_name: registration.first_name,
                middle_name: registration.middle_name,
                last_name: registration.last_name,
                first_name_ar: registration.first_name_ar,
                middle_name_ar: registration.middle_name_ar,
                last_name_ar: registration.last_name_ar,
                display_name: registration.display_name,
                national_id: registration.national_id,
                passport_number: registration.passport_number,
                gender: registration.gender,
                birth_date: registration
                    .birth_date
                    .as_deref()
                    .and_then(|date| NaiveDate::parse_from_str(date, "%Y-%m-%d").ok()),
                place_of_birth: registration.place_of_birth,
                nationality: registration.nationality,
                blood_type: registration.blood_type,
                marital_status: registration.marital_status,
                medical_notes: registration.medical_notes,
                special_needs_notes: registration.special_needs_notes,
                phone: registration.phone,
                email: registration.email,
                address: registration.address,
                avatar_url: None,
                account: registration
                    .account
                    .map(|account| AccountProvisioningConfig {
                        enabled: account.enabled,
                        preferred_username: account.preferred_username,
                        role: account.role,
                        group_ids: account
                            .group_ids
                            .into_iter()
                            .filter_map(|id| Uuid::parse_str(&id).ok())
                            .collect(),
                        delivery_channel: account.delivery_channel.and_then(
                            |channel| match channel.to_lowercase().as_str() {
                                "email" => Some(DeliveryChannel::Email),
                                "sms" => Some(DeliveryChannel::Sms),
                                "whatsapp" => Some(DeliveryChannel::WhatsApp),
                                _ => Some(DeliveryChannel::Manual),
                            },
                        ),
                        custom_password: account.custom_password,
                    }),
            };
            orbis_app::services::employee_service::BulkRegisterEmployeeItemInput {
                row_index: item.row_index,
                registration: input,
            }
        })
        .collect()
}

#[tauri::command]
pub async fn employee_start_bulk_register_job(
    items: Vec<BulkRegisterEmployeeCommandItem>,
    atomic: Option<bool>,
    dry_run: Option<bool>,
    state: State<'_, AppState>,
    jobs: State<'_, BulkRegisterEmployeeJobRegistry>,
) -> Result<ApiResponse<BulkRegisterEmployeeJobDto>, String> {
    let total = items.len();
    let job_id = Uuid::new_v4().to_string();
    let job = BulkRegisterEmployeeJobDto {
        id: job_id.clone(),
        status: "running".to_string(),
        total,
        processed: 0,
        succeeded: 0,
        failed: 0,
        result: None,
        error: None,
    };
    jobs.jobs.write().await.insert(job_id.clone(), job.clone());

    let app_state = state.inner().clone();
    let ctx = make_ctx(&app_state);
    let registry = jobs.inner().clone();
    let domain_items = to_domain_employee_items(items);

    tauri::async_runtime::spawn(async move {
        let mut processed = 0usize;
        let mut succeeded = 0usize;
        let mut failed = 0usize;
        let mut result_items = Vec::new();

        for chunk in domain_items.chunks(100) {
            match app_state
                .employee_service
                .bulk_register_employees(
                    &ctx,
                    ctx.tenant_id,
                    orbis_app::services::employee_service::BulkRegisterEmployeesInput {
                        items: chunk.to_vec(),
                        atomic,
                        dry_run,
                    },
                )
                .await
            {
                Ok(res) => {
                    succeeded += res.succeeded;
                    failed += res.failed;
                    processed += res.total;
                    result_items.extend(res.items.into_iter().map(|item| {
                        BulkRegisterEmployeeItemCommandResultDto {
                            row_index: item.row_index,
                            success: item.success,
                            employee_id: item.employee_id.map(|id| id.to_string()),
                            employee_number: item.employee_number,
                            user_id: item.user_id.map(|id| id.to_string()),
                            error: item.error,
                        }
                    }));
                }
                Err(error) => {
                    let reason = error.to_string();
                    processed += chunk.len();
                    failed += chunk.len();
                    result_items.extend(chunk.iter().map(|item| {
                        BulkRegisterEmployeeItemCommandResultDto {
                            row_index: item.row_index,
                            success: false,
                            employee_id: None,
                            employee_number: None,
                            user_id: None,
                            error: Some(reason.clone()),
                        }
                    }));
                }
            }

            let mut jobs = registry.jobs.write().await;
            if let Some(current) = jobs.get_mut(&job_id) {
                current.processed = processed;
                current.succeeded = succeeded;
                current.failed = failed;
            }
        }

        let mut jobs = registry.jobs.write().await;
        if let Some(current) = jobs.get_mut(&job_id) {
            current.status = "completed".to_string();
            current.processed = total;
            current.result = Some(BulkRegisterEmployeeCommandResultDto {
                total,
                succeeded,
                failed,
                items: result_items,
            });
        }
    });

    Ok(ok(job))
}

#[tauri::command]
pub async fn employee_get_bulk_register_job(
    job_id: String,
    jobs: State<'_, BulkRegisterEmployeeJobRegistry>,
) -> Result<ApiResponse<Option<BulkRegisterEmployeeJobDto>>, String> {
    Ok(ok(jobs.jobs.read().await.get(&job_id).cloned()))
}

#[tauri::command]
pub async fn employee_delete_employee(
    id: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<bool>, String> {
    let ctx = make_ctx(&state);
    let employee_id = Uuid::parse_str(&id).map_err(|e| e.to_string())?;

    state
        .employee_service
        .delete_employee(&ctx, employee_id)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(true))
}

#[tauri::command]
pub async fn employee_restore_employee(
    id: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<EmployeeDetailDto>, String> {
    let ctx = make_ctx(&state);
    let employee_id = Uuid::parse_str(&id).map_err(|e| e.to_string())?;

    let restored = state
        .employee_service
        .restore_employee(&ctx, employee_id)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(employee_detail_to_dto(restored)))
}
