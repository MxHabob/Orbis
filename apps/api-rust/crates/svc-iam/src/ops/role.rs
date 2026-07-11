//! Role operations — assign/revoke roles.

use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, ModelTrait, QueryFilter};
use uuid::Uuid;

use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;
use shared_kernel::event_bus::EventEnvelope;
use shared_kernel::traits::{AuditEntry, Permission};

use crate::contracts::events::event_types;
use crate::entities::user_role;

pub async fn list_by_user(
    ctx: &ServiceContext, user_id: &str,
) -> Result<Vec<user_role::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::UserRead).await?;
    Ok(user_role::Entity::find()
        .filter(user_role::Column::UserId.eq(user_id))
        .filter(user_role::Column::OrgId.eq(&ctx.org_id))
        .all(&ctx.db).await?)
}

pub async fn assign(
    ctx: &ServiceContext, user_id: String, role: String,
    scope_type: String, scope_id: Option<String>,
) -> Result<user_role::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::RoleWrite).await?;

    let id = Uuid::new_v4().to_string();
    let model = user_role::Model {
        id: id.clone(), user_id: user_id.clone(), role: role.clone(),
        org_id: ctx.org_id.clone(), scope_type, scope_id,
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
        payload: serde_json::json!({ "user_id": user_id, "role": role, "action": "assign" }),
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

pub async fn revoke(ctx: &ServiceContext, role_id: &str) -> Result<(), AppError> {
    ctx.permissions.check(&ctx.actor, Permission::RoleWrite).await?;

    let existing = user_role::Entity::find_by_id(role_id.to_string())
        .one(&ctx.db).await?
        .ok_or_else(|| AppError::not_found("user_role", role_id))?;

    let user_id = existing.user_id.clone();
    let role = existing.role.clone();
    existing.delete(&ctx.db).await?;

    ctx.event_bus.publish(EventEnvelope {
        event_type: event_types::ROLE_CHANGED.into(),
        entity_type: "user_role".into(), entity_id: role_id.into(),
        org_id: ctx.org_id.clone(), actor: ctx.actor.as_str().into(),
        actor_type: ctx.actor.actor_type().into(),
        correlation_id: ctx.correlation_id.clone(),
        payload: serde_json::json!({ "user_id": user_id, "role": role, "action": "revoke" }),
        chain_depth: 0,
    });

    ctx.audit.log(AuditEntry {
        org_id: ctx.org_id.clone(), actor: ctx.actor.clone(),
        action: "role.revoke".into(), entity_type: "user_role".into(),
        entity_id: role_id.into(), old_data: None, new_data: None,
        correlation_id: ctx.correlation_id.clone(),
    }).await?;

    Ok(())
}
