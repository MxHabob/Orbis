//! Policy operations — concrete PolicyEngine implementation.
//!
//! Reads the `org_academic_policies` table to determine approval requirements.

use async_trait::async_trait;
use sea_orm::{EntityTrait, ColumnTrait, QueryFilter};

use shared_kernel::errors::AppError;
use shared_kernel::traits::PolicyEngine;

use crate::entities::org_academic_policy;

// ═══════════════════════════════════════════════════════════════
// Concrete PolicyEngine Implementation
// ═══════════════════════════════════════════════════════════════

/// Concrete `PolicyEngine` backed by the `org_academic_policies` table.
///
/// Evaluates whether specific actions require approval based on org policies.
pub struct OrgPolicyEngine {
    db: sea_orm::DatabaseConnection,
}

impl OrgPolicyEngine {
    pub fn new(db: sea_orm::DatabaseConnection) -> Self {
        Self { db }
    }

    /// Load the policy for an organization.
    async fn get_policy(&self, org_id: &str) -> Result<Option<org_academic_policy::Model>, AppError> {
        let policy = org_academic_policy::Entity::find()
            .filter(org_academic_policy::Column::OrgId.eq(org_id))
            .one(&self.db)
            .await?;
        Ok(policy)
    }
}

#[async_trait]
impl PolicyEngine for OrgPolicyEngine {
    async fn requires_approval(&self, action: &str, org_id: &str) -> Result<bool, AppError> {
        let policy = self.get_policy(org_id).await?;

        let Some(policy) = policy else {
            // No policy configured — no approval required by default
            return Ok(false);
        };

        // Check action-specific approval requirements
        let requires = match action {
            // Late enrollment
            "enrollment.create_late" => policy.late_enrollment_requires_approval == 1,

            // Grade modification after cycle close
            "grade.modify_after_close" => policy.grade_modification_requires_approval == 1,

            // Generic — check the settings JSON for custom approval rules
            _ => {
                if let Some(ref settings_str) = policy.settings {
                    if let Ok(settings) = serde_json::from_str::<serde_json::Value>(settings_str) {
                        settings.get("approval_required")
                            .and_then(|v| v.get(action))
                            .and_then(|v| v.as_bool())
                            .unwrap_or(false)
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
        };

        Ok(requires)
    }

    async fn check_override(
        &self,
        _action: &str,
        org_id: &str,
        override_id: &str,
    ) -> Result<bool, AppError> {
        use crate::entities::academic_override_request;

        // Check if an approved override exists
        let override_req = academic_override_request::Entity::find()
            .filter(academic_override_request::Column::Id.eq(override_id))
            .filter(academic_override_request::Column::OrgId.eq(org_id))
            .filter(academic_override_request::Column::Status.eq("approved"))
            .one(&self.db)
            .await?;

        Ok(override_req.is_some())
    }
}
