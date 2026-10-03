use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Class {
    pub id: Uuid,
    pub org_id: Uuid,
    pub unit_id: Uuid,
    pub period_id: Uuid,
    pub name: String,
    pub name_ar: Option<String>,
    pub level: String,
    pub division: Option<String>,
    pub homeroom_teacher_id: Option<Uuid>,
    pub capacity: Option<i32>,
    pub current_students: Option<i32>,
    pub room_id: Option<Uuid>,
    pub color: Option<String>,
    pub is_active: bool,
    pub is_deleted: bool,
    pub deleted_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
