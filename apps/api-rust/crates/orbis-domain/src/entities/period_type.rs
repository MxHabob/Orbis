use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Configurable period type definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeriodType {
    pub id: Uuid,
    pub org_id: Uuid,
    pub code: String,
    pub name: String,
    pub name_ar: Option<String>,
    pub sort_order: i32,
}
