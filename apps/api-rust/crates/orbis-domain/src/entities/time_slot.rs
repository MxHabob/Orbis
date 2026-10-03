use uuid::Uuid;
use chrono::{DateTime, Utc, NaiveTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeSlot {
    pub id: Uuid,
    pub period_id: Uuid,
    pub day_of_week: i16,
    pub slot_index: i32,
    pub name: Option<String>,
    pub name_ar: Option<String>,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub is_break: bool,
    pub break_type: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
