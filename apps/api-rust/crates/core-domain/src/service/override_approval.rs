//! Override & Approval Service — risk-based multi-level approval workflow.
//!
//! Risk Levels and Required Approval:
//!
//! | Level    | Required Approvers                       |
//! |----------|------------------------------------------|
//! | low      | Auto-approved (no approval needed)        |
//! | medium   | Department/School Manager                 |
//! | high     | School Manager + Admin                    |
//! | critical | Manager → Admin → Super Admin (sequential)|
//!
//! The approval chain is stored as a JSON array in `approval_chain`.
//! Each step records who approved/rejected and when.

use uuid::Uuid;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait,
    QueryFilter, QueryOrder, Set,
};

use crate::entities::academic_override_request;

// ── Types ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOverrideRequest {
    pub override_type: String,
    pub risk_level: Option<String>,
    pub entity_type: String,
    pub entity_id: String,
    pub field_name: Option<String>,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
    pub reason: String,
    pub requested_by: String,
    pub requested_by_name: Option<String>,
    pub expires_at: Option<String>,
    pub metadata: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalAction {
    pub action: String, // approve | reject | escalate
    pub user_id: String,
    pub user_name: Option<String>,
    pub user_role: String,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalStep {
    pub role: String,
    pub status: String,
    pub by: String,
    pub by_name: Option<String>,
    pub at: String,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverrideListParams {
    pub status: Option<String>,
    pub override_type: Option<String>,
    pub requested_by: Option<String>,
}

// ── Service ────────────────────────────────────────────────────────

impl super::Service {
    /// Create an override request.
    /// If risk_level is "low", the request is auto-approved immediately.
    pub async fn create_override_request(
        &self,
        org_id: &str,
        req: CreateOverrideRequest,
    ) -> Result<academic_override_request::Model, String> {
        let risk_level = req.risk_level.as_deref().unwrap_or("medium");
        let valid_levels = ["low", "medium", "high", "critical"];
        if !valid_levels.contains(&risk_level) {
            return Err(format!(
                "Invalid risk_level: '{}'. Valid: {:?}",
                risk_level, valid_levels
            ));
        }

        let now = Utc::now().to_rfc3339();

        // Determine initial status and approver
        let (status, current_approver_role) = if risk_level == "low" {
            ("approved".to_string(), None)
        } else {
            let first_approver = Self::first_approver_for_risk(risk_level);
            ("pending".to_string(), Some(first_approver.to_string()))
        };

        // For auto-approved (low risk), create the approval chain immediately
        let approval_chain = if risk_level == "low" {
            let step = ApprovalStep {
                role: "system".to_string(),
                status: "approved".to_string(),
                by: "system".to_string(),
                by_name: Some("Auto-approved (low risk)".to_string()),
                at: now.clone(),
                notes: Some("Auto-approved: low risk override".to_string()),
            };
            serde_json::to_string(&vec![step]).unwrap_or_else(|_| "[]".to_string())
        } else {
            "[]".to_string()
        };

        let model = academic_override_request::Model {
            id: Uuid::new_v4().to_string(),
            org_id: org_id.to_string(),
            override_type: req.override_type,
            risk_level: risk_level.to_string(),
            entity_type: req.entity_type,
            entity_id: req.entity_id,
            field_name: req.field_name,
            old_value: req.old_value,
            new_value: req.new_value,
            reason: req.reason,
            requested_by: req.requested_by.clone(),
            requested_by_name: req.requested_by_name,
            requested_at: now.clone(),
            status,
            current_approver_role,
            approval_chain,
            resolved_by: if risk_level == "low" { Some("system".to_string()) } else { None },
            resolved_by_name: if risk_level == "low" { Some("Auto-approved".to_string()) } else { None },
            resolved_at: if risk_level == "low" { Some(now.clone()) } else { None },
            resolution_notes: None,
            expires_at: req.expires_at,
            metadata: req.metadata,
            created_at: now.clone(),
            updated_at: now,
        };

        let active: academic_override_request::ActiveModel = model.into();
        active
            .insert(&self.repo.db)
            .await
            .map_err(|e| e.to_string())
    }

    /// Process an approval action (approve/reject/escalate) on a pending request.
    pub async fn process_approval(
        &self,
        request_id: &str,
        action: ApprovalAction,
    ) -> Result<academic_override_request::Model, String> {
        let existing = academic_override_request::Entity::find_by_id(request_id)
            .one(&self.repo.db)
            .await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "Override request not found".to_string())?;

        if existing.status != "pending" && existing.status != "escalated" {
            return Err(format!(
                "Cannot process request in status '{}'",
                existing.status
            ));
        }

        // Parse existing approval chain
        let mut chain: Vec<ApprovalStep> =
            serde_json::from_str(&existing.approval_chain).unwrap_or_default();

        let now = Utc::now().to_rfc3339();

        // Add the new approval step
        chain.push(ApprovalStep {
            role: action.user_role.clone(),
            status: action.action.clone(),
            by: action.user_id.clone(),
            by_name: action.user_name.clone(),
            at: now.clone(),
            notes: action.notes.clone(),
        });

        let mut active: academic_override_request::ActiveModel = existing.clone().into();
        active.approval_chain = Set(
            serde_json::to_string(&chain).unwrap_or_else(|_| "[]".to_string()),
        );
        active.updated_at = Set(now.clone());

        match action.action.as_str() {
            "approve" => {
                // Check if more approvals are needed (for high/critical)
                let next_approver = Self::next_approver(
                    &existing.risk_level,
                    &action.user_role,
                );

                if let Some(next_role) = next_approver {
                    // Escalate to next approver
                    active.status = Set("escalated".to_string());
                    active.current_approver_role = Set(Some(next_role));
                } else {
                    // Fully approved — resolve
                    active.status = Set("approved".to_string());
                    active.current_approver_role = Set(None);
                    active.resolved_by = Set(Some(action.user_id.clone()));
                    active.resolved_by_name = Set(action.user_name.clone());
                    active.resolved_at = Set(Some(now));
                    active.resolution_notes = Set(action.notes);
                }
            }
            "reject" => {
                active.status = Set("rejected".to_string());
                active.current_approver_role = Set(None);
                active.resolved_by = Set(Some(action.user_id.clone()));
                active.resolved_by_name = Set(action.user_name.clone());
                active.resolved_at = Set(Some(now));
                active.resolution_notes = Set(action.notes);
            }
            "escalate" => {
                let next_approver = Self::next_approver(
                    &existing.risk_level,
                    &action.user_role,
                );
                if let Some(next_role) = next_approver {
                    active.status = Set("escalated".to_string());
                    active.current_approver_role = Set(Some(next_role));
                } else {
                    return Err("Cannot escalate — no higher approver configured".to_string());
                }
            }
            _ => {
                return Err(format!("Invalid action: '{}'", action.action));
            }
        }

        active
            .update(&self.repo.db)
            .await
            .map_err(|e| e.to_string())
    }

    /// Cancel a pending override request (by the requester).
    pub async fn cancel_override_request(
        &self,
        request_id: &str,
        user_id: &str,
    ) -> Result<academic_override_request::Model, String> {
        let existing = academic_override_request::Entity::find_by_id(request_id)
            .one(&self.repo.db)
            .await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "Override request not found".to_string())?;

        if existing.status != "pending" && existing.status != "escalated" {
            return Err("Can only cancel pending or escalated requests".to_string());
        }

        if existing.requested_by != user_id {
            return Err("Only the requester can cancel their own request".to_string());
        }

        let mut active: academic_override_request::ActiveModel = existing.into();
        let now = Utc::now().to_rfc3339();

        active.status = Set("cancelled".to_string());
        active.resolved_at = Set(Some(now.clone()));
        active.resolved_by = Set(Some(user_id.to_string()));
        active.resolution_notes = Set(Some("Cancelled by requester".to_string()));
        active.updated_at = Set(now);

        active
            .update(&self.repo.db)
            .await
            .map_err(|e| e.to_string())
    }

    /// List override requests with optional filters.
    pub async fn list_override_requests(
        &self,
        org_id: &str,
        params: OverrideListParams,
    ) -> Result<Vec<academic_override_request::Model>, String> {
        let mut query = academic_override_request::Entity::find()
            .filter(academic_override_request::Column::OrgId.eq(org_id));

        if let Some(ref status) = params.status {
            query = query.filter(academic_override_request::Column::Status.eq(status.as_str()));
        }
        if let Some(ref override_type) = params.override_type {
            query = query.filter(
                academic_override_request::Column::OverrideType.eq(override_type.as_str()),
            );
        }
        if let Some(ref requested_by) = params.requested_by {
            query = query.filter(
                academic_override_request::Column::RequestedBy.eq(requested_by.as_str()),
            );
        }

        query
            .order_by_desc(academic_override_request::Column::CreatedAt)
            .all(&self.repo.db)
            .await
            .map_err(|e| e.to_string())
    }

    /// Get a single override request by ID.
    pub async fn get_override_request(
        &self,
        request_id: &str,
    ) -> Result<academic_override_request::Model, String> {
        academic_override_request::Entity::find_by_id(request_id)
            .one(&self.repo.db)
            .await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "Override request not found".to_string())
    }

    /// Get requests pending for a specific approver role.
    pub async fn get_pending_approvals(
        &self,
        org_id: &str,
        approver_role: &str,
    ) -> Result<Vec<academic_override_request::Model>, String> {
        academic_override_request::Entity::find()
            .filter(academic_override_request::Column::OrgId.eq(org_id))
            .filter(
                academic_override_request::Column::Status
                    .is_in(["pending", "escalated"]),
            )
            .filter(
                academic_override_request::Column::CurrentApproverRole.eq(approver_role),
            )
            .order_by_asc(academic_override_request::Column::CreatedAt)
            .all(&self.repo.db)
            .await
            .map_err(|e| e.to_string())
    }

    // ── Internal helpers ──────────────────────────────────────────

    /// Get the first approver role for a risk level.
    fn first_approver_for_risk(risk_level: &str) -> &'static str {
        match risk_level {
            "medium" => "school_manager",
            "high" => "school_manager",
            "critical" => "school_manager",
            _ => "school_manager",
        }
    }

    /// Get the next approver role in the chain, or None if fully approved.
    fn next_approver(risk_level: &str, current_role: &str) -> Option<String> {
        match (risk_level, current_role) {
            // Medium: school_manager is enough
            ("medium", _) => None,

            // High: school_manager → admin
            ("high", "school_manager") => Some("admin".to_string()),
            ("high", _) => None,

            // Critical: school_manager → admin → super_admin
            ("critical", "school_manager") => Some("admin".to_string()),
            ("critical", "admin") => Some("super_admin".to_string()),
            ("critical", _) => None,

            _ => None,
        }
    }
}
