use crate::entities::{grade, grade_history};
use sea_orm::{entity::prelude::*, ActiveModelTrait, ActiveValue::Set, QueryFilter, EntityTrait, QueryOrder, QuerySelect};

impl super::Repository {
    pub async fn create_grade(&self, g: grade::Model) -> Result<grade::Model, DbErr> {
        let active_model = grade::ActiveModel {
            id: Set(g.id),
            org_id: Set(g.org_id),
            cycle_id: Set(g.cycle_id),
            enrollment_id: Set(g.enrollment_id),
            assessment_id: Set(g.assessment_id),
            class_subject_id: Set(g.class_subject_id),
            subject_name: Set(g.subject_name),
            subject_name_ar: Set(g.subject_name_ar),
            score: Set(g.score),
            max_score: Set(g.max_score),
            weight: Set(g.weight),
            grade_letter: Set(g.grade_letter),
            grade_points: Set(g.grade_points),
            exam_type: Set(g.exam_type),
            semester: Set(g.semester),
            notes: Set(g.notes),
            graded_by: Set(g.graded_by),
            graded_at: Set(g.graded_at),
            is_final: Set(g.is_final),
            created_at: Set(g.created_at),
            updated_at: Set(g.updated_at),
        };
        active_model.insert(&self.db).await
    }

    pub async fn get_grades(
        &self,
        org_id: &str,
        params: crate::params::GradeParams,
    ) -> Result<Vec<grade::Model>, DbErr> {
        let mut query = grade::Entity::find()
            .filter(grade::Column::OrgId.eq(org_id));

        if let Some(ref enrollment_id) = params.enrollment_id {
            if !enrollment_id.trim().is_empty() {
                query = query.filter(grade::Column::EnrollmentId.eq(enrollment_id));
            }
        }

        if let Some(ref cycle_id) = params.cycle_id {
            if !cycle_id.trim().is_empty() {
                query = query.filter(grade::Column::CycleId.eq(cycle_id));
            }
        }

        if let Some(ref subject_name) = params.subject_name {
            if !subject_name.trim().is_empty() {
                query = query.filter(grade::Column::SubjectName.eq(subject_name));
            }
        }

        if let Some(ref exam_type) = params.exam_type {
            if !exam_type.trim().is_empty() {
                query = query.filter(grade::Column::ExamType.eq(exam_type));
            }
        }

        if let Some(ref semester) = params.semester {
            if !semester.trim().is_empty() {
                query = query.filter(grade::Column::Semester.eq(semester));
            }
        }

        if let Some(is_final) = params.is_final {
            query = query.filter(grade::Column::IsFinal.eq(is_final));
        }

        if let Some(ref assessment_id) = params.assessment_id {
            if !assessment_id.trim().is_empty() {
                query = query.filter(grade::Column::AssessmentId.eq(assessment_id));
            }
        }

        if let Some(ref class_subject_id) = params.class_subject_id {
            if !class_subject_id.trim().is_empty() {
                query = query.filter(grade::Column::ClassSubjectId.eq(class_subject_id));
            }
        }

        // If class_id is provided, find enrollments for that class and filter
        if let Some(ref class_id) = params.class_id {
            if !class_id.trim().is_empty() {
                let enrollment_ids: Vec<String> = crate::entities::enrollment::Entity::find()
                    .filter(crate::entities::enrollment::Column::ClassId.eq(class_id))
                    .all(&self.db)
                    .await?
                    .into_iter()
                    .map(|e| e.id)
                    .collect();
                query = query.filter(grade::Column::EnrollmentId.is_in(enrollment_ids));
            }
        }

        query = query.order_by_desc(grade::Column::CreatedAt);

        if let Some(limit) = params.limit {
            query = query.limit(limit);
        }
        if let Some(offset) = params.offset {
            query = query.offset(offset);
        }

        query.all(&self.db).await
    }

