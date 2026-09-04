//! Assessment Engine — centralized grading service.
//!
//! This module replaces ALL hardcoded `score_to_grade_letter()` /
//! `score_to_grade_points()` calls. Every grade calculation now goes through
//! the unified pipeline:
//!
//!   CalculationPolicy → CalcEngine.evaluate() → apply_rounding() → GradeMapper.map_score()
//!
//! When no CalculationPolicy or GradeScale is configured for the period,
//! the engine falls back to the organization-level `academic_policies`
//! defaults (grading_system, passing_grade, max_grade).

use sea_orm::{EntityTrait, QueryFilter, ColumnTrait, DatabaseConnection};
use shared_kernel::errors::AppError;
use tracing::warn;

use crate::entities::{grade_scale, grade_scale_item};
use crate::ops::calc_engine::apply_rounding;
use crate::ops::grade_mapper::{GradeMapper, GradeScaleItemData};

// ── Public Result ────────────────────────────────────────────────

/// The result of mapping a raw score through the Assessment Engine.
#[derive(Debug, Clone)]
pub struct AssessmentResult {
    pub grade_letter: String,
    pub grade_points: Option<f64>,
    pub is_passing: bool,
}

// ── Assessment Engine ────────────────────────────────────────────

/// Centralized assessment engine that loads CalculationPolicy + GradeScale
/// for a given period and uses CalcEngine + GradeMapper to produce grades.
pub struct AssessmentEngine {
    mapper: GradeMapper,
    rounding_policy: String,
}

impl AssessmentEngine {
    /// Build an AssessmentEngine for the given period.
    ///
    /// 1. Looks for a `CalculationPolicy` bound to `target_period_id`.
    /// 2. Loads the associated `GradeScale` + items.
    /// 3. Falls back to org-level `academic_policies` defaults if none found.
    pub async fn for_period(
        db: &DatabaseConnection,
        org_id: &str,
        period_id: &str,
    ) -> Result<Self, AppError> {
        let policy = crate::ops::resolution::resolve_calc_policy(
            db,
            org_id,
            period_id,
            "weighted_sum",
        )
        .await?;

        let scale = if let Ok(Some(id)) = crate::ops::resolution::resolve_scale_id(db, org_id, period_id).await {
            grade_scale::Entity::find_by_id(id).one(db).await?
        } else {
            grade_scale::Entity::find()
                .filter(grade_scale::Column::OrgId.eq(org_id))
                .one(db)
                .await?
        };

        let rounding_policy = policy
            .as_ref()
            .map(|p| p.rounding_policy.clone())
            .unwrap_or_else(|| "round_half_up".to_string());

        let mapper = if let Some(ref sc) = scale {
            // Load scale items from DB
            let items = grade_scale_item::Entity::find()
                .filter(grade_scale_item::Column::ScaleId.eq(&sc.id))
                .all(db)
                .await?;

            let mapper_items: Vec<GradeScaleItemData> = items
                .into_iter()
                .map(|i| GradeScaleItemData {
                    label: i.grade_label,
                    min_score: i.min_score,
                    max_score: i.max_score,
                    gpa_points: i.gpa_points,
                    is_passing: i.is_passing == 1,
                })
                .collect();

            if mapper_items.is_empty() {
                warn!(org_id=%org_id, "GradeScale exists but has no items — falling back to defaults");
                Self::default_mapper(db, org_id).await?
            } else {
                GradeMapper::new(mapper_items)
            }
        } else {
            Self::default_mapper(db, org_id).await?
        };

        Ok(Self {
            mapper,
            rounding_policy,
        })
    }

    /// Map a raw percentage (0-100) to a grade letter, GPA points, and pass/fail.
    pub fn map_score(&self, percentage: f64) -> Result<AssessmentResult, AppError> {
        let rounded = apply_rounding(percentage, &self.rounding_policy);
        let result = self.mapper.map_score(rounded)?;
        Ok(AssessmentResult {
            grade_letter: result.label,
            grade_points: result.gpa_points,
            is_passing: result.is_passing,
        })
    }

    /// Convenience: compute percentage from score/max_score and map.
    pub fn compute_and_map(&self, score: f64, max_score: f64) -> Result<AssessmentResult, AppError> {
        if max_score <= 0.0 {
            return Err(AppError::validation("max_score must be greater than zero"));
        }
        let percentage = (score / max_score) * 100.0;
        self.map_score(percentage)
    }

    // ── Fallback: build a mapper from `academic_policies` defaults ──

