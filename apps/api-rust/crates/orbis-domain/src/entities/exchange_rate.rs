use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExchangeRate {
    pub id: Uuid,
    pub org_id: Uuid,
    pub from_currency: String,
    pub to_currency: String,
    pub rate: Decimal,
    pub effective_date: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}
