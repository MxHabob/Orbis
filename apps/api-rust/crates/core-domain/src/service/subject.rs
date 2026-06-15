use uuid::Uuid;
use chrono::Utc;
use sea_orm::{entity::prelude::*, ActiveModelTrait, ActiveValue::Set, EntityTrait};

use crate::entities::subject;

#[derive(serde::Serialize, serde::Deserialize, utoipa::ToSchema, Clone, Debug)]
pub struct CreateSubjectRequest {
    pub name: String,
    pub code: Option<String>,
    pub name_ar: Option<String>,
    pub name_en: Option<String>,
    pub description: Option<String>,
    pub color: Option<String>,
    pub sort_order: Option<i32>,
    pub is_core: Option<i32>,
    pub requires_lab: Option<i32>,
}

#[derive(serde::Serialize, serde::Deserialize, utoipa::ToSchema, Clone, Debug)]
pub struct UpdateSubjectRequest {
    pub name: Option<String>,
    pub code: Option<String>,
    pub name_ar: Option<String>,
    pub name_en: Option<String>,
    pub description: Option<String>,
    pub color: Option<String>,
    pub sort_order: Option<i32>,
    pub is_core: Option<i32>,
    pub requires_lab: Option<i32>,
}

impl super::Service {
    pub async fn get_subjects(&self, org_id: &str) -> Result<Vec<subject::Model>, String> {
        self.repo.get_subjects(org_id).await.map_err(|e| e.to_string())
    }

    pub async fn get_subject(&self, id: &str) -> Result<subject::Model, String> {
        self.repo.get_subject_by_id(id).await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "subject not found".to_string())
    }

    pub async fn create_subject(
        &self,
        org_id: &str,
        req: CreateSubjectRequest,
    ) -> Result<subject::Model, String> {
        if req.name.trim().is_empty() {
            return Err("subject name is required".to_string());
        }

        let now = Utc::now().to_rfc3339();
        let s = subject::Model {
            id: Uuid::new_v4().to_string(),
            org_id: org_id.to_string(),
            unit_id: None,
            code: req.code.unwrap_or_default(),
            name: req.name,
            name_ar: req.name_ar,
            name_en: req.name_en,
            description: req.description,
            short_description: None,
            is_core: req.is_core.unwrap_or(1),
            is_elective: 0,
            credit_hours: None,
            requires_lab: req.requires_lab.unwrap_or(0),
            requires_practical: 0,
            max_students_per_class: None,
            default_grade_groups: None,
            prerequisites: None,
            learning_outcomes: None,
            color: req.color.or(Some("#6366f1".to_string())),
            icon: None,
            cover_image: None,
            sort_order: req.sort_order,
            is_active: 1,
            created_at: now.clone(),
            updated_at: now,
        };
        self.repo.insert_subject(s).await.map_err(|e| e.to_string())
    }

    pub async fn update_subject(
        &self,
        id: &str,
        req: UpdateSubjectRequest,
    ) -> Result<subject::Model, String> {
        let subj = self.repo.get_subject_by_id(id).await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "subject not found".to_string())?;

        let mut active: subject::ActiveModel = subj.into();
        if let Some(v) = req.name { active.name = Set(v); }
        if let Some(v) = req.code { active.code = Set(v); }
        if let Some(v) = req.name_ar { active.name_ar = Set(Some(v)); }
        if let Some(v) = req.name_en { active.name_en = Set(Some(v)); }
        if let Some(v) = req.description { active.description = Set(Some(v)); }
        if let Some(v) = req.color { active.color = Set(Some(v)); }
        if let Some(v) = req.sort_order { active.sort_order = Set(Some(v)); }
        if let Some(v) = req.is_core { active.is_core = Set(v); }
        if let Some(v) = req.requires_lab { active.requires_lab = Set(v); }
        active.updated_at = Set(Utc::now().to_rfc3339());

        active.update(&self.repo.db).await.map_err(|e| e.to_string())
    }

    pub async fn delete_subject(&self, id: &str) -> Result<(), String> {
        let subj = self.repo.get_subject_by_id(id).await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "subject not found".to_string())?;

        let mut active: subject::ActiveModel = subj.into();
        active.is_active = Set(0);
        active.updated_at = Set(Utc::now().to_rfc3339());
        active.update(&self.repo.db).await.map_err(|e| e.to_string())?;
        Ok(())
    }
}
