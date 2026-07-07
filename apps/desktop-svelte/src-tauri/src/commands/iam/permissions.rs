use tauri::State;
use std::sync::Arc;
use std::collections::HashMap;
use sea_orm::{EntityTrait, ColumnTrait, QueryFilter, ActiveModelTrait};
use domain_iam::entities::{permission, role_permission, user_permission};
use crate::state::AppState;
use crate::commands::error::{CmdResult, CommandError};

#[tauri::command]
pub async fn get_permissions(
    state: State<'_, Arc<AppState>>,
) -> CmdResult<Vec<permission::Model>> {
    let db = state.active_db().await;
    let perms = permission::Entity::find()
        .all(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?;
    Ok(perms)
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct PermissionRuleDto {
    pub role: String,
    pub document_type: String,
    pub level: i32,
    pub only_if_creator: bool,
    pub actions: Vec<String>,
}

#[tauri::command]
pub async fn get_role_permissions(
    state: State<'_, Arc<AppState>>,
) -> CmdResult<Vec<PermissionRuleDto>> {
    let db = state.active_db().await;
    let role_perms = role_permission::Entity::find()
        .all(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?;

    let mut result = Vec::new();
    for rp in role_perms {
        let actions: Vec<String> = serde_json::from_str(&rp.actions)
            .unwrap_or_default();
        result.push(PermissionRuleDto {
            role: rp.role,
            document_type: rp.document_type,
            level: rp.level,
            only_if_creator: rp.only_if_creator == 1,
            actions,
        });
    }

    Ok(result)
}

#[tauri::command]
pub async fn save_role_permissions(
    state: State<'_, Arc<AppState>>,
    role: String,
    rules: Vec<PermissionRuleDto>,
) -> CmdResult<()> {
    let db = state.active_db().await;
    
    // Delete existing role permissions for this role
    role_permission::Entity::delete_many()
        .filter(role_permission::Column::Role.eq(&role))
        .exec(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?;

    // Insert new role permissions
    for r in rules {
        let now = chrono::Utc::now().to_rfc3339();
        let model = role_permission::Model {
            role: r.role,
            document_type: r.document_type,
            level: r.level,
            only_if_creator: if r.only_if_creator { 1 } else { 0 },
            actions: serde_json::to_string(&r.actions).unwrap_or_default(),
            created_at: now,
        };
        let active: role_permission::ActiveModel = model.into();
        active.insert(&db).await.map_err(|e| CommandError::new(e.to_string()))?;
    }

    Ok(())
}

#[tauri::command]
pub async fn get_user_permissions(
    state: State<'_, Arc<AppState>>,
    user_id: String,
) -> CmdResult<HashMap<String, String>> {
    let db = state.active_db().await;
    
    let user_perms = user_permission::Entity::find()
        .filter(user_permission::Column::UserId.eq(user_id))
        .all(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?;

    let all_perms = permission::Entity::find()
        .all(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?;

    let perm_map: HashMap<i32, String> = all_perms.into_iter().map(|p| (p.id, p.code)).collect();

    let mut result = HashMap::new();
    for up in user_perms {
        if let Some(code) = perm_map.get(&up.permission_id) {
            result.insert(code.clone(), up.grant_type);
        }
    }
    
    Ok(result)
}

#[tauri::command]
pub async fn save_user_permissions(
    state: State<'_, Arc<AppState>>,
    user_id: String,
    permissions: HashMap<String, String>,
) -> CmdResult<()> {
    let db = state.active_db().await;
    
    // Delete existing user permissions for this user
    user_permission::Entity::delete_many()
        .filter(user_permission::Column::UserId.eq(&user_id))
        .exec(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?;

    // Load all permissions to map code to ID
    let all_perms = permission::Entity::find()
        .all(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?;

    let code_map: HashMap<String, i32> = all_perms.into_iter().map(|p| (p.code, p.id)).collect();

    // Insert new user permissions
    for (code, grant_type) in permissions {
        if let Some(&permission_id) = code_map.get(&code) {
            let now = chrono::Utc::now().to_rfc3339();
            let model = user_permission::Model {
                id: uuid::Uuid::new_v4().to_string(),
                user_id: user_id.clone(),
                permission_id,
                grant_type,
                conditions: None,
                valid_from: None,
                valid_until: None,
                reason: None,
                granted_by: None,
                granted_at: now,
                is_active: 1,
            };
            let active: user_permission::ActiveModel = model.into();
            active.insert(&db).await.map_err(|e| CommandError::new(e.to_string()))?;
        }
    }
    
    Ok(())
}

#[tauri::command]
pub async fn get_available_roles(
    _state: State<'_, Arc<AppState>>,
) -> CmdResult<Vec<String>> {
    Ok(vec![
        "owner".to_string(),
        "admin".to_string(),
        "teacher".to_string(),
        "student".to_string(),
        "guardian".to_string(),
        "staff".to_string(),
    ])
}
