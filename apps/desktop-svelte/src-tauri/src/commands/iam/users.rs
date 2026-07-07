use tauri::State;
use std::sync::Arc;
use sea_orm::{EntityTrait, ColumnTrait, QueryFilter, ActiveModelTrait, ActiveValue::Set, QueryOrder};
use domain_iam::entities::{user, user_session};
use domain_lifecycle::entities::organization;
use crate::state::AppState;
use crate::commands::error::{CmdResult, CommandError};

// ── Request and Param Types ──────────────────────────────────────────────────

#[derive(serde::Serialize, serde::Deserialize)]
pub struct UserParams {
    pub role: Option<String>,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CreateUserRequest {
    pub email: Option<String>,
    pub full_name: String,
    pub role: String,
    pub password: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct UpdateUserRequest {
    pub full_name: Option<String>,
    pub name_ar: Option<String>,
    pub name_en: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChangeRoleRequest {
    pub role: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChangePasswordRequest {
    pub new_password: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct UpdateOrganizationRequest {
    pub name: Option<String>,
    pub name_ar: Option<String>,
    pub primary_color: Option<String>,
}

// ── Organization ───────────────────────────────────────────────────────────

#[tauri::command]
pub async fn get_organization(
    state: State<'_, Arc<AppState>>,
    org_id: String,
) -> CmdResult<organization::Model> {
    let db = state.active_db().await;
    let org_cmd = Arc::new(domain_lifecycle::repository::LifecycleCommandRepo::new(db.clone()));
    let org_query = Arc::new(domain_lifecycle::repository::LifecycleQueryRepo::new(db.clone()));
    let org_service = domain_lifecycle::services::organization::OrganizationService::new(org_cmd, org_query);
    
    let org = org_service.get(&org_id).await.map_err(CommandError::from)?;
    Ok(org)
}

#[tauri::command]
pub async fn update_organization(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    request: UpdateOrganizationRequest,
) -> CmdResult<organization::Model> {
    let db = state.active_db().await;
    let org_cmd = Arc::new(domain_lifecycle::repository::LifecycleCommandRepo::new(db.clone()));
    let org_query = Arc::new(domain_lifecycle::repository::LifecycleQueryRepo::new(db.clone()));
    let org_service = domain_lifecycle::services::organization::OrganizationService::new(org_cmd, org_query);
    
    let existing = org_service.get(&org_id).await.map_err(CommandError::from)?;
    let mut active: organization::ActiveModel = existing.into();
    if let Some(v) = request.name { active.name = Set(v); }
    if let Some(v) = request.name_ar { active.name_ar = Set(Some(v)); }
    if let Some(v) = request.primary_color { active.primary_color = Set(Some(v)); }
    active.updated_at = Set(chrono::Utc::now().to_rfc3339());
    
    let updated = active.update(&db).await.map_err(|e| CommandError::new(e.to_string()))?;
    Ok(updated)
}

#[tauri::command]
pub async fn get_main_organization(
    state: State<'_, Arc<AppState>>,
) -> CmdResult<Option<organization::Model>> {
    let db = state.active_db().await;
    let org = organization::Entity::find()
        .one(&db)
        .await
        .map_err(|e| CommandError::from(e.to_string()))?;
    Ok(org)
}

// ── Users ──────────────────────────────────────────────────────────────────

#[tauri::command]
pub async fn get_users(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    params: UserParams,
) -> CmdResult<Vec<user::Model>> {
    let db = state.active_db().await;
    let cmd = Arc::new(domain_iam::repository::IamCommandRepo::new(db.clone()));
    let query = Arc::new(domain_iam::repository::IamQueryRepo::new(db.clone()));
    let user_service = domain_iam::services::user_management::UserService::new(cmd, query);
    
    let users = user_service.list_users(
        &org_id,
        params.role.as_deref(),
        params.limit.unwrap_or(50),
        params.offset.unwrap_or(0),
    ).await.map_err(CommandError::from)?;
    
    Ok(users)
}

#[tauri::command]
pub async fn get_user(
    state: State<'_, Arc<AppState>>,
    user_id: String,
) -> CmdResult<user::Model> {
    let db = state.active_db().await;
    let cmd = Arc::new(domain_iam::repository::IamCommandRepo::new(db.clone()));
    let query = Arc::new(domain_iam::repository::IamQueryRepo::new(db.clone()));
    let user_service = domain_iam::services::user_management::UserService::new(cmd, query);
    
    let u = user_service.get_user(&user_id).await.map_err(CommandError::from)?;
    Ok(u)
}

#[tauri::command]
pub async fn create_user(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    _created_byy: String,
    request: CreateUserRequest,
) -> CmdResult<user::Model> {
    let db = state.active_db().await;
    let cmd = Arc::new(domain_iam::repository::IamCommandRepo::new(db.clone()));
    let query = Arc::new(domain_iam::repository::IamQueryRepo::new(db.clone()));
    let user_service = domain_iam::services::user_management::UserService::new(cmd, query);
    
    let hashed = match &request.password {
        Some(p) => Some(domain_iam::services::auth::AuthService::hash_password(p).map_err(CommandError::from)?),
        None => None,
    };
    
    let u = user_service.create_user(
        &org_id,
        request.email.as_deref(),
        &request.full_name,
        &request.role,
        hashed,
    ).await.map_err(CommandError::from)?;
    
    Ok(u)
}

#[tauri::command]
pub async fn update_user(
    state: State<'_, Arc<AppState>>,
    user_id: String,
    request: UpdateUserRequest,
) -> CmdResult<user::Model> {
    let db = state.active_db().await;
    let cmd = Arc::new(domain_iam::repository::IamCommandRepo::new(db.clone()));
    let query = Arc::new(domain_iam::repository::IamQueryRepo::new(db.clone()));
    let user_service = domain_iam::services::user_management::UserService::new(cmd, query);
    
    let existing = user_service.get_user(&user_id).await.map_err(CommandError::from)?;
    let mut active: user::ActiveModel = existing.into();
    if let Some(v) = request.full_name { active.full_name = Set(v); }
    if let Some(v) = request.name_ar { active.name_ar = Set(Some(v)); }
    if let Some(v) = request.name_en { active.name_en = Set(Some(v)); }
    if let Some(v) = request.email { active.email = Set(Some(v)); }
    if let Some(v) = request.phone { active.phone = Set(Some(v)); }
    active.updated_at = Set(chrono::Utc::now().to_rfc3339());
    
    let u = active.update(&db).await.map_err(|e| CommandError::new(e.to_string()))?;
    Ok(u)
}

#[tauri::command]
pub async fn delete_user(
    state: State<'_, Arc<AppState>>,
    user_id: String,
    deleted_by: String,
) -> CmdResult<()> {
    let db = state.active_db().await;
    let cmd = Arc::new(domain_iam::repository::IamCommandRepo::new(db.clone()));
    let query = Arc::new(domain_iam::repository::IamQueryRepo::new(db.clone()));
    let user_service = domain_iam::services::user_management::UserService::new(cmd, query);
    
    user_service.soft_delete(&user_id, &deleted_by).await.map_err(CommandError::from)?;
    Ok(())
}

#[tauri::command]
pub async fn change_user_role(
    state: State<'_, Arc<AppState>>,
    user_id: String,
    request: ChangeRoleRequest,
) -> CmdResult<user::Model> {
    let db = state.active_db().await;
    let cmd = Arc::new(domain_iam::repository::IamCommandRepo::new(db.clone()));
    let query = Arc::new(domain_iam::repository::IamQueryRepo::new(db.clone()));
    let user_service = domain_iam::services::user_management::UserService::new(cmd, query);
    
    let existing = user_service.get_user(&user_id).await.map_err(CommandError::from)?;
    let mut active: user::ActiveModel = existing.into();
    active.primary_role = Set(request.role);
    active.updated_at = Set(chrono::Utc::now().to_rfc3339());
    
    let u = active.update(&db).await.map_err(|e| CommandError::new(e.to_string()))?;
    Ok(u)
}

#[tauri::command]
pub async fn change_user_password(
    state: State<'_, Arc<AppState>>,
    user_id: String,
    request: ChangePasswordRequest,
) -> CmdResult<user::Model> {
    let db = state.active_db().await;
    let cmd = Arc::new(domain_iam::repository::IamCommandRepo::new(db.clone()));
    let query = Arc::new(domain_iam::repository::IamQueryRepo::new(db.clone()));
    let user_service = domain_iam::services::user_management::UserService::new(cmd, query);
    
    let hashed = domain_iam::services::auth::AuthService::hash_password(&request.new_password)
        .map_err(CommandError::from)?;
        
    let existing = user_service.get_user(&user_id).await.map_err(CommandError::from)?;
    let mut active: user::ActiveModel = existing.into();
    active.password_hash = Set(Some(hashed));
    active.updated_at = Set(chrono::Utc::now().to_rfc3339());
    
    let u = active.update(&db).await.map_err(|e| CommandError::new(e.to_string()))?;
    Ok(u)
}

#[tauri::command]
pub async fn get_user_roles(
    state: State<'_, Arc<AppState>>,
    user_id: String,
) -> CmdResult<Vec<String>> {
    let db = state.active_db().await;
    let roles = domain_iam::entities::user_role::Entity::find()
        .filter(domain_iam::entities::user_role::Column::UserId.eq(user_id))
        .all(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?;
        
    Ok(roles.into_iter().map(|r| r.role).collect())
}

#[tauri::command]
pub async fn save_user_roles(
    state: State<'_, Arc<AppState>>,
    user_id: String,
    roles: Vec<String>,
) -> CmdResult<()> {
    let db = state.active_db().await;
    
    // Delete existing roles
    domain_iam::entities::user_role::Entity::delete_many()
        .filter(domain_iam::entities::user_role::Column::UserId.eq(&user_id))
        .exec(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?;

    // Insert new roles
    for role in roles {
        let now = chrono::Utc::now().to_rfc3339();
        let model = domain_iam::entities::user_role::Model {
            id: uuid::Uuid::new_v4().to_string(),
            user_id: user_id.clone(),
            role,
            org_id: "".to_string(),
            scope_type: "global".to_string(),
            scope_id: None,
            valid_from: None,
            valid_until: None,
            is_primary: 0,
            assigned_by: None,
            assigned_at: now,
        };
        let active: domain_iam::entities::user_role::ActiveModel = model.into();
        active.insert(&db).await.map_err(|e| CommandError::new(e.to_string()))?;
    }
    
    Ok(())
}

#[tauri::command]
pub async fn get_user_sessions(
    state: State<'_, Arc<AppState>>,
    user_id: String,
) -> CmdResult<Vec<user_session::Model>> {
    let db = state.active_db().await;
    let sessions = user_session::Entity::find()
        .filter(user_session::Column::UserId.eq(user_id))
        .order_by_desc(user_session::Column::CreatedAt)
        .all(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?;
        
    Ok(sessions)
}
