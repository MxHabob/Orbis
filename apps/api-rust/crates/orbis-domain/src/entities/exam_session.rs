use uuid::Uuid;
use chrono::{DateTime, Utc, NaiveDate};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExamSession {
    pub id: Uuid,
    pub org_id: Uuid,
    pub academic_period_id: Uuid,
    pub name: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub status: String, // Scheduled, Ongoing, Completed
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
