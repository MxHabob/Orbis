use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExamProctor {
    pub id: Uuid,
    pub exam_schedule_id: Uuid,
    pub room_id: Uuid,
    pub teacher_id: Uuid,
    pub is_chief: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
