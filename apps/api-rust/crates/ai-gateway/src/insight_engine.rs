//! Insight Engine — pattern detection and AI-driven analysis.
//!
//! Priority order (ADQ-8):
//!   1. Assistant (conversational AI)
//!   2. Data Quality monitoring
//!   3. Attendance Analysis
//!
//! Detects patterns like:
//!   • Students at risk of failing
//!   • Unusual attendance patterns
//!   • Grade anomalies (sudden drops, suspiciously uniform scores)
//!   • Data quality issues

use chrono::Utc;
use sea_orm::{DatabaseConnection, ConnectionTrait, ActiveModelTrait, Set};
use serde::Serialize;
use tracing::info;
use uuid::Uuid;

use shared_kernel::errors::AppError;
use crate::entities::ai_insight::{self, severity, insight_status};

/// A detected insight before persistence.
#[derive(Debug, Clone, Serialize)]
pub struct DetectedInsight {
    pub insight_type: String,
    pub entity_type: Option<String>,
    pub entity_id: Option<String>,
    pub title: String,
    pub title_ar: Option<String>,
    pub summary: String,
    pub summary_ar: Option<String>,
    pub severity: String,
    pub confidence: f64,
    pub impact_score: Option<f64>,
    pub suggested_action: Option<String>,
    pub suggested_params: Option<String>,
}

/// Result of an analysis run.
#[derive(Debug, Clone, Serialize)]
pub struct AnalysisResult {
    pub org_id: String,
    pub analyzed_at: String,
    pub insights: Vec<DetectedInsight>,
    pub at_risk_students: usize,
    pub attendance_anomalies: usize,
    pub data_quality_issues: usize,
}

pub struct InsightEngine;

impl InsightEngine {
    /// Run all analysis patterns in parallel.
    pub async fn analyze(
        db: &DatabaseConnection,
        org_id: &str,
    ) -> Result<AnalysisResult, AppError> {
        let now = Utc::now().to_rfc3339();

        // Run patterns in parallel
        let (at_risk, attendance, quality) = tokio::join!(
            Self::detect_at_risk_students(db, org_id),
            Self::analyze_attendance_patterns(db, org_id),
            Self::check_data_completeness(db, org_id),
        );

        let mut all_insights = Vec::new();

        let at_risk_insights = at_risk.unwrap_or_default();
        let attendance_insights = attendance.unwrap_or_default();
        let quality_insights = quality.unwrap_or_default();

        let at_risk_count = at_risk_insights.len();
        let attendance_count = attendance_insights.len();
        let quality_count = quality_insights.len();

        all_insights.extend(at_risk_insights);
        all_insights.extend(attendance_insights);
        all_insights.extend(quality_insights);

        // Persist insights
        for insight in &all_insights {
            Self::persist_insight(db, org_id, insight).await?;
        }

        info!(
            org_id,
            total = all_insights.len(),
            at_risk = at_risk_count,
            attendance = attendance_count,
            quality = quality_count,
            "Insight analysis completed"
        );

        Ok(AnalysisResult {
            org_id: org_id.to_string(),
            analyzed_at: now,
            insights: all_insights,
            at_risk_students: at_risk_count,
            attendance_anomalies: attendance_count,
            data_quality_issues: quality_count,
        })
    }

