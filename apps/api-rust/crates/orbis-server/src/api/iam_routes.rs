use axum::{extract::State, Json, routing::post, Router};
use serde::{Deserialize, Serialize};
use orbis_app::context::RequestContext;
use orbis_app::usecases::iam::RegisterUserUseCase;
use orbis_app::usecases::login::LoginUseCase;
use orbis_domain::entities::user::Role;
use uuid::Uuid;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct RegisterUserRequest {
    pub email: String,
    pub full_name: String,
    pub password: Option<String>,
    pub role: String,
    pub org_id: Option<Uuid>,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: Option<String>,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponsePayload {
    pub token: String,
    pub refresh_token: String,
}

async fn register_user(
    State(state): State<AppState>,
    Json(payload): Json<RegisterUserRequest>,
) -> Result<Json<UserResponse>, String> {
    let ctx = RequestContext {
        tenant_id: Uuid::new_v4(),
        actor: orbis_domain::iam::actor::Actor::System,
        request_id: Uuid::new_v4().to_string(),
    };

    let use_case = RegisterUserUseCase {
        user_repo: state.user_repo.clone(),
        person_repo: state.person_repo.clone(),
        event_bus: state.event_bus.clone(),
        password_hasher: state.password_hasher.clone(),
    };

    let role = match payload.role.as_str() {
        "SuperAdmin" => Role::SuperAdmin,
        "OrgAdmin" => Role::OrgAdmin,
        "Teacher" => Role::Teacher,
        "Student" => Role::Student,
        "Parent" => Role::Parent,
        _ => Role::Custom(payload.role.clone()),
    };

    match use_case.execute(&ctx, payload.full_name, Some(payload.email), payload.password, role, payload.org_id).await {
        Ok(user) => Ok(Json(UserResponse {
            id: user.id,
            email: user.email,
        })),
        Err(e) => Err(e.to_string()),
    }
}

async fn login_user(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponsePayload>, String> {
    let ctx = RequestContext {
        tenant_id: Uuid::new_v4(),
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

    match use_case.execute(&ctx, &payload.email, &payload.password).await {
        Ok(res) => Ok(Json(LoginResponsePayload {
            token: res.token,
            refresh_token: res.refresh_token,
        })),
        Err(e) => Err(e.to_string()),
    }
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/users/register", post(register_user))
        .route("/auth/login", post(login_user))
}
