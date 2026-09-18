//! Entitlement contract — boundary between Orbis and a future external Licensing Service.
//!
//! Orbis consumes `EntitlementResult`; a future Licensing Service produces it.
//! In this phase, the default provider always returns `Unlicensed` (unrestricted).

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
    /// Trial period is active.
    Trial,
    /// License has expired.
    Expired,
    /// License expired but within grace period.
    GracePeriod,
    /// No license — system runs with defaults (unrestricted in this phase).
    Unlicensed,
}

impl EntitlementResult {
    /// Returns an unrestricted entitlement — the default for this phase.
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

    /// Whether the system should operate normally (Active, Trial, GracePeriod, or Unlicensed).
    pub fn is_operational(&self) -> bool {
        matches!(
            self.status,
            EntitlementStatus::Active
                | EntitlementStatus::Trial
                | EntitlementStatus::GracePeriod
                | EntitlementStatus::Unlicensed
        )
    }
}
