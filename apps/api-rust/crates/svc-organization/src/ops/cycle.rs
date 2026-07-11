//! Academic cycle operations — CRUD + lock/unlock + grade entry window.

use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter, QueryOrder};
use uuid::Uuid;

use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;
use shared_kernel::traits::{AuditEntry, Permission};

use crate::entities::academic_cycle;

pub async fn list(ctx: &ServiceContext) -> Result<Vec<academic_cycle::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::CycleRead).await?;
    Ok(academic_cycle::Entity::find()
        .filter(academic_cycle::Column::OrgId.eq(&ctx.org_id))
        .order_by_desc(academic_cycle::Column::StartDate)
        .all(&ctx.db).await?)
}

pub async fn get(ctx: &ServiceContext, cycle_id: &str) -> Result<academic_cycle::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::CycleRead).await?;
    academic_cycle::Entity::find_by_id(cycle_id.to_string())
        .one(&ctx.db).await?
        .ok_or_else(|| AppError::not_found("academic_cycle", cycle_id))
}

pub async fn get_current(ctx: &ServiceContext) -> Result<academic_cycle::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::CycleRead).await?;
    academic_cycle::Entity::find()
        .filter(academic_cycle::Column::OrgId.eq(&ctx.org_id))
        .filter(academic_cycle::Column::IsCurrent.eq(1))
        .one(&ctx.db).await?
        .ok_or_else(|| AppError::not_found("academic_cycle", "current"))
}

#[derive(serde::Deserialize)]
pub struct CreateCyclePayload {
    pub name: String,
    pub name_ar: Option<String>,
    pub cycle_type: String,
    pub parent_id: Option<String>,
    pub start_date: String,
    pub end_date: String,
    pub grading_system: String,
}

pub async fn create(
    ctx: &ServiceContext,
    data: serde_json::Value,
) -> Result<academic_cycle::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::CycleWrite).await?;

    let req: CreateCyclePayload = serde_json::from_value(data)
        .map_err(|e| AppError::validation(format!("Invalid create cycle payload: {e}")))?;

    let now = Utc::now().to_rfc3339();
    let id = Uuid::new_v4().to_string();

    let model = academic_cycle::Model {
        id: id.clone(), org_id: ctx.org_id.clone(),
        name: req.name, name_ar: req.name_ar, code: None, cycle_type: req.cycle_type, parent_id: req.parent_id,
        start_date: req.start_date, end_date: req.end_date,
        hijri_start_date: None, hijri_end_date: None,
        exam_start_date: None, exam_end_date: None,
        weight: None, grading_system: req.grading_system,
        passing_grade: None, max_grade: None,
        is_current: 0, is_locked: 0, is_enrollment_open: 0,
        is_grade_entry_open: 0,
        grade_entry_start: None, grade_entry_end: None,
        locked_at: None, locked_by: None,
        settings: None, sort_order: None,
        created_at: now.clone(), updated_at: now,
    };

    let active: academic_cycle::ActiveModel = model.into();
    let created = active.insert(&ctx.db).await?;

    ctx.audit.log(AuditEntry {
        org_id: ctx.org_id.clone(), actor: ctx.actor.clone(),
        action: "cycle.create".into(), entity_type: "academic_cycle".into(),
        entity_id: id, old_data: None,
        new_data: Some(serde_json::to_value(&created).unwrap_or_default()),
        correlation_id: ctx.correlation_id.clone(),
    }).await?;

    Ok(created)
}

#[derive(serde::Deserialize)]
pub struct UpdateCyclePayload {
    pub name: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

pub async fn update(
    ctx: &ServiceContext,
    cycle_id: &str,
    data: serde_json::Value,
) -> Result<academic_cycle::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::CycleWrite).await?;

    let req: UpdateCyclePayload = serde_json::from_value(data)
        .map_err(|e| AppError::validation(format!("Invalid update cycle payload: {e}")))?;

    let existing = get(ctx, cycle_id).await?;
    let mut active: academic_cycle::ActiveModel = existing.into();

    if let Some(n) = req.name { active.name = Set(n); }
    if let Some(sd) = req.start_date { active.start_date = Set(sd); }
    if let Some(ed) = req.end_date { active.end_date = Set(ed); }
    active.updated_at = Set(Utc::now().to_rfc3339());

    Ok(active.update(&ctx.db).await?)
}

