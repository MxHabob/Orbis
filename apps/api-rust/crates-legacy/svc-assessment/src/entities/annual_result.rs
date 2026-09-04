use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// Annual Result — the authoritative, calculated result for a student across
/// all terms in an academic year.
///
/// This record is produced by the centralized `AnnualEngine` and consumed by
/// DocForge annual certificate templates. It combines `TermResult` rows.
///
/// The `policy_snapshot` field captures the annual calculation configuration
/// at finalization time for historical reproducibility.
///
/// Lifecycle: draft → official → finalized (immutable)
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, utoipa::ToSchema)]
#[sea_orm(table_name = "annual_results")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub org_id: String,
    pub student_id: String,
    /// The academic-year-level period ID.
    pub year_period_id: String,
    pub enrollment_id: String,
    pub class_subject_id: String,

    // ── Per-Term Scores ─────────────────────────────────────
    /// JSON array of term contributions, e.g.:
    /// [{"term_period_id":"...", "total_score":40, "total_max":50}, ...]
    pub term_scores_json: String,

    // ── Annual Total ────────────────────────────────────────
    /// Sum of all term totals.
    pub annual_total: f64,
    /// Sum of all term maximums.
    pub annual_max: f64,
    /// (annual_total / annual_max) * 100
    pub percentage: f64,

    // ── Grade Mapping ───────────────────────────────────────
    pub grade_label: Option<String>,
    pub gpa_points: Option<f64>,
    pub is_passing: i32,

    // ── Ranking ─────────────────────────────────────────────
    pub rank: Option<i32>,
    pub rank_scope: Option<String>,

    // ── Traceability ────────────────────────────────────────
    pub calculation_policy_id: Option<String>,
    pub calculation_version: i32,
    /// JSON snapshot of the annual calculation policy at finalization time.
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
