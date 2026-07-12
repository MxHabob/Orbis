//! Academic period operations — CRUD + lock/unlock + enrollment/grade entry windows.
//!
//! Replaces the old `period.rs` that operated on `academic_periods`.
//! All operations now work on `academic_periods` with explicit year scoping via ctx.year_id.

use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter, QueryOrder, IntoActiveModel};
use uuid::Uuid;

use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;
use shared_kernel::traits::{AuditEntry, Permission};

use crate::entities::academic_period;

/// List all periods for the current year.
pub async fn list(ctx: &ServiceContext) -> Result<Vec<academic_period::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::PeriodRead).await?;
    Ok(academic_period::Entity::find()
        .filter(academic_period::Column::OrgId.eq(&ctx.org_id))
        .filter(academic_period::Column::YearId.eq(&ctx.year_id))
        .order_by_asc(academic_period::Column::SortOrder)
        .all(&ctx.db).await?)
}

/// List periods for a specific year (cross-year queries).
pub async fn list_by_year(ctx: &ServiceContext, year_id: &str) -> Result<Vec<academic_period::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::PeriodRead).await?;
    Ok(academic_period::Entity::find()
        .filter(academic_period::Column::OrgId.eq(&ctx.org_id))
        .filter(academic_period::Column::YearId.eq(year_id))
        .order_by_asc(academic_period::Column::SortOrder)
        .all(&ctx.db).await?)
}

/// Get a single period by ID.
pub async fn get(ctx: &ServiceContext, period_id: &str) -> Result<academic_period::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::PeriodRead).await?;
    academic_period::Entity::find_by_id(period_id.to_string())
        .one(&ctx.db).await?
        .ok_or_else(|| AppError::not_found("academic_period", period_id))
}

/// Get the current (active) period for the current year.
pub async fn get_current(ctx: &ServiceContext) -> Result<academic_period::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::PeriodRead).await?;
    
    academic_period::Entity::find()
        .filter(academic_period::Column::OrgId.eq(&ctx.org_id))
        .filter(academic_period::Column::YearId.eq(&ctx.year_id))
        .order_by_asc(academic_period::Column::SortOrder)
        .one(&ctx.db).await?
        .ok_or_else(|| AppError::not_found("academic_period", "current"))
}

#[derive(serde::Deserialize)]
pub struct CreatePeriodPayload {
    pub name: String,
    pub name_ar: Option<String>,
    pub code: Option<String>,
    pub period_type: String,
    pub parent_id: Option<String>,
    pub start_date: String,
    pub end_date: String,
    pub enrollment_start: Option<String>,
    pub enrollment_end: Option<String>,
    pub grade_entry_start: Option<String>,
    pub grade_entry_end: Option<String>,
    pub exam_start: Option<String>,
    pub exam_end: Option<String>,
    pub grading_system: Option<String>,
    pub passing_grade: Option<i32>,
    pub max_grade: Option<i32>,
    pub weight: Option<i32>,
    pub sort_order: Option<i32>,
}

pub async fn create(
    ctx: &ServiceContext,
    data: serde_json::Value,
) -> Result<academic_period::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::PeriodWrite).await?;

    let req: CreatePeriodPayload = serde_json::from_value(data)
        .map_err(|e| AppError::validation(format!("Invalid create period payload: {e}")))?;

    let now = Utc::now().to_rfc3339();
    let id = Uuid::new_v4().to_string();

    let model = academic_period::ActiveModel {
        id: Set(id.clone()),
        org_id: Set(ctx.org_id.clone()),
        year_id: Set(ctx.year_id.clone()),
        parent_id: Set(req.parent_id),
        name: Set(req.name),
        name_ar: Set(req.name_ar),
        code: Set(req.code),
        period_type: Set(req.period_type),
        start_date: Set(req.start_date),
        end_date: Set(req.end_date),
        hijri_start: Set(None),
        hijri_end: Set(None),
        enrollment_start: Set(req.enrollment_start),
        enrollment_end: Set(req.enrollment_end),
        is_enrollment_open: Set(0),
        grade_entry_start: Set(req.grade_entry_start),
        grade_entry_end: Set(req.grade_entry_end),
        is_grade_entry_open: Set(0),
        exam_start: Set(req.exam_start),
        exam_end: Set(req.exam_end),
        grading_system: Set(req.grading_system.unwrap_or_else(|| "percentage".to_string())),
        passing_grade: Set(req.passing_grade),
        max_grade: Set(req.max_grade),
        weight: Set(req.weight),
        status: Set("planned".to_string()),
        is_locked: Set(0),
        locked_at: Set(None),
        locked_by: Set(None),
        sort_order: Set(req.sort_order),
        created_at: Set(now.clone()),
        updated_at: Set(now),
    };

    let created = model.insert(&ctx.db).await?;

    ctx.audit.log(AuditEntry {
        org_id: ctx.org_id.clone(), actor: ctx.actor.clone(),
        action: "period.create".into(), entity_type: "academic_period".into(),
        entity_id: id, old_data: None,
        new_data: Some(serde_json::to_value(&created).unwrap_or_default()),
        correlation_id: ctx.correlation_id.clone(),
    }).await?;

    Ok(created)
}

pub async fn lock(ctx: &ServiceContext, period_id: &str) -> Result<academic_period::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::PeriodLock).await?;

    let existing = get(ctx, period_id).await?;
    let mut active = existing.into_active_model();
    active.is_locked = Set(1);
    active.locked_at = Set(Some(Utc::now().to_rfc3339()));
    active.locked_by = Set(Some(ctx.actor.as_str().to_string()));
    active.updated_at = Set(Utc::now().to_rfc3339());

    Ok(active.update(&ctx.db).await?)
}

pub async fn unlock(ctx: &ServiceContext, period_id: &str) -> Result<academic_period::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::PeriodLock).await?;

    let existing = get(ctx, period_id).await?;
    let mut active = existing.into_active_model();
    active.is_locked = Set(0);
    active.locked_at = Set(None);
    active.locked_by = Set(None);
    active.updated_at = Set(Utc::now().to_rfc3339());

    Ok(active.update(&ctx.db).await?)
}

pub async fn toggle_enrollment(
    ctx: &ServiceContext,
    period_id: &str,
    open: bool,
) -> Result<academic_period::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::PeriodWrite).await?;

    let existing = get(ctx, period_id).await?;
    let mut active = existing.into_active_model();
    active.is_enrollment_open = Set(if open { 1 } else { 0 });
    active.updated_at = Set(Utc::now().to_rfc3339());

    Ok(active.update(&ctx.db).await?)
}

pub async fn toggle_grade_entry(
    ctx: &ServiceContext,
    period_id: &str,
    open: bool,
) -> Result<academic_period::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::PeriodWrite).await?;

    let existing = get(ctx, period_id).await?;
    let mut active = existing.into_active_model();
    active.is_grade_entry_open = Set(if open { 1 } else { 0 });
    active.updated_at = Set(Utc::now().to_rfc3339());

    Ok(active.update(&ctx.db).await?)
}
