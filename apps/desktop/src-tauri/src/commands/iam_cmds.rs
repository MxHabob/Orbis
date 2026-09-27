use tauri::State;
use serde::{Deserialize, Serialize};
use crate::state::AppState;
use uuid::Uuid;

use super::make_ctx;
use super::org_cmds::ApiResponse;
fn ok<T: Serialize>(data: T) -> ApiResponse<T> { ApiResponse { data } }

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserDto {
    pub id: String,
    pub org_id: String,
    pub person_id: String,
    pub email: Option<String>,
    pub full_name: String,
    pub full_name_ar: Option<String>,
    pub avatar_url: Option<String>,
    pub status: String,
    pub is_email_verified: bool,
    pub is_phone_verified: bool,
    pub is_mfa_enabled: bool,
    pub roles: Vec<String>,
    pub phone: Option<String>,
    pub national_id: Option<String>,
    pub gender: Option<String>,
    pub birth_date: Option<String>,
    pub address: Option<String>,
    pub is_active: bool,
}

#[derive(Serialize)]
pub struct LinkedDependentDto {
    pub student_id: String,
    pub full_name: String,
    pub full_name_ar: Option<String>,
    pub relationship: String,
    pub is_primary_contact: bool,
    pub is_emergency_contact: bool,
    pub is_fee_payer: bool,
    pub current_grade_level: Option<String>,
    pub current_section_name: Option<String>,
}

#[derive(Serialize)]
pub struct EffectiveContextDto {
    pub effective_role: String,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
    pub active_period_id: Option<String>,
    pub dependents: Vec<LinkedDependentDto>,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub refresh_token: String,
    pub user: UserDto,
    pub effective_context: EffectiveContextDto,
}

#[derive(Serialize)]
pub struct SessionResponse {
    pub user: UserDto,
    pub org_id: String,
    pub effective_context: EffectiveContextDto,
}

pub async fn get_roles_for_user(db: &sea_orm::DatabaseConnection, user_id: Uuid) -> Vec<String> {
    use orbis_infra::models::user_role_assignment;
    use sea_orm::{EntityTrait, QueryFilter, ColumnTrait};

    user_role_assignment::Entity::find()
        .filter(user_role_assignment::Column::UserId.eq(user_id))
        .all(db)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|r| r.role_id)
        .collect()
}

fn user_to_dto(
    u: orbis_domain::entities::user::User,
    p: Option<&orbis_domain::entities::person::Person>,
    roles: Vec<String>,
) -> UserDto {
    let (full_name, full_name_ar, avatar_url, phone, national_id, gender, birth_date, address) = match p {
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
        id: u.id.to_string(),
        org_id: u.org_id.to_string(),
        person_id: u.person_id.to_string(),
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
    }
}

fn map_effective_context(ctx: orbis_app::services::auth_service::EffectiveContextDto) -> EffectiveContextDto {
    EffectiveContextDto {
        effective_role: ctx.effective_role,
        roles: ctx.roles,
        permissions: ctx.permissions,
        active_period_id: ctx.active_period_id.map(|id| id.to_string()),
        dependents: ctx.dependents.into_iter().map(|d| LinkedDependentDto {
            student_id: d.student_id.to_string(),
            full_name: d.full_name,
            full_name_ar: d.full_name_ar,
            relationship: d.relationship,
            is_primary_contact: d.is_primary_contact,
            is_emergency_contact: d.is_emergency_contact,
            is_fee_payer: d.is_fee_payer,
            current_grade_level: d.current_grade_level,
            current_section_name: d.current_section_name,
        }).collect(),
    }
}

#[tauri::command]
pub async fn auth_login(request: serde_json::Value, state: State<'_, AppState>) -> Result<ApiResponse<LoginResponse>, String> {
    let ctx = make_ctx(&state);
    let email = request["email"].as_str().unwrap_or_default();
    let password = request["password"].as_str().unwrap_or_default();
    let res = state.auth_service.login(&ctx, email, password).await.map_err(|e| e.to_string())?;

    state.set_active_session(crate::state::ActiveSession {
        user_id: res.user.id,
        org_id: res.user.org_id,
        active_unit_id: None,
        token: res.token.clone(),
    });

    let roles = get_roles_for_user(&state.db, res.user.id).await;
    let user_dto = user_to_dto(res.user, res.person.as_ref(), roles);
    let effective_context = map_effective_context(res.effective_context);

    Ok(ok(LoginResponse {
        token: res.token,
        refresh_token: res.refresh_token,
        user: user_dto,
        effective_context,
    }))
}

