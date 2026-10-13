use async_trait::async_trait;
use orbis_app::ports::repositories::{
    AssessmentSchemeRepository, AttendanceRepository, BehaviorRecordRepository, GradeRepository,
};
use orbis_domain::entities::assessment_scheme::AssessmentScheme as DomainAssessmentScheme;
use orbis_domain::entities::attendance::Attendance as DomainAttendance;
use orbis_domain::entities::behavior_record::BehaviorRecord as DomainBehaviorRecord;
use orbis_domain::entities::grade::Grade as DomainGrade;
use orbis_domain::errors::DomainError;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    Set,
};
use uuid::Uuid;

use crate::models::{
    assessment_scheme::{self, Entity as AssessmentSchemeEntity},
    attendance::{self, Entity as AttendanceEntity},
    behavior_record::{self, Entity as BehaviorRecordEntity},
    grade::{self, Entity as GradeEntity},
};

use orbis_domain::entities::assessment_scheme::{
    AssessmentComponent, AssessmentLevel, InstitutionScope,
};

pub struct AssessmentRepoImpl {
    pub db: DatabaseConnection,
}

fn map_scheme_model_to_domain(m: assessment_scheme::Model) -> DomainAssessmentScheme {
    let components: Vec<AssessmentComponent> =
        serde_json::from_str(&m.components_json).unwrap_or_default();
    DomainAssessmentScheme {
        id: m.id,
        org_id: m.org_id,
        name: m.name,
        name_ar: m.name_ar,
        description: m.description,
        max_score: rust_decimal::Decimal::from_f64_retain(m.max_score).unwrap_or_default(),
        formula: m.formula,
        level: AssessmentLevel::from_str(&m.level),
        institution_scope: InstitutionScope::from_str(&m.institution_scope),
        components,
        is_default: m.is_default,
        is_system: m.is_system,
        created_at: m.created_at,
        updated_at: m.updated_at,
    }
}

#[async_trait]
impl AssessmentSchemeRepository for AssessmentRepoImpl {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<DomainAssessmentScheme>, DomainError> {
        let model = AssessmentSchemeEntity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(model.map(map_scheme_model_to_domain))
    }

