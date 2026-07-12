//! Payment service — record fee receipts, posts cash inflows, updates accounts and fee statuses.

use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sea_orm::EntityTrait;
use svc_academic::entities::enrollment;

use shared_kernel::errors::AppError;
use crate::entities::{payment, journal_entry, treasury_transaction};
use crate::params::PaymentParams;
use crate::repository::{FinanceCommandRepo, FinanceQueryRepo};
use crate::services::ledger::LedgerService;



#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct RecordPaymentRequest {
    pub fee_id: String,
    pub amount: f64,
    pub method: String, // cash, bank_transfer, card
    pub reference: Option<String>,
    pub notes: Option<String>,
    pub payer_name: Option<String>,
    pub payer_phone: Option<String>,
    pub treasury_id: Option<String>,
    pub currency: Option<String>,
}

pub struct PaymentService {
    pub cmd: Arc<FinanceCommandRepo>,
    pub query: Arc<FinanceQueryRepo>,
    pub ledger_service: Arc<LedgerService>,
}

impl PaymentService {
    pub fn new(
        cmd: Arc<FinanceCommandRepo>,
        query: Arc<FinanceQueryRepo>,
        ledger_service: Arc<LedgerService>,
    ) -> Self {
        Self {
            cmd,
            query,
            ledger_service,
        }
    }

    pub async fn get_payments(&self, org_id: &str, enrollment_id: Option<&str>) -> Result<Vec<payment::Model>, AppError> {
        let params = PaymentParams { enrollment_id: enrollment_id.map(String::from), fee_id: None, treasury_id: None };
        Ok(self.query.get_payments(org_id, params).await?)
    }

