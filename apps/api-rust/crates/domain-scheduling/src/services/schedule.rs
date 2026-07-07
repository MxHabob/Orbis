//! Schedule service — manages slots, lessons, rooms, availability, and conflicts.

use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, EntityTrait, QueryFilter, ColumnTrait};

use shared_kernel::errors::AppError;
use crate::entities::{time_slot, scheduled_lesson, room, teacher_availability};
use crate::params::{ScheduleParams, RoomParams};
use crate::repository::{SchedulingCommandRepo, SchedulingQueryRepo};

use domain_academic::entities::class_subject;

#[derive(Serialize, Deserialize, utoipa::ToSchema, Clone, Debug)]
pub struct CreateTimeSlotRequest {
    pub cycle_id: String,
    pub slot_number: i32,
    pub name: Option<String>,
    pub name_ar: Option<String>,
    pub start_time: String,
    pub end_time: String,
    pub duration_minutes: i32,
    pub is_break: Option<i32>,
    pub break_type: Option<String>,
    pub applicable_days: Option<String>,
    pub sort_order: Option<i32>,
}

#[derive(Serialize, Deserialize, utoipa::ToSchema, Clone, Debug)]
pub struct UpdateTimeSlotRequest {
    pub name: Option<String>,
    pub name_ar: Option<String>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub duration_minutes: Option<i32>,
    pub is_break: Option<i32>,
    pub break_type: Option<String>,
    pub applicable_days: Option<String>,
    pub sort_order: Option<i32>,
    pub is_active: Option<i32>,
}

#[derive(Serialize, Deserialize, utoipa::ToSchema, Clone, Debug)]
pub struct CreateLessonRequest {
    pub cycle_id: String,
    pub class_id: String,
    pub class_subject_id: Option<String>,
    pub subject_id: String,
    pub teacher_id: String,
    pub time_slot_id: String,
    pub day_of_week: i32,
    pub room_id: Option<String>,
    pub topic: Option<String>,
    pub topic_ar: Option<String>,
    pub homework: Option<String>,
    pub notes: Option<String>,
}

#[derive(Serialize, Deserialize, utoipa::ToSchema, Clone, Debug)]
pub struct UpdateLessonRequest {
    pub teacher_id: Option<String>,
    pub time_slot_id: Option<String>,
    pub day_of_week: Option<i32>,
    pub room_id: Option<String>,
    pub topic: Option<String>,
    pub topic_ar: Option<String>,
    pub homework: Option<String>,
    pub notes: Option<String>,
    pub status: Option<String>,
    pub substitute_teacher_id: Option<String>,
    pub substitution_reason: Option<String>,
}

#[derive(Serialize, Deserialize, utoipa::ToSchema, Clone, Debug)]
pub struct ScheduleConflict {
    pub lesson_id: String,
    pub conflict_type: String,
    pub message: String,
}

#[derive(Serialize, Deserialize, utoipa::ToSchema, Clone, Debug)]
pub struct ScheduledLessonView {
    pub id: String,
    pub org_id: String,
    pub cycle_id: String,
    pub class_id: String,
    pub class_subject_id: String,
    pub subject_id: String,
    pub teacher_id: String,
    pub substitute_teacher_id: Option<String>,
    pub time_slot_id: String,
    pub day_of_week: i32,
    pub room_id: Option<String>,
    pub room_name: Option<String>,
    pub status: String,
    pub is_substituted: i32,
    pub substitution_reason: Option<String>,
    pub topic: Option<String>,
    pub topic_ar: Option<String>,
    pub homework: Option<String>,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl ScheduledLessonView {
    pub fn from_model(lesson: scheduled_lesson::Model, room_name: Option<String>) -> Self {
        Self {
            id: lesson.id,
            org_id: lesson.org_id,
            cycle_id: lesson.cycle_id,
            class_id: lesson.class_id,
            class_subject_id: lesson.class_subject_id,
            subject_id: lesson.subject_id,
            teacher_id: lesson.teacher_id,
            substitute_teacher_id: lesson.substitute_teacher_id,
            time_slot_id: lesson.time_slot_id,
            day_of_week: lesson.day_of_week,
            room_id: lesson.room_id,
            room_name,
            status: lesson.status,
            is_substituted: lesson.is_substituted,
            substitution_reason: lesson.substitution_reason,
            topic: lesson.topic,
            topic_ar: lesson.topic_ar,
            homework: lesson.homework,
            notes: lesson.notes,
            created_at: lesson.created_at,
            updated_at: lesson.updated_at,
        }
    }
}

#[derive(Clone, Debug)]
pub struct LessonPlacement {
    pub cycle_id: String,
    pub class_id: String,
    pub class_subject_id: String,
    pub subject_id: String,
    pub teacher_id: String,
    pub time_slot_id: String,
    pub day_of_week: i32,
    pub room_id: Option<String>,
    pub topic: Option<String>,
    pub topic_ar: Option<String>,
    pub homework: Option<String>,
    pub notes: Option<String>,
}

impl LessonPlacement {
    pub fn from_create_request(req: CreateLessonRequest, class_subject_id: String) -> Self {
        Self {
            cycle_id: req.cycle_id,
            class_id: req.class_id,
            class_subject_id,
            subject_id: req.subject_id,
            teacher_id: req.teacher_id,
            time_slot_id: req.time_slot_id,
            day_of_week: req.day_of_week,
            room_id: req.room_id,
            topic: req.topic,
            topic_ar: req.topic_ar,
            homework: req.homework,
            notes: req.notes,
        }
    }
}

pub struct ScheduleService {
    pub cmd: Arc<SchedulingCommandRepo>,
    pub query: Arc<SchedulingQueryRepo>,
}

impl ScheduleService {
    pub fn new(cmd: Arc<SchedulingCommandRepo>, query: Arc<SchedulingQueryRepo>) -> Self {
        Self { cmd, query }
    }

