use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// Ranking Policy — configurable ranking behaviour per institution/period.
///
/// Ranking is an institutional policy, not a hard-coded behaviour.
/// Each institution (or individual period) can configure:
///
/// - Whether ranking is enabled at all.
/// - The scope of the ranking population (class, grade level, section, branch, program).
/// - The ranking method (dense, standard, modified_competition).
/// - How ties are handled.
/// - Whether students who failed participate in ranking.
/// - Which metric to rank by (total_score, gpa, percentage).
///
/// When no period-specific policy exists, the org-level default is used.
/// The ranking value is always stored as a numeric integer.
/// Localized ordinal text (1st / الأول) is a presentation concern.
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, utoipa::ToSchema)]
#[sea_orm(table_name = "ranking_policies")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub org_id: String,
    /// Optional period this policy applies to. NULL = org-wide default.
    pub period_id: Option<String>,
    pub name: String,

    /// Whether ranking is enabled. 0 = disabled, 1 = enabled.
    pub is_enabled: i32,

    /// Population scope: "class", "grade", "section", "branch", "program"
    pub ranking_scope: String,

    /// Method: "dense" (1,2,2,3), "standard" (1,2,2,4), "modified_competition" (1,2,2,4)
    pub ranking_method: String,

    /// Tie handling: "same_rank" (ties share rank), "next_rank" (skip after tie)
    pub tie_handling: String,

    /// Whether failed students participate in ranking. 0 = excluded, 1 = included.
    pub include_failed_students: i32,

    /// Metric to rank by: "total_score", "gpa", "percentage"
    pub ranking_metric: String,

    pub created_at: String,
    pub updated_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
