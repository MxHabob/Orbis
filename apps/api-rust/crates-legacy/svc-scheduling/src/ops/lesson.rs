//! Lesson operations — scheduled class periods.

use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter, QueryOrder, Condition};
use uuid::Uuid;

use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;
use shared_kernel::traits::{AuditEntry, Permission};

use crate::entities::scheduled_lesson;
use svc_academic::entities::{class, class_subject, subject};
use crate::entities::{room, time_slot};

pub async fn list_by_class(
    ctx: &ServiceContext, period_id: &str, class_id: &str,
) -> Result<Vec<scheduled_lesson::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::ScheduleRead).await?;
    svc_academic::ops::scope::ensure_class_scope(ctx, class_id).await?;
    Ok(scheduled_lesson::Entity::find()
        .filter(scheduled_lesson::Column::OrgId.eq(&ctx.org_id))
        .filter(scheduled_lesson::Column::PeriodId.eq(period_id))
        .filter(scheduled_lesson::Column::ClassId.eq(class_id))
        .filter(scheduled_lesson::Column::Status.ne("cancelled"))
        .order_by_asc(scheduled_lesson::Column::DayOfWeek)
        .all(&ctx.db).await?)
}

pub async fn list_by_teacher(
    ctx: &ServiceContext, period_id: &str, teacher_id: &str,
) -> Result<Vec<scheduled_lesson::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::ScheduleRead).await?;
    let mut query = scheduled_lesson::Entity::find()
        .filter(scheduled_lesson::Column::OrgId.eq(&ctx.org_id))
        .filter(scheduled_lesson::Column::PeriodId.eq(period_id))
        .filter(scheduled_lesson::Column::TeacherId.eq(teacher_id))
        .filter(scheduled_lesson::Column::Status.ne("cancelled"));
    query = apply_branch_filter(ctx, query).await?;
    Ok(query.order_by_asc(scheduled_lesson::Column::DayOfWeek).all(&ctx.db).await?)
}

pub async fn list_by_period(
    ctx: &ServiceContext, period_id: &str,
) -> Result<Vec<scheduled_lesson::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::ScheduleRead).await?;
    let mut query = scheduled_lesson::Entity::find()
        .filter(scheduled_lesson::Column::OrgId.eq(&ctx.org_id))
        .filter(scheduled_lesson::Column::PeriodId.eq(period_id))
        .filter(scheduled_lesson::Column::Status.ne("cancelled"));
    query = apply_branch_filter(ctx, query).await?;
    Ok(query.order_by_asc(scheduled_lesson::Column::DayOfWeek).order_by_asc(scheduled_lesson::Column::TimeSlotId).all(&ctx.db).await?)
}

pub async fn list_by_subject(
    ctx: &ServiceContext, period_id: &str, subject_id: &str,
) -> Result<Vec<scheduled_lesson::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::ScheduleRead).await?;
    let mut query = scheduled_lesson::Entity::find()
        .filter(scheduled_lesson::Column::OrgId.eq(&ctx.org_id))
        .filter(scheduled_lesson::Column::PeriodId.eq(period_id))
        .filter(scheduled_lesson::Column::SubjectId.eq(subject_id))
        .filter(scheduled_lesson::Column::Status.ne("cancelled"));
    query = apply_branch_filter(ctx, query).await?;
    Ok(query.order_by_asc(scheduled_lesson::Column::DayOfWeek).order_by_asc(scheduled_lesson::Column::TimeSlotId).all(&ctx.db).await?)
}

pub async fn list_by_room(
    ctx: &ServiceContext, period_id: &str, room_id: &str,
) -> Result<Vec<scheduled_lesson::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::ScheduleRead).await?;
    let mut query = scheduled_lesson::Entity::find()
        .filter(scheduled_lesson::Column::OrgId.eq(&ctx.org_id))
        .filter(scheduled_lesson::Column::PeriodId.eq(period_id))
        .filter(scheduled_lesson::Column::RoomId.eq(room_id))
        .filter(scheduled_lesson::Column::Status.ne("cancelled"));
    query = apply_branch_filter(ctx, query).await?;
    Ok(query.order_by_asc(scheduled_lesson::Column::DayOfWeek).order_by_asc(scheduled_lesson::Column::TimeSlotId).all(&ctx.db).await?)
}

pub async fn get(ctx: &ServiceContext, lesson_id: &str) -> Result<scheduled_lesson::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::ScheduleRead).await?;
    let lesson = scheduled_lesson::Entity::find_by_id(lesson_id.to_string())
        .filter(scheduled_lesson::Column::OrgId.eq(&ctx.org_id))
        .one(&ctx.db).await?
        .ok_or_else(|| AppError::not_found("scheduled_lesson", lesson_id))?;
    svc_academic::ops::scope::ensure_class_scope(ctx, &lesson.class_id).await?;
    Ok(lesson)
}

 

