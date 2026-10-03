use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretNumber {
    pub id: Uuid,
    pub exam_schedule_id: Uuid, // Specific to a particular subject's exam
    pub student_id: Uuid,
    pub secret_number: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
