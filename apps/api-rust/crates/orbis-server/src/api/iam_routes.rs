//! IAM routes — Authentication, users, roles.

use axum::{extract::State, routing::{get, post}, Json, Router};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use super::{ApiError, ApiResponse};
use super::extractors::AuthContext;
use crate::state::AppState;
use orbis_app::usecases::iam::RegisterUserUseCase;
use orbis_app::usecases::login::LoginUseCase;
use orbis_domain::entities::user::Role as DomainRole;

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
    pub email: Option<String>,
    pub full_name: String,
    pub status: String,
}

#[derive(Serialize, ToSchema)]
pub struct SessionResponse {
    pub user: UserDto,
    pub org_id: Uuid,
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

    let use_case = LoginUseCase {
        user_repo: state.user_repo.clone(),
        session_repo: state.session_repo.clone(),
        password_hasher: state.password_hasher.clone(),
        token_service: state.token_service.clone(),
        event_bus: state.event_bus.clone(),
    };

    let res = use_case.execute(&ctx, &payload.email, &payload.password).await?;

    // Fetch user info to include in login response
    let user = state.user_repo.find_by_email(&payload.email).await?
        .ok_or_else(|| ApiError::internal("User not found after login"))?;

    Ok(ApiResponse::ok(LoginResponse {
        token: res.token,
        refresh_token: res.refresh_token,
        user: UserDto {
            id: user.id,
            org_id: user.org_id,
            email: user.email,
            full_name: user.full_name,
            status: user.status,
        },
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

    let user = state.user_repo.find_by_id(user_id).await?
        .ok_or_else(|| ApiError::not_found("User not found"))?;

    Ok(ApiResponse::ok(SessionResponse {
        org_id: ctx.tenant_id,
        user: UserDto {
            id: user.id,
            org_id: user.org_id,
            email: user.email,
            full_name: user.full_name,
            status: user.status,
        },
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
    let use_case = RegisterUserUseCase {
        user_repo: state.user_repo.clone(),
        person_repo: state.person_repo.clone(),
        event_bus: state.event_bus.clone(),
        password_hasher: state.password_hasher.clone(),
    };

    let role = match payload.role.as_str() {
        "SuperAdmin" => DomainRole::SuperAdmin,
        "OrgAdmin" => DomainRole::OrgAdmin,
        "Teacher" => DomainRole::Teacher,
        "Student" => DomainRole::Student,
        "Parent" => DomainRole::Parent,
        _ => DomainRole::Custom(payload.role.clone()),
    };

    let user = use_case.execute(&ctx, payload.full_name, Some(payload.email), payload.password, role, Some(ctx.tenant_id)).await?;

    Ok(ApiResponse::ok(UserDto {
        id: user.id,
        org_id: user.org_id,
        email: user.email,
        full_name: user.full_name,
        status: user.status,
    }))
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
    let dtos: Vec<UserDto> = users.into_iter().map(|u| UserDto {
        id: u.id,
        org_id: u.org_id,
        email: u.email,
        full_name: u.full_name,
        status: u.status,
    }).collect();
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

    Ok(ApiResponse::ok(UserDto {
        id: user.id,
        org_id: user.org_id,
        email: user.email,
        full_name: user.full_name,
        status: user.status,
    }))
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
    let use_case = orbis_app::usecases::auth::EnrollTotpUseCase {
        user_repo: state.user_repo.clone(),
        mfa_factor_repo: state.mfa_factor_repo.clone(),
        recovery_code_repo: state.recovery_code_repo.clone(),
        totp_service: state.totp_service.clone(),
        password_hasher: state.password_hasher.clone(),
    };
    let res = use_case.execute(&ctx, payload.factor_name).await?;
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
    let use_case = orbis_app::usecases::auth::VerifyTotpEnrollmentUseCase {
        user_repo: state.user_repo.clone(),
        mfa_factor_repo: state.mfa_factor_repo.clone(),
        totp_service: state.totp_service.clone(),
    };
    let verified = use_case.execute(&ctx, payload.factor_id, &payload.code).await?;
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
    let use_case = orbis_app::usecases::auth::VerifyMfaChallengeUseCase {
        user_repo: state.user_repo.clone(),
        mfa_factor_repo: state.mfa_factor_repo.clone(),
        recovery_code_repo: state.recovery_code_repo.clone(),
        device_repo: state.device_repo.clone(),
        session_repo: state.session_repo.clone(),
        totp_service: state.totp_service.clone(),
        token_service: state.token_service.clone(),
        password_hasher: state.password_hasher.clone(),
    };
    let res = use_case.execute(
        payload.user_id,
        &payload.code,
        payload.device_id,
        payload.remember_device.unwrap_or(false),
    ).await?;

    let user = state.user_repo.find_by_id(payload.user_id).await?
        .ok_or_else(|| ApiError::not_found("User not found"))?;

    Ok(ApiResponse::ok(LoginResponse {
        token: res.token,
        refresh_token: res.refresh_token,
        user: UserDto {
            id: user.id,
            org_id: user.org_id,
            email: user.email,
            full_name: user.full_name,
            status: user.status,
        },
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
    let use_case = orbis_app::usecases::auth::LinkGoogleAccountUseCase {
        identity_repo: state.identity_repo.clone(),
        oidc_service: state.oidc_service.clone(),
    };
    let identity = use_case.execute(&ctx, &payload.id_token).await?;
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
    let use_case = orbis_app::usecases::auth::LoginWithGoogleUseCase {
        identity_repo: state.identity_repo.clone(),
        user_repo: state.user_repo.clone(),
        session_repo: state.session_repo.clone(),
        oidc_service: state.oidc_service.clone(),
        token_service: state.token_service.clone(),
    };
    let res = use_case.execute(&payload.id_token).await?;
    let user = state.user_repo.find_by_id(res.user_id).await?
        .ok_or_else(|| ApiError::not_found("User not found"))?;

    Ok(ApiResponse::ok(LoginResponse {
        token: res.token,
        refresh_token: res.refresh_token,
        user: UserDto {
            id: user.id,
            org_id: user.org_id,
            email: user.email,
            full_name: user.full_name,
            status: user.status,
        },
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
    let use_case = orbis_app::usecases::auth::DeviceManagementUseCase {
        device_repo: state.device_repo.clone(),
    };
    let devices = use_case.list_devices(&ctx).await?;
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
    let use_case = orbis_app::usecases::auth::DeviceManagementUseCase {
        device_repo: state.device_repo.clone(),
    };
    use_case.revoke_device(&ctx, id).await?;
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
        .route("/api/v1/iam/users/:id", get(get_user))
}

