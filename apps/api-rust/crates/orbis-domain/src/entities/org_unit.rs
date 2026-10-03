use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrgUnit {
    pub id: Uuid,
    pub org_id: Uuid,
    pub parent_id: Option<Uuid>,

    // Materialized Path
    pub path: String,
    pub depth: i32,

    pub name: String,
    pub name_ar: Option<String>,
    pub code: Option<String>,
    pub unit_type: String, // organization, branch, department, division, unit
    pub head_id: Option<Uuid>, // references User/Person ID

    // Physical Location
    pub capacity: Option<i32>,
    pub room_number: Option<String>,
    pub floor: Option<i32>,
    pub building: Option<String>,

    // Settings & Isolation
    pub settings: Option<serde_json::Value>,
    pub local_policies: Option<serde_json::Value>,
    pub data_isolation: String, // shared, isolated, read_only

    // Denormalized Hierarchy Counts
    pub children_count: i32,
    pub descendant_count: i32,

    pub sort_order: Option<i32>,
    pub is_active: bool,

    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
