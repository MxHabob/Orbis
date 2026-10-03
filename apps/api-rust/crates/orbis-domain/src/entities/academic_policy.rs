use serde::{Deserialize, Serialize};
use uuid::Uuid;
use rust_decimal::Decimal;
use chrono::{DateTime, Utc};

/// Rounding mode used by the academic calculation engine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RoundingMode {
    RoundHalfUp,
    RoundUp,
    RoundDown,
    Truncate,
}

impl Default for RoundingMode {
    fn default() -> Self {
        RoundingMode::RoundHalfUp
    }
}

/// Scope used for ranking students.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RankingScope {
    Class,
    GradeLevel,
    OrgUnit,
    Cohort,
}

impl Default for RankingScope {
    fn default() -> Self {
        RankingScope::Class
    }
}

/// Tie handling policy for ranking calculations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TiePolicy {
    /// Standard competition ranking: 1, 2, 2, 4
    StandardCompetition,
    /// Dense ranking: 1, 2, 2, 3
    Dense,
    /// Fractional / Average ranking: 1, 2.5, 2.5, 4
    Fractional,
}

impl Default for TiePolicy {
    fn default() -> Self {
        TiePolicy::StandardCompetition
    }
}

/// Academic outcome status for a student result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AcademicStatus {
    Passed,
    PassedWithHonors,
    ConditionalPass,
    Failed,
    Incomplete,
    Absent,
    Probation,
    Withdrawn,
}

impl std::fmt::Display for AcademicStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AcademicStatus::Passed => write!(f, "passed"),
            AcademicStatus::PassedWithHonors => write!(f, "passed_with_honors"),
            AcademicStatus::ConditionalPass => write!(f, "conditional_pass"),
            AcademicStatus::Failed => write!(f, "failed"),
            AcademicStatus::Incomplete => write!(f, "incomplete"),
            AcademicStatus::Absent => write!(f, "absent"),
            AcademicStatus::Probation => write!(f, "probation"),
            AcademicStatus::Withdrawn => write!(f, "withdrawn"),
        }
    }
}

/// Detailed breakdown of a monthly period assessment.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MonthlyAssessmentBreakdown {
    pub assignments: Decimal,
    pub assignments_max: Decimal,
    pub oral: Decimal,
    pub oral_max: Decimal,
    pub attendance: Decimal,
    pub attendance_max: Decimal,
    pub written_test: Decimal,
    pub written_test_max: Decimal,
    pub total: Decimal,
    pub total_max: Decimal,
}

impl Default for MonthlyAssessmentBreakdown {
    fn default() -> Self {
        Self {
            assignments: Decimal::ZERO,
            assignments_max: Decimal::from(20),
            oral: Decimal::ZERO,
            oral_max: Decimal::from(20),
            attendance: Decimal::ZERO,
            attendance_max: Decimal::from(20),
            written_test: Decimal::ZERO,
            written_test_max: Decimal::from(40),
            total: Decimal::ZERO,
            total_max: Decimal::from(100),
        }
    }
}

impl MonthlyAssessmentBreakdown {
    pub fn compute_total(&mut self) {
        self.total = self.assignments + self.oral + self.attendance + self.written_test;
    }
}

/// Result of evaluating a subject for a term.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubjectTermResult {
    pub subject_id: Uuid,
    pub subject_code: String,
    pub subject_name: String,
    pub subject_name_ar: Option<String>,
    pub credit_hours: Decimal,
    pub month_1: Option<MonthlyAssessmentBreakdown>,
    pub month_2: Option<MonthlyAssessmentBreakdown>,
    pub month_3: Option<MonthlyAssessmentBreakdown>,
    pub monthly_average: Decimal,
    pub monthly_contribution: Decimal,
    pub monthly_contribution_max: Decimal,
    pub term_final_exam: Option<Decimal>,
    pub term_final_exam_max: Decimal,
    pub term_total: Decimal,
    pub term_max: Decimal,
    pub percentage: Decimal,
    pub letter_grade: Option<String>,
    pub status: AcademicStatus,
}

/// Result of evaluating a subject for an entire academic year.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubjectAnnualResult {
    pub subject_id: Uuid,
    pub subject_code: String,
    pub subject_name: String,
    pub subject_name_ar: Option<String>,
    pub credit_hours: Decimal,
    pub term_1_result: Option<SubjectTermResult>,
    pub term_2_result: Option<SubjectTermResult>,
    pub annual_total: Decimal,
    pub annual_max: Decimal,
    pub percentage: Decimal,
    pub letter_grade: Option<String>,
    pub status: AcademicStatus,
}

/// Comprehensive student academic report card / result for a term.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudentTermReportCard {
    pub student_id: Uuid,
    pub student_name: String,
    pub student_number: Option<String>,
    pub class_id: Uuid,
    pub class_name: String,
    pub period_id: Uuid,
    pub period_name: String,
    pub subjects: Vec<SubjectTermResult>,
    pub total_score: Decimal,
    pub max_possible_score: Decimal,
    pub overall_percentage: Decimal,
    pub gpa: Option<Decimal>,
    pub rank_in_class: Option<u32>,
    pub rank_in_grade: Option<u32>,
    pub total_students_in_class: usize,
    pub total_students_in_grade: usize,
    pub status: AcademicStatus,
    pub calculated_at: DateTime<Utc>,
}

/// Comprehensive student academic report card / result for an academic year.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudentAnnualReportCard {
    pub student_id: Uuid,
    pub student_name: String,
    pub student_number: Option<String>,
    pub class_id: Uuid,
    pub class_name: String,
    pub academic_year_id: Uuid,
    pub academic_year_name: String,
    pub subjects: Vec<SubjectAnnualResult>,
    pub total_score: Decimal,
    pub max_possible_score: Decimal,
    pub overall_percentage: Decimal,
    pub cumulative_gpa: Option<Decimal>,
    pub rank_in_class: Option<u32>,
    pub rank_in_grade: Option<u32>,
    pub total_students_in_class: usize,
    pub total_students_in_grade: usize,
    pub status: AcademicStatus,
    pub calculated_at: DateTime<Utc>,
}

/// Organization-wide or class-level Academic Policy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcademicPolicy {
    pub org_id: Uuid,
    pub passing_threshold_percentage: Decimal,
    pub honors_threshold_percentage: Decimal,
    pub minimum_final_exam_score: Option<Decimal>,
    pub max_failed_subjects_for_conditional_pass: usize,
    pub rounding_mode: RoundingMode,
    pub decimal_places: u32,
    pub ranking_scope: RankingScope,
    pub tie_policy: TiePolicy,
}

impl Default for AcademicPolicy {
    fn default() -> Self {
        Self {
            org_id: Uuid::nil(),
            passing_threshold_percentage: Decimal::from(50),
            honors_threshold_percentage: Decimal::from(85),
            minimum_final_exam_score: Some(Decimal::from(10)),
            max_failed_subjects_for_conditional_pass: 2,
            rounding_mode: RoundingMode::RoundHalfUp,
            decimal_places: 2,
            ranking_scope: RankingScope::Class,
            tie_policy: TiePolicy::StandardCompetition,
        }
    }
}