    pub async fn update_grade(&self, id: &str, score: f64, max_score: f64, grade_letter: Option<String>, grade_points: Option<f64>, notes: Option<String>) -> Result<grade::Model, DbErr> {
        let g = grade::Entity::find_by_id(id.to_string())
            .one(&self.db)
            .await?
            .ok_or(DbErr::RecordNotFound("grade not found".to_string()))?;

        let mut active: grade::ActiveModel = g.into();
        active.score = Set(score);
        active.max_score = Set(max_score);
        active.grade_letter = Set(grade_letter);
        active.grade_points = Set(grade_points);
        if let Some(notes_val) = notes {
            active.notes = Set(Some(notes_val));
        }
        active.updated_at = Set(chrono::Utc::now().to_rfc3339());

        active.update(&self.db).await
    }

    pub async fn get_grade_by_id(&self, id: &str) -> Result<Option<grade::Model>, DbErr> {
        grade::Entity::find_by_id(id.to_string()).one(&self.db).await
    }

    pub async fn get_grades_by_enrollment(&self, enrollment_id: &str) -> Result<Vec<grade::Model>, DbErr> {
        grade::Entity::find()
            .filter(grade::Column::EnrollmentId.eq(enrollment_id))
            .order_by_asc(grade::Column::SubjectName)
            .all(&self.db)
            .await
    }

    pub async fn get_grades_by_enrollment_final(&self, enrollment_id: &str) -> Result<Vec<grade::Model>, DbErr> {
        grade::Entity::find()
            .filter(grade::Column::EnrollmentId.eq(enrollment_id))
            .filter(grade::Column::IsFinal.eq(1))
            .order_by_asc(grade::Column::SubjectName)
            .all(&self.db)
            .await
    }

    /// Get all grades for a specific assessment (all students)
    pub async fn get_grades_by_assessment(&self, assessment_id: &str) -> Result<Vec<grade::Model>, DbErr> {
        grade::Entity::find()
            .filter(grade::Column::AssessmentId.eq(assessment_id))
            .order_by_asc(grade::Column::EnrollmentId)
            .all(&self.db)
            .await
    }

    /// Get the grade matrix: all grades for a class_subject in a cycle
    pub async fn get_grade_matrix(
        &self,
        class_subject_id: &str,
        cycle_id: &str,
    ) -> Result<Vec<grade::Model>, DbErr> {
        grade::Entity::find()
            .filter(grade::Column::ClassSubjectId.eq(class_subject_id))
            .filter(grade::Column::CycleId.eq(cycle_id))
            .order_by_asc(grade::Column::EnrollmentId)
            .order_by_asc(grade::Column::ExamType)
            .all(&self.db)
            .await
    }

    /// Find existing grade for a specific enrollment + assessment combo
    pub async fn find_grade_by_enrollment_assessment(
        &self,
        enrollment_id: &str,
        assessment_id: &str,
    ) -> Result<Option<grade::Model>, DbErr> {
        grade::Entity::find()
            .filter(grade::Column::EnrollmentId.eq(enrollment_id))
            .filter(grade::Column::AssessmentId.eq(assessment_id))
            .one(&self.db)
            .await
    }

    /// Save a grade history record for audit trail
    pub async fn create_grade_history(&self, h: grade_history::Model) -> Result<grade_history::Model, DbErr> {
        let active = grade_history::ActiveModel {
            id: Set(h.id),
            grade_id: Set(h.grade_id),
            score: Set(h.score),
            grade_letter: Set(h.grade_letter),
            is_absent: Set(h.is_absent),
            is_excused: Set(h.is_excused),
            comment: Set(h.comment),
            changed_by: Set(h.changed_by),
            changed_at: Set(h.changed_at),
            change_reason: Set(h.change_reason),
            change_type: Set(h.change_type),
            snapshot: Set(h.snapshot),
            created_at: Set(h.created_at),
        };
        active.insert(&self.db).await
    }
}
