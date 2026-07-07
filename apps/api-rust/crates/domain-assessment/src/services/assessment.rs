//! Assessment service — manage class assessment items (exams, quizzes, homework).

use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, EntityTrait, QueryFilter, ColumnTrait};

use shared_kernel::errors::AppError;
use crate::entities::assessment;
use crate::repository::{AssessmentCommandRepo, AssessmentQueryRepo};

use domain_academic::entities::class_subject;
use domain_lifecycle::entities::academic_cycle;

#[derive(Serialize, Deserialize, utoipa::ToSchema, Clone, Debug)]
pub struct CreateAssessmentRequest {
    pub class_id: String,
    pub subject_id: String,
    pub cycle_id: String,
    pub title: String,
    pub title_ar: Option<String>,
    pub item_type: String,
    pub max_score: f64,
    pub weight: Option<f64>,
    pub grade_group_name: Option<String>,
    pub assessment_date: Option<String>,
    pub due_date: Option<String>,
}

#[derive(Serialize, Deserialize, utoipa::ToSchema, Clone, Debug)]
pub struct UpdateAssessmentRequest {
    pub title: Option<String>,
    pub title_ar: Option<String>,
    pub item_type: Option<String>,
    pub max_score: Option<f64>,
    pub weight: Option<f64>,
    pub grade_group_name: Option<String>,
    pub assessment_date: Option<String>,
    pub due_date: Option<String>,
    pub is_published: Option<i32>,
    pub is_locked: Option<i32>,
}

pub struct AssessmentService {
    pub cmd: Arc<AssessmentCommandRepo>,
    pub query: Arc<AssessmentQueryRepo>,
}

impl AssessmentService {
    pub fn new(cmd: Arc<AssessmentCommandRepo>, query: Arc<AssessmentQueryRepo>) -> Self {
        Self { cmd, query }
    }

    /// Retrieve all assessments for a class and subject.
    pub async fn get_assessments(
        &self,
        class_id: &str,
        subject_id: &str,
        cycle_id: &str,
    ) -> Result<Vec<assessment::Model>, AppError> {
        let cs = class_subject::Entity::find()
            .filter(class_subject::Column::ClassId.eq(class_id))
            .filter(class_subject::Column::SubjectId.eq(subject_id))
            .filter(class_subject::Column::IsActive.eq(1))
            .one(&self.query.db)
            .await?;

        match cs {
            Some(c) => Ok(self.query.get_assessments(&c.id, cycle_id).await?),
            None => Ok(vec![]),
        }
    }

    /// Get a single assessment.
    pub async fn get_assessment(&self, id: &str) -> Result<assessment::Model, AppError> {
        self.query.get_assessment_by_id(id).await?
            .ok_or_else(|| AppError::not_found("assessment", id))
    }

