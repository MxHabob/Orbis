//! Lifecycle command repository — write operations.

use crate::entities::{
    organization, academic_cycle, academic_calendar_event,
    org_academic_policy, promotion_policy,
    promotion_job, promotion_detail, student_registry_entry,
};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DbErr, EntityTrait, ColumnTrait, QueryFilter};
use super::LifecycleCommandRepo;

impl LifecycleCommandRepo {
    // ── Organization Commands ───────────────────────────────────

    pub async fn create_organization(&self, o: organization::Model) -> Result<organization::Model, DbErr> {
        let active = organization::ActiveModel {
            id: Set(o.id),
            name: Set(o.name),
            name_ar: Set(o.name_ar),
            slug: Set(o.slug),
            code: Set(o.code),
            timezone: Set(o.timezone),
            locale: Set(o.locale),
            currency: Set(o.currency),
            date_format: Set(o.date_format),
            hijri_enabled: Set(o.hijri_enabled),
            max_users: Set(o.max_users),
            max_students: Set(o.max_students),
            max_storage_gb: Set(o.max_storage_gb),
            settings: Set(o.settings),
            features: Set(o.features),
            subscription_plan: Set(o.subscription_plan),
            subscription_expires_at: Set(o.subscription_expires_at),
            logo: Set(o.logo),
            favicon: Set(o.favicon),
            primary_color: Set(o.primary_color),
            is_active: Set(o.is_active),
            is_verified: Set(o.is_verified),
            is_deleted: Set(o.is_deleted),
            deleted_at: Set(o.deleted_at),
            deleted_by: Set(o.deleted_by),
            created_at: Set(o.created_at),
            updated_at: Set(o.updated_at),
        };
        active.insert(&self.db).await
    }

    pub async fn update_organization(&self, model: organization::ActiveModel) -> Result<organization::Model, DbErr> {
        model.update(&self.db).await
    }

    // ── Cycle Commands ──────────────────────────────────────────

    pub async fn create_cycle(&self, c: academic_cycle::Model) -> Result<academic_cycle::Model, DbErr> {
        let active = academic_cycle::ActiveModel {
            id: Set(c.id),
            org_id: Set(c.org_id),
            name: Set(c.name),
            name_ar: Set(c.name_ar),
            code: Set(c.code),
            cycle_type: Set(c.cycle_type),
            parent_id: Set(c.parent_id),
            start_date: Set(c.start_date),
            end_date: Set(c.end_date),
            hijri_start_date: Set(c.hijri_start_date),
            hijri_end_date: Set(c.hijri_end_date),
            exam_start_date: Set(c.exam_start_date),
            exam_end_date: Set(c.exam_end_date),
            weight: Set(c.weight),
            grading_system: Set(c.grading_system),
            passing_grade: Set(c.passing_grade),
            max_grade: Set(c.max_grade),
            is_current: Set(c.is_current),
            is_locked: Set(c.is_locked),
            is_enrollment_open: Set(c.is_enrollment_open),
            is_grade_entry_open: Set(c.is_grade_entry_open),
            grade_entry_start: Set(c.grade_entry_start),
            grade_entry_end: Set(c.grade_entry_end),
            locked_at: Set(c.locked_at),
            locked_by: Set(c.locked_by),
            settings: Set(c.settings),
            sort_order: Set(c.sort_order),
            created_at: Set(c.created_at),
            updated_at: Set(c.updated_at),
        };
        active.insert(&self.db).await
    }

    pub async fn update_cycle(&self, model: academic_cycle::ActiveModel) -> Result<academic_cycle::Model, DbErr> {
        model.update(&self.db).await
    }

    // ── Calendar Event Commands ─────────────────────────────────

    pub async fn create_calendar_event(&self, e: academic_calendar_event::Model) -> Result<academic_calendar_event::Model, DbErr> {
        let active = academic_calendar_event::ActiveModel {
            id: Set(e.id),
            org_id: Set(e.org_id),
            cycle_id: Set(e.cycle_id),
            event_type: Set(e.event_type),
            title: Set(e.title),
            title_ar: Set(e.title_ar),
            description: Set(e.description),
            start_date: Set(e.start_date),
            end_date: Set(e.end_date),
            start_time: Set(e.start_time),
            end_time: Set(e.end_time),
            execution_mode: Set(e.execution_mode),
            auto_action: Set(e.auto_action),
            prerequisites: Set(e.prerequisites),
            executed_at: Set(e.executed_at),
            executed_by: Set(e.executed_by),
            execution_result: Set(e.execution_result),
            execution_notes: Set(e.execution_notes),
            is_recurring: Set(e.is_recurring),
            recurrence_rule: Set(e.recurrence_rule),
            color: Set(e.color),
            is_active: Set(e.is_active),
            sort_order: Set(e.sort_order),
            created_by: Set(e.created_by),
            created_at: Set(e.created_at),
            updated_at: Set(e.updated_at),
        };
        active.insert(&self.db).await
    }

    pub async fn update_calendar_event(&self, model: academic_calendar_event::ActiveModel) -> Result<academic_calendar_event::Model, DbErr> {
        model.update(&self.db).await
    }

