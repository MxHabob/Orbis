use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Role {
    SuperAdmin,
    OrgAdmin,
    Teacher,
    Student,
    Parent,
    Custom(String),
}

impl ToString for Role {
    fn to_string(&self) -> String {
        match self {
            Role::SuperAdmin => "SuperAdmin".to_string(),
            Role::OrgAdmin => "OrgAdmin".to_string(),
            Role::Teacher => "Teacher".to_string(),
            Role::Student => "Student".to_string(),
            Role::Parent => "Parent".to_string(),
            Role::Custom(s) => s.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub person_id: Uuid,
    
    pub org_id: Uuid,
    pub email: Option<String>,
    pub email_verified: bool,
    pub phone: Option<String>,
    pub phone_verified: bool,
    pub password_hash: Option<String>,
    pub mfa_enabled: bool,
    pub full_name: String,
    
    pub status: String,
    pub login_count: i32,
    pub failed_logins: i32,
    pub locked_until: Option<DateTime<Utc>>,
    pub is_deleted: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
