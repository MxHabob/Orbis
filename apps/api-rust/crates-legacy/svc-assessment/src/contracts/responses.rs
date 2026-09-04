//! Assessment domain response types — returned from queries.

use serde::{Deserialize, Serialize};

// ── Grade Responses ─────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct GradeResponse {
    pub id: String,
    pub enrollment_id: String,
    pub subject_name: String,
    pub subject_name_ar: Option<String>,
    pub score: f64,
    pub max_score: f64,
    pub weight: f64,
    pub grade_letter: Option<String>,
    pub grade_points: Option<f64>,
    pub exam_type: String,
    pub semester: String,
    pub is_final: bool,
    pub graded_by: String,
    pub graded_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GradeMatrixResponse {
    pub class_id: String,
    pub period_id: String,
    pub subjects: Vec<String>,
    pub students: Vec<StudentGradeRow>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StudentGradeRow {
    pub enrollment_id: String,
    pub student_name: String,
    pub student_number: Option<String>,
    pub grades: Vec<Option<f64>>,
    pub total: Option<f64>,
    pub average: Option<f64>,
    pub rank: Option<i32>,
}

// ── Report Card Response ────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct ReportCardResponse {
    pub enrollment_id: String,
    pub period_id: String,
    pub student_name: String,
    pub class_name: String,
    pub subjects: Vec<SubjectGradeSummary>,
    pub total_score: f64,
    pub percentage: f64,
    pub gpa: Option<f64>,
    pub overall_grade: Option<String>,
    pub rank: Option<i32>,
    pub total_students: Option<i32>,
    pub attendance_summary: Option<AttendanceSummaryResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubjectGradeSummary {
    pub subject_name: String,
    pub subject_name_ar: Option<String>,
    pub exam_scores: Vec<ExamScore>,
    pub final_score: f64,
    pub max_score: f64,
    pub grade_letter: Option<String>,
    pub is_passing: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExamScore {
    pub exam_type: String,
    pub score: f64,
    pub max_score: f64,
    pub weight: f64,
}

// ── Attendance Responses ────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct AttendanceSummaryResponse {
    pub total_days: i32,
    pub present_days: i32,
    pub absent_days: i32,
    pub excused_days: i32,
    pub late_days: i32,
    pub attendance_rate: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AttendanceReportResponse {
    pub class_id: String,
    pub students: Vec<StudentAttendanceSummary>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StudentAttendanceSummary {
    pub enrollment_id: String,
    pub student_name: String,
    pub summary: AttendanceSummaryResponse,
}

// ── Behavior Responses ──────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct BehaviorSummaryResponse {
    pub enrollment_id: String,
    pub total_records: i32,
    pub positive_count: i32,
    pub negative_count: i32,
    pub neutral_count: i32,
    pub total_points: i32,
    pub pending_actions: i32,
}

// ── Snapshot Responses ──────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct SnapshotResponse {
    pub id: String,
    pub student_id: String,
    pub period_id: String,
    pub class_subject_id: String,
    pub final_score: f64,
    pub grade_letter: Option<String>,
    pub gpa_points: Option<f64>,
    pub is_passing: bool,
    pub rank: Option<i32>,
    pub status: String,
}
