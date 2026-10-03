use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subject {
    pub id: Uuid,
    pub org_id: Uuid,
    pub unit_id: Option<Uuid>,
    pub code: String,
    pub name: String,
    pub name_ar: Option<String>,
    pub description: Option<String>,
    pub is_core: bool,
    pub is_elective: bool,
    pub credit_hours: Option<Decimal>,
    pub theory_hours: Option<Decimal>,
    pub practical_hours: Option<Decimal>,
    pub is_lab_required: bool,
    pub max_students_per_class: Option<u32>,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub sort_order: Option<i32>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