    // ── Policy Commands ─────────────────────────────────────────

    pub async fn upsert_academic_policy(&self, p: org_academic_policy::Model) -> Result<org_academic_policy::Model, DbErr> {
        let active = org_academic_policy::ActiveModel {
            id: Set(p.id),
            org_id: Set(p.org_id),
            default_grading_system: Set(p.default_grading_system),
            default_passing_grade: Set(p.default_passing_grade),
            default_max_grade: Set(p.default_max_grade),
            enrollment_auto_close_on_capacity: Set(p.enrollment_auto_close_on_capacity),
            allow_late_enrollment: Set(p.allow_late_enrollment),
            late_enrollment_requires_approval: Set(p.late_enrollment_requires_approval),
            grade_entry_auto_close_on_deadline: Set(p.grade_entry_auto_close_on_deadline),
            allow_grade_modification_after_close: Set(p.allow_grade_modification_after_close),
            grade_modification_requires_approval: Set(p.grade_modification_requires_approval),
            default_event_execution_mode: Set(p.default_event_execution_mode),
            settings: Set(p.settings),
            created_at: Set(p.created_at),
            updated_at: Set(p.updated_at),
        };
        active.insert(&self.db).await
    }

    pub async fn upsert_promotion_policy(&self, p: promotion_policy::Model) -> Result<promotion_policy::Model, DbErr> {
        let active = promotion_policy::ActiveModel {
            id: Set(p.id),
            org_id: Set(p.org_id),
            passing_score: Set(p.passing_score),
            min_attendance_rate: Set(p.min_attendance_rate),
            auto_graduate_final_level: Set(p.auto_graduate_final_level),
            final_level: Set(p.final_level),
            exclude_withdrawn: Set(p.exclude_withdrawn),
            fee_carry_over_mode: Set(p.fee_carry_over_mode),
            fee_carry_over_categories: Set(p.fee_carry_over_categories),
            carry_over_guardians: Set(p.carry_over_guardians),
            carry_over_time_slots: Set(p.carry_over_time_slots),
            carry_over_class_subjects: Set(p.carry_over_class_subjects),
            auto_create_cycles: Set(p.auto_create_cycles),
            cycle_template: Set(p.cycle_template),
            created_at: Set(p.created_at),
            updated_at: Set(p.updated_at),
        };
        active.insert(&self.db).await
    }

    // ── Promotion Commands ──

    pub async fn create_promotion_job(&self, job: promotion_job::Model) -> Result<promotion_job::Model, DbErr> {
        let active: promotion_job::ActiveModel = job.into();
        active.insert(&self.db).await
    }

    pub async fn update_promotion_job(&self, job: promotion_job::Model) -> Result<promotion_job::Model, DbErr> {
        let active: promotion_job::ActiveModel = job.into();
        active.update(&self.db).await
    }

    pub async fn create_promotion_details_batch(
        &self,
        details: Vec<promotion_detail::Model>,
    ) -> Result<(), DbErr> {
        if details.is_empty() {
            return Ok(());
        }
        let models: Vec<promotion_detail::ActiveModel> =
            details.into_iter().map(|d| d.into()).collect();
        promotion_detail::Entity::insert_many(models)
            .exec(&self.db)
            .await?;
        Ok(())
    }

    pub async fn update_promotion_detail(&self, detail: promotion_detail::Model) -> Result<promotion_detail::Model, DbErr> {
        let active: promotion_detail::ActiveModel = detail.into();
        active.update(&self.db).await
    }

    // ── Student Registry Commands ──

    pub async fn upsert_student_registry(&self, entry: student_registry_entry::Model) -> Result<student_registry_entry::Model, DbErr> {
        let existing = student_registry_entry::Entity::find()
            .filter(student_registry_entry::Column::UserId.eq(&entry.user_id))
            .one(&self.db)
            .await?;

        if let Some(existing) = existing {
            let mut active: student_registry_entry::ActiveModel = existing.into();
            active.status = Set(entry.status);
            active.current_year_id = Set(entry.current_year_id);
            active.current_level = Set(entry.current_level);
            active.current_class_name = Set(entry.current_class_name);
            active.years_enrolled = Set(entry.years_enrolled);
            active.full_name = Set(entry.full_name);
            active.name_ar = Set(entry.name_ar);
            active.updated_at = Set(chrono::Utc::now().to_rfc3339());
            active.update(&self.db).await
        } else {
            let active: student_registry_entry::ActiveModel = entry.into();
            active.insert(&self.db).await
        }
    }

    pub async fn update_student_registry_status(&self, user_id: &str, status: &str) -> Result<(), DbErr> {
        let entry = student_registry_entry::Entity::find()
            .filter(student_registry_entry::Column::UserId.eq(user_id))
            .one(&self.db)
            .await?
            .ok_or(DbErr::RecordNotFound("student registry entry not found".into()))?;

        let mut active: student_registry_entry::ActiveModel = entry.into();
        active.status = Set(status.to_string());
        active.updated_at = Set(chrono::Utc::now().to_rfc3339());
        active.update(&self.db).await?;
        Ok(())
    }
}
