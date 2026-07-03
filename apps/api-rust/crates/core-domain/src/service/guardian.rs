use uuid::Uuid;
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, EntityTrait};

use crate::entities::student_guardian;

#[derive(serde::Serialize, serde::Deserialize, utoipa::ToSchema, Clone, Debug)]
pub struct CreateGuardianLinkRequest {
    pub student_id: String,
    pub guardian_id: String,
    pub relationship: String,
    pub relationship_ar: Option<String>,
    pub is_primary: Option<i32>,
    pub is_emergency_contact: Option<i32>,
    pub can_view_grades: Option<i32>,
    pub can_view_attendance: Option<i32>,
    pub can_view_behavior: Option<i32>,
    pub can_view_finance: Option<i32>,
    pub can_view_schedule: Option<i32>,
    pub can_receive_notifications: Option<i32>,
    pub is_payer: Option<i32>,
    pub payment_share_percent: Option<f64>,
    pub occupation: Option<String>,
    pub employer: Option<String>,
    pub work_phone: Option<String>,
    pub notes: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, utoipa::ToSchema, Clone, Debug)]
pub struct UpdateGuardianLinkRequest {
    pub relationship: Option<String>,
    pub relationship_ar: Option<String>,
    pub is_primary: Option<i32>,
    pub is_emergency_contact: Option<i32>,
    pub can_view_grades: Option<i32>,
    pub can_view_attendance: Option<i32>,
    pub can_view_behavior: Option<i32>,
    pub can_view_finance: Option<i32>,
    pub can_view_schedule: Option<i32>,
    pub can_receive_notifications: Option<i32>,
    pub is_payer: Option<i32>,
    pub payment_share_percent: Option<f64>,
    pub occupation: Option<String>,
    pub employer: Option<String>,
    pub work_phone: Option<String>,
    pub notes: Option<String>,
}

impl super::Service {
    pub async fn get_guardians_by_student(
        &self,
        student_id: &str,
    ) -> Result<Vec<student_guardian::Model>, String> {
        self.repo.get_guardians_by_student(student_id).await.map_err(|e| e.to_string())
    }

    pub async fn get_students_by_guardian(
        &self,
        guardian_id: &str,
    ) -> Result<Vec<student_guardian::Model>, String> {
        self.repo.get_students_by_guardian(guardian_id).await.map_err(|e| e.to_string())
    }

    pub async fn create_guardian_link(
        &self,
        req: CreateGuardianLinkRequest,
    ) -> Result<student_guardian::Model, String> {
        let now = Utc::now().to_rfc3339();
        let link = student_guardian::Model {
            id: Uuid::new_v4().to_string(),
            student_id: req.student_id,
            guardian_id: req.guardian_id,
            relationship: req.relationship,
            relationship_ar: req.relationship_ar,
            is_primary: req.is_primary.unwrap_or(0),
            is_emergency_contact: req.is_emergency_contact.unwrap_or(0),
            is_authorized_pickup: 1,
            can_view_grades: req.can_view_grades.unwrap_or(1),
            can_view_attendance: req.can_view_attendance.unwrap_or(1),
            can_view_behavior: req.can_view_behavior.unwrap_or(1),
            can_view_finance: req.can_view_finance.unwrap_or(0),
            can_view_schedule: req.can_view_schedule.unwrap_or(1),
            can_receive_notifications: req.can_receive_notifications.unwrap_or(1),
            is_payer: req.is_payer.unwrap_or(0),
            payment_share_percent: req.payment_share_percent,
            occupation: req.occupation,
            employer: req.employer,
            work_phone: req.work_phone,
            notes: req.notes,
            created_at: now.clone(),
            updated_at: now,
        };
        self.repo.insert_guardian_link(link).await.map_err(|e| e.to_string())
    }

    pub async fn update_guardian_link(
        &self,
        id: &str,
        req: UpdateGuardianLinkRequest,
    ) -> Result<student_guardian::Model, String> {
        let link = self.repo.get_guardian_link_by_id(id).await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "guardian link not found".to_string())?;

        let mut active: student_guardian::ActiveModel = link.into();
        if let Some(v) = req.relationship { active.relationship = Set(v); }
        if let Some(v) = req.relationship_ar { active.relationship_ar = Set(Some(v)); }
        if let Some(v) = req.is_primary { active.is_primary = Set(v); }
        if let Some(v) = req.is_emergency_contact { active.is_emergency_contact = Set(v); }
        if let Some(v) = req.can_view_grades { active.can_view_grades = Set(v); }
        if let Some(v) = req.can_view_attendance { active.can_view_attendance = Set(v); }
        if let Some(v) = req.can_view_behavior { active.can_view_behavior = Set(v); }
        if let Some(v) = req.can_view_finance { active.can_view_finance = Set(v); }
        if let Some(v) = req.can_view_schedule { active.can_view_schedule = Set(v); }
        if let Some(v) = req.can_receive_notifications { active.can_receive_notifications = Set(v); }
        if let Some(v) = req.is_payer { active.is_payer = Set(v); }
        if let Some(v) = req.payment_share_percent { active.payment_share_percent = Set(Some(v)); }
        if let Some(v) = req.occupation { active.occupation = Set(Some(v)); }
        if let Some(v) = req.employer { active.employer = Set(Some(v)); }
        if let Some(v) = req.work_phone { active.work_phone = Set(Some(v)); }
        if let Some(v) = req.notes { active.notes = Set(Some(v)); }
        active.updated_at = Set(Utc::now().to_rfc3339());

        active.update(&self.repo.db).await.map_err(|e| e.to_string())
    }

    pub async fn delete_guardian_link(&self, id: &str) -> Result<(), String> {
        let link = self.repo.get_guardian_link_by_id(id).await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "guardian link not found".to_string())?;

        student_guardian::Entity::delete_by_id(link.id)
            .exec(&self.repo.db)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn search_guardians(
        &self,
        org_id: &str,
        query: &str,
    ) -> Result<Vec<crate::entities::user::Model>, String> {
        use sea_orm::{ColumnTrait, QueryFilter, QueryOrder, QuerySelect};
        use crate::entities::user;

        let search_pattern = format!("%{}%", query.trim());
        user::Entity::find()
            .filter(user::Column::OrgId.eq(org_id))
            .filter(user::Column::IsDeleted.eq(0))
            .filter(user::Column::PrimaryRole.eq("parent"))
            .filter(
                user::Column::FullName.like(&search_pattern)
                    .or(user::Column::Phone.like(&search_pattern))
                    .or(user::Column::Email.like(&search_pattern))
                    .or(user::Column::NationalId.like(&search_pattern))
            )
            .order_by_asc(user::Column::FullName)
            .limit(20)
            .all(&self.repo.db)
            .await
            .map_err(|e| e.to_string())
    }
}
