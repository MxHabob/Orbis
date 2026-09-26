use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RequirementTargetType {
    Global,
    Stage,
    GradeLevel,
    Program,
}

impl std::fmt::Display for RequirementTargetType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RequirementTargetType::Global => write!(f, "global"),
            RequirementTargetType::Stage => write!(f, "stage"),
            RequirementTargetType::GradeLevel => write!(f, "grade_level"),
            RequirementTargetType::Program => write!(f, "program"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocumentCondition {
    AllStudents,
    NonNationalOnly,
    TransferStudentsOnly,
    SpecialNeedsOnly,
}

impl std::fmt::Display for DocumentCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DocumentCondition::AllStudents => write!(f, "all_students"),
            DocumentCondition::NonNationalOnly => write!(f, "non_national_only"),
            DocumentCondition::TransferStudentsOnly => write!(f, "transfer_students_only"),
            DocumentCondition::SpecialNeedsOnly => write!(f, "special_needs_only"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VerificationStatus {
    Pending,
    Verified,
    Rejected,
    Expired,
}

impl std::fmt::Display for VerificationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VerificationStatus::Pending => write!(f, "pending"),
            VerificationStatus::Verified => write!(f, "verified"),
            VerificationStatus::Rejected => write!(f, "rejected"),
            VerificationStatus::Expired => write!(f, "expired"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentRequirement {
    pub id: Uuid,
    pub org_id: Uuid,
    pub target_type: RequirementTargetType,
    pub target_id: Option<Uuid>,
    pub code: String,
    pub name_ar: String,
    pub name_en: String,
    pub is_mandatory: bool,
    pub blocks_enrollment: bool,
    pub condition: DocumentCondition,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl DocumentRequirement {
    pub fn new(
        org_id: Uuid,
        target_type: RequirementTargetType,
        target_id: Option<Uuid>,
        code: String,
        name_ar: String,
        name_en: String,
        is_mandatory: bool,
        blocks_enrollment: bool,
        condition: DocumentCondition,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            org_id,
            target_type,
            target_id,
            code,
            name_ar,
            name_en,
            is_mandatory,
            blocks_enrollment,
            condition,
            created_at: now,
            updated_at: now,
        }
    }
}
