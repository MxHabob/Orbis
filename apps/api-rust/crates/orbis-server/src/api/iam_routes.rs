//! IAM routes — Authentication, users, roles, permissions.

use axum::{
    Json, Router,
    extract::State,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use super::employee_routes::AccountProvisioningResultDto;
use super::extractors::AuthContext;
use super::{ApiError, ApiResponse};
use crate::state::AppState;

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct LinkedDependentDto {
    pub student_id: Uuid,
    pub full_name: String,
    pub full_name_ar: Option<String>,
    pub relationship: String,
    pub is_primary_contact: bool,
    pub is_emergency_contact: bool,
    pub is_fee_payer: bool,
    pub current_grade_level: Option<String>,
    pub current_section_name: Option<String>,
}

impl From<orbis_app::services::auth_service::LinkedDependentDto> for LinkedDependentDto {
    fn from(d: orbis_app::services::auth_service::LinkedDependentDto) -> Self {
        Self {
            student_id: d.student_id,
            full_name: d.full_name,
            full_name_ar: d.full_name_ar,
            relationship: d.relationship,
            is_primary_contact: d.is_primary_contact,
            is_emergency_contact: d.is_emergency_contact,
            is_fee_payer: d.is_fee_payer,
            current_grade_level: d.current_grade_level,
            current_section_name: d.current_section_name,
        }
    }
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct EffectiveContextDto {
    pub effective_role: String,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
    pub active_period_id: Option<Uuid>,
    pub student_id: Option<Uuid>,
    pub dependents: Vec<LinkedDependentDto>,
    pub teaching_classes: Vec<TeachingClassDto>,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct TeachingClassDto {
    pub section_id: Uuid,
    pub name: String,
    pub name_ar: Option<String>,
    pub level: Option<String>,
}

impl From<orbis_app::services::auth_service::EffectiveContextDto> for EffectiveContextDto {
    fn from(c: orbis_app::services::auth_service::EffectiveContextDto) -> Self {
        Self {
            effective_role: c.effective_role,
            roles: c.roles,
            permissions: c.permissions,
            active_period_id: c.active_period_id,
            student_id: c.student_id,
            dependents: c.dependents.into_iter().map(Into::into).collect(),
            teaching_classes: c
                .teaching_classes
                .into_iter()
                .map(|class| TeachingClassDto {
                    section_id: class.section_id,
                    name: class.name,
                    name_ar: class.name_ar,
                    level: class.level,
                })
                .collect(),
        }
    }
}

// ── DTOs ────────────────────────────────────────────────────────

#[derive(Deserialize, ToSchema)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, ToSchema)]
pub struct LoginResponse {
    pub token: Option<String>,
    pub refresh_token: Option<String>,
    pub user: UserDto,
    pub effective_context: Option<EffectiveContextDto>,
    pub mfa_required: bool,
    pub mfa_token: Option<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct RegisterUserRequest {
    pub email: String,
    pub full_name: String,
    pub password: Option<String>,
    #[serde(default)]
    pub role: Option<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct ProvisionUserRequest {
    pub person_id: Option<Uuid>,
    pub employee_id: Option<Uuid>,
    pub student_id: Option<Uuid>,
    pub guardian_id: Option<Uuid>,
    pub actor_type: Option<String>,
    pub full_name: Option<String>,
    pub full_name_ar: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub preferred_username: Option<String>,
    pub custom_password: Option<String>,
    pub roles: Vec<String>,
    #[serde(default)]
    pub group_ids: Vec<Uuid>,
    pub delivery_channel: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct UserDto {
    pub id: Uuid,
    pub org_id: Uuid,
    pub person_id: Uuid,
    pub email: Option<String>,
    pub full_name: String,
    pub full_name_ar: Option<String>,
    pub avatar_url: Option<String>,
    pub status: String,
    pub is_email_verified: bool,
    pub is_phone_verified: bool,
    pub is_mfa_enabled: bool,
    #[serde(default)]
    pub roles: Vec<String>,
    pub phone: Option<String>,
    pub national_id: Option<String>,
    pub gender: Option<String>,
    pub birth_date: Option<String>,
    pub address: Option<String>,
    pub is_active: bool,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

pub fn user_to_dto(
    u: orbis_domain::entities::user::User,
    p: Option<&orbis_domain::entities::person::Person>,
    roles: Vec<String>,
) -> UserDto {
    let (full_name, full_name_ar, avatar_url, phone, national_id, gender, birth_date, address) =
        match p {
            Some(person) => (
                person.full_name(),
                person.full_name_ar(),
                person.avatar_url.clone(),
                person.phone.clone(),
                person.national_id.clone(),
                person.gender.clone(),
                person.birth_date.map(|d| d.to_string()),
                person.address.clone(),
            ),
            None => (String::new(), None, None, None, None, None, None, None),
        };
    let is_active = u.status == "active";
    UserDto {
        id: u.id,
        org_id: u.org_id,
        person_id: u.person_id,
        email: u.email,
        full_name,
        full_name_ar,
        avatar_url,
        status: u.status,
        is_email_verified: u.is_email_verified,
        is_phone_verified: u.is_phone_verified,
        is_mfa_enabled: u.is_mfa_enabled,
        roles,
        phone,
        national_id,
        gender,
        birth_date,
        address,
        is_active,
        created_by: u.created_by,
        updated_by: u.updated_by,
        created_at: u.created_at,
        updated_at: u.updated_at,
    }
}

#[derive(Serialize, ToSchema)]
pub struct SessionResponse {
    pub user: UserDto,
    pub org_id: Uuid,
    pub effective_context: EffectiveContextDto,
}

fn canonicalize_resource(res: &str) -> String {
    match res {
        "OrgUnit" => "org_unit".to_string(),
        "FeeTemplate" => "fee_template".to_string(),
        "DiscountTemplate" => "discount_template".to_string(),
        "StudentFee" => "student_fee".to_string(),
        "JournalEntry" => "journal_entry".to_string(),
        "ExamSession" => "exam_session".to_string(),
        "ExamSchedule" => "exam_schedule".to_string(),
        "SeatingNumber" => "seating_number".to_string(),
        "SecretNumber" => "secret_number".to_string(),
        "AcademicAdvisor" => "academic_advisor".to_string(),
        "DepartmentHead" => "department_head".to_string(),
        other => other.to_lowercase(),
    }
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct RoleDto {
    pub id: String,
    pub name: String,
    pub name_ar: Option<String>,
    pub is_system: bool,
    pub permissions: std::collections::HashMap<String, Vec<String>>,
}

#[derive(Deserialize, ToSchema)]
pub struct SaveRoleRequest {
    pub id: String,
    pub name: String,
    pub name_ar: Option<String>,
    pub permissions: Option<std::collections::HashMap<String, Vec<String>>>,
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct PermissionRuleDto {
    pub id: Option<String>,
    pub role: String,
    pub document_type: String,
    pub level: Option<i32>,
    pub only_if_creator: Option<bool>,
    pub actions: Vec<String>,
}

// ── Handlers ────────────────────────────────────────────────────

#[utoipa::path(
    post,
    path = "/api/v1/auth/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = LoginResponse),
        (status = 401, description = "Invalid credentials")
    ),
    tag = "Auth"
)]
async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<ApiResponse<LoginResponse>>, ApiError> {
    let ctx = orbis_app::context::RequestContext {
        tenant_id: Uuid::default(),
        actor: orbis_domain::iam::actor::Actor::System,
        request_id: Uuid::new_v4().to_string(),
        source: orbis_app::context::ActionSource::Api,
        org_unit_path: None,
    };

    let res = state
        .auth_service
        .login(&ctx, &payload.email, &payload.password)
        .await?;
    let roles = res
        .effective_context
        .as_ref()
        .map(|context| context.roles.clone())
        .unwrap_or_default();
    let user_dto = user_to_dto(res.user, res.person.as_ref(), roles);

    Ok(ApiResponse::ok(LoginResponse {
        token: res.token,
        refresh_token: res.refresh_token,
        user: user_dto,
        effective_context: res.effective_context.map(Into::into),
        mfa_required: res.mfa_required,
        mfa_token: res.mfa_token,
    }))
}

#[utoipa::path(
    get,
    path = "/api/v1/auth/session",
    responses(
        (status = 200, description = "Current session info", body = SessionResponse),
        (status = 401, description = "Not authenticated")
    ),
    security(("bearer" = [])),
    tag = "Auth"
)]
async fn get_session(
    ctx: AuthContext,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<SessionResponse>>, ApiError> {
    let user_id = ctx
        .actor
        .id()
        .ok_or_else(|| ApiError::unauthorized("No user identity in token"))?;

    let user = state
        .auth_service
        .get_user(user_id)
        .await?
        .ok_or_else(|| ApiError::not_found("User not found"))?;

    let person = state.person_repo.find_by_id(user.person_id).await?;
    let effective_context = state
        .auth_service
        .build_effective_context(&user, person.as_ref())
        .await?;
    let roles = effective_context.roles.clone();
    let user_dto = user_to_dto(user, person.as_ref(), roles);

    Ok(ApiResponse::ok(SessionResponse {
        org_id: ctx.tenant_id,
        user: user_dto,
        effective_context: effective_context.into(),
    }))
}

#[utoipa::path(
    post,
    path = "/api/v1/iam/users",
    request_body = RegisterUserRequest,
    responses(
        (status = 201, description = "User registered", body = UserDto),
        (status = 409, description = "Email already exists")
    ),
    security(("bearer" = [])),
    tag = "IAM"
)]
async fn register_user(
    ctx: AuthContext,
    State(state): State<AppState>,
    Json(payload): Json<RegisterUserRequest>,
) -> Result<Json<ApiResponse<UserDto>>, ApiError> {
    let user = state
        .auth_service
        .register_user(
            &ctx,
            payload.full_name,
            Some(payload.email),
            payload.password,
            payload.role.clone(),
            Some(ctx.tenant_id),
        )
        .await?;

    if let Some(role) = payload.role {
        use orbis_infra::models::user_role_assignment;
        use sea_orm::{ActiveModelTrait, Set};
        let model = user_role_assignment::ActiveModel {
            id: Set(Uuid::new_v4()),
            user_id: Set(user.id),
            role_id: Set(role),
            org_id: Set(ctx.tenant_id),
            scope_type: Set("global".to_string()),
            scope_id: Set(None),
            assigned_by: Set(None),
            created_at: Set(chrono::Utc::now()),
        };
        let _ = model.insert(&state.db).await;
    }

    let person = state.person_repo.find_by_id(user.person_id).await?;
    let roles = state
        .permission_checker
        .get_user_roles(user.id, user.org_id)
        .await
        .unwrap_or_default();

    Ok(ApiResponse::ok(user_to_dto(user, person.as_ref(), roles)))
}

#[utoipa::path(
    post,
    path = "/api/v1/iam/users/provision",
    request_body = ProvisionUserRequest,
    responses(
        (status = 200, description = "User account provisioned successfully", body = AccountProvisioningResultDto),
        (status = 400, description = "Validation error"),
        (status = 409, description = "User already exists")
    ),
    security(("bearer" = [])),
    tag = "IAM"
)]
async fn provision_user(
    ctx: AuthContext,
    State(state): State<AppState>,
    Json(payload): Json<ProvisionUserRequest>,
) -> Result<Json<ApiResponse<AccountProvisioningResultDto>>, ApiError> {
    use orbis_app::services::identity_provisioning_service::ProvisionAccountInput;
    use orbis_domain::entities::identity_provisioning::{ActorType, DeliveryChannel};

    // 1. Resolve Person ID & Actor Type
    let mut resolved_person_id = payload.person_id;
    let mut actor_type = payload
        .actor_type
        .as_deref()
        .map(ActorType::from)
        .unwrap_or(ActorType::Custom);

    if let Some(emp_id) = payload.employee_id {
        let emp = state
            .employee_repo
            .find_by_id(emp_id)
            .await?
            .ok_or_else(|| ApiError::not_found(format!("Employee {} not found", emp_id)))?;
        resolved_person_id = Some(emp.person_id);
        actor_type = ActorType::Employee;
    } else if let Some(std_id) = payload.student_id {
        let std = state
            .student_repo
            .find_by_id(std_id)
            .await?
            .ok_or_else(|| ApiError::not_found(format!("Student {} not found", std_id)))?;
        resolved_person_id = Some(std.person_id);
        actor_type = ActorType::Student;
    } else if let Some(g_id) = payload.guardian_id {
        resolved_person_id = Some(g_id);
        actor_type = ActorType::Guardian;
    }

    let person_id = if let Some(pid) = resolved_person_id {
        pid
    } else {
        // Create new person
        let full_name = payload
            .full_name
            .clone()
            .unwrap_or_else(|| "User".to_string());
        let parts: Vec<&str> = full_name.split_whitespace().collect();
        let first_name = parts.first().copied().unwrap_or("User").to_string();
        let last_name = if parts.len() > 1 {
            parts[1..].join(" ")
        } else {
            String::new()
        };

        let new_person = orbis_domain::entities::person::Person {
            id: Uuid::new_v4(),
            org_id: Some(ctx.tenant_id),
            first_name,
            middle_name: None,
            last_name,
            first_name_ar: payload.full_name_ar.clone(),
            middle_name_ar: None,
            last_name_ar: None,
            display_name: payload.full_name.clone(),
            national_id: None,
            passport_number: None,
            birth_date: None,
            place_of_birth: None,
            gender: None,
            nationality: None,
            blood_type: None,
            marital_status: None,
            medical_notes: None,
            special_needs_notes: None,
            email: payload.email.clone(),
            phone: payload.phone.clone(),
            address: None,
            avatar_url: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        state.person_repo.save(&new_person).await?;
        new_person.id
    };

    let delivery_channel = match payload.delivery_channel.as_deref() {
        Some("email") => Some(DeliveryChannel::Email),
        Some("sms") => Some(DeliveryChannel::Sms),
        Some("whatsapp") => Some(DeliveryChannel::WhatsApp),
        _ => Some(DeliveryChannel::Manual),
    };

    let input = ProvisionAccountInput {
        person_id,
        actor_type,
        preferred_username: payload.preferred_username,
        email: payload.email,
        phone: payload.phone,
        roles: payload.roles,
        group_ids: payload.group_ids,
        delivery_channel,
        custom_password: payload.custom_password,
    };

    let result = state
        .identity_provisioning_service
        .provision_account(&ctx, ctx.tenant_id, input)
        .await?;

    // If employee_id was provided, update employee.user_id
    if let Some(emp_id) = payload.employee_id {
        if let Ok(Some(mut emp)) = state.employee_repo.find_by_id(emp_id).await {
            emp.user_id = Some(result.user_id);
            emp.updated_at = chrono::Utc::now();
            let _ = state.employee_repo.save(&emp).await;
        }
    }

    // If student_id was provided, update student.user_id
    if let Some(std_id) = payload.student_id {
        if let Ok(Some(mut std)) = state.student_repo.find_by_id(std_id).await {
            std.user_id = Some(result.user_id);
            std.updated_at = chrono::Utc::now();
            let _ = state.student_repo.save(&std).await;
        }
    }

    Ok(ApiResponse::ok(AccountProvisioningResultDto::from(result)))
}

#[utoipa::path(
    get,
    path = "/api/v1/iam/users",
    responses(
        (status = 200, description = "List of users", body = Vec<UserDto>)
    ),
    security(("bearer" = [])),
    tag = "IAM"
)]
async fn list_users(
    ctx: AuthContext,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<UserDto>>>, ApiError> {
    let users = state.user_repo.find_by_org(ctx.tenant_id).await?;
    let mut dtos = Vec::new();
    for u in users {
        let p = state
            .person_repo
            .find_by_id(u.person_id)
            .await
            .unwrap_or(None);
        let roles = state
            .permission_checker
            .get_user_roles(u.id, u.org_id)
            .await
            .unwrap_or_default();
        dtos.push(user_to_dto(u, p.as_ref(), roles));
    }
    Ok(ApiResponse::ok(dtos))
}

#[utoipa::path(
    get,
    path = "/api/v1/iam/users/{id}",
    params(("id" = Uuid, Path, description = "User ID")),
    responses(
        (status = 200, description = "User details", body = UserDto),
        (status = 404, description = "User not found")
    ),
    security(("bearer" = [])),
    tag = "IAM"
)]
async fn get_user(
    ctx: AuthContext,
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<Uuid>,
) -> Result<Json<ApiResponse<UserDto>>, ApiError> {
    let user = state
        .user_repo
        .find_by_id(id)
        .await?
        .ok_or_else(|| ApiError::not_found(format!("User {} not found", id)))?;

    // Tenant isolation
    if user.org_id != ctx.tenant_id && !ctx.actor.is_system() {
        return Err(ApiError::forbidden("Access denied to this user"));
    }

    let person = state.person_repo.find_by_id(user.person_id).await?;
    let roles = state
        .permission_checker
        .get_user_roles(user.id, user.org_id)
        .await
        .unwrap_or_default();
    Ok(ApiResponse::ok(user_to_dto(user, person.as_ref(), roles)))
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateUserRequest {
    pub full_name: Option<String>,
    pub full_name_ar: Option<String>,
    pub phone: Option<String>,
    pub national_id: Option<String>,
    pub gender: Option<String>,
    pub birth_date: Option<String>,
    pub address: Option<String>,
    pub avatar_url: Option<String>,
    pub is_active: Option<bool>,
    pub status: Option<String>,
    pub roles: Option<Vec<String>>,
}

#[utoipa::path(
    put,
    path = "/api/v1/iam/users/{id}",
    params(("id" = Uuid, Path, description = "User ID")),
    request_body = UpdateUserRequest,
    responses(
        (status = 200, description = "User updated successfully", body = UserDto),
        (status = 404, description = "User not found")
    ),
    security(("bearer" = [])),
    tag = "IAM"
)]
pub async fn update_user(
    ctx: AuthContext,
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<Uuid>,
    Json(p): Json<UpdateUserRequest>,
) -> Result<Json<ApiResponse<UserDto>>, ApiError> {
    let mut user = state
        .user_repo
        .find_by_id(id)
        .await?
        .ok_or_else(|| ApiError::not_found(format!("User {} not found", id)))?;

    if user.org_id != ctx.tenant_id && !ctx.actor.is_system() {
        return Err(ApiError::forbidden("Access denied to this user"));
    }

    if let Some(ref name) = p.full_name {
        let _ = state
            .auth_service
            .update_user(&ctx, id, Some(name.clone()))
            .await;
    }

    if let Some(is_active) = p.is_active {
        user.status = if is_active {
            "active".to_string()
        } else {
            "inactive".to_string()
        };
        user.updated_at = chrono::Utc::now();
        state.user_repo.save(&user).await?;
    } else if let Some(status) = p.status {
        user.status = status;
        user.updated_at = chrono::Utc::now();
        state.user_repo.save(&user).await?;
    }

    // Person updates
    if let Ok(Some(mut person)) = state.person_repo.find_by_id(user.person_id).await {
        let mut changed = false;
        if let Some(phone) = p.phone {
            person.phone = if phone.is_empty() { None } else { Some(phone) };
            changed = true;
        }
        if let Some(name_ar) = p.full_name_ar {
            person.first_name_ar = if name_ar.is_empty() {
                None
            } else {
                Some(name_ar)
            };
            changed = true;
        }
        if let Some(nid) = p.national_id {
            person.national_id = if nid.is_empty() { None } else { Some(nid) };
            changed = true;
        }
        if let Some(g) = p.gender {
            person.gender = if g.is_empty() { None } else { Some(g) };
            changed = true;
        }
        if let Some(addr) = p.address {
            person.address = if addr.is_empty() { None } else { Some(addr) };
            changed = true;
        }
        if let Some(avatar_url) = p.avatar_url {
            person.avatar_url = if avatar_url.is_empty() {
                None
            } else {
                Some(avatar_url)
            };
            changed = true;
        }
        if let Some(dob_str) = p.birth_date {
            if let Ok(d) = chrono::NaiveDate::parse_from_str(&dob_str, "%Y-%m-%d") {
                person.birth_date = Some(d);
                changed = true;
            }
        }
        if changed {
            person.updated_at = chrono::Utc::now();
            let _ = state.person_repo.save(&person).await;
        }
    }

    // Roles updates
    if let Some(new_roles) = p.roles {
        use orbis_infra::models::user_role_assignment;
        use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};

        let _ = user_role_assignment::Entity::delete_many()
            .filter(user_role_assignment::Column::UserId.eq(user.id))
            .exec(&state.db)
            .await;

        for r in new_roles {
            let model = user_role_assignment::ActiveModel {
                id: Set(Uuid::new_v4()),
                user_id: Set(user.id),
                role_id: Set(r),
                org_id: Set(ctx.tenant_id),
                scope_type: Set("global".to_string()),
                scope_id: Set(None),
                assigned_by: Set(ctx.actor.id().map(|id| id.to_string())),
                created_at: Set(chrono::Utc::now()),
            };
            let _ = model.insert(&state.db).await;
        }
    }

    let person = state.person_repo.find_by_id(user.person_id).await?;
    let roles = state
        .permission_checker
        .get_user_roles(user.id, user.org_id)
        .await
        .unwrap_or_default();
    Ok(ApiResponse::ok(user_to_dto(user, person.as_ref(), roles)))
}

