use crate::entities::{
    exam_session, exam_subject, exam_schedule,
    seating_number, secret_number,
    exam_hall, exam_hall_allocation,
    exam_proctor, teacher_exam_availability,
    student_ordering,
};
use sea_orm::{entity::prelude::*, ActiveModelTrait, ActiveValue::Set, QueryFilter, EntityTrait, QueryOrder, sea_query::Expr};

impl super::Repository {
    // ── Student Ordering ────────────────────────────────────────────────────

    pub async fn get_student_ordering(
        &self,
        org_id: &str,
        cycle_id: &str,
        class_id: Option<&str>,
    ) -> Result<Vec<student_ordering::Model>, DbErr> {
        let mut query = student_ordering::Entity::find()
            .filter(student_ordering::Column::OrgId.eq(org_id))
            .filter(student_ordering::Column::CycleId.eq(cycle_id));

        if let Some(cid) = class_id {
            query = query.filter(student_ordering::Column::ClassId.eq(cid));
        }

        query
            .order_by_asc(student_ordering::Column::OrderInClass)
            .all(&self.db)
            .await
    }

    pub async fn get_student_order_by_enrollment(
        &self,
        enrollment_id: &str,
    ) -> Result<Option<student_ordering::Model>, DbErr> {
        student_ordering::Entity::find()
            .filter(student_ordering::Column::EnrollmentId.eq(enrollment_id))
            .one(&self.db)
            .await
    }

