//! Time slot operations — define the daily period structure.

use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, ModelTrait, QueryFilter, QueryOrder};
use uuid::Uuid;

use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;
use shared_kernel::traits::{AuditEntry, Permission};

use crate::entities::time_slot;

pub async fn list(ctx: &ServiceContext, cycle_id: &str) -> Result<Vec<time_slot::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::ScheduleRead).await?;
    Ok(time_slot::Entity::find()
        .filter(time_slot::Column::OrgId.eq(&ctx.org_id))
        .filter(time_slot::Column::CycleId.eq(cycle_id))
        .filter(time_slot::Column::IsActive.eq(1))
        .order_by_asc(time_slot::Column::SortOrder)
        .all(&ctx.db).await?)
}

pub async fn create(
    ctx: &ServiceContext, cycle_id: String, slot_number: i32,
    name: Option<String>, name_ar: Option<String>,
    start_time: String, end_time: String, duration_minutes: i32,
    is_break: bool, break_type: Option<String>,
    applicable_days: Option<String>,
) -> Result<time_slot::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::ScheduleWrite).await?;

    let now = Utc::now().to_rfc3339();
    let id = Uuid::new_v4().to_string();

    let model = time_slot::Model {
        id: id.clone(), org_id: ctx.org_id.clone(), cycle_id,
        slot_number, name, name_ar, start_time, end_time, duration_minutes,
        is_break: if is_break { 1 } else { 0 }, break_type,
        applicable_days, sort_order: Some(slot_number),
        is_active: 1, created_at: now.clone(), updated_at: now,
    };

    let active: time_slot::ActiveModel = model.into();
    let created = active.insert(&ctx.db).await?;

    ctx.audit.log(AuditEntry {
        org_id: ctx.org_id.clone(), actor: ctx.actor.clone(),
        action: "time_slot.create".into(), entity_type: "time_slot".into(),
        entity_id: id, old_data: None,
        new_data: Some(serde_json::to_value(&created).unwrap_or_default()),
        correlation_id: ctx.correlation_id.clone(),
    }).await?;

    Ok(created)
}

pub async fn update(
    ctx: &ServiceContext, slot_id: &str,
    name: Option<String>, start_time: Option<String>, end_time: Option<String>,
) -> Result<time_slot::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::ScheduleWrite).await?;

    let existing = time_slot::Entity::find_by_id(slot_id.to_string())
        .one(&ctx.db).await?
        .ok_or_else(|| AppError::not_found("time_slot", slot_id))?;

    let mut active: time_slot::ActiveModel = existing.into();
    if let Some(n) = name { active.name = Set(Some(n)); }
    if let Some(s) = start_time { active.start_time = Set(s); }
    if let Some(e) = end_time { active.end_time = Set(e); }
    active.updated_at = Set(Utc::now().to_rfc3339());

    Ok(active.update(&ctx.db).await?)
}

pub async fn delete(ctx: &ServiceContext, slot_id: &str) -> Result<(), AppError> {
    ctx.permissions.check(&ctx.actor, Permission::ScheduleDelete).await?;

    let existing = time_slot::Entity::find_by_id(slot_id.to_string())
        .one(&ctx.db).await?
        .ok_or_else(|| AppError::not_found("time_slot", slot_id))?;

    existing.delete(&ctx.db).await?;
    Ok(())
}
