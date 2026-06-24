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
    pub default_amount: f64,
    pub revenue_account_id: String,
    pub receivable_account_id: String,
    pub applicable_levels: Option<String>,
    pub frequency: String,
    pub installment_config: Option<String>,
    pub late_fee_rules: Option<String>,
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
            entity_type: None,
            entity_id: None,
            is_control_account: 0,
            is_system_account: 0,
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
            default_amount: req.default_amount,
            revenue_account_id: req.revenue_account_id,
            receivable_account_id: req.receivable_account_id,
            applicable_levels: req.applicable_levels,
            frequency: req.frequency,
            installment_config: req.installment_config,
            late_fee_rules: req.late_fee_rules,
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
        active.default_amount = sea_orm::ActiveValue::Set(req.default_amount);
        active.revenue_account_id = sea_orm::ActiveValue::Set(req.revenue_account_id);
        active.receivable_account_id = sea_orm::ActiveValue::Set(req.receivable_account_id);
        active.applicable_levels = sea_orm::ActiveValue::Set(req.applicable_levels);
        active.frequency = sea_orm::ActiveValue::Set(req.frequency);
        active.installment_config = sea_orm::ActiveValue::Set(req.installment_config);
        active.late_fee_rules = sea_orm::ActiveValue::Set(req.late_fee_rules);
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
        // Find enrollment to get cycle_id and student_id
        let enrollment = self.repo.get_enrollment_by_id(&req.enrollment_id).await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "Enrollment not found".to_string())?;

        // Find student sub-ledger account
        let student_account = self.repo.get_ledger_account_by_entity(org_id, "student", &enrollment.student_id).await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "Student ledger account not found. Please verify the student has a registered sub-ledger account.".to_string())?;

        // Find fee template to get revenue account
        let template = self.repo.get_fee_template_by_id(&req.fee_template_id).await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "Fee template not found".to_string())?;

        let fee_id = Uuid::new_v4().to_string();

        // Create the student fee record (invoice)
        let sf = student_fee::Model {
            id: fee_id.clone(),
            org_id: org_id.to_string(),
            cycle_id: enrollment.cycle_id.clone(),
            enrollment_id: req.enrollment_id.clone(),
            fee_template_id: req.fee_template_id.clone(),
            original_amount: req.amount,
            discount_amount: 0.0,
            net_amount: req.amount,
            status: "unpaid".to_string(),
            due_date: req.due_date,
            discount_details: None,
            parent_fee_id: None,
            installment_number: None,
            notes: req.notes.clone(),
            created_at: Utc::now().to_rfc3339(),
            updated_at: Utc::now().to_rfc3339(),
        };

        let created_fee = self.repo.create_student_fee(sf).await.map_err(|e| e.to_string())?;

        // Create Double-Entry Journal Entry:
        // Debit: Student Sub-account (increases asset / Accounts Receivable)
        // Credit: Tuition Revenue account (increases revenue)
        let je_id = Uuid::new_v4().to_string();
        let description = format!("Tuition fee charged: {}", template.name);
        let description_ar = Some(format!("قيد رسوم دراسية: {}", template.name_ar.unwrap_or(template.name.clone())));

        let je = crate::entities::journal_entry::Model {
            id: je_id,
            org_id: org_id.to_string(),
            transaction_ref: format!("INV-{}", &fee_id[..8].to_uppercase()),
            accounting_date: Utc::now().format("%Y-%m-%d").to_string(),
            fiscal_period_id: enrollment.cycle_id,
            amount: req.amount,
            debit_account_id: student_account.id.clone(),
            credit_account_id: template.revenue_account_id.clone(),
            currency: student_account.currency.clone(),
            exchange_rate: Some(1.0),
            base_currency_amount: Some(req.amount),
            source_type: "student_fee".to_string(),
            source_id: Some(fee_id),
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
            created_by: None,
            created_at: Utc::now().to_rfc3339(),
        };

        self.repo.create_journal_entry(je).await.map_err(|e| e.to_string())?;

        // Update cached ledger account balances
        // Debit increases Student Sub-account
        self.repo.update_ledger_account_balance(&student_account.id, req.amount).await.map_err(|e| e.to_string())?;
        // Credit increases Revenue (so we add the credit amount)
        self.repo.update_ledger_account_balance(&template.revenue_account_id, req.amount).await.map_err(|e| e.to_string())?;

        Ok(created_fee)
    }

    // ── Payments ──
    pub async fn get_payments(&self, org_id: &str, enrollment_id: Option<&str>) -> Result<Vec<payment::Model>, String> {
        self.repo.get_payments(org_id, enrollment_id).await.map_err(|e| e.to_string())
    }

    pub async fn record_payment(&self, org_id: &str, req: RecordPaymentRequest, created_by: &str) -> Result<payment::Model, String> {
        // 1. Get student fee (invoice)
        let fee = self.repo.get_student_fee_by_id(&req.fee_id).await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "Student fee not found".to_string())?;

        // Find enrollment to get student_id
        let enrollment = self.repo.get_enrollment_by_id(&fee.enrollment_id).await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "Enrollment not found".to_string())?;

        // Find student sub-ledger account
        let student_account = self.repo.get_ledger_account_by_entity(org_id, "student", &enrollment.student_id).await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "Student ledger account not found".to_string())?;

        // Find treasury
        let treasury_id = req.treasury_id.clone()
            .ok_or_else(|| "Treasury ID is required".to_string())?;
        let treasury = self.repo.get_treasury_by_id(&treasury_id).await
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "Treasury not found".to_string())?;

        if req.amount <= 0.0 {
            return Err("Payment amount must be greater than zero".to_string());
        }

        // Get currency exchange rate if paying in a foreign currency
        let payment_currency = req.currency.clone().unwrap_or_else(|| student_account.currency.clone());
        let exchange_rate = if payment_currency == student_account.currency {
            1.0
        } else {
            let cur = self.repo.get_currency(org_id, &payment_currency).await
                .map_err(|e| e.to_string())?
                .ok_or_else(|| format!("Currency {} not supported", payment_currency))?;
            cur.exchange_rate
        };

        // Convert payment amount to student's billing currency (base amount)
        let base_amount = req.amount * exchange_rate;

        // 2. Create the payment record
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

        let payment_record = self.repo.create_payment(p).await.map_err(|e| e.to_string())?;

        // 3. Create Double-Entry Journal Entry:
        // Debit: Treasury's ledger account (increases asset)
        // Credit: Student's sub-ledger account (decreases accounts receivable asset)
        let description = format!("Fee payment received via {}: {}", req.method, receipt_number);
        let description_ar = Some(format!("سداد رسوم عبر {}: {}", req.method, receipt_number));

        let je = crate::entities::journal_entry::Model {
            id: journal_entry_id.clone(),
            org_id: org_id.to_string(),
            transaction_ref: receipt_number,
            accounting_date: Utc::now().format("%Y-%m-%d").to_string(),
            fiscal_period_id: fee.cycle_id.clone(),
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

        self.repo.create_journal_entry(je).await.map_err(|e| e.to_string())?;

        // 4. Create Treasury Transaction
        let tx = crate::entities::treasury_transaction::Model {
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
            description: format!("Tuition payment for student fee"),
            notes: req.notes.clone(),
            attachments: None,
            created_by: Some(created_by.to_string()),
            approved_by: Some(created_by.to_string()),
            approved_at: Some(Utc::now().to_rfc3339()),
            transaction_date: Utc::now().format("%Y-%m-%d").to_string(),
            created_at: Utc::now().to_rfc3339(),
        };
        self.repo.create_treasury_transaction(tx).await.map_err(|e| e.to_string())?;

        // 5. Update cached balances
        // Update Treasury cached balance
        self.repo.update_treasury_balance(&treasury_id, req.amount).await.map_err(|e| e.to_string())?;
        // Update Treasury's ledger account balance (Debit increases Asset)
        self.repo.update_ledger_account_balance(&treasury.ledger_account_id, base_amount).await.map_err(|e| e.to_string())?;
        // Update Student's ledger account balance (Credit decreases Student Receivable, so we subtract base_amount)
        self.repo.update_ledger_account_balance(&student_account.id, -base_amount).await.map_err(|e| e.to_string())?;

        // 6. Update student fee status (invoice)
        let payments = self.repo.get_payments(org_id, Some(&fee.enrollment_id)).await
            .map_err(|e| e.to_string())?;
        
        let total_paid: f64 = payments.iter()
            .filter(|p| p.fee_id.as_ref() == Some(&fee.id))
            .map(|p| p.base_amount)
            .sum::<f64>() + base_amount;

        let new_status = if total_paid >= fee.net_amount {
            "paid".to_string()
        } else if total_paid > 0.0 {
            "partially_paid".to_string()
        } else {
            "unpaid".to_string()
        };

        self.repo.update_student_fee_status(&fee.id, &new_status).await.map_err(|e| e.to_string())?;

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
