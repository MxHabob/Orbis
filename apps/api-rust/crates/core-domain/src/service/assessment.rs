use uuid::Uuid;
use chrono::Utc;
use sea_orm::{entity::prelude::*, ActiveModelTrait, ActiveValue::Set, EntityTrait};

use crate::entities::assessment;
use super::schedule::CreateClassSubjectRequest;

#[derive(serde::Serialize, serde::Deserialize, utoipa::ToSchema, Clone, Debug)]
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

#[derive(serde::Serialize, serde::Deserialize, utoipa::ToSchema, Clone, Debug)]
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

impl super::Service {
    pub async fn get_assessments(
        &self,
        class_id: &str,
        subject_id: &str,
        cycle_id: &str,
    ) -> Result<Vec<assessment::Model>, String> {
        let cs = self.repo.find_class_subject_for_assessment(class_id, subject_id).await
            .map_err(|e| e.to_string())?;

        match cs {
            Some(c) => self.repo.get_assessments(&c.id, cycle_id).await.map_err(|e| e.to_string()),
            None => Ok(vec![]),
        }
    }

    pub async fn get_assessment(&self, id: &str) -> Result<assessment::Model, String> {
        self.repo.get_assessment_by_id(id).await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "assessment not found".to_string())
    }

    pub async fn create_assessment(
        &self,
        org_id: &str,
        user_id: &str,
        req: CreateAssessmentRequest,
    ) -> Result<assessment::Model, String> {
        let class_subject_id = match self.repo.find_class_subject_for_assessment(&req.class_id, &req.subject_id).await
            .map_err(|e| e.to_string())? {
            Some(cs) => cs.id,
            None => {
                let created = self.create_class_subject(CreateClassSubjectRequest {
                    class_id: req.class_id.clone(),
                    subject_id: req.subject_id.clone(),
                    cycle_id: req.cycle_id.clone(),
                    primary_teacher_id: user_id.to_string(),
                    assistant_teacher_id: None,
                    weekly_hours: None,
                    theory_hours: None,
                    practical_hours: None,
                }).await?;
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
        self.repo.insert_assessment(a).await.map_err(|e| e.to_string())
    }

    pub async fn update_assessment(
        &self,
        id: &str,
        req: UpdateAssessmentRequest,
    ) -> Result<assessment::Model, String> {
        let assess = self.repo.get_assessment_by_id(id).await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "assessment not found".to_string())?;

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

        active.update(&self.repo.db).await.map_err(|e| e.to_string())
    }

    pub async fn delete_assessment(&self, id: &str) -> Result<(), String> {
        let assess = self.repo.get_assessment_by_id(id).await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "assessment not found".to_string())?;

        assessment::Entity::delete_by_id(assess.id)
            .exec(&self.repo.db)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}
