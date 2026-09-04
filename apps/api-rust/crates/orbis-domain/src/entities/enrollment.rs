use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Enrollment {
    pub id: Uuid,
    pub org_id: Uuid,
    pub period_id: Uuid,
    pub student_id: Uuid,
    pub class_id: Uuid,
    pub student_number: String,
    pub status: String,
    pub enrollment_date: DateTime<Utc>,
    pub enrollment_type: String,
    pub withdrawal_date: Option<DateTime<Utc>>,
    pub withdrawal_reason: Option<String>,
    pub meta: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
