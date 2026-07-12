//! Lesson operations — scheduled class periods.

use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, ModelTrait, QueryFilter, QueryOrder, Condition};
use uuid::Uuid;

use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;
use shared_kernel::traits::{AuditEntry, Permission};

use crate::entities::scheduled_lesson;

pub async fn list_by_class(
    ctx: &ServiceContext, period_id: &str, class_id: &str,
) -> Result<Vec<scheduled_lesson::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::ScheduleRead).await?;
    Ok(scheduled_lesson::Entity::find()
        .filter(scheduled_lesson::Column::OrgId.eq(&ctx.org_id))
        .filter(scheduled_lesson::Column::PeriodId.eq(period_id))
        .filter(scheduled_lesson::Column::ClassId.eq(class_id))
        .order_by_asc(scheduled_lesson::Column::DayOfWeek)
        .all(&ctx.db).await?)
}

pub async fn list_by_teacher(
    ctx: &ServiceContext, period_id: &str, teacher_id: &str,
) -> Result<Vec<scheduled_lesson::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::ScheduleRead).await?;
    Ok(scheduled_lesson::Entity::find()
        .filter(scheduled_lesson::Column::OrgId.eq(&ctx.org_id))
        .filter(scheduled_lesson::Column::PeriodId.eq(period_id))
        .filter(scheduled_lesson::Column::TeacherId.eq(teacher_id))
        .order_by_asc(scheduled_lesson::Column::DayOfWeek)
        .all(&ctx.db).await?)
}

pub async fn get(ctx: &ServiceContext, lesson_id: &str) -> Result<scheduled_lesson::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::ScheduleRead).await?;
    scheduled_lesson::Entity::find_by_id(lesson_id.to_string())
        .one(&ctx.db).await?
        .ok_or_else(|| AppError::not_found("scheduled_lesson", lesson_id))
}

#[derive(serde::Deserialize)]
pub struct CreateLessonRequest {
    pub period_id: String,
    pub class_id: String,
    pub class_subject_id: String,
    pub subject_id: String,
    pub teacher_id: String,
    pub time_slot_id: String,
    pub day_of_week: i32,
    pub room_id: Option<String>,
}

pub async fn create(
    ctx: &ServiceContext,
    data: serde_json::Value,
) -> Result<scheduled_lesson::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::ScheduleWrite).await?;

    let req: CreateLessonRequest = serde_json::from_value(data)
        .map_err(|e| AppError::validation(format!("Invalid create lesson payload: {e}")))?;

    let now = Utc::now().to_rfc3339();
    let id = Uuid::new_v4().to_string();

    let model = scheduled_lesson::Model {
        id: id.clone(), org_id: ctx.org_id.clone(), period_id: req.period_id,
        class_id: req.class_id, class_subject_id: req.class_subject_id,
        subject_id: req.subject_id, teacher_id: req.teacher_id,
        substitute_teacher_id: None, time_slot_id: req.time_slot_id, day_of_week: req.day_of_week,
        room_id: req.room_id, room_name: None, status: "scheduled".into(),
        is_substituted: 0, substitution_reason: None,
        topic: None, topic_ar: None, objectives: None,
        materials: None, homework: None, homework_due_date: None,
        attendance_taken: 0, attendance_taken_at: None,
        attendance_taken_by: None, attendance_method: None,
        notes: None, cancellation_reason: None,
        created_at: now.clone(), updated_at: now,
    };

    let active: scheduled_lesson::ActiveModel = model.into();
    let created = active.insert(&ctx.db).await?;

    ctx.audit.log(AuditEntry {
        org_id: ctx.org_id.clone(), actor: ctx.actor.clone(),
        action: "lesson.create".into(), entity_type: "scheduled_lesson".into(),
        entity_id: id, old_data: None,
        new_data: Some(serde_json::to_value(&created).unwrap_or_default()),
        correlation_id: ctx.correlation_id.clone(),
    }).await?;

    Ok(created)
}

#[derive(serde::Deserialize)]
pub struct UpdateLessonRequest {
    pub teacher_id: Option<String>,
    pub room_id: Option<String>,
    pub topic: Option<String>,
    pub status: Option<String>,
}

pub async fn update(
    ctx: &ServiceContext, lesson_id: &str,
    data: serde_json::Value,
) -> Result<scheduled_lesson::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::ScheduleWrite).await?;

    let req: UpdateLessonRequest = serde_json::from_value(data)
        .map_err(|e| AppError::validation(format!("Invalid update lesson payload: {e}")))?;

    let existing = get(ctx, lesson_id).await?;
    let mut active: scheduled_lesson::ActiveModel = existing.into();

    if let Some(tid) = req.teacher_id { active.teacher_id = Set(tid); }
    if let Some(rid) = req.room_id { active.room_id = Set(Some(rid)); }
    if let Some(t) = req.topic { active.topic = Set(Some(t)); }
    if let Some(s) = req.status { active.status = Set(s); }
    active.updated_at = Set(Utc::now().to_rfc3339());

    Ok(active.update(&ctx.db).await?)
}

pub async fn delete(ctx: &ServiceContext, lesson_id: &str) -> Result<(), AppError> {
    ctx.permissions.check(&ctx.actor, Permission::ScheduleDelete).await?;
    let existing = get(ctx, lesson_id).await?;
    existing.delete(&ctx.db).await?;
    Ok(())
}

/// Check for scheduling conflicts (same teacher or room at same time/day).
pub async fn check_conflicts(
    ctx: &ServiceContext, period_id: &str, teacher_id: &str,
    time_slot_id: &str, day_of_week: i32, room_id: Option<String>,
) -> Result<Vec<scheduled_lesson::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::ScheduleRead).await?;

    let mut condition = Condition::any()
        .add(scheduled_lesson::Column::TeacherId.eq(teacher_id));

    if let Some(rid) = room_id {
        condition = condition.add(scheduled_lesson::Column::RoomId.eq(rid));
    }

    Ok(scheduled_lesson::Entity::find()
        .filter(scheduled_lesson::Column::OrgId.eq(&ctx.org_id))
        .filter(scheduled_lesson::Column::PeriodId.eq(period_id))
        .filter(scheduled_lesson::Column::TimeSlotId.eq(time_slot_id))
        .filter(scheduled_lesson::Column::DayOfWeek.eq(day_of_week))
        .filter(condition)
        .all(&ctx.db).await?)
}
