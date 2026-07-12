//! Lifecycle query repository — read operations.

use crate::entities::{
    organization, academic_year, academic_period,
    academic_policy,
    promotion_job, promotion_detail, student_registry_entry,
};
use sea_orm::{entity::prelude::*, QueryFilter, QueryOrder, EntityTrait, ColumnTrait};
use super::LifecycleQueryRepo;

impl LifecycleQueryRepo {
    // ── Organization Queries ────────────────────────────────────

    pub async fn get_organization_by_id(&self, id: &str) -> Result<Option<organization::Model>, DbErr> {
        organization::Entity::find()
            .filter(organization::Column::Id.eq(id))
            .filter(organization::Column::IsDeleted.eq(0))
            .one(&self.db)
            .await
    }

    pub async fn get_organization_by_slug(&self, slug: &str) -> Result<Option<organization::Model>, DbErr> {
        organization::Entity::find()
            .filter(organization::Column::Slug.eq(slug))
            .filter(organization::Column::IsDeleted.eq(0))
            .one(&self.db)
            .await
    }

    // ── Year & Period Queries ───────────────────────────────────────────

    pub async fn get_year_by_id(&self, id: &str) -> Result<Option<academic_year::Model>, DbErr> {
        academic_year::Entity::find_by_id(id.to_string())
            .one(&self.db)
            .await
    }

    pub async fn list_years_by_org(&self, org_id: &str) -> Result<Vec<academic_year::Model>, DbErr> {
        academic_year::Entity::find()
            .filter(academic_year::Column::OrgId.eq(org_id))
            .order_by_desc(academic_year::Column::StartDate)
            .all(&self.db)
            .await
    }

    pub async fn get_child_periods(&self, parent_id: &str) -> Result<Vec<academic_period::Model>, DbErr> {
        academic_period::Entity::find()
            .filter(academic_period::Column::ParentId.eq(parent_id))
            .order_by_asc(academic_period::Column::SortOrder)
            .all(&self.db)
            .await
    }



    // ── Policy Queries ──────────────────────────────────────────

    pub async fn get_academic_policy(&self, org_id: &str) -> Result<Option<academic_policy::Model>, DbErr> {
        academic_policy::Entity::find()
            .filter(academic_policy::Column::OrgId.eq(org_id))
            .one(&self.db)
            .await
    }

    // ── Promotion Queries ──

    pub async fn get_promotion_jobs(&self, org_id: &str) -> Result<Vec<promotion_job::Model>, DbErr> {
        promotion_job::Entity::find()
            .filter(promotion_job::Column::OrgId.eq(org_id))
            .order_by_desc(promotion_job::Column::CreatedAt)
            .all(&self.db)
            .await
    }

    pub async fn get_promotion_job_by_id(&self, job_id: &str) -> Result<Option<promotion_job::Model>, DbErr> {
        promotion_job::Entity::find_by_id(job_id.to_string()).one(&self.db).await
    }

    pub async fn get_promotion_details(&self, job_id: &str) -> Result<Vec<promotion_detail::Model>, DbErr> {
        promotion_detail::Entity::find()
            .filter(promotion_detail::Column::JobId.eq(job_id))
            .order_by_asc(promotion_detail::Column::SourceLevel)
            .order_by_asc(promotion_detail::Column::StudentName)
            .all(&self.db)
            .await
    }

    pub async fn get_promotion_details_by_decision(
        &self,
        job_id: &str,
        decision: &str,
    ) -> Result<Vec<promotion_detail::Model>, DbErr> {
        promotion_detail::Entity::find()
            .filter(promotion_detail::Column::JobId.eq(job_id))
            .filter(promotion_detail::Column::Decision.eq(decision))
            .all(&self.db)
            .await
    }

    // ── Student Registry Queries ──

    pub async fn get_student_registry(&self, org_id: &str) -> Result<Vec<student_registry_entry::Model>, DbErr> {
        student_registry_entry::Entity::find()
            .filter(student_registry_entry::Column::OrgId.eq(org_id))
            .order_by_asc(student_registry_entry::Column::FullName)
            .all(&self.db)
            .await
    }

    pub async fn get_student_registry_entry(&self, user_id: &str) -> Result<Option<student_registry_entry::Model>, DbErr> {
        student_registry_entry::Entity::find()
            .filter(student_registry_entry::Column::UserId.eq(user_id))
            .one(&self.db)
            .await
    }
}