    async fn to_lesson_views(
        &self,
        lessons: Vec<scheduled_lesson::Model>,
    ) -> Result<Vec<ScheduledLessonView>, AppError> {
        let mut room_names = std::collections::HashMap::new();
        for lesson in &lessons {
            if let Some(rid) = &lesson.room_id {
                if !room_names.contains_key(rid) {
                    if let Some(room) = self.query.get_room_by_id(rid).await? {
                        room_names.insert(rid.clone(), room.name);
                    }
                }
            }
        }

        Ok(lessons
            .into_iter()
            .map(|l| {
                let room_name = l.room_id.as_ref().and_then(|id| room_names.get(id)).cloned();
                ScheduledLessonView::from_model(l, room_name)
            })
            .collect())
    }

    async fn to_lesson_view(&self, lesson: scheduled_lesson::Model) -> Result<ScheduledLessonView, AppError> {
        let views = self.to_lesson_views(vec![lesson]).await?;
        views.into_iter().next().ok_or_else(|| AppError::internal("Could not format lesson view"))
    }

    // ── Time Slots ──────────────────────────────────────────────

    pub async fn get_time_slots(
        &self,
        org_id: &str,
        cycle_id: &str,
    ) -> Result<Vec<time_slot::Model>, AppError> {
        Ok(self.query.get_time_slots(org_id, cycle_id).await?)
    }

    pub async fn create_time_slot(
        &self,
        org_id: &str,
        req: CreateTimeSlotRequest,
    ) -> Result<time_slot::Model, AppError> {
        let now = Utc::now().to_rfc3339();
        let slot = time_slot::Model {
            id: Uuid::new_v4().to_string(),
            org_id: org_id.to_string(),
            cycle_id: req.cycle_id,
            slot_number: req.slot_number,
            name: req.name,
            name_ar: req.name_ar,
            start_time: req.start_time,
            end_time: req.end_time,
            duration_minutes: req.duration_minutes,
            is_break: req.is_break.unwrap_or(0),
            break_type: req.break_type,
            applicable_days: req.applicable_days,
            sort_order: req.sort_order,
            is_active: 1,
            created_at: now.clone(),
            updated_at: now,
        };
        Ok(self.cmd.create_time_slot(slot).await?)
    }

    pub async fn update_time_slot(
        &self,
        id: &str,
        req: UpdateTimeSlotRequest,
    ) -> Result<time_slot::Model, AppError> {
        use sea_orm::ActiveValue::Set;

        let slot = self.query.get_time_slot_by_id(id).await?
            .ok_or_else(|| AppError::not_found("time_slot", id))?;

        let mut active: time_slot::ActiveModel = slot.into();
        if let Some(v) = req.name { active.name = Set(Some(v)); }
        if let Some(v) = req.name_ar { active.name_ar = Set(Some(v)); }
        if let Some(v) = req.start_time { active.start_time = Set(v); }
        if let Some(v) = req.end_time { active.end_time = Set(v); }
        if let Some(v) = req.duration_minutes { active.duration_minutes = Set(v); }
        if let Some(v) = req.is_break { active.is_break = Set(v); }
        if let Some(v) = req.break_type { active.break_type = Set(Some(v)); }
        if let Some(v) = req.applicable_days { active.applicable_days = Set(Some(v)); }
        if let Some(v) = req.sort_order { active.sort_order = Set(Some(v)); }
        if let Some(v) = req.is_active { active.is_active = Set(v); }
        active.updated_at = Set(Utc::now().to_rfc3339());

        Ok(active.update(&self.query.db).await.map_err(AppError::from)?)
    }