#[utoipa::path(
    post,
    path = "/api/v1/iam/users/{id}/reset-password",
    params(("id" = Uuid, Path, description = "User ID")),
    responses(
        (status = 200, description = "Password reset successfully", body = AccountProvisioningResultDto),
        (status = 404, description = "User not found")
    ),
    security(("bearer" = [])),
    tag = "IAM"
)]
pub async fn reset_user_password(
    ctx: AuthContext,
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<Uuid>,
) -> Result<Json<ApiResponse<AccountProvisioningResultDto>>, ApiError> {
    use orbis_domain::entities::identity_provisioning::DeliveryChannel;

    let res = state
        .identity_provisioning_service
        .reset_password(&ctx, id, Some(DeliveryChannel::Manual))
        .await?;

    Ok(ApiResponse::ok(AccountProvisioningResultDto::from(res)))
}

#[utoipa::path(
    get,
    path = "/api/v1/iam/roles",
    responses((status = 200, description = "List of roles", body = Vec<RoleDto>)),
    security(("bearer" = [])),
    tag = "IAM"
)]
pub async fn list_roles(
    ctx: AuthContext,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<RoleDto>>>, ApiError> {
    use orbis_infra::models::{role, role_permission};
    use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

    let permissions = role_permission::Entity::find()
        .filter(role_permission::Column::OrgId.eq(ctx.tenant_id))
        .all(&state.db)
        .await
        .unwrap_or_default();

    let mut perm_map: std::collections::HashMap<
        String,
        std::collections::HashMap<String, Vec<String>>,
    > = std::collections::HashMap::new();
    for p in permissions {
        let role_perms = perm_map.entry(p.role_id.clone()).or_default();
        let res_key = canonicalize_resource(&p.resource);
        let acts = role_perms.entry(res_key).or_default();
        if !acts.contains(&p.action) {
            acts.push(p.action);
        }
    }

    let mut result_roles = Vec::new();
    if let Ok(db_roles) = role::Entity::find()
        .filter(role::Column::OrgId.eq(ctx.tenant_id))
        .all(&state.db)
        .await
    {
        for r in db_roles {
            let role_perms = perm_map.remove(&r.id).unwrap_or_default();
            result_roles.push(RoleDto {
                id: r.id,
                name: r.name,
                name_ar: r.name_ar,
                is_system: r.is_system,
                permissions: role_perms,
            });
        }
    }

    Ok(ApiResponse::ok(result_roles))
}

#[utoipa::path(
    post,
    path = "/api/v1/iam/roles",
    request_body = SaveRoleRequest,
    responses((status = 200, description = "Role saved", body = RoleDto)),
    security(("bearer" = [])),
    tag = "IAM"
)]
pub async fn save_role(
    ctx: AuthContext,
    State(state): State<AppState>,
    Json(request): Json<SaveRoleRequest>,
) -> Result<Json<ApiResponse<RoleDto>>, ApiError> {
    use orbis_infra::models::{role, role_permission};
    use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};

    let role_id = request.id.clone();
    let now = chrono::Utc::now();

    let existing = role::Entity::find()
        .filter(role::Column::OrgId.eq(ctx.tenant_id))
        .filter(role::Column::Id.eq(&role_id))
        .one(&state.db)
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?;

    let is_system = existing.as_ref().map(|e| e.is_system).unwrap_or(false);

    if let Some(existing_model) = existing {
        let mut active: role::ActiveModel = existing_model.into();
        active.name = Set(request.name.clone());
        active.name_ar = Set(request.name_ar.clone());
        active.updated_at = Set(now);
        let _ = active
            .update(&state.db)
            .await
            .map_err(|e| ApiError::internal(e.to_string()))?;
    } else {
        let active_role = role::ActiveModel {
            id: Set(role_id.clone()),
            org_id: Set(ctx.tenant_id),
            name: Set(request.name.clone()),
            name_ar: Set(request.name_ar.clone()),
            is_system: Set(false),
            created_at: Set(now),
            updated_at: Set(now),
        };
        let _ = active_role
            .insert(&state.db)
            .await
            .map_err(|e| ApiError::internal(e.to_string()))?;
    }

    if let Some(perms) = request.permissions.clone() {
        let _ = role_permission::Entity::delete_many()
            .filter(role_permission::Column::OrgId.eq(ctx.tenant_id))
            .filter(role_permission::Column::RoleId.eq(&role_id))
            .exec(&state.db)
            .await;

        for (resource, actions) in &perms {
            for action in actions {
                let p_model = role_permission::ActiveModel {
                    id: Set(Uuid::new_v4()),
                    role_id: Set(role_id.clone()),
                    org_id: Set(ctx.tenant_id),
                    resource: Set(resource.clone()),
                    action: Set(action.clone()),
                    condition: Set(None),
                    created_at: Set(now),
                };
                let _ = p_model.insert(&state.db).await;
            }
        }
    }

    Ok(ApiResponse::ok(RoleDto {
        id: role_id,
        name: request.name,
        name_ar: request.name_ar,
        is_system,
        permissions: request.permissions.unwrap_or_default(),
    }))
}

