use uuid::Uuid;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sea_orm::ActiveModelTrait;
use crate::entities::{ledger_account, treasury, student_fee, payment, fee_template, discount_template};

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct CreateDiscountTemplateRequest {
    pub name: String,
    pub code: Option<String>,
    pub item_type: String, // mapped to 'type' in DB (e.g. system, manual, etc.)
    pub calculation_method: String, // percentage, fixed
    pub value: i32,
    pub eligibility_rules: Option<String>,
    pub limits: Option<String>,
    pub requires_approval: i32,
    pub approver_roles: Option<String>,
    pub is_auto_apply: i32,
    pub is_stackable: i32,
    pub priority: Option<i32>,
    pub is_active: i32,
}

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
pub struct CreateFeeTemplateRequest {
    pub name: String,
    pub name_ar: Option<String>,
    pub code: Option<String>,
    pub category: String,
    pub revenue_account_id: String,
    pub receivable_account_id: String,
    pub pricing_type: String,
    pub pricing_config: Option<String>,
    pub applicability_rules: Option<String>,
    pub frequency: String,
    pub frequency_config: Option<String>,
    pub late_fee_rules: Option<String>,
    pub allowed_discounts: Option<String>,
    pub is_mandatory: i32,
    pub is_active: i32,
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

    pub async fn get_fee_templates(&self, org_id: &str) -> Result<Vec<fee_template::Model>, String> {
        self.repo.get_fee_templates(org_id).await.map_err(|e| e.to_string())
    }

    pub async fn create_fee_template(&self, org_id: &str, req: CreateFeeTemplateRequest) -> Result<fee_template::Model, String> {
        let tpl = fee_template::Model {
            id: Uuid::new_v4().to_string(),
            org_id: org_id.to_string(),
            name: req.name,
            name_ar: req.name_ar,
            code: req.code,
            category: req.category,
            revenue_account_id: req.revenue_account_id,
            receivable_account_id: req.receivable_account_id,
            pricing_type: req.pricing_type,
            pricing_config: req.pricing_config,
            applicability_rules: req.applicability_rules,
            frequency: req.frequency,
            frequency_config: req.frequency_config,
            late_fee_rules: req.late_fee_rules,
            allowed_discounts: req.allowed_discounts,
            is_mandatory: req.is_mandatory,
            is_active: req.is_active,
            sort_order: None,
            created_at: Utc::now().to_rfc3339(),
            updated_at: Utc::now().to_rfc3339(),
        };
        self.repo.create_fee_template(tpl).await.map_err(|e| e.to_string())
    }

    pub async fn update_fee_template(&self, org_id: &str, id: &str, req: CreateFeeTemplateRequest) -> Result<fee_template::Model, String> {
        let tpl = self.repo.get_fee_template_by_id(id).await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "Fee template not found".to_string())?;

        if tpl.org_id != org_id {
            return Err("Unauthorized".to_string());
        }

        let mut active: fee_template::ActiveModel = tpl.into();
        active.name = sea_orm::ActiveValue::Set(req.name);
        active.name_ar = sea_orm::ActiveValue::Set(req.name_ar);
        active.code = sea_orm::ActiveValue::Set(req.code);
        active.category = sea_orm::ActiveValue::Set(req.category);
        active.revenue_account_id = sea_orm::ActiveValue::Set(req.revenue_account_id);
        active.receivable_account_id = sea_orm::ActiveValue::Set(req.receivable_account_id);
        active.pricing_type = sea_orm::ActiveValue::Set(req.pricing_type);
        active.pricing_config = sea_orm::ActiveValue::Set(req.pricing_config);
        active.applicability_rules = sea_orm::ActiveValue::Set(req.applicability_rules);
        active.frequency = sea_orm::ActiveValue::Set(req.frequency);
        active.frequency_config = sea_orm::ActiveValue::Set(req.frequency_config);
        active.late_fee_rules = sea_orm::ActiveValue::Set(req.late_fee_rules);
        active.allowed_discounts = sea_orm::ActiveValue::Set(req.allowed_discounts);
        active.is_mandatory = sea_orm::ActiveValue::Set(req.is_mandatory);
        active.is_active = sea_orm::ActiveValue::Set(req.is_active);
        active.updated_at = sea_orm::ActiveValue::Set(Utc::now().to_rfc3339());

