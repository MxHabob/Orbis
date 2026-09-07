use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiscountType {
    Percentage,
    FixedAmount,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscountTemplate {
    pub id: Uuid,
    pub org_id: Uuid,
    pub code: String,
    pub name: String,
    pub name_ar: Option<String>,
    pub discount_type: DiscountType,
    pub value: Decimal,
    pub eligibility_rules: Option<serde_json::Value>,
    pub requires_approval: bool,
    pub is_auto_apply: bool,
    pub is_stackable: bool,
    pub priority: u32,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
