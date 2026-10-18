use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Domain entity for subject prerequisites.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prerequisite {
    pub id: Uuid,
    pub org_id: Uuid,
    pub subject_id: Uuid,
    pub prerequisite_subject_id: Uuid,
    pub requirement_type: String,
    pub min_grade: Option<String>,
    pub min_score: Option<Decimal>,
}

/// Compat alias for old CoursePrerequisiteType
pub type PrerequisiteType = String;
