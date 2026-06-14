use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, utoipa::ToSchema)]
#[sea_orm(table_name = "payments")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub org_id: String,
    pub fee_id: String,
    pub enrollment_id: String,
    pub amount: f64,
    pub currency: String,
    pub exchange_rate: Option<f64>,
    pub method: String,
    pub reference: Option<String>,
    pub transaction_id: Option<String>,
    pub receipt_number: Option<String>,
    pub journal_entry_id: Option<String>,
    pub treasury_id: Option<String>,
    pub payer_name: Option<String>,
    pub payer_phone: Option<String>,
    pub payer_relation: Option<String>,
    pub payer_id: Option<String>,
    pub is_verified: i32,
    pub verified_by: Option<String>,
    pub verified_at: Option<String>,
    pub is_refund: i32,
    pub refund_amount: Option<f64>,
    pub refund_reason: Option<String>,
    pub refund_approved_by: Option<String>,
    pub receipt_url: Option<String>,
    pub proof_url: Option<String>,
    pub reconciled: Option<i32>,
    pub reconciled_at: Option<String>,
    pub reconciled_by: Option<String>,
    pub notes: Option<String>,
    pub meta: Option<String>,
    pub paid_at: String,
    pub created_at: String,
    pub created_by: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
