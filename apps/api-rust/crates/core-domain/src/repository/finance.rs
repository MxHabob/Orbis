use crate::entities::{ledger_account, treasury, student_fee, payment, fee_template, journal_entry, discount_template, treasury_transaction};
use sea_orm::{entity::prelude::*, ActiveModelTrait, ActiveValue::Set, QueryFilter, EntityTrait, QueryOrder};

impl super::Repository {
    // ── Ledger Accounts ──
    pub async fn get_ledger_accounts(&self, org_id: &str) -> Result<Vec<ledger_account::Model>, DbErr> {
        ledger_account::Entity::find()
            .filter(ledger_account::Column::OrgId.eq(org_id))
            .order_by_asc(ledger_account::Column::Code)
            .all(&self.db)
            .await
    }

    pub async fn create_ledger_account(&self, la: ledger_account::Model) -> Result<ledger_account::Model, DbErr> {
        let active = ledger_account::ActiveModel {
            id: Set(la.id),
            org_id: Set(la.org_id),
            code: Set(la.code),
            name: Set(la.name),
            name_ar: Set(la.name_ar),
            item_type: Set(la.item_type),
            sub_type: Set(la.sub_type),
            parent_id: Set(la.parent_id),
            path: Set(la.path),
            depth: Set(la.depth),
            currency: Set(la.currency),
            opening_balance: Set(la.opening_balance),
            current_balance: Set(la.current_balance),
            is_bank_account: Set(la.is_bank_account),
            is_cash_account: Set(la.is_cash_account),
            is_control_account: Set(la.is_control_account),
            external_ref: Set(la.external_ref),
            is_active: Set(la.is_active),
            created_at: Set(la.created_at),
            updated_at: Set(la.updated_at),
        };
        active.insert(&self.db).await
    }

    // ── Treasuries ──
    pub async fn get_treasuries(&self, org_id: &str) -> Result<Vec<treasury::Model>, DbErr> {
        treasury::Entity::find()
            .filter(treasury::Column::OrgId.eq(org_id))
            .all(&self.db)
            .await
    }

    pub async fn get_treasury_by_id(&self, id: &str) -> Result<Option<treasury::Model>, DbErr> {
        treasury::Entity::find_by_id(id.to_string())
            .one(&self.db)
            .await
    }

    pub async fn create_treasury(&self, t: treasury::Model) -> Result<treasury::Model, DbErr> {
        let active = treasury::ActiveModel {
            id: Set(t.id),
            org_id: Set(t.org_id),
            unit_id: Set(t.unit_id),
            name: Set(t.name),
            name_ar: Set(t.name_ar),
            code: Set(t.code),
            item_type: Set(t.item_type),
            ledger_account_id: Set(t.ledger_account_id),
            bank_name: Set(t.bank_name),
            bank_branch: Set(t.bank_branch),
            account_number: Set(t.account_number),
            account_name: Set(t.account_name),
            iban: Set(t.iban),
            swift_code: Set(t.swift_code),
            opening_balance: Set(t.opening_balance),
            current_balance: Set(t.current_balance),
            currency: Set(t.currency),
            authorized_users: Set(t.authorized_users),
            is_default: Set(t.is_default),
            is_active: Set(t.is_active),
            created_at: Set(t.created_at),
            updated_at: Set(t.updated_at),
        };
        active.insert(&self.db).await
    }

    // ── Student Fees ──
    pub async fn get_student_fees(&self, org_id: &str, enrollment_id: Option<&str>) -> Result<Vec<student_fee::Model>, DbErr> {
        let mut query = student_fee::Entity::find()
            .filter(student_fee::Column::OrgId.eq(org_id));
        if let Some(enroll_id) = enrollment_id {
            query = query.filter(student_fee::Column::EnrollmentId.eq(enroll_id));
        }
        query.all(&self.db).await
    }

    pub async fn get_student_fee_by_id(&self, id: &str) -> Result<Option<student_fee::Model>, DbErr> {
        student_fee::Entity::find_by_id(id.to_string())
            .one(&self.db)
            .await
    }

