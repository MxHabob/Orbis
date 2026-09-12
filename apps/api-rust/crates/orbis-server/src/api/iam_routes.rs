//! IAM routes — Authentication, users, roles.

use axum::{extract::State, routing::{get, post}, Json, Router};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use super::{ApiError, ApiResponse};
use super::extractors::AuthContext;
use crate::state::AppState;
use orbis_domain::entities::user::Role as DomainRole;
use orbis_app::services::auth_service::EffectiveContextDto;

// ── DTOs ────────────────────────────────────────────────────────

#[derive(Deserialize, ToSchema)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, ToSchema)]
pub struct LoginResponse {
    pub token: String,
    pub refresh_token: String,
    pub user: UserDto,
    pub effective_context: EffectiveContextDto,
}

#[derive(Deserialize, ToSchema)]
pub struct RegisterUserRequest {
    pub email: String,
    pub full_name: String,
    pub password: Option<String>,
    pub role: String,
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct UserDto {
    pub id: Uuid,
    pub org_id: Uuid,
    pub person_id: Uuid,
    pub email: Option<String>,
    pub full_name: String,
    pub full_name_ar: Option<String>,
    pub status: String,
    pub is_email_verified: bool,
    pub is_phone_verified: bool,
    pub is_mfa_enabled: bool,
}

pub fn user_to_dto(u: orbis_domain::entities::user::User, p: Option<&orbis_domain::entities::person::Person>) -> UserDto {
    let (full_name, full_name_ar) = match p {
        Some(person) => (person.full_name(), person.full_name_ar()),
        None => (String::new(), None),
    };
    UserDto {
        id: u.id,
        org_id: u.org_id,
        person_id: u.person_id,
        email: u.email,
        full_name,
        full_name_ar,
        status: u.status,
        is_email_verified: u.is_email_verified,
        is_phone_verified: u.is_phone_verified,
        is_mfa_enabled: u.is_mfa_enabled,
    }
}

#[derive(Serialize, ToSchema)]
pub struct SessionResponse {
    pub user: UserDto,
    pub org_id: Uuid,
    pub effective_context: EffectiveContextDto,
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
    };

    let res = state.auth_service.login(&ctx, &payload.email, &payload.password).await?;
    let user_dto = user_to_dto(res.user, res.person.as_ref());

