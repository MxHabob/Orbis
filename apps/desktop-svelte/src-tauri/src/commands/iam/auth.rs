use tauri::State;
use std::sync::Arc;
use domain_iam::entities::user;
use domain_iam::services::auth::LoginResponse;
use crate::state::AppState;
use crate::commands::error::{CmdResult, CommandError};

// ── Request types ──────────────────────────────────────────────────────────

#[derive(serde::Serialize, serde::Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub full_name: String,
    pub org_slug: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

// ── Commands ───────────────────────────────────────────────────────────────

/// Register a new user and create their organization if it doesn't exist.
#[tauri::command]
pub async fn register(
    state: State<'_, Arc<AppState>>,
    request: RegisterRequest,
) -> CmdResult<user::Model> {
    let db = state.active_db().await;
    
    // 1. Create organization
    let org_cmd = Arc::new(domain_lifecycle::repository::LifecycleCommandRepo::new(db.clone()));
    let org_query = Arc::new(domain_lifecycle::repository::LifecycleQueryRepo::new(db.clone()));
    let org_service = domain_lifecycle::services::organization::OrganizationService::new(org_cmd, org_query);
    let org = org_service.create(&request.org_slug, &request.org_slug, "UTC", "en", "SAR")
        .await
        .map_err(CommandError::from)?;
    
    // 2. Hash password
    let hashed_pw = domain_iam::services::auth::AuthService::hash_password(&request.password)
        .map_err(CommandError::from)?;
    
    // 3. Create user
    let user_cmd = Arc::new(domain_iam::repository::IamCommandRepo::new(db.clone()));
    let user_query = Arc::new(domain_iam::repository::IamQueryRepo::new(db.clone()));
    let user_service = domain_iam::services::user_management::UserService::new(user_cmd, user_query);
    
    let user = user_service.create_user(&org.id, Some(&request.email), &request.full_name, "owner", Some(hashed_pw))
        .await
        .map_err(CommandError::from)?;
    
    Ok(user)
}

/// Authenticate with email + password. Returns user model and JWT tokens.
#[tauri::command]
pub async fn login(
    state: State<'_, Arc<AppState>>,
    request: LoginRequest,
) -> CmdResult<LoginResponse> {
    let db = state.active_db().await;
    let cmd = Arc::new(domain_iam::repository::IamCommandRepo::new(db.clone()));
    let query = Arc::new(domain_iam::repository::IamQueryRepo::new(db.clone()));
    let service = domain_iam::services::auth::AuthService::new(cmd, query, state.config.clone());
    
    let resp = service.login(&request.email, &request.password)
        .await
        .map_err(CommandError::from)?;
        
    Ok(resp)
}

/// Validate a JWT token and return the associated user.
/// Called on app startup to restore session.
#[tauri::command]
pub async fn validate_token(
    state: State<'_, Arc<AppState>>,
    token: String,
) -> CmdResult<user::Model> {
    let db = state.active_db().await;
    let cmd = Arc::new(domain_iam::repository::IamCommandRepo::new(db.clone()));
    let query = Arc::new(domain_iam::repository::IamQueryRepo::new(db.clone()));
    let service = domain_iam::services::auth::AuthService::new(cmd, query, state.config.clone());
    
    let user = service.validate_token(&token)
        .await
        .map_err(CommandError::from)?;
        
    Ok(user)
}

/// Invalidate the current session token.
#[tauri::command]
pub async fn logout(
    state: State<'_, Arc<AppState>>,
    token: String,
) -> CmdResult<()> {
    use sea_orm::ColumnTrait;
    use sea_orm::EntityTrait;
    use sea_orm::QueryFilter;
    
    let db = state.active_db().await;
    domain_iam::entities::user_session::Entity::delete_many()
        .filter(domain_iam::entities::user_session::Column::Token.eq(token))
        .exec(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?;
        
    Ok(())
}