    /// Constructs a default GradeMapper from the org-level `academic_policies`
    /// table. This is used when no GradeScale has been explicitly configured.
    /// It reads `default_grading_system` to decide which default scale to build.
    async fn default_mapper(
        db: &DatabaseConnection,
        org_id: &str,
    ) -> Result<GradeMapper, AppError> {
        // Read the academic_policies row for this org to detect grading system
        let grading_system = Self::get_org_grading_system(db, org_id).await;

        let items = match grading_system.as_str() {
            "gpa_4_0" => vec![
                GradeScaleItemData { label: "A+".into(), min_score: 95.0, max_score: 100.0, gpa_points: Some(4.0), is_passing: true },
                GradeScaleItemData { label: "A".into(),  min_score: 90.0, max_score: 94.99, gpa_points: Some(3.75), is_passing: true },
                GradeScaleItemData { label: "B+".into(), min_score: 85.0, max_score: 89.99, gpa_points: Some(3.5), is_passing: true },
                GradeScaleItemData { label: "B".into(),  min_score: 80.0, max_score: 84.99, gpa_points: Some(3.0), is_passing: true },
                GradeScaleItemData { label: "C+".into(), min_score: 75.0, max_score: 79.99, gpa_points: Some(2.5), is_passing: true },
                GradeScaleItemData { label: "C".into(),  min_score: 70.0, max_score: 74.99, gpa_points: Some(2.0), is_passing: true },
                GradeScaleItemData { label: "D+".into(), min_score: 65.0, max_score: 69.99, gpa_points: Some(1.5), is_passing: true },
                GradeScaleItemData { label: "D".into(),  min_score: 60.0, max_score: 64.99, gpa_points: Some(1.0), is_passing: true },
                GradeScaleItemData { label: "F".into(),  min_score: 0.0,  max_score: 59.99, gpa_points: Some(0.0), is_passing: false },
            ],
            "pass_fail" => vec![
                GradeScaleItemData { label: "ناجح".into(), min_score: 70.0, max_score: 100.0, gpa_points: None, is_passing: true },
                GradeScaleItemData { label: "راسب".into(), min_score: 0.0,  max_score: 69.99, gpa_points: None, is_passing: false },
            ],
            _ => {
                // Default: "percentage" — Arabic descriptive scale
                vec![
                    GradeScaleItemData { label: "ممتاز".into(),   min_score: 90.0, max_score: 100.0, gpa_points: Some(4.0), is_passing: true },
                    GradeScaleItemData { label: "جيد جداً".into(), min_score: 80.0, max_score: 89.99, gpa_points: Some(3.5), is_passing: true },
                    GradeScaleItemData { label: "جيد".into(),      min_score: 70.0, max_score: 79.99, gpa_points: Some(3.0), is_passing: true },
                    GradeScaleItemData { label: "مقبول".into(),    min_score: 60.0, max_score: 69.99, gpa_points: Some(2.5), is_passing: true },
                    GradeScaleItemData { label: "ضعيف".into(),    min_score: 50.0, max_score: 59.99, gpa_points: Some(2.0), is_passing: false },
                    GradeScaleItemData { label: "راسب".into(),    min_score: 0.0,  max_score: 49.99, gpa_points: Some(0.0), is_passing: false },
                ]
            }
        };

        Ok(GradeMapper::new(items))
    }

    /// Reads `default_grading_system` from `academic_policies` table.
    async fn get_org_grading_system(db: &DatabaseConnection, org_id: &str) -> String {
        use sea_orm::{Statement, ConnectionTrait};

        let result = db
            .query_one(Statement::from_sql_and_values(
                db.get_database_backend(),
                "SELECT default_grading_system FROM academic_policies WHERE org_id = $1 LIMIT 1",
                vec![org_id.into()],
            ))
            .await;

        match result {
            Ok(Some(row)) => row
                .try_get::<String>("", "default_grading_system")
                .unwrap_or_else(|_| "percentage".to_string()),
            _ => "percentage".to_string(),
        }
    }

    /// Read the default passing score from `academic_policies` for the org.
    pub async fn get_org_passing_grade(
        db: &DatabaseConnection,
        org_id: &str,
    ) -> f64 {
        use sea_orm::{Statement, ConnectionTrait};

        let result = db
            .query_one(Statement::from_sql_and_values(
                db.get_database_backend(),
                "SELECT default_passing_score FROM academic_policies WHERE org_id = $1 LIMIT 1",
                vec![org_id.into()],
            ))
            .await;

        match result {
            Ok(Some(row)) => row
                .try_get::<f64>("", "default_passing_score")
                .unwrap_or(50.0),
            _ => 50.0,
        }
    }
}
