//! At-Risk Student Predictor — first real EventAnalyzer.
//!
//! Listens to grade and attendance events, analyzes patterns,
//! and flags students who may be at academic risk.
//!
//! Trigger events:
//!   - `grade.submitted` / `grade.updated` (low score)
//!   - `attendance.recorded` (absence)
//!
//! Produces:
//!   - DetectedInsight with type "at_risk_student"

use async_trait::async_trait;
use sea_orm::DatabaseConnection;
use tracing::info;

use shared_kernel::event_bus::EventEnvelope;
use shared_kernel::reactive::{DetectedInsight, EventAnalyzer};

/// Minimum grade percentage that triggers at-risk flag.
const LOW_GRADE_THRESHOLD: f64 = 50.0;

/// At-Risk Student Predictor.
///
/// Analyzes grade and attendance events in real-time.
/// When a student's grade falls below threshold or absence count
/// exceeds limits, produces a DetectedInsight.
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

    fn interested_in(&self) -> &[&str] {
        &["grade.submitted", "grade.updated", "attendance.recorded"]
    }

    async fn analyze(
        &self,
        _db: &DatabaseConnection,
        _org_id: &str,
        event: &EventEnvelope,
    ) -> Vec<DetectedInsight> {
        let mut insights = Vec::new();

        match event.event_type.as_str() {
            "grade.submitted" | "grade.updated" => {
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
                                "At-risk: low grade detected"
                            );

                            insights.push(DetectedInsight {
                                insight_type: "at_risk_student".to_string(),
                                entity_type: Some("student".to_string()),
                                entity_id: Some(student_id.to_string()),
                                title: format!("Low grade in {}", subject),
                                title_ar: Some(format!("درجة منخفضة في {}", subject)),
                                summary: format!(
                                    "Student scored {:.0}% (below {:.0}% threshold) in {}",
                                    percentage, LOW_GRADE_THRESHOLD, subject
                                ),
                                summary_ar: Some(format!(
                                    "حصل الطالب على {:.0}% (أقل من حد {:.0}%) في {}",
                                    percentage, LOW_GRADE_THRESHOLD, subject
                                )),
                                severity: "warning".to_string(),
                                confidence: 0.9,
                                impact_score: Some(1.0 - (percentage / 100.0)),
                                suggested_action: Some("schedule_meeting".to_string()),
                                suggested_params: Some(serde_json::json!({
                                    "student_id": student_id,
                                    "subject": subject,
                                    "notify_guardian": true,
                                })),
                            });
                        }
                    }
                }
            }

            "attendance.recorded" => {
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

                    if consecutive >= 3 {
                        info!(
                            student_id = student_id,
                            consecutive = consecutive,
                            "At-risk: excessive absences"
                        );

                        insights.push(DetectedInsight {
                            insight_type: "at_risk_student".to_string(),
                            entity_type: Some("student".to_string()),
                            entity_id: Some(student_id.to_string()),
                            title: format!("{} consecutive absences", consecutive),
                            title_ar: Some(format!("{} أيام غياب متتالية", consecutive)),
                            summary: format!(
                                "Student has {} consecutive unexcused absences — immediate guardian contact recommended",
                                consecutive
                            ),
                            summary_ar: Some(format!(
                                "الطالب لديه {} أيام غياب متتالية — يُنصح بالتواصل مع ولي الأمر فوراً",
                                consecutive
                            )),
                            severity: if consecutive >= 5 { "critical" } else { "warning" }.to_string(),
                            confidence: 0.95,
                            impact_score: Some((consecutive as f64 / 10.0).min(1.0)),
                            suggested_action: Some("contact_guardian".to_string()),
                            suggested_params: Some(serde_json::json!({
                                "student_id": student_id,
                                "consecutive_absences": consecutive,
                                "urgency": if consecutive >= 5 { "high" } else { "medium" },
                            })),
                        });
                    }
                }
            }

            _ => {}
        }

        insights
    }
}
