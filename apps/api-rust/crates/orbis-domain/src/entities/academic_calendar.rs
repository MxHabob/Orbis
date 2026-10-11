use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AcademicCalendarStatus {
    Planning,
    Active,
    Closed,
    Archived,
}

impl std::fmt::Display for AcademicCalendarStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Planning => write!(f, "planning"),
            Self::Active => write!(f, "active"),
            Self::Closed => write!(f, "closed"),
            Self::Archived => write!(f, "archived"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcademicCalendar {
    pub id: Uuid,
    pub org_id: Uuid,
    pub code: Option<String>,
    pub name: String,
    pub name_ar: Option<String>,
    pub starts_at: Option<DateTime<Utc>>,
    pub ends_at: Option<DateTime<Utc>>,
    pub status: AcademicCalendarStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcademicPeriodType {
    pub id: Uuid,
    pub org_id: Uuid,
    pub code: String,
    pub name: String,
    pub name_ar: Option<String>,
    pub sort_order: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcademicCalendarPeriod {
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
