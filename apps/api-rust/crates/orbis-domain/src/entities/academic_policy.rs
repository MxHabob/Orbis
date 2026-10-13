use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
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
    pub section_id: Uuid,
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
    pub section_id: Uuid,
    pub class_name: String,
    pub calendar_id: Uuid,
    pub academic_calendar_name: String,
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

/// Result lifecycle states for a calculation snapshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum ResultLifecycleState {
    Draft,
    Calculated,
    Submitted,
    Reviewed,
    Approved,
    Published,
    Revised,
    Archived,
}

/// Reference to a source record used when calculating and storing a result.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct ResultInputReference {
    pub source_kind: String,
    pub source_id: Uuid,
    pub source_revision: Option<String>,
    pub policy_version: Option<u32>,
}

/// Approval transition record used for auditability and review history.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct ResultApprovalTransition {
    pub actor_id: Uuid,
    pub from_state: ResultLifecycleState,
    pub to_state: ResultLifecycleState,
    pub reason: Option<String>,
    pub occurred_at: DateTime<Utc>,
}

/// Audit log entry for every result lifecycle event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct ResultAuditEntry {
    pub id: Uuid,
    pub snapshot_id: Uuid,
    pub actor_id: Uuid,
    pub action: String,
    pub from_state: ResultLifecycleState,
    pub to_state: ResultLifecycleState,
    pub reason: Option<String>,
    pub occurred_at: DateTime<Utc>,
}

/// Immutable academic result snapshot tied to a specific policy version.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ResultSnapshot {
    pub id: Uuid,
    pub org_id: Uuid,
    pub learner_id: Uuid,
    pub subject_id: Option<Uuid>,
    pub period_id: Option<Uuid>,
    pub calendar_id: Option<Uuid>,
    pub result_type: String,
    pub result_key: String,
    pub policy_version: u32,
    pub revision: u32,
    pub parent_snapshot_id: Option<Uuid>,
    pub supersedes_snapshot_id: Option<Uuid>,
    pub state: ResultLifecycleState,
    pub produced_at: DateTime<Utc>,
    pub approved_by: Option<Uuid>,
    pub approved_at: Option<DateTime<Utc>>,
    pub created_by: Option<Uuid>,
    pub input_references: Vec<ResultInputReference>,
    pub payload: JsonValue,
}