pub async fn set_current(ctx: &ServiceContext, cycle_id: &str) -> Result<academic_cycle::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::CycleWrite).await?;

    // Unset all current
    let all = academic_cycle::Entity::find()
        .filter(academic_cycle::Column::OrgId.eq(&ctx.org_id))
        .filter(academic_cycle::Column::IsCurrent.eq(1))
        .all(&ctx.db).await?;

    for c in all {
        let mut a: academic_cycle::ActiveModel = c.into();
        a.is_current = Set(0);
        a.update(&ctx.db).await?;
    }

    let existing = get(ctx, cycle_id).await?;
    let mut active: academic_cycle::ActiveModel = existing.into();
    active.is_current = Set(1);
    active.updated_at = Set(Utc::now().to_rfc3339());

    let updated = active.update(&ctx.db).await?;

    ctx.audit.log(AuditEntry {
        org_id: ctx.org_id.clone(), actor: ctx.actor.clone(),
        action: "cycle.set_current".into(), entity_type: "academic_cycle".into(),
        entity_id: cycle_id.into(), old_data: None,
        new_data: Some(serde_json::json!({ "is_current": true })),
        correlation_id: ctx.correlation_id.clone(),
    }).await?;

    Ok(updated)
}

pub async fn lock(ctx: &ServiceContext, cycle_id: &str) -> Result<academic_cycle::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::CycleLock).await?;

    let existing = get(ctx, cycle_id).await?;
    let mut active: academic_cycle::ActiveModel = existing.into();
    active.is_locked = Set(1);
    active.locked_at = Set(Some(Utc::now().to_rfc3339()));
    active.locked_by = Set(Some(ctx.actor.as_str().to_string()));
    active.updated_at = Set(Utc::now().to_rfc3339());

    Ok(active.update(&ctx.db).await?)
}

pub async fn unlock(ctx: &ServiceContext, cycle_id: &str) -> Result<academic_cycle::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::CycleLock).await?;

    let existing = get(ctx, cycle_id).await?;
    let mut active: academic_cycle::ActiveModel = existing.into();
    active.is_locked = Set(0);
    active.locked_at = Set(None);
    active.locked_by = Set(None);
    active.updated_at = Set(Utc::now().to_rfc3339());

    Ok(active.update(&ctx.db).await?)
}

pub async fn toggle_grade_entry(
    ctx: &ServiceContext, cycle_id: &str, open: bool,
    start: Option<String>, end: Option<String>,
) -> Result<academic_cycle::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::CycleWrite).await?;

    let existing = get(ctx, cycle_id).await?;
    let mut active: academic_cycle::ActiveModel = existing.into();
    active.is_grade_entry_open = Set(if open { 1 } else { 0 });
    if let Some(s) = start { active.grade_entry_start = Set(Some(s)); }
    if let Some(e) = end { active.grade_entry_end = Set(Some(e)); }
    active.updated_at = Set(Utc::now().to_rfc3339());

    Ok(active.update(&ctx.db).await?)
}

pub async fn open_grade_entry(
    ctx: &ServiceContext,
    cycle_id: &str,
) -> Result<academic_cycle::Model, AppError> {
    toggle_grade_entry(ctx, cycle_id, true, None, None).await
}

pub async fn close_grade_entry(
    ctx: &ServiceContext,
    cycle_id: &str,
) -> Result<academic_cycle::Model, AppError> {
    toggle_grade_entry(ctx, cycle_id, false, None, None).await
}
