use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExchangeRate {
    pub id: Uuid,
    pub org_id: Uuid,
    pub base_currency_id: String,
    pub target_currency_id: String,
    pub rate: f64,
    pub effective_date: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}