    /// Create a new assessment.
    pub async fn create_assessment(
        &self,
        org_id: &str,
        user_id: &str,
        req: CreateAssessmentRequest,
    ) -> Result<assessment::Model, AppError> {
        let cycle = academic_cycle::Entity::find_by_id(req.cycle_id.clone())
            .one(&self.query.db)
            .await?
            .ok_or_else(|| AppError::not_found("academic_cycle", &req.cycle_id))?;

        if cycle.is_locked == 1 {
            return Err(AppError::unauthorized("Cycle is locked"));
        }

        let class_subject_id = match class_subject::Entity::find()
            .filter(class_subject::Column::ClassId.eq(&req.class_id))
            .filter(class_subject::Column::SubjectId.eq(&req.subject_id))
            .filter(class_subject::Column::IsActive.eq(1))
            .one(&self.query.db)
            .await?
        {
            Some(cs) => cs.id,
            None => {
                let now = Utc::now().to_rfc3339();
                let active_cs = class_subject::ActiveModel {
                    id: Set(Uuid::new_v4().to_string()),
                    class_id: Set(req.class_id),
                    subject_id: Set(req.subject_id),
                    cycle_id: Set(req.cycle_id.clone()),
                    primary_teacher_id: Set(user_id.to_string()),
                    assistant_teacher_id: Set(None),
                    substitute_teacher_id: Set(None),
                    grade_groups: Set(None),
                    weekly_hours: Set(None),
                    theory_hours: Set(None),
                    practical_hours: Set(None),
                    assessment_config: Set(None),
                    is_active: Set(1),
                    created_at: Set(now.clone()),
                    updated_at: Set(now),
                };
                let created = active_cs.insert(&self.query.db).await?;
                created.id
            }
        };

        let now = Utc::now().to_rfc3339();
        let a = assessment::Model {
            id: Uuid::new_v4().to_string(),
            org_id: org_id.to_string(),
            cycle_id: req.cycle_id,
            class_subject_id,
            title: req.title,
            title_ar: req.title_ar,
            description: None,
            item_type: req.item_type,
            max_score: req.max_score,
            passing_score: None,
            weight: req.weight.unwrap_or(1.0),
            grade_group_name: req.grade_group_name.unwrap_or_else(|| "1".to_string()),
            assessment_date: req.assessment_date.or(Some(now.clone())),
            due_date: req.due_date,
            duration_minutes: None,
            is_published: 0,
            published_at: None,
            published_by: None,
            is_locked: 0,
            locked_at: None,
            locked_by: None,
            grading_method: None,
            rubric_id: None,
            instructions: None,
            instructions_ar: None,
            attachments: None,
            created_by: user_id.to_string(),
            created_at: now.clone(),
            updated_at: now,
        };

        Ok(self.cmd.create_assessment(a).await?)
    }

    /// Update an assessment.
    pub async fn update_assessment(
        &self,
        id: &str,
        req: UpdateAssessmentRequest,
    ) -> Result<assessment::Model, AppError> {
        let assess = self.get_assessment(id).await?;

        let cycle = academic_cycle::Entity::find_by_id(assess.cycle_id.clone())
            .one(&self.query.db)
            .await?
            .ok_or_else(|| AppError::not_found("academic_cycle", &assess.cycle_id))?;

        if cycle.is_locked == 1 {
            return Err(AppError::Forbidden {
                action: "update_assessment".into(),
                reason: "Cycle is locked".into(),
            });
        }

        let mut active: assessment::ActiveModel = assess.into();
        if let Some(v) = req.title { active.title = Set(v); }
        if let Some(v) = req.title_ar { active.title_ar = Set(Some(v)); }
        if let Some(v) = req.item_type { active.item_type = Set(v); }
        if let Some(v) = req.max_score { active.max_score = Set(v); }
        if let Some(v) = req.weight { active.weight = Set(v); }
        if let Some(v) = req.grade_group_name { active.grade_group_name = Set(v); }
        if let Some(v) = req.assessment_date { active.assessment_date = Set(Some(v)); }
        if let Some(v) = req.due_date { active.due_date = Set(Some(v)); }
        if let Some(v) = req.is_published { active.is_published = Set(v); }
        if let Some(v) = req.is_locked { active.is_locked = Set(v); }
        active.updated_at = Set(Utc::now().to_rfc3339());

        Ok(self.cmd.update_assessment(active).await?)
    }

    /// Delete an assessment.
    pub async fn delete_assessment(&self, id: &str) -> Result<(), AppError> {
        let assess = self.get_assessment(id).await?;

        let cycle = academic_cycle::Entity::find_by_id(assess.cycle_id.clone())
            .one(&self.query.db)
            .await?
            .ok_or_else(|| AppError::not_found("academic_cycle", &assess.cycle_id))?;

        if cycle.is_locked == 1 {
            return Err(AppError::Forbidden {
                action: "delete_assessment".into(),
                reason: "Cycle is locked".into(),
            });
        }

        assessment::Entity::delete_by_id(assess.id)
            .exec(&self.query.db)
            .await?;
        Ok(())
    }
}
