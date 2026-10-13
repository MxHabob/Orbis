use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Domain entity for a section — unified grouping concept.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Section {
    pub id: Uuid,
    pub org_id: Uuid,
    pub org_unit_id: Option<Uuid>,
    pub section_type: String,
    pub code: String,
    pub name: String,
    pub name_ar: Option<String>,
    pub level: Option<String>,
    pub track: Option<String>,
    pub capacity: Option<i32>,
    pub color: Option<String>,
    pub is_active: bool,
    pub is_deleted: bool,
    pub deleted_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
