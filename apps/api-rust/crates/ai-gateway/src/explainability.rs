//! Explainability — decision explanation framework for AI transparency.
//!
//! Provides human-readable (English + Arabic) explanations for:
//!   • Why an action was taken or suggested
//!   • What data was considered
//!   • What the expected impact is
//!   • What alternatives were considered

use serde::Serialize;

/// An explanation of an AI decision.
#[derive(Debug, Clone, Serialize)]
pub struct Explanation {
    /// What action was taken/suggested.
    pub action: String,
    /// Why this action was chosen.
    pub reasoning: String,
    /// Arabic reasoning.
    pub reasoning_ar: Option<String>,
    /// Data points that influenced the decision.
    pub data_points: Vec<DataPoint>,
    /// Confidence level (0.0 - 1.0).
    pub confidence: f64,
    /// Expected impact.
    pub expected_impact: Option<String>,
    /// Alternative actions that were considered.
    pub alternatives: Vec<Alternative>,
}

/// A data point that influenced a decision.
#[derive(Debug, Clone, Serialize)]
pub struct DataPoint {
    pub label: String,
    pub value: String,
    pub source: String,
    pub weight: f64,
}

/// An alternative action that was considered.
#[derive(Debug, Clone, Serialize)]
pub struct Alternative {
    pub action: String,
    pub reason_not_chosen: String,
}

pub struct ExplainabilityEngine;

impl ExplainabilityEngine {
    /// Generate an explanation for an at-risk student insight.
    pub fn explain_at_risk(
        student_name: &str,
        avg_score: f64,
        absence_rate: f64,
        grade_count: i32,
    ) -> Explanation {
        let mut data_points = vec![
            DataPoint {
                label: "Average Score".into(),
                value: format!("{:.1}%", avg_score),
                source: "grades".into(),
                weight: 0.6,
            },
        ];

        if absence_rate > 0.0 {
            data_points.push(DataPoint {
                label: "Absence Rate".into(),
                value: format!("{:.1}%", absence_rate),
                source: "attendance_daily".into(),
                weight: 0.3,
            });
        }

        data_points.push(DataPoint {
            label: "Grades Recorded".into(),
            value: grade_count.to_string(),
            source: "grades".into(),
            weight: 0.1,
        });

        Explanation {
            action: format!("Flag {} as at-risk student", student_name),
            reasoning: format!(
                "Student has an average score of {:.1}% which is below the passing threshold of 50%. \
                 {} grade records were analyzed.",
                avg_score, grade_count
            ),
            reasoning_ar: Some(format!(
                "الطالب لديه متوسط درجات {:.1}% وهو أقل من الحد الأدنى للنجاح 50%. \
                 تم تحليل {} سجل درجات.",
                avg_score, grade_count
            )),
            data_points,
            confidence: if grade_count >= 5 { 0.9 } else { 0.7 },
            expected_impact: Some("Early intervention can improve student outcomes".into()),
            alternatives: vec![
                Alternative {
                    action: "Monitor without flagging".into(),
                    reason_not_chosen: "Score is significantly below threshold".into(),
                },
                Alternative {
                    action: "Automatically assign tutor".into(),
                    reason_not_chosen: "Requires manual review for appropriate assignment".into(),
                },
            ],
        }
    }

    /// Generate an explanation for a data quality finding.
    pub fn explain_data_quality(
        rule_code: &str,
        entity_count: usize,
        description: &str,
    ) -> Explanation {
        Explanation {
            action: format!("Data quality alert: {}", rule_code),
            reasoning: format!(
                "Found {} entities matching rule '{}'. {}",
                entity_count, rule_code, description
            ),
            reasoning_ar: None,
            data_points: vec![
                DataPoint {
                    label: "Affected Entities".into(),
                    value: entity_count.to_string(),
                    source: "data_quality_findings".into(),
                    weight: 1.0,
                },
            ],
            confidence: 1.0,
            expected_impact: Some("Data completeness affects reporting accuracy".into()),
            alternatives: vec![],
        }
    }
}
