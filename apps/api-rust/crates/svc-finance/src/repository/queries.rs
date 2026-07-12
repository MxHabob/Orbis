//! Finance query repository — read operations.

use crate::entities::{account, treasury, student_fee, payment, fee_template, journal_entry, discount_template, currency};
use crate::params::{FeeTemplateParams, StudentFeeParams, PaymentParams, TreasuryParams, LedgerAccountParams};
use sea_orm::{entity::prelude::*, QueryFilter, QueryOrder, EntityTrait, ColumnTrait};
use super::FinanceQueryRepo;

impl FinanceQueryRepo {
    // ── Ledger Accounts ──

    pub async fn get_accounts(&self, org_id: &str, params: LedgerAccountParams) -> Result<Vec<account::Model>, DbErr> {
        let mut query = account::Entity::find()
            .filter(account::Column::OrgId.eq(org_id));

        if let Some(ref itype) = params.item_type {
            query = query.filter(account::Column::ItemType.eq(itype));
        }
        if let Some(active) = params.is_active {
            query = query.filter(account::Column::IsActive.eq(active));
        }

        query.order_by_asc(account::Column::Code).all(&self.db).await
    }

    pub async fn get_account_by_id(&self, id: &str) -> Result<Option<account::Model>, DbErr> {
        account::Entity::find_by_id(id.to_string()).one(&self.db).await
    }

    pub async fn get_account_by_entity(&self, org_id: &str, entity_type: &str, entity_id: &str) -> Result<Option<account::Model>, DbErr> {
        account::Entity::find()
            .filter(account::Column::OrgId.eq(org_id))
            .filter(account::Column::EntityType.eq(Some(entity_type.to_string())))
            .filter(account::Column::EntityId.eq(Some(entity_id.to_string())))
            .one(&self.db)
            .await
    }

    pub async fn get_control_account_by_sub_type(&self, org_id: &str, sub_type: &str) -> Result<Option<account::Model>, DbErr> {
        account::Entity::find()
            .filter(account::Column::OrgId.eq(org_id))
            .filter(account::Column::IsControlAccount.eq(1))
            .filter(account::Column::SubType.eq(Some(sub_type.to_string())))
            .one(&self.db)
            .await
    }

    pub async fn is_leaf_account(&self, account_id: &str) -> Result<bool, DbErr> {
        let child_count = account::Entity::find()
            .filter(account::Column::ParentId.eq(Some(account_id.to_string())))
            .count(&self.db)
            .await?;
        Ok(child_count == 0)
    }

    // ── Treasuries ──

    pub async fn get_treasuries(&self, org_id: &str, params: TreasuryParams) -> Result<Vec<treasury::Model>, DbErr> {
        let mut query = treasury::Entity::find()
            .filter(treasury::Column::OrgId.eq(org_id));

        if let Some(ref itype) = params.item_type {
            query = query.filter(treasury::Column::ItemType.eq(itype));
        }
        if let Some(active) = params.is_active {
            query = query.filter(treasury::Column::IsActive.eq(active));
        }

        query.all(&self.db).await
    }

    pub async fn get_treasury_by_id(&self, id: &str) -> Result<Option<treasury::Model>, DbErr> {
        treasury::Entity::find_by_id(id.to_string()).one(&self.db).await
    }

    // ── Student Fees ──

    pub async fn get_student_fees(&self, org_id: &str, params: StudentFeeParams) -> Result<Vec<student_fee::Model>, DbErr> {
        let mut query = student_fee::Entity::find()
            .filter(student_fee::Column::OrgId.eq(org_id));

        if let Some(ref enroll_id) = params.enrollment_id {
            query = query.filter(student_fee::Column::EnrollmentId.eq(enroll_id));
        }
        if let Some(ref period_id) = params.period_id {
            query = query.filter(student_fee::Column::PeriodId.eq(period_id));
        }
        if let Some(ref status) = params.status {
            query = query.filter(student_fee::Column::Status.eq(status));
        }

        query.all(&self.db).await
    }

    pub async fn get_student_fee_by_id(&self, id: &str) -> Result<Option<student_fee::Model>, DbErr> {
        student_fee::Entity::find_by_id(id.to_string()).one(&self.db).await
    }

    // ── Payments ──

