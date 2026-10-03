use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeriodTemplate {
    pub id: Uuid,
    pub org_id: Uuid,
    pub name: String,
    pub name_ar: Option<String>,
    pub institution_type: String, // school, university, institute, custom
    pub structure: serde_json::Value,
    pub is_default: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
