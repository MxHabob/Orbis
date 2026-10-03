use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethod {
    Cash,
    BankTransfer,
    Check,
    CreditCard,
    Mada,
    ApplePay,
    Other,
}

impl std::fmt::Display for PaymentMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PaymentMethod::Cash => write!(f, "cash"),
            PaymentMethod::BankTransfer => write!(f, "bank_transfer"),
            PaymentMethod::Check => write!(f, "check"),
            PaymentMethod::CreditCard => write!(f, "credit_card"),
            PaymentMethod::Mada => write!(f, "mada"),
            PaymentMethod::ApplePay => write!(f, "apple_pay"),
            PaymentMethod::Other => write!(f, "other"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payment {
    pub id: Uuid,
    pub org_id: Uuid,
    pub fee_id: Option<Uuid>,
    pub enrollment_id: Option<Uuid>,
    pub student_id: Option<Uuid>,
    pub account_id: Uuid,
    pub amount: Decimal,
    pub currency_code: String,
    pub exchange_rate: Decimal,
    pub base_amount: Decimal,
    pub method: PaymentMethod,
    pub reference: Option<String>,
    pub receipt_number: Option<String>,
    pub journal_entry_id: Option<Uuid>,
    pub payer_name: Option<String>,
    pub payer_phone: Option<String>,
    pub notes: Option<String>,
    pub paid_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub org_unit_id: Option<Uuid>,
}
