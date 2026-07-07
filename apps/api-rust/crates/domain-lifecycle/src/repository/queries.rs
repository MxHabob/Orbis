//! Lifecycle query repository — read operations.

use crate::entities::{
    organization, academic_cycle, academic_calendar_event,
    org_academic_policy, promotion_policy,
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

    // ── Cycle Queries ───────────────────────────────────────────

    pub async fn get_cycle_by_id(&self, id: &str) -> Result<Option<academic_cycle::Model>, DbErr> {
        academic_cycle::Entity::find_by_id(id.to_string())
            .one(&self.db)
            .await
    }

    pub async fn list_cycles_by_org(&self, org_id: &str) -> Result<Vec<academic_cycle::Model>, DbErr> {
        academic_cycle::Entity::find()
            .filter(academic_cycle::Column::OrgId.eq(org_id))
            .order_by_desc(academic_cycle::Column::StartDate)
            .all(&self.db)
            .await
    }

    pub async fn get_current_cycle(&self, org_id: &str) -> Result<Option<academic_cycle::Model>, DbErr> {
        academic_cycle::Entity::find()
            .filter(academic_cycle::Column::OrgId.eq(org_id))
            .filter(academic_cycle::Column::IsCurrent.eq(1))
            .one(&self.db)
            .await
    }

    pub async fn get_child_cycles(&self, parent_id: &str) -> Result<Vec<academic_cycle::Model>, DbErr> {
        academic_cycle::Entity::find()
            .filter(academic_cycle::Column::ParentId.eq(parent_id))
            .order_by_asc(academic_cycle::Column::SortOrder)
            .all(&self.db)
            .await
    }

    // ── Calendar Event Queries ──────────────────────────────────

    pub async fn list_events_by_cycle(&self, cycle_id: &str) -> Result<Vec<academic_calendar_event::Model>, DbErr> {
        academic_calendar_event::Entity::find()
            .filter(academic_calendar_event::Column::CycleId.eq(cycle_id))
            .filter(academic_calendar_event::Column::IsActive.eq(1))
            .order_by_asc(academic_calendar_event::Column::StartDate)
            .all(&self.db)
            .await
    }

    pub async fn get_pending_events(
        &self,
        org_id: &str,
        before_date: &str,
    ) -> Result<Vec<academic_calendar_event::Model>, DbErr> {
        academic_calendar_event::Entity::find()
            .filter(academic_calendar_event::Column::OrgId.eq(org_id))
            .filter(academic_calendar_event::Column::IsActive.eq(1))
            .filter(academic_calendar_event::Column::ExecutedAt.is_null())
            .filter(academic_calendar_event::Column::StartDate.lte(before_date))
            .order_by_asc(academic_calendar_event::Column::StartDate)
            .all(&self.db)
            .await
    }

    // ── Policy Queries ──────────────────────────────────────────

    pub async fn get_academic_policy(&self, org_id: &str) -> Result<Option<org_academic_policy::Model>, DbErr> {
        org_academic_policy::Entity::find()
            .filter(org_academic_policy::Column::OrgId.eq(org_id))
            .one(&self.db)
            .await
    }

    pub async fn get_promotion_policy(&self, org_id: &str) -> Result<Option<promotion_policy::Model>, DbErr> {
        promotion_policy::Entity::find()
            .filter(promotion_policy::Column::OrgId.eq(org_id))
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
