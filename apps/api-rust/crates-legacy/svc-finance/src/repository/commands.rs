//! Finance command repository — write operations.

use crate::entities::{account, student_fee, payment, fee_template, journal_entry, discount_template};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DbErr, EntityTrait, ColumnTrait, QueryFilter, ConnectionTrait};
use super::FinanceCommandRepo;

/// Update account balances on any connection (including transactions).
pub async fn update_account_balance_on<C: ConnectionTrait>(
    conn: &C,
    account_id: &str,
    currency: &str,
    amount: f64,
) -> Result<(), DbErr> {
    let mut current_id = Some(account_id.to_string());
    while let Some(id) = current_id {
        let account = account::Entity::find_by_id(id).one(conn).await?;
        if let Some(account) = account {
            let mut cb: shared_kernel::currency::CurrencyBalance =
                serde_json::from_value(account.balances.clone()).unwrap_or_default();
            cb.add(currency, amount);

            let mut active: account::ActiveModel = account.into();
            active.balances = Set(serde_json::to_value(&cb).unwrap());
            active.updated_at = Set(chrono::Utc::now().to_rfc3339());
            let updated = active.update(conn).await?;
            current_id = updated.parent_id;
        } else {
            break;
        }
    }
    Ok(())
}

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
        update_account_balance_on(&self.db, account_id, currency, amount).await
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
            cash_account_id: Set(p.cash_account_id),
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

}
