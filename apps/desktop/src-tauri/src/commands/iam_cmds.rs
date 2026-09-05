//! Tauri IPC commands — Auth & IAM.

use tauri::State;
use serde::Serialize;
use crate::state::AppState;
use orbis_app::usecases::iam::RegisterUserUseCase;
use orbis_app::usecases::login::LoginUseCase;
use orbis_domain::entities::user::Role as DomainRole;
use uuid::Uuid;

use super::make_ctx;
use super::org_cmds::ApiResponse;
fn ok<T: Serialize>(data: T) -> ApiResponse<T> { ApiResponse { data } }

#[derive(Serialize)]
pub struct UserDto {
    pub id: String, pub org_id: String, pub email: Option<String>,
    pub full_name: String, pub status: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String, pub refresh_token: String, pub user: UserDto,
}

#[derive(Serialize)]
pub struct SessionResponse {
    pub user: UserDto, pub org_id: String,
}

fn user_to_dto(u: orbis_domain::entities::user::User) -> UserDto {
    UserDto {
        id: u.id.to_string(), org_id: u.org_id.to_string(),
        email: u.email, full_name: u.full_name, status: u.status,
    }
}

#[tauri::command]
pub async fn auth_login(request: serde_json::Value, state: State<'_, AppState>) -> Result<ApiResponse<LoginResponse>, String> {
    let ctx = make_ctx(&state);
    let email = request["email"].as_str().unwrap_or_default();
    let password = request["password"].as_str().unwrap_or_default();
    let uc = LoginUseCase {
        user_repo: state.user_repo.clone(), session_repo: state.session_repo.clone(),
        password_hasher: state.password_hasher.clone(), token_service: state.token_service.clone(),
        event_bus: state.event_bus.clone(),
    };
    let res = uc.execute(&ctx, email, password).await.map_err(|e| e.to_string())?;
    let user = state.user_repo.find_by_email(email).await.map_err(|e| e.to_string())?
        .ok_or_else(|| "User not found".to_string())?;
    Ok(ok(LoginResponse { token: res.token, refresh_token: res.refresh_token, user: user_to_dto(user) }))
}

#[tauri::command]
pub async fn auth_get_session(state: State<'_, AppState>) -> Result<ApiResponse<SessionResponse>, String> {
    let ctx = make_ctx(&state);
    let user_id = ctx.actor.id().ok_or_else(|| "No user session".to_string())?;
    let user = state.user_repo.find_by_id(user_id).await.map_err(|e| e.to_string())?
        .ok_or_else(|| "User not found".to_string())?;
    Ok(ok(SessionResponse { org_id: ctx.tenant_id.to_string(), user: user_to_dto(user) }))
}

#[tauri::command]
pub async fn auth_logout(_state: State<'_, AppState>) -> Result<(), String> {
    // TODO: invalidate session in managed state
    Ok(())
}

#[tauri::command]
pub async fn iam_register_user(request: serde_json::Value, state: State<'_, AppState>) -> Result<ApiResponse<UserDto>, String> {
    let ctx = make_ctx(&state);
    let uc = RegisterUserUseCase {
        user_repo: state.user_repo.clone(), person_repo: state.person_repo.clone(),
        event_bus: state.event_bus.clone(), password_hasher: state.password_hasher.clone(),
    };
    let full_name = request["fullName"].as_str().unwrap_or_default().to_string();
    let email = request["email"].as_str().unwrap_or_default().to_string();
    let password = request["password"].as_str().map(|s| s.to_string());
    let role_str = request["role"].as_str().unwrap_or("Student");
    let role = match role_str {
        "SuperAdmin" => DomainRole::SuperAdmin, "OrgAdmin" => DomainRole::OrgAdmin,
        "Teacher" => DomainRole::Teacher, "Student" => DomainRole::Student,
        "Parent" => DomainRole::Parent, r => DomainRole::Custom(r.to_string()),
    };
    let user = uc.execute(&ctx, full_name, Some(email), password, role, Some(ctx.tenant_id)).await.map_err(|e| e.to_string())?;
    Ok(ok(user_to_dto(user)))
}

#[tauri::command]
pub async fn iam_list_users(state: State<'_, AppState>) -> Result<ApiResponse<Vec<UserDto>>, String> {
    let ctx = make_ctx(&state);
    let users = state.user_repo.find_by_org(ctx.tenant_id).await.map_err(|e| e.to_string())?;
    Ok(ok(users.into_iter().map(user_to_dto).collect()))
}

#[tauri::command]
pub async fn iam_get_user(id: String, state: State<'_, AppState>) -> Result<ApiResponse<UserDto>, String> {
    let uid = Uuid::parse_str(&id).map_err(|_| "Invalid ID")?;
    let user = state.user_repo.find_by_id(uid).await.map_err(|e| e.to_string())?
        .ok_or_else(|| "User not found".to_string())?;
    Ok(ok(user_to_dto(user)))
}