#[utoipa::path(
    delete,
    path = "/api/v1/iam/roles/{id}",
    responses((status = 200, description = "Role deleted", body = bool)),
    params(("id" = String, Path, description = "Role identifier")),
    security(("bearer" = [])),
    tag = "IAM"
)]
pub async fn delete_role(
    ctx: AuthContext,
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ApiResponse<bool>>, ApiError> {
    use orbis_infra::models::{role, role_permission, user_role_assignment};
    use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

    // Prevent deletion of system roles
    if let Ok(Some(r)) = role::Entity::find()
        .filter(role::Column::OrgId.eq(ctx.tenant_id))
        .filter(role::Column::Id.eq(&id))
        .one(&state.db)
        .await
    {
        if r.is_system {
            return Err(ApiError::bad_request("Cannot delete system roles"));
        }
    }

    let _ = user_role_assignment::Entity::delete_many()
        .filter(user_role_assignment::Column::OrgId.eq(ctx.tenant_id))
        .filter(user_role_assignment::Column::RoleId.eq(&id))
        .exec(&state.db)
        .await;

    let _ = role_permission::Entity::delete_many()
        .filter(role_permission::Column::OrgId.eq(ctx.tenant_id))
        .filter(role_permission::Column::RoleId.eq(&id))
        .exec(&state.db)
        .await;

    let _ = role::Entity::delete_many()
        .filter(role::Column::OrgId.eq(ctx.tenant_id))
        .filter(role::Column::Id.eq(&id))
        .exec(&state.db)
        .await;

    Ok(ApiResponse::ok(true))
}

#[utoipa::path(
    get,
    path = "/api/v1/iam/permissions",
    responses((status = 200, description = "List of permissions rules", body = Vec<PermissionRuleDto>)),
    security(("bearer" = [])),
    tag = "IAM"
)]
pub async fn list_permissions(
    ctx: AuthContext,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<PermissionRuleDto>>>, ApiError> {
    use orbis_infra::models::role_permission;
    use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

    let permissions = role_permission::Entity::find()
        .filter(role_permission::Column::OrgId.eq(ctx.tenant_id))
        .all(&state.db)
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?;

    let mut grouped: std::collections::HashMap<(String, String), Vec<String>> =
        std::collections::HashMap::new();
    for p in permissions {
        grouped
            .entry((p.role_id, p.resource))
            .or_default()
            .push(p.action);
    }

    let result = grouped
        .into_iter()
        .map(|((role, document_type), actions)| PermissionRuleDto {
            id: None,
            role,
            document_type,
            level: Some(10),
            only_if_creator: Some(false),
            actions,
        })
        .collect();

    Ok(ApiResponse::ok(result))
}

#[utoipa::path(
    post,
    path = "/api/v1/iam/permissions",
    request_body = Vec<PermissionRuleDto>,
    responses((status = 200, description = "Permissions saved", body = bool)),
    security(("bearer" = [])),
    tag = "IAM"
)]
pub async fn save_permissions(
    ctx: AuthContext,
    State(state): State<AppState>,
    Json(request): Json<Vec<PermissionRuleDto>>,
) -> Result<Json<ApiResponse<bool>>, ApiError> {
    use orbis_infra::models::role_permission;
    use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};

    let now = chrono::Utc::now();
    for rule in request {
        let _ = role_permission::Entity::delete_many()
            .filter(role_permission::Column::OrgId.eq(ctx.tenant_id))
            .filter(role_permission::Column::RoleId.eq(&rule.role))
            .filter(role_permission::Column::Resource.eq(&rule.document_type))
            .exec(&state.db)
            .await;

        for action in rule.actions {
            let p_model = role_permission::ActiveModel {
                id: Set(Uuid::new_v4()),
                role_id: Set(rule.role.clone()),
                org_id: Set(ctx.tenant_id),
                resource: Set(rule.document_type.clone()),
                action: Set(action),
                condition: Set(None),
                created_at: Set(now),
            };
            let _ = p_model.insert(&state.db).await;
        }
    }

    Ok(ApiResponse::ok(true))
}