    pub async fn create_student_fee(&self, sf: student_fee::Model) -> Result<student_fee::Model, DbErr> {
        let active = student_fee::ActiveModel {
            id: Set(sf.id),
            org_id: Set(sf.org_id),
            cycle_id: Set(sf.cycle_id),
            enrollment_id: Set(sf.enrollment_id),
            fee_template_id: Set(sf.fee_template_id),
            original_amount: Set(sf.original_amount),
            discount_amount: Set(sf.discount_amount),
            late_fee_amount: Set(sf.late_fee_amount),
            total_amount: Set(sf.total_amount),
            paid_amount: Set(sf.paid_amount),
            balance: Set(sf.balance),
            applied_discounts: Set(sf.applied_discounts),
            payment_plan_id: Set(sf.payment_plan_id),
            status: Set(sf.status),
            due_date: Set(sf.due_date),
            is_overdue: Set(sf.is_overdue),
            overdue_since: Set(sf.overdue_since),
            overdue_days: Set(sf.overdue_days),
            is_installment: Set(sf.is_installment),
            parent_fee_id: Set(sf.parent_fee_id),
            installment_number: Set(sf.installment_number),
            total_installments: Set(sf.total_installments),
            reminder_sent: Set(sf.reminder_sent),
            reminder_sent_at: Set(sf.reminder_sent_at),
            final_notice_sent: Set(sf.final_notice_sent),
            final_notice_sent_at: Set(sf.final_notice_sent_at),
            is_locked: Set(sf.is_locked),
            locked_at: Set(sf.locked_at),
            locked_by: Set(sf.locked_by),
            notes: Set(sf.notes),
            meta: Set(sf.meta),
            created_at: Set(sf.created_at),
            updated_at: Set(sf.updated_at),
        };
        active.insert(&self.db).await
    }

    pub async fn update_student_fee_balance(&self, id: &str, paid_amount: f64, status: &str) -> Result<student_fee::Model, DbErr> {
        let fee = student_fee::Entity::find_by_id(id.to_string())
            .one(&self.db)
            .await?
            .ok_or(DbErr::RecordNotFound("student fee not found".to_string()))?;
        
        let new_paid = fee.paid_amount + paid_amount;
        let new_balance = fee.total_amount - new_paid;

        let mut active: student_fee::ActiveModel = fee.into();
        active.paid_amount = Set(new_paid);
        active.balance = Set(new_balance);
        active.status = Set(status.to_string());
        active.updated_at = Set(chrono::Utc::now().to_rfc3339());

        active.update(&self.db).await
    }

    // ── Payments ──
    pub async fn get_payments(&self, org_id: &str, enrollment_id: Option<&str>) -> Result<Vec<payment::Model>, DbErr> {
        let mut query = payment::Entity::find()
            .filter(payment::Column::OrgId.eq(org_id));
        if let Some(enroll_id) = enrollment_id {
            query = query.filter(payment::Column::EnrollmentId.eq(enroll_id));
        }
        query.all(&self.db).await
    }

    pub async fn create_payment(&self, p: payment::Model) -> Result<payment::Model, DbErr> {
        let active = payment::ActiveModel {
            id: Set(p.id),
            org_id: Set(p.org_id),
            fee_id: Set(p.fee_id),
            enrollment_id: Set(p.enrollment_id),
            amount: Set(p.amount),
            currency: Set(p.currency),
            exchange_rate: Set(p.exchange_rate),
            method: Set(p.method),
            reference: Set(p.reference),
            transaction_id: Set(p.transaction_id),
            receipt_number: Set(p.receipt_number),
            journal_entry_id: Set(p.journal_entry_id),
            treasury_id: Set(p.treasury_id),
            payer_name: Set(p.payer_name),
            payer_phone: Set(p.payer_phone),
            payer_relation: Set(p.payer_relation),
            payer_id: Set(p.payer_id),
            is_verified: Set(p.is_verified),
            verified_by: Set(p.verified_by),
            verified_at: Set(p.verified_at),
            is_refund: Set(p.is_refund),
            refund_amount: Set(p.refund_amount),
            refund_reason: Set(p.refund_reason),
            refund_approved_by: Set(p.refund_approved_by),
            receipt_url: Set(p.receipt_url),
            proof_url: Set(p.proof_url),
            reconciled: Set(p.reconciled),
            reconciled_at: Set(p.reconciled_at),
            reconciled_by: Set(p.reconciled_by),
            notes: Set(p.notes),
            meta: Set(p.meta),
            paid_at: Set(p.paid_at),
            created_at: Set(p.created_at),
            created_by: Set(p.created_by),
        };
        active.insert(&self.db).await
    }

    // ── Fee Templates ──
    pub async fn get_active_fee_templates(&self, org_id: &str) -> Result<Vec<fee_template::Model>, DbErr> {
        fee_template::Entity::find()
            .filter(fee_template::Column::OrgId.eq(org_id))
            .filter(fee_template::Column::IsActive.eq(1))
            .all(&self.db)
            .await
    }

    pub async fn get_fee_templates(&self, org_id: &str) -> Result<Vec<fee_template::Model>, DbErr> {
        fee_template::Entity::find()
            .filter(fee_template::Column::OrgId.eq(org_id))
            .all(&self.db)
            .await
    }

    pub async fn get_fee_template_by_id(&self, id: &str) -> Result<Option<fee_template::Model>, DbErr> {
        fee_template::Entity::find_by_id(id.to_string())
            .one(&self.db)
            .await
    }

