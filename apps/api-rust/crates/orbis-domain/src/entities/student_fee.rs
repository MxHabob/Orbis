use uuid::Uuid;
use chrono::{DateTime, Utc, NaiveDate};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudentFee {
    pub id: Uuid,
    pub org_id: Uuid,
    pub student_id: Uuid,
    pub enrollment_id: Option<Uuid>,
    pub fee_template_id: Uuid,
    pub original_amount: f64,
    pub discount_amount: f64,
    pub net_amount: f64,
    pub status: String, // Pending, Paid, PartiallyPaid
    pub due_date: NaiveDate,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
