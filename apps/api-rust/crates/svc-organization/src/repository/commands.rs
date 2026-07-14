//! Organization command repository — write operations.

use crate::entities::{
    organization, academic_period,
    academic_policy,
    promotion_job, promotion_detail,
};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DbErr, EntityTrait};
use super::OrganizationCommandRepo;

impl OrganizationCommandRepo {
    // ── Organization Commands ───────────────────────────────────

    pub async fn create_organization(&self, o: organization::Model) -> Result<organization::Model, DbErr> {
        let active = organization::ActiveModel {
            current_period_id: Set(o.current_period_id),
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

    // ── Year Commands ──────────────────────────────────────────

    pub async fn create_year(&self, y: crate::entities::academic_year::Model) -> Result<crate::entities::academic_year::Model, DbErr> {
        let active: crate::entities::academic_year::ActiveModel = y.into();
        active.insert(&self.db).await
    }

    pub async fn update_year(&self, model: crate::entities::academic_year::ActiveModel) -> Result<crate::entities::academic_year::Model, DbErr> {
        model.update(&self.db).await
    }

    pub async fn delete_year(&self, period_id: &str) -> Result<(), DbErr> {
        crate::entities::academic_year::Entity::delete_by_id(period_id.to_string())
            .exec(&self.db)
            .await?;
        Ok(())
    }

    // ── Period Commands ──────────────────────────────────────────

    pub async fn create_period(&self, c: academic_period::Model) -> Result<academic_period::Model, DbErr> {
        let active = academic_period::ActiveModel {
            id: Set(c.id),
            org_id: Set(c.org_id),
            period_id: Set(c.period_id),
            parent_id: Set(c.parent_id),
            name: Set(c.name),
            name_ar: Set(c.name_ar),
            code: Set(c.code),
            period_type: Set(c.period_type),
            start_date: Set(c.start_date),
            end_date: Set(c.end_date),
            hijri_start: Set(c.hijri_start),
            hijri_end: Set(c.hijri_end),
            enrollment_start: Set(c.enrollment_start),
            enrollment_end: Set(c.enrollment_end),
            is_enrollment_open: Set(c.is_enrollment_open),
            grade_entry_start: Set(c.grade_entry_start),
            grade_entry_end: Set(c.grade_entry_end),
            is_grade_entry_open: Set(c.is_grade_entry_open),
            exam_start: Set(c.exam_start),
            exam_end: Set(c.exam_end),
            grading_system: Set(c.grading_system),
            passing_grade: Set(c.passing_grade),
            max_grade: Set(c.max_grade),
            weight: Set(c.weight),
            status: Set(c.status),
            is_locked: Set(c.is_locked),
            locked_at: Set(c.locked_at),
            locked_by: Set(c.locked_by),
            sort_order: Set(c.sort_order),
            created_at: Set(c.created_at),
            updated_at: Set(c.updated_at),
        };
        active.insert(&self.db).await
    }

    pub async fn update_period(&self, model: academic_period::ActiveModel) -> Result<academic_period::Model, DbErr> {
        model.update(&self.db).await
    }



    // ── Policy Commands ─────────────────────────────────────────

    pub async fn upsert_academic_policy(&self, p: academic_policy::Model) -> Result<academic_policy::Model, DbErr> {
        let active = academic_policy::ActiveModel {
            id: Set(p.id),
            org_id: Set(p.org_id),
            org_unit_id: Set(p.org_unit_id),
            default_grading_system: Set(p.default_grading_system),
            default_passing_grade: Set(p.default_passing_grade),
            default_max_grade: Set(p.default_max_grade),
            enrollment_auto_close: Set(p.enrollment_auto_close),
            allow_late_enrollment: Set(p.allow_late_enrollment),
            late_enrollment_approval: Set(p.late_enrollment_approval),
            grade_entry_auto_close: Set(p.grade_entry_auto_close),
            allow_grade_edit_after_close: Set(p.allow_grade_edit_after_close),
            grade_edit_approval: Set(p.grade_edit_approval),
            passing_score: Set(p.passing_score),
            min_attendance_rate: Set(p.min_attendance_rate),
            auto_graduate_final_level: Set(p.auto_graduate_final_level),
            final_level: Set(p.final_level),
            exclude_withdrawn: Set(p.exclude_withdrawn),
            autopilot_mode: Set(p.autopilot_mode),
            autopilot_enabled: Set(p.autopilot_enabled),
            max_auto_retries: Set(p.max_auto_retries),
            pause_on_error: Set(p.pause_on_error),
            auto_open_enrollment: Set(p.auto_open_enrollment),
            auto_close_enrollment: Set(p.auto_close_enrollment),
            auto_open_grade_entry: Set(p.auto_open_grade_entry),
            auto_close_grade_entry: Set(p.auto_close_grade_entry),
            auto_lock_periods: Set(p.auto_lock_periods),
            auto_advance_periods: Set(p.auto_advance_periods),
            auto_archive_years: Set(p.auto_archive_years),
            grace_days_after_deadline: Set(p.grace_days_after_deadline),
            notify_before_deadline_days: Set(p.notify_before_deadline_days),
            default_template_id: Set(p.default_template_id),
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


}

