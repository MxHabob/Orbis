use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, utoipa::ToSchema)]
#[sea_orm(table_name = "payments")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub org_id: String,
    pub fee_id: Option<String>,
    pub enrollment_id: Option<String>,
    pub account_id: String,
    pub amount: f64,
    pub currency: String,
    pub exchange_rate: f64,
    pub base_amount: f64,
    pub method: String,
    pub reference: Option<String>,
    pub receipt_number: Option<String>,
    pub journal_entry_id: Option<String>,
    pub cash_account_id: String,
    pub payer_name: Option<String>,
    pub payer_phone: Option<String>,
    pub payment_type: String,
    pub related_payment_id: Option<String>,
    pub notes: Option<String>,
    pub paid_at: String,
    pub created_at: String,
    pub created_by: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
