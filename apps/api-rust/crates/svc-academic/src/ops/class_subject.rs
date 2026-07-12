//! Class-Subject assignment operations.

use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter};
use uuid::Uuid;

use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;
use shared_kernel::traits::{AuditEntry, Permission};

use crate::entities::class_subject;

// ── Queries ─────────────────────────────────────────────────────

/// List subjects assigned to a class in a period.
pub async fn list(
    ctx: &ServiceContext,
    class_id: &str,
    period_id: Option<&str>,
) -> Result<Vec<class_subject::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::ClassRead).await?;

    let cid = period_id.unwrap_or("default");

    Ok(class_subject::Entity::find()
        .filter(class_subject::Column::ClassId.eq(class_id))
        .filter(class_subject::Column::PeriodId.eq(cid))
        .filter(class_subject::Column::IsActive.eq(1))
        .all(&ctx.db)
        .await?)
}

/// List subjects assigned to a teacher in a period.
pub async fn list_by_teacher(
    ctx: &ServiceContext,
    teacher_id: &str,
    period_id: &str,
) -> Result<Vec<class_subject::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::ClassRead).await?;

    Ok(class_subject::Entity::find()
        .filter(class_subject::Column::PrimaryTeacherId.eq(teacher_id))
        .filter(class_subject::Column::PeriodId.eq(period_id))
        .filter(class_subject::Column::IsActive.eq(1))
        .all(&ctx.db)
        .await?)
}

// ── Commands ────────────────────────────────────────────────────

#[derive(serde::Deserialize)]
pub struct CreateClassSubjectPayload {
    pub class_id: String,
    pub subject_id: String,
    pub period_id: String,
    pub primary_teacher_id: Option<String>,
    pub weekly_hours: Option<i32>,
}

/// Assign a subject to a class with a teacher.
pub async fn create(
    ctx: &ServiceContext,
    data: serde_json::Value,
) -> Result<class_subject::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::ClassWrite).await?;

    let req: CreateClassSubjectPayload = serde_json::from_value(data)
        .map_err(|e| AppError::validation(format!("Invalid create class subject payload: {e}")))?;

    let now = Utc::now().to_rfc3339();
    let id = Uuid::new_v4().to_string();

    let model = class_subject::Model {
        id: id.clone(),
        class_id: req.class_id,
        subject_id: req.subject_id,
        period_id: req.period_id,
        primary_teacher_id: req.primary_teacher_id.unwrap_or_default(),
        assistant_teacher_id: None,
        substitute_teacher_id: None,
        grade_groups: None,
        weekly_hours: req.weekly_hours,
        theory_hours: None,
        practical_hours: None,
        assessment_config: None,
        is_active: 1,
        created_at: now.clone(),
        updated_at: now,
    };

    let active: class_subject::ActiveModel = model.into();
    let created = active.insert(&ctx.db).await?;

    ctx.audit.log(AuditEntry {
        org_id: ctx.org_id.clone(),
        actor: ctx.actor.clone(),
        action: "class_subject.create".into(),
        entity_type: "class_subject".into(),
        entity_id: id,
        old_data: None,
        new_data: Some(serde_json::to_value(&created).unwrap_or_default()),
        correlation_id: ctx.correlation_id.clone(),
    }).await?;

    Ok(created)
}

#[derive(serde::Deserialize)]
pub struct UpdateClassSubjectPayload {
    pub primary_teacher_id: Option<String>,
    pub weekly_hours: Option<i32>,
}

/// Update a class-subject assignment (teacher, hours, etc.).
pub async fn update(
    ctx: &ServiceContext,
    id: &str,
    data: serde_json::Value,
) -> Result<class_subject::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::ClassWrite).await?;

    let req: UpdateClassSubjectPayload = serde_json::from_value(data)
        .map_err(|e| AppError::validation(format!("Invalid update class subject payload: {e}")))?;

    let existing = class_subject::Entity::find_by_id(id.to_string())
        .one(&ctx.db)
        .await?
        .ok_or_else(|| AppError::not_found("class_subject", id))?;

    let mut active: class_subject::ActiveModel = existing.into();
    if let Some(tid) = req.primary_teacher_id {
        active.primary_teacher_id = Set(tid);
    }
    if let Some(hrs) = req.weekly_hours {
        active.weekly_hours = Set(Some(hrs));
    }
    active.updated_at = Set(Utc::now().to_rfc3339());

    let updated = active.update(&ctx.db).await?;

    ctx.audit.log(AuditEntry {
        org_id: ctx.org_id.clone(),
        actor: ctx.actor.clone(),
        action: "class_subject.update".into(),
        entity_type: "class_subject".into(),
        entity_id: id.into(),
        old_data: None,
        new_data: Some(serde_json::to_value(&updated).unwrap_or_default()),
        correlation_id: ctx.correlation_id.clone(),
    }).await?;

    Ok(updated)
}

/// Soft-delete a class-subject assignment.
pub async fn delete(ctx: &ServiceContext, id: &str) -> Result<(), AppError> {
    ctx.permissions.check(&ctx.actor, Permission::ClassWrite).await?;

    let existing = class_subject::Entity::find_by_id(id.to_string())
        .one(&ctx.db)
        .await?
        .ok_or_else(|| AppError::not_found("class_subject", id))?;

    let mut active: class_subject::ActiveModel = existing.into();
    active.is_active = Set(0);
    active.updated_at = Set(Utc::now().to_rfc3339());
    active.update(&ctx.db).await?;

    ctx.audit.log(AuditEntry {
        org_id: ctx.org_id.clone(),
        actor: ctx.actor.clone(),
        action: "class_subject.delete".into(),
        entity_type: "class_subject".into(),
        entity_id: id.into(),
        old_data: None,
        new_data: None,
        correlation_id: ctx.correlation_id.clone(),
    }).await?;

    Ok(())
}
