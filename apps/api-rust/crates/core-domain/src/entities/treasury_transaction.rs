use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, utoipa::ToSchema)]
#[sea_orm(table_name = "treasury_transactions")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub org_id: String,
    pub treasury_id: String,
    pub journal_entry_id: String,
    #[sea_orm(column_name = "type")]
    pub item_type: String,
    pub category: String,
    pub amount: f64,
    pub currency: String,
    pub exchange_rate: Option<f64>,
    pub reference_type: Option<String>,
    pub reference_id: Option<String>,
    pub description: String,
    pub notes: Option<String>,
    pub attachments: Option<String>,
    pub created_by: Option<String>,
    pub approved_by: Option<String>,
    pub approved_at: Option<String>,
    pub transaction_date: String,
    pub created_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
