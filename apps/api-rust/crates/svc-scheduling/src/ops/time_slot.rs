//! Time slot operations — define the daily period structure.

use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, ModelTrait, QueryFilter, QueryOrder};
use uuid::Uuid;

use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;
use shared_kernel::traits::{AuditEntry, Permission};

use crate::entities::time_slot;

pub async fn list(ctx: &ServiceContext) -> Result<Vec<time_slot::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::ScheduleRead).await?;
    Ok(time_slot::Entity::find()
        .filter(time_slot::Column::OrgId.eq(&ctx.org_id))
        .filter(time_slot::Column::IsActive.eq(1))
        .order_by_asc(time_slot::Column::SortOrder)
        .all(&ctx.db).await?)
}

#[derive(serde::Deserialize)]
pub struct CreateTimeSlotPayload {
    pub period_id: String,
    pub slot_number: i32,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub name_ar: Option<String>,
    pub start_time: String,
    pub end_time: String,
    pub duration_minutes: i32,
    #[serde(default)]
    pub is_break: bool,
    #[serde(default)]
    pub break_type: Option<String>,
    pub applicable_days: Option<String>,
}

pub async fn create(
    ctx: &ServiceContext,
    data: serde_json::Value,
) -> Result<time_slot::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::ScheduleWrite).await?;

    let req: CreateTimeSlotPayload = serde_json::from_value(data)
        .map_err(|e| AppError::validation(format!("Invalid create time slot payload: {e}")))?;

    let now = Utc::now().to_rfc3339();
    let id = Uuid::new_v4().to_string();

    let model = time_slot::Model {
        id: id.clone(), org_id: ctx.org_id.clone(), period_id: req.period_id,
        slot_number: req.slot_number, name: req.name, name_ar: req.name_ar,
        start_time: req.start_time, end_time: req.end_time, duration_minutes: req.duration_minutes,
        is_break: if req.is_break { 1 } else { 0 }, break_type: req.break_type,
        applicable_days: req.applicable_days, sort_order: Some(req.slot_number),
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

#[derive(serde::Deserialize)]
pub struct UpdateTimeSlotPayload {
    pub name: Option<String>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
}

pub async fn update(
    ctx: &ServiceContext,
    slot_id: &str,
    data: serde_json::Value,
) -> Result<time_slot::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::ScheduleWrite).await?;

    let req: UpdateTimeSlotPayload = serde_json::from_value(data)
        .map_err(|e| AppError::validation(format!("Invalid update time slot payload: {e}")))?;

    let existing = time_slot::Entity::find_by_id(slot_id.to_string())
        .one(&ctx.db).await?
        .ok_or_else(|| AppError::not_found("time_slot", slot_id))?;

    let mut active: time_slot::ActiveModel = existing.into();
    if let Some(n) = req.name { active.name = Set(Some(n)); }
    if let Some(s) = req.start_time { active.start_time = Set(s); }
    if let Some(e) = req.end_time { active.end_time = Set(e); }
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
