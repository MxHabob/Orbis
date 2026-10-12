use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Domain entity for section-subject assignment — unified teaching assignment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SectionSubject {
    pub id: Uuid,
    pub org_id: Uuid,
    pub section_id: Uuid,
    pub subject_id: Uuid,
    pub period_id: Uuid,
    pub instructor_id: Option<Uuid>,
    pub assistant_instructor_id: Option<Uuid>,
    pub scheme_id: Option<Uuid>,
    pub weekly_hours: Option<i32>,
    pub status: String,
    pub is_active: bool,
}

/// Compat alias
pub type SectionSubjectstatus = String;

