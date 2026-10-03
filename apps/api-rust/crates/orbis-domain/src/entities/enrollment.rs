use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StudyMode {
    Regular,
    Homeschooled,
    Auditor,
    Visiting,
    DistanceLearning,
    PartTime,
    Exchange,
    SpecialNeeds,
}

impl std::fmt::Display for StudyMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StudyMode::Regular => write!(f, "regular"),
            StudyMode::Homeschooled => write!(f, "homeschooled"),
            StudyMode::Auditor => write!(f, "auditor"),
            StudyMode::Visiting => write!(f, "visiting"),
            StudyMode::DistanceLearning => write!(f, "distance_learning"),
            StudyMode::PartTime => write!(f, "part_time"),
            StudyMode::Exchange => write!(f, "exchange"),
            StudyMode::SpecialNeeds => write!(f, "special_needs"),
        }
    }
}

impl StudyMode {
    pub fn default_attendance_exempt(&self) -> bool {
        matches!(self, StudyMode::Homeschooled)
    }

    pub fn default_grade_exempt(&self) -> bool {
        matches!(self, StudyMode::Auditor)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EnrollmentStatus {
    Active,
    PendingApproval,
    OnHold,
    Withdrawn,
    Transferred,
    Graduated,
    Suspended,
}

impl std::fmt::Display for EnrollmentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EnrollmentStatus::Active => write!(f, "active"),
            EnrollmentStatus::PendingApproval => write!(f, "pending_approval"),
            EnrollmentStatus::OnHold => write!(f, "on_hold"),
            EnrollmentStatus::Withdrawn => write!(f, "withdrawn"),
            EnrollmentStatus::Transferred => write!(f, "transferred"),
            EnrollmentStatus::Graduated => write!(f, "graduated"),
            EnrollmentStatus::Suspended => write!(f, "suspended"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Enrollment {
    pub id: Uuid,
    pub org_id: Uuid,
    pub period_id: Uuid,
    pub student_id: Uuid,
    pub class_id: Option<Uuid>,
    pub student_number: String,
    pub study_mode: StudyMode,
    pub status: EnrollmentStatus,
    pub is_attendance_exempt: bool,
    pub is_grade_exempt: bool,
    pub enrolled_at: DateTime<Utc>,
    pub withdrawn_at: Option<DateTime<Utc>>,
    pub withdrawal_reason: Option<String>,
    pub meta: serde_json::Value,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Enrollment {
    pub fn new(
        org_id: Uuid,
        period_id: Uuid,
        student_id: Uuid,
        class_id: Option<Uuid>,
        student_number: String,
        study_mode: StudyMode,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            org_id,
            period_id,
            student_id,
            class_id,
            student_number,
            is_attendance_exempt: study_mode.default_attendance_exempt(),
            is_grade_exempt: study_mode.default_grade_exempt(),
            study_mode,
            status: EnrollmentStatus::Active,
            enrolled_at: now,
            withdrawn_at: None,
            withdrawal_reason: None,
            meta: serde_json::json!({}),
            created_by: None,
            updated_by: None,
            created_at: now,
            updated_at: now,
        }
    }
}
