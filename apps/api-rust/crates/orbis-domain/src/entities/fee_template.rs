use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FeeFrequency {
    Once,
    PerTerm,
    Monthly,
    Annual,
}

impl std::fmt::Display for FeeFrequency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FeeFrequency::Once => write!(f, "once"),
            FeeFrequency::PerTerm => write!(f, "per_term"),
            FeeFrequency::Monthly => write!(f, "monthly"),
            FeeFrequency::Annual => write!(f, "annual"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeTemplate {
    pub id: Uuid,
    pub org_id: Uuid,
    pub code: String,
    pub name: String,
    pub name_ar: Option<String>,
    pub category: String,
    pub default_amount: Decimal,
    pub currency: String,
    pub revenue_account_id: Uuid,
    pub receivable_account_id: Uuid,
    pub applicable_levels: Option<Vec<String>>,
    pub applicable_study_modes: Option<Vec<String>>,
    pub frequency: FeeFrequency,
    pub is_mandatory: bool,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
