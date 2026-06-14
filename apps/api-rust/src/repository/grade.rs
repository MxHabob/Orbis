use crate::entities::grade;
use sea_orm::{entity::prelude::*, ActiveModelTrait, ActiveValue::Set, QueryFilter, EntityTrait, QueryOrder, QuerySelect};

impl super::Repository {
    pub async fn create_grade(&self, g: grade::Model) -> Result<grade::Model, DbErr> {
        let active_model = grade::ActiveModel {
            id: Set(g.id),
            org_id: Set(g.org_id),
            cycle_id: Set(g.cycle_id),
            enrollment_id: Set(g.enrollment_id),
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
        params: crate::handler::params::GradeParams,
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
}
