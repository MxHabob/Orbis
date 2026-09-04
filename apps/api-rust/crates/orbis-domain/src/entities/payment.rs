use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payment {
    pub id: Uuid,
    pub org_id: Uuid,
    pub fee_id: Option<Uuid>,
    pub account_id: Uuid, // Cash or Bank account where money is received
    pub amount: f64,
    pub method: String, // Cash, Card, BankTransfer
    pub reference: Option<String>,
    pub journal_entry_id: Option<Uuid>,
    pub paid_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}
