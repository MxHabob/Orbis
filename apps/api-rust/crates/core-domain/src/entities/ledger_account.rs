use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, utoipa::ToSchema)]
#[sea_orm(table_name = "ledger_accounts")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub org_id: String,
    pub code: String,
    pub name: String,
    pub name_ar: Option<String>,
    #[sea_orm(column_name = "type")]
    pub item_type: String,
    pub sub_type: Option<String>,
    pub parent_id: Option<String>,
    pub path: String,
    pub depth: i32,
    pub currency: String,
    pub opening_balance: Option<f64>,
    pub current_balance: Option<f64>,
    pub entity_type: Option<String>,
    pub entity_id: Option<String>,
    pub is_control_account: i32,
    pub is_system_account: i32,
    pub is_active: i32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
