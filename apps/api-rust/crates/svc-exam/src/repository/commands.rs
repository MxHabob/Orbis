//! Exam command repository — write operations.

use crate::entities::{
    exam_session, exam_subject, exam_schedule,
    seating_number, secret_number,
    exam_hall, exam_hall_allocation,
    exam_proctor, student_ordering,
};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DbErr, EntityTrait, ColumnTrait, QueryFilter, sea_query::Expr};
use super::ExamCommandRepo;

impl ExamCommandRepo {
    // ── Student Ordering ──

    pub async fn upsert_student_ordering(&self, model: student_ordering::Model) -> Result<student_ordering::Model, DbErr> {
        let existing = student_ordering::Entity::find()
            .filter(student_ordering::Column::EnrollmentId.eq(&model.enrollment_id))
            .one(&self.db)
            .await?;

        let active = student_ordering::ActiveModel {
            id: Set(model.id),
            org_id: Set(model.org_id),
            period_id: Set(model.period_id),
            enrollment_id: Set(model.enrollment_id),
            student_id: Set(model.student_id),
            class_id: Set(model.class_id),
            sort_key: Set(model.sort_key),
            order_in_class: Set(model.order_in_class),
            order_in_division: Set(model.order_in_division),
            order_in_level: Set(model.order_in_level),
            order_in_org: Set(model.order_in_org),
            level: Set(model.level),
            division: Set(model.division),
            student_name_ar: Set(model.student_name_ar),
            student_sex: Set(model.student_sex),
            academic_number: Set(model.academic_number),
            is_frozen: Set(model.is_frozen),
            frozen_at: Set(model.frozen_at),
            frozen_by: Set(model.frozen_by),
            strategy: Set(model.strategy),
            version: Set(model.version),
            created_at: Set(model.created_at),
            updated_at: Set(model.updated_at),
        };

        if existing.is_some() {
            active.update(&self.db).await
        } else {
            active.insert(&self.db).await
        }
    }

    pub async fn delete_student_ordering_for_period(&self, org_id: &str, period_id: &str) -> Result<u64, DbErr> {
        let result = student_ordering::Entity::delete_many()
            .filter(student_ordering::Column::OrgId.eq(org_id))
            .filter(student_ordering::Column::PeriodId.eq(period_id))
            .filter(student_ordering::Column::IsFrozen.eq(0))
            .exec(&self.db)
            .await?;
        Ok(result.rows_affected)
    }

    pub async fn freeze_student_ordering(&self, org_id: &str, period_id: &str, frozen_by: &str) -> Result<u64, DbErr> {
        let now = chrono::Utc::now().to_rfc3339();
        let result = student_ordering::Entity::update_many()
            .filter(student_ordering::Column::OrgId.eq(org_id))
            .filter(student_ordering::Column::PeriodId.eq(period_id))
            .col_expr(student_ordering::Column::IsFrozen, Expr::value(1))
            .col_expr(student_ordering::Column::FrozenAt, Expr::value(now.clone()))
            .col_expr(student_ordering::Column::FrozenBy, Expr::value(frozen_by))
            .col_expr(student_ordering::Column::UpdatedAt, Expr::value(now))
            .exec(&self.db)
            .await?;
        Ok(result.rows_affected)
    }

    // ── Exam Sessions ──

    pub async fn insert_exam_session(&self, model: exam_session::Model) -> Result<exam_session::Model, DbErr> {
        let active = exam_session::ActiveModel {
            id: Set(model.id),
            org_id: Set(model.org_id),
            period_id: Set(model.period_id),
            name: Set(model.name),
            name_ar: Set(model.name_ar),
            code: Set(model.code),
            session_type: Set(model.session_type),
            status: Set(model.status),
            start_date: Set(model.start_date),
            end_date: Set(model.end_date),
            hijri_start_date: Set(model.hijri_start_date),
            hijri_end_date: Set(model.hijri_end_date),
            applicable_levels: Set(model.applicable_levels),
            applicable_class_ids: Set(model.applicable_class_ids),
            settings: Set(model.settings),
            grading_deadline: Set(model.grading_deadline),
            results_publish_date: Set(model.results_publish_date),
            is_results_published: Set(model.is_results_published),
            created_by: Set(model.created_by),
            approved_by: Set(model.approved_by),
            approved_at: Set(model.approved_at),
            created_at: Set(model.created_at),
            updated_at: Set(model.updated_at),
        };
        active.insert(&self.db).await
    }

