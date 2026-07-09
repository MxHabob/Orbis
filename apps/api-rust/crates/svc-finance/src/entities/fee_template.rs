use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, utoipa::ToSchema)]
#[sea_orm(table_name = "fee_templates")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub org_id: String,
    pub name: String,
    pub name_ar: Option<String>,
    pub code: Option<String>,
    pub category: String,
    pub default_amount: f64,
    pub revenue_account_id: String,
    pub receivable_account_id: String,
    pub applicable_levels: Option<String>,
    pub frequency: String,
    pub installment_config: Option<String>,
    pub late_fee_rules: Option<String>,
    pub is_mandatory: i32,
    pub is_active: i32,
    pub sort_order: Option<i32>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
