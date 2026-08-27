//! Academic period operations — CRUD + lock/unlock + enrollment/grade entry windows.
//!
//! Replaces the old `period.rs` that operated on `academic_periods`.
//! All operations now work on `academic_periods` with explicit year scoping via ctx.period_id.

use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter, QueryOrder, IntoActiveModel};
use uuid::Uuid;

use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;
use shared_kernel::traits::{AuditEntry, Permission};

use crate::entities::academic_period;

async fn invalidate_period_gate(ctx: &ServiceContext, period_id: &str) {
    ctx.period_gate.invalidate_period(period_id).await;
    ctx.period_gate.invalidate_policy(&ctx.org_id).await;
}

/// List all periods for the current year.
pub async fn list(ctx: &ServiceContext) -> Result<Vec<academic_period::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::PeriodRead).await?;
    let mut query = academic_period::Entity::find()
        .filter(academic_period::Column::OrgId.eq(&ctx.org_id));

    if !ctx.period_id.is_empty() {
        query = query.filter(academic_period::Column::PeriodId.eq(&ctx.period_id));
    }

    Ok(query.order_by_asc(academic_period::Column::SortOrder).all(&ctx.db).await?)
}

/// List all years and periods combined into a hierarchical JSON structure.
pub async fn list_hierarchy(ctx: &ServiceContext) -> Result<serde_json::Value, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::PeriodRead).await?;

    // Fetch all years
    let years = crate::entities::academic_year::Entity::find()
        .filter(crate::entities::academic_year::Column::OrgId.eq(&ctx.org_id))
        .order_by_desc(crate::entities::academic_year::Column::StartDate)
        .all(&ctx.db)
        .await?;

    // Fetch all periods
    let periods = academic_period::Entity::find()
        .filter(academic_period::Column::OrgId.eq(&ctx.org_id))
        .order_by_asc(academic_period::Column::SortOrder)
        .all(&ctx.db)
        .await?;

    let mut combined: Vec<serde_json::Value> = Vec::new();

    let year_ids: std::collections::HashSet<String> = years.iter().map(|y| y.id.clone()).collect();
    // Add years as root items (parent_id = null)
    for y in years {
        let mut val = serde_json::to_value(y).unwrap_or_default();
        val["parent_id"] = serde_json::Value::Null;
        val["period_type"] = serde_json::Value::String("year".to_string());
        combined.push(val);
    }

    // Add periods as child items (parent_id = period_id if no parent_id)
    for p in periods {
        // Skip if this period is already represented in years as a root year
        if year_ids.contains(&p.id) {
            continue;
        }

        let mut val = serde_json::to_value(p).unwrap_or_default();
        if val["parent_id"].is_null() {
            val["parent_id"] = val["period_id"].clone();
        }
        // Prevent self-referential parent_id
        if val["parent_id"] == val["id"] {
            val["parent_id"] = serde_json::Value::Null;
        }
        combined.push(val);
    }

    Ok(serde_json::Value::Array(combined))
}

/// List periods for a specific year (cross-year queries).
pub async fn list_by_year(ctx: &ServiceContext, period_id: &str) -> Result<Vec<academic_period::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::PeriodRead).await?;
    Ok(academic_period::Entity::find()
        .filter(academic_period::Column::OrgId.eq(&ctx.org_id))
        .filter(academic_period::Column::PeriodId.eq(period_id))
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
pub async fn get_current(ctx: &ServiceContext) -> Result<Option<academic_period::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::PeriodRead).await?;
    
    // First try active period for this org
    let active = academic_period::Entity::find()
        .filter(academic_period::Column::OrgId.eq(&ctx.org_id))
        .filter(academic_period::Column::Status.eq("active"))
        .one(&ctx.db).await?;

    if active.is_some() {
        return Ok(active);
    }

    // Fallback to most recent period for this org
    let latest = academic_period::Entity::find()
        .filter(academic_period::Column::OrgId.eq(&ctx.org_id))
        .order_by_desc(academic_period::Column::StartDate)
        .one(&ctx.db).await?;

    Ok(latest)
}

#[derive(serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
pub struct CreatePeriodPayload {
    pub name: String,
    #[serde(default)]
    pub name_ar: Option<String>,
    #[serde(default)]
    pub code: Option<String>,
    pub period_type: String,
    #[serde(default)]
    pub parent_id: Option<String>,
    pub start_date: String,
    pub end_date: String,
    #[serde(default)]
    pub enrollment_start: Option<String>,
    #[serde(default)]
    pub enrollment_end: Option<String>,
    #[serde(default)]
    pub grade_entry_start: Option<String>,
    #[serde(default)]
    pub grade_entry_end: Option<String>,
    #[serde(default)]
    pub exam_start: Option<String>,
    #[serde(default)]
    pub exam_end: Option<String>,
    #[serde(default)]
    pub scale_id: Option<String>,
    #[serde(default)]
    pub pass_fail_policy_id: Option<String>,
    #[serde(default)]
    pub sort_order: Option<i32>,
    #[serde(default)]
    pub metadata: Option<String>,
}

