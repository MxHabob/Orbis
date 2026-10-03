use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeatingNumber {
    pub id: Uuid,
    pub exam_session_id: Uuid,
    pub student_id: Uuid,
    pub seating_number: String,
    pub room_id: Option<Uuid>, // We are reusing Room from Scheduling module
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