impl ResultSnapshot {
    pub fn new(
        org_id: Uuid,
        learner_id: Uuid,
        result_type: impl Into<String>,
        result_key: impl Into<String>,
        policy_version: u32,
        produced_at: DateTime<Utc>,
        payload: JsonValue,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            org_id,
            learner_id,
            subject_id: None,
            period_id: None,
            calendar_id: None,
            result_type: result_type.into(),
            result_key: result_key.into(),
            policy_version,
            revision: 1,
            parent_snapshot_id: None,
            supersedes_snapshot_id: None,
            state: ResultLifecycleState::Calculated,
            produced_at,
            approved_by: None,
            approved_at: None,
            created_by: None,
            input_references: Vec::new(),
            payload,
        }
    }

    pub fn with_context(
        mut self,
        subject_id: Option<Uuid>,
        period_id: Option<Uuid>,
        calendar_id: Option<Uuid>,
        created_by: Option<Uuid>,
        input_references: Vec<ResultInputReference>,
    ) -> Self {
        self.subject_id = subject_id;
        self.period_id = period_id;
        self.calendar_id = calendar_id;
        self.created_by = created_by;
        self.input_references = input_references;
        self
    }

    pub fn is_approved(&self) -> bool {
        matches!(self.state, ResultLifecycleState::Approved)
    }

    pub fn is_published(&self) -> bool {
        matches!(self.state, ResultLifecycleState::Published)
    }

    pub fn is_finalized(&self) -> bool {
        matches!(
            self.state,
            ResultLifecycleState::Approved | ResultLifecycleState::Published
        )
    }

    pub fn ensure_mutation_allowed(&self) -> Result<(), String> {
        if self.is_approved()
            || self.is_published()
            || matches!(self.state, ResultLifecycleState::Archived)
        {
            return Err(
                "Approved, published, or archived result snapshots cannot be directly modified; create a revision with a correction reason.".into(),
            );
        }
        Ok(())
    }

    pub fn transition_to(
        &mut self,
        actor_id: Uuid,
        to_state: ResultLifecycleState,
        occurred_at: DateTime<Utc>,
        reason: Option<&str>,
    ) -> Result<ResultApprovalTransition, String> {
        let from_state = self.state;
        let valid_transition = match (from_state, to_state) {
            (ResultLifecycleState::Draft, ResultLifecycleState::Calculated) => true,
            (ResultLifecycleState::Calculated, ResultLifecycleState::Submitted) => true,
            (ResultLifecycleState::Submitted, ResultLifecycleState::Reviewed) => true,
            (ResultLifecycleState::Reviewed, ResultLifecycleState::Approved) => true,
            (ResultLifecycleState::Approved, ResultLifecycleState::Published) => true,
            (ResultLifecycleState::Submitted, ResultLifecycleState::Archived) => true,
            (ResultLifecycleState::Approved, ResultLifecycleState::Revised) => true,
            (ResultLifecycleState::Approved, ResultLifecycleState::Archived) => true,
            (ResultLifecycleState::Published, ResultLifecycleState::Revised) => true,
            (ResultLifecycleState::Published, ResultLifecycleState::Archived) => true,
            _ => false,
        };

        if !valid_transition {
            return Err(format!(
                "Invalid lifecycle transition from {:?} to {:?}.",
                from_state, to_state
            ));
        }

        if matches!(to_state, ResultLifecycleState::Approved | ResultLifecycleState::Published) && reason.is_none() {
            return Err("Approval and publication require a recorded reason.".into());
        }

        if (from_state == ResultLifecycleState::Approved || from_state == ResultLifecycleState::Published)
            && !matches!(to_state, ResultLifecycleState::Published | ResultLifecycleState::Archived | ResultLifecycleState::Revised)
        {
            return Err("Finalized results must move through a recorded publication, revision, or archival action.".into());
        }

        self.state = to_state;
        if matches!(to_state, ResultLifecycleState::Approved) {
            self.approved_by = Some(actor_id);
            self.approved_at = Some(occurred_at);
        }

        Ok(ResultApprovalTransition {
            actor_id,
            from_state,
            to_state,
            reason: reason.map(str::to_owned),
            occurred_at,
        })
    }

    pub fn approve(
        &mut self,
        approved_by: Uuid,
        approved_at: DateTime<Utc>,
        reason: Option<&str>,
    ) -> Result<ResultApprovalTransition, String> {
        self.transition_to(
            approved_by,
            ResultLifecycleState::Approved,
            approved_at,
            reason,
        )
    }

    pub fn publish(
        &mut self,
        published_by: Uuid,
        published_at: DateTime<Utc>,
        reason: Option<&str>,
    ) -> Result<ResultApprovalTransition, String> {
        self.transition_to(
            published_by,
            ResultLifecycleState::Published,
            published_at,
            reason,
        )
    }

    pub fn audit_entry(
        &self,
        actor_id: Uuid,
        action: &str,
        from_state: ResultLifecycleState,
        to_state: ResultLifecycleState,
        reason: Option<&str>,
        occurred_at: DateTime<Utc>,
    ) -> ResultAuditEntry {
        ResultAuditEntry {
            id: Uuid::new_v4(),
            snapshot_id: self.id,
            actor_id,
            action: action.to_string(),
            from_state,
            to_state,
            reason: reason.map(str::to_owned),
            occurred_at,
        }
    }

    pub fn new_revision(
        &self,
        next_policy_version: u32,
        next_payload: JsonValue,
        correction_reason: Option<&str>,
    ) -> Result<Self, String> {
        if (self.state == ResultLifecycleState::Approved || self.state == ResultLifecycleState::Published)
            && correction_reason.is_none()
        {
            return Err("Approved or published results require a correction reason for a new revision.".into());
        }

        Ok(Self {
            id: Uuid::new_v4(),
            org_id: self.org_id,
            learner_id: self.learner_id,
            subject_id: self.subject_id,
            period_id: self.period_id,
            calendar_id: self.calendar_id,
            result_type: self.result_type.clone(),
            result_key: self.result_key.clone(),
            policy_version: next_policy_version,
            revision: self.revision + 1,
            parent_snapshot_id: Some(self.id),
            supersedes_snapshot_id: Some(self.id),
            state: ResultLifecycleState::Revised,
            produced_at: Utc::now(),
            approved_by: None,
            approved_at: None,
            created_by: self.created_by,
            input_references: self.input_references.clone(),
            payload: next_payload,
        })
    }
}

