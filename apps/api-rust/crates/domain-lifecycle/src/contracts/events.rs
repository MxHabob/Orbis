//! Organization domain events.

/// Event types emitted by organization operations.
pub mod event_types {
    pub const ORGANIZATION_CREATED: &str = "organization.created";
    pub const ORGANIZATION_UPDATED: &str = "organization.updated";
    pub const SETTINGS_CHANGED: &str = "organization.settings_changed";
    pub const POLICY_CHANGED: &str = "organization.policy_changed";
}