async fn apply_branch_filter(
    ctx: &ServiceContext,
    query: sea_orm::Select<scheduled_lesson::Entity>,
) -> Result<sea_orm::Select<scheduled_lesson::Entity>, AppError> {
    if ctx.org_unit_id.is_some() {
        let class_ids = svc_academic::ops::scope::scoped_class_ids(ctx).await?;
        Ok(query.filter(scheduled_lesson::Column::ClassId.is_in(class_ids)))
    } else {
        Ok(query)
    }
}

#[derive(serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
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
    req: CreateLessonRequest,
) -> Result<scheduled_lesson::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::ScheduleWrite).await?;
    svc_academic::ops::scope::ensure_class_scope(ctx, &req.class_id).await?;

    validate_placement(ctx, &req.period_id, &req.class_id, &req.class_subject_id, &req.subject_id, &req.teacher_id, &req.time_slot_id, req.room_id.as_deref()).await?;
    let conflicts = check_conflicts_for_placement(
        ctx,
        &req.period_id,
        &req.class_id,
        &req.teacher_id,
        &req.time_slot_id,
        req.day_of_week,
        req.room_id.as_deref(),
        None,
    ).await?;
    if !conflicts.is_empty() {
        return Err(AppError::conflict(format!("Schedule conflict with lesson {}", conflicts[0].id)));
    }

    let now = Utc::now().to_rfc3339();
    let id = Uuid::new_v4().to_string();

    let model = scheduled_lesson::Model {
        id: id.clone(), org_id: ctx.org_id.clone(), period_id: req.period_id,
        class_id: req.class_id, class_subject_id: req.class_subject_id,
        subject_id: req.subject_id, teacher_id: req.teacher_id,
        substitute_teacher_id: None, time_slot_id: req.time_slot_id, day_of_week: req.day_of_week,
        room_id: req.room_id, status: "scheduled".into(),
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

#[derive(serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
pub struct UpdateLessonRequest {
    pub teacher_id: Option<String>,
    pub time_slot_id: Option<String>,
    pub day_of_week: Option<i32>,
    pub room_id: Option<String>,
    pub topic: Option<String>,
    pub status: Option<String>,
}

pub async fn update(
    ctx: &ServiceContext, lesson_id: &str,
    req: UpdateLessonRequest,
) -> Result<scheduled_lesson::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::ScheduleWrite).await?;

    let existing = get(ctx, lesson_id).await?;
    svc_academic::ops::scope::ensure_class_scope(ctx, &existing.class_id).await?;
    let teacher_id = req.teacher_id.as_deref().unwrap_or(&existing.teacher_id);
    let time_slot_id = req.time_slot_id.as_deref().unwrap_or(&existing.time_slot_id);
    let day_of_week = req.day_of_week.unwrap_or(existing.day_of_week);
    let room_id = req.room_id.as_deref().or(existing.room_id.as_deref());

    validate_placement(ctx, &existing.period_id, &existing.class_id, &existing.class_subject_id, &existing.subject_id, teacher_id, time_slot_id, room_id).await?;
    let conflicts = check_conflicts_for_placement(
        ctx,
        &existing.period_id,
        &existing.class_id,
        teacher_id,
        time_slot_id,
        day_of_week,
        room_id,
        Some(lesson_id),
    ).await?;
    if !conflicts.is_empty() {
        return Err(AppError::conflict(format!("Schedule conflict with lesson {}", conflicts[0].id)));
    }

    let mut active: scheduled_lesson::ActiveModel = existing.into();

    if let Some(tid) = req.teacher_id { active.teacher_id = Set(tid); }
    if let Some(slot_id) = req.time_slot_id { active.time_slot_id = Set(slot_id); }
    if let Some(day) = req.day_of_week { active.day_of_week = Set(day); }
    if let Some(rid) = req.room_id { active.room_id = Set(Some(rid)); }
    if let Some(t) = req.topic { active.topic = Set(Some(t)); }
    if let Some(s) = req.status { active.status = Set(s); }
    active.updated_at = Set(Utc::now().to_rfc3339());

    Ok(active.update(&ctx.db).await?)
}

pub async fn delete(ctx: &ServiceContext, lesson_id: &str) -> Result<(), AppError> {
    ctx.permissions.check(&ctx.actor, Permission::ScheduleDelete).await?;
    let existing = get(ctx, lesson_id).await?;
    let mut active: scheduled_lesson::ActiveModel = existing.into();
    active.status = Set("cancelled".into());
    active.updated_at = Set(Utc::now().to_rfc3339());
    active.update(&ctx.db).await?;
    Ok(())
}