pub async fn create(
    ctx: &ServiceContext,
    req: CreatePeriodPayload,
) -> Result<academic_period::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::PeriodWrite).await?;

    let now = Utc::now().to_rfc3339();
    let id = Uuid::new_v4().to_string();
    let period_id_val = if !ctx.period_id.is_empty() {
        ctx.period_id.clone()
    } else if let Some(ref pid) = req.parent_id {
        if let Ok(Some(parent_period)) = academic_period::Entity::find_by_id(pid.clone()).one(&ctx.db).await {
            parent_period.period_id
        } else {
            pid.clone()
        }
    } else {
        id.clone()
    };

    let valid_types = [
        "year", "semester", "trimester", "quarter", "module", "block", "custom", "term", "month",
    ];
    let period_type_val = if valid_types.contains(&req.period_type.as_str()) {
        req.period_type
    } else {
        "custom".to_string()
    };

    // Ensure corresponding entry in academic_years exists to satisfy FK constraint
    let year_exists = crate::entities::academic_year::Entity::find_by_id(period_id_val.clone())
        .one(&ctx.db)
        .await?;

    if year_exists.is_none() {
        let year_model = crate::entities::academic_year::ActiveModel {
            id: Set(period_id_val.clone()),
            org_id: Set(ctx.org_id.clone()),
            name: Set(req.name.clone()),
            name_ar: Set(req.name_ar.clone()),
            start_date: Set(req.start_date.clone()),
            end_date: Set(req.end_date.clone()),
            hijri_start: Set(None),
            hijri_end: Set(None),
            status: Set("active".to_string()),
            is_locked: Set(0),
            locked_at: Set(None),
            locked_by: Set(None),
            lock_reason: Set(None),
            current_period_id: Set(None),
            archived_at: Set(None),
            archived_by: Set(None),
            notes: Set(None),
            created_at: Set(now.clone()),
            updated_at: Set(now.clone()),
        };
        let _ = year_model.insert(&ctx.db).await?;
    }

    let model = academic_period::ActiveModel {
        id: Set(id.clone()),
        org_id: Set(ctx.org_id.clone()),
        period_id: Set(period_id_val),
        parent_id: Set(req.parent_id),
        name: Set(req.name),
        name_ar: Set(req.name_ar),
        code: Set(req.code),
        period_type: Set(period_type_val),
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
        scale_id: Set(req.scale_id),
        pass_fail_policy_id: Set(req.pass_fail_policy_id),
        status: Set("planned".to_string()),
        is_locked: Set(0),
        locked_at: Set(None),
        locked_by: Set(None),
        sort_order: Set(req.sort_order),
        metadata: Set(req.metadata),
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

    let updated = active.update(&ctx.db).await?;
    invalidate_period_gate(ctx, period_id).await;
    Ok(updated)
}

pub async fn unlock(ctx: &ServiceContext, period_id: &str) -> Result<academic_period::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::PeriodLock).await?;

    let existing = get(ctx, period_id).await?;
    let mut active = existing.into_active_model();
    active.is_locked = Set(0);
    active.locked_at = Set(None);
    active.locked_by = Set(None);
    active.updated_at = Set(Utc::now().to_rfc3339());

    let updated = active.update(&ctx.db).await?;
    invalidate_period_gate(ctx, period_id).await;
    Ok(updated)
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

    let updated = active.update(&ctx.db).await?;
    invalidate_period_gate(ctx, period_id).await;
    Ok(updated)
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

    let updated = active.update(&ctx.db).await?;
    invalidate_period_gate(ctx, period_id).await;
    Ok(updated)
}

#[derive(serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
pub struct UpdatePeriodPayload {
    pub name: Option<String>,
    pub name_ar: Option<String>,
    pub code: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub scale_id: Option<String>,
    pub pass_fail_policy_id: Option<String>,
    pub sort_order: Option<i32>,
    pub metadata: Option<String>,
}

pub async fn update(
    ctx: &ServiceContext,
    period_id: &str,
    req: UpdatePeriodPayload,
) -> Result<academic_period::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::PeriodWrite).await?;

    let existing = get(ctx, period_id).await?;
    let mut active = existing.into_active_model();

    if let Some(name) = req.name { active.name = Set(name); }
    if let Some(name_ar) = req.name_ar { active.name_ar = Set(Some(name_ar)); }
    if let Some(code) = req.code { active.code = Set(Some(code)); }
    if let Some(start_date) = req.start_date { active.start_date = Set(start_date); }
    if let Some(end_date) = req.end_date { active.end_date = Set(end_date); }
    if let Some(scale_id) = req.scale_id { active.scale_id = Set(Some(scale_id)); }
    if let Some(pfp_id) = req.pass_fail_policy_id { active.pass_fail_policy_id = Set(Some(pfp_id)); }
    if let Some(so) = req.sort_order { active.sort_order = Set(Some(so)); }
    if let Some(meta) = req.metadata { active.metadata = Set(Some(meta)); }

    active.updated_at = Set(Utc::now().to_rfc3339());

    Ok(active.update(&ctx.db).await?)
}

pub async fn delete(
    ctx: &ServiceContext,
    period_id: &str,
) -> Result<(), AppError> {
    ctx.permissions.check(&ctx.actor, Permission::PeriodWrite).await?;

    let existing = get(ctx, period_id).await?;
    
    academic_period::Entity::delete_by_id(existing.id)
        .exec(&ctx.db)
        .await?;

    Ok(())
}
