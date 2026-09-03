use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// Term Result — the authoritative, calculated result for a student in a term.
///
/// This record is produced by the centralized `TermEngine` and consumed by
/// DocForge certificate templates. It is the single source of truth for
/// term-level academic results.
///
/// The `policy_snapshot` field captures the full calculation configuration
/// (formula, weights, rounding) at finalization time so that historical
/// results can always be reproduced even if the live policy changes.
///
/// Lifecycle: draft → official → finalized (immutable)
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, utoipa::ToSchema)]
#[sea_orm(table_name = "term_results")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub org_id: String,
    pub student_id: String,
    /// The term-level academic period ID (e.g. "Semester 1").
    pub term_period_id: String,
    pub enrollment_id: String,
    pub class_subject_id: String,

    // ── Coursework Breakdown ────────────────────────────────
    /// Aggregated coursework score from monthly/grading-period results.
    pub coursework_score: f64,
    /// Maximum possible coursework score (e.g. 20).
    pub coursework_max: f64,

    // ── Examination ─────────────────────────────────────────
    /// Term examination raw score.
    pub exam_score: f64,
    /// Maximum possible examination score (e.g. 30).
    pub exam_max: f64,

    // ── Term Total ──────────────────────────────────────────
    /// coursework_score + exam_score
    pub total_score: f64,
    /// coursework_max + exam_max (e.g. 50)
    pub total_max: f64,
    /// (total_score / total_max) * 100
    pub percentage: f64,

    // ── Grade Mapping ───────────────────────────────────────
    pub grade_label: Option<String>,
    pub gpa_points: Option<f64>,
    pub is_passing: i32,

    // ── Ranking ─────────────────────────────────────────────
    pub rank: Option<i32>,
    /// Scope at which the rank was computed (e.g. "class", "grade").
    pub rank_scope: Option<String>,

    // ── Traceability ────────────────────────────────────────
    /// ID of the CalculationPolicy used.
    pub calculation_policy_id: Option<String>,
    /// Version of the CalculationPolicy used.
    pub calculation_version: i32,
    /// JSON snapshot of the full policy at finalization time.
    /// Sufficient to reproduce the calculation independently.
    pub policy_snapshot: Option<String>,

    // ── Lifecycle ───────────────────────────────────────────
    /// "draft" | "official" | "finalized"
    pub status: String,
    pub finalized_at: Option<String>,
    pub finalized_by: Option<String>,

    pub created_at: String,
    pub updated_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