/// Semantic contract for downstream report generation and DocForge.
/// This intentionally exposes a stable, institution-neutral payload rather than any
/// database table shape or legacy school-only report model.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AcademicResultView {
    pub snapshot_id: Uuid,
    pub org_id: Uuid,
    pub learner_id: Uuid,
    pub subject_id: Option<Uuid>,
    pub period_id: Option<Uuid>,
    pub calendar_id: Option<Uuid>,
    pub result_type: String,
    pub result_key: String,
    pub policy_version: u32,
    pub revision: u32,
    pub state: ResultLifecycleState,
    pub produced_at: DateTime<Utc>,
    pub approved_by: Option<Uuid>,
    pub approved_at: Option<DateTime<Utc>>,
    pub payload: JsonValue,
}

/// Typed projection contract used by report templates and DocForge outputs.
/// It ensures a report is bound to a specific semantic result shape without coupling
/// to a single certificate or database schema.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ReportProjectionSpec {
    pub template_key: String,
    pub result_type: String,
    pub required_fields: Vec<String>,
    pub optional_fields: Vec<String>,
    pub accepted_statuses: Vec<String>,
    pub requires_approval: bool,
    pub policy_versions: Vec<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ProjectionValidationResult {
    pub is_valid: bool,
    pub missing_fields: Vec<String>,
    pub unknown_fields: Vec<String>,
    pub warnings: Vec<String>,
}

impl AcademicResultView {
    /// Validates an institution-neutral result payload against a DocForge projection.
    /// Unknown keys are flagged so reports can fail fast when templates rely on fields
    /// the result contract does not expose.
    pub fn validate_projection(&self, projection: &ReportProjectionSpec) -> ProjectionValidationResult {
        let mut missing_fields = Vec::new();
        let mut unknown_fields = Vec::new();
        let mut warnings = Vec::new();

        let payload_map = match &self.payload {
            JsonValue::Object(map) => map,
            _ => {
                warnings.push("Result payload must be a JSON object to validate projection fields.".to_string());
                return ProjectionValidationResult {
                    is_valid: false,
                    missing_fields: projection.required_fields.clone(),
                    unknown_fields: vec![],
                    warnings,
                };
            }
        };

        let known_fields: Vec<String> = projection
            .required_fields
            .iter()
            .chain(projection.optional_fields.iter())
            .cloned()
            .collect();

        for field in &projection.required_fields {
            if !payload_map.contains_key(field) {
                missing_fields.push(field.clone());
            }
        }

        for field in payload_map.keys() {
            if !known_fields.iter().any(|known| known == field) {
                unknown_fields.push(field.clone());
            }
        }

        if !projection.accepted_statuses.is_empty()
            && !projection
                .accepted_statuses
                .iter()
                .any(|status| status == &format!("{:?}", self.state).to_ascii_lowercase())
        {
            warnings.push(format!(
                "Result state {:?} is not accepted by this projection.",
                self.state
            ));
        }

        if projection.requires_approval && !self.state.eq(&ResultLifecycleState::Approved) {
            warnings.push("Projection requires an approved result snapshot.".to_string());
        }

        if !projection.policy_versions.is_empty()
            && !projection
                .policy_versions
                .iter()
                .any(|version| *version == self.policy_version)
        {
            warnings.push(format!(
                "Policy version {} is not included in the projection policy set.",
                self.policy_version
            ));
        }

        let is_valid = missing_fields.is_empty() && unknown_fields.is_empty();
        ProjectionValidationResult {
            is_valid,
            missing_fields,
            unknown_fields,
            warnings,
        }
    }
}