    pub async fn update_exam_session(&self, model: exam_session::Model) -> Result<exam_session::Model, DbErr> {
        let active = exam_session::ActiveModel {
            id: Set(model.id),
            org_id: Set(model.org_id),
            period_id: Set(model.period_id),
            name: Set(model.name),
            name_ar: Set(model.name_ar),
            code: Set(model.code),
            session_type: Set(model.session_type),
            status: Set(model.status),
            start_date: Set(model.start_date),
            end_date: Set(model.end_date),
            hijri_start_date: Set(model.hijri_start_date),
            hijri_end_date: Set(model.hijri_end_date),
            applicable_levels: Set(model.applicable_levels),
            applicable_class_ids: Set(model.applicable_class_ids),
            settings: Set(model.settings),
            grading_deadline: Set(model.grading_deadline),
            results_publish_date: Set(model.results_publish_date),
            is_results_published: Set(model.is_results_published),
            created_by: Set(model.created_by),
            approved_by: Set(model.approved_by),
            approved_at: Set(model.approved_at),
            created_at: Set(model.created_at),
            updated_at: Set(model.updated_at),
        };
        active.update(&self.db).await
    }

    // ── Exam Subjects ──

    pub async fn insert_exam_subject(&self, model: exam_subject::Model) -> Result<exam_subject::Model, DbErr> {
        let active = exam_subject::ActiveModel {
            id: Set(model.id),
            exam_session_id: Set(model.exam_session_id),
            subject_id: Set(model.subject_id),
            class_subject_id: Set(model.class_subject_id),
            max_score: Set(model.max_score),
            passing_score: Set(model.passing_score),
            duration_minutes: Set(model.duration_minutes),
            exam_type: Set(model.exam_type),
            is_included: Set(model.is_included),
            weight: Set(model.weight),
            instructions: Set(model.instructions),
            instructions_ar: Set(model.instructions_ar),
            sort_order: Set(model.sort_order),
            created_at: Set(model.created_at),
            updated_at: Set(model.updated_at),
        };
        active.insert(&self.db).await
    }

    // ── Exam Schedules ──

    pub async fn insert_exam_schedule(&self, model: exam_schedule::Model) -> Result<exam_schedule::Model, DbErr> {
        let active = exam_schedule::ActiveModel {
            id: Set(model.id),
            exam_session_id: Set(model.exam_session_id),
            exam_subject_id: Set(model.exam_subject_id),
            exam_date: Set(model.exam_date),
            start_time: Set(model.start_time),
            end_time: Set(model.end_time),
            period: Set(model.period),
            applicable_class_ids: Set(model.applicable_class_ids),
            room_ids: Set(model.room_ids),
            status: Set(model.status),
            postponed_to: Set(model.postponed_to),
            postponement_reason: Set(model.postponement_reason),
            notes: Set(model.notes),
            created_at: Set(model.created_at),
            updated_at: Set(model.updated_at),
        };
        active.insert(&self.db).await
    }

    pub async fn update_exam_schedule(&self, model: exam_schedule::Model) -> Result<exam_schedule::Model, DbErr> {
        let active = exam_schedule::ActiveModel {
            id: Set(model.id),
            exam_session_id: Set(model.exam_session_id),
            exam_subject_id: Set(model.exam_subject_id),
            exam_date: Set(model.exam_date),
            start_time: Set(model.start_time),
            end_time: Set(model.end_time),
            period: Set(model.period),
            applicable_class_ids: Set(model.applicable_class_ids),
            room_ids: Set(model.room_ids),
            status: Set(model.status),
            postponed_to: Set(model.postponed_to),
            postponement_reason: Set(model.postponement_reason),
            notes: Set(model.notes),
            created_at: Set(model.created_at),
            updated_at: Set(model.updated_at),
        };
        active.update(&self.db).await
    }

    pub async fn delete_exam_schedule(&self, id: &str) -> Result<u64, DbErr> {
        exam_schedule::Entity::delete_by_id(id.to_string())
            .exec(&self.db)
            .await
            .map(|res| res.rows_affected)
    }