    pub async fn upsert_student_ordering(
        &self,
        model: student_ordering::Model,
    ) -> Result<student_ordering::Model, DbErr> {
        // Try to find existing record by enrollment_id
        let existing = student_ordering::Entity::find()
            .filter(student_ordering::Column::EnrollmentId.eq(&model.enrollment_id))
            .one(&self.db)
            .await?;

        if let Some(_) = existing {
            // Update
            let active = student_ordering::ActiveModel {
                id: Set(model.id),
                org_id: Set(model.org_id),
                cycle_id: Set(model.cycle_id),
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
            active.update(&self.db).await
        } else {
            let active = student_ordering::ActiveModel {
                id: Set(model.id),
                org_id: Set(model.org_id),
                cycle_id: Set(model.cycle_id),
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
            active.insert(&self.db).await
        }
    }

    pub async fn delete_student_ordering_for_cycle(
        &self,
        org_id: &str,
        cycle_id: &str,
    ) -> Result<u64, DbErr> {
        let result = student_ordering::Entity::delete_many()
            .filter(student_ordering::Column::OrgId.eq(org_id))
            .filter(student_ordering::Column::CycleId.eq(cycle_id))
            .filter(student_ordering::Column::IsFrozen.eq(0))
            .exec(&self.db)
            .await?;
        Ok(result.rows_affected)
    }

    pub async fn freeze_student_ordering(
        &self,
        org_id: &str,
        cycle_id: &str,
        frozen_by: &str,
    ) -> Result<u64, DbErr> {
        let now = chrono::Utc::now().to_rfc3339();
        let result = student_ordering::Entity::update_many()
            .filter(student_ordering::Column::OrgId.eq(org_id))
            .filter(student_ordering::Column::CycleId.eq(cycle_id))
            .col_expr(student_ordering::Column::IsFrozen, Expr::value(1))
            .col_expr(student_ordering::Column::FrozenAt, Expr::value(now.clone()))
            .col_expr(student_ordering::Column::FrozenBy, Expr::value(frozen_by))
            .col_expr(student_ordering::Column::UpdatedAt, Expr::value(now))
            .exec(&self.db)
            .await?;
        Ok(result.rows_affected)
    }

    // ── Exam Sessions ───────────────────────────────────────────────────────

    pub async fn get_exam_sessions(
        &self,
        org_id: &str,
        cycle_id: Option<&str>,
    ) -> Result<Vec<exam_session::Model>, DbErr> {
        let mut query = exam_session::Entity::find()
            .filter(exam_session::Column::OrgId.eq(org_id));

        if let Some(cid) = cycle_id {
            query = query.filter(exam_session::Column::CycleId.eq(cid));
        }

        query
            .order_by_desc(exam_session::Column::CreatedAt)
            .all(&self.db)
            .await
    }

    pub async fn get_exam_session_by_id(
        &self,
        id: &str,
    ) -> Result<Option<exam_session::Model>, DbErr> {
        exam_session::Entity::find_by_id(id.to_string())
            .one(&self.db)
            .await
    }

    pub async fn insert_exam_session(
        &self,
        model: exam_session::Model,
    ) -> Result<exam_session::Model, DbErr> {
        let active = exam_session::ActiveModel {
            id: Set(model.id),
            org_id: Set(model.org_id),
            cycle_id: Set(model.cycle_id),
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

    pub async fn update_exam_session(
        &self,
        model: exam_session::Model,
    ) -> Result<exam_session::Model, DbErr> {
        let active = exam_session::ActiveModel {
            id: Set(model.id),
            org_id: Set(model.org_id),
            cycle_id: Set(model.cycle_id),
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

    // ── Exam Subjects ───────────────────────────────────────────────────────

    pub async fn get_exam_subjects(
        &self,
        exam_session_id: &str,
    ) -> Result<Vec<exam_subject::Model>, DbErr> {
        exam_subject::Entity::find()
            .filter(exam_subject::Column::ExamSessionId.eq(exam_session_id))
            .order_by_asc(exam_subject::Column::SortOrder)
            .all(&self.db)
            .await
    }

    pub async fn get_exam_subject_by_id(
        &self,
        id: &str,
    ) -> Result<Option<exam_subject::Model>, DbErr> {
        exam_subject::Entity::find_by_id(id.to_string())
            .one(&self.db)
            .await
    }

    pub async fn insert_exam_subject(
        &self,
        model: exam_subject::Model,
    ) -> Result<exam_subject::Model, DbErr> {
        let active = exam_subject::ActiveModel {
            id: Set(model.id),
            exam_session_id: Set(model.exam_session_id),
            subject_id: Set(model.subject_id),
            class_subject_id: Set(model.class_subject_id),
            max_score: Set(model.max_score),
            passing_score: Set(model.passing_score),
            practical_score: Set(model.practical_score),
            oral_score: Set(model.oral_score),
            coursework_score: Set(model.coursework_score),
            exam_written_score: Set(model.exam_written_score),
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

    // ── Exam Schedules ──────────────────────────────────────────────────────

    pub async fn get_exam_schedules(
        &self,
        exam_session_id: &str,
    ) -> Result<Vec<exam_schedule::Model>, DbErr> {
        exam_schedule::Entity::find()
            .filter(exam_schedule::Column::ExamSessionId.eq(exam_session_id))
            .order_by_asc(exam_schedule::Column::ExamDate)
            .order_by_asc(exam_schedule::Column::StartTime)
            .all(&self.db)
            .await
    }

    pub async fn insert_exam_schedule(
        &self,
        model: exam_schedule::Model,
    ) -> Result<exam_schedule::Model, DbErr> {
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

    // ── Seating Numbers ─────────────────────────────────────────────────────

    pub async fn get_seating_numbers(
        &self,
        exam_session_id: &str,
        class_id: Option<&str>,
    ) -> Result<Vec<seating_number::Model>, DbErr> {
        let mut query = seating_number::Entity::find()
            .filter(seating_number::Column::ExamSessionId.eq(exam_session_id))
            .filter(seating_number::Column::IsActive.eq(1));

        if let Some(cid) = class_id {
            query = query.filter(seating_number::Column::ClassId.eq(cid));
        }

        query
            .order_by_asc(seating_number::Column::SequenceNumber)
            .all(&self.db)
            .await
    }

    pub async fn insert_seating_number(
        &self,
        model: seating_number::Model,
    ) -> Result<seating_number::Model, DbErr> {
        let active = seating_number::ActiveModel {
            id: Set(model.id),
            org_id: Set(model.org_id),
            exam_session_id: Set(model.exam_session_id),
            enrollment_id: Set(model.enrollment_id),
            student_id: Set(model.student_id),
            class_id: Set(model.class_id),
            seating_number: Set(model.seating_number),
            sequence_number: Set(model.sequence_number),
            level: Set(model.level),
            division: Set(model.division),
            student_name_ar: Set(model.student_name_ar),
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

    pub async fn deactivate_seating_numbers(
        &self,
        exam_session_id: &str,
    ) -> Result<u64, DbErr> {
        let now = chrono::Utc::now().to_rfc3339();
        let result = seating_number::Entity::update_many()
            .filter(seating_number::Column::ExamSessionId.eq(exam_session_id))
            .col_expr(seating_number::Column::IsActive, Expr::value(0))
            .col_expr(seating_number::Column::UpdatedAt, Expr::value(now))
            .exec(&self.db)
            .await?;
        Ok(result.rows_affected)
    }

    // ── Secret Numbers ──────────────────────────────────────────────────────

    pub async fn get_secret_numbers(
        &self,
        exam_session_id: &str,
    ) -> Result<Vec<secret_number::Model>, DbErr> {
        secret_number::Entity::find()
            .filter(secret_number::Column::ExamSessionId.eq(exam_session_id))
            .filter(secret_number::Column::IsActive.eq(1))
            .all(&self.db)
            .await
    }

    pub async fn insert_secret_number(
        &self,
        model: secret_number::Model,
    ) -> Result<secret_number::Model, DbErr> {
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

    pub async fn reveal_secret_numbers(
        &self,
        exam_session_id: &str,
        revealed_by: &str,
    ) -> Result<u64, DbErr> {
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

    pub async fn deactivate_secret_numbers(
        &self,
        exam_session_id: &str,
    ) -> Result<u64, DbErr> {
        let now = chrono::Utc::now().to_rfc3339();
        let result = secret_number::Entity::update_many()
            .filter(secret_number::Column::ExamSessionId.eq(exam_session_id))
            .col_expr(secret_number::Column::IsActive, Expr::value(0))
            .col_expr(secret_number::Column::UpdatedAt, Expr::value(now))
            .exec(&self.db)
            .await?;
        Ok(result.rows_affected)
    }

    pub async fn lookup_by_secret(
        &self,
        exam_session_id: &str,
        secret_hash: &str,
    ) -> Result<Option<secret_number::Model>, DbErr> {
        secret_number::Entity::find()
            .filter(secret_number::Column::ExamSessionId.eq(exam_session_id))
            .filter(secret_number::Column::SecretHash.eq(secret_hash))
            .filter(secret_number::Column::IsActive.eq(1))
            .one(&self.db)
            .await
    }

    // ── Exam Halls ──────────────────────────────────────────────────────────

    pub async fn get_exam_halls(
        &self,
        exam_session_id: &str,
    ) -> Result<Vec<exam_hall::Model>, DbErr> {
        exam_hall::Entity::find()
            .filter(exam_hall::Column::ExamSessionId.eq(exam_session_id))
            .filter(exam_hall::Column::Status.eq("active"))
            .order_by_asc(exam_hall::Column::HallNumber)
            .all(&self.db)
            .await
    }

    pub async fn insert_exam_hall(
        &self,
        model: exam_hall::Model,
    ) -> Result<exam_hall::Model, DbErr> {
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

    pub async fn update_exam_hall_allocated_count(
        &self,
        hall_id: &str,
        count: i32,
    ) -> Result<(), DbErr> {
        let now = chrono::Utc::now().to_rfc3339();
        exam_hall::Entity::update_many()
            .filter(exam_hall::Column::Id.eq(hall_id))
            .col_expr(exam_hall::Column::AllocatedCount, Expr::value(count))
            .col_expr(exam_hall::Column::UpdatedAt, Expr::value(now))
            .exec(&self.db)
            .await?;
        Ok(())
    }

    // ── Exam Hall Allocations ───────────────────────────────────────────────

    pub async fn get_hall_allocations(
        &self,
        exam_session_id: &str,
        hall_id: Option<&str>,
    ) -> Result<Vec<exam_hall_allocation::Model>, DbErr> {
        let mut query = exam_hall_allocation::Entity::find()
            .filter(exam_hall_allocation::Column::ExamSessionId.eq(exam_session_id));

        if let Some(hid) = hall_id {
            query = query.filter(exam_hall_allocation::Column::ExamHallId.eq(hid));
        }

        query.all(&self.db).await
    }

    pub async fn insert_hall_allocation(
        &self,
        model: exam_hall_allocation::Model,
    ) -> Result<exam_hall_allocation::Model, DbErr> {
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

    pub async fn delete_hall_allocations(
        &self,
        exam_session_id: &str,
    ) -> Result<u64, DbErr> {
        let result = exam_hall_allocation::Entity::delete_many()
            .filter(exam_hall_allocation::Column::ExamSessionId.eq(exam_session_id))
            .exec(&self.db)
            .await?;
        Ok(result.rows_affected)
    }

    // ── Exam Proctors ───────────────────────────────────────────────────────

    pub async fn get_exam_proctors(
        &self,
        exam_session_id: &str,
        schedule_id: Option<&str>,
    ) -> Result<Vec<exam_proctor::Model>, DbErr> {
        let mut query = exam_proctor::Entity::find()
            .filter(exam_proctor::Column::ExamSessionId.eq(exam_session_id));

        if let Some(sid) = schedule_id {
            query = query.filter(exam_proctor::Column::ExamScheduleId.eq(sid));
        }

        query.all(&self.db).await
    }

    pub async fn insert_exam_proctor(
        &self,
        model: exam_proctor::Model,
    ) -> Result<exam_proctor::Model, DbErr> {
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

    // ── Teacher Exam Availability ───────────────────────────────────────────

    pub async fn get_teacher_exam_availability(
        &self,
        exam_session_id: &str,
        date: Option<&str>,
    ) -> Result<Vec<teacher_exam_availability::Model>, DbErr> {
        let mut query = teacher_exam_availability::Entity::find()
            .filter(teacher_exam_availability::Column::ExamSessionId.eq(exam_session_id));

        if let Some(d) = date {
            query = query.filter(teacher_exam_availability::Column::Date.eq(d));
        }

        query.all(&self.db).await
    }

    pub async fn get_available_teachers_for_date(
        &self,
        org_id: &str,
        exam_session_id: &str,
        date: &str,
    ) -> Result<Vec<String>, DbErr> {
        // Get all teachers
        use crate::entities::user;
        let teachers = user::Entity::find()
            .filter(user::Column::OrgId.eq(org_id))
            .filter(user::Column::PrimaryRole.eq("teacher"))
            .filter(user::Column::Status.eq("active"))
            .filter(user::Column::IsDeleted.eq(0))
            .all(&self.db)
            .await?;

        let teacher_ids: Vec<String> = teachers.iter().map(|t| t.id.clone()).collect();

        // Get unavailable teachers
        let unavailable = teacher_exam_availability::Entity::find()
            .filter(teacher_exam_availability::Column::ExamSessionId.eq(exam_session_id))
            .filter(teacher_exam_availability::Column::Date.eq(date))
            .filter(teacher_exam_availability::Column::IsAvailable.eq(0))
            .all(&self.db)
            .await?;

        let unavailable_ids: std::collections::HashSet<String> =
            unavailable.iter().map(|u| u.teacher_id.clone()).collect();

        Ok(teacher_ids
            .into_iter()
            .filter(|id| !unavailable_ids.contains(id))
            .collect())
    }
}
