use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, utoipa::ToSchema)]
#[sea_orm(table_name = "journal_entries")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub org_id: String,
    pub transaction_ref: String,
    pub accounting_date: String,
    pub fiscal_period_id: String,
    pub amount: f64,
    pub debit_account_id: String,
    pub credit_account_id: String,
    pub currency: String,
    pub exchange_rate: Option<f64>,
    pub base_currency_amount: Option<f64>,
    pub source_type: String,
    pub source_id: Option<String>,
    pub source_module: Option<String>,
    pub description: String,
    pub description_ar: Option<String>,
    pub metadata: Option<String>,
    pub is_reversed: i32,
    pub reversed_by: Option<String>,
    pub reversal_reason: Option<String>,
    pub is_reconciled: i32,
    pub reconciled_at: Option<String>,
    pub reconciliation_ref: Option<String>,
    pub status: String,
    pub posted_by: Option<String>,
    pub posted_at: Option<String>,
    pub approved_by: Option<String>,
    pub approved_at: Option<String>,
    pub created_by: Option<String>,
    pub created_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
