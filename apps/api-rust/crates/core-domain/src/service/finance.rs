use uuid::Uuid;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use crate::entities::{ledger_account, treasury, student_fee, payment, fee_template};

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct CreateLedgerAccountRequest {
    pub code: String,
    pub name: String,
    pub name_ar: Option<String>,
    pub item_type: String, // e.g., asset, liability, equity, revenue, expense
    pub sub_type: Option<String>,
    pub parent_id: Option<String>,
    pub currency: String,
    pub opening_balance: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct CreateTreasuryRequest {
    pub name: String,
    pub name_ar: Option<String>,
    pub item_type: String, // bank, cash
    pub ledger_account_id: String,
    pub bank_name: Option<String>,
    pub account_number: Option<String>,
    pub currency: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct AssignStudentFeeRequest {
    pub enrollment_id: String,
    pub fee_template_id: String,
    pub amount: f64,
    pub due_date: String, // YYYY-MM-DD
    pub notes: Option<String>,
}

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

impl super::Service {
    // ── Ledger Accounts ──
    pub async fn get_ledger_accounts(&self, org_id: &str) -> Result<Vec<ledger_account::Model>, String> {
        self.repo.get_ledger_accounts(org_id).await.map_err(|e| e.to_string())
    }

    pub async fn create_ledger_account(&self, org_id: &str, req: CreateLedgerAccountRequest) -> Result<ledger_account::Model, String> {
        let la = ledger_account::Model {
            id: Uuid::new_v4().to_string(),
            org_id: org_id.to_string(),
            code: req.code,
            name: req.name,
            name_ar: req.name_ar,
            item_type: req.item_type,
            sub_type: req.sub_type,
            parent_id: req.parent_id,
            path: "".to_string(), // leaf path
            depth: 0,
            currency: req.currency,
            opening_balance: req.opening_balance,
            current_balance: req.opening_balance.or(Some(0.0)),
            is_bank_account: 0,
            is_cash_account: 0,
            is_control_account: 0,
            external_ref: None,
            is_active: 1,
            created_at: Utc::now().to_rfc3339(),
            updated_at: Utc::now().to_rfc3339(),
        };
        self.repo.create_ledger_account(la).await.map_err(|e| e.to_string())
    }

    // ── Treasuries ──
    pub async fn get_treasuries(&self, org_id: &str) -> Result<Vec<treasury::Model>, String> {
        self.repo.get_treasuries(org_id).await.map_err(|e| e.to_string())
    }

    pub async fn create_treasury(&self, org_id: &str, req: CreateTreasuryRequest) -> Result<treasury::Model, String> {
        let t = treasury::Model {
            id: Uuid::new_v4().to_string(),
            org_id: org_id.to_string(),
            unit_id: None,
            name: req.name,
            name_ar: req.name_ar,
            code: None,
            item_type: req.item_type,
            ledger_account_id: req.ledger_account_id,
            bank_name: req.bank_name,
            bank_branch: None,
            account_number: req.account_number,
            account_name: None,
            iban: None,
            swift_code: None,
            opening_balance: Some(0.0),
            current_balance: Some(0.0),
            currency: req.currency,
            authorized_users: None,
            is_default: 0,
            is_active: 1,
            created_at: Utc::now().to_rfc3339(),
            updated_at: Utc::now().to_rfc3339(),
        };
        self.repo.create_treasury(t).await.map_err(|e| e.to_string())
    }

    pub async fn get_active_fee_templates(&self, org_id: &str) -> Result<Vec<fee_template::Model>, String> {
        self.repo.get_active_fee_templates(org_id).await.map_err(|e| e.to_string())
    }

    // ── Student Fees ──
    pub async fn get_student_fees(&self, org_id: &str, enrollment_id: Option<&str>) -> Result<Vec<student_fee::Model>, String> {
        self.repo.get_student_fees(org_id, enrollment_id).await.map_err(|e| e.to_string())
    }

    pub async fn assign_student_fee(&self, org_id: &str, req: AssignStudentFeeRequest) -> Result<student_fee::Model, String> {
        // Find enrollment to get cycle_id
        let enrollment = self.repo.get_enrollment_by_id(&req.enrollment_id).await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "Enrollment not found".to_string())?;

        let sf = student_fee::Model {
            id: Uuid::new_v4().to_string(),
            org_id: org_id.to_string(),
            cycle_id: enrollment.cycle_id,
            enrollment_id: req.enrollment_id,
            fee_template_id: req.fee_template_id,
            original_amount: req.amount,
            discount_amount: 0.0,
            late_fee_amount: 0.0,
            total_amount: req.amount,
            paid_amount: 0.0,
            balance: req.amount,
            applied_discounts: None,
            payment_plan_id: None,
            status: "unpaid".to_string(),
            due_date: req.due_date,
            is_overdue: 0,
            overdue_since: None,
            overdue_days: None,
            is_installment: 0,
            parent_fee_id: None,
            installment_number: None,
            total_installments: None,
            reminder_sent: Some(0),
            reminder_sent_at: None,
            final_notice_sent: Some(0),
            final_notice_sent_at: None,
            is_locked: 0,
            locked_at: None,
            locked_by: None,
            notes: req.notes,
            meta: None,
            created_at: Utc::now().to_rfc3339(),
            updated_at: Utc::now().to_rfc3339(),
        };
        self.repo.create_student_fee(sf).await.map_err(|e| e.to_string())
    }

    // ── Payments ──
    pub async fn get_payments(&self, org_id: &str, enrollment_id: Option<&str>) -> Result<Vec<payment::Model>, String> {
        self.repo.get_payments(org_id, enrollment_id).await.map_err(|e| e.to_string())
    }

    pub async fn record_payment(&self, org_id: &str, req: RecordPaymentRequest, created_by: &str) -> Result<payment::Model, String> {
        // 1. Get student fee to verify existence and check balance
        let fee = self.repo.get_student_fee_by_id(&req.fee_id).await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "Student fee not found".to_string())?;

        if fee.balance <= 0.0 {
            return Err("This fee is already fully paid".to_string());
        }

        if req.amount <= 0.0 {
            return Err("Payment amount must be greater than zero".to_string());
        }

        // Determine status of the fee after this payment
        let remaining = fee.balance - req.amount;
        let new_status = if remaining <= 0.0 {
            "paid".to_string()
        } else {
            "partially_paid".to_string()
        };

        // Determine currency of the payment
        let payment_currency = req.currency.clone().unwrap_or_else(|| "YER".to_string());

        // 2. Record the payment
        let p = payment::Model {
            id: Uuid::new_v4().to_string(),
            org_id: org_id.to_string(),
            fee_id: req.fee_id.clone(),
            enrollment_id: fee.enrollment_id.clone(),
            amount: req.amount,
            currency: payment_currency,
            exchange_rate: Some(1.0),
            method: req.method,
            reference: req.reference,
            transaction_id: None,
            receipt_number: Some(format!("REC-{}", Utc::now().timestamp_millis())),
            journal_entry_id: None,
            treasury_id: req.treasury_id.clone(),
            payer_name: req.payer_name,
            payer_phone: req.payer_phone,
            payer_relation: None,
            payer_id: None,
            is_verified: 1,
            verified_by: Some(created_by.to_string()),
            verified_at: Some(Utc::now().to_rfc3339()),
            is_refund: 0,
            refund_amount: None,
            refund_reason: None,
            refund_approved_by: None,
            receipt_url: None,
            proof_url: None,
            reconciled: Some(0),
            reconciled_at: None,
            reconciled_by: None,
            notes: req.notes,
            meta: None,
            paid_at: Utc::now().to_rfc3339(),
            created_at: Utc::now().to_rfc3339(),
            created_by: Some(created_by.to_string()),
        };

        let payment_record = self.repo.create_payment(p).await.map_err(|e| e.to_string())?;

        // 3. Update the student fee balance
        let _ = self.repo.update_student_fee_balance(&req.fee_id, req.amount, &new_status).await
            .map_err(|e| e.to_string())?;

        Ok(payment_record)
    }
}
