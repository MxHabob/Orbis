use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

// ═══════════════════════════════════════════════════════════════
// Period Type — structural role in the academic hierarchy
// ═══════════════════════════════════════════════════════════════

/// The structural type of a period in the academic hierarchy.
///
/// This determines where the period sits in the Year → Term → Period tree:
/// - `Annual` — represents the full academic year (schools enroll students here)
/// - `Semester` / `Trimester` / `Quarter` — top-level terms under a Year
/// - `Month` — monthly sub-period within a term
/// - `Module` / `Block` — university/institute sub-periods
/// - `Custom` — institution-defined
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum PeriodType {
    /// Full academic year period — schools enroll students in this period
    Annual,
    /// Two terms per year (e.g. Term 1, Term 2)
    Semester,
    /// Three terms per year
    Trimester,
    /// Four terms per year
    Quarter,
    /// Monthly sub-period within a term
    Month,
    /// University course module
    Module,
    /// Intensive block scheduling
    Block,
    /// Institution-defined period type
    Custom,
}

impl std::fmt::Display for PeriodType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PeriodType::Annual => write!(f, "annual"),
            PeriodType::Semester => write!(f, "semester"),
            PeriodType::Trimester => write!(f, "trimester"),
            PeriodType::Quarter => write!(f, "quarter"),
            PeriodType::Month => write!(f, "month"),
            PeriodType::Module => write!(f, "module"),
            PeriodType::Block => write!(f, "block"),
            PeriodType::Custom => write!(f, "custom"),
        }
    }
}

impl PeriodType {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "annual" | "year" | "full_year" => Self::Annual,
            "semester" => Self::Semester,
            "trimester" => Self::Trimester,
            "quarter" => Self::Quarter,
            "month" | "monthly" => Self::Month,
            "module" => Self::Module,
            "block" => Self::Block,
            _ => Self::Custom,
        }
    }

    /// Whether this type represents the full academic year.
    pub fn is_annual(&self) -> bool {
        matches!(self, Self::Annual)
    }

    /// Whether this type represents a top-level term (direct child of Year).
    pub fn is_term(&self) -> bool {
        matches!(self, Self::Semester | Self::Trimester | Self::Quarter)
    }

    /// Whether this type represents a sub-period within a term.
    pub fn is_sub_period(&self) -> bool {
        matches!(self, Self::Month | Self::Module | Self::Block)
    }
}

impl Default for PeriodType {
    fn default() -> Self {
        PeriodType::Semester
    }
}

// ═══════════════════════════════════════════════════════════════
// Period Kind — semantic function / purpose of the period
// ═══════════════════════════════════════════════════════════════

/// The semantic function/purpose of a period.
///
/// This distinguishes what happens during the period:
/// - `Teaching` — regular instruction months
/// - `Examination` — term/final exam period (e.g. the 4th period in a school term)
/// - `Break` — vacation, holiday
/// - `Registration` — enrollment/registration window
/// - `Preparatory` — pre-academic preparation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum PeriodKind {
    /// Regular teaching/instruction period (monthly assessment)
    Teaching,
    /// Term examination / final exam period
    Examination,
    /// Break / vacation / holiday period
    Break,
    /// Registration / enrollment window
    Registration,
    /// Preparatory / orientation period
    Preparatory,
}

impl std::fmt::Display for PeriodKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PeriodKind::Teaching => write!(f, "teaching"),
            PeriodKind::Examination => write!(f, "examination"),
            PeriodKind::Break => write!(f, "break"),
            PeriodKind::Registration => write!(f, "registration"),
            PeriodKind::Preparatory => write!(f, "preparatory"),
        }
    }
}

impl PeriodKind {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "teaching" | "monthly" | "month" => Self::Teaching,
            "examination" | "exam" | "term_exam" | "final_exam" => Self::Examination,
            "break" | "vacation" | "holiday" => Self::Break,
            "registration" | "enrollment" => Self::Registration,
            "preparatory" | "preparation" | "orientation" => Self::Preparatory,
            _ => Self::Teaching,
        }
    }

    /// Whether grades are expected to be entered during this period kind.
    pub fn has_grade_entry(&self) -> bool {
        matches!(self, Self::Teaching | Self::Examination)
    }

    /// Whether attendance tracking applies during this period kind.
    pub fn has_attendance(&self) -> bool {
        matches!(self, Self::Teaching | Self::Examination | Self::Preparatory)
    }
}

impl Default for PeriodKind {
    fn default() -> Self {
        PeriodKind::Teaching
    }
}

// ═══════════════════════════════════════════════════════════════
// Period State — lifecycle state machine
// ═══════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
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

// ═══════════════════════════════════════════════════════════════
// Academic Period — the core entity
// ═══════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcademicPeriod {
    pub id: Uuid,
    pub org_id: Uuid,
    pub year_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub name: String,
    pub name_ar: Option<String>,
    pub code: Option<String>,
    /// Structural type (semester, month, module, etc.)
    pub period_type: PeriodType,
    /// Semantic function (teaching, examination, break, etc.)
    pub period_kind: PeriodKind,
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
        if !self.period_kind.has_grade_entry() {
            return false;
        }
        if matches!(
            self.state,
            AcademicPeriodState::GradeSubmission
                | AcademicPeriodState::ActiveTeaching
                | AcademicPeriodState::FinalExams
        ) {
            let now = Utc::now();
            let after_start = self.grade_entry_starts_at.map(|s| now >= s).unwrap_or(true);
            let before_end = self.grade_entry_ends_at.map(|e| now <= e).unwrap_or(true);
            self.is_grade_entry_open || (after_start && before_end)
        } else {
            false
        }
    }

    pub fn can_enroll_students(&self) -> bool {
        if self.is_locked
            || matches!(
                self.state,
                AcademicPeriodState::Closed | AcademicPeriodState::Archived
            )
        {
            return false;
        }
        let now = Utc::now();
        let after_start = self.enrollment_starts_at.map(|s| now >= s).unwrap_or(true);
        let before_end = self.enrollment_ends_at.map(|e| now <= e).unwrap_or(true);
        self.is_enrollment_open || (after_start && before_end)
    }

    /// Whether this period is a top-level term (direct child of the academic year).
    pub fn is_term(&self) -> bool {
        self.period_type.is_term() && self.parent_id.is_none()
    }

    /// Whether this period is a sub-period within a term.
    pub fn is_sub_period(&self) -> bool {
        self.parent_id.is_some() && self.period_type.is_sub_period()
    }

    /// Whether this is a term examination period (not a teaching month).
    pub fn is_exam_period(&self) -> bool {
        self.period_kind == PeriodKind::Examination
    }

    /// Whether this is a regular teaching/assessment month.
    pub fn is_teaching_period(&self) -> bool {
        self.period_kind == PeriodKind::Teaching
    }

    /// For backward compat: return period_type as a string.
    pub fn period_type_str(&self) -> String {
        self.period_type.to_string()
    }

    /// For backward compat: return period_kind as a string.
    pub fn period_kind_str(&self) -> String {
        self.period_kind.to_string()
    }
}
