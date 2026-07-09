use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, utoipa::ToSchema)]
#[sea_orm(table_name = "discount_templates")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub org_id: String,
    pub name: String,
    pub code: Option<String>,
    #[sea_orm(column_name = "type")]
    pub item_type: String,
    pub calculation_method: String,
    pub value: i32,
    pub eligibility_rules: Option<String>,
    pub limits: Option<String>,
    pub requires_approval: i32,
    pub approver_roles: Option<String>,
    pub is_auto_apply: i32,
    pub is_stackable: i32,
    pub priority: Option<i32>,
    pub is_active: i32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