#[tauri::command]
pub async fn auth_get_session(request: Option<serde_json::Value>, state: State<'_, AppState>) -> Result<ApiResponse<SessionResponse>, String> {
    // 1. Check in-memory active session
    if let Some(session) = state.get_active_session() {
        if let Ok(Some(user)) = state.auth_service.get_user(session.user_id).await {
            let person = state.person_repo.find_by_id(user.person_id).await.unwrap_or(None);
            let eff = state.auth_service.build_effective_context(&user, person.as_ref()).await.map_err(|e| e.to_string())?;
            let roles = get_roles_for_user(&state.db, user.id).await;
            return Ok(ok(SessionResponse {
                org_id: session.org_id.to_string(),
                user: user_to_dto(user, person.as_ref(), roles),
                effective_context: map_effective_context(eff),
            }));
        }
    }

    // 2. Check token from request
    let token_opt = request.as_ref()
        .and_then(|r| r.get("token"))
        .and_then(|v| v.as_str());

    if let Some(token_str) = token_opt {
        if !token_str.is_empty() {
            if let Ok(Some(db_session)) = state.session_repo.find_by_token(token_str).await {
                if !db_session.is_revoked {
                    if let Ok(Some(user)) = state.auth_service.get_user(db_session.user_id).await {
                        state.set_active_session(crate::state::ActiveSession {
                            user_id: user.id,
                            org_id: user.org_id,
                            active_unit_id: None,
                            token: token_str.to_string(),
                        });
                        let person = state.person_repo.find_by_id(user.person_id).await.unwrap_or(None);
                        let eff = state.auth_service.build_effective_context(&user, person.as_ref()).await.map_err(|e| e.to_string())?;
                        let roles = get_roles_for_user(&state.db, user.id).await;
                        return Ok(ok(SessionResponse {
                            org_id: user.org_id.to_string(),
                            user: user_to_dto(user, person.as_ref(), roles),
                            effective_context: map_effective_context(eff),
                        }));
                    }
                }
            }
            if let Ok(claims) = state.token_service.validate_token(token_str).await {
                if let (Ok(uid), Ok(oid)) = (Uuid::parse_str(&claims.sub), Uuid::parse_str(&claims.org_id)) {
                    if let Ok(Some(user)) = state.auth_service.get_user(uid).await {
                        state.set_active_session(crate::state::ActiveSession {
                            user_id: uid,
                            org_id: oid,
                            active_unit_id: None,
                            token: token_str.to_string(),
                        });
                        let person = state.person_repo.find_by_id(user.person_id).await.unwrap_or(None);
                        let eff = state.auth_service.build_effective_context(&user, person.as_ref()).await.map_err(|e| e.to_string())?;
                        let roles = get_roles_for_user(&state.db, user.id).await;
                        return Ok(ok(SessionResponse {
                            org_id: oid.to_string(),
                            user: user_to_dto(user, person.as_ref(), roles),
                            effective_context: map_effective_context(eff),
                        }));
                    }
                }
            }
        }
    }

    // 3. Desktop local mode single-tenant fallback
    if let Ok(Some(org)) = state.org_repo.find_first().await {
        if let Ok(users) = state.auth_service.list_users(org.id).await {
            if users.len() == 1 {
                let u = &users[0];
                state.set_active_session(crate::state::ActiveSession {
                    user_id: u.id,
                    org_id: org.id,
                    active_unit_id: None,
                    token: "desktop-local-token".to_string(),
                });
                let person = state.person_repo.find_by_id(u.person_id).await.unwrap_or(None);
                let eff = state.auth_service.build_effective_context(u, person.as_ref()).await.map_err(|e| e.to_string())?;
                let roles = get_roles_for_user(&state.db, u.id).await;
                return Ok(ok(SessionResponse {
                    org_id: org.id.to_string(),
                    user: user_to_dto(u.clone(), person.as_ref(), roles),
                    effective_context: map_effective_context(eff),
                }));
            }
        }
    }

    Err("No user session".to_string())
}