    pub async fn delete_time_slot(&self, id: &str) -> Result<(), AppError> {
        use sea_orm::ActiveValue::Set;

        let slot = self.query.get_time_slot_by_id(id).await?
            .ok_or_else(|| AppError::not_found("time_slot", id))?;

        let mut active: time_slot::ActiveModel = slot.into();
        active.is_active = Set(0);
        active.updated_at = Set(Utc::now().to_rfc3339());
        active.update(&self.query.db).await?;
        Ok(())
    }

    // ── Scheduled Lessons ────────────────────────────────────────

    pub async fn get_class_schedule(
        &self,
        org_id: &str,
        cycle_id: &str,
        class_id: &str,
    ) -> Result<Vec<ScheduledLessonView>, AppError> {
        let params = ScheduleParams {
            cycle_id: Some(cycle_id.to_string()),
            class_id: Some(class_id.to_string()),
            teacher_id: None,
        };
        let lessons = self.query.get_scheduled_lessons(org_id, params).await?;
        self.to_lesson_views(lessons).await
    }

    pub async fn get_teacher_schedule(
        &self,
        org_id: &str,
        cycle_id: &str,
        teacher_id: &str,
    ) -> Result<Vec<ScheduledLessonView>, AppError> {
        let params = ScheduleParams {
            cycle_id: Some(cycle_id.to_string()),
            class_id: None,
            teacher_id: Some(teacher_id.to_string()),
        };
        let lessons = self.query.get_scheduled_lessons(org_id, params).await?;
        self.to_lesson_views(lessons).await
    }

    pub async fn get_lesson_by_id(&self, id: &str) -> Result<ScheduledLessonView, AppError> {
        let lesson = self.query.get_scheduled_lesson_by_id(id).await?
            .ok_or_else(|| AppError::not_found("scheduled_lesson", id))?;
        self.to_lesson_view(lesson).await
    }

    pub async fn check_schedule_conflicts(
        &self,
        org_id: &str,
        cycle_id: &str,
        time_slot_id: &str,
        day_of_week: i32,
        exclude_id: Option<&str>,
        teacher_id: &str,
        class_id: &str,
        room_id: Option<&str>,
    ) -> Result<Vec<ScheduleConflict>, AppError> {
        let conflicts = self.query.find_lesson_conflicts(
            org_id, cycle_id, time_slot_id, day_of_week, exclude_id,
            teacher_id, class_id, room_id,
        ).await?;

        let mapped = conflicts.into_iter().map(|c| {
            let conflict_type = if c.teacher_id == teacher_id {
                "teacher"
            } else if c.class_id == class_id {
                "class"
            } else {
                "room"
            };
            ScheduleConflict {
                lesson_id: c.id.clone(),
                conflict_type: conflict_type.to_string(),
                message: format!("Conflict with lesson {} ({})", c.id, conflict_type),
            }
        }).collect();

        Ok(mapped)
    }

    pub async fn create_lesson(
        &self,
        org_id: &str,
        req: CreateLessonRequest,
    ) -> Result<ScheduledLessonView, AppError> {
        let class_subject_id = match &req.class_subject_id {
            Some(id) => id.clone(),
            None => {
                let cs = class_subject::Entity::find()
                    .filter(class_subject::Column::ClassId.eq(&req.class_id))
                    .filter(class_subject::Column::SubjectId.eq(&req.subject_id))
                    .filter(class_subject::Column::CycleId.eq(&req.cycle_id))
                    .one(&self.query.db)
                    .await?
                    .ok_or_else(|| AppError::not_found("class_subject", "combination"))?;
                cs.id
            }
        };

        let placement = LessonPlacement::from_create_request(req, class_subject_id);
        let lesson = self.persist_lesson(org_id, placement, None).await?;
        self.to_lesson_view(lesson).await
    }