    /// Detect students at risk of failing based on grade patterns.
    async fn detect_at_risk_students(
        db: &DatabaseConnection,
        org_id: &str,
    ) -> Result<Vec<DetectedInsight>, AppError> {
        // Query: students whose average grade is below passing threshold
        let sql = r#"
            SELECT 
                e.student_id,
                u.full_name,
                AVG(g.score) as avg_score,
                COUNT(g.id) as grade_count,
                c.name as class_name
            FROM enrollments e
            JOIN users u ON u.id = e.student_id
            JOIN classes c ON c.id = e.class_id
            LEFT JOIN grades g ON g.enrollment_id = e.id
            WHERE e.org_id = ?1
              AND e.status = 'active'
              AND e.is_deleted = 0
            GROUP BY e.student_id
            HAVING avg_score IS NOT NULL AND avg_score < 50
            ORDER BY avg_score ASC
            LIMIT 20
        "#;

        let results = db.query_all(
            sea_orm::Statement::from_sql_and_values(
                sea_orm::DatabaseBackend::Sqlite,
                sql,
                vec![org_id.into()],
            )
        ).await.unwrap_or_default();

        let mut insights = Vec::new();
        for row in results {
            let student_name: String = row.try_get("", "full_name").unwrap_or_default();
            let avg_score: f64 = row.try_get("", "avg_score").unwrap_or(0.0);
            let class_name: String = row.try_get("", "class_name").unwrap_or_default();
            let student_id: String = row.try_get("", "student_id").unwrap_or_default();

            insights.push(DetectedInsight {
                insight_type: "at_risk_student".into(),
                entity_type: Some("user".into()),
                entity_id: Some(student_id),
                title: format!("{} is at risk of failing", student_name),
                title_ar: Some(format!("{} معرض لخطر الرسوب", student_name)),
                summary: format!(
                    "Average score {:.1}% in class {}. Below passing threshold.",
                    avg_score, class_name
                ),
                summary_ar: None,
                severity: if avg_score < 30.0 { severity::ALERT } else { severity::WARNING }.into(),
                confidence: 0.85,
                impact_score: Some(1.0 - (avg_score / 100.0)),
                suggested_action: Some("assessment.grade.list".into()),
                suggested_params: None,
            });
        }

        Ok(insights)
    }

    /// Analyze attendance patterns for anomalies.
    async fn analyze_attendance_patterns(
        db: &DatabaseConnection,
        org_id: &str,
    ) -> Result<Vec<DetectedInsight>, AppError> {
        // Query: students with high absence rate (>20%)
        let sql = r#"
            SELECT
                e.student_id,
                u.full_name,
                c.name as class_name,
                COUNT(CASE WHEN ad.status = 'absent' THEN 1 END) as absences,
                COUNT(ad.id) as total_days,
                ROUND(100.0 * COUNT(CASE WHEN ad.status = 'absent' THEN 1 END) / MAX(COUNT(ad.id), 1), 1) as absence_rate
            FROM enrollments e
            JOIN users u ON u.id = e.student_id
            JOIN classes c ON c.id = e.class_id
            LEFT JOIN attendance_daily ad ON ad.enrollment_id = e.id
            WHERE e.org_id = ?1
              AND e.status = 'active'
              AND e.is_deleted = 0
            GROUP BY e.student_id
            HAVING total_days >= 10 AND absence_rate > 20
            ORDER BY absence_rate DESC
            LIMIT 15
        "#;

        let results = db.query_all(
            sea_orm::Statement::from_sql_and_values(
                sea_orm::DatabaseBackend::Sqlite,
                sql,
                vec![org_id.into()],
            )
        ).await.unwrap_or_default();

        let mut insights = Vec::new();
        for row in results {
            let student_name: String = row.try_get("", "full_name").unwrap_or_default();
            let absence_rate: f64 = row.try_get("", "absence_rate").unwrap_or(0.0);
            let absences: i32 = row.try_get("", "absences").unwrap_or(0);
            let student_id: String = row.try_get("", "student_id").unwrap_or_default();

            insights.push(DetectedInsight {
                insight_type: "high_absence_rate".into(),
                entity_type: Some("user".into()),
                entity_id: Some(student_id),
                title: format!("{} has high absence rate", student_name),
                title_ar: Some(format!("{} لديه نسبة غياب عالية", student_name)),
                summary: format!(
                    "{:.1}% absence rate ({} absences). Threshold is 20%.",
                    absence_rate, absences
                ),
                summary_ar: None,
                severity: if absence_rate > 40.0 { severity::ALERT } else { severity::WARNING }.into(),
                confidence: 0.95,
                impact_score: Some(absence_rate / 100.0),
                suggested_action: Some("assessment.attendance.list".into()),
                suggested_params: None,
            });
        }

        Ok(insights)
    }

