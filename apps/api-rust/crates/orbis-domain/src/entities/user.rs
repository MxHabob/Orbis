use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub person_id: Uuid,
    pub org_id: Uuid,
    pub email: Option<String>,
    pub is_email_verified: bool,
    pub phone: Option<String>,
    pub is_phone_verified: bool,
    pub password_hash: Option<String>,
    pub is_mfa_enabled: bool,
    pub status: String,
    pub login_count: i32,
    pub failed_login_attempts: i32,
    pub locked_until: Option<DateTime<Utc>>,
    pub is_deleted: bool,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