        active.update(&self.repo.db).await.map_err(|e| e.to_string())
    }

    pub async fn delete_fee_template(&self, org_id: &str, id: &str) -> Result<u64, String> {
        self.repo.delete_fee_template(org_id, id).await.map_err(|e| e.to_string())
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

    pub async fn get_journal_entries(&self, org_id: &str) -> Result<Vec<crate::entities::journal_entry::Model>, String> {
        self.repo.get_journal_entries(org_id).await.map_err(|e| e.to_string())
    }

    // ── Discount Templates ──
    pub async fn get_active_discount_templates(&self, org_id: &str) -> Result<Vec<discount_template::Model>, String> {
        self.repo.get_active_discount_templates(org_id).await.map_err(|e| e.to_string())
    }

    pub async fn get_discount_templates(&self, org_id: &str) -> Result<Vec<discount_template::Model>, String> {
        self.repo.get_discount_templates(org_id).await.map_err(|e| e.to_string())
    }

    pub async fn create_discount_template(&self, org_id: &str, req: CreateDiscountTemplateRequest) -> Result<discount_template::Model, String> {
        let tpl = discount_template::Model {
            id: Uuid::new_v4().to_string(),
            org_id: org_id.to_string(),
            name: req.name,
            code: req.code,
            item_type: req.item_type,
            calculation_method: req.calculation_method,
            value: req.value,
            eligibility_rules: req.eligibility_rules,
            limits: req.limits,
            requires_approval: req.requires_approval,
            approver_roles: req.approver_roles,
            is_auto_apply: req.is_auto_apply,
            is_stackable: req.is_stackable,
            priority: req.priority,
            is_active: req.is_active,
            created_at: Utc::now().to_rfc3339(),
            updated_at: Utc::now().to_rfc3339(),
        };
        self.repo.create_discount_template(tpl).await.map_err(|e| e.to_string())
    }

    pub async fn update_discount_template(&self, org_id: &str, id: &str, req: CreateDiscountTemplateRequest) -> Result<discount_template::Model, String> {
        let tpl = self.repo.get_discount_template_by_id(id).await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "Discount template not found".to_string())?;

        if tpl.org_id != org_id {
            return Err("Unauthorized".to_string());
        }

        let mut active: discount_template::ActiveModel = tpl.into();
        active.name = sea_orm::ActiveValue::Set(req.name);
        active.code = sea_orm::ActiveValue::Set(req.code);
        active.item_type = sea_orm::ActiveValue::Set(req.item_type);
        active.calculation_method = sea_orm::ActiveValue::Set(req.calculation_method);
        active.value = sea_orm::ActiveValue::Set(req.value);
        active.eligibility_rules = sea_orm::ActiveValue::Set(req.eligibility_rules);
        active.limits = sea_orm::ActiveValue::Set(req.limits);
        active.requires_approval = sea_orm::ActiveValue::Set(req.requires_approval);
        active.approver_roles = sea_orm::ActiveValue::Set(req.approver_roles);
        active.is_auto_apply = sea_orm::ActiveValue::Set(req.is_auto_apply);
        active.is_stackable = sea_orm::ActiveValue::Set(req.is_stackable);
        active.priority = sea_orm::ActiveValue::Set(req.priority);
        active.is_active = sea_orm::ActiveValue::Set(req.is_active);
        active.updated_at = sea_orm::ActiveValue::Set(Utc::now().to_rfc3339());

        active.update(&self.repo.db).await.map_err(|e| e.to_string())
    }

    pub async fn delete_discount_template(&self, org_id: &str, id: &str) -> Result<u64, String> {
        self.repo.delete_discount_template(org_id, id).await.map_err(|e| e.to_string())
    }
}
