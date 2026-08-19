//! Organization command repository — write operations.

use crate::entities::{
    organization,
};
use svc_calendar::entities::{academic_year, academic_period};
use svc_enrollment_workflow::entities::{promotion_job, promotion_detail};
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

    pub async fn create_year(&self, y: academic_year::Model) -> Result<academic_year::Model, DbErr> {
        let active: academic_year::ActiveModel = y.into();
        active.insert(&self.db).await
    }

    pub async fn update_year(&self, model: academic_year::ActiveModel) -> Result<academic_year::Model, DbErr> {
        model.update(&self.db).await
    }

    pub async fn delete_year(&self, period_id: &str) -> Result<(), DbErr> {
        academic_year::Entity::delete_by_id(period_id.to_string())
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
            scale_id: Set(c.scale_id),
            pass_fail_policy_id: Set(c.pass_fail_policy_id),
            status: Set(c.status),
            is_locked: Set(c.is_locked),
            locked_at: Set(c.locked_at),
            locked_by: Set(c.locked_by),
            sort_order: Set(c.sort_order),
            metadata: Set(c.metadata),
            created_at: Set(c.created_at),
            updated_at: Set(c.updated_at),
        };
        active.insert(&self.db).await
    }

    pub async fn update_period(&self, model: academic_period::ActiveModel) -> Result<academic_period::Model, DbErr> {
        model.update(&self.db).await
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

