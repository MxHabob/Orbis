//! Scheduling command repository — write operations.

use crate::entities::{time_slot, scheduled_lesson, room, solve_job};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DbErr, EntityTrait};
use super::SchedulingCommandRepo;

impl SchedulingCommandRepo {
    // ── Time Slots ──────────────────────────────────────────────

    pub async fn create_time_slot(&self, slot: time_slot::Model) -> Result<time_slot::Model, DbErr> {
        let active = time_slot::ActiveModel {
            id: Set(slot.id),
            org_id: Set(slot.org_id),
            cycle_id: Set(slot.cycle_id),
            slot_number: Set(slot.slot_number),
            name: Set(slot.name),
            name_ar: Set(slot.name_ar),
            start_time: Set(slot.start_time),
            end_time: Set(slot.end_time),
            duration_minutes: Set(slot.duration_minutes),
            is_break: Set(slot.is_break),
            break_type: Set(slot.break_type),
            applicable_days: Set(slot.applicable_days),
            sort_order: Set(slot.sort_order),
            is_active: Set(slot.is_active),
            created_at: Set(slot.created_at),
            updated_at: Set(slot.updated_at),
        };
        active.insert(&self.db).await
    }

    // ── Scheduled Lessons ────────────────────────────────────────

    pub async fn create_scheduled_lesson(&self, lesson: scheduled_lesson::Model) -> Result<scheduled_lesson::Model, DbErr> {
        let active = scheduled_lesson::ActiveModel {
            id: Set(lesson.id),
            org_id: Set(lesson.org_id),
            cycle_id: Set(lesson.cycle_id),
            class_id: Set(lesson.class_id),
            class_subject_id: Set(lesson.class_subject_id),
            subject_id: Set(lesson.subject_id),
            teacher_id: Set(lesson.teacher_id),
            substitute_teacher_id: Set(lesson.substitute_teacher_id),
            time_slot_id: Set(lesson.time_slot_id),
            day_of_week: Set(lesson.day_of_week),
            room_id: Set(lesson.room_id),
            room_name: Set(lesson.room_name),
            status: Set(lesson.status),
            is_substituted: Set(lesson.is_substituted),
            substitution_reason: Set(lesson.substitution_reason),
            topic: Set(lesson.topic),
            topic_ar: Set(lesson.topic_ar),
            objectives: Set(lesson.objectives),
            materials: Set(lesson.materials),
            homework: Set(lesson.homework),
            homework_due_date: Set(lesson.homework_due_date),
            attendance_taken: Set(lesson.attendance_taken),
            attendance_taken_at: Set(lesson.attendance_taken_at),
            attendance_taken_by: Set(lesson.attendance_taken_by),
            attendance_method: Set(lesson.attendance_method),
            notes: Set(lesson.notes),
            cancellation_reason: Set(lesson.cancellation_reason),
            created_at: Set(lesson.created_at),
            updated_at: Set(lesson.updated_at),
        };
        active.insert(&self.db).await
    }

    pub async fn update_scheduled_lesson(&self, active: scheduled_lesson::ActiveModel) -> Result<scheduled_lesson::Model, DbErr> {
        active.update(&self.db).await
    }

    // ── Rooms ───────────────────────────────────────────────────

    pub async fn create_room(&self, model: room::Model) -> Result<room::Model, DbErr> {
        let active = room::ActiveModel {
            id: Set(model.id),
            org_id: Set(model.org_id),
            name: Set(model.name),
            name_ar: Set(model.name_ar),
            capacity: Set(model.capacity),
            room_type: Set(model.room_type),
            building: Set(model.building),
            floor: Set(model.floor),
            is_active: Set(model.is_active),
            created_at: Set(model.created_at),
            updated_at: Set(model.updated_at),
        };
        active.insert(&self.db).await
    }

    pub async fn update_room(&self, active: room::ActiveModel) -> Result<room::Model, DbErr> {
        active.update(&self.db).await
    }

    // ── Solve Jobs ──────────────────────────────────────────────

    pub async fn create_solve_job(&self, job: solve_job::Model) -> Result<solve_job::Model, DbErr> {
        let active = solve_job::ActiveModel {
            id: Set(job.id),
            org_id: Set(job.org_id),
            job_type: Set(job.job_type),
            target_id: Set(job.target_id),
            mode: Set(job.mode),
            status: Set(job.status),
            problem_snapshot: Set(job.problem_snapshot),
            best_solution: Set(job.best_solution),
            score: Set(job.score),
            items_created: Set(job.items_created),
            error_message: Set(job.error_message),
            created_by: Set(job.created_by),
            started_at: Set(job.started_at),
            completed_at: Set(job.completed_at),
            created_at: Set(job.created_at),
            updated_at: Set(job.updated_at),
        };
        active.insert(&self.db).await
    }

    pub async fn update_solve_job(&self, job: solve_job::Model) -> Result<solve_job::Model, DbErr> {
        let active = solve_job::ActiveModel {
            id: Set(job.id),
            org_id: Set(job.org_id),
            job_type: Set(job.job_type),
            target_id: Set(job.target_id),
            mode: Set(job.mode),
            status: Set(job.status),
            problem_snapshot: Set(job.problem_snapshot),
            best_solution: Set(job.best_solution),
            score: Set(job.score),
            items_created: Set(job.items_created),
            error_message: Set(job.error_message),
            created_by: Set(job.created_by),
            started_at: Set(job.started_at),
            completed_at: Set(job.completed_at),
            created_at: Set(job.created_at),
            updated_at: Set(chrono::Utc::now().to_rfc3339()),
        };
        active.update(&self.db).await
    }

    pub async fn cancel_lessons_for_cycle(
        &self,
        org_id: &str,
        cycle_id: &str,
    ) -> Result<u64, DbErr> {
        use sea_orm::{ColumnTrait, QueryFilter, sea_query::Expr};
        let now = chrono::Utc::now().to_rfc3339();
        let result = scheduled_lesson::Entity::update_many()
            .col_expr(scheduled_lesson::Column::Status, Expr::value("cancelled"))
            .col_expr(scheduled_lesson::Column::UpdatedAt, Expr::value(now))
            .filter(scheduled_lesson::Column::OrgId.eq(org_id))
            .filter(scheduled_lesson::Column::CycleId.eq(cycle_id))
            .filter(scheduled_lesson::Column::Status.ne("cancelled"))
            .exec(&self.db)
            .await?;
        Ok(result.rows_affected)
    }
}

