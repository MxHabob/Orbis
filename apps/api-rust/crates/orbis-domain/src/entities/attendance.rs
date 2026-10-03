use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attendance {
    pub id: Uuid,
    pub student_id: Uuid,
    pub class_id: Option<Uuid>,
    pub class_subject_id: Option<Uuid>,
    pub scheduled_lesson_id: Option<Uuid>,
    pub attendance_date: NaiveDate,
    pub status: String, // Present, Absent, Late, Excused
    pub notes: Option<String>,
    pub marked_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
