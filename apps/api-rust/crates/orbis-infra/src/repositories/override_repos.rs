use async_trait::async_trait;
use orbis_app::ports::repositories::OverrideRequestRepository;
use orbis_domain::entities::override_request::{AcademicOverrideRequest, OverrideType, RiskLevel, OverrideStatus, ApprovalStep};
use orbis_domain::errors::DomainError;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set, ColumnTrait, QueryFilter};
use uuid::Uuid;

use crate::models::override_request::{self, Entity as OverrideEntity};

pub struct OverrideRepoImpl {
    pub db: DatabaseConnection,
}

#[async_trait]
impl OverrideRequestRepository for OverrideRepoImpl {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<AcademicOverrideRequest>, DomainError> {
        let model = OverrideEntity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(model.map(map_model_to_domain))
    }

    async fn find_by_org(&self, org_id: Uuid, status: Option<&str>) -> Result<Vec<AcademicOverrideRequest>, DomainError> {
        let mut query = OverrideEntity::find().filter(override_request::Column::OrgId.eq(org_id));
        if let Some(st) = status {
            query = query.filter(override_request::Column::Status.eq(st));
        }
        let models = query.all(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(models.into_iter().map(map_model_to_domain).collect())
    }

    async fn find_active_override(
        &self,
        org_id: Uuid,
        override_type: &str,
        entity_type: &str,
        entity_id: Uuid,
    ) -> Result<Option<AcademicOverrideRequest>, DomainError> {
        let model = OverrideEntity::find()
            .filter(override_request::Column::OrgId.eq(org_id))
            .filter(override_request::Column::OverrideType.eq(override_type))
            .filter(override_request::Column::EntityType.eq(entity_type))
            .filter(override_request::Column::EntityId.eq(entity_id))
            .filter(override_request::Column::Status.eq("approved"))
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(model.map(map_model_to_domain))
    }

    async fn find_pending_for_role(&self, org_id: Uuid, role: &str) -> Result<Vec<AcademicOverrideRequest>, DomainError> {
        let models = OverrideEntity::find()
            .filter(override_request::Column::OrgId.eq(org_id))
            .filter(override_request::Column::Status.eq("pending"))
            .filter(override_request::Column::CurrentApproverRole.eq(role))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(models.into_iter().map(map_model_to_domain).collect())
    }

    async fn save(&self, r: &AcademicOverrideRequest) -> Result<(), DomainError> {
        let chain_json = serde_json::to_value(&r.approval_chain).unwrap_or_default();
        let active = override_request::ActiveModel {
            id: Set(r.id),
            org_id: Set(r.org_id),
            override_type: Set(r.override_type.to_string()),
            risk_level: Set(r.risk_level.to_string()),
            entity_type: Set(r.entity_type.clone()),
            entity_id: Set(r.entity_id),
            field_name: Set(r.field_name.clone()),
            old_value: Set(r.old_value.clone()),
            new_value: Set(r.new_value.clone()),
            reason: Set(r.reason.clone()),
            requested_by: Set(r.requested_by),
            requested_by_name: Set(r.requested_by_name.clone()),
            status: Set(r.status.to_string()),
            current_approver_role: Set(r.current_approver_role.clone()),
            approval_chain: Set(chain_json),
            resolved_by: Set(r.resolved_by),
            resolved_by_name: Set(r.resolved_by_name.clone()),
            resolved_at: Set(r.resolved_at),
            resolution_notes: Set(r.resolution_notes.clone()),
            expires_at: Set(r.expires_at),
            metadata: Set(Some(r.metadata.clone())),
            created_at: Set(r.created_at),
            updated_at: Set(r.updated_at),
        };

        if OverrideEntity::find_by_id(r.id).one(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?.is_some() {
            active.update(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        } else {
            active.insert(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        }
        Ok(())
    }
}

fn map_model_to_domain(m: override_request::Model) -> AcademicOverrideRequest {
    let chain: Vec<ApprovalStep> = serde_json::from_value(m.approval_chain).unwrap_or_default();
    AcademicOverrideRequest {
        id: m.id,
        org_id: m.org_id,
        override_type: match m.override_type.as_str() {
            "grade_change" => OverrideType::GradeChange,
            "prerequisite_bypass" => OverrideType::PrerequisiteBypass,
            "late_enrollment" => OverrideType::LateEnrollment,
            "capacity_override" => OverrideType::CapacityOverride,
            "credit_overload" => OverrideType::CreditOverload,
            "discount_override" => OverrideType::DiscountOverride,
            "standing_override" => OverrideType::AcademicStandingOverride,
            _ => OverrideType::Custom(0),
        },
        risk_level: match m.risk_level.as_str() {
            "low" => RiskLevel::Low,
            "high" => RiskLevel::High,
            "critical" => RiskLevel::Critical,
            _ => RiskLevel::Medium,
        },
        entity_type: m.entity_type,
        entity_id: m.entity_id,
        field_name: m.field_name,
        old_value: m.old_value,
        new_value: m.new_value,
        reason: m.reason,
        requested_by: m.requested_by,
        requested_by_name: m.requested_by_name,
        status: match m.status.as_str() {
            "approved" => OverrideStatus::Approved,
            "rejected" => OverrideStatus::Rejected,
            "expired" => OverrideStatus::Expired,
            "executed" => OverrideStatus::Executed,
            "cancelled" => OverrideStatus::Cancelled,
            _ => OverrideStatus::Pending,
        },
        current_approver_role: m.current_approver_role,
        approval_chain: chain,
        resolved_by: m.resolved_by,
        resolved_by_name: m.resolved_by_name,
        resolved_at: m.resolved_at,
        resolution_notes: m.resolution_notes,
        expires_at: m.expires_at,
        metadata: m.metadata.unwrap_or_default(),
        created_at: m.created_at,
        updated_at: m.updated_at,
    }
}
