use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcademicYear {
    pub id: Uuid,
    pub org_id: Uuid,
    pub name: String,
    pub name_ar: Option<String>,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub hijri_start: Option<String>,
    pub hijri_end: Option<String>,
    pub status: String, // "planning", "active", "closing", "archived"
    pub is_locked: bool,
    pub locked_at: Option<DateTime<Utc>>,
    pub locked_by: Option<Uuid>,
    pub lock_reason: Option<String>,
    pub current_period_id: Option<Uuid>,
    pub archived_at: Option<DateTime<Utc>>,
    pub archived_by: Option<Uuid>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl AcademicYear {
    pub fn is_writable(&self) -> bool {
        self.status != "archived" && !self.is_locked
    }

    pub fn can_enroll(&self) -> bool {
        self.is_writable() && self.status == "active"
    }

    pub fn can_enter_grades(&self) -> bool {
        self.is_writable() && (self.status == "active" || self.status == "closing")
    }
}