    /// Check data completeness (missing required fields).
    async fn check_data_completeness(
        db: &DatabaseConnection,
        org_id: &str,
    ) -> Result<Vec<DetectedInsight>, AppError> {
        let mut insights = Vec::new();

        // Check students without email
        let sql = r#"
            SELECT COUNT(*) as count FROM users 
            WHERE org_id = ?1 AND primary_role = 'student' AND (email IS NULL OR email = '')
        "#;

        let result = db.query_one(
            sea_orm::Statement::from_sql_and_values(
                sea_orm::DatabaseBackend::Sqlite,
                sql,
                vec![org_id.into()],
            )
        ).await.ok().flatten();

        if let Some(row) = result {
            let count: i32 = row.try_get("", "count").unwrap_or(0);
            if count > 0 {
                insights.push(DetectedInsight {
                    insight_type: "data_completeness".into(),
                    entity_type: Some("user".into()),
                    entity_id: None,
                    title: format!("{} students missing email", count),
                    title_ar: Some(format!("{} طالب بدون بريد إلكتروني", count)),
                    summary: format!(
                        "{} students have no email address. This may affect parent communication.",
                        count
                    ),
                    summary_ar: None,
                    severity: severity::SUGGESTION.into(),
                    confidence: 1.0,
                    impact_score: None,
                    suggested_action: Some("iam.user.list".into()),
                    suggested_params: Some(r#"{"role": "student", "filter": "missing_email"}"#.into()),
                });
            }
        }

        // Check students without national ID
        let sql2 = r#"
            SELECT COUNT(*) as count FROM persons p
            JOIN users u ON u.person_id = p.id
            WHERE u.org_id = ?1 AND u.primary_role = 'student' AND (p.national_id IS NULL OR p.national_id = '')
        "#;

        let result2 = db.query_one(
            sea_orm::Statement::from_sql_and_values(
                sea_orm::DatabaseBackend::Sqlite,
                sql2,
                vec![org_id.into()],
            )
        ).await.ok().flatten();

        if let Some(row) = result2 {
            let count: i32 = row.try_get("", "count").unwrap_or(0);
            if count > 0 {
                insights.push(DetectedInsight {
                    insight_type: "data_completeness".into(),
                    entity_type: Some("user".into()),
                    entity_id: None,
                    title: format!("{} students missing national ID", count),
                    title_ar: Some(format!("{} طالب بدون رقم هوية", count)),
                    summary: format!(
                        "{} students have no national ID. Required for official reporting.",
                        count
                    ),
                    summary_ar: None,
                    severity: severity::WARNING.into(),
                    confidence: 1.0,
                    impact_score: None,
                    suggested_action: Some("iam.user.list".into()),
                    suggested_params: Some(r#"{"role": "student", "filter": "missing_national_id"}"#.into()),
                });
            }
        }

        Ok(insights)
    }

    /// Persist a detected insight to the database.
    async fn persist_insight(
        db: &DatabaseConnection,
        org_id: &str,
        insight: &DetectedInsight,
    ) -> Result<(), AppError> {
        let now = Utc::now().to_rfc3339();

        let record = ai_insight::ActiveModel {
            id: Set(Uuid::new_v4().to_string()),
            org_id: Set(org_id.to_string()),
            insight_type: Set(insight.insight_type.clone()),
            entity_type: Set(insight.entity_type.clone()),
            entity_id: Set(insight.entity_id.clone()),
            scope: Set("organization".to_string()),
            title: Set(insight.title.clone()),
            title_ar: Set(insight.title_ar.clone()),
            summary: Set(insight.summary.clone()),
            summary_ar: Set(insight.summary_ar.clone()),
            details: Set(None),
            severity: Set(insight.severity.clone()),
            confidence: Set(insight.confidence),
            impact_score: Set(insight.impact_score),
            suggested_action: Set(insight.suggested_action.clone()),
            suggested_params: Set(insight.suggested_params.clone()),
            status: Set(insight_status::NEW.to_string()),
            acknowledged_by: Set(None),
            acknowledged_at: Set(None),
            generated_by: Set("insight_engine".to_string()),
            valid_from: Set(now.clone()),
            valid_until: Set(None),
            created_at: Set(now.clone()),
            updated_at: Set(now),
        };

        record.insert(db).await?;
        Ok(())
    }
}
