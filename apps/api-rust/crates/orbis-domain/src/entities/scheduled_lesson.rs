use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledLesson {
    pub id: Uuid,
    pub class_subject_id: Uuid,
    pub room_id: Option<Uuid>,
    pub time_slot_id: Uuid,
    pub teacher_id: Option<Uuid>, // Could override the primary teacher
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
