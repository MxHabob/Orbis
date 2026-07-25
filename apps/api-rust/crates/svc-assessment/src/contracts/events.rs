//! Assessment domain events — emitted after write operations.

use serde::{Deserialize, Serialize};

// ── Event Type Constants ────────────────────────────────────────

pub mod event_types {
    pub const GRADE_RECORDED: &str = "assessment.grade.recorded";
    pub const GRADES_BULK_RECORDED: &str = "assessment.grades.bulk_recorded";
    pub const GRADE_UPDATED: &str = "assessment.grade.updated";
    pub const SEMESTER_COMPILED: &str = "assessment.semester.compiled";

    pub const ATTENDANCE_RECORDED: &str = "assessment.attendance.recorded";
    pub const ATTENDANCE_BULK_RECORDED: &str = "assessment.attendance.bulk_recorded";

    pub const BEHAVIOR_RECORDED: &str = "assessment.behavior.recorded";
    pub const BEHAVIOR_REVIEWED: &str = "assessment.behavior.reviewed";
    pub const BEHAVIOR_RESOLVED: &str = "assessment.behavior.resolved";

    pub const SNAPSHOT_GENERATED: &str = "assessment.snapshot.generated";
    pub const SNAPSHOT_FAILED: &str = "assessment.snapshot.failed";

    pub const SCHEME_CREATED: &str = "assessment.scheme.created";
    pub const SCHEME_UPDATED: &str = "assessment.scheme.updated";

    pub const POLICY_CREATED: &str = "assessment.policy.created";

    pub const AT_RISK_DETECTED: &str = "assessment.at_risk.detected";
}

// ── Event Payloads ──────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GradeRecordedEvent {
    pub grade_id: String,
    pub enrollment_id: String,
    pub subject_name: String,
    pub score: f64,
    pub max_score: f64,
    pub is_final: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemesterCompiledEvent {
    pub period_id: String,
    pub class_id: String,
    pub students_compiled: usize,
    pub snapshots_generated: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttendanceRecordedEvent {
    pub enrollment_id: String,
    pub class_id: String,
    pub date: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorRecordedEvent {
    pub behavior_id: String,
    pub enrollment_id: String,
    pub behavior_type: String,
    pub severity: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotGeneratedEvent {
    pub snapshot_id: String,
    pub student_id: String,
    pub period_id: String,
    pub final_score: f64,
    pub grade_letter: Option<String>,
    pub is_passing: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtRiskDetectedEvent {
    pub student_id: String,
    pub enrollment_id: String,
    pub risk_factors: Vec<String>,
    pub risk_score: f64,
}
