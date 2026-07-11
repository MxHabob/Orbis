//! Exam hall operations — CRUD + allocation.

use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, ModelTrait, QueryFilter, QueryOrder};
use uuid::Uuid;

use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;
use shared_kernel::traits::{AuditEntry, Permission};

use crate::entities::{exam_hall, exam_hall_allocation};

#[derive(serde::Deserialize)]
pub struct CreateHallRequest {
    pub name: String,
    pub capacity: i32,
    pub exam_session_id: Option<String>,
    pub room_id: Option<String>,
    pub building: Option<String>,
    pub floor_number: Option<i32>,
}

pub async fn list(ctx: &ServiceContext) -> Result<Vec<exam_hall::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::ExamSessionRead).await?;
    Ok(exam_hall::Entity::find()
        .filter(exam_hall::Column::OrgId.eq(&ctx.org_id))
        .order_by_asc(exam_hall::Column::HallNumber)
        .all(&ctx.db).await?)
}

pub async fn create(
    ctx: &ServiceContext,
    data: serde_json::Value,
) -> Result<exam_hall::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::ExamSessionWrite).await?;

    let req: CreateHallRequest = serde_json::from_value(data)
        .map_err(|e| AppError::validation(format!("Invalid request payload: {e}")))?;

    let now = Utc::now().to_rfc3339();
    let id = Uuid::new_v4().to_string();

    let model = exam_hall::Model {
        id: id.clone(),
        org_id: ctx.org_id.clone(),
        exam_session_id: req.exam_session_id.unwrap_or_default(),
        room_id: req.room_id.unwrap_or_default(),
        hall_number: req.name,
        capacity: req.capacity,
        allocated_count: 0,
        seating_arrangement: None,
        floor_number: req.floor_number,
        building: req.building,
        is_accessible: 1,
        status: "active".to_string(),
        notes: None,
        created_at: now.clone(),
        updated_at: now,
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

/// List hall allocations for an exam session.
pub async fn list_allocations(
    ctx: &ServiceContext, exam_session_id: &str,
) -> Result<Vec<exam_hall_allocation::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::ExamSessionRead).await?;
    Ok(exam_hall_allocation::Entity::find()
        .filter(exam_hall_allocation::Column::ExamSessionId.eq(exam_session_id))
        .all(&ctx.db).await?)
}

/// Allocate a hall for an enrollment in an exam session.
pub async fn allocate(
    ctx: &ServiceContext,
    exam_session_id: String, exam_hall_id: String, enrollment_id: String,
) -> Result<exam_hall_allocation::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::ExamSessionWrite).await?;

    let now = Utc::now().to_rfc3339();
    let id = Uuid::new_v4().to_string();

    let model = exam_hall_allocation::Model {
        id: id.clone(),
        exam_hall_id,
        exam_session_id,
        enrollment_id,
        seating_number_id: None,
        seat_position: None,
        is_present: None,
        created_at: now.clone(),
        updated_at: now,
    };

    let active: exam_hall_allocation::ActiveModel = model.into();
    Ok(active.insert(&ctx.db).await?)
}
