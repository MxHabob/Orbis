use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, utoipa::ToSchema)]
pub struct FeeTemplateParams {
    pub category: Option<String>,
    pub is_active: Option<i32>,
}

#[derive(Deserialize, Serialize, Debug, Clone, utoipa::ToSchema)]
pub struct StudentFeeParams {
    pub enrollment_id: Option<String>,
    pub period_id: Option<String>,
    pub status: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, utoipa::ToSchema)]
pub struct PaymentParams {
    pub enrollment_id: Option<String>,
    pub fee_id: Option<String>,
    pub cash_account_id: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, utoipa::ToSchema)]
pub struct LedgerAccountParams {
    pub item_type: Option<String>,
    pub is_active: Option<i32>,
}
