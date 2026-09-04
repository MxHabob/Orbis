use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Grade {
    pub id: Uuid,
    pub student_id: Uuid,
    pub class_subject_id: Uuid,
    pub scheme_id: Uuid, // Or scheme_component_id if we want finer granularity
    pub score: f64,
    pub is_excused: bool,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