    pub async fn delete_exam_schedules_for_session(&self, exam_session_id: &str) -> Result<u64, DbErr> {
        exam_schedule::Entity::delete_many()
            .filter(exam_schedule::Column::ExamSessionId.eq(exam_session_id))
            .exec(&self.db)
            .await
            .map(|res| res.rows_affected)
    }

    // ── Seating Numbers ──

    pub async fn insert_seating_number(&self, model: seating_number::Model) -> Result<seating_number::Model, DbErr> {
        let active = seating_number::ActiveModel {
            id: Set(model.id),
            org_id: Set(model.org_id),
            exam_session_id: Set(model.exam_session_id),
            enrollment_id: Set(model.enrollment_id),
            student_id: Set(model.student_id),
            class_id: Set(model.class_id),
            seating_number: Set(model.seating_number),
            sequence_number: Set(model.sequence_number),
            is_active: Set(model.is_active),
            generated_at: Set(model.generated_at),
            generated_by: Set(model.generated_by),
            voided_at: Set(model.voided_at),
            voided_by: Set(model.voided_by),
            void_reason: Set(model.void_reason),
            created_at: Set(model.created_at),
            updated_at: Set(model.updated_at),
        };
        active.insert(&self.db).await
    }

    pub async fn deactivate_seating_numbers(&self, exam_session_id: &str) -> Result<u64, DbErr> {
        let now = chrono::Utc::now().to_rfc3339();
        let result = seating_number::Entity::update_many()
            .filter(seating_number::Column::ExamSessionId.eq(exam_session_id))
            .col_expr(seating_number::Column::IsActive, Expr::value(0))
            .col_expr(seating_number::Column::UpdatedAt, Expr::value(now))
            .exec(&self.db)
            .await?;
        Ok(result.rows_affected)
    }

    // ── Secret Numbers ──

    pub async fn insert_secret_number(&self, model: secret_number::Model) -> Result<secret_number::Model, DbErr> {
        let active = secret_number::ActiveModel {
            id: Set(model.id),
            org_id: Set(model.org_id),
            exam_session_id: Set(model.exam_session_id),
            enrollment_id: Set(model.enrollment_id),
            student_id: Set(model.student_id),
            secret_number: Set(model.secret_number),
            secret_hash: Set(model.secret_hash),
            barcode_data: Set(model.barcode_data),
            qr_data: Set(model.qr_data),
            is_active: Set(model.is_active),
            is_revealed: Set(model.is_revealed),
            revealed_at: Set(model.revealed_at),
            revealed_by: Set(model.revealed_by),
            generated_at: Set(model.generated_at),
            generated_by: Set(model.generated_by),
            generation_batch: Set(model.generation_batch),
            previous_secret_id: Set(model.previous_secret_id),
            created_at: Set(model.created_at),
            updated_at: Set(model.updated_at),
        };
        active.insert(&self.db).await
    }

    pub async fn reveal_secret_numbers(&self, exam_session_id: &str, revealed_by: &str) -> Result<u64, DbErr> {
        let now = chrono::Utc::now().to_rfc3339();
        let result = secret_number::Entity::update_many()
            .filter(secret_number::Column::ExamSessionId.eq(exam_session_id))
            .filter(secret_number::Column::IsActive.eq(1))
            .col_expr(secret_number::Column::IsRevealed, Expr::value(1))
            .col_expr(secret_number::Column::RevealedAt, Expr::value(now.clone()))
            .col_expr(secret_number::Column::RevealedBy, Expr::value(revealed_by))
            .col_expr(secret_number::Column::UpdatedAt, Expr::value(now))
            .exec(&self.db)
            .await?;
        Ok(result.rows_affected)
    }

    pub async fn deactivate_secret_numbers(&self, exam_session_id: &str) -> Result<u64, DbErr> {
        let now = chrono::Utc::now().to_rfc3339();
        let result = secret_number::Entity::update_many()
            .filter(secret_number::Column::ExamSessionId.eq(exam_session_id))
            .col_expr(secret_number::Column::IsActive, Expr::value(0))
            .col_expr(secret_number::Column::UpdatedAt, Expr::value(now))
            .exec(&self.db)
            .await?;
        Ok(result.rows_affected)
    }