    pub async fn get_payments(&self, org_id: &str, params: PaymentParams) -> Result<Vec<payment::Model>, DbErr> {
        let mut query = payment::Entity::find()
            .filter(payment::Column::OrgId.eq(org_id));

        if let Some(ref enroll_id) = params.enrollment_id {
            query = query.filter(payment::Column::EnrollmentId.eq(enroll_id));
        }
        if let Some(ref fee_id) = params.fee_id {
            query = query.filter(payment::Column::FeeId.eq(fee_id));
        }
        if let Some(ref treasury_id) = params.treasury_id {
            query = query.filter(payment::Column::TreasuryId.eq(treasury_id));
        }

        query.order_by_desc(payment::Column::PaidAt).all(&self.db).await
    }

    // ── Fee Templates ──

    pub async fn get_fee_templates(&self, org_id: &str, params: FeeTemplateParams) -> Result<Vec<fee_template::Model>, DbErr> {
        let mut query = fee_template::Entity::find()
            .filter(fee_template::Column::OrgId.eq(org_id));

        if let Some(ref cat) = params.category {
            query = query.filter(fee_template::Column::Category.eq(cat));
        }
        if let Some(active) = params.is_active {
            query = query.filter(fee_template::Column::IsActive.eq(active));
        }

        query.order_by_asc(fee_template::Column::SortOrder).all(&self.db).await
    }

    pub async fn get_fee_template_by_id(&self, id: &str) -> Result<Option<fee_template::Model>, DbErr> {
        fee_template::Entity::find_by_id(id.to_string()).one(&self.db).await
    }

    // ── Journal Entries ──

    pub async fn get_journal_entries(&self, org_id: &str) -> Result<Vec<journal_entry::Model>, DbErr> {
        journal_entry::Entity::find()
            .filter(journal_entry::Column::OrgId.eq(org_id))
            .order_by_desc(journal_entry::Column::AccountingDate)
            .all(&self.db)
            .await
    }

    pub async fn get_journal_entries_by_account(&self, org_id: &str, account_id: &str) -> Result<Vec<journal_entry::Model>, DbErr> {
        journal_entry::Entity::find()
            .filter(journal_entry::Column::OrgId.eq(org_id))
            .filter(
                journal_entry::Column::DebitAccountId.eq(account_id)
                    .or(journal_entry::Column::CreditAccountId.eq(account_id))
            )
            .order_by_asc(journal_entry::Column::AccountingDate)
            .all(&self.db)
            .await
    }

    pub async fn get_journal_entries_by_accounts(&self, org_id: &str, account_ids: &[String]) -> Result<Vec<journal_entry::Model>, DbErr> {
        journal_entry::Entity::find()
            .filter(journal_entry::Column::OrgId.eq(org_id))
            .filter(
                journal_entry::Column::DebitAccountId.is_in(account_ids.to_vec())
                    .or(journal_entry::Column::CreditAccountId.is_in(account_ids.to_vec()))
            )
            .order_by_asc(journal_entry::Column::AccountingDate)
            .all(&self.db)
            .await
    }

    // ── Discount Templates ──

    pub async fn get_discount_templates(&self, org_id: &str, is_active: Option<i32>) -> Result<Vec<discount_template::Model>, DbErr> {
        let mut query = discount_template::Entity::find()
            .filter(discount_template::Column::OrgId.eq(org_id));

        if let Some(active) = is_active {
            query = query.filter(discount_template::Column::IsActive.eq(active));
        }

        query.all(&self.db).await
    }

    pub async fn get_discount_template_by_id(&self, id: &str) -> Result<Option<discount_template::Model>, DbErr> {
        discount_template::Entity::find_by_id(id.to_string()).one(&self.db).await
    }

    // ── Currencies ──

    pub async fn get_currencies(&self, org_id: &str, is_active: Option<i32>) -> Result<Vec<currency::Model>, DbErr> {
        let mut query = currency::Entity::find()
            .filter(currency::Column::OrgId.eq(org_id));

        if let Some(active) = is_active {
            query = query.filter(currency::Column::IsActive.eq(active));
        }

        query.all(&self.db).await
    }

    pub async fn get_currency_by_code(&self, org_id: &str, code: &str) -> Result<Option<currency::Model>, DbErr> {
        currency::Entity::find()
            .filter(currency::Column::OrgId.eq(org_id))
            .filter(currency::Column::Code.eq(code))
            .one(&self.db)
            .await
    }
}
