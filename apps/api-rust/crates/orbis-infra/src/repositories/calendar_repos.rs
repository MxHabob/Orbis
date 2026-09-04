use async_trait::async_trait;
use orbis_app::ports::repositories::{AcademicYearRepository, AcademicPeriodRepository, PeriodTemplateRepository};
use orbis_domain::entities::academic_year::AcademicYear as DomainYear;
use orbis_domain::entities::academic_period::AcademicPeriod as DomainPeriod;
use orbis_domain::entities::period_template::PeriodTemplate as DomainTemplate;
use orbis_domain::errors::DomainError;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set, ColumnTrait, QueryFilter};
use uuid::Uuid;

use crate::models::{
    academic_year::{self, Entity as YearEntity},
    academic_period::{self, Entity as PeriodEntity},
    period_template::{self, Entity as TemplateEntity},
};

pub struct CalendarRepoImpl {
    pub db: DatabaseConnection,
}

#[async_trait]
impl AcademicYearRepository for CalendarRepoImpl {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<DomainYear>, DomainError> {
        let model = YearEntity::find_by_id(id).one(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(model.map(|m| DomainYear {
            id: m.id, org_id: m.org_id, name: m.name, name_ar: m.name_ar,
            start_date: m.start_date, end_date: m.end_date, hijri_start: m.hijri_start,
            hijri_end: m.hijri_end, status: m.status, is_locked: m.is_locked,
            locked_at: m.locked_at, locked_by: m.locked_by, lock_reason: m.lock_reason,
            current_period_id: m.current_period_id, archived_at: m.archived_at,
            archived_by: m.archived_by, notes: m.notes, created_at: m.created_at, updated_at: m.updated_at
        }))
    }

