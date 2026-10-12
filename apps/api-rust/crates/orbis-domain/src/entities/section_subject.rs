use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Domain entity for section-subject assignment — unified teaching assignment.
/// Works for schools, universities, institutes, and training centers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SectionSubject {
    pub id: Uuid,
    pub org_id: Uuid,
    pub section_id: Uuid,
    pub subject_id: Uuid,
    pub period_id: Uuid,
    pub instructor_id: Option<Uuid>,
    pub assistant_instructor_id: Option<Uuid>,
    pub substitute_instructor_id: Option<Uuid>,
    pub scheme_id: Option<Uuid>,
    pub weekly_hours: Option<i32>,
    pub theory_hours: Option<i32>,
    pub practical_hours: Option<i32>,
    pub assessment_config: Option<serde_json::Value>,
    pub grade_groups: Option<serde_json::Value>,
    pub capacity: Option<i32>,
    pub status: String,
    pub is_active: bool,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// Compat alias
pub type SectionSubjectstatus = String;
