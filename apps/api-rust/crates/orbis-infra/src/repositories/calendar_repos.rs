use async_trait::async_trait;
use orbis_app::ports::repositories::{
    AcademicPeriodRepository, AcademicYearRepository, PeriodTemplateRepository,
};
use orbis_domain::entities::academic_period::AcademicPeriod as DomainPeriod;
use orbis_domain::entities::academic_year::AcademicYear as DomainYear;
use orbis_domain::entities::period_template::PeriodTemplate as DomainTemplate;
use orbis_domain::errors::DomainError;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

use crate::models::{
    academic_period::{self, Entity as PeriodEntity},
    academic_year::{self, Entity as YearEntity},
    period_template::{self, Entity as TemplateEntity},
};

pub struct CalendarRepoImpl {
    pub db: DatabaseConnection,
}

#[async_trait]
impl AcademicYearRepository for CalendarRepoImpl {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<DomainYear>, DomainError> {
        let model = YearEntity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(model.map(|m| DomainYear {
            id: m.id,
            org_id: m.org_id,
            name: m.name,
            name_ar: m.name_ar,
            start_date: m.start_date,
            end_date: m.end_date,
            hijri_start: m.hijri_start,
            hijri_end: m.hijri_end,
            status: m.status,
            is_locked: m.is_locked,
            locked_at: m.locked_at,
            locked_by: m.locked_by,
            lock_reason: m.lock_reason,
            current_period_id: m.current_period_id,
            archived_at: m.archived_at,
            archived_by: m.archived_by,
            notes: m.notes,
            created_at: m.created_at,
            updated_at: m.updated_at,
        }))
    }

