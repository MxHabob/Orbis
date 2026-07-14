//! Tenant Guard — enforces tenant boundary at the session level.
//!
//! Every request that touches tenant data must pass through TenantGuard.
//! The guard ensures that the `org_id` used in operations matches the
//! authenticated session's organization, preventing cross-tenant access.
//!
//! # Super Admin
//!
//! Super Admins can operate on multiple organizations. Their `allowed_org_ids`
//! list is populated at login time from the IAM service.

use serde::{Deserialize, Serialize};

use crate::errors::AppError;

/// Enforces tenant isolation at the session level.
///
/// Created once per login session and attached to the application state.
/// All context-building operations must validate through this guard.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantGuard {
    /// The primary org_id from the authenticated session.
    session_org_id: String,
    /// Additional org_ids the user is authorized to access (Super Admin).
    allowed_org_ids: Vec<String>,
}

impl TenantGuard {
    /// Create a guard for a regular user (single org).
    pub fn new(session_org_id: impl Into<String>) -> Self {
        Self {
            session_org_id: session_org_id.into(),
            allowed_org_ids: Vec::new(),
        }
    }

    /// Create a guard for a Super Admin (multiple orgs).
    pub fn with_allowed_orgs(
        session_org_id: impl Into<String>,
        allowed_org_ids: Vec<String>,
    ) -> Self {
        Self {
            session_org_id: session_org_id.into(),
            allowed_org_ids,
        }
    }

    /// Validate that the requested org_id is authorized for this session.
    ///
    /// Returns `Ok(())` if the org_id matches the session or is in the
    /// allowed list. Returns `Err(AppError::forbidden)` otherwise.
    pub fn validate(&self, requested_org_id: &str) -> Result<(), AppError> {
        if requested_org_id == self.session_org_id {
            return Ok(());
        }
        if self.allowed_org_ids.iter().any(|id| id == requested_org_id) {
            return Ok(());
        }
        Err(AppError::forbidden(
            "Tenant boundary violation: unauthorized org_id",
        ))
    }

    /// Get the session's primary org_id.
    pub fn org_id(&self) -> &str {
        &self.session_org_id
    }

    /// Check if this guard has Super Admin access to multiple orgs.
    pub fn is_multi_org(&self) -> bool {
        !self.allowed_org_ids.is_empty()
    }
}
