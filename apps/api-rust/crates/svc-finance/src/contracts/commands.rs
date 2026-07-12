//! Finance write commands.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateLedgerAccountCmd {
    pub code: String,
    pub name: String,
    pub name_ar: Option<String>,
    pub item_type: String, // asset, liability, equity, revenue, expense
    pub sub_type: Option<String>,
    pub parent_id: Option<String>,
    pub currency: String,
    pub opening_balance: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateJournalEntryCmd {
    pub transaction_ref: String,
    pub accounting_date: String,
    pub fiscal_period_id: String,
    pub amount: f64,
    pub debit_account_id: String,
    pub credit_account_id: String,
    pub currency: String,
    pub exchange_rate: Option<f64>,
    pub description: String,
    pub description_ar: Option<String>,
    pub source_type: String,
    pub source_id: Option<String>,
    pub source_module: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordPaymentCmd {
    pub fee_id: Option<String>,
    pub enrollment_id: Option<String>,
    pub account_id: String,
    pub amount: f64,
    pub currency: String,
    pub exchange_rate: f64,
    pub method: String,
    pub treasury_id: Option<String>,
    pub payer_name: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFeeTemplateCmd {
    pub name: String,
    pub name_ar: Option<String>,
    pub category: String,
    pub default_amount: f64,
    pub revenue_account_id: String,
    pub receivable_account_id: String,
    pub frequency: String,
    pub is_mandatory: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFeeTemplateCmd {
    pub name: Option<String>,
    pub default_amount: Option<f64>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssignFeeCmd {
    pub enrollment_id: String,
    pub fee_template_id: String,
    pub amount_override: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTreasuryCmd {
    pub name: String,
    pub name_ar: Option<String>,
    pub code: String,
    pub treasury_type: String,
    pub account_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDiscountTemplateCmd {
    pub name: String,
    pub name_ar: Option<String>,
    pub amount: f64,
    pub is_percentage: bool,
    pub account_id: String,
}
