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
    pub section_subject_id: Uuid,
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

    pub fn entry_status(&self) -> GradeEntryStatus {
        if self.is_exempt {
            GradeEntryStatus::Exempt
        } else if self.is_excused {
            GradeEntryStatus::Excused
        } else if self.notes.as_deref() == Some("ABSENT")
            || self.change_reason.as_deref() == Some("ABSENT")
        {
            GradeEntryStatus::Absent
        } else {
            GradeEntryStatus::Recorded
        }
    }
}

/// Status of an individual assessment entry, distinguishing recorded, missing, excused, and absent marks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GradeEntryStatus {
    Recorded,
    Missing,
    Excused,
    Exempt,
    Absent,
}
