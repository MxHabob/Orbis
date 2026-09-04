use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Person {
    pub id: Uuid,
    pub party_id: Option<Uuid>,
    pub org_id: Option<Uuid>,
    pub first_name: String,
    pub last_name: String,
    pub full_name: Option<String>,
    pub national_id: Option<String>,
    pub gender: Option<String>,
    pub date_of_birth: Option<String>, // Can be NaiveDate in future, but keeping string for now to match legacy
    pub nationality: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
