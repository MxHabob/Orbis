use orbis_domain::entities::override_request::{
    AcademicOverrideRequest, OverrideType, RiskLevel, OverrideStatus, ApprovalStep
};
use orbis_domain::events::DomainEvent;
use orbis_domain::errors::DomainError;
use crate::context::RequestContext;
use crate::ports::repositories::OverrideRequestRepository;
use crate::ports::event_bus::EventBus;
use crate::ports::iam::PermissionCheckerPort;
use orbis_domain::iam::permission::Permission;
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;

pub struct SubmitOverrideRequestUseCase {
    pub override_repo: Arc<dyn OverrideRequestRepository>,
    pub event_bus: Arc<dyn EventBus>,
    pub permission_checker: Arc<dyn PermissionCheckerPort>,
}

impl SubmitOverrideRequestUseCase {
    pub async fn execute(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        override_type: OverrideType,
        risk_level: RiskLevel,
        entity_type: String,
        entity_id: Uuid,
        field_name: Option<String>,
        old_value: Option<String>,
        new_value: Option<String>,
        reason: String,
        initial_approver_role: Option<String>,
    ) -> Result<AcademicOverrideRequest, DomainError> {
        self.permission_checker.check(ctx, Permission::WorkflowApprove).await?;

        let now = Utc::now();
        let request_id = Uuid::new_v4();
        let requester_id = ctx.user_id().unwrap_or_default();

        let initial_steps = vec![
            ApprovalStep {
                step_order: 1,
                approver_role: initial_approver_role.clone().unwrap_or_else(|| "AcademicDean".into()),
                approver_id: None,
                approver_name: None,
                status: OverrideStatus::Pending,
                decision_at: None,
                notes: None,
            }
        ];

        let req = AcademicOverrideRequest {
            id: request_id,
            org_id,
            override_type,
            risk_level,
            entity_type,
            entity_id,
            field_name,
            old_value,
            new_value,
            reason,
            requested_by: requester_id,
            requested_by_name: None,
            status: OverrideStatus::Pending,
            current_approver_role: initial_approver_role,
            approval_chain: initial_steps,
            resolved_by: None,
            resolved_by_name: None,
            resolved_at: None,
            resolution_notes: None,
            expires_at: Some(now + chrono::Duration::days(30)),
            metadata: serde_json::json!({}),
            created_at: now,
            updated_at: now,
        };

        self.override_repo.save(&req).await?;

        let event = DomainEvent::OverrideSubmitted {
            request_id,
            org_id,
            override_type: override_type.to_string(),
            requested_by: requester_id,
            timestamp: now,
        };
        self.event_bus.publish(ctx, &event).await?;

        Ok(req)
    }
}

pub struct ApproveOverrideRequestUseCase {
    pub override_repo: Arc<dyn OverrideRequestRepository>,
    pub event_bus: Arc<dyn EventBus>,
    pub permission_checker: Arc<dyn PermissionCheckerPort>,
}

impl ApproveOverrideRequestUseCase {
    pub async fn execute(
        &self,
        ctx: &RequestContext,
        request_id: Uuid,
        notes: Option<String>,
    ) -> Result<AcademicOverrideRequest, DomainError> {
        self.permission_checker.check(ctx, Permission::WorkflowApprove).await?;

        let mut req = self.override_repo.find_by_id(request_id).await?
            .ok_or_else(|| DomainError::NotFound(format!("Override request {} not found", request_id)))?;

        if req.status != OverrideStatus::Pending {
            return Err(DomainError::Validation(format!(
                "Cannot approve override request with status {:?}", req.status
            )));
        }

        let now = Utc::now();
        let user_id = ctx.user_id();

        // Mark the current approval step as approved
        if let Some(step) = req.approval_chain.last_mut() {
            step.status = OverrideStatus::Approved;
            step.approver_id = user_id;
            step.decision_at = Some(now);
            step.notes = notes.clone();
        }

        req.status = OverrideStatus::Approved;
        req.resolved_by = user_id;
        req.resolved_at = Some(now);
        req.resolution_notes = notes;
        req.updated_at = now;

        self.override_repo.save(&req).await?;

        let event = DomainEvent::OverrideApproved {
            request_id,
            org_id: req.org_id,
            approver_id: user_id.unwrap_or_default(),
            timestamp: now,
        };
        self.event_bus.publish(ctx, &event).await?;

        Ok(req)
    }
}

pub struct RejectOverrideRequestUseCase {
    pub override_repo: Arc<dyn OverrideRequestRepository>,
    pub event_bus: Arc<dyn EventBus>,
    pub permission_checker: Arc<dyn PermissionCheckerPort>,
}

impl RejectOverrideRequestUseCase {
    pub async fn execute(
        &self,
        ctx: &RequestContext,
        request_id: Uuid,
        reason: String,
    ) -> Result<AcademicOverrideRequest, DomainError> {
        self.permission_checker.check(ctx, Permission::WorkflowApprove).await?;

        let mut req = self.override_repo.find_by_id(request_id).await?
            .ok_or_else(|| DomainError::NotFound(format!("Override request {} not found", request_id)))?;

        if req.status != OverrideStatus::Pending {
            return Err(DomainError::Validation(format!(
                "Cannot reject override request with status {:?}", req.status
            )));
        }

        let now = Utc::now();
        let user_id = ctx.user_id();

        if let Some(step) = req.approval_chain.last_mut() {
            step.status = OverrideStatus::Rejected;
            step.approver_id = user_id;
            step.decision_at = Some(now);
            step.notes = Some(reason.clone());
        }

        req.status = OverrideStatus::Rejected;
        req.resolved_by = user_id;
        req.resolved_at = Some(now);
        req.resolution_notes = Some(reason.clone());
        req.updated_at = now;

        self.override_repo.save(&req).await?;

        let event = DomainEvent::OverrideRejected {
            request_id,
            org_id: req.org_id,
            rejecter_id: user_id.unwrap_or_default(),
            reason,
            timestamp: now,
        };
        self.event_bus.publish(ctx, &event).await?;

        Ok(req)
    }
}
