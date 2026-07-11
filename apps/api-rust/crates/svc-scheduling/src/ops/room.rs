//! Room operations.

use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, ModelTrait, QueryFilter, QueryOrder};
use uuid::Uuid;

use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;
use shared_kernel::traits::{AuditEntry, Permission};

use crate::entities::room;

pub async fn list(ctx: &ServiceContext) -> Result<Vec<room::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::ScheduleRead).await?;
    Ok(room::Entity::find()
        .filter(room::Column::OrgId.eq(&ctx.org_id))
        .filter(room::Column::IsActive.eq(1))
        .order_by_asc(room::Column::Name)
        .all(&ctx.db).await?)
}

pub async fn create(
    ctx: &ServiceContext, name: String, name_ar: Option<String>,
    capacity: Option<i32>, room_type: String,
    building: Option<String>, floor: Option<i32>,
) -> Result<room::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::ScheduleWrite).await?;

    let now = Utc::now().to_rfc3339();
    let id = Uuid::new_v4().to_string();

    let model = room::Model {
        id: id.clone(), org_id: ctx.org_id.clone(),
        name, name_ar, capacity, room_type, building, floor,
        is_active: 1, created_at: now.clone(), updated_at: now,
    };

    let active: room::ActiveModel = model.into();
    let created = active.insert(&ctx.db).await?;

    ctx.audit.log(AuditEntry {
        org_id: ctx.org_id.clone(), actor: ctx.actor.clone(),
        action: "room.create".into(), entity_type: "room".into(),
        entity_id: id, old_data: None,
        new_data: Some(serde_json::to_value(&created).unwrap_or_default()),
        correlation_id: ctx.correlation_id.clone(),
    }).await?;

    Ok(created)
}

pub async fn update(
    ctx: &ServiceContext, room_id: &str,
    name: Option<String>, capacity: Option<i32>,
) -> Result<room::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::ScheduleWrite).await?;

    let existing = room::Entity::find_by_id(room_id.to_string())
        .one(&ctx.db).await?
        .ok_or_else(|| AppError::not_found("room", room_id))?;

    let mut active: room::ActiveModel = existing.into();
    if let Some(n) = name { active.name = Set(n); }
    if let Some(c) = capacity { active.capacity = Set(Some(c)); }
    active.updated_at = Set(Utc::now().to_rfc3339());

    Ok(active.update(&ctx.db).await?)
}

pub async fn delete(ctx: &ServiceContext, room_id: &str) -> Result<(), AppError> {
    ctx.permissions.check(&ctx.actor, Permission::ScheduleDelete).await?;
    let existing = room::Entity::find_by_id(room_id.to_string())
        .one(&ctx.db).await?
        .ok_or_else(|| AppError::not_found("room", room_id))?;
    existing.delete(&ctx.db).await?;
    Ok(())
}