    /// Record a payment received, post double-entry receipts, and transition student fee payment status.
    pub async fn record_payment(&self, org_id: &str, req: RecordPaymentRequest, created_by: &str) -> Result<payment::Model, AppError> {
        let fee = self.query.get_student_fee_by_id(&req.fee_id).await?
            .ok_or_else(|| AppError::not_found("student_fee", &req.fee_id))?;

        let enrollment = enrollment::Entity::find_by_id(fee.enrollment_id.clone())
            .one(&self.query.db)
            .await?
            .ok_or_else(|| AppError::not_found("enrollment", &fee.enrollment_id))?;

        let student_account = match self.query.get_ledger_account_by_entity(org_id, "student", &enrollment.student_id).await? {
            Some(acc) => acc,
            None => {
                self.ledger_service.get_or_create_sub_account(org_id, "student", &enrollment.student_id, None, None).await?
            }
        };

        let treasury_id = req.treasury_id.clone()
            .ok_or_else(|| AppError::validation("Treasury ID is required"))?;
        let treasury = self.query.get_treasury_by_id(&treasury_id).await?
            .ok_or_else(|| AppError::not_found("treasury", &treasury_id))?;

        if req.amount <= 0.0 {
            return Err(AppError::validation("Payment amount must be greater than zero"));
        }

        let payment_currency = req.currency.clone().unwrap_or_else(|| student_account.currency.clone());
        let exchange_rate = if payment_currency == student_account.currency {
            1.0
        } else {
            let cur = self.query.get_currency_by_code(org_id, &payment_currency).await?
                .ok_or_else(|| AppError::validation(format!("Currency {} not supported", payment_currency)))?;
            cur.exchange_rate
        };

        let base_amount = req.amount * exchange_rate;
        let payment_id = Uuid::new_v4().to_string();
        let receipt_number = format!("REC-{}", Utc::now().timestamp_millis());
        let journal_entry_id = Uuid::new_v4().to_string();

        let p = payment::Model {
            id: payment_id.clone(),
            org_id: org_id.to_string(),
            fee_id: Some(req.fee_id.clone()),
            enrollment_id: Some(fee.enrollment_id.clone()),
            account_id: student_account.id.clone(),
            amount: req.amount,
            currency: payment_currency.clone(),
            exchange_rate,
            base_amount,
            method: req.method.clone(),
            reference: req.reference.clone(),
            receipt_number: Some(receipt_number.clone()),
            journal_entry_id: Some(journal_entry_id.clone()),
            treasury_id: Some(treasury_id.clone()),
            payer_name: req.payer_name.clone(),
            payer_phone: req.payer_phone.clone(),
            payment_type: "receipt".to_string(),
            related_payment_id: None,
            notes: req.notes.clone(),
            paid_at: Utc::now().to_rfc3339(),
            created_at: Utc::now().to_rfc3339(),
            created_by: Some(created_by.to_string()),
        };

        let payment_record = self.cmd.create_payment(p).await?;

        let description = format!("Fee payment received via {}: {}", req.method, receipt_number);
        let description_ar = Some(format!("سداد رسوم عبر {}: {}", req.method, receipt_number));

        let je = journal_entry::Model {
            id: journal_entry_id.clone(),
            org_id: org_id.to_string(),
            transaction_ref: receipt_number,
            accounting_date: Utc::now().format("%Y-%m-%d").to_string(),
            fiscal_period_id: fee.period_id.clone(),
            amount: req.amount,
            debit_account_id: treasury.ledger_account_id.clone(),
            credit_account_id: student_account.id.clone(),
            currency: payment_currency.clone(),
            exchange_rate: Some(exchange_rate),
            base_currency_amount: Some(base_amount),
            source_type: "payment".to_string(),
            source_id: Some(payment_id.clone()),
            source_module: Some("finance".to_string()),
            description,
            description_ar,
            metadata: None,
            is_reversed: 0,
            reversed_by: None,
            reversal_reason: None,
            is_reconciled: 0,
            reconciled_at: None,
            reconciliation_ref: None,
            status: "posted".to_string(),
            posted_by: None,
            posted_at: Some(Utc::now().to_rfc3339()),
            approved_by: None,
            approved_at: Some(Utc::now().to_rfc3339()),
            created_by: Some(created_by.to_string()),
            created_at: Utc::now().to_rfc3339(),
        };

        self.cmd.create_journal_entry(je).await?;

        let tx = treasury_transaction::Model {
            id: Uuid::new_v4().to_string(),
            org_id: org_id.to_string(),
            treasury_id: treasury_id.clone(),
            journal_entry_id: journal_entry_id.clone(),
            item_type: "inflow".to_string(),
            category: "tuition".to_string(),
            amount: req.amount,
            currency: payment_currency.clone(),
            exchange_rate: Some(exchange_rate),
            reference_type: Some("payment".to_string()),
            reference_id: Some(payment_id),
            description: "Tuition payment for student fee".to_string(),
            notes: req.notes.clone(),
            attachments: None,
            created_by: Some(created_by.to_string()),
            approved_by: Some(created_by.to_string()),
            approved_at: Some(Utc::now().to_rfc3339()),
            transaction_date: Utc::now().format("%Y-%m-%d").to_string(),
            created_at: Utc::now().to_rfc3339(),
        };
        self.cmd.create_treasury_transaction(tx).await?;

        self.cmd.update_treasury_balance(&treasury_id, req.amount).await?;
        self.cmd.update_ledger_account_balance(&treasury.ledger_account_id, base_amount).await?;
        self.cmd.update_ledger_account_balance(&student_account.id, -base_amount).await?;

        let payments = self.query.get_payments(org_id, PaymentParams { enrollment_id: Some(fee.enrollment_id.clone()), fee_id: None, treasury_id: None }).await?;
        let total_paid: f64 = payments.iter()
            .filter(|p| p.fee_id.as_ref() == Some(&fee.id))
            .map(|p| p.base_amount)
            .sum::<f64>();

        let new_status = if total_paid >= fee.net_amount {
            "paid".to_string()
        } else if total_paid > 0.0 {
            "partially_paid".to_string()
        } else {
            "unpaid".to_string()
        };

        self.cmd.update_student_fee_status(&fee.id, &new_status).await?;
        Ok(payment_record)
    }
}
