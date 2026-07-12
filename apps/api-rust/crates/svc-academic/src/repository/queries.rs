//! Academic query repository — optimized read operations.

use crate::entities::{class, subject, class_subject, enrollment, org_unit};
use sea_orm::{entity::prelude::*, QueryFilter, QueryOrder, EntityTrait, ColumnTrait};
use super::AcademicQueryRepo;

impl AcademicQueryRepo {
    // ── Class Queries ───────────────────────────────────────────

    pub async fn get_class_by_id(&self, id: &str) -> Result<Option<class::Model>, DbErr> {
        class::Entity::find_by_id(id.to_string())
            .one(&self.db)
            .await
    }

    pub async fn list_classes_by_period(
        &self,
        org_id: &str,
        period_id: &str,
    ) -> Result<Vec<class::Model>, DbErr> {
        class::Entity::find()
            .filter(class::Column::OrgId.eq(org_id))
            .filter(class::Column::PeriodId.eq(period_id))
            .filter(class::Column::IsDeleted.eq(0))
            .order_by_asc(class::Column::Level)
            .order_by_asc(class::Column::Name)
            .all(&self.db)
            .await
    }

    // ── Subject Queries ─────────────────────────────────────────

    pub async fn list_subjects_by_org(&self, org_id: &str) -> Result<Vec<subject::Model>, DbErr> {
        subject::Entity::find()
            .filter(subject::Column::OrgId.eq(org_id))
            .filter(subject::Column::IsActive.eq(1))
            .order_by_asc(subject::Column::SortOrder)
            .all(&self.db)
            .await
    }

    // ── Class Subject Queries ───────────────────────────────────

    pub async fn list_class_subjects(
        &self,
        class_id: &str,
        period_id: &str,
    ) -> Result<Vec<class_subject::Model>, DbErr> {
        class_subject::Entity::find()
            .filter(class_subject::Column::ClassId.eq(class_id))
            .filter(class_subject::Column::PeriodId.eq(period_id))
            .filter(class_subject::Column::IsActive.eq(1))
            .all(&self.db)
            .await
    }

    pub async fn list_teacher_class_subjects(
        &self,
        teacher_id: &str,
        period_id: &str,
    ) -> Result<Vec<class_subject::Model>, DbErr> {
        class_subject::Entity::find()
            .filter(class_subject::Column::PrimaryTeacherId.eq(teacher_id))
            .filter(class_subject::Column::PeriodId.eq(period_id))
            .filter(class_subject::Column::IsActive.eq(1))
            .all(&self.db)
            .await
    }

    // ── Enrollment Queries ──────────────────────────────────────

    pub async fn list_enrollments_by_class(
        &self,
        class_id: &str,
        period_id: &str,
    ) -> Result<Vec<enrollment::Model>, DbErr> {
        enrollment::Entity::find()
            .filter(enrollment::Column::ClassId.eq(class_id))
            .filter(enrollment::Column::PeriodId.eq(period_id))
            .filter(enrollment::Column::Status.ne("withdrawn"))
            .order_by_asc(enrollment::Column::StudentNumber)
            .all(&self.db)
            .await
    }

    pub async fn get_enrollment_by_student(
        &self,
        student_id: &str,
        period_id: &str,
    ) -> Result<Option<enrollment::Model>, DbErr> {
        enrollment::Entity::find()
            .filter(enrollment::Column::StudentId.eq(student_id))
            .filter(enrollment::Column::PeriodId.eq(period_id))
            .one(&self.db)
            .await
    }

    pub async fn count_enrollments_by_class(
        &self,
        class_id: &str,
        period_id: &str,
    ) -> Result<u64, DbErr> {
        enrollment::Entity::find()
            .filter(enrollment::Column::ClassId.eq(class_id))
            .filter(enrollment::Column::PeriodId.eq(period_id))
            .filter(enrollment::Column::Status.ne("withdrawn"))
            .count(&self.db)
            .await
    }

    // ── Org Unit Queries ────────────────────────────────────────

    pub async fn list_units_by_org(&self, org_id: &str) -> Result<Vec<org_unit::Model>, DbErr> {
        org_unit::Entity::find()
            .filter(org_unit::Column::OrgId.eq(org_id))
            .filter(org_unit::Column::IsActive.eq(1))
            .order_by_asc(org_unit::Column::SortOrder)
            .all(&self.db)
            .await
    }
}
