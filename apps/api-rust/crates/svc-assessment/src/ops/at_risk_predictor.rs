//! At-Risk Student Predictor — first real EventAnalyzer.
//!
//! Listens to grade and attendance events, analyzes patterns,
//! and flags students who may be at academic risk.
//!
//! Trigger events:
//!   - `grade.submitted` (low score)
//!   - `attendance.recorded` (absence)
//!
//! Produces:
//!   - `student.at_risk` event (for notification pipeline)

use async_trait::async_trait;
use serde_json;
use tracing::info;

use shared_kernel::event_bus::EventEnvelope;
use shared_kernel::reactive::EventAnalyzer;

/// Minimum grade percentage that triggers at-risk flag.
const LOW_GRADE_THRESHOLD: f64 = 50.0;

/// At-Risk Student Predictor.
///
/// Analyzes grade and attendance events in real-time.
/// When a student's grade falls below threshold or absence count
/// exceeds limits, emits a `student.at_risk` event.
pub struct AtRiskPredictor;

impl AtRiskPredictor {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl EventAnalyzer for AtRiskPredictor {
    fn name(&self) -> &str {
        "at_risk_predictor"
    }

    fn handles_event(&self, event_type: &str) -> bool {
        matches!(event_type, "grade.submitted" | "attendance.recorded" | "grade.updated")
    }

    async fn analyze(&self, event: &EventEnvelope) -> Vec<EventEnvelope> {
        let mut derived_events = Vec::new();

        match event.event_type.as_str() {
            "grade.submitted" | "grade.updated" => {
                // Check if the grade is below threshold
                if let Some(score) = event.payload.get("score").and_then(|v| v.as_f64()) {
                    if let Some(max_score) = event.payload.get("max_score").and_then(|v| v.as_f64()) {
                        let percentage = if max_score > 0.0 { (score / max_score) * 100.0 } else { 0.0 };

                        if percentage < LOW_GRADE_THRESHOLD {
                            let student_id = event.payload.get("student_id")
                                .and_then(|v| v.as_str())
                                .unwrap_or("unknown");
                            let subject = event.payload.get("subject_name")
                                .and_then(|v| v.as_str())
                                .unwrap_or("unknown");

                            info!(
                                student_id = student_id,
                                subject = subject,
                                percentage = percentage,
                                "🚨 At-risk: low grade detected"
                            );

                            derived_events.push(event.child(
                                "student.at_risk",
                                "student",
                                student_id,
                                serde_json::json!({
                                    "risk_type": "low_grade",
                                    "student_id": student_id,
                                    "subject": subject,
                                    "score": score,
                                    "max_score": max_score,
                                    "percentage": percentage,
                                    "threshold": LOW_GRADE_THRESHOLD,
                                    "trigger_event": event.event_type,
                                    "recommendation": "Schedule a meeting with the student and notify guardian",
                                }),
                            ));
                        }
                    }
                }
            }

            "attendance.recorded" => {
                // Check for absence
                let status = event.payload.get("status")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");

                if status == "absent" || status == "absent_unexcused" {
                    let student_id = event.payload.get("student_id")
                        .and_then(|v| v.as_str())
                        .unwrap_or("unknown");
                    let consecutive = event.payload.get("consecutive_absences")
                        .and_then(|v| v.as_u64())
                        .unwrap_or(1);

                    // Flag at-risk after 3+ consecutive absences
                    if consecutive >= 3 {
                        info!(
                            student_id = student_id,
                            consecutive = consecutive,
                            "🚨 At-risk: excessive absences"
                        );

                        derived_events.push(event.child(
                            "student.at_risk",
                            "student",
                            student_id,
                            serde_json::json!({
                                "risk_type": "excessive_absence",
                                "student_id": student_id,
                                "consecutive_absences": consecutive,
                                "status": status,
                                "trigger_event": event.event_type,
                                "recommendation": "Contact guardian immediately — student has missed 3+ days",
                            }),
                        ));
                    }
                }
            }

            _ => {}
        }

        derived_events
    }
}
