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

// ── Router ──────────────────────────────────────────────────────

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/v1/auth/login", post(login))
        .route("/api/v1/auth/session", get(get_session))
        .route("/api/v1/iam/users", post(register_user).get(list_users))
        .route("/api/v1/iam/users/{id}", get(get_user))
}
