//! Entitlement contract — boundary between Orbis and a future external Licensing Service.
//!
//! Orbis consumes `EntitlementResult` from the local license runtime.

use serde::{Deserialize, Serialize};

/// The result of an entitlement check — what the external Licensing Service would return.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntitlementResult {
    /// Current entitlement status.
    pub status: EntitlementStatus,
    /// Plan identifier, e.g. "basic", "professional", "enterprise".
    pub plan: Option<String>,
    /// Maximum allowed users (None = no limit).
    pub max_users: Option<u32>,
    /// Maximum allowed students (None = no limit).
    pub max_students: Option<u32>,
    /// Enabled feature flags.
    pub features: Vec<String>,
    /// Expiration date (ISO 8601). None = no expiry.
    pub expires_at: Option<String>,
    /// Grace period end date (ISO 8601). None = no grace period.
    pub grace_period_until: Option<String>,
}

/// Entitlement status values.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum EntitlementStatus {
    /// License is active and valid.
    Active,
    /// License is valid but not yet in its validity window.
    Pending,
    /// Trial period is active.
    Trial,
    /// License has expired.
    Expired,
    /// License is temporarily disabled by the licensing authority.
    Suspended,
    /// License expired but within grace period.
    GracePeriod,
    /// No license is activated.
    Unlicensed,
}

impl EntitlementResult {
    /// Returns the no-license state. It grants no paid capabilities.
    pub fn unlicensed() -> Self {
        Self {
            status: EntitlementStatus::Unlicensed,
            plan: None,
            max_users: None,
            max_students: None,
            features: vec![],
            expires_at: None,
            grace_period_until: None,
        }
    }

    /// Whether the system should allow licensed operations.
    pub fn is_operational(&self) -> bool {
        matches!(
            self.status,
            EntitlementStatus::Active | EntitlementStatus::Trial | EntitlementStatus::GracePeriod
        )
    }
}

#[cfg(test)]
mod tests {
    use super::EntitlementResult;

    #[test]
    fn unlicensed_entitlement_does_not_grant_operations() {
        let entitlement = EntitlementResult::unlicensed();
        assert!(!entitlement.is_operational());
        assert!(entitlement.features.is_empty());
        assert_eq!(entitlement.max_users, None);
        assert_eq!(entitlement.max_students, None);
    }
}
