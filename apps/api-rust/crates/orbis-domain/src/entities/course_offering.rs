use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CourseOfferingStatus {
    Draft,
    Open,
    InProgress,
    Completed,
    Cancelled,
}

impl std::fmt::Display for CourseOfferingStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Draft => write!(f, "draft"),
            Self::Open => write!(f, "open"),
            Self::InProgress => write!(f, "in_progress"),
            Self::Completed => write!(f, "completed"),
            Self::Cancelled => write!(f, "cancelled"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseOffering {
    pub id: Uuid,
    pub org_id: Uuid,
    pub course_id: Uuid,
    pub period_id: Option<Uuid>,
    pub section_code: Option<String>,
    pub name: Option<String>,
    pub status: CourseOfferingStatus,
    pub capacity: Option<u32>,
    pub starts_at: Option<DateTime<Utc>>,
    pub ends_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
