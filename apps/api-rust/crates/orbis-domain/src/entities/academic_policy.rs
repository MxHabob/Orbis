use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

/// Rounding mode used by the academic calculation engine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
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

/// A single component score within a period assessment.
/// Driven by `AssessmentScheme.components` — not hardcoded.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
pub struct ComponentScore {
    /// Component code (e.g. "HW", "ORAL", "ATT", "EXAM_M")
    pub code: String,
    /// Human-readable name
    pub name: String,
    /// Arabic name
    pub name_ar: Option<String>,
    /// Actual score achieved
    pub score: Decimal,
    /// Maximum possible score for this component
    pub max_score: Decimal,
}

/// Detailed breakdown of a period assessment using generic component scores.
/// Replaces the old hardcoded 4-field `MonthlyAssessmentBreakdown`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
pub struct PeriodComponentScores {
    /// Individual component scores driven by the active AssessmentScheme
    pub components: Vec<ComponentScore>,
    /// Computed total (sum of component scores, each capped at its max)
    pub total: Decimal,
    /// Computed maximum (sum of component max_scores)
    pub total_max: Decimal,
}

impl PeriodComponentScores {
    /// Create a new empty breakdown from component definitions.
    pub fn empty_from_components(components: &[(String, String, Option<String>, Decimal)]) -> Self {
        let mut total_max = Decimal::ZERO;
        let scores = components
            .iter()
            .map(|(code, name, name_ar, max_score)| {
                total_max += *max_score;
                ComponentScore {
                    code: code.clone(),
                    name: name.clone(),
                    name_ar: name_ar.clone(),
                    score: Decimal::ZERO,
                    max_score: *max_score,
                }
            })
            .collect();
        Self {
            components: scores,
            total: Decimal::ZERO,
            total_max,
        }
    }

    /// Compute total from individual component scores (each capped at max).
    pub fn compute_total(&mut self) {
        self.total = self
            .components
            .iter()
            .map(|c| c.score.min(c.max_score))
            .sum();
    }

    /// Get a component score by code, if it exists.
    pub fn get_component(&self, code: &str) -> Option<&ComponentScore> {
        self.components.iter().find(|c| c.code == code)
    }
}

/// Backward-compatible type alias. Callers that previously used
/// `MonthlyAssessmentBreakdown` can migrate incrementally.
pub type MonthlyAssessmentBreakdown = PeriodComponentScores;

/// A single monthly (or sub-period) assessment result with period metadata.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PeriodAssessmentResult {
    /// The academic period ID this result belongs to
    pub period_id: Uuid,
    /// Period code (e.g. "M1", "M2", "M3")
    pub period_code: String,
    /// Period name
    pub period_name: String,
    /// Arabic period name
    pub period_name_ar: Option<String>,
    /// Component breakdown for this period
    pub breakdown: PeriodComponentScores,
}

/// Result of evaluating a subject for a term.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SubjectTermResult {
    pub subject_id: Uuid,
    pub subject_code: String,
    pub subject_name: String,
    pub subject_name_ar: Option<String>,
    pub credit_hours: Decimal,
    /// Dynamic list of monthly/sub-period results (replaces fixed month_1/month_2/month_3)
    pub monthly_results: Vec<PeriodAssessmentResult>,
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
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
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
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
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
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
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
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
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
    /// Whether ranking is enabled for reports and certificates.
    #[serde(default = "default_true")]
    pub ranking_enabled: bool,
    /// Maximum rank to display on reports/certificates (None = show all).
    #[serde(default)]
    pub max_displayed_rank: Option<u32>,
}

fn default_true() -> bool {
    true
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
            ranking_enabled: true,
            max_displayed_rank: None,
        }
    }
}
