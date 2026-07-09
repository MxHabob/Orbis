//! Behavior service — student behavior records and AI-assisted logs.

use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;
use serde::{Deserialize, Serialize};

use shared_kernel::errors::AppError;
use crate::entities::behavior_record;
use crate::params::BehaviorParams;
use crate::repository::{AssessmentCommandRepo, AssessmentQueryRepo};

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct AIBehaviorSubmission {
    pub student_id: String,
    pub behavior_type: String,
    pub confidence: f64,
    pub location: String,
    pub image_evidence: Option<String>,
    pub model_version: String,
    pub camera_id: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct UpdateBehaviorRequest {
    pub status: Option<String>,
    pub resolution_notes: Option<String>,
    pub action_taken: Option<String>,
    pub action_details: Option<String>,
}

pub struct BehaviorService {
    pub cmd: Arc<AssessmentCommandRepo>,
    pub query: Arc<AssessmentQueryRepo>,
}

impl BehaviorService {
    pub fn new(cmd: Arc<AssessmentCommandRepo>, query: Arc<AssessmentQueryRepo>) -> Self {
        Self { cmd, query }
    }

    /// Submit a behavior record detected by AI.
    /// Expects the caller to have resolved the student's org, cycle and enrollment.
    pub async fn submit_ai_behavior(
        &self,
        org_id: &str,
        cycle_id: &str,
        enrollment_id: &str,
        req: AIBehaviorSubmission,
    ) -> Result<behavior_record::Model, AppError> {
        let mut severity = "minor".to_string();
        let mut points = 0;
        let mut title_ar = "".to_string();

        match req.behavior_type.as_str() {
            "fighting" | "violence" | "aggressive" => {
                severity = "critical".to_string();
                points = -20;
                title_ar = "سلوك عدواني أو شجار".to_string();
            }
            "sleeping" | "inattentive" | "distracted" => {
                severity = "minor".to_string();
                points = -2;
                title_ar = "النوم أثناء الحصة أو عدم الانتباه".to_string();
            }
            "helping" | "positive" | "collaborating" => {
                severity = "minor".to_string();
                points = 10;
                title_ar = "مساعدة الزملاء أو مشاركة إيجابية".to_string();
            }
            "cheating" => {
                severity = "major".to_string();
                points = -10;
                title_ar = "محاولة غش".to_string();
            }
            _ => {}
        }

        let evidence_json = match req.image_evidence.as_ref() {
            Some(img) => format!("[\"{}\"]", img),
            None => "[]".to_string(),
        };

        let record = behavior_record::Model {
            id: Uuid::new_v4().to_string(),
            org_id: org_id.to_string(),
            cycle_id: cycle_id.to_string(),
            enrollment_id: enrollment_id.to_string(),
            behavior_type: req.behavior_type.clone(),
            severity,
            category: None,
            title: format!("AI Detected Behavior: {}", req.behavior_type),
            title_ar: Some(title_ar),
            description: req.description,
            incident_date: Utc::now().format("%Y-%m-%d").to_string(),
            incident_time: Some(Utc::now().format("%H:%M:%S").to_string()),
            location: Some(req.location),
            points: Some(points),
            action_taken: None,
            action_details: None,
            action_completed: Some(0),
            reported_by: "AI_CAMERA_AGENT".to_string(),
            reviewed_by: None,
            reviewed_at: None,
            witnesses: Some("[]".to_string()),
            parent_notified: 0,
            parent_notified_at: None,
            parent_response: None,
            evidence_urls: Some(evidence_json),
            status: "active".to_string(),
            resolution_notes: None,
            resolved_at: None,
            resolved_by: None,
            is_private: 0,
            ai_source: Some("camera_ai".to_string()),
            ai_confidence: Some(req.confidence),
            ai_model_version: Some(req.model_version),
            camera_id: Some(req.camera_id),
            created_at: Utc::now().to_rfc3339(),
            updated_at: Utc::now().to_rfc3339(),
        };

        Ok(self.cmd.create_behavior_record(record).await?)
    }

    /// List behavior records with filters.
    pub async fn get_behavior_records(
        &self,
        org_id: &str,
        params: BehaviorParams,
    ) -> Result<Vec<behavior_record::Model>, AppError> {
        Ok(self.query.get_behavior_records(org_id, params).await?)
    }

    /// Update status or details of a behavior record.
    pub async fn update_behavior_record(
        &self,
        id: &str,
        req: UpdateBehaviorRequest,
        reviewed_by: &str,
    ) -> Result<behavior_record::Model, AppError> {
        use sea_orm::ActiveValue::Set;

        let record = self.query.get_behavior_record_by_id(id).await?
            .ok_or_else(|| AppError::not_found("behavior_record", id))?;

        let mut active: behavior_record::ActiveModel = record.into();

        if let Some(status) = req.status {
            active.status = Set(status.clone());
            if status == "resolved" {
                active.resolved_at = Set(Some(Utc::now().to_rfc3339()));
                active.resolved_by = Set(Some(reviewed_by.to_string()));
            }
        }
        if let Some(notes) = req.resolution_notes {
            active.resolution_notes = Set(Some(notes));
        }
        if let Some(action) = req.action_taken {
            active.action_taken = Set(Some(action));
        }
        if let Some(details) = req.action_details {
            active.action_details = Set(Some(details));
        }
        active.reviewed_by = Set(Some(reviewed_by.to_string()));
        active.reviewed_at = Set(Some(Utc::now().to_rfc3339()));
        active.updated_at = Set(Utc::now().to_rfc3339());

        Ok(self.cmd.update_behavior_record(active).await?)
    }

    /// Soft delete a behavior record by changing status to 'deleted'.
    pub async fn delete_behavior_record(&self, id: &str) -> Result<(), AppError> {
        use sea_orm::ActiveValue::Set;

        let record = self.query.get_behavior_record_by_id(id).await?
            .ok_or_else(|| AppError::not_found("behavior_record", id))?;

        let mut active: behavior_record::ActiveModel = record.into();
        active.status = Set("deleted".to_string());
        active.updated_at = Set(Utc::now().to_rfc3339());

        self.cmd.update_behavior_record(active).await?;
        Ok(())
    }
}