    Ok(ApiResponse::ok(LoginResponse {
        token: res.token,
        refresh_token: res.refresh_token,
        user: user_dto,
        effective_context: res.effective_context,
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
    let user_id = ctx.actor.id()
        .ok_or_else(|| ApiError::unauthorized("No user identity in token"))?;

    let user = state.auth_service.get_user(user_id).await?
        .ok_or_else(|| ApiError::not_found("User not found"))?;

    let person = state.person_repo.find_by_id(user.person_id).await?;
    let effective_context = state.auth_service.build_effective_context(&user, person.as_ref()).await?;
    let user_dto = user_to_dto(user, person.as_ref());

    Ok(ApiResponse::ok(SessionResponse {
        org_id: ctx.tenant_id,
        user: user_dto,
        effective_context,
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
    let role = match payload.role.as_str() {
        "SuperAdmin" => DomainRole::SuperAdmin,
        "OrgAdmin" => DomainRole::OrgAdmin,
        "Teacher" => DomainRole::Teacher,
        "Student" => DomainRole::Student,
        "Parent" => DomainRole::Parent,
        _ => DomainRole::Custom(payload.role.clone()),
    };

    let user = state.auth_service.register_user(&ctx, payload.full_name, Some(payload.email), payload.password, role, Some(ctx.tenant_id)).await?;
    let person = state.person_repo.find_by_id(user.person_id).await?;

    Ok(ApiResponse::ok(user_to_dto(user, person.as_ref())))
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
        let p = state.person_repo.find_by_id(u.person_id).await.unwrap_or(None);
        dtos.push(user_to_dto(u, p.as_ref()));
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
    let user = state.user_repo.find_by_id(id).await?
        .ok_or_else(|| ApiError::not_found(format!("User {} not found", id)))?;

    // Tenant isolation
    if user.org_id != ctx.tenant_id && !ctx.actor.is_system() {
        return Err(ApiError::forbidden("Access denied to this user"));
    }

    let person = state.person_repo.find_by_id(user.person_id).await?;
    Ok(ApiResponse::ok(user_to_dto(user, person.as_ref())))
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateUserRequest {
    pub full_name: Option<String>,
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
    let mut user = state.user_repo.find_by_id(id).await?
        .ok_or_else(|| ApiError::not_found(format!("User {} not found", id)))?;

    if user.org_id != ctx.tenant_id && !ctx.actor.is_system() {
        return Err(ApiError::forbidden("Access denied to this user"));
    }

    if let Some(name) = p.full_name {
        if let Ok(Some(mut person)) = state.person_repo.find_by_id(user.person_id).await {
            let parts: Vec<&str> = name.split_whitespace().collect();
            person.first_name = parts.first().unwrap_or(&"").to_string();
            if parts.len() > 1 {
                person.last_name = parts[1..].join(" ");
            }
            person.updated_at = chrono::Utc::now();
            let _ = state.person_repo.save(&person).await;
        }
    }
    user.updated_at = chrono::Utc::now();
    state.user_repo.save(&user).await?;

    let person = state.person_repo.find_by_id(user.person_id).await?;
    Ok(ApiResponse::ok(user_to_dto(user, person.as_ref())))
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
    pub recovery_codes: Vec<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct MfaVerifyEnrollmentRequest {
    pub factor_id: Uuid,
    pub code: String,
}

#[derive(Deserialize, ToSchema)]
pub struct MfaChallengeRequest {
    pub user_id: Uuid,
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
    let res = state.auth_service.enroll_totp(&ctx, payload.factor_name).await?;
    Ok(ApiResponse::ok(MfaEnrollResponse {
        factor_id: res.factor_id,
        secret: res.secret,
        qr_code_uri: res.qr_code_uri,
        recovery_codes: res.recovery_codes,
    }))
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/mfa/verify-enrollment",
    request_body = MfaVerifyEnrollmentRequest,
    responses((status = 200, description = "MFA Factor verified and activated")),
    security(("bearer" = [])),
    tag = "Auth"
)]
async fn mfa_verify_enrollment(
    ctx: AuthContext,
    State(state): State<AppState>,
    Json(payload): Json<MfaVerifyEnrollmentRequest>,
) -> Result<Json<ApiResponse<bool>>, ApiError> {
    let verified = state.auth_service.verify_totp_enrollment(&ctx, payload.factor_id, &payload.code).await?;
    Ok(ApiResponse::ok(verified))
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
    let res = state.auth_service.verify_mfa_challenge(
        payload.user_id,
        &payload.code,
        payload.device_id,
        payload.remember_device.unwrap_or(false),
    ).await?;

    let user = state.user_repo.find_by_id(payload.user_id).await?
        .ok_or_else(|| ApiError::not_found("User not found"))?;
    let person = state.person_repo.find_by_id(user.person_id).await?;
    let effective_context = state.auth_service.build_effective_context(&user, person.as_ref()).await?;

    Ok(ApiResponse::ok(LoginResponse {
        token: res.token,
        refresh_token: res.refresh_token,
        user: user_to_dto(user, person.as_ref()),
        effective_context,
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
    let identity = state.auth_service.link_google(&ctx, &payload.id_token).await?;
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
    let user = state.user_repo.find_by_id(res.user_id).await?
        .ok_or_else(|| ApiError::not_found("User not found"))?;
    let person = state.person_repo.find_by_id(user.person_id).await?;
    let effective_context = state.auth_service.build_effective_context(&user, person.as_ref()).await?;

    Ok(ApiResponse::ok(LoginResponse {
        token: res.token,
        refresh_token: res.refresh_token,
        user: user_to_dto(user, person.as_ref()),
        effective_context,
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
    let dtos: Vec<UserDeviceDto> = devices.into_iter().map(|d| UserDeviceDto {
        id: d.id,
        device_id: d.device_id,
        device_name: d.device_name,
        device_type: d.device_type.to_string(),
        os: d.os,
        browser: d.browser,
        is_trusted: d.is_trusted,
        last_active_at: d.last_active_at,
    }).collect();
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
        .route("/api/v1/auth/mfa/verify-enrollment", post(mfa_verify_enrollment))
        .route("/api/v1/auth/mfa/challenge", post(mfa_challenge))
        .route("/api/v1/auth/identities/google/link", post(google_link))
        .route("/api/v1/auth/identities/google/login", post(google_login))
        .route("/api/v1/auth/devices", get(list_devices))
        .route("/api/v1/auth/devices/:id", axum::routing::delete(revoke_device))
        .route("/api/v1/iam/users", post(register_user).get(list_users))
        .route("/api/v1/iam/users/:id", get(get_user).put(update_user))
}