// ── MFA & Identity DTOs ─────────────────────────────────────────

#[derive(Deserialize, ToSchema)]
pub struct MfaEnrollRequest {
    pub factor_name: Option<String>,
}

#[derive(Serialize, ToSchema)]
pub struct MfaEnrollResponse {
    pub factor_id: Uuid,
    pub secret: String,
    pub qr_code_uri: String,
}

#[derive(Deserialize, ToSchema)]
pub struct MfaVerifyEnrollmentRequest {
    pub factor_id: Uuid,
    pub code: String,
}

#[derive(Deserialize, ToSchema)]
pub struct MfaChallengeRequest {
    pub mfa_token: String,
    pub code: String,
    pub device_id: Option<String>,
    pub remember_device: Option<bool>,
}

#[derive(Deserialize, ToSchema)]
pub struct GoogleAuthRequest {
    pub id_token: String,
}

#[derive(Serialize, ToSchema)]
pub struct UserDeviceDto {
    pub id: Uuid,
    pub device_id: String,
    pub device_name: String,
    pub device_type: String,
    pub os: Option<String>,
    pub browser: Option<String>,
    pub is_trusted: bool,
    pub last_active_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, ToSchema)]
pub struct UserIdentityDto {
    pub id: Uuid,
    pub provider: String,
    pub provider_user_id: String,
    pub email: Option<String>,
    pub connected_at: chrono::DateTime<chrono::Utc>,
}