#[tauri::command]
pub async fn auth_logout(state: State<'_, AppState>) -> Result<(), String> {
    if let Some(session) = state.get_active_session() {
        let _ = state.auth_service.logout(&session.token).await;
    }
    state.clear_active_session();
    Ok(())
}

#[tauri::command]
pub async fn iam_register_user(request: serde_json::Value, state: State<'_, AppState>) -> Result<ApiResponse<UserDto>, String> {
    let ctx = make_ctx(&state);
    let full_name = request["fullName"].as_str().or(request["full_name"].as_str()).unwrap_or_default().to_string();
    let email = request["email"].as_str().map(|s| s.to_string());
    let password = request["password"].as_str().map(|s| s.to_string());
    let role = request["role"].as_str().map(|s| s.to_string());

    let user = state.auth_service.register_user(&ctx, full_name, email, password, role.clone(), Some(ctx.tenant_id)).await.map_err(|e| e.to_string())?;
    
    // Assign role in user_role_assignments if given
    if let Some(r) = role {
        use orbis_infra::models::user_role_assignment;
        use sea_orm::{ActiveModelTrait, Set};
        let model = user_role_assignment::ActiveModel {
            id: Set(Uuid::new_v4()),
            user_id: Set(user.id),
            role_id: Set(r),
            org_id: Set(ctx.tenant_id),
            scope_type: Set("global".to_string()),
            scope_id: Set(None),
            assigned_by: Set(None),
            created_at: Set(chrono::Utc::now()),
        };
        let _ = model.insert(&state.db).await;
    }

    let person = state.person_repo.find_by_id(user.person_id).await.unwrap_or(None);
    let roles = get_roles_for_user(&state.db, user.id).await;
    Ok(ok(user_to_dto(user, person.as_ref(), roles)))
}

#[tauri::command]
pub async fn iam_list_users(state: State<'_, AppState>) -> Result<ApiResponse<Vec<UserDto>>, String> {
    let ctx = make_ctx(&state);
    let users = state.auth_service.list_users(ctx.tenant_id).await.map_err(|e| e.to_string())?;
    let mut dtos = Vec::new();
    for u in users {
        let p = state.person_repo.find_by_id(u.person_id).await.unwrap_or(None);
        let roles = get_roles_for_user(&state.db, u.id).await;
        dtos.push(user_to_dto(u, p.as_ref(), roles));
    }
    Ok(ok(dtos))
}

#[tauri::command]
pub async fn iam_get_user(id: String, state: State<'_, AppState>) -> Result<ApiResponse<UserDto>, String> {
    let uid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    let user = state.auth_service.get_user(uid).await.map_err(|e| e.to_string())?
        .ok_or_else(|| "User not found".to_string())?;
    let person = state.person_repo.find_by_id(user.person_id).await.unwrap_or(None);
    let roles = get_roles_for_user(&state.db, user.id).await;
    Ok(ok(user_to_dto(user, person.as_ref(), roles)))
}

