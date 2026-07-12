use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, utoipa::ToSchema)]
#[sea_orm(table_name = "treasuries")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub org_id: String,
    pub unit_id: Option<String>,
    pub name: String,
    pub name_ar: Option<String>,
    pub code: Option<String>,
    #[sea_orm(column_name = "type")]
    pub item_type: String,
    pub account_id: String,
    pub bank_name: Option<String>,
    pub bank_branch: Option<String>,
    pub account_number: Option<String>,
    pub account_name: Option<String>,
    pub iban: Option<String>,
    pub swift_code: Option<String>,
    pub opening_balance: Option<f64>,
    pub current_balance: Option<f64>,
    pub currency: String,
    pub authorized_users: Option<String>,
    pub is_default: i32,
    pub is_active: i32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
