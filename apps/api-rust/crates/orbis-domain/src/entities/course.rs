use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Course {
    pub id: Uuid,
    pub org_id: Uuid,
    pub unit_id: Option<Uuid>,
    pub code: String,
    pub name: String,
    pub name_ar: Option<String>,
    pub description: Option<String>,
    pub credit_units: Option<Decimal>,
    pub workload_units: Option<Decimal>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
