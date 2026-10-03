use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExamSession {
    pub id: Uuid,
    pub org_id: Uuid,
    pub period_id: Uuid,
    pub name: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub status: String, // Scheduled, Ongoing, Completed
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