// ── MFA & Identity Handlers ─────────────────────────────────────

#[utoipa::path(
    post,
    path = "/api/v1/auth/mfa/enroll",
    request_body = MfaEnrollRequest,
    responses((status = 200, description = "MFA Enrollment initiated", body = MfaEnrollResponse)),
    security(("bearer" = [])),
    tag = "Auth"
)]
async fn mfa_enroll(
    ctx: AuthContext,
    State(state): State<AppState>,
    Json(payload): Json<MfaEnrollRequest>,
) -> Result<Json<ApiResponse<MfaEnrollResponse>>, ApiError> {
    let res = state
        .auth_service
        .enroll_totp(&ctx, payload.factor_name)
        .await?;
    Ok(ApiResponse::ok(MfaEnrollResponse {
        factor_id: res.factor_id,
        secret: res.secret,
        qr_code_uri: res.qr_code_uri,
    }))
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/mfa/verify-enrollment",
    request_body = MfaVerifyEnrollmentRequest,
    responses((status = 200, description = "MFA Factor verified and activated", body = Vec<String>)),
    security(("bearer" = [])),
    tag = "Auth"
)]
async fn mfa_verify_enrollment(
    ctx: AuthContext,
    State(state): State<AppState>,
    Json(payload): Json<MfaVerifyEnrollmentRequest>,
) -> Result<Json<ApiResponse<Vec<String>>>, ApiError> {
    let recovery_codes = state
        .auth_service
        .verify_totp_enrollment(&ctx, payload.factor_id, &payload.code)
        .await?;
    Ok(ApiResponse::ok(recovery_codes))
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/mfa/challenge",
    request_body = MfaChallengeRequest,
    responses((status = 200, description = "MFA Challenge resolved and session issued", body = LoginResponse)),
    tag = "Auth"
)]
async fn mfa_challenge(
    State(state): State<AppState>,
    Json(payload): Json<MfaChallengeRequest>,
) -> Result<Json<ApiResponse<LoginResponse>>, ApiError> {
    let res = state
        .auth_service
        .verify_mfa_challenge(
            &payload.mfa_token,
            &payload.code,
            payload.device_id,
            payload.remember_device.unwrap_or(false),
        )
        .await?;

    let user = state
        .user_repo
        .find_by_id(res.user_id)
        .await?
        .ok_or_else(|| ApiError::not_found("User not found"))?;
    let person = state.person_repo.find_by_id(user.person_id).await?;
    let effective_context = state
        .auth_service
        .build_effective_context(&user, person.as_ref())
        .await?;
    let roles = effective_context.roles.clone();

    Ok(ApiResponse::ok(LoginResponse {
        token: Some(res.token),
        refresh_token: Some(res.refresh_token),
        user: user_to_dto(user, person.as_ref(), roles),
        effective_context: Some(effective_context.into()),
        mfa_required: false,
        mfa_token: None,
    }))
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/identities/google/link",
    request_body = GoogleAuthRequest,
    responses((status = 200, description = "Google identity linked successfully", body = UserIdentityDto)),
    security(("bearer" = [])),
    tag = "Auth"
)]
async fn google_link(
    ctx: AuthContext,
    State(state): State<AppState>,
    Json(payload): Json<GoogleAuthRequest>,
) -> Result<Json<ApiResponse<UserIdentityDto>>, ApiError> {
    let identity = state
        .auth_service
        .link_google(&ctx, &payload.id_token)
        .await?;
    Ok(ApiResponse::ok(UserIdentityDto {
        id: identity.id,
        provider: identity.provider.to_string(),
        provider_user_id: identity.provider_user_id,
        email: identity.email,
        connected_at: identity.connected_at,
    }))
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/identities/google/login",
    request_body = GoogleAuthRequest,
    responses((status = 200, description = "Logged in with Google", body = LoginResponse)),
    tag = "Auth"
)]
async fn google_login(
    State(state): State<AppState>,
    Json(payload): Json<GoogleAuthRequest>,
) -> Result<Json<ApiResponse<LoginResponse>>, ApiError> {
    let res = state.auth_service.login_google(&payload.id_token).await?;
    let user = state
        .user_repo
        .find_by_id(res.user_id)
        .await?
        .ok_or_else(|| ApiError::not_found("User not found"))?;
    let person = state.person_repo.find_by_id(user.person_id).await?;
    let effective_context = state
        .auth_service
        .build_effective_context(&user, person.as_ref())
        .await?;
    let roles = effective_context.roles.clone();

    Ok(ApiResponse::ok(LoginResponse {
        token: res.token,
        refresh_token: res.refresh_token,
        user: user_to_dto(user, person.as_ref(), roles),
        effective_context: Some(effective_context.into()),
        mfa_required: res.mfa_required,
        mfa_token: res.mfa_token,
    }))
}

