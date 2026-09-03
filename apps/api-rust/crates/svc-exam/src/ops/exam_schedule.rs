//! Exam schedule operations — exam timetable CRUD.

use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, ModelTrait, QueryFilter, QueryOrder};
use uuid::Uuid;

use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;
use shared_kernel::traits::{AuditEntry, Permission};

use crate::entities::exam_schedule;

use shared_kernel::query_filter::UniversalFilter;

pub async fn list(
    ctx: &ServiceContext, session_id: &str,
    filter: Option<UniversalFilter>,
) -> Result<Vec<exam_schedule::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::ExamSessionRead).await?;
    crate::ops::session::get(ctx, session_id).await?;
    let mut query = exam_schedule::Entity::find()
        .filter(exam_schedule::Column::ExamSessionId.eq(session_id));

    if let Some(f) = filter {
        let condition = f.build_condition(|field| match field {
            "exam_date" => Some(exam_schedule::Column::ExamDate.into_expr().into()),
            "status" => Some(exam_schedule::Column::Status.into_expr().into()),
            "period" => Some(exam_schedule::Column::Period.into_expr().into()),
            _ => None,
        });
        query = query.filter(condition);
    }

    Ok(query.order_by_asc(exam_schedule::Column::ExamDate)
        .order_by_asc(exam_schedule::Column::StartTime)
        .all(&ctx.db).await?)
}

pub async fn create(
    ctx: &ServiceContext,
    exam_session_id: String, exam_subject_id: String,
    exam_date: String, start_time: String, end_time: String,
    period: String, applicable_class_ids: Option<String>,
    room_ids: Option<String>,
) -> Result<exam_schedule::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::ExamSessionWrite).await?;
    crate::ops::session::get(ctx, &exam_session_id).await?;

    let now = Utc::now().to_rfc3339();
    let id = Uuid::new_v4().to_string();

    let model = exam_schedule::Model {
        id: id.clone(), exam_session_id, exam_subject_id,
        exam_date, start_time, end_time, period,
        applicable_class_ids, room_ids,
        status: "scheduled".into(),
        postponed_to: None, postponement_reason: None,
        notes: None,
        created_at: now.clone(), updated_at: now,
    };

    let active: exam_schedule::ActiveModel = model.into();
    let created = active.insert(&ctx.db).await?;

    ctx.audit.log(AuditEntry {
        org_id: ctx.org_id.clone(), actor: ctx.actor.clone(),
        action: "exam_schedule.create".into(), entity_type: "exam_schedule".into(),
        entity_id: id, old_data: None,
        new_data: Some(serde_json::to_value(&created).unwrap_or_default()),
        correlation_id: ctx.correlation_id.clone(),
    }).await?;

    Ok(created)
}

pub async fn update(
    ctx: &ServiceContext, schedule_id: &str,
    exam_date: Option<String>, start_time: Option<String>,
    status: Option<String>,
) -> Result<exam_schedule::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::ExamSessionWrite).await?;

    let existing = exam_schedule::Entity::find_by_id(schedule_id.to_string())
        .one(&ctx.db).await?
        .ok_or_else(|| AppError::not_found("exam_schedule", schedule_id))?;
    crate::ops::session::get(ctx, &existing.exam_session_id).await?;

    let mut active: exam_schedule::ActiveModel = existing.into();
    if let Some(d) = exam_date { active.exam_date = Set(d); }
    if let Some(t) = start_time { active.start_time = Set(t); }
    if let Some(s) = status { active.status = Set(s); }
    active.updated_at = Set(Utc::now().to_rfc3339());

    Ok(active.update(&ctx.db).await?)
}

pub async fn delete(ctx: &ServiceContext, schedule_id: &str) -> Result<(), AppError> {
    ctx.permissions.check(&ctx.actor, Permission::ExamSessionWrite).await?;
    let existing = exam_schedule::Entity::find_by_id(schedule_id.to_string())
        .one(&ctx.db).await?
        .ok_or_else(|| AppError::not_found("exam_schedule", schedule_id))?;
    crate::ops::session::get(ctx, &existing.exam_session_id).await?;
    existing.delete(&ctx.db).await?;
    Ok(())
}