    pub async fn create_fee_template(&self, tpl: fee_template::Model) -> Result<fee_template::Model, DbErr> {
        let active = fee_template::ActiveModel {
            id: Set(tpl.id),
            org_id: Set(tpl.org_id),
            name: Set(tpl.name),
            name_ar: Set(tpl.name_ar),
            code: Set(tpl.code),
            category: Set(tpl.category),
            revenue_account_id: Set(tpl.revenue_account_id),
            receivable_account_id: Set(tpl.receivable_account_id),
            pricing_type: Set(tpl.pricing_type),
            pricing_config: Set(tpl.pricing_config),
            applicability_rules: Set(tpl.applicability_rules),
            frequency: Set(tpl.frequency),
            frequency_config: Set(tpl.frequency_config),
            late_fee_rules: Set(tpl.late_fee_rules),
            allowed_discounts: Set(tpl.allowed_discounts),
            is_mandatory: Set(tpl.is_mandatory),
            is_active: Set(tpl.is_active),
            sort_order: Set(tpl.sort_order),
            created_at: Set(tpl.created_at),
            updated_at: Set(tpl.updated_at),
        };
        active.insert(&self.db).await
    }

    pub async fn update_fee_template(&self, tpl: fee_template::Model) -> Result<fee_template::Model, DbErr> {
        let active = fee_template::ActiveModel {
            id: Set(tpl.id),
            org_id: Set(tpl.org_id),
            name: Set(tpl.name),
            name_ar: Set(tpl.name_ar),
            code: Set(tpl.code),
            category: Set(tpl.category),
            revenue_account_id: Set(tpl.revenue_account_id),
            receivable_account_id: Set(tpl.receivable_account_id),
            pricing_type: Set(tpl.pricing_type),
            pricing_config: Set(tpl.pricing_config),
            applicability_rules: Set(tpl.applicability_rules),
            frequency: Set(tpl.frequency),
            frequency_config: Set(tpl.frequency_config),
            late_fee_rules: Set(tpl.late_fee_rules),
            allowed_discounts: Set(tpl.allowed_discounts),
            is_mandatory: Set(tpl.is_mandatory),
            is_active: Set(tpl.is_active),
            sort_order: Set(tpl.sort_order),
            created_at: Set(tpl.created_at),
            updated_at: Set(tpl.updated_at),
        };
        active.update(&self.db).await
    }

    pub async fn delete_fee_template(&self, org_id: &str, id: &str) -> Result<u64, DbErr> {
        let result = fee_template::Entity::delete_by_id(id.to_string())
            .filter(fee_template::Column::OrgId.eq(org_id))
            .exec(&self.db)
            .await?;
        Ok(result.rows_affected)
    }

    // ── Journal Entries ──
    pub async fn create_journal_entry(&self, je: journal_entry::Model) -> Result<journal_entry::Model, DbErr> {
        let active = journal_entry::ActiveModel {
            id: Set(je.id),
            org_id: Set(je.org_id),
            transaction_ref: Set(je.transaction_ref),
            accounting_date: Set(je.accounting_date),
            fiscal_period_id: Set(je.fiscal_period_id),
            amount: Set(je.amount),
            debit_account_id: Set(je.debit_account_id),
            credit_account_id: Set(je.credit_account_id),
            currency: Set(je.currency),
            exchange_rate: Set(je.exchange_rate),
            base_currency_amount: Set(je.base_currency_amount),
            source_type: Set(je.source_type),
            source_id: Set(je.source_id),
            source_module: Set(je.source_module),
            description: Set(je.description),
            description_ar: Set(je.description_ar),
            metadata: Set(je.metadata),
            is_reversed: Set(je.is_reversed),
            reversed_by: Set(je.reversed_by),
            reversal_reason: Set(je.reversal_reason),
            is_reconciled: Set(je.is_reconciled),
            reconciled_at: Set(je.reconciled_at),
            reconciliation_ref: Set(je.reconciliation_ref),
            status: Set(je.status),
            posted_by: Set(je.posted_by),
            posted_at: Set(je.posted_at),
            approved_by: Set(je.approved_by),
            approved_at: Set(je.approved_at),
            created_by: Set(je.created_by),
            created_at: Set(je.created_at),
        };
        active.insert(&self.db).await
    }

    pub async fn get_journal_entries(&self, org_id: &str) -> Result<Vec<journal_entry::Model>, DbErr> {
        journal_entry::Entity::find()
            .filter(journal_entry::Column::OrgId.eq(org_id))
            .order_by_desc(journal_entry::Column::AccountingDate)
            .all(&self.db)
            .await
    }