    async fn find_by_org(&self, org_id: Uuid) -> Result<Vec<DomainYear>, DomainError> {
        let models = YearEntity::find().filter(academic_year::Column::OrgId.eq(org_id)).all(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(models.into_iter().map(|m| DomainYear {
            id: m.id, org_id: m.org_id, name: m.name, name_ar: m.name_ar,
            start_date: m.start_date, end_date: m.end_date, hijri_start: m.hijri_start,
            hijri_end: m.hijri_end, status: m.status, is_locked: m.is_locked,
            locked_at: m.locked_at, locked_by: m.locked_by, lock_reason: m.lock_reason,
            current_period_id: m.current_period_id, archived_at: m.archived_at,
            archived_by: m.archived_by, notes: m.notes, created_at: m.created_at, updated_at: m.updated_at
        }).collect())
    }

    async fn save(&self, year: &DomainYear) -> Result<(), DomainError> {
        let active = academic_year::ActiveModel {
            id: Set(year.id), org_id: Set(year.org_id), name: Set(year.name.clone()),
            name_ar: Set(year.name_ar.clone()), start_date: Set(year.start_date),
            end_date: Set(year.end_date), hijri_start: Set(year.hijri_start.clone()),
            hijri_end: Set(year.hijri_end.clone()), status: Set(year.status.clone()),
            is_locked: Set(year.is_locked), locked_at: Set(year.locked_at),
            locked_by: Set(year.locked_by), lock_reason: Set(year.lock_reason.clone()),
            current_period_id: Set(year.current_period_id), archived_at: Set(year.archived_at),
            archived_by: Set(year.archived_by), notes: Set(year.notes.clone()),
            created_at: Set(year.created_at), updated_at: Set(year.updated_at),
        };

        if YearEntity::find_by_id(year.id).one(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?.is_some() {
            active.update(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        } else {
            active.insert(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        }
        Ok(())
    }
}

#[async_trait]
impl AcademicPeriodRepository for CalendarRepoImpl {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<DomainPeriod>, DomainError> {
        let model = PeriodEntity::find_by_id(id).one(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(model.map(|m| DomainPeriod {
            id: m.id, org_id: m.org_id, year_id: m.year_id, parent_id: m.parent_id,
            name: m.name, name_ar: m.name_ar, code: m.code, period_type: m.period_type,
            start_date: m.start_date, end_date: m.end_date, hijri_start: m.hijri_start,
            hijri_end: m.hijri_end, enrollment_start: m.enrollment_start,
            enrollment_end: m.enrollment_end, is_enrollment_open: m.is_enrollment_open,
            grade_entry_start: m.grade_entry_start, grade_entry_end: m.grade_entry_end,
            is_grade_entry_open: m.is_grade_entry_open, exam_start: m.exam_start,
            exam_end: m.exam_end, scale_id: m.scale_id, pass_fail_policy_id: m.pass_fail_policy_id,
            status: m.status, is_locked: m.is_locked, locked_at: m.locked_at,
            locked_by: m.locked_by, sort_order: m.sort_order, metadata: m.metadata,
            created_at: m.created_at, updated_at: m.updated_at
        }))
    }

    async fn find_by_year(&self, year_id: Uuid) -> Result<Vec<DomainPeriod>, DomainError> {
        let models = PeriodEntity::find().filter(academic_period::Column::YearId.eq(year_id)).all(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(models.into_iter().map(|m| DomainPeriod {
            id: m.id, org_id: m.org_id, year_id: m.year_id, parent_id: m.parent_id,
            name: m.name, name_ar: m.name_ar, code: m.code, period_type: m.period_type,
            start_date: m.start_date, end_date: m.end_date, hijri_start: m.hijri_start,
            hijri_end: m.hijri_end, enrollment_start: m.enrollment_start,
            enrollment_end: m.enrollment_end, is_enrollment_open: m.is_enrollment_open,
            grade_entry_start: m.grade_entry_start, grade_entry_end: m.grade_entry_end,
            is_grade_entry_open: m.is_grade_entry_open, exam_start: m.exam_start,
            exam_end: m.exam_end, scale_id: m.scale_id, pass_fail_policy_id: m.pass_fail_policy_id,
            status: m.status, is_locked: m.is_locked, locked_at: m.locked_at,
            locked_by: m.locked_by, sort_order: m.sort_order, metadata: m.metadata,
            created_at: m.created_at, updated_at: m.updated_at
        }).collect())
    }

    async fn save(&self, period: &DomainPeriod) -> Result<(), DomainError> {
        let active = academic_period::ActiveModel {
            id: Set(period.id), org_id: Set(period.org_id), year_id: Set(period.year_id),
            parent_id: Set(period.parent_id), name: Set(period.name.clone()),
            name_ar: Set(period.name_ar.clone()), code: Set(period.code.clone()),
            period_type: Set(period.period_type.clone()), start_date: Set(period.start_date),
            end_date: Set(period.end_date), hijri_start: Set(period.hijri_start.clone()),
            hijri_end: Set(period.hijri_end.clone()), enrollment_start: Set(period.enrollment_start),
            enrollment_end: Set(period.enrollment_end), is_enrollment_open: Set(period.is_enrollment_open),
            grade_entry_start: Set(period.grade_entry_start), grade_entry_end: Set(period.grade_entry_end),
            is_grade_entry_open: Set(period.is_grade_entry_open), exam_start: Set(period.exam_start),
            exam_end: Set(period.exam_end), scale_id: Set(period.scale_id),
            pass_fail_policy_id: Set(period.pass_fail_policy_id), status: Set(period.status.clone()),
            is_locked: Set(period.is_locked), locked_at: Set(period.locked_at),
            locked_by: Set(period.locked_by), sort_order: Set(period.sort_order),
            metadata: Set(period.metadata.clone()), created_at: Set(period.created_at),
            updated_at: Set(period.updated_at),
        };

        if PeriodEntity::find_by_id(period.id).one(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?.is_some() {
            active.update(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        } else {
            active.insert(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        }
        Ok(())
    }
}

#[async_trait]
impl PeriodTemplateRepository for CalendarRepoImpl {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<DomainTemplate>, DomainError> {
        let model = TemplateEntity::find_by_id(id).one(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(model.map(|m| DomainTemplate {
            id: m.id, org_id: m.org_id, name: m.name, name_ar: m.name_ar,
            institution_type: m.institution_type, structure: m.structure,
            is_default: m.is_default, created_at: m.created_at, updated_at: m.updated_at
        }))
    }

    async fn find_by_org(&self, org_id: Uuid) -> Result<Vec<DomainTemplate>, DomainError> {
        let models = TemplateEntity::find().filter(period_template::Column::OrgId.eq(org_id)).all(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(models.into_iter().map(|m| DomainTemplate {
            id: m.id, org_id: m.org_id, name: m.name, name_ar: m.name_ar,
            institution_type: m.institution_type, structure: m.structure,
            is_default: m.is_default, created_at: m.created_at, updated_at: m.updated_at
        }).collect())
    }

    async fn save(&self, template: &DomainTemplate) -> Result<(), DomainError> {
        let active = period_template::ActiveModel {
            id: Set(template.id), org_id: Set(template.org_id), name: Set(template.name.clone()),
            name_ar: Set(template.name_ar.clone()), institution_type: Set(template.institution_type.clone()),
            structure: Set(template.structure.clone()), is_default: Set(template.is_default),
            created_at: Set(template.created_at), updated_at: Set(template.updated_at),
        };

        if TemplateEntity::find_by_id(template.id).one(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?.is_some() {
            active.update(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        } else {
            active.insert(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        }
        Ok(())
    }
}
