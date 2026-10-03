use chrono::Utc;
use orbis_domain::entities::override_request::{
    AcademicOverrideRequest, ApprovalStep, OverrideStatus, OverrideType, RiskLevel,
};
use orbis_domain::errors::DomainError;
use orbis_domain::events::DomainEvent;
use orbis_domain::iam::permission::Permission;
use std::sync::Arc;
use uuid::Uuid;

use crate::context::RequestContext;
use crate::ports::event_bus::EventBus;
use crate::ports::iam::PermissionCheckerPort;
use crate::ports::repositories::{OverrideRequestRepository, WorkflowRepository};

#[derive(Debug, Clone)]
pub struct SubmitOverrideInput {
    pub override_type: OverrideType,
    pub risk_level: RiskLevel,
    pub entity_type: String,
    pub entity_id: Uuid,
    pub field_name: Option<String>,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
    pub reason: String,
    pub requested_by: Uuid,
    pub requested_by_name: Option<String>,
    pub approval_chain: Vec<ApprovalStep>,
    pub metadata: Option<serde_json::Value>,
}

pub struct WorkflowService {
    pub override_repo: Arc<dyn OverrideRequestRepository>,
    pub workflow_repo: Arc<dyn WorkflowRepository>,
    pub event_bus: Arc<dyn EventBus>,
    pub permission_checker: Arc<dyn PermissionCheckerPort>,
}

impl WorkflowService {
    pub fn new(
        override_repo: Arc<dyn OverrideRequestRepository>,
        workflow_repo: Arc<dyn WorkflowRepository>,
        event_bus: Arc<dyn EventBus>,
        permission_checker: Arc<dyn PermissionCheckerPort>,
    ) -> Self {
        Self {
            override_repo,
            workflow_repo,
            event_bus,
            permission_checker,
        }
    }

    /// Submit a new override request
    pub async fn submit_override(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        input: SubmitOverrideInput,
    ) -> Result<AcademicOverrideRequest, DomainError> {
        self.permission_checker
            .check(ctx, Permission::WorkflowWrite)
            .await?;

        let now = Utc::now();
        let request_id = Uuid::new_v4();

        let initial_approver_role = input
            .approval_chain
            .first()
            .map(|s| s.approver_role.clone());

        let req = AcademicOverrideRequest {
            id: request_id,
            org_id,
            override_type: input.override_type,
            risk_level: input.risk_level,
            entity_type: input.entity_type,
            entity_id: input.entity_id,
            field_name: input.field_name,
            old_value: input.old_value,
            new_value: input.new_value,
            reason: input.reason,
            requested_by: input.requested_by,
            requested_by_name: input.requested_by_name,
            status: OverrideStatus::Pending,
            current_approver_role: initial_approver_role,
            approval_chain: input.approval_chain,
            resolved_by: None,
            resolved_by_name: None,
            resolved_at: None,
            resolution_notes: None,
            expires_at: Some(now + chrono::Duration::days(14)),
            metadata: input.metadata.unwrap_or_else(|| serde_json::json!({})),
            created_at: now,
            updated_at: now,
        };

        self.override_repo.save(&req).await?;

        let event = DomainEvent::OverrideSubmitted {
            request_id,
            org_id,
            override_type: req.override_type.to_string(),
            requested_by: req.requested_by,
            timestamp: now,
        };
        self.event_bus.publish(ctx, &event).await?;

        Ok(req)
    }

    /// Approve an override request step or full resolution
    pub async fn approve_override(
        &self,
        ctx: &RequestContext,
        request_id: Uuid,
        approver_id: Uuid,
        approver_name: Option<String>,
        notes: Option<String>,
    ) -> Result<AcademicOverrideRequest, DomainError> {
        self.permission_checker
            .check(ctx, Permission::WorkflowApprove)
            .await?;

        let mut req = self
            .override_repo
            .find_by_id(request_id)
            .await?
            .ok_or_else(|| {
                DomainError::NotFound(format!("Override Request {} not found", request_id))
            })?;

        let now = Utc::now();

        // Advance approval chain
        let mut all_approved = true;
        let mut next_role = None;

        for step in req.approval_chain.iter_mut() {
            if step.status == OverrideStatus::Pending {
                step.status = OverrideStatus::Approved;
                step.approver_id = Some(approver_id);
                step.approver_name = approver_name.clone();
                step.decision_at = Some(now);
                step.notes = notes.clone();
                break;
            }
        }

        for step in &req.approval_chain {
            if step.status == OverrideStatus::Pending {
                all_approved = false;
                next_role = Some(step.approver_role.clone());
                break;
            }
        }

        if all_approved {
            req.status = OverrideStatus::Approved;
            req.resolved_by = Some(approver_id);
            req.resolved_by_name = approver_name;
            req.resolved_at = Some(now);
            req.resolution_notes = notes;
            req.current_approver_role = None;
        } else {
            req.current_approver_role = next_role;
        }

        req.updated_at = now;
        self.override_repo.save(&req).await?;

        let event = DomainEvent::OverrideApproved {
            request_id,
            org_id: req.org_id,
            approver_id,
            timestamp: now,
        };
        self.event_bus.publish(ctx, &event).await?;

        Ok(req)
    }

    /// Reject an override request
    pub async fn reject_override(
        &self,
        ctx: &RequestContext,
        request_id: Uuid,
        rejecter_id: Uuid,
        rejecter_name: Option<String>,
        reason: String,
    ) -> Result<AcademicOverrideRequest, DomainError> {
        self.permission_checker
            .check(ctx, Permission::WorkflowApprove)
            .await?;

        let mut req = self
            .override_repo
            .find_by_id(request_id)
            .await?
            .ok_or_else(|| {
                DomainError::NotFound(format!("Override Request {} not found", request_id))
            })?;

        let now = Utc::now();
        req.status = OverrideStatus::Rejected;
        req.resolved_by = Some(rejecter_id);
        req.resolved_by_name = rejecter_name;
        req.resolved_at = Some(now);
        req.resolution_notes = Some(reason.clone());
        req.current_approver_role = None;
        req.updated_at = now;

        self.override_repo.save(&req).await?;

        let event = DomainEvent::OverrideRejected {
            request_id,
            org_id: req.org_id,
            rejecter_id,
            reason,
            timestamp: now,
        };
        self.event_bus.publish(ctx, &event).await?;

        Ok(req)
    }

    /// List override requests for organization
    pub async fn list_overrides(
        &self,
        org_id: Uuid,
        status: Option<&str>,
    ) -> Result<Vec<AcademicOverrideRequest>, DomainError> {
        self.override_repo.find_by_org(org_id, status).await
    }

    /// Get override request by ID
    pub async fn get_override(
        &self,
        id: Uuid,
    ) -> Result<Option<AcademicOverrideRequest>, DomainError> {
        self.override_repo.find_by_id(id).await
    }
}
