use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// Pass/Fail Policy — determines passing thresholds per academic period.
///
/// Each period can have its own passing policy. When none is set for a period,
/// the system falls back to the org-level `academic_policies.default_passing_grade`.
///
/// Fields:
/// - `min_passing_score`: Minimum percentage to pass (e.g. 50.0, 60.0)
/// - `requires_all_subjects`: If 1, student must pass every subject to pass overall
/// - `max_failed_subjects`: Maximum number of subjects a student can fail and still pass
///   (only relevant when `requires_all_subjects` is 0)
/// - `supplementary_exam_eligible`: If 1, students who fail within range get supplementary exam
/// - `supplementary_max_score`: Max percentage achievable in supplementary exam
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, utoipa::ToSchema)]
#[sea_orm(table_name = "pass_fail_policies")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub org_id: String,
    pub period_id: Option<String>,
    pub name: String,
    pub min_passing_score: f64,
    pub requires_all_subjects: i32,
    pub max_failed_subjects: i32,
    pub supplementary_exam_eligible: i32,
    pub supplementary_max_score: Option<f64>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
