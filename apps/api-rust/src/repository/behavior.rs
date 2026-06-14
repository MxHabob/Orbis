use crate::entities::{behavior_record, audit_log};
use sea_orm::{entity::prelude::*, ActiveModelTrait, ActiveValue::Set, QueryFilter, EntityTrait, QueryOrder};

impl super::Repository {
    pub async fn create_behavior_record(&self, b: behavior_record::Model) -> Result<behavior_record::Model, DbErr> {
        let active_model = behavior_record::ActiveModel {
            id: Set(b.id),
            org_id: Set(b.org_id),
            cycle_id: Set(b.cycle_id),
            enrollment_id: Set(b.enrollment_id),
            behavior_type: Set(b.behavior_type),
            severity: Set(b.severity),
            category: Set(b.category),
            title: Set(b.title),
            title_ar: Set(b.title_ar),
            description: Set(b.description),
            incident_date: Set(b.incident_date),
            incident_time: Set(b.incident_time),
            location: Set(b.location),
            points: Set(b.points),
            action_taken: Set(b.action_taken),
            action_details: Set(b.action_details),
            action_completed: Set(b.action_completed),
            reported_by: Set(b.reported_by),
            reviewed_by: Set(b.reviewed_by),
            reviewed_at: Set(b.reviewed_at),
            witnesses: Set(b.witnesses),
            parent_notified: Set(b.parent_notified),
            parent_notified_at: Set(b.parent_notified_at),
            parent_response: Set(b.parent_response),
            evidence_urls: Set(b.evidence_urls),
            status: Set(b.status),
            resolution_notes: Set(b.resolution_notes),
            resolved_at: Set(b.resolved_at),
            resolved_by: Set(b.resolved_by),
            is_private: Set(b.is_private),
            ai_source: Set(b.ai_source),
            ai_confidence: Set(b.ai_confidence),
            ai_model_version: Set(b.ai_model_version),
            camera_id: Set(b.camera_id),
            created_at: Set(b.created_at),
            updated_at: Set(b.updated_at),
        };
        active_model.insert(&self.db).await
    }

    pub async fn get_behavior_records(&self, org_id: &str) -> Result<Vec<behavior_record::Model>, DbErr> {
        behavior_record::Entity::find()
            .filter(behavior_record::Column::OrgId.eq(org_id))
            .order_by_desc(behavior_record::Column::CreatedAt)
            .all(&self.db)
            .await
    }

    // ── Audit Log ───────────────────────────────────────────────────

    pub async fn create_audit_log(&self, a: audit_log::Model) -> Result<audit_log::Model, DbErr> {
        let active_model = audit_log::ActiveModel {
            id: Set(a.id),
            org_id: Set(a.org_id),
            user_id: Set(a.user_id),
            action: Set(a.action),
            entity_type: Set(a.entity_type),
            entity_id: Set(a.entity_id),
            old_data: Set(a.old_data),
            new_data: Set(a.new_data),
            ip_address: Set(a.ip_address),
            user_agent: Set(a.user_agent),
            session_id: Set(a.session_id),
            metadata: Set(a.metadata),
            created_at: Set(a.created_at),
        };
        active_model.insert(&self.db).await
    }

    pub async fn get_audit_logs(&self, org_id: &str) -> Result<Vec<audit_log::Model>, DbErr> {
        audit_log::Entity::find()
            .filter(audit_log::Column::OrgId.eq(org_id))
            .order_by_desc(audit_log::Column::CreatedAt)
            .limit(100)
            .all(&self.db)
            .await
    }
}