async fn validate_placement(
    ctx: &ServiceContext,
    period_id: &str,
    class_id: &str,
    class_subject_id: &str,
    subject_id: &str,
    teacher_id: &str,
    time_slot_id: &str,
    room_id: Option<&str>,
) -> Result<(), AppError> {
    let class_record = class::Entity::find_by_id(class_id.to_string())
        .one(&ctx.db).await?
        .ok_or_else(|| AppError::not_found("class", class_id))?;
    if class_record.period_id != period_id {
        return Err(AppError::validation("Class does not belong to the selected period"));
    }

    let class_subject_record = class_subject::Entity::find_by_id(class_subject_id.to_string())
        .one(&ctx.db).await?
        .ok_or_else(|| AppError::not_found("class_subject", class_subject_id))?;
    if class_subject_record.period_id != period_id || class_subject_record.class_id != class_id {
        return Err(AppError::validation("Class subject does not belong to the selected class and period"));
    }
    if class_subject_record.subject_id != subject_id {
        return Err(AppError::validation("Subject does not match the class subject"));
    }
    if class_subject_record.primary_teacher_id != teacher_id
        && class_subject_record.assistant_teacher_id.as_deref() != Some(teacher_id)
        && class_subject_record.substitute_teacher_id.as_deref() != Some(teacher_id)
    {
        return Err(AppError::validation("Teacher is not assigned to the class subject"));
    }

    let subject_exists = subject::Entity::find_by_id(subject_id.to_string()).one(&ctx.db).await?.is_some();
    if !subject_exists {
        return Err(AppError::not_found("subject", subject_id));
    }

    let slot = time_slot::Entity::find_by_id(time_slot_id.to_string())
        .one(&ctx.db).await?
        .ok_or_else(|| AppError::not_found("time_slot", time_slot_id))?;
    if slot.period_id != period_id || slot.is_active == 0 {
        return Err(AppError::validation("Time slot does not belong to the selected period"));
    }

    if let Some(rid) = room_id {
        let room_record = room::Entity::find_by_id(rid.to_string()).one(&ctx.db).await?
            .ok_or_else(|| AppError::not_found("room", rid))?;
        if room_record.org_id != ctx.org_id || room_record.is_active == 0 {
            return Err(AppError::validation("Room is not active for this organization"));
        }
    }
    Ok(())
}

async fn check_conflicts_for_placement(
    ctx: &ServiceContext,
    period_id: &str,
    class_id: &str,
    teacher_id: &str,
    time_slot_id: &str,
    day_of_week: i32,
    room_id: Option<&str>,
    exclude_id: Option<&str>,
) -> Result<Vec<scheduled_lesson::Model>, AppError> {
    let mut condition = Condition::any()
        .add(scheduled_lesson::Column::TeacherId.eq(teacher_id))
        .add(scheduled_lesson::Column::ClassId.eq(class_id));
    if let Some(rid) = room_id {
        condition = condition.add(scheduled_lesson::Column::RoomId.eq(rid));
    }

    let mut query = scheduled_lesson::Entity::find()
        .filter(scheduled_lesson::Column::OrgId.eq(&ctx.org_id))
        .filter(scheduled_lesson::Column::PeriodId.eq(period_id))
        .filter(scheduled_lesson::Column::TimeSlotId.eq(time_slot_id))
        .filter(scheduled_lesson::Column::DayOfWeek.eq(day_of_week))
        .filter(scheduled_lesson::Column::Status.ne("cancelled"))
        .filter(condition);
    if let Some(id) = exclude_id {
        query = query.filter(scheduled_lesson::Column::Id.ne(id));
    }
    Ok(query.all(&ctx.db).await?)
}

/// Check for scheduling conflicts (same teacher or room at same time/day).
pub async fn check_conflicts(
    ctx: &ServiceContext, period_id: &str, class_id: &str, teacher_id: &str,
    time_slot_id: &str, day_of_week: i32, room_id: Option<String>,
) -> Result<Vec<scheduled_lesson::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::ScheduleRead).await?;

    let mut condition = Condition::any()
        .add(scheduled_lesson::Column::TeacherId.eq(teacher_id))
        .add(scheduled_lesson::Column::ClassId.eq(class_id));

    if let Some(rid) = room_id {
        condition = condition.add(scheduled_lesson::Column::RoomId.eq(rid));
    }

    Ok(scheduled_lesson::Entity::find()
        .filter(scheduled_lesson::Column::OrgId.eq(&ctx.org_id))
        .filter(scheduled_lesson::Column::PeriodId.eq(period_id))
        .filter(scheduled_lesson::Column::TimeSlotId.eq(time_slot_id))
        .filter(scheduled_lesson::Column::DayOfWeek.eq(day_of_week))
        .filter(scheduled_lesson::Column::Status.ne("cancelled"))
        .filter(condition)
        .all(&ctx.db).await?)
}
