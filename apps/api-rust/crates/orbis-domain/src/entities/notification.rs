use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Category of a notification — determines which preference toggle controls it
/// and how the UI groups/filters notifications.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NotificationCategory {
    Enrollment,
    Grades,
    Attendance,
    Finance,
    Behavior,
    Schedule,
    Approval,
    System,
    Announcement,
}

impl NotificationCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Enrollment => "enrollment",
            Self::Grades => "grades",
            Self::Attendance => "attendance",
            Self::Finance => "finance",
            Self::Behavior => "behavior",
            Self::Schedule => "schedule",
            Self::Approval => "approval",
            Self::System => "system",
            Self::Announcement => "announcement",
        }
    }

    pub fn from_str(val: &str) -> Option<Self> {
        match val {
            "enrollment" => Some(Self::Enrollment),
            "grades" => Some(Self::Grades),
            "attendance" => Some(Self::Attendance),
            "finance" => Some(Self::Finance),
            "behavior" => Some(Self::Behavior),
            "schedule" => Some(Self::Schedule),
            "approval" => Some(Self::Approval),
            "system" => Some(Self::System),
            "announcement" => Some(Self::Announcement),
            _ => None,
        }
    }
}

impl std::fmt::Display for NotificationCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Priority of a notification — influences delivery urgency and policy behavior.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NotificationPriority {
    /// Eligible for digests; may be batched.
    Low,
    /// Standard delivery.
    Normal,
    /// Bypasses quiet hours.
    High,
    /// Immediate delivery + escalation.
    Urgent,
}

impl NotificationPriority {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Low => "low",
            Self::Normal => "normal",
            Self::High => "high",
            Self::Urgent => "urgent",
        }
    }

    pub fn from_str(val: &str) -> Option<Self> {
        match val {
            "low" => Some(Self::Low),
            "normal" => Some(Self::Normal),
            "high" => Some(Self::High),
            "urgent" => Some(Self::Urgent),
            _ => None,
        }
    }
}

impl Default for NotificationPriority {
    fn default() -> Self {
        Self::Normal
    }
}

impl std::fmt::Display for NotificationPriority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// A persisted notification record targeted at a specific user within an organization.
///
/// Notifications are generated from domain events by the `NotificationService`.
/// They are distinct from transient UI toasts — notifications persist, support
/// read/dismissed state, and can drive in-app notification centers and badges.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    pub id: Uuid,
    pub org_id: Uuid,
    pub recipient_user_id: Uuid,
    pub category: String,
    pub priority: String,
    /// English title.
    pub title: String,
    /// Arabic title (optional).
    pub title_ar: Option<String>,
    /// English body.
    pub body: String,
    /// Arabic body (optional).
    pub body_ar: Option<String>,
    /// The type of the related entity (e.g. "student", "enrollment", "grade").
    pub entity_type: Option<String>,
    /// The UUID of the related entity for deep-link navigation.
    pub entity_id: Option<Uuid>,
    /// Client-side navigation route (e.g. "/(app)/student/UUID").
    pub action_route: Option<String>,
    /// The `DomainEvent` variant name that triggered this notification.
    pub source_event: String,
    /// Deduplication key — prevents creating duplicate notifications for the same event.
    pub correlation_id: String,
    /// Schema version for forward compatibility of notification payloads.
    pub schema_version: i32,
    pub is_read: bool,
    pub read_at: Option<DateTime<Utc>>,
    pub is_dismissed: bool,
    pub dismissed_at: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}
