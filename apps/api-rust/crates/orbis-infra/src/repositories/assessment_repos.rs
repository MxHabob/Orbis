use async_trait::async_trait;
use orbis_app::ports::repositories::{AssessmentSchemeRepository, GradeRepository, AttendanceRepository, BehaviorRecordRepository};
use orbis_domain::entities::assessment_scheme::AssessmentScheme as DomainAssessmentScheme;
use orbis_domain::entities::grade::Grade as DomainGrade;
use orbis_domain::entities::attendance::Attendance as DomainAttendance;
use orbis_domain::entities::behavior_record::BehaviorRecord as DomainBehaviorRecord;
use orbis_domain::errors::DomainError;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set, ColumnTrait, QueryFilter};
use uuid::Uuid;

use crate::models::{
    assessment_scheme::{self, Entity as AssessmentSchemeEntity},
    grade::{self, Entity as GradeEntity},
    attendance::{self, Entity as AttendanceEntity},
    behavior_record::{self, Entity as BehaviorRecordEntity},
};

pub struct AssessmentRepoImpl {
    pub db: DatabaseConnection,
}

#[async_trait]
impl AssessmentSchemeRepository for AssessmentRepoImpl {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<DomainAssessmentScheme>, DomainError> {
        let model = AssessmentSchemeEntity::find_by_id(id).one(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(model.map(|m| DomainAssessmentScheme {
            id: m.id, org_id: m.org_id, name: m.name, description: m.description,
            max_score: m.max_score, pass_score: m.pass_score, formula: m.formula,
            created_at: m.created_at, updated_at: m.updated_at
        }))
    }

    async fn save(&self, scheme: &DomainAssessmentScheme) -> Result<(), DomainError> {
        let active = assessment_scheme::ActiveModel {
            id: Set(scheme.id), org_id: Set(scheme.org_id), name: Set(scheme.name.clone()),
            description: Set(scheme.description.clone()), max_score: Set(scheme.max_score),
            pass_score: Set(scheme.pass_score), formula: Set(scheme.formula.clone()), created_at: Set(scheme.created_at),
            updated_at: Set(scheme.updated_at),
        };

        if AssessmentSchemeEntity::find_by_id(scheme.id).one(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?.is_some() {
            active.update(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        } else {
            active.insert(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        }
        Ok(())
    }
}

#[async_trait]
impl GradeRepository for AssessmentRepoImpl {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<DomainGrade>, DomainError> {
        let model = GradeEntity::find_by_id(id).one(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(model.map(|m| DomainGrade {
            id: m.id, student_id: m.student_id, class_subject_id: m.class_subject_id,
            scheme_id: m.scheme_id, score: m.score, is_excused: m.is_excused,
            notes: m.notes, created_at: m.created_at, updated_at: m.updated_at
        }))
    }

    async fn find_by_student(&self, student_id: Uuid) -> Result<Vec<DomainGrade>, DomainError> {
        let models = GradeEntity::find().filter(grade::Column::StudentId.eq(student_id)).all(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(models.into_iter().map(|m| DomainGrade {
            id: m.id, student_id: m.student_id, class_subject_id: m.class_subject_id,
            scheme_id: m.scheme_id, score: m.score, is_excused: m.is_excused,
            notes: m.notes, created_at: m.created_at, updated_at: m.updated_at
        }).collect())
    }

    async fn save(&self, grade: &DomainGrade) -> Result<(), DomainError> {
        let active = grade::ActiveModel {
            id: Set(grade.id), student_id: Set(grade.student_id),
            class_subject_id: Set(grade.class_subject_id), scheme_id: Set(grade.scheme_id),
            score: Set(grade.score), is_excused: Set(grade.is_excused),
            notes: Set(grade.notes.clone()), created_at: Set(grade.created_at),
            updated_at: Set(grade.updated_at),
        };

        if GradeEntity::find_by_id(grade.id).one(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?.is_some() {
            active.update(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        } else {
            active.insert(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        }
        Ok(())
    }
}

#[async_trait]
impl AttendanceRepository for AssessmentRepoImpl {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<DomainAttendance>, DomainError> {
        let model = AttendanceEntity::find_by_id(id).one(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(model.map(|m| DomainAttendance {
            id: m.id, student_id: m.student_id, class_id: m.class_id,
            class_subject_id: m.class_subject_id,
            scheduled_lesson_id: m.scheduled_lesson_id,
            date: m.date, status: m.status, notes: m.notes,
            created_at: m.created_at, updated_at: m.updated_at
        }))
    }

    async fn save(&self, attendance: &DomainAttendance) -> Result<(), DomainError> {
        let active = attendance::ActiveModel {
            id: Set(attendance.id), student_id: Set(attendance.student_id),
            class_id: Set(attendance.class_id),
            class_subject_id: Set(attendance.class_subject_id),
            scheduled_lesson_id: Set(attendance.scheduled_lesson_id), date: Set(attendance.date),
            status: Set(attendance.status.clone()), notes: Set(attendance.notes.clone()),
            created_at: Set(attendance.created_at), updated_at: Set(attendance.updated_at),
        };

        if AttendanceEntity::find_by_id(attendance.id).one(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?.is_some() {
            active.update(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        } else {
            active.insert(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        }
        Ok(())
    }
}

#[async_trait]
impl BehaviorRecordRepository for AssessmentRepoImpl {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<DomainBehaviorRecord>, DomainError> {
        let model = BehaviorRecordEntity::find_by_id(id).one(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(model.map(|m| DomainBehaviorRecord {
            id: m.id, student_id: m.student_id, org_id: m.org_id,
            reporter_id: m.reporter_id, record_type: m.record_type,
            description: m.description, points: m.points,
            created_at: m.created_at, updated_at: m.updated_at
        }))
    }

    async fn save(&self, record: &DomainBehaviorRecord) -> Result<(), DomainError> {
        let active = behavior_record::ActiveModel {
            id: Set(record.id), student_id: Set(record.student_id),
            org_id: Set(record.org_id), reporter_id: Set(record.reporter_id),
            record_type: Set(record.record_type.clone()), description: Set(record.description.clone()),
            points: Set(record.points), created_at: Set(record.created_at),
            updated_at: Set(record.updated_at),
        };

        if BehaviorRecordEntity::find_by_id(record.id).one(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?.is_some() {
            active.update(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        } else {
            active.insert(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        }
        Ok(())
    }
}