#[tauri::command]
pub async fn iam_update_user(id: String, request: serde_json::Value, state: State<'_, AppState>) -> Result<ApiResponse<UserDto>, String> {
    let ctx = make_ctx(&state);
    let uid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    
    // 1. Get user
    let mut user = state.auth_service.get_user(uid).await.map_err(|e| e.to_string())?
        .ok_or_else(|| "User not found".to_string())?;

    // 2. Update user name
    let full_name = request.get("fullName").or(request.get("full_name")).and_then(|v| v.as_str()).map(|s| s.to_string());
    if let Some(name) = full_name.as_ref() {
        let _ = state.auth_service.update_user(&ctx, uid, Some(name.clone())).await;
    }

    if let Some(is_active) = request.get("is_active").and_then(|v| v.as_bool()) {
        user.status = if is_active { "active".to_string() } else { "inactive".to_string() };
        let _ = state.user_repo.save(&user).await;
    } else if let Some(status) = request.get("status").and_then(|v| v.as_str()) {
        user.status = status.to_string();
        let _ = state.user_repo.save(&user).await;
    }

    // 3. Update Person details
    if let Ok(Some(mut person)) = state.person_repo.find_by_id(user.person_id).await {
        let mut changed = false;
        if let Some(phone) = request.get("phone").and_then(|v| v.as_str()) {
            person.phone = if phone.is_empty() { None } else { Some(phone.to_string()) };
            changed = true;
        }
        if let Some(name_ar) = request.get("name_ar").and_then(|v| v.as_str()) {
            person.first_name_ar = Some(name_ar.to_string());
            changed = true;
        }
        if let Some(name_en) = request.get("name_en").and_then(|v| v.as_str()) {
            person.first_name = name_en.to_string();
            changed = true;
        }
        if let Some(nid) = request.get("national_id").and_then(|v| v.as_str()) {
            person.national_id = if nid.is_empty() { None } else { Some(nid.to_string()) };
            changed = true;
        }
        if let Some(gender) = request.get("gender").and_then(|v| v.as_str()) {
            person.gender = if gender.is_empty() { None } else { Some(gender.to_string()) };
            changed = true;
        }
        if let Some(addr) = request.get("address").and_then(|v| v.as_str()) {
            person.address = if addr.is_empty() { None } else { Some(addr.to_string()) };
            changed = true;
        }
        if changed {
            let _ = state.person_repo.save(&person).await;
        }
    }

    // 4. Update Role assignments
    use orbis_infra::models::user_role_assignment;
    use sea_orm::{ActiveModelTrait, EntityTrait, QueryFilter, ColumnTrait, Set};

    let mut new_roles = Vec::new();
    if let Some(roles_arr) = request.get("roles").and_then(|v| v.as_array()) {
        for r in roles_arr {
            if let Some(s) = r.as_str() {
                if !s.is_empty() && !new_roles.contains(&s.to_string()) {
                    new_roles.push(s.to_string());
                }
            }
        }
    } else if let Some(r) = request.get("primary_role").or(request.get("role")).and_then(|v| v.as_str()) {
        if !r.is_empty() {
            new_roles.push(r.to_string());
        }
    }

    if !new_roles.is_empty() {
        let _ = user_role_assignment::Entity::delete_many()
            .filter(user_role_assignment::Column::UserId.eq(uid))
            .exec(&state.db)
            .await;

        let now = chrono::Utc::now();
        for role_id in &new_roles {
            let model = user_role_assignment::ActiveModel {
                id: Set(Uuid::new_v4()),
                user_id: Set(uid),
                role_id: Set(role_id.clone()),
                org_id: Set(ctx.tenant_id),
                scope_type: Set("global".to_string()),
                scope_id: Set(None),
                assigned_by: Set(None),
                created_at: Set(now),
            };
            let _ = model.insert(&state.db).await;
        }
    }

    let person = state.person_repo.find_by_id(user.person_id).await.unwrap_or(None);
    let roles = get_roles_for_user(&state.db, user.id).await;
    Ok(ok(user_to_dto(user, person.as_ref(), roles)))
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserProvisioningResultDto {
    pub user_id: String,
    pub username: String,
    pub temporary_password: Option<String>,
    pub delivery_channel: String,
    pub delivery_status: String,
}

#[tauri::command]
pub async fn iam_reset_user_password(
    user_id: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<UserProvisioningResultDto>, String> {
    let ctx = make_ctx(&state);
    let uid = Uuid::parse_str(&user_id).map_err(|e| e.to_string())?;

    let res = state.identity_provisioning_service.reset_password(
        &ctx,
        uid,
        Some(orbis_app::services::identity_provisioning_service::DeliveryChannel::Manual),
    ).await.map_err(|e| e.to_string())?;

    Ok(ok(UserProvisioningResultDto {
        user_id: res.user_id.to_string(),
        username: res.username,
        temporary_password: res.temporary_password,
        delivery_channel: res.delivery_channel.to_string(),
        delivery_status: res.delivery_status.to_string(),
    }))
}

#[derive(Serialize)]
pub struct MfaEnrollResponse {
    pub factor_id: String,
    pub secret: String,
    pub qr_code_uri: String,
    pub recovery_codes: Vec<String>,
}

#[derive(Serialize)]
pub struct UserDeviceDto {
    pub id: String,
    pub device_id: String,
    pub device_name: String,
    pub device_type: String,
    pub os: Option<String>,
    pub browser: Option<String>,
    pub is_trusted: bool,
    pub last_active_at: String,
}

#[derive(Serialize)]
pub struct UserIdentityDto {
    pub id: String,
    pub provider: String,
    pub provider_user_id: String,
    pub email: Option<String>,
    pub connected_at: String,
}

#[tauri::command]
pub async fn auth_mfa_enroll(request: serde_json::Value, state: State<'_, AppState>) -> Result<ApiResponse<MfaEnrollResponse>, String> {
    let ctx = make_ctx(&state);
    let factor_name = request["factor_name"].as_str().map(|s| s.to_string());
    let res = state.auth_service.enroll_totp(&ctx, factor_name).await.map_err(|e| e.to_string())?;
    Ok(ok(MfaEnrollResponse {
        factor_id: res.factor_id.to_string(),
        secret: res.secret,
        qr_code_uri: res.qr_code_uri,
        recovery_codes: res.recovery_codes,
    }))
}

#[tauri::command]
pub async fn auth_mfa_verify_enrollment(request: serde_json::Value, state: State<'_, AppState>) -> Result<ApiResponse<bool>, String> {
    let ctx = make_ctx(&state);
    let factor_id_str = request["factor_id"].as_str().ok_or("factor_id is required")?;
    let factor_id = Uuid::parse_str(factor_id_str).map_err(|_| "Invalid factor_id")?;
    let code = request["code"].as_str().ok_or("code is required")?;
    let verified = state.auth_service.verify_totp_enrollment(&ctx, factor_id, code).await.map_err(|e| e.to_string())?;
    Ok(ok(verified))
}

#[tauri::command]
pub async fn auth_mfa_challenge(request: serde_json::Value, state: State<'_, AppState>) -> Result<ApiResponse<LoginResponse>, String> {
    let user_id_str = request["user_id"].as_str().ok_or("user_id is required")?;
    let user_id = Uuid::parse_str(user_id_str).map_err(|_| "Invalid user_id")?;
    let code = request["code"].as_str().ok_or("code is required")?;
    let device_id = request["device_id"].as_str().map(|s| s.to_string());
    let remember_device = request["remember_device"].as_bool().unwrap_or(false);

    let res = state.auth_service.verify_mfa_challenge(user_id, code, device_id, remember_device).await.map_err(|e| e.to_string())?;
    let user = state.auth_service.get_user(user_id).await.map_err(|e| e.to_string())?
        .ok_or_else(|| "User not found".to_string())?;

    state.set_active_session(crate::state::ActiveSession {
        user_id: user.id,
        org_id: user.org_id,
        active_unit_id: None,
        token: res.token.clone(),
    });

    let person = state.person_repo.find_by_id(user.person_id).await.unwrap_or(None);
    let eff = state.auth_service.build_effective_context(&user, person.as_ref()).await.map_err(|e| e.to_string())?;
    let roles = get_roles_for_user(&state.db, user.id).await;

    Ok(ok(LoginResponse {
        token: res.token,
        refresh_token: res.refresh_token,
        user: user_to_dto(user, person.as_ref(), roles),
        effective_context: map_effective_context(eff),
    }))
}

#[tauri::command]
pub async fn auth_link_google(request: serde_json::Value, state: State<'_, AppState>) -> Result<ApiResponse<UserIdentityDto>, String> {
    let ctx = make_ctx(&state);
    let id_token = request["id_token"].as_str().ok_or("id_token is required")?;
    let identity = state.auth_service.link_google(&ctx, id_token).await.map_err(|e| e.to_string())?;
    Ok(ok(UserIdentityDto {
        id: identity.id.to_string(),
        provider: identity.provider.to_string(),
        provider_user_id: identity.provider_user_id,
        email: identity.email,
        connected_at: identity.connected_at.to_rfc3339(),
    }))
}

#[tauri::command]
pub async fn auth_login_google(request: serde_json::Value, state: State<'_, AppState>) -> Result<ApiResponse<LoginResponse>, String> {
    let id_token = request["id_token"].as_str().ok_or("id_token is required")?;
    let res = state.auth_service.login_google(id_token).await.map_err(|e| e.to_string())?;
    let user = state.auth_service.get_user(res.user_id).await.map_err(|e| e.to_string())?
        .ok_or_else(|| "User not found".to_string())?;

    state.set_active_session(crate::state::ActiveSession {
        user_id: user.id,
        org_id: user.org_id,
        active_unit_id: None,
        token: res.token.clone(),
    });

    let person = state.person_repo.find_by_id(user.person_id).await.unwrap_or(None);
    let eff = state.auth_service.build_effective_context(&user, person.as_ref()).await.map_err(|e| e.to_string())?;
    let roles = get_roles_for_user(&state.db, user.id).await;

    Ok(ok(LoginResponse {
        token: res.token,
        refresh_token: res.refresh_token,
        user: user_to_dto(user, person.as_ref(), roles),
        effective_context: map_effective_context(eff),
    }))
}

#[tauri::command]
pub async fn auth_list_devices(state: State<'_, AppState>) -> Result<ApiResponse<Vec<UserDeviceDto>>, String> {
    let ctx = make_ctx(&state);
    let devices = state.auth_service.list_devices(&ctx).await.map_err(|e| e.to_string())?;
    let dtos = devices.into_iter().map(|d| UserDeviceDto {
        id: d.id.to_string(),
        device_id: d.device_id,
        device_name: d.device_name,
        device_type: d.device_type.to_string(),
        os: d.os,
        browser: d.browser,
        is_trusted: d.is_trusted,
        last_active_at: d.last_active_at.to_rfc3339(),
    }).collect();
    Ok(ok(dtos))
}

#[tauri::command]
pub async fn auth_revoke_device(id: String, state: State<'_, AppState>) -> Result<ApiResponse<bool>, String> {
    let ctx = make_ctx(&state);
    let device_uuid = Uuid::parse_str(&id).map_err(|_| "Invalid device ID")?;
    state.auth_service.revoke_device(&ctx, device_uuid).await.map_err(|e| e.to_string())?;
    Ok(ok(true))
}

#[derive(Serialize, Deserialize)]
pub struct RoleDto {
    pub id: String,
    pub name: String,
    pub name_ar: Option<String>,
    pub is_system: bool,
    pub permissions: std::collections::HashMap<String, Vec<String>>,
}

#[derive(Serialize, Deserialize)]
pub struct SaveRoleRequest {
    pub id: String,
    pub name: String,
    pub name_ar: Option<String>,
    pub permissions: Option<std::collections::HashMap<String, Vec<String>>>,
}

#[derive(Serialize, Deserialize)]
pub struct PermissionRuleDto {
    pub id: Option<String>,
    pub role: String,
    pub document_type: String,
    pub level: Option<i32>,
    pub only_if_creator: Option<bool>,
    pub actions: Vec<String>,
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

#[tauri::command]
pub async fn iam_list_roles(state: State<'_, AppState>) -> Result<ApiResponse<Vec<RoleDto>>, String> {
    let ctx = make_ctx(&state);
    use orbis_infra::models::{role, role_permission};
    use sea_orm::{EntityTrait, QueryFilter, ColumnTrait};

    let permissions = role_permission::Entity::find()
        .filter(role_permission::Column::OrgId.eq(ctx.tenant_id))
        .all(&state.db)
        .await
        .unwrap_or_default();

    let mut perm_map: std::collections::HashMap<String, std::collections::HashMap<String, Vec<String>>> = std::collections::HashMap::new();
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

    Ok(ok(result_roles))
}

#[tauri::command]
pub async fn iam_save_role(request: SaveRoleRequest, state: State<'_, AppState>) -> Result<ApiResponse<RoleDto>, String> {
    let ctx = make_ctx(&state);
    use orbis_infra::models::{role, role_permission};
    use sea_orm::{ActiveModelTrait, EntityTrait, QueryFilter, ColumnTrait, Set};

    let role_id = request.id.clone();
    let now = chrono::Utc::now();

    let existing = role::Entity::find()
        .filter(role::Column::OrgId.eq(ctx.tenant_id))
        .filter(role::Column::Id.eq(&role_id))
        .one(&state.db)
        .await
        .unwrap_or(None);

    let is_system = existing.as_ref().map(|e| e.is_system).unwrap_or(false);

    if let Some(existing_model) = existing {
        let mut active: role::ActiveModel = existing_model.into();
        active.name = Set(request.name.clone());
        active.name_ar = Set(request.name_ar.clone());
        active.updated_at = Set(now);
        let _ = active.update(&state.db).await;
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
        let _ = active_role.insert(&state.db).await;
    }

    // If permissions provided, save them
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

    Ok(ok(RoleDto {
        id: role_id,
        name: request.name,
        name_ar: request.name_ar,
        is_system,
        permissions: request.permissions.unwrap_or_default(),
    }))
}

#[tauri::command]
pub async fn iam_delete_role(role_id: String, state: State<'_, AppState>) -> Result<ApiResponse<bool>, String> {
    let ctx = make_ctx(&state);
    use orbis_infra::models::{role, role_permission, user_role_assignment};
    use sea_orm::{EntityTrait, QueryFilter, ColumnTrait};

    // Prevent deletion of system roles
    if let Ok(Some(r)) = role::Entity::find()
        .filter(role::Column::OrgId.eq(ctx.tenant_id))
        .filter(role::Column::Id.eq(&role_id))
        .one(&state.db)
        .await
    {
        if r.is_system {
            return Err("Cannot delete system roles".to_string());
        }
    }

    let _ = user_role_assignment::Entity::delete_many()
        .filter(user_role_assignment::Column::OrgId.eq(ctx.tenant_id))
        .filter(user_role_assignment::Column::RoleId.eq(&role_id))
        .exec(&state.db)
        .await;

    let _ = role_permission::Entity::delete_many()
        .filter(role_permission::Column::OrgId.eq(ctx.tenant_id))
        .filter(role_permission::Column::RoleId.eq(&role_id))
        .exec(&state.db)
        .await;

    let _ = role::Entity::delete_many()
        .filter(role::Column::OrgId.eq(ctx.tenant_id))
        .filter(role::Column::Id.eq(&role_id))
        .exec(&state.db)
        .await;

    Ok(ok(true))
}

#[tauri::command]
pub async fn iam_list_permissions(state: State<'_, AppState>) -> Result<ApiResponse<Vec<PermissionRuleDto>>, String> {
    let ctx = make_ctx(&state);
    use orbis_infra::models::role_permission;
    use sea_orm::{EntityTrait, QueryFilter, ColumnTrait};

    let permissions = role_permission::Entity::find()
        .filter(role_permission::Column::OrgId.eq(ctx.tenant_id))
        .all(&state.db)
        .await
        .map_err(|e| e.to_string())?;

    let mut grouped: std::collections::HashMap<(String, String), Vec<String>> = std::collections::HashMap::new();
    for p in permissions {
        grouped.entry((p.role_id, p.resource)).or_default().push(p.action);
    }

    let result = grouped.into_iter().map(|((role, document_type), actions)| PermissionRuleDto {
        id: None,
        role,
        document_type,
        level: Some(10),
        only_if_creator: Some(false),
        actions,
    }).collect();

    Ok(ok(result))
}

#[tauri::command]
pub async fn iam_save_permissions(request: Vec<PermissionRuleDto>, state: State<'_, AppState>) -> Result<ApiResponse<bool>, String> {
    let ctx = make_ctx(&state);
    use orbis_infra::models::role_permission;
    use sea_orm::{ActiveModelTrait, EntityTrait, QueryFilter, ColumnTrait, Set};

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

    Ok(ok(true))
}

