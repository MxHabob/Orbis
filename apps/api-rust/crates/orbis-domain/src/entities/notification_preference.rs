use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Per-user notification preferences within an organization.
///
/// Controls which notification categories are enabled/disabled and through
/// which channels (in-app, email, push) the user wants to receive them.
/// These are stored server-side and override the organization defaults.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserNotificationPreference {
    pub id: Uuid,
    pub user_id: Uuid,
    pub org_id: Uuid,
    /// Notification category (e.g. "enrollment", "grades", "attendance").
    pub category: String,
    /// Whether in-app notifications are enabled for this category.
    pub in_app_enabled: bool,
    /// Whether email notifications are enabled for this category.
    pub email_enabled: bool,
    /// Whether push notifications are enabled for this category.
    pub push_enabled: bool,
}

impl UserNotificationPreference {
    /// Returns a default preference with all channels enabled.
    pub fn default_for(user_id: Uuid, org_id: Uuid, category: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            org_id,
            category: category.to_string(),
            in_app_enabled: true,
            email_enabled: false,
            push_enabled: true,
        }
    }
}
