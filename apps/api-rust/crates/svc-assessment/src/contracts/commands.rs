//! Assessment domain commands — CQRS write operations.

use serde::{Deserialize, Serialize};

// ── Grade Commands ──────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct RecordGrade {
    pub enrollment_id: String,
    pub assessment_id: Option<String>,
    pub class_subject_id: Option<String>,
    pub subject_name: String,
    pub subject_name_ar: Option<String>,
    pub score: f64,
    pub max_score: f64,
    pub weight: f64,
    pub exam_type: String,
    pub semester: String,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BulkRecordGrades {
    pub period_id: String,
    pub class_id: String,
    pub grades: Vec<RecordGrade>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompileSemesterResults {
    pub period_id: String,
    pub class_id: String,
    pub policy_id: Option<String>,
    pub scale_id: Option<String>,
}

// ── Attendance Commands ─────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct RecordAttendance {
    pub enrollment_id: String,
    pub class_id: String,
    pub date: String,
    pub status: String,
    pub check_in_time: Option<String>,
    pub check_out_time: Option<String>,
    pub period: Option<String>,
    pub subject_name: Option<String>,
    pub notes: Option<String>,
    pub excuse_type: Option<String>,
    pub excuse_document: Option<String>,
    pub is_excused: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BulkRecordAttendance {
    pub class_id: String,
    pub date: String,
    pub records: Vec<RecordAttendance>,
}

// ── Behavior Commands ───────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct RecordBehavior {
    pub enrollment_id: String,
    pub behavior_type: String,
    pub severity: String,
    pub category: Option<String>,
    pub title: String,
    pub title_ar: Option<String>,
    pub description: String,
    pub incident_date: String,
    pub incident_time: Option<String>,
    pub location: Option<String>,
    pub points: Option<i32>,
    pub action_taken: Option<String>,
}

// ── Assessment Scheme Commands ──────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAssessmentScheme {
    pub name: String,
    pub description: Option<String>,
    pub components: Vec<CreateSchemeComponent>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSchemeComponent {
    pub name: String,
    pub component_type: String,
    pub weight: f64,
    pub max_score: f64,
    pub passing_score: Option<f64>,
}

// ── Calculation Policy Commands ─────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCalculationPolicy {
    pub target_period_id: Option<String>,
    pub name: String,
    pub rule_type: String,
    pub formula_expression: String,
    pub rounding_policy: Option<String>,
}

// ── Snapshot Commands ───────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateSnapshot {
    pub student_id: String,
    pub period_id: String,
    pub class_subject_id: String,
    pub policy_id: String,
    pub scale_id: String,
}
