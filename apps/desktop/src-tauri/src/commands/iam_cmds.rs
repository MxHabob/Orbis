//! Tauri IPC commands — Auth & IAM.

use tauri::State;
use serde::Serialize;
use crate::state::AppState;
use orbis_domain::entities::user::Role as DomainRole;
use uuid::Uuid;

use super::make_ctx;
use super::org_cmds::ApiResponse;
fn ok<T: Serialize>(data: T) -> ApiResponse<T> { ApiResponse { data } }

#[derive(Serialize)]
pub struct UserDto {
    pub id: String,
    pub org_id: String,
    pub person_id: String,
    pub email: Option<String>,
    pub full_name: String,
    pub full_name_ar: Option<String>,
    pub status: String,
    pub is_email_verified: bool,
    pub is_phone_verified: bool,
    pub is_mfa_enabled: bool,
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

fn user_to_dto(u: orbis_domain::entities::user::User, p: Option<&orbis_domain::entities::person::Person>) -> UserDto {
    let (full_name, full_name_ar) = match p {
        Some(person) => (person.full_name(), person.full_name_ar()),
        None => (String::new(), None),
    };
    UserDto {
        id: u.id.to_string(),
        org_id: u.org_id.to_string(),
        person_id: u.person_id.to_string(),
        email: u.email,
        full_name,
        full_name_ar,
        status: u.status,
        is_email_verified: u.is_email_verified,
        is_phone_verified: u.is_phone_verified,
        is_mfa_enabled: u.is_mfa_enabled,
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

    let user_dto = user_to_dto(res.user, res.person.as_ref());
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
            return Ok(ok(SessionResponse {
                org_id: session.org_id.to_string(),
                user: user_to_dto(user, person.as_ref()),
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
                        return Ok(ok(SessionResponse {
                            org_id: user.org_id.to_string(),
                            user: user_to_dto(user, person.as_ref()),
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
                        return Ok(ok(SessionResponse {
                            org_id: oid.to_string(),
                            user: user_to_dto(user, person.as_ref()),
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
                return Ok(ok(SessionResponse {
                    org_id: org.id.to_string(),
                    user: user_to_dto(u.clone(), person.as_ref()),
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
    let role_str = request["role"].as_str().unwrap_or("Student");
    let role = match role_str {
        "SuperAdmin" => DomainRole::SuperAdmin,
        "OrgAdmin" => DomainRole::OrgAdmin,
        "Teacher" => DomainRole::Teacher,
        "Student" => DomainRole::Student,
        "Parent" => DomainRole::Parent,
        r => DomainRole::Custom(r.to_string()),
    };
    let user = state.auth_service.register_user(&ctx, full_name, email, password, role, Some(ctx.tenant_id)).await.map_err(|e| e.to_string())?;
    let person = state.person_repo.find_by_id(user.person_id).await.unwrap_or(None);
    Ok(ok(user_to_dto(user, person.as_ref())))
}

#[tauri::command]
pub async fn iam_list_users(state: State<'_, AppState>) -> Result<ApiResponse<Vec<UserDto>>, String> {
    let ctx = make_ctx(&state);
    let users = state.auth_service.list_users(ctx.tenant_id).await.map_err(|e| e.to_string())?;
    let mut dtos = Vec::new();
    for u in users {
        let p = state.person_repo.find_by_id(u.person_id).await.unwrap_or(None);
        dtos.push(user_to_dto(u, p.as_ref()));
    }
    Ok(ok(dtos))
}

#[tauri::command]
pub async fn iam_get_user(id: String, state: State<'_, AppState>) -> Result<ApiResponse<UserDto>, String> {
    let uid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    let user = state.auth_service.get_user(uid).await.map_err(|e| e.to_string())?
        .ok_or_else(|| "User not found".to_string())?;
    let person = state.person_repo.find_by_id(user.person_id).await.unwrap_or(None);
    Ok(ok(user_to_dto(user, person.as_ref())))
}

#[tauri::command]
pub async fn iam_update_user(id: String, request: serde_json::Value, state: State<'_, AppState>) -> Result<ApiResponse<UserDto>, String> {
    let ctx = make_ctx(&state);
    let uid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    let full_name = request.get("fullName").or(request.get("full_name")).and_then(|v| v.as_str()).map(|s| s.to_string());
    let user = state.auth_service.update_user(&ctx, uid, full_name).await.map_err(|e| e.to_string())?;
    let person = state.person_repo.find_by_id(user.person_id).await.unwrap_or(None);
    Ok(ok(user_to_dto(user, person.as_ref())))
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

    Ok(ok(LoginResponse {
        token: res.token,
        refresh_token: res.refresh_token,
        user: user_to_dto(user, person.as_ref()),
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

    Ok(ok(LoginResponse {
        token: res.token,
        refresh_token: res.refresh_token,
        user: user_to_dto(user, person.as_ref()),
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
