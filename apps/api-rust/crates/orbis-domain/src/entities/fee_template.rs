use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeTemplate {
    pub id: Uuid,
    pub org_id: Uuid,
    pub name: String,
    pub default_amount: f64,
    pub revenue_account_id: Uuid,
    pub receivable_account_id: Uuid,
    pub is_mandatory: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
