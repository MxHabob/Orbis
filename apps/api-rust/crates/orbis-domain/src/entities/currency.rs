use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Currency {
    pub id: String,
    pub name: String,
    pub name_ar: Option<String>,
    pub symbol: String,
    pub decimal_places: i32,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}
