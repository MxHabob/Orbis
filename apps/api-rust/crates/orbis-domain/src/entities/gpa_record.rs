use crate::entities::student::AcademicStanding;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CumulativeGpaRecord {
    pub id: Uuid,
    pub org_id: Uuid,
    pub student_id: Uuid,
    pub period_id: Uuid,
    pub term_attempted_credits: Decimal,
    pub term_earned_credits: Decimal,
    pub term_points: Decimal,
    pub term_gpa: Decimal,
    pub cumulative_attempted_credits: Decimal,
    pub cumulative_earned_credits: Decimal,
    pub cumulative_points: Decimal,
    pub cumulative_gpa: Decimal,
    pub standing: AcademicStanding,
    pub calculated_at: DateTime<Utc>,
}
