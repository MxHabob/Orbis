use tauri::State;
use crate::state::AppState;
use orbis_app::usecases::iam::RegisterUserUseCase;
use orbis_app::usecases::login::LoginUseCase;
use orbis_domain::entities::user::Role;
use orbis_app::context::RequestContext;
use uuid::Uuid;
use serde::Serialize;

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub refresh_token: String,
}

#[tauri::command]
pub async fn register_user(
    full_name: String,
    email: Option<String>,
    password: Option<String>,
    role_str: String,
    org_id: Option<String>,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let ctx = RequestContext {
        tenant_id: Uuid::new_v4(),
        actor: orbis_domain::iam::actor::Actor::System,
        request_id: Uuid::new_v4().to_string(),
    };

    let usecase = RegisterUserUseCase {
        user_repo: state.user_repo.clone(),
        person_repo: state.person_repo.clone(),
        event_bus: state.event_bus.clone(),
        password_hasher: state.password_hasher.clone(),
    };

    let role = match role_str.as_str() {
        "SuperAdmin" => Role::SuperAdmin,
        "OrgAdmin" => Role::OrgAdmin,
        "Teacher" => Role::Teacher,
        "Student" => Role::Student,
        "Parent" => Role::Parent,
        _ => Role::Custom(role_str),
    };

    let parsed_org_id = match org_id {
        Some(id_str) => match Uuid::parse_str(&id_str) {
            Ok(uuid) => Some(uuid),
            Err(_) => return Err("Invalid org ID format".into()),
        },
        None => None,
    };

    match usecase.execute(&ctx, full_name, email, password, role, parsed_org_id).await {
        Ok(user) => Ok(user.id.to_string()),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub async fn login_user(
    email: String,
    password: String,
    state: State<'_, AppState>,
) -> Result<LoginResponse, String> {
    let ctx = RequestContext {
        tenant_id: Uuid::new_v4(),
        actor: orbis_domain::iam::actor::Actor::System,
        request_id: Uuid::new_v4().to_string(),
    };

    let usecase = LoginUseCase {
        user_repo: state.user_repo.clone(),
        session_repo: state.session_repo.clone(),
        password_hasher: state.password_hasher.clone(),
        token_service: state.token_service.clone(),
        event_bus: state.event_bus.clone(),
    };

    match usecase.execute(&ctx, &email, &password).await {
        Ok(res) => Ok(LoginResponse {
            token: res.token,
            refresh_token: res.refresh_token,
        }),
        Err(e) => Err(e.to_string()),
    }
}
