use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorRecord {
    pub id: Uuid,
    pub student_id: Uuid,
    pub org_id: Uuid,
    pub reporter_id: Uuid,   // User ID of who reported this
    pub record_type: String, // Violation, Merit
    pub description: String,
    pub points: i32, // negative for violation, positive for merit
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
