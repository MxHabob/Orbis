//! Finance command repository — write operations.

use crate::entities::{account, treasury, student_fee, payment, fee_template, journal_entry, discount_template, treasury_transaction};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DbErr, EntityTrait, ColumnTrait, QueryFilter};
use super::FinanceCommandRepo;

impl FinanceCommandRepo {
    // ── Ledger Accounts ──

    pub async fn create_account(&self, la: account::Model) -> Result<account::Model, DbErr> {
        let active = account::ActiveModel {
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
            balances: Set(la.balances),
            entity_type: Set(la.entity_type),
            entity_id: Set(la.entity_id),
            is_control_account: Set(la.is_control_account),
            is_system_account: Set(la.is_system_account),
            is_active: Set(la.is_active),
            created_at: Set(la.created_at),
            updated_at: Set(la.updated_at),
        };
        active.insert(&self.db).await
    }

    pub async fn update_account_balance(&self, account_id: &str, currency: &str, amount: f64) -> Result<(), DbErr> {
        let mut current_id = Some(account_id.to_string());
        while let Some(id) = current_id {
            let account = account::Entity::find_by_id(id)
                .one(&self.db)
                .await?;
            if let Some(account) = account {
                let mut cb: shared_kernel::currency::CurrencyBalance = serde_json::from_value(account.balances.clone()).unwrap_or_default();
                cb.add(currency, amount);

                let mut active: account::ActiveModel = account.into();
                active.balances = Set(serde_json::to_value(&cb).unwrap());
                active.updated_at = Set(chrono::Utc::now().to_rfc3339());
                let updated = active.update(&self.db).await?;
                current_id = updated.parent_id;
            } else {
                break;
            }
        }
        Ok(())
    }

    // ── Treasuries ──

    pub async fn create_treasury(&self, t: treasury::Model) -> Result<treasury::Model, DbErr> {
        let active = treasury::ActiveModel {
            id: Set(t.id),
            org_id: Set(t.org_id),
            unit_id: Set(t.unit_id),
            name: Set(t.name),
            name_ar: Set(t.name_ar),
            code: Set(t.code),
            item_type: Set(t.item_type),
            account_id: Set(t.account_id),
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

    pub async fn update_treasury_balance(&self, treasury_id: &str, amount: f64) -> Result<(), DbErr> {
        let t = treasury::Entity::find_by_id(treasury_id.to_string())
            .one(&self.db)
            .await?;
        if let Some(t) = t {
            let current = t.current_balance.unwrap_or(0.0);
            let mut active: treasury::ActiveModel = t.into();
            active.current_balance = Set(Some(current + amount));
            active.updated_at = Set(chrono::Utc::now().to_rfc3339());
            active.update(&self.db).await?;
        }
        Ok(())
    }

    // ── Student Fees ──

    pub async fn create_student_fee(&self, sf: student_fee::Model) -> Result<student_fee::Model, DbErr> {
        let active = student_fee::ActiveModel {
            id: Set(sf.id),
            org_id: Set(sf.org_id),
            period_id: Set(sf.period_id),
            enrollment_id: Set(sf.enrollment_id),
            fee_template_id: Set(sf.fee_template_id),
            original_amount: Set(sf.original_amount),
            discount_amount: Set(sf.discount_amount),
            net_amount: Set(sf.net_amount),
            status: Set(sf.status),
            due_date: Set(sf.due_date),
            discount_details: Set(sf.discount_details),
            parent_fee_id: Set(sf.parent_fee_id),
            installment_number: Set(sf.installment_number),
            notes: Set(sf.notes),
            created_at: Set(sf.created_at),
            updated_at: Set(sf.updated_at),
        };
        active.insert(&self.db).await
    }

    pub async fn update_student_fee_status(&self, id: &str, status: &str) -> Result<student_fee::Model, DbErr> {
        let fee = student_fee::Entity::find_by_id(id.to_string())
            .one(&self.db)
            .await?
            .ok_or(DbErr::RecordNotFound("student fee not found".to_string()))?;

        let mut active: student_fee::ActiveModel = fee.into();
        active.status = Set(status.to_string());
        active.updated_at = Set(chrono::Utc::now().to_rfc3339());

        active.update(&self.db).await
    }

    // ── Payments ──

    pub async fn create_payment(&self, p: payment::Model) -> Result<payment::Model, DbErr> {
        let active = payment::ActiveModel {
            id: Set(p.id),
            org_id: Set(p.org_id),
            fee_id: Set(p.fee_id),
            enrollment_id: Set(p.enrollment_id),
            account_id: Set(p.account_id),
            amount: Set(p.amount),
            currency: Set(p.currency),
            exchange_rate: Set(p.exchange_rate),
            base_amount: Set(p.base_amount),
            method: Set(p.method),
            reference: Set(p.reference),
            receipt_number: Set(p.receipt_number),
            journal_entry_id: Set(p.journal_entry_id),
            treasury_id: Set(p.treasury_id),
            payer_name: Set(p.payer_name),
            payer_phone: Set(p.payer_phone),
            payment_type: Set(p.payment_type),
            related_payment_id: Set(p.related_payment_id),
            notes: Set(p.notes),
            paid_at: Set(p.paid_at),
            created_at: Set(p.created_at),
            created_by: Set(p.created_by),
        };
        active.insert(&self.db).await
    }

    // ── Fee Templates ──

    pub async fn create_fee_template(&self, tpl: fee_template::Model) -> Result<fee_template::Model, DbErr> {
        let active = fee_template::ActiveModel {
            id: Set(tpl.id),
            org_id: Set(tpl.org_id),
            name: Set(tpl.name),
            name_ar: Set(tpl.name_ar),
            code: Set(tpl.code),
            category: Set(tpl.category),
            default_amount: Set(tpl.default_amount),
            revenue_account_id: Set(tpl.revenue_account_id),
            receivable_account_id: Set(tpl.receivable_account_id),
            applicable_levels: Set(tpl.applicable_levels),
            frequency: Set(tpl.frequency),
            installment_config: Set(tpl.installment_config),
            late_fee_rules: Set(tpl.late_fee_rules),
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
            default_amount: Set(tpl.default_amount),
            revenue_account_id: Set(tpl.revenue_account_id),
            receivable_account_id: Set(tpl.receivable_account_id),
            applicable_levels: Set(tpl.applicable_levels),
            frequency: Set(tpl.frequency),
            installment_config: Set(tpl.installment_config),
            late_fee_rules: Set(tpl.late_fee_rules),
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

    // ── Discount Templates ──

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

    // ── Treasury Transactions ──

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