    // ── Exam Halls ──

    pub async fn insert_exam_hall(&self, model: exam_hall::Model) -> Result<exam_hall::Model, DbErr> {
        let active = exam_hall::ActiveModel {
            id: Set(model.id),
            org_id: Set(model.org_id),
            exam_session_id: Set(model.exam_session_id),
            room_id: Set(model.room_id),
            hall_number: Set(model.hall_number),
            capacity: Set(model.capacity),
            allocated_count: Set(model.allocated_count),
            seating_arrangement: Set(model.seating_arrangement),
            floor_number: Set(model.floor_number),
            building: Set(model.building),
            is_accessible: Set(model.is_accessible),
            status: Set(model.status),
            notes: Set(model.notes),
            created_at: Set(model.created_at),
            updated_at: Set(model.updated_at),
        };
        active.insert(&self.db).await
    }

    pub async fn update_exam_hall_allocated_count(&self, hall_id: &str, count: i32) -> Result<(), DbErr> {
        let now = chrono::Utc::now().to_rfc3339();
        exam_hall::Entity::update_many()
            .filter(exam_hall::Column::Id.eq(hall_id))
            .col_expr(exam_hall::Column::AllocatedCount, Expr::value(count))
            .col_expr(exam_hall::Column::UpdatedAt, Expr::value(now))
            .exec(&self.db)
            .await?;
        Ok(())
    }

    // ── Exam Hall Allocations ──

    pub async fn insert_hall_allocation(&self, model: exam_hall_allocation::Model) -> Result<exam_hall_allocation::Model, DbErr> {
        let active = exam_hall_allocation::ActiveModel {
            id: Set(model.id),
            exam_hall_id: Set(model.exam_hall_id),
            exam_session_id: Set(model.exam_session_id),
            enrollment_id: Set(model.enrollment_id),
            seating_number_id: Set(model.seating_number_id),
            seat_position: Set(model.seat_position),
            is_present: Set(model.is_present),
            created_at: Set(model.created_at),
            updated_at: Set(model.updated_at),
        };
        active.insert(&self.db).await
    }

    pub async fn delete_hall_allocations(&self, exam_session_id: &str) -> Result<u64, DbErr> {
        let result = exam_hall_allocation::Entity::delete_many()
            .filter(exam_hall_allocation::Column::ExamSessionId.eq(exam_session_id))
            .exec(&self.db)
            .await?;
        Ok(result.rows_affected)
    }

    // ── Exam Proctors ──

    pub async fn insert_exam_proctor(&self, model: exam_proctor::Model) -> Result<exam_proctor::Model, DbErr> {
        self.cmd_insert_exam_proctor(model).await
    }

    pub async fn cmd_insert_exam_proctor(&self, model: exam_proctor::Model) -> Result<exam_proctor::Model, DbErr> {
        let active = exam_proctor::ActiveModel {
            id: Set(model.id),
            org_id: Set(model.org_id),
            exam_session_id: Set(model.exam_session_id),
            exam_schedule_id: Set(model.exam_schedule_id),
            exam_hall_id: Set(model.exam_hall_id),
            teacher_id: Set(model.teacher_id),
            role: Set(model.role),
            is_confirmed: Set(model.is_confirmed),
            confirmed_at: Set(model.confirmed_at),
            attendance_status: Set(model.attendance_status),
            check_in_time: Set(model.check_in_time),
            notes: Set(model.notes),
            created_at: Set(model.created_at),
            updated_at: Set(model.updated_at),
        };
        active.insert(&self.db).await
    }

    // ── Solve Jobs ──

    pub async fn insert_solve_job(&self, j: svc_scheduling::entities::solve_job::Model) -> Result<svc_scheduling::entities::solve_job::Model, DbErr> {
        let active: svc_scheduling::entities::solve_job::ActiveModel = j.into();
        active.insert(&self.db).await
    }

    pub async fn update_solve_job(&self, j: svc_scheduling::entities::solve_job::Model) -> Result<svc_scheduling::entities::solve_job::Model, DbErr> {
        let active: svc_scheduling::entities::solve_job::ActiveModel = j.into();
        active.update(&self.db).await
    }
}
