use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    pub id: Uuid,
    pub org_id: Uuid,
    pub name: String,
    pub name_ar: Option<String>,
    pub description: Option<String>,
    pub is_system: bool,
    pub status: String, // "active", "inactive"
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserGroupMembership {
    pub id: Uuid,
    pub org_id: Uuid,
    pub user_id: Uuid,
    pub group_id: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupPermission {
    pub id: Uuid,
    pub org_id: Uuid,
    pub group_id: Uuid,
    pub permission: String, // Canonical permission e.g. "student:read", "employee:write"
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupDetail {
    pub group: Group,
    pub member_count: usize,
    pub permissions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupMemberInfo {
    pub user_id: Uuid,
    pub person_id: Uuid,
    pub full_name: String,
    pub full_name_ar: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub avatar_url: Option<String>,
    pub user_status: String,
    pub joined_at: DateTime<Utc>,
}