    async fn find_by_org(&self, org_id: Uuid) -> Result<Vec<DomainYear>, DomainError> {
        let models = YearEntity::find()
            .filter(academic_year::Column::OrgId.eq(org_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(models
            .into_iter()
            .map(|m| DomainYear {
                id: m.id,
                org_id: m.org_id,
                name: m.name,
                name_ar: m.name_ar,
                start_date: m.start_date,
                end_date: m.end_date,
                hijri_start: m.hijri_start,
                hijri_end: m.hijri_end,
                status: m.status,
                is_locked: m.is_locked,
                locked_at: m.locked_at,
                locked_by: m.locked_by,
                lock_reason: m.lock_reason,
                current_period_id: m.current_period_id,
                archived_at: m.archived_at,
                archived_by: m.archived_by,
                notes: m.notes,
                created_at: m.created_at,
                updated_at: m.updated_at,
            })
            .collect())
    }

    async fn find_current(&self, org_id: Uuid) -> Result<Option<DomainYear>, DomainError> {
        let model = YearEntity::find()
            .filter(academic_year::Column::OrgId.eq(org_id))
            .filter(academic_year::Column::Status.eq("active"))
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(model.map(|m| DomainYear {
            id: m.id,
            org_id: m.org_id,
            name: m.name,
            name_ar: m.name_ar,
            start_date: m.start_date,
            end_date: m.end_date,
            hijri_start: m.hijri_start,
            hijri_end: m.hijri_end,
            status: m.status,
            is_locked: m.is_locked,
            locked_at: m.locked_at,
            locked_by: m.locked_by,
            lock_reason: m.lock_reason,
            current_period_id: m.current_period_id,
            archived_at: m.archived_at,
            archived_by: m.archived_by,
            notes: m.notes,
            created_at: m.created_at,
            updated_at: m.updated_at,
        }))
    }

    async fn save(&self, year: &DomainYear) -> Result<(), DomainError> {
        let active = academic_year::ActiveModel {
            id: Set(year.id),
            org_id: Set(year.org_id),
            name: Set(year.name.clone()),
            name_ar: Set(year.name_ar.clone()),
            start_date: Set(year.start_date),
            end_date: Set(year.end_date),
            hijri_start: Set(year.hijri_start.clone()),
            hijri_end: Set(year.hijri_end.clone()),
            status: Set(year.status.clone()),
            is_locked: Set(year.is_locked),
            locked_at: Set(year.locked_at),
            locked_by: Set(year.locked_by),
            lock_reason: Set(year.lock_reason.clone()),
            current_period_id: Set(year.current_period_id),
            archived_at: Set(year.archived_at),
            archived_by: Set(year.archived_by),
            notes: Set(year.notes.clone()),
            created_at: Set(year.created_at),
            updated_at: Set(year.updated_at),
        };

        if YearEntity::find_by_id(year.id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?
            .is_some()
        {
            active
                .update(&self.db)
                .await
                .map_err(|e| DomainError::Internal(e.to_string()))?;
        } else {
            active
                .insert(&self.db)
                .await
                .map_err(|e| DomainError::Internal(e.to_string()))?;
        }
        Ok(())
    }
}

use orbis_domain::entities::academic_period::{AcademicPeriodState, PeriodKind, PeriodType};

fn str_to_period_state(s: &str) -> AcademicPeriodState {
    match s {
        "planning" => AcademicPeriodState::Planning,
        "enrollment_open" => AcademicPeriodState::EnrollmentOpen,
        "active_teaching" => AcademicPeriodState::ActiveTeaching,
        "final_exams" => AcademicPeriodState::FinalExams,
        "grade_submission" => AcademicPeriodState::GradeSubmission,
        "archived" => AcademicPeriodState::Archived,
        _ => AcademicPeriodState::Closed,
    }
}

fn map_period_model_to_domain(m: academic_period::Model) -> DomainPeriod {
    DomainPeriod {
        id: m.id,
        org_id: m.org_id,
        year_id: m.year_id,
        parent_id: m.parent_id,
        name: m.name,
        name_ar: m.name_ar,
        code: m.code,
        period_type: PeriodType::from_str(&m.period_type),
        period_kind: PeriodKind::from_str(&m.period_kind),
        state: str_to_period_state(&m.status),
        start_date: m.start_date,
        end_date: m.end_date,
        hijri_start: m.hijri_start,
        hijri_end: m.hijri_end,
        enrollment_starts_at: m.enrollment_starts_at,
        enrollment_ends_at: m.enrollment_ends_at,
        is_enrollment_open: m.is_enrollment_open,
        grade_entry_starts_at: m.grade_entry_starts_at,
        grade_entry_ends_at: m.grade_entry_ends_at,
        is_grade_entry_open: m.is_grade_entry_open,
        exam_starts_at: m.exam_starts_at,
        exam_ends_at: m.exam_ends_at,
        is_locked: m.is_locked,
        locked_at: m.locked_at,
        locked_by: m.locked_by,
        sort_order: m.sort_order,
        metadata: m.metadata,
        created_at: m.created_at,
        updated_at: m.updated_at,
    }
}

#[async_trait]
impl AcademicPeriodRepository for CalendarRepoImpl {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<DomainPeriod>, DomainError> {
        let model = PeriodEntity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(model.map(map_period_model_to_domain))
    }

    async fn find_by_year(&self, year_id: Uuid) -> Result<Vec<DomainPeriod>, DomainError> {
        let models = PeriodEntity::find()
            .filter(academic_period::Column::YearId.eq(year_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(models.into_iter().map(map_period_model_to_domain).collect())
    }

    async fn find_by_org(&self, org_id: Uuid) -> Result<Vec<DomainPeriod>, DomainError> {
        let models = PeriodEntity::find()
            .filter(academic_period::Column::OrgId.eq(org_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(models.into_iter().map(map_period_model_to_domain).collect())
    }

    async fn find_current(&self, org_id: Uuid) -> Result<Option<DomainPeriod>, DomainError> {
        let model = PeriodEntity::find()
            .filter(academic_period::Column::OrgId.eq(org_id))
            .filter(
                academic_period::Column::Status
                    .eq("active")
                    .or(academic_period::Column::Status.eq("open")),
            )
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(model.map(map_period_model_to_domain))
    }

    async fn save(&self, period: &DomainPeriod) -> Result<(), DomainError> {
        let active = academic_period::ActiveModel {
            id: Set(period.id),
            org_id: Set(period.org_id),
            year_id: Set(period.year_id),
            parent_id: Set(period.parent_id),
            name: Set(period.name.clone()),
            name_ar: Set(period.name_ar.clone()),
            code: Set(period.code.clone()),
            period_type: Set(period.period_type.to_string()),
            period_kind: Set(period.period_kind.to_string()),
            start_date: Set(period.start_date),
            end_date: Set(period.end_date),
            hijri_start: Set(period.hijri_start.clone()),
            hijri_end: Set(period.hijri_end.clone()),
            enrollment_starts_at: Set(period.enrollment_starts_at),
            enrollment_ends_at: Set(period.enrollment_ends_at),
            is_enrollment_open: Set(period.is_enrollment_open),
            grade_entry_starts_at: Set(period.grade_entry_starts_at),
            grade_entry_ends_at: Set(period.grade_entry_ends_at),
            is_grade_entry_open: Set(period.is_grade_entry_open),
            exam_starts_at: Set(period.exam_starts_at),
            exam_ends_at: Set(period.exam_ends_at),
            status: Set(period.state.to_string()),
            is_locked: Set(period.is_locked),
            locked_at: Set(period.locked_at),
            locked_by: Set(period.locked_by),
            sort_order: Set(period.sort_order),
            metadata: Set(period.metadata.clone()),
            created_at: Set(period.created_at),
            updated_at: Set(period.updated_at),
        };

        if PeriodEntity::find_by_id(period.id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?
            .is_some()
        {
            active
                .update(&self.db)
                .await
                .map_err(|e| DomainError::Internal(e.to_string()))?;
        } else {
            active
                .insert(&self.db)
                .await
                .map_err(|e| DomainError::Internal(e.to_string()))?;
        }
        Ok(())
    }

    async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        PeriodEntity::delete_by_id(id)
            .exec(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(())
    }
}

#[async_trait]
impl PeriodTemplateRepository for CalendarRepoImpl {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<DomainTemplate>, DomainError> {
        let model = TemplateEntity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(model.map(|m| DomainTemplate {
            id: m.id,
            org_id: m.org_id,
            name: m.name,
            name_ar: m.name_ar,
            institution_type: m.institution_type,
            structure: m.structure,
            is_default: m.is_default,
            created_at: m.created_at,
            updated_at: m.updated_at,
        }))
    }

    async fn find_by_org(&self, org_id: Uuid) -> Result<Vec<DomainTemplate>, DomainError> {
        let models = TemplateEntity::find()
            .filter(period_template::Column::OrgId.eq(org_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(models
            .into_iter()
            .map(|m| DomainTemplate {
                id: m.id,
                org_id: m.org_id,
                name: m.name,
                name_ar: m.name_ar,
                institution_type: m.institution_type,
                structure: m.structure,
                is_default: m.is_default,
                created_at: m.created_at,
                updated_at: m.updated_at,
            })
            .collect())
    }

    async fn save(&self, template: &DomainTemplate) -> Result<(), DomainError> {
        let active = period_template::ActiveModel {
            id: Set(template.id),
            org_id: Set(template.org_id),
            name: Set(template.name.clone()),
            name_ar: Set(template.name_ar.clone()),
            institution_type: Set(template.institution_type.clone()),
            structure: Set(template.structure.clone()),
            is_default: Set(template.is_default),
            created_at: Set(template.created_at),
            updated_at: Set(template.updated_at),
        };

        if TemplateEntity::find_by_id(template.id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?
            .is_some()
        {
            active
                .update(&self.db)
                .await
                .map_err(|e| DomainError::Internal(e.to_string()))?;
        } else {
            active
                .insert(&self.db)
                .await
                .map_err(|e| DomainError::Internal(e.to_string()))?;
        }
        Ok(())
    }
}
