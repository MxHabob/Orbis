//! Policy operations — concrete PolicyEngine implementation.
//!
//! Reads the `org_academic_policies` table to determine approval requirements.

use async_trait::async_trait;
use sea_orm::{EntityTrait, ColumnTrait, QueryFilter};

use shared_kernel::errors::AppError;
use shared_kernel::traits::PolicyEngine;

use crate::entities::academic_policy;

// ═══════════════════════════════════════════════════════════════
// Concrete PolicyEngine Implementation
// ═══════════════════════════════════════════════════════════════

/// Concrete `PolicyEngine` backed by the `academic_policies` table.
///
/// Evaluates whether specific actions require approval based on org policies.
/// Supports branch-level policy overrides via `org_unit_id`.
pub struct PlatformPolicyEngine {
    db: sea_orm::DatabaseConnection,
}

impl PlatformPolicyEngine {
    pub fn new(db: sea_orm::DatabaseConnection) -> Self {
        Self { db }
    }

    /// Load the default policy for an organization (org_unit_id = NULL).
    async fn get_policy(&self, org_id: &str) -> Result<Option<academic_policy::Model>, AppError> {
        let policy = academic_policy::Entity::find()
            .filter(academic_policy::Column::OrgId.eq(org_id))
            .filter(academic_policy::Column::OrgUnitId.is_null())
            .one(&self.db)
            .await?;
        Ok(policy)
    }

    /// Resolve the effective policy for an org + optional org_unit.
    ///
    /// 1. If `org_unit_id` is provided, look for a unit-specific override.
    /// 2. If no override found, fall back to the org-wide default.
    pub async fn get_effective_policy(
        &self,
        org_id: &str,
        org_unit_id: Option<&str>,
    ) -> Result<Option<academic_policy::Model>, AppError> {
        // Try unit-specific override first
        if let Some(unit_id) = org_unit_id {
            let unit_policy = academic_policy::Entity::find()
                .filter(academic_policy::Column::OrgId.eq(org_id))
                .filter(academic_policy::Column::OrgUnitId.eq(unit_id))
                .one(&self.db)
                .await?;
            if unit_policy.is_some() {
                return Ok(unit_policy);
            }
        }

        // Fall back to org-wide default
        self.get_policy(org_id).await
    }
}

#[async_trait]
impl PolicyEngine for PlatformPolicyEngine {
    async fn requires_approval(&self, action: &str, org_id: &str) -> Result<bool, AppError> {
        let policy = self.get_policy(org_id).await?;

        let Some(policy) = policy else {
            // No policy configured — no approval required by default
            return Ok(false);
        };

        // Check action-specific approval requirements
        let requires = match action {
            // Late enrollment
            "enrollment.create_late" => policy.late_enrollment_approval == 1,

            // Grade modification after period close
            "grade.modify_after_close" => policy.grade_edit_approval == 1,

            // Generic — check the settings JSON for custom approval rules (Removed in restructuring)
            _ => false,
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
