use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum JournalPostingStatus {
    Draft,
    Posted,
    Reconciled,
    Reversed,
}

impl std::fmt::Display for JournalPostingStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JournalPostingStatus::Draft => write!(f, "draft"),
            JournalPostingStatus::Posted => write!(f, "posted"),
            JournalPostingStatus::Reconciled => write!(f, "reconciled"),
            JournalPostingStatus::Reversed => write!(f, "reversed"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JournalEntryLine {
    pub id: Uuid,
    pub account_id: Uuid,
    pub debit_amount: Decimal,
    pub credit_amount: Decimal,
    pub description: Option<String>,
    pub entity_type: Option<String>,
    pub entity_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JournalEntry {
    pub id: Uuid,
    pub org_id: Uuid,
    pub reference_number: String,
    pub accounting_date: NaiveDate,
    pub amount: Decimal,
    pub currency_code: String,
    pub exchange_rate: Decimal,
    pub base_amount: Decimal,
    pub debit_account_id: Uuid,
    pub credit_account_id: Uuid,
    pub description: String,
    pub description_ar: Option<String>,
    pub source_type: String,
    pub source_id: Option<Uuid>,
    pub status: JournalPostingStatus,
    pub lines: Vec<JournalEntryLine>,
    pub is_reversed: bool,
    pub reversed_by: Option<Uuid>,
    pub reversal_reason: Option<String>,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub updated_by: Option<Uuid>,
    pub org_unit_id: Option<Uuid>,
    pub version: i32,
}

impl JournalEntry {
    pub fn is_balanced(&self) -> bool {
        if self.lines.is_empty() {
            // Simple 2-legged entry
            return self.amount > Decimal::ZERO;
        }
        let total_debit: Decimal = self.lines.iter().map(|l| l.debit_amount).sum();
        let total_credit: Decimal = self.lines.iter().map(|l| l.credit_amount).sum();
        total_debit == total_credit && total_debit > Decimal::ZERO
    }

    pub fn validate_balance(&self) -> Result<(), String> {
        if self.lines.is_empty() {
            if self.amount <= Decimal::ZERO {
                return Err("Transaction amount must be strictly greater than zero".into());
            }
            return Ok(());
        }
        let total_debit: Decimal = self.lines.iter().map(|l| l.debit_amount).sum();
        let total_credit: Decimal = self.lines.iter().map(|l| l.credit_amount).sum();
        if total_debit != total_credit {
            return Err(format!(
                "Unbalanced journal entry: Total Debit ({}) != Total Credit ({})",
                total_debit, total_credit
            ));
        }
        Ok(())
    }
}