impl From<ResultSnapshot> for AcademicResultView {
    fn from(snapshot: ResultSnapshot) -> Self {
        Self {
            snapshot_id: snapshot.id,
            org_id: snapshot.org_id,
            learner_id: snapshot.learner_id,
            subject_id: snapshot.subject_id,
            period_id: snapshot.period_id,
            calendar_id: snapshot.calendar_id,
            result_type: snapshot.result_type,
            result_key: snapshot.result_key,
            policy_version: snapshot.policy_version,
            revision: snapshot.revision,
            state: snapshot.state,
            produced_at: snapshot.produced_at,
            approved_by: snapshot.approved_by,
            approved_at: snapshot.approved_at,
            payload: snapshot.payload,
        }
    }
}

/// Evaluated letter and qualitative grade detail.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct GradeEvaluationDetail {
    pub letter_grade: String,
    pub letter_grade_ar: String,
    pub grade_points: Decimal,
    pub is_passed: bool,
    pub is_honors: bool,
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
    /// Configurable maximum coursework/continuous assessment score for a term (default 20 in Yemen schools, 40 in universities).
    #[serde(default = "default_twenty")]
    pub coursework_max_score: Decimal,
    /// Configurable maximum term exam score (default 30 in Yemen schools, 60 in universities).
    #[serde(default = "default_thirty")]
    pub exam_max_score: Decimal,
    /// Configurable maximum term total score (default 50 in Yemen schools, 100 in universities).
    #[serde(default = "default_fifty")]
    pub term_max_score: Decimal,
    /// Configurable maximum annual total score (default 100).
    #[serde(default = "default_annual_hundred")]
    pub annual_max_score: Decimal,
}

fn default_true() -> bool {
    true
}

fn default_twenty() -> Decimal {
    Decimal::from(20)
}

fn default_thirty() -> Decimal {
    Decimal::from(30)
}

fn default_fifty() -> Decimal {
    Decimal::from(50)
}

fn default_annual_hundred() -> Decimal {
    Decimal::from(100)
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
            coursework_max_score: Decimal::from(20),
            exam_max_score: Decimal::from(30),
            term_max_score: Decimal::from(50),
            annual_max_score: Decimal::from(100),
        }
    }
}

impl AcademicPolicy {
    /// University default policy: 40 coursework + 60 exam = 100, cohort ranking.
    pub fn university_default(org_id: Uuid) -> Self {
        Self {
            org_id,
            passing_threshold_percentage: Decimal::from(50),
            honors_threshold_percentage: Decimal::from(85),
            minimum_final_exam_score: Some(Decimal::from(20)),
            max_failed_subjects_for_conditional_pass: 0,
            rounding_mode: RoundingMode::RoundHalfUp,
            decimal_places: 2,
            ranking_scope: RankingScope::Cohort,
            tie_policy: TiePolicy::StandardCompetition,
            ranking_enabled: true,
            max_displayed_rank: None,
            coursework_max_score: Decimal::from(40),
            exam_max_score: Decimal::from(60),
            term_max_score: Decimal::from(100),
            annual_max_score: Decimal::from(100),
        }
    }