    pub async fn update_lesson(
        &self,
        org_id: &str,
        id: &str,
        req: UpdateLessonRequest,
    ) -> Result<ScheduledLessonView, AppError> {
        use sea_orm::ActiveValue::Set;

        let lesson = self.query.get_scheduled_lesson_by_id(id).await?
            .ok_or_else(|| AppError::not_found("scheduled_lesson", id))?;

        let teacher_id = req.teacher_id.as_ref().unwrap_or(&lesson.teacher_id);
        let time_slot_id = req.time_slot_id.as_ref().unwrap_or(&lesson.time_slot_id);
        let day_of_week = req.day_of_week.unwrap_or(lesson.day_of_week);
        let room_id = req.room_id.as_ref().or(lesson.room_id.as_ref());

        let conflicts = self.check_schedule_conflicts(
            org_id, &lesson.cycle_id, time_slot_id, day_of_week, Some(id),
            teacher_id, &lesson.class_id, room_id.map(|s| s.as_str()),
        ).await?;
        if !conflicts.is_empty() {
            return Err(AppError::conflict(format!("Schedule conflict: {}", conflicts[0].message)));
        }

        if let Some(rid) = &req.room_id {
            let exists = self.query.get_room_by_id(rid).await?.is_some();
            if !exists {
                return Err(AppError::not_found("room", rid));
            }
        }

        let mut active: scheduled_lesson::ActiveModel = lesson.into();
        if let Some(v) = req.teacher_id { active.teacher_id = Set(v); }
        if let Some(v) = req.time_slot_id { active.time_slot_id = Set(v); }
        if let Some(v) = req.day_of_week { active.day_of_week = Set(v); }
        if let Some(v) = req.room_id { active.room_id = Set(Some(v)); }
        if let Some(v) = req.topic { active.topic = Set(Some(v)); }
        if let Some(v) = req.topic_ar { active.topic_ar = Set(Some(v)); }
        if let Some(v) = req.homework { active.homework = Set(Some(v)); }
        if let Some(v) = req.notes { active.notes = Set(Some(v)); }
        if let Some(v) = req.status { active.status = Set(v); }
        if let Some(v) = req.substitute_teacher_id { active.substitute_teacher_id = Set(Some(v)); }
        if let Some(v) = req.substitution_reason { active.substitution_reason = Set(Some(v)); }
        active.updated_at = Set(Utc::now().to_rfc3339());

        let updated = self.cmd.update_scheduled_lesson(active).await?;
        self.to_lesson_view(updated).await
    }

    pub async fn delete_lesson(&self, id: &str) -> Result<(), AppError> {
        use sea_orm::ActiveValue::Set;

        let lesson = self.query.get_scheduled_lesson_by_id(id).await?
            .ok_or_else(|| AppError::not_found("scheduled_lesson", id))?;

        let mut active: scheduled_lesson::ActiveModel = lesson.into();
        active.status = Set("cancelled".to_string());
        active.updated_at = Set(Utc::now().to_rfc3339());
        self.cmd.update_scheduled_lesson(active).await?;
        Ok(())
    }

    pub(crate) async fn persist_lesson(
        &self,
        org_id: &str,
        placement: LessonPlacement,
        exclude_id: Option<&str>,
    ) -> Result<scheduled_lesson::Model, AppError> {
        let conflicts = self.check_schedule_conflicts(
            org_id,
            &placement.cycle_id,
            &placement.time_slot_id,
            placement.day_of_week,
            exclude_id,
            &placement.teacher_id,
            &placement.class_id,
            placement.room_id.as_deref(),
        ).await?;

        if !conflicts.is_empty() {
            return Err(AppError::conflict(format!("Schedule conflict: {}", conflicts[0].message)));
        }

        if let Some(ref rid) = placement.room_id {
            let exists = self.query.get_room_by_id(rid).await?.is_some();
            if !exists {
                return Err(AppError::not_found("room", rid));
            }
        }

        let now = Utc::now().to_rfc3339();
        let lesson = scheduled_lesson::Model {
            id: Uuid::new_v4().to_string(),
            org_id: org_id.to_string(),
            cycle_id: placement.cycle_id,
            class_id: placement.class_id,
            class_subject_id: placement.class_subject_id,
            subject_id: placement.subject_id,
            teacher_id: placement.teacher_id,
            substitute_teacher_id: None,
            time_slot_id: placement.time_slot_id,
            day_of_week: placement.day_of_week,
            room_id: placement.room_id,
            room_name: None,
            status: "scheduled".to_string(),
            is_substituted: 0,
            substitution_reason: None,
            topic: placement.topic,
            topic_ar: placement.topic_ar,
            objectives: None,
            materials: None,
            homework: placement.homework,
            homework_due_date: None,
            attendance_taken: 0,
            attendance_taken_at: None,
            attendance_taken_by: None,
            attendance_method: None,
            notes: placement.notes,
            cancellation_reason: None,
            created_at: now.clone(),
            updated_at: now,
        };

        Ok(self.cmd.create_scheduled_lesson(lesson).await?)
    }
}
