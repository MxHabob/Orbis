//! Exam hall operations — CRUD + allocation.

use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, ModelTrait, QueryFilter, QueryOrder};
use uuid::Uuid;

use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;
use shared_kernel::traits::{AuditEntry, Permission};

use crate::entities::{exam_hall, exam_hall_allocation};

pub async fn list(ctx: &ServiceContext) -> Result<Vec<exam_hall::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::ExamSessionRead).await?;
    Ok(exam_hall::Entity::find()
        .filter(exam_hall::Column::OrgId.eq(&ctx.org_id))
        .order_by_asc(exam_hall::Column::Name)
        .all(&ctx.db).await?)
}

pub async fn create(
    ctx: &ServiceContext,
    name: String, name_ar: Option<String>, capacity: i32,
    building: Option<String>, floor: Option<i32>,
) -> Result<exam_hall::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::ExamSessionWrite).await?;

    let now = Utc::now().to_rfc3339();
    let id = Uuid::new_v4().to_string();

    let model = exam_hall::Model {
        id: id.clone(), org_id: ctx.org_id.clone(),
        name, name_ar, capacity, rows: None, columns: None,
        building, floor, is_active: 1,
        created_at: now.clone(), updated_at: now,
    };

    let active: exam_hall::ActiveModel = model.into();
    let created = active.insert(&ctx.db).await?;

    ctx.audit.log(AuditEntry {
        org_id: ctx.org_id.clone(), actor: ctx.actor.clone(),
        action: "exam_hall.create".into(), entity_type: "exam_hall".into(),
        entity_id: id, old_data: None,
        new_data: Some(serde_json::to_value(&created).unwrap_or_default()),
        correlation_id: ctx.correlation_id.clone(),
    }).await?;

    Ok(created)
}

pub async fn delete(ctx: &ServiceContext, hall_id: &str) -> Result<(), AppError> {
    ctx.permissions.check(&ctx.actor, Permission::ExamSessionWrite).await?;
    let existing = exam_hall::Entity::find_by_id(hall_id.to_string())
        .one(&ctx.db).await?
        .ok_or_else(|| AppError::not_found("exam_hall", hall_id))?;
    existing.delete(&ctx.db).await?;
    Ok(())
}

/// List hall allocations for an exam subject.
pub async fn list_allocations(
    ctx: &ServiceContext, exam_subject_id: &str,
) -> Result<Vec<exam_hall_allocation::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::ExamSessionRead).await?;
    Ok(exam_hall_allocation::Entity::find()
        .filter(exam_hall_allocation::Column::ExamSubjectId.eq(exam_subject_id))
        .all(&ctx.db).await?)
}

/// Allocate a hall for an exam subject.
pub async fn allocate(
    ctx: &ServiceContext,
    exam_subject_id: String, hall_id: String, student_count: i32,
) -> Result<exam_hall_allocation::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::ExamSessionWrite).await?;

    let now = Utc::now().to_rfc3339();
    let id = Uuid::new_v4().to_string();

    let model = exam_hall_allocation::Model {
        id: id.clone(), exam_subject_id, hall_id,
        student_count, proctor_count: None,
        start_seat: None, end_seat: None,
        created_at: now.clone(), updated_at: now,
    };

    let active: exam_hall_allocation::ActiveModel = model.into();
    Ok(active.insert(&ctx.db).await?)
}
