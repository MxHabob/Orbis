use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PrerequisiteType {
    Enforce,
    Recommended,
    Corequisite,
}

impl std::fmt::Display for PrerequisiteType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PrerequisiteType::Enforce => write!(f, "enforce"),
            PrerequisiteType::Recommended => write!(f, "recommended"),
            PrerequisiteType::Corequisite => write!(f, "corequisite"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoursePrerequisite {
    pub id: Uuid,
    pub org_id: Uuid,
    pub subject_id: Uuid,
    pub prerequisite_subject_id: Uuid,
    pub requirement_type: PrerequisiteType,
    pub min_grade: Option<String>,
    pub min_score: Option<rust_decimal::Decimal>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