#[utoipa::path(
    get,
    path = "/api/v1/auth/devices",
    responses((status = 200, description = "List of user devices", body = Vec<UserDeviceDto>)),
    security(("bearer" = [])),
    tag = "Auth"
)]
async fn list_devices(
    ctx: AuthContext,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<UserDeviceDto>>>, ApiError> {
    let devices = state.auth_service.list_devices(&ctx).await?;
    let dtos: Vec<UserDeviceDto> = devices
        .into_iter()
        .map(|d| UserDeviceDto {
            id: d.id,
            device_id: d.device_id,
            device_name: d.device_name,
            device_type: d.device_type.to_string(),
            os: d.os,
            browser: d.browser,
            is_trusted: d.is_trusted,
            last_active_at: d.last_active_at,
        })
        .collect();
    Ok(ApiResponse::ok(dtos))
}

#[utoipa::path(
    delete,
    path = "/api/v1/auth/devices/{id}",
    params(("id" = Uuid, Path, description = "Device ID to revoke")),
    responses((status = 200, description = "Device revoked successfully")),
    security(("bearer" = [])),
    tag = "Auth"
)]
async fn revoke_device(
    ctx: AuthContext,
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<Uuid>,
) -> Result<Json<ApiResponse<bool>>, ApiError> {
    state.auth_service.revoke_device(&ctx, id).await?;
    Ok(ApiResponse::ok(true))
}