    // ── Discount Templates ──
    pub async fn get_active_discount_templates(&self, org_id: &str) -> Result<Vec<discount_template::Model>, DbErr> {
        discount_template::Entity::find()
            .filter(discount_template::Column::OrgId.eq(org_id))
            .filter(discount_template::Column::IsActive.eq(1))
            .all(&self.db)
            .await
    }

    pub async fn get_discount_templates(&self, org_id: &str) -> Result<Vec<discount_template::Model>, DbErr> {
        discount_template::Entity::find()
            .filter(discount_template::Column::OrgId.eq(org_id))
            .all(&self.db)
            .await
    }

    pub async fn get_discount_template_by_id(&self, id: &str) -> Result<Option<discount_template::Model>, DbErr> {
        discount_template::Entity::find_by_id(id.to_string())
            .one(&self.db)
            .await
    }

    pub async fn create_discount_template(&self, tpl: discount_template::Model) -> Result<discount_template::Model, DbErr> {
        let active = discount_template::ActiveModel {
            id: Set(tpl.id),
            org_id: Set(tpl.org_id),
            name: Set(tpl.name),
            code: Set(tpl.code),
            item_type: Set(tpl.item_type),
            calculation_method: Set(tpl.calculation_method),
            value: Set(tpl.value),
            eligibility_rules: Set(tpl.eligibility_rules),
            limits: Set(tpl.limits),
            requires_approval: Set(tpl.requires_approval),
            approver_roles: Set(tpl.approver_roles),
            is_auto_apply: Set(tpl.is_auto_apply),
            is_stackable: Set(tpl.is_stackable),
            priority: Set(tpl.priority),
            is_active: Set(tpl.is_active),
            created_at: Set(tpl.created_at),
            updated_at: Set(tpl.updated_at),
        };
        active.insert(&self.db).await
    }

    pub async fn update_discount_template(&self, tpl: discount_template::Model) -> Result<discount_template::Model, DbErr> {
        let active = discount_template::ActiveModel {
            id: Set(tpl.id),
            org_id: Set(tpl.org_id),
            name: Set(tpl.name),
            code: Set(tpl.code),
            item_type: Set(tpl.item_type),
            calculation_method: Set(tpl.calculation_method),
            value: Set(tpl.value),
            eligibility_rules: Set(tpl.eligibility_rules),
            limits: Set(tpl.limits),
            requires_approval: Set(tpl.requires_approval),
            approver_roles: Set(tpl.approver_roles),
            is_auto_apply: Set(tpl.is_auto_apply),
            is_stackable: Set(tpl.is_stackable),
            priority: Set(tpl.priority),
            is_active: Set(tpl.is_active),
            created_at: Set(tpl.created_at),
            updated_at: Set(tpl.updated_at),
        };
        active.update(&self.db).await
    }

    pub async fn delete_discount_template(&self, org_id: &str, id: &str) -> Result<u64, DbErr> {
        let result = discount_template::Entity::delete_by_id(id.to_string())
            .filter(discount_template::Column::OrgId.eq(org_id))
            .exec(&self.db)
            .await?;
        Ok(result.rows_affected)
    }

    pub async fn update_treasury_balance(&self, treasury_id: &str, amount: f64) -> Result<(), DbErr> {
        let t = treasury::Entity::find_by_id(treasury_id.to_string())
            .one(&self.db)
            .await?;
        if let Some(t) = t {
            let mut active: treasury::ActiveModel = t.into();
            let current = active.current_balance.unwrap_or(Some(0.0)).unwrap_or(0.0);
            active.current_balance = Set(Some(current + amount));
            active.updated_at = Set(chrono::Utc::now().to_rfc3339());
            active.update(&self.db).await?;
        }
        Ok(())
    }

    pub async fn create_treasury_transaction(&self, tx: treasury_transaction::Model) -> Result<treasury_transaction::Model, DbErr> {
        let active = treasury_transaction::ActiveModel {
            id: Set(tx.id),
            org_id: Set(tx.org_id),
            treasury_id: Set(tx.treasury_id),
            journal_entry_id: Set(tx.journal_entry_id),
            item_type: Set(tx.item_type),
            category: Set(tx.category),
            amount: Set(tx.amount),
            currency: Set(tx.currency),
            exchange_rate: Set(tx.exchange_rate),
            reference_type: Set(tx.reference_type),
            reference_id: Set(tx.reference_id),
            description: Set(tx.description),
            notes: Set(tx.notes),
            attachments: Set(tx.attachments),
            created_by: Set(tx.created_by),
            approved_by: Set(tx.approved_by),
            approved_at: Set(tx.approved_at),
            transaction_date: Set(tx.transaction_date),
            created_at: Set(tx.created_at),
        };
        active.insert(&self.db).await
    }
}
