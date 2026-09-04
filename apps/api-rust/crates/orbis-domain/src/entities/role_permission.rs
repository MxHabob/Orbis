use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RolePermission {
    pub id: Uuid,
    pub role_id: String,
    pub org_id: Uuid,
    pub resource: String,
    pub action: String,
    pub condition: Option<String>,
    pub created_at: DateTime<Utc>,
}
