use uuid::Uuid;
use chrono::{DateTime, Utc, NaiveDate};
use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FeeStatus {
    Pending,
    PartiallyPaid,
    Paid,
    Overdue,
    Cancelled,
    Refunded,
}

impl std::fmt::Display for FeeStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FeeStatus::Pending => write!(f, "pending"),
            FeeStatus::PartiallyPaid => write!(f, "partially_paid"),
            FeeStatus::Paid => write!(f, "paid"),
            FeeStatus::Overdue => write!(f, "overdue"),
            FeeStatus::Cancelled => write!(f, "cancelled"),
            FeeStatus::Refunded => write!(f, "refunded"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudentFee {
    pub id: Uuid,
    pub org_id: Uuid,
    pub period_id: Uuid,
    pub student_id: Uuid,
    pub enrollment_id: Option<Uuid>,
    pub fee_template_id: Uuid,
    pub original_amount: Decimal,
    pub discount_amount: Decimal,
    pub net_amount: Decimal,
    pub paid_amount: Decimal,
    pub balance_due: Decimal,
    pub status: FeeStatus,
    pub due_date: NaiveDate,
    pub discount_details: Option<serde_json::Value>,
    pub installment_number: Option<u32>,
    pub notes: Option<String>,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl StudentFee {
    pub fn apply_payment(&mut self, payment_amount: Decimal) {
        self.paid_amount += payment_amount;
        if self.paid_amount >= self.net_amount {
            self.balance_due = Decimal::ZERO;
            self.status = FeeStatus::Paid;
        } else if self.paid_amount > Decimal::ZERO {
            self.balance_due = self.net_amount - self.paid_amount;
            self.status = FeeStatus::PartiallyPaid;
        }
        self.updated_at = Utc::now();
    }
}
