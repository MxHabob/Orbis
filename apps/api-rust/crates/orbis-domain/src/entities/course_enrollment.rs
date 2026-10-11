use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CourseEnrollmentStatus {
    Pending,
    Active,
    Withdrawn,
    Completed,
    Failed,
    Exempt,
}

impl std::fmt::Display for CourseEnrollmentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pending => write!(f, "pending"),
            Self::Active => write!(f, "active"),
            Self::Withdrawn => write!(f, "withdrawn"),
            Self::Completed => write!(f, "completed"),
            Self::Failed => write!(f, "failed"),
            Self::Exempt => write!(f, "exempt"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseEnrollment {
    pub id: Uuid,
    pub org_id: Uuid,
    pub offering_id: Uuid,
    pub student_id: Uuid,
    pub attempt_number: u32,
    pub status: CourseEnrollmentStatus,
    pub enrolled_at: DateTime<Utc>,
    pub withdrawn_at: Option<DateTime<Utc>>,
    pub withdrawal_reason: Option<String>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