    /// Institute / vocational default policy: 40 theory + 60 practical = 100.
    pub fn institute_default(org_id: Uuid) -> Self {
        Self {
            org_id,
            passing_threshold_percentage: Decimal::from(60),
            honors_threshold_percentage: Decimal::from(85),
            minimum_final_exam_score: Some(Decimal::from(35)),
            max_failed_subjects_for_conditional_pass: 1,
            rounding_mode: RoundingMode::RoundHalfUp,
            decimal_places: 2,
            ranking_scope: RankingScope::Class,
            tie_policy: TiePolicy::StandardCompetition,
            ranking_enabled: true,
            max_displayed_rank: None,
            coursework_max_score: Decimal::from(40),
            exam_max_score: Decimal::from(60),
            term_max_score: Decimal::from(100),
            annual_max_score: Decimal::from(100),
        }
    }

    /// Evaluates a percentage mark and returns full evaluation details (letter, Arabic appreciation, grade points, pass/honors).
    pub fn evaluate_percentage(&self, percentage: Decimal) -> GradeEvaluationDetail {
        let is_honors = percentage >= self.honors_threshold_percentage;
        let is_passed = percentage >= self.passing_threshold_percentage;

        let (letter, letter_ar, points) = if percentage >= Decimal::from(90) {
            ("A", "ممتاز", Decimal::from(4))
        } else if percentage >= Decimal::from(80) {
            ("B", "جيد جداً", Decimal::from(3))
        } else if percentage >= Decimal::from(70) {
            ("C", "جيد", Decimal::from(2))
        } else if percentage >= Decimal::from(65) {
            ("D+", "مقبول مرتفع", Decimal::from_str_exact("1.5").unwrap_or(Decimal::ONE))
        } else if percentage >= Decimal::from(50) {
            ("D", "مقبول", Decimal::ONE)
        } else {
            ("F", "راسب", Decimal::ZERO)
        };

        GradeEvaluationDetail {
            letter_grade: letter.to_string(),
            letter_grade_ar: letter_ar.to_string(),
            grade_points: points,
            is_passed,
            is_honors,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn result_snapshot_captures_policy_version_and_input_references() {
        let org = Uuid::new_v4();
        let learner = Uuid::new_v4();
        let subject = Uuid::new_v4();
        let period = Uuid::new_v4();
        let now = Utc::now();
        let snapshot = ResultSnapshot::new(
            org,
            learner,
            "term_report_card",
            "term-2025-q1",
            7,
            now,
            serde_json::json!({"overall_percentage": 88.5, "status": "passed"}),
        )
        .with_context(
            Some(subject),
            Some(period),
            None,
            Some(Uuid::new_v4()),
            vec![ResultInputReference {
                source_kind: "assessment_record".into(),
                source_id: Uuid::new_v4(),
                source_revision: Some("v2".into()),
                policy_version: Some(7),
            }],
        );

        assert_eq!(snapshot.policy_version, 7);
        assert_eq!(snapshot.revision, 1);
        assert_eq!(snapshot.input_references.len(), 1);
        assert_eq!(snapshot.payload["overall_percentage"], 88.5);
    }

    #[test]
    fn result_snapshot_revision_keeps_original_record_immutable() {
        let org = Uuid::new_v4();
        let learner = Uuid::new_v4();
        let original = ResultSnapshot::new(
            org,
            learner,
            "annual_report_card",
            "annual-2025",
            3,
            Utc::now(),
            serde_json::json!({"overall_percentage": 82.0, "status": "passed"}),
        );

        let revised = original
            .new_revision(
                4,
                serde_json::json!({"overall_percentage": 84.5, "status": "passed"}),
                Some("Corrected assessment input"),
            )
            .unwrap();

        assert_eq!(original.policy_version, 3);
        assert_eq!(revised.policy_version, 4);
        assert_eq!(revised.parent_snapshot_id, Some(original.id));
        assert_eq!(revised.revision, original.revision + 1);
        assert_eq!(original.payload["overall_percentage"], 82.0);
        assert_eq!(revised.payload["overall_percentage"], 84.5);
    }

    #[test]
    fn result_snapshot_requires_reason_for_approved_revision_and_blocks_mutation() {
        let org = Uuid::new_v4();
        let learner = Uuid::new_v4();
        let mut snapshot = ResultSnapshot::new(
            org,
            learner,
            "term_report_card",
            "term-2025-a",
            1,
            Utc::now(),
            serde_json::json!({"overall_percentage": 80.0, "status": "passed"}),
        );

        let actor = Uuid::new_v4();
        let approval = snapshot
            .transition_to(
                actor,
                ResultLifecycleState::Submitted,
                Utc::now(),
                Some("Ready for review"),
            )
            .unwrap();
        assert_eq!(approval.to_state, ResultLifecycleState::Submitted);

        let result = snapshot
            .transition_to(
                actor,
                ResultLifecycleState::Reviewed,
                Utc::now(),
                Some("Reviewed by registrar"),
            )
            .unwrap();
        assert_eq!(result.to_state, ResultLifecycleState::Reviewed);

        let approved = snapshot
            .transition_to(
                actor,
                ResultLifecycleState::Approved,
                Utc::now(),
                Some("Approved by registrar"),
            )
            .unwrap();
        assert_eq!(approved.to_state, ResultLifecycleState::Approved);
        assert!(snapshot.is_approved());

        assert!(snapshot.ensure_mutation_allowed().is_err());
        assert!(snapshot
            .new_revision(
                2,
                serde_json::json!({"overall_percentage": 82.0, "status": "passed"}),
                None,
            )
            .is_err());
    }

    #[test]
    fn academic_result_view_exposes_semantic_contract_for_docforge() {
        let snapshot = ResultSnapshot::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            "annual_report_card",
            "annual-2025",
            7,
            Utc::now(),
            serde_json::json!({"overall_percentage": 92.5, "status": "passed"}),
        );

        let view: AcademicResultView = snapshot.clone().into();
        assert_eq!(view.snapshot_id, snapshot.id);
        assert_eq!(view.policy_version, 7);
        assert_eq!(view.payload["overall_percentage"], 92.5);
        assert_eq!(view.state, ResultLifecycleState::Calculated);
    }

    #[test]
    fn projection_validation_surfaces_unknown_fields_and_non_gpa_certification_payloads() {
        let snapshot = ResultSnapshot::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            "competency_report",
            "competency-2025",
            9,
            Utc::now(),
            serde_json::json!({
                "overall_percentage": 88.0,
                "status": "passed",
                "competency_summary": ["digital_literacy", "teamwork"],
                "gpa": null
            }),
        );

        let view: AcademicResultView = snapshot.into();
        let projection = ReportProjectionSpec {
            template_key: "competency_certificate".to_string(),
            result_type: "competency_report".to_string(),
            required_fields: vec!["overall_percentage".to_string(), "status".to_string()],
            optional_fields: vec!["competency_summary".to_string(), "gpa".to_string()],
            accepted_statuses: vec!["passed".to_string(), "conditional_pass".to_string()],
            requires_approval: false,
            policy_versions: vec![9, 10],
        };

        let validation = view.validate_projection(&projection);
        assert!(validation.is_valid);
        assert!(validation.missing_fields.is_empty());
        assert!(validation.unknown_fields.is_empty());

        let snapshot_with_legacy_field = ResultSnapshot::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            "annual_report_card",
            "annual-2025",
            9,
            Utc::now(),
            serde_json::json!({
                "overall_percentage": 91.5,
                "status": "passed",
                "legacy_gpa": 3.8,
                "certificate_type": "school_certificate"
            }),
        );

        let view_with_legacy_fields: AcademicResultView = snapshot_with_legacy_field.into();
        let validation_for_legacy_fields = view_with_legacy_fields.validate_projection(&projection);
        assert!(!validation_for_legacy_fields.is_valid);
        assert_eq!(validation_for_legacy_fields.unknown_fields.len(), 2);
        assert!(validation_for_legacy_fields.unknown_fields.contains(&"legacy_gpa".to_string()));
        assert!(validation_for_legacy_fields.unknown_fields.contains(&"certificate_type".to_string()));
    }
}