// ── Router ──────────────────────────────────────────────────────

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/v1/auth/login", post(login))
        .route("/api/v1/auth/session", get(get_session))
        .route("/api/v1/auth/mfa/enroll", post(mfa_enroll))
        .route(
            "/api/v1/auth/mfa/verify-enrollment",
            post(mfa_verify_enrollment),
        )
        .route("/api/v1/auth/mfa/challenge", post(mfa_challenge))
        .route("/api/v1/auth/identities/google/link", post(google_link))
        .route("/api/v1/auth/identities/google/login", post(google_login))
        .route("/api/v1/auth/devices", get(list_devices))
        .route(
            "/api/v1/auth/devices/:id",
            axum::routing::delete(revoke_device),
        )
        .route("/api/v1/iam/users", post(register_user).get(list_users))
        .route("/api/v1/iam/users/provision", post(provision_user))
        .route("/api/v1/iam/users/:id", get(get_user).put(update_user))
        .route(
            "/api/v1/iam/users/:id/reset-password",
            post(reset_user_password),
        )
        .route("/api/v1/iam/roles", get(list_roles).post(save_role))
        .route("/api/v1/iam/roles/:id", axum::routing::delete(delete_role))
        .route(
            "/api/v1/iam/permissions",
            get(list_permissions).post(save_permissions),
        )
}
