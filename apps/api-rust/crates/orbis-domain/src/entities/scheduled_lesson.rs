use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledLesson {
    pub id: Uuid,
    pub section_subject_id: Uuid,
    pub room_id: Option<Uuid>,
    pub time_slot_id: Uuid,
    pub teacher_id: Option<Uuid>, // Could override the primary teacher
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
