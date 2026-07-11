//! Room operations.

use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, ModelTrait, QueryFilter, QueryOrder};
use uuid::Uuid;

use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;
use shared_kernel::traits::{AuditEntry, Permission};

use crate::entities::room;

#[derive(serde::Deserialize)]
pub struct CreateRoomRequest {
    pub name: String,
    pub name_ar: Option<String>,
    pub capacity: Option<i32>,
    pub room_type: String,
    pub building: Option<String>,
    pub floor: Option<i32>,
}

#[derive(serde::Deserialize)]
pub struct UpdateRoomRequest {
    pub name: Option<String>,
    pub name_ar: Option<String>,
    pub capacity: Option<i32>,
    pub room_type: Option<String>,
    pub building: Option<String>,
    pub floor: Option<i32>,
}

pub async fn list(ctx: &ServiceContext) -> Result<Vec<room::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::ScheduleRead).await?;
    Ok(room::Entity::find()
        .filter(room::Column::OrgId.eq(&ctx.org_id))
        .filter(room::Column::IsActive.eq(1))
        .order_by_asc(room::Column::Name)
        .all(&ctx.db).await?)
}

pub async fn create(
    ctx: &ServiceContext,
    data: serde_json::Value,
) -> Result<room::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::ScheduleWrite).await?;

    let req: CreateRoomRequest = serde_json::from_value(data)
        .map_err(|e| AppError::validation(format!("Invalid create room payload: {e}")))?;

    let now = Utc::now().to_rfc3339();
    let id = Uuid::new_v4().to_string();

    let model = room::Model {
        id: id.clone(), org_id: ctx.org_id.clone(),
        name: req.name, name_ar: req.name_ar, capacity: req.capacity,
        room_type: req.room_type, building: req.building, floor: req.floor,
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
    data: serde_json::Value,
) -> Result<room::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::ScheduleWrite).await?;

    let req: UpdateRoomRequest = serde_json::from_value(data)
        .map_err(|e| AppError::validation(format!("Invalid update room payload: {e}")))?;

    let existing = room::Entity::find_by_id(room_id.to_string())
        .one(&ctx.db).await?
        .ok_or_else(|| AppError::not_found("room", room_id))?;

    let mut active: room::ActiveModel = existing.into();
    if let Some(n) = req.name { active.name = Set(n); }
    if let Some(na) = req.name_ar { active.name_ar = Set(Some(na)); }
    if let Some(c) = req.capacity { active.capacity = Set(Some(c)); }
    if let Some(rt) = req.room_type { active.room_type = Set(rt); }
    if let Some(b) = req.building { active.building = Set(Some(b)); }
    if let Some(f) = req.floor { active.floor = Set(Some(f)); }
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
