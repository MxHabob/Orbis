use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRoleAssignment {
    pub id: Uuid,
    pub user_id: Uuid,
    pub role_id: String,
    pub org_id: Uuid,
    pub scope_type: String,
    pub scope_id: Option<Uuid>,
    pub assigned_by: Option<String>,
    pub created_at: DateTime<Utc>,
}
