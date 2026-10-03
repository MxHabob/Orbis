use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Grade {
    pub id: Uuid,
    pub org_id: Uuid,
    pub period_id: Uuid,
    pub student_id: Uuid,
    pub enrollment_id: Option<Uuid>,
    pub class_subject_id: Uuid,
    pub component_id: Option<Uuid>,
    pub score: Decimal,
    pub max_score: Decimal,
    pub weight: Option<Decimal>,
    pub letter_grade: Option<String>,
    pub is_final: bool,
    pub is_excused: bool,
    pub is_exempt: bool, // For Auditor students
    pub change_reason: Option<String>,
    pub recorded_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Grade {
    pub fn percentage(&self) -> Option<Decimal> {
        if self.max_score > Decimal::ZERO {
            Some((self.score / self.max_score) * Decimal::from(100))
        } else {
            None
        }
    }
}
