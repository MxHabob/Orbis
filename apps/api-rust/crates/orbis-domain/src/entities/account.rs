use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: Uuid,
    pub org_id: Uuid,
    pub code: String,
    pub name: String,
    pub account_type: String, // Asset, Liability, Equity, Revenue, Expense
    pub parent_id: Option<Uuid>,
    pub currency: String,
    pub is_active: bool,
    pub balances: Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
