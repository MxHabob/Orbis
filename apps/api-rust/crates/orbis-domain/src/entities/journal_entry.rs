use uuid::Uuid;
use chrono::{DateTime, Utc, NaiveDate};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JournalEntry {
    pub id: Uuid,
    pub org_id: Uuid,
    pub transaction_ref: String,
    pub accounting_date: NaiveDate,
    pub amount: f64,
    pub currency_id: String,
    pub exchange_rate: f64,
    pub base_amount: f64,
    pub debit_account_id: Uuid,
    pub credit_account_id: Uuid,
    pub description: String,
    pub created_at: DateTime<Utc>,
}