    async fn find_by_org(&self, org_id: Uuid) -> Result<Vec<DomainAssessmentScheme>, DomainError> {
        let models = AssessmentSchemeEntity::find()
            .filter(assessment_scheme::Column::OrgId.eq(org_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(models.into_iter().map(map_scheme_model_to_domain).collect())
    }

    async fn save(&self, scheme: &DomainAssessmentScheme) -> Result<(), DomainError> {
        use rust_decimal::prelude::ToPrimitive;
        let active = assessment_scheme::ActiveModel {
            id: Set(scheme.id),
            org_id: Set(scheme.org_id),
            name: Set(scheme.name.clone()),
            name_ar: Set(scheme.name_ar.clone()),
            description: Set(scheme.description.clone()),
            max_score: Set(scheme.max_score.to_f64().unwrap_or(100.0)),
            formula: Set(scheme.formula.clone()),
            level: Set(scheme.level.to_string()),
            institution_scope: Set(scheme.institution_scope.to_string()),
            components_json: Set(
                serde_json::to_string(&scheme.components).unwrap_or_else(|_| "[]".into())
            ),
            is_default: Set(scheme.is_default),
            is_system: Set(scheme.is_system),
            created_at: Set(scheme.created_at),
            updated_at: Set(scheme.updated_at),
        };

        if AssessmentSchemeEntity::find_by_id(scheme.id)
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
        AssessmentSchemeEntity::delete_by_id(id)
            .exec(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(())
    }
}

#[async_trait]
impl GradeRepository for AssessmentRepoImpl {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<DomainGrade>, DomainError> {
        let model = GradeEntity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(model.map(map_grade_model_to_domain))
    }

    async fn find_by_student(&self, student_id: Uuid) -> Result<Vec<DomainGrade>, DomainError> {
        let models = GradeEntity::find()
            .filter(grade::Column::StudentId.eq(student_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(models.into_iter().map(map_grade_model_to_domain).collect())
    }

    async fn find_by_student_and_subject(
        &self,
        student_id: Uuid,
        cs_id: Uuid,
    ) -> Result<Vec<DomainGrade>, DomainError> {
        let models = GradeEntity::find()
            .filter(grade::Column::StudentId.eq(student_id))
            .filter(grade::Column::SectionSubjectId.eq(cs_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        Ok(models.into_iter().map(map_grade_model_to_domain).collect())
    }

    async fn find_by_student_and_period(
        &self,
        student_id: Uuid,
        period_id: Uuid,
    ) -> Result<Vec<DomainGrade>, DomainError> {
        let models = GradeEntity::find()
            .filter(grade::Column::StudentId.eq(student_id))
            .filter(grade::Column::PeriodId.eq(period_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(models.into_iter().map(map_grade_model_to_domain).collect())
    }

    async fn find_by_enrollment(
        &self,
        enrollment_id: Uuid,
    ) -> Result<Vec<DomainGrade>, DomainError> {
        let models = GradeEntity::find()
            .filter(grade::Column::EnrollmentId.eq(enrollment_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(models.into_iter().map(map_grade_model_to_domain).collect())
    }

    async fn find_by_section_subject(
        &self,
        section_subject_id: Uuid,
    ) -> Result<Vec<DomainGrade>, DomainError> {
        let models = GradeEntity::find()
            .filter(grade::Column::SectionSubjectId.eq(section_subject_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(models.into_iter().map(map_grade_model_to_domain).collect())
    }

    async fn list(
        &self,
        org_id: Uuid,
        filter: orbis_domain::queries::filters::GradeFilter,
    ) -> Result<orbis_domain::queries::filters::PaginatedList<DomainGrade>, DomainError> {
        let mut query = GradeEntity::find().filter(grade::Column::OrgId.eq(org_id));
        if let Some(student_id) = filter.student_id {
            query = query.filter(grade::Column::StudentId.eq(student_id));
        }
        if let Some(period_id) = filter.period_id {
            query = query.filter(grade::Column::PeriodId.eq(period_id));
        }
        if let Some(section_subject_id) = filter.section_subject_id {
            query = query.filter(grade::Column::SectionSubjectId.eq(section_subject_id));
        }
        let pagination = filter.pagination.unwrap_or_default();
        let page = pagination.page();
        let page_size = pagination.page_size();
        let paginator = query.paginate(&self.db, page_size);
        let total_count = paginator
            .num_items()
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        let rows = paginator
            .fetch_page(page - 1)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        let items = rows.into_iter().map(map_grade_model_to_domain).collect();
        Ok(orbis_domain::queries::filters::PaginatedList::new(
            items,
            total_count,
            page,
            page_size,
        ))
    }

    async fn save(&self, grade: &DomainGrade) -> Result<(), DomainError> {
        let active = grade::ActiveModel {
            id: Set(grade.id),
            org_id: Set(grade.org_id),
            period_id: Set(Some(grade.period_id)),
            student_id: Set(grade.student_id),
            enrollment_id: Set(grade.enrollment_id),
            section_subject_id: Set(grade.section_subject_id),
            component_id: Set(grade.component_id),
            score: Set(grade.score),
            max_score: Set(Some(grade.max_score)),
            weight: Set(grade.weight),
            letter_grade: Set(grade.letter_grade.clone()),
            is_final: Set(Some(grade.is_final)),
            is_excused: Set(grade.is_excused),
            is_exempt: Set(Some(grade.is_exempt)),
            change_reason: Set(grade.change_reason.clone()),
            recorded_by: Set(grade.recorded_by),
            updated_by: Set(grade.updated_by),
            notes: Set(grade.notes.clone()),
            created_at: Set(grade.created_at),
            updated_at: Set(grade.updated_at),
        };

        if GradeEntity::find_by_id(grade.id)
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

    async fn save_batch(&self, grades: &[DomainGrade]) -> Result<(), DomainError> {
        for g in grades {
            GradeRepository::save(self, g).await?;
        }
        Ok(())
    }

    async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        GradeEntity::delete_by_id(id)
            .exec(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(())
    }
}

fn map_attendance_model_to_domain(m: attendance::Model) -> DomainAttendance {
    DomainAttendance {
        id: m.id,
        student_id: m.student_id,
        section_id: m.section_id,
        section_subject_id: m.section_subject_id,
        scheduled_lesson_id: m.scheduled_lesson_id,
        attendance_date: m.attendance_date,
        status: m.status,
        notes: m.notes,
        marked_by: m.marked_by,
        created_at: m.created_at,
        updated_at: m.updated_at,
    }
}

#[async_trait]
impl AttendanceRepository for AssessmentRepoImpl {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<DomainAttendance>, DomainError> {
        let model = AttendanceEntity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(model.map(map_attendance_model_to_domain))
    }

    async fn find_by_section_and_date(
        &self,
        section_id: Uuid,
        date: chrono::NaiveDate,
    ) -> Result<Vec<DomainAttendance>, DomainError> {
        let models = AttendanceEntity::find()
            .filter(attendance::Column::SectionId.eq(section_id))
            .filter(attendance::Column::AttendanceDate.eq(date))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(models
            .into_iter()
            .map(map_attendance_model_to_domain)
            .collect())
    }

    async fn find_by_student(
        &self,
        student_id: Uuid,
    ) -> Result<Vec<DomainAttendance>, DomainError> {
        let models = AttendanceEntity::find()
            .filter(attendance::Column::StudentId.eq(student_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(models
            .into_iter()
            .map(map_attendance_model_to_domain)
            .collect())
    }

    async fn list(
        &self,
        _org_id: Uuid,
        filter: orbis_domain::queries::filters::AttendanceFilter,
    ) -> Result<orbis_domain::queries::filters::PaginatedList<DomainAttendance>, DomainError> {
        let mut query = AttendanceEntity::find();
        if let Some(student_id) = filter.student_id {
            query = query.filter(attendance::Column::StudentId.eq(student_id));
        }
        if let Some(section_id) = filter.section_id {
            query = query.filter(attendance::Column::SectionId.eq(section_id));
        }
        if let Some(date) = filter.date {
            query = query.filter(attendance::Column::AttendanceDate.eq(date));
        }
        let pagination = filter.pagination.unwrap_or_default();
        let page = pagination.page();
        let page_size = pagination.page_size();
        let paginator = query.paginate(&self.db, page_size);
        let total_count = paginator
            .num_items()
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        let rows = paginator
            .fetch_page(page - 1)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        let items = rows
            .into_iter()
            .map(map_attendance_model_to_domain)
            .collect();
        Ok(orbis_domain::queries::filters::PaginatedList::new(
            items,
            total_count,
            page,
            page_size,
        ))
    }

    async fn save(&self, attendance: &DomainAttendance) -> Result<(), DomainError> {
        let active = attendance::ActiveModel {
            id: Set(attendance.id),
            org_id: Set(None),
            student_id: Set(attendance.student_id),
            section_id: Set(attendance.section_id),
            section_subject_id: Set(attendance.section_subject_id),
            scheduled_lesson_id: Set(attendance.scheduled_lesson_id),
            attendance_date: Set(attendance.attendance_date),
            status: Set(attendance.status.clone()),
            notes: Set(attendance.notes.clone()),
            marked_by: Set(attendance.marked_by),
            created_at: Set(attendance.created_at),
            updated_at: Set(attendance.updated_at),
        };

        if AttendanceEntity::find_by_id(attendance.id)
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

    async fn save_batch(&self, attendances: &[DomainAttendance]) -> Result<(), DomainError> {
        for a in attendances {
            AttendanceRepository::save(self, a).await?;
        }
        Ok(())
    }

    async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        AttendanceEntity::delete_by_id(id)
            .exec(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(())
    }
}

fn map_grade_model_to_domain(m: grade::Model) -> DomainGrade {
    DomainGrade {
        id: m.id,
        org_id: m.org_id,
        period_id: m.period_id.unwrap_or_default(),
        student_id: m.student_id,
        enrollment_id: m.enrollment_id,
        section_subject_id: m.section_subject_id,
        component_id: m.component_id,
        score: m.score,
        max_score: m
            .max_score
            .unwrap_or_else(|| rust_decimal::Decimal::from(100)),
        weight: m.weight,
        letter_grade: m.letter_grade,
        is_final: m.is_final.unwrap_or(false),
        is_excused: m.is_excused,
        is_exempt: m.is_exempt.unwrap_or(false),
        change_reason: m.change_reason,
        recorded_by: m.recorded_by,
        updated_by: m.updated_by,
        notes: m.notes,
        created_at: m.created_at,
        updated_at: m.updated_at,
    }
}

#[async_trait]
impl BehaviorRecordRepository for AssessmentRepoImpl {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<DomainBehaviorRecord>, DomainError> {
        let model = BehaviorRecordEntity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(model.map(|m| DomainBehaviorRecord {
            id: m.id,
            student_id: m.student_id,
            org_id: m.org_id,
            reporter_id: m.reporter_id,
            record_type: m.record_type,
            description: m.description,
            points: m.points,
            created_at: m.created_at,
            updated_at: m.updated_at,
        }))
    }

    async fn find_by_student(
        &self,
        student_id: Uuid,
    ) -> Result<Vec<DomainBehaviorRecord>, DomainError> {
        let models = BehaviorRecordEntity::find()
            .filter(behavior_record::Column::StudentId.eq(student_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(models
            .into_iter()
            .map(|m| DomainBehaviorRecord {
                id: m.id,
                student_id: m.student_id,
                org_id: m.org_id,
                reporter_id: m.reporter_id,
                record_type: m.record_type,
                description: m.description,
                points: m.points,
                created_at: m.created_at,
                updated_at: m.updated_at,
            })
            .collect())
    }

    async fn find_by_org(&self, org_id: Uuid) -> Result<Vec<DomainBehaviorRecord>, DomainError> {
        let models = BehaviorRecordEntity::find()
            .filter(behavior_record::Column::OrgId.eq(org_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(models
            .into_iter()
            .map(|m| DomainBehaviorRecord {
                id: m.id,
                student_id: m.student_id,
                org_id: m.org_id,
                reporter_id: m.reporter_id,
                record_type: m.record_type,
                description: m.description,
                points: m.points,
                created_at: m.created_at,
                updated_at: m.updated_at,
            })
            .collect())
    }

    async fn save(&self, record: &DomainBehaviorRecord) -> Result<(), DomainError> {
        let active = behavior_record::ActiveModel {
            id: Set(record.id),
            student_id: Set(record.student_id),
            org_id: Set(record.org_id),
            reporter_id: Set(record.reporter_id),
            record_type: Set(record.record_type.clone()),
            description: Set(record.description.clone()),
            points: Set(record.points),
            created_at: Set(record.created_at),
            updated_at: Set(record.updated_at),
        };

        if BehaviorRecordEntity::find_by_id(record.id)
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
        BehaviorRecordEntity::delete_by_id(id)
            .exec(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(())
    }
}

#[async_trait]
impl orbis_app::ports::repositories::GpaRecordRepository for AssessmentRepoImpl {
    async fn find_by_student(
        &self,
        student_id: Uuid,
    ) -> Result<Vec<orbis_domain::entities::gpa_record::CumulativeGpaRecord>, DomainError> {
        let models = crate::models::gpa_record::Entity::find()
            .filter(crate::models::gpa_record::Column::StudentId.eq(student_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        Ok(models
            .into_iter()
            .map(
                |m| orbis_domain::entities::gpa_record::CumulativeGpaRecord {
                    id: m.id,
                    org_id: m.org_id,
                    student_id: m.student_id,
                    period_id: m.period_id,
                    term_attempted_credits: m.term_attempted_credits,
                    term_earned_credits: m.term_earned_credits,
                    term_points: m.term_points,
                    term_gpa: m.term_gpa,
                    cumulative_attempted_credits: m.cumulative_attempted_credits,
                    cumulative_earned_credits: m.cumulative_earned_credits,
                    cumulative_points: m.cumulative_points,
                    cumulative_gpa: m.cumulative_gpa,
                    standing: serde_json::from_str(&format!("\"{}\"", m.standing))
                        .unwrap_or(orbis_domain::entities::student::AcademicStanding::GoodStanding),
                    calculated_at: m.calculated_at,
                },
            )
            .collect())
    }

    async fn find_by_student_and_period(
        &self,
        student_id: Uuid,
        period_id: Uuid,
    ) -> Result<Option<orbis_domain::entities::gpa_record::CumulativeGpaRecord>, DomainError> {
        let model = crate::models::gpa_record::Entity::find()
            .filter(crate::models::gpa_record::Column::StudentId.eq(student_id))
            .filter(crate::models::gpa_record::Column::PeriodId.eq(period_id))
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        Ok(model.map(
            |m| orbis_domain::entities::gpa_record::CumulativeGpaRecord {
                id: m.id,
                org_id: m.org_id,
                student_id: m.student_id,
                period_id: m.period_id,
                term_attempted_credits: m.term_attempted_credits,
                term_earned_credits: m.term_earned_credits,
                term_points: m.term_points,
                term_gpa: m.term_gpa,
                cumulative_attempted_credits: m.cumulative_attempted_credits,
                cumulative_earned_credits: m.cumulative_earned_credits,
                cumulative_points: m.cumulative_points,
                cumulative_gpa: m.cumulative_gpa,
                standing: serde_json::from_str(&format!("\"{}\"", m.standing))
                    .unwrap_or(orbis_domain::entities::student::AcademicStanding::GoodStanding),
                calculated_at: m.calculated_at,
            },
        ))
    }

    async fn save(
        &self,
        record: &orbis_domain::entities::gpa_record::CumulativeGpaRecord,
    ) -> Result<(), DomainError> {
        let active = crate::models::gpa_record::ActiveModel {
            id: Set(record.id),
            org_id: Set(record.org_id),
            student_id: Set(record.student_id),
            period_id: Set(record.period_id),
            term_attempted_credits: Set(record.term_attempted_credits),
            term_earned_credits: Set(record.term_earned_credits),
            term_points: Set(record.term_points),
            term_gpa: Set(record.term_gpa),
            cumulative_attempted_credits: Set(record.cumulative_attempted_credits),
            cumulative_earned_credits: Set(record.cumulative_earned_credits),
            cumulative_points: Set(record.cumulative_points),
            cumulative_gpa: Set(record.cumulative_gpa),
            standing: Set(format!("{:?}", record.standing)),
            calculated_at: Set(record.calculated_at),
        };

        if crate::models::gpa_record::Entity::find_by_id(record.id)
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
