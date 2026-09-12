use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AcademicPeriodState {
    Planning,
    EnrollmentOpen,
    ActiveTeaching,
    FinalExams,
    GradeSubmission,
    Closed,
    Archived,
}

impl std::fmt::Display for AcademicPeriodState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AcademicPeriodState::Planning => write!(f, "planning"),
            AcademicPeriodState::EnrollmentOpen => write!(f, "enrollment_open"),
            AcademicPeriodState::ActiveTeaching => write!(f, "active_teaching"),
            AcademicPeriodState::FinalExams => write!(f, "final_exams"),
            AcademicPeriodState::GradeSubmission => write!(f, "grade_submission"),
            AcademicPeriodState::Closed => write!(f, "closed"),
            AcademicPeriodState::Archived => write!(f, "archived"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcademicPeriod {
    pub id: Uuid,
    pub org_id: Uuid,
    pub year_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub name: String,
    pub name_ar: Option<String>,
    pub code: Option<String>,
    pub period_type: String, // semester, trimester, quarter, module, block
    pub state: AcademicPeriodState,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub hijri_start: Option<String>,
    pub hijri_end: Option<String>,
    pub enrollment_starts_at: Option<DateTime<Utc>>,
    pub enrollment_ends_at: Option<DateTime<Utc>>,
    pub is_enrollment_open: bool,
    pub grade_entry_starts_at: Option<DateTime<Utc>>,
    pub grade_entry_ends_at: Option<DateTime<Utc>>,
    pub is_grade_entry_open: bool,
    pub exam_starts_at: Option<DateTime<Utc>>,
    pub exam_ends_at: Option<DateTime<Utc>>,
    pub is_locked: bool,
    pub locked_at: Option<DateTime<Utc>>,
    pub locked_by: Option<Uuid>,
    pub sort_order: Option<i32>,
    pub metadata: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl AcademicPeriod {
    pub fn can_enter_grades(&self) -> bool {
        if self.is_locked {
            return false;
        }
        if matches!(self.state, AcademicPeriodState::GradeSubmission | AcademicPeriodState::ActiveTeaching | AcademicPeriodState::FinalExams) {
            let now = Utc::now();
            let after_start = self.grade_entry_starts_at.map(|s| now >= s).unwrap_or(true);
            let before_end = self.grade_entry_ends_at.map(|e| now <= e).unwrap_or(true);
            self.is_grade_entry_open || (after_start && before_end)
        } else {
            false
        }
    }

    pub fn can_enroll_students(&self) -> bool {
        if self.is_locked || matches!(self.state, AcademicPeriodState::Closed | AcademicPeriodState::Archived) {
            return false;
        }
        let now = Utc::now();
        let after_start = self.enrollment_starts_at.map(|s| now >= s).unwrap_or(true);
        let before_end = self.enrollment_ends_at.map(|e| now <= e).unwrap_or(true);
        self.is_enrollment_open || (after_start && before_end)
    }
}
