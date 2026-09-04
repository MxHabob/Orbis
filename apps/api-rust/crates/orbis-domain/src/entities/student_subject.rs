use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudentSubject {
    pub id: Uuid,
    pub org_id: Uuid,
    pub student_id: Uuid,
    pub class_subject_id: Uuid,
    pub enrollment_id: Option<Uuid>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
