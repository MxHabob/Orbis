//! Data Quality — monitors data integrity and completeness.
//!
//! Runs quality rules against entities and creates findings in
//! the `data_quality_findings` table.

use chrono::Utc;
use sea_orm::{DatabaseConnection, ConnectionTrait, ActiveModelTrait, Set};
use tracing::info;
use uuid::Uuid;

use shared_kernel::errors::AppError;
use crate::entities::data_quality_finding;

/// A quality rule definition.
#[derive(Debug, Clone)]
pub struct QualityRule {
    pub code: String,
    pub description: String,
    pub entity_type: String,
    pub severity: String,
    pub sql: String,
}

/// Result of a quality check run.
#[derive(Debug, Clone)]
pub struct QualityCheckResult {
    pub org_id: String,
    pub rules_checked: usize,
    pub findings_created: usize,
}

pub struct DataQualityMonitor;

impl DataQualityMonitor {
    /// Run all data quality rules for an organization.
    pub async fn check(
        db: &DatabaseConnection,
        org_id: &str,
    ) -> Result<QualityCheckResult, AppError> {
        let rules = Self::get_rules();
        let mut findings_count = 0;

        for rule in &rules {
            let sql = rule.sql.replace("?org_id", &format!("'{}'", org_id));

            let results = db.query_all(
                sea_orm::Statement::from_string(
                    sea_orm::DatabaseBackend::Sqlite,
                    sql,
                )
            ).await.unwrap_or_default();

            for row in results {
                let entity_id: String = row.try_get("", "entity_id").unwrap_or_default();
                let field_name: Option<String> = row.try_get("", "field_name").ok();
                let current_value: Option<String> = row.try_get("", "current_value").ok();

                let now = Utc::now().to_rfc3339();
                let finding = data_quality_finding::ActiveModel {
                    id: Set(Uuid::new_v4().to_string()),
                    org_id: Set(org_id.to_string()),
                    rule_code: Set(rule.code.clone()),
                    entity_type: Set(rule.entity_type.clone()),
                    entity_id: Set(entity_id),
                    field_name: Set(field_name),
                    severity: Set(rule.severity.clone()),
                    current_value: Set(current_value),
                    expected_description: Set(Some(rule.description.clone())),
                    suggested_fix: Set(None),
                    status: Set("open".to_string()),
                    fixed_by: Set(None),
                    fixed_at: Set(None),
                    detected_at: Set(now.clone()),
                    created_at: Set(now),
                };

                let _ = finding.insert(db).await;
                findings_count += 1;
            }
        }

        info!(
            org_id,
            rules = rules.len(),
            findings = findings_count,
            "Data quality check completed"
        );

        Ok(QualityCheckResult {
            org_id: org_id.to_string(),
            rules_checked: rules.len(),
            findings_created: findings_count,
        })
    }

    /// Built-in quality rules.
    fn get_rules() -> Vec<QualityRule> {
        vec![
            QualityRule {
                code: "completeness.student.email".into(),
                description: "Student must have an email address".into(),
                entity_type: "user".into(),
                severity: "warning".into(),
                sql: r#"
                    SELECT id as entity_id, 'email' as field_name, email as current_value
                    FROM users
                    WHERE org_id = ?org_id AND role = 'student'
                      AND (email IS NULL OR email = '')
                    LIMIT 50
                "#.into(),
            },
            QualityRule {
                code: "completeness.student.national_id".into(),
                description: "Student must have a national ID for official records".into(),
                entity_type: "user".into(),
                severity: "warning".into(),
                sql: r#"
                    SELECT id as entity_id, 'national_id' as field_name, national_id as current_value
                    FROM users
                    WHERE org_id = ?org_id AND role = 'student'
                      AND (national_id IS NULL OR national_id = '')
                    LIMIT 50
                "#.into(),
            },
            QualityRule {
                code: "completeness.student.phone".into(),
                description: "Student should have a phone number".into(),
                entity_type: "user".into(),
                severity: "info".into(),
                sql: r#"
                    SELECT id as entity_id, 'phone' as field_name, phone as current_value
                    FROM users
                    WHERE org_id = ?org_id AND role = 'student'
                      AND (phone IS NULL OR phone = '')
                    LIMIT 50
                "#.into(),
            },
            QualityRule {
                code: "range.grade.score".into(),
                description: "Grade score must be between 0 and max_score".into(),
                entity_type: "grade".into(),
                severity: "alert".into(),
                sql: r#"
                    SELECT g.id as entity_id, 'score' as field_name, 
                           CAST(g.score AS TEXT) as current_value
                    FROM grades g
                    JOIN assessments a ON a.id = g.assessment_id
                    WHERE g.org_id = ?org_id
                      AND (g.score < 0 OR g.score > a.max_score)
                    LIMIT 50
                "#.into(),
            },
            QualityRule {
                code: "orphan.enrollment.no_class".into(),
                description: "Enrollment must reference an existing class".into(),
                entity_type: "enrollment".into(),
                severity: "alert".into(),
                sql: r#"
                    SELECT e.id as entity_id, 'class_id' as field_name, 
                           e.class_id as current_value
                    FROM enrollments e
                    LEFT JOIN classes c ON c.id = e.class_id
                    WHERE e.org_id = ?org_id AND c.id IS NULL
                    LIMIT 50
                "#.into(),
            },
        ]
    }
}
