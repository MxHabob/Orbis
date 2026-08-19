//! Role operations — assign/revoke roles.

use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, ModelTrait, QueryFilter};
use uuid::Uuid;

use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;
use shared_kernel::event_bus::EventEnvelope;
use shared_kernel::traits::{AuditEntry, Permission};

use crate::contracts::events::event_types;
use crate::entities::{role_permission, user_role};

pub async fn list_by_user(
    ctx: &ServiceContext, user_id: &str,
) -> Result<Vec<user_role::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::UserRead).await?;
    Ok(user_role::Entity::find()
        .filter(user_role::Column::UserId.eq(user_id))
        .filter(user_role::Column::OrgId.eq(&ctx.org_id))
        .all(&ctx.db).await?)
}

#[derive(serde::Deserialize)]
pub struct AssignRolePayload {
    pub user_id: String,
    pub role: String,
    pub scope_type: String,
    pub scope_id: Option<String>,
}

pub async fn assign(
    ctx: &ServiceContext,
    data: serde_json::Value,
) -> Result<user_role::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::UserRoleWrite).await?;

    let req: AssignRolePayload = serde_json::from_value(data)
        .map_err(|e| AppError::validation(format!("Invalid assign role payload: {e}")))?;

    let id = Uuid::new_v4().to_string();
    let model = user_role::Model {
        id: id.clone(), user_id: req.user_id.clone(), role: req.role.clone(),
        org_id: ctx.org_id.clone(), scope_type: req.scope_type, scope_id: req.scope_id,
        valid_from: None, valid_until: None, is_primary: 0,
        assigned_by: Some(ctx.actor.as_str().to_string()),
        assigned_at: Utc::now().to_rfc3339(),
    };

    let active: user_role::ActiveModel = model.into();
    let created = active.insert(&ctx.db).await?;

    ctx.event_bus.publish(EventEnvelope {
        event_type: event_types::ROLE_CHANGED.into(),
        entity_type: "user_role".into(), entity_id: id.clone(),
        org_id: ctx.org_id.clone(), actor: ctx.actor.as_str().into(),
        actor_type: ctx.actor.actor_type().into(),
        correlation_id: ctx.correlation_id.clone(),
        payload: serde_json::json!({ "user_id": req.user_id, "role": req.role, "action": "assign" }),
        chain_depth: 0,
    });

    ctx.audit.log(AuditEntry {
        org_id: ctx.org_id.clone(), actor: ctx.actor.clone(),
        action: "role.assign".into(), entity_type: "user_role".into(),
        entity_id: id, old_data: None,
        new_data: Some(serde_json::to_value(&created).unwrap_or_default()),
        correlation_id: ctx.correlation_id.clone(),
    }).await?;

    Ok(created)
}

pub async fn revoke(db: &sea_orm::DatabaseConnection, role_id: &str) -> Result<(), AppError> {
    let existing = user_role::Entity::find_by_id(role_id.to_string())
        .one(db).await?
        .ok_or_else(|| AppError::not_found("user_role", role_id))?;

    existing.delete(db).await?;
    Ok(())
}

pub async fn save_for_user(
    db: &sea_orm::DatabaseConnection,
    user_id: &str,
    roles: Vec<String>,
) -> Result<(), AppError> {
    // Delete all current roles for this user
    user_role::Entity::delete_many()
        .filter(user_role::Column::UserId.eq(user_id))
        .exec(db)
        .await?;

    // Fetch user record to determine the real org_id
    let user_record = crate::entities::user::Entity::find_by_id(user_id.to_string())
        .one(db)
        .await?
        .ok_or_else(|| AppError::not_found("user", user_id))?;

    let org_id = user_record.org_id;

    // Insert new roles
    for r in roles {
        let model = user_role::Model {
            id: Uuid::new_v4().to_string(),
            user_id: user_id.to_string(),
            role: r,
            org_id: org_id.clone(),
            scope_type: "global".to_string(),
            scope_id: None,
            valid_from: None,
            valid_until: None,
            is_primary: 0,
            assigned_by: Some("system".to_string()),
            assigned_at: Utc::now().to_rfc3339(),
        };
        let active: user_role::ActiveModel = model.into();
        active.insert(db).await?;
    }

    Ok(())
}

pub async fn get_permissions(
    db: &sea_orm::DatabaseConnection,
    role: Option<String>,
) -> Result<Vec<role_permission::Model>, AppError> {
    let query = role_permission::Entity::find();
    let perms = match role {
        Some(ref r) if !r.trim().is_empty() => {
            query.filter(role_permission::Column::Role.eq(r)).all(db).await?
        }
        _ => query.all(db).await?,
    };
    Ok(perms)
}

pub async fn save_permissions(
    db: &sea_orm::DatabaseConnection,
    role: &str,
    permissions: Vec<serde_json::Value>,
) -> Result<(), AppError> {

    role_permission::Entity::delete_many()
        .filter(role_permission::Column::Role.eq(role))
        .exec(db)
        .await?;

    let now = Utc::now().to_rfc3339();
    for perm in permissions {
        let actions_str = if perm["actions"].is_array() {
            serde_json::to_string(&perm["actions"]).unwrap_or_else(|_| "[]".to_string())
        } else if let Some(s) = perm["actions"].as_str() {
            if s.starts_with('[') {
                s.to_string()
            } else {
                let list: Vec<String> = s.split(',').map(|x| x.trim().to_string()).filter(|x| !x.is_empty()).collect();
                serde_json::to_string(&list).unwrap_or_else(|_| "[]".to_string())
            }
        } else {
            "[]".to_string()
        };

        let only_if_creator_val = if perm["only_if_creator"].as_bool().unwrap_or(false)
            || perm["only_if_creator"].as_i64().unwrap_or(0) == 1
        {
            1
        } else {
            0
        };

        let active = role_permission::ActiveModel {
            role: sea_orm::ActiveValue::Set(role.to_string()),
            document_type: sea_orm::ActiveValue::Set(perm["document_type"].as_str().unwrap_or_default().to_string()),
            level: sea_orm::ActiveValue::Set(perm["level"].as_i64().unwrap_or(0) as i32),
            only_if_creator: sea_orm::ActiveValue::Set(only_if_creator_val),
            actions: sea_orm::ActiveValue::Set(actions_str),
            created_at: sea_orm::ActiveValue::Set(now.clone()),
        };
        active.insert(db).await?;
    }
    Ok(())
}
