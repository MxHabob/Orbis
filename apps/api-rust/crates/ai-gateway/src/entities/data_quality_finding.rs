use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// Unified data quality rules + findings in a single table.
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "data_quality_findings")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub org_id: String,
    /// Rule code: "completeness.student.email", "range.grade.score", etc.
    pub rule_code: String,
    pub entity_type: String,
    pub entity_id: String,
    pub field_name: Option<String>,
    pub severity: String,
    pub current_value: Option<String>,
    pub expected_description: Option<String>,
    pub suggested_fix: Option<String>,
    /// "open" | "auto_fixed" | "manually_fixed" | "ignored" | "false_positive"
    pub status: String,
    pub fixed_by: Option<String>,
    pub fixed_at: Option<String>,
    pub detected_at: String,
    pub created_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
