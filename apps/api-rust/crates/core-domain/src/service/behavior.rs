use uuid::Uuid;
use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::entities::{behavior_record, audit_log};

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

impl super::Service {
    pub async fn submit_ai_behavior(&self, req: AIBehaviorSubmission) -> Result<behavior_record::Model, String> {
        // 1. Find student by ID or Email
        let mut user = self.repo.get_user_by_id(&req.student_id).await
            .map_err(|e| e.to_string())?;

        if user.is_none() {
            user = self.repo.get_user_by_email(&req.student_id).await
                .map_err(|e| e.to_string())?;
        }

        let user = user.ok_or_else(|| format!("student not found: {}", req.student_id))?;

        // 2. Find current cycle
        let cycle = self.repo.get_current_cycle(&user.org_id).await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "no active academic cycle for this student's organization".to_string())?;

        // 3. Find active enrollment
        let enrollment = self.repo.get_enrollment_by_student_id(&user.id, &cycle.id).await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| format!("student {} is not actively enrolled in current cycle", user.full_name))?;

        // 4. Map severity and points
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

        // 5. Create behavior record
        let record = behavior_record::Model {
            id: Uuid::new_v4().to_string(),
            org_id: user.org_id.clone(),
            cycle_id: cycle.id.clone(),
            enrollment_id: enrollment.id.clone(),
            behavior_type: req.behavior_type.clone(),
            severity: severity.clone(),
            category: None,
            title: format!("AI Detected Behavior: {}", req.behavior_type),
            title_ar: Some(title_ar.clone()),
            description: req.description.clone(),
            incident_date: Utc::now().format("%Y-%m-%d").to_string(),
            incident_time: Some(Utc::now().format("%H:%M:%S").to_string()),
            location: Some(req.location.clone()),
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
            ai_model_version: Some(req.model_version.clone()),
            camera_id: Some(req.camera_id.clone()),
            created_at: Utc::now().to_rfc3339(),
            updated_at: Utc::now().to_rfc3339(),
        };

        let created_record = self.repo.create_behavior_record(record).await
            .map_err(|e| e.to_string())?;

        // 6. Audit Log
        let audit = audit_log::Model {
            id: Uuid::new_v4().to_string(),
            org_id: user.org_id.clone(),
            user_id: Some("AI_CAMERA_AGENT".to_string()),
            action: "behavior.ai_detection".to_string(),
            entity_type: "behavior_record".to_string(),
            entity_id: created_record.id.clone(),
            old_data: None,
            new_data: None,
            ip_address: None,
            user_agent: None,
            session_id: None,
            metadata: "{}".to_string(),
            created_at: Utc::now().to_rfc3339(),
        };
        let _ = self.repo.create_audit_log(audit).await;

        // 7. Dispatch prioritized notification
        let mut priority = "low".to_string();
        if severity == "critical" {
            priority = "high".to_string();
        } else if severity == "major" {
            priority = "medium".to_string();
        }

        let n_title = "AI Camera Behavior Alert".to_string();
        let n_title_ar = "تنبيه سلوك من كاميرا الذكاء الاصطناعي".to_string();
        let n_msg = format!("AI Camera detected behavior: {} at {}. Confidence: {:.0}%.", req.behavior_type, req.location, req.confidence * 100.0);
        let n_msg_ar = format!("تم رصد سلوك ({}) في ({}) بنسبة ثقة {:.0}%.", title_ar, req.location, req.confidence * 100.0);

        let _ = self.send_notification(
            &user.id,
            &user.org_id,
            &n_title,
            &n_title_ar,
            &n_msg,
            &n_msg_ar,
            &priority,
            "behavior",
        ).await;

        Ok(created_record)
    }

    pub async fn get_behavior_records(&self, org_id: &str, params: crate::params::BehaviorParams) -> Result<Vec<behavior_record::Model>, String> {
        self.repo.get_behavior_records(org_id, params).await.map_err(|e| e.to_string())
    }

    pub async fn update_behavior_record(&self, id: &str, req: UpdateBehaviorRequest, reviewed_by: &str) -> Result<behavior_record::Model, String> {
        use sea_orm::{entity::prelude::*, ActiveModelTrait, ActiveValue::Set};

        let record = behavior_record::Entity::find_by_id(id.to_string())
            .one(&self.repo.db)
            .await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "behavior record not found".to_string())?;

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

        active.update(&self.repo.db).await.map_err(|e| e.to_string())
    }

    pub async fn delete_behavior_record(&self, id: &str) -> Result<(), String> {
        use sea_orm::{entity::prelude::*, ActiveModelTrait, ActiveValue::Set};

        let record = behavior_record::Entity::find_by_id(id.to_string())
            .one(&self.repo.db)
            .await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "behavior record not found".to_string())?;

        let mut active: behavior_record::ActiveModel = record.into();
        active.status = Set("deleted".to_string());
        active.updated_at = Set(Utc::now().to_rfc3339());

        active.update(&self.repo.db).await.map_err(|e| e.to_string())?;
        Ok(())
    }
}

