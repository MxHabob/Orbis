use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, utoipa::ToSchema)]
#[sea_orm(table_name = "calculation_policies")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub org_id: String,
    pub target_period_id: Option<String>,
    pub name: String,
    pub version: i32,
    pub rule_type: String,
    pub formula_expression: String,
    pub rounding_policy: String,
    pub valid_from: Option<String>,
    pub valid_until: Option<String>,
    /// 0 = active, 1 = archived (superseded by newer version).
    pub is_archived: i32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
