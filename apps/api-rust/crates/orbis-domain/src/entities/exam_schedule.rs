use uuid::Uuid;
use chrono::{DateTime, Utc, NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExamSchedule {
    pub id: Uuid,
    pub exam_session_id: Uuid,
    pub class_subject_id: Uuid,
    pub exam_date: NaiveDate,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
