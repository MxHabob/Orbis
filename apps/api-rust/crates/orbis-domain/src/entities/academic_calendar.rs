use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AcademicCalendarStatus {
    Planning,
    Active,
    Closing,
    Closed,
    Archived,
}

impl std::fmt::Display for AcademicCalendarStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Planning => write!(f, "planning"),
            Self::Active => write!(f, "active"),
            Self::Closing => write!(f, "closing"),
            Self::Closed => write!(f, "closed"),
            Self::Archived => write!(f, "archived"),
        }
    }
}

impl From<&str> for AcademicCalendarStatus {
    fn from(s: &str) -> Self {
        match s {
            "planning" => Self::Planning,
            "closing" => Self::Closing,
            "closed" => Self::Closed,
            "archived" => Self::Archived,
            _ => Self::Active,
        }
    }
}

impl PartialEq<&str> for AcademicCalendarStatus {
    fn eq(&self, other: &&str) -> bool {
        match self {
            Self::Planning => *other == "planning",
            Self::Active => *other == "active",
            Self::Closing => *other == "closing",
            Self::Closed => *other == "closed",
            Self::Archived => *other == "archived",
        }
    }
}

impl PartialEq<str> for AcademicCalendarStatus {
    fn eq(&self, other: &str) -> bool {
        match self {
            Self::Planning => other == "planning",
            Self::Active => other == "active",
            Self::Closing => other == "closing",
            Self::Closed => other == "closed",
            Self::Archived => other == "archived",
        }
    }
}

/// Unified academic calendar entity (formerly academic year).
/// Top-level temporal container for schools, universities, institutes, and training centers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcademicCalendar {
    pub id: Uuid,
    pub org_id: Uuid,
    pub code: Option<String>,
    pub name: String,
    pub name_ar: Option<String>,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub starts_at: Option<DateTime<Utc>>,
    pub ends_at: Option<DateTime<Utc>>,
    pub hijri_start: Option<String>,
    pub hijri_end: Option<String>,
    pub status: AcademicCalendarStatus,
    pub is_locked: bool,
    pub locked_at: Option<DateTime<Utc>>,
    pub locked_by: Option<Uuid>,
    pub lock_reason: Option<String>,
    pub current_period_id: Option<Uuid>,
    pub archived_at: Option<DateTime<Utc>>,
    pub archived_by: Option<Uuid>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Configurable period type catalog entity for academic calendar framework.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeriodType {
    pub id: Uuid,
    pub org_id: Uuid,
    pub code: String,
    pub name: String,
    pub name_ar: Option<String>,
    pub sort_order: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Academic period within a calendar framework.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcademicPeriod {
    pub id: Uuid,
    pub org_id: Uuid,
    pub calendar_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub period_type_id: Uuid,
    pub code: Option<String>,
    pub name: String,
    pub name_ar: Option<String>,
    pub starts_at: Option<DateTime<Utc>>,
    pub ends_at: Option<DateTime<Utc>>,
    pub sort_order: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Backward compatibility alias
pub type AcademicYear = AcademicCalendar;
