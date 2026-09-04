use async_trait::async_trait;
use orbis_app::ports::repositories::{
    AccountRepository, JournalEntryRepository, FeeTemplateRepository,
    StudentFeeRepository, PaymentRepository
};
use orbis_domain::entities::account::Account as DomainAccount;
use orbis_domain::entities::journal_entry::JournalEntry as DomainJournalEntry;
use orbis_domain::entities::fee_template::FeeTemplate as DomainFeeTemplate;
use orbis_domain::entities::student_fee::StudentFee as DomainStudentFee;
use orbis_domain::entities::payment::Payment as DomainPayment;
use orbis_domain::entities::currency::Currency as DomainCurrency;
use orbis_domain::entities::exchange_rate::ExchangeRate as DomainExchangeRate;
use orbis_domain::errors::DomainError;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use uuid::Uuid;

use crate::models::{
    account::{self, Entity as AccountEntity},
    journal_entry::{self, Entity as JournalEntryEntity},
    fee_template::{self, Entity as FeeTemplateEntity},
    student_fee::{self, Entity as StudentFeeEntity},
    payment::{self, Entity as PaymentEntity},
    currency::{self, Entity as CurrencyEntity},
    exchange_rate::{self, Entity as ExchangeRateEntity},
};

pub struct FinanceRepoImpl {
    pub db: DatabaseConnection,
}

#[async_trait]
impl AccountRepository for FinanceRepoImpl {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<DomainAccount>, DomainError> {
        let model = AccountEntity::find_by_id(id).one(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(model.map(|m| DomainAccount {
            id: m.id, org_id: m.org_id, code: m.code, name: m.name,
            account_type: m.account_type, parent_id: m.parent_id,
            currency: m.currency, is_active: m.is_active, balances: m.balances,
            created_at: m.created_at.into(), updated_at: m.updated_at.into()
        }))
    }

    async fn save(&self, account: &DomainAccount) -> Result<(), DomainError> {
        let active = account::ActiveModel {
            id: Set(account.id), org_id: Set(account.org_id),
            code: Set(account.code.clone()), name: Set(account.name.clone()),
            account_type: Set(account.account_type.clone()), parent_id: Set(account.parent_id),
            currency: Set(account.currency.clone()), is_active: Set(account.is_active),
            balances: Set(account.balances.clone()),
            created_at: Set(account.created_at.into()), updated_at: Set(account.updated_at.into()),
        };

        if AccountEntity::find_by_id(account.id).one(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?.is_some() {
            active.update(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        } else {
            active.insert(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        }
        Ok(())
    }
}

#[async_trait]
impl JournalEntryRepository for FinanceRepoImpl {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<DomainJournalEntry>, DomainError> {
        let model = JournalEntryEntity::find_by_id(id).one(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(model.map(|m| DomainJournalEntry {
            id: m.id, org_id: m.org_id, transaction_ref: m.transaction_ref,
            accounting_date: m.accounting_date, amount: m.amount,
            base_amount: m.base_amount,
            currency_id: m.currency_id,
            exchange_rate: m.exchange_rate,
            debit_account_id: m.debit_account_id, credit_account_id: m.credit_account_id,
            description: m.description, created_at: m.created_at.into()
        }))
    }

    async fn save(&self, entry: &DomainJournalEntry) -> Result<(), DomainError> {
        let active = journal_entry::ActiveModel {
            id: Set(entry.id), org_id: Set(entry.org_id),
            transaction_ref: Set(entry.transaction_ref.clone()), accounting_date: Set(entry.accounting_date),
            amount: Set(entry.amount),
            base_amount: Set(entry.base_amount),
            currency_id: Set(entry.currency_id.clone()),
            exchange_rate: Set(entry.exchange_rate), debit_account_id: Set(entry.debit_account_id),
            credit_account_id: Set(entry.credit_account_id), description: Set(entry.description.clone()),
            created_at: Set(entry.created_at.into()),
        };

        if JournalEntryEntity::find_by_id(entry.id).one(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?.is_some() {
            active.update(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        } else {
            active.insert(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        }
        Ok(())
    }
}

#[async_trait]
impl FeeTemplateRepository for FinanceRepoImpl {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<DomainFeeTemplate>, DomainError> {
        let model = FeeTemplateEntity::find_by_id(id).one(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(model.map(|m| DomainFeeTemplate {
            id: m.id, org_id: m.org_id, name: m.name, default_amount: m.default_amount,
            revenue_account_id: m.revenue_account_id, receivable_account_id: m.receivable_account_id,
            is_mandatory: m.is_mandatory, created_at: m.created_at.into(), updated_at: m.updated_at.into()
        }))
    }

    async fn save(&self, template: &DomainFeeTemplate) -> Result<(), DomainError> {
        let active = fee_template::ActiveModel {
            id: Set(template.id), org_id: Set(template.org_id),
            name: Set(template.name.clone()), default_amount: Set(template.default_amount),
            revenue_account_id: Set(template.revenue_account_id), receivable_account_id: Set(template.receivable_account_id),
            is_mandatory: Set(template.is_mandatory),
            created_at: Set(template.created_at.into()), updated_at: Set(template.updated_at.into()),
        };

        if FeeTemplateEntity::find_by_id(template.id).one(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?.is_some() {
            active.update(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        } else {
            active.insert(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        }
        Ok(())
    }
}

#[async_trait]
impl StudentFeeRepository for FinanceRepoImpl {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<DomainStudentFee>, DomainError> {
        let model = StudentFeeEntity::find_by_id(id).one(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(model.map(|m| DomainStudentFee {
            id: m.id, org_id: m.org_id, student_id: m.student_id,
            enrollment_id: m.enrollment_id,
            fee_template_id: m.fee_template_id, original_amount: m.original_amount,
            discount_amount: m.discount_amount, net_amount: m.net_amount,
            status: m.status, due_date: m.due_date,
            created_at: m.created_at.into(), updated_at: m.updated_at.into()
        }))
    }

    async fn save(&self, fee: &DomainStudentFee) -> Result<(), DomainError> {
        let active = student_fee::ActiveModel {
            id: Set(fee.id), org_id: Set(fee.org_id), student_id: Set(fee.student_id),
            enrollment_id: Set(fee.enrollment_id),
            fee_template_id: Set(fee.fee_template_id), original_amount: Set(fee.original_amount),
            discount_amount: Set(fee.discount_amount), net_amount: Set(fee.net_amount),
            status: Set(fee.status.clone()), due_date: Set(fee.due_date),
            created_at: Set(fee.created_at.into()), updated_at: Set(fee.updated_at.into()),
        };

        if StudentFeeEntity::find_by_id(fee.id).one(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?.is_some() {
            active.update(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        } else {
            active.insert(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        }
        Ok(())
    }
}

#[async_trait]
impl PaymentRepository for FinanceRepoImpl {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<DomainPayment>, DomainError> {
        let model = PaymentEntity::find_by_id(id).one(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(model.map(|m| DomainPayment {
            id: m.id, org_id: m.org_id, fee_id: m.fee_id,
            account_id: m.account_id, amount: m.amount,
            method: m.method,
            reference: m.reference, journal_entry_id: m.journal_entry_id,
            paid_at: m.paid_at.into(), created_at: m.created_at.into()
        }))
    }

    async fn save(&self, payment: &DomainPayment) -> Result<(), DomainError> {
        let active = payment::ActiveModel {
            id: Set(payment.id), org_id: Set(payment.org_id), fee_id: Set(payment.fee_id),
            account_id: Set(payment.account_id), amount: Set(payment.amount),
            method: Set(payment.method.clone()), reference: Set(payment.reference.clone()),
            journal_entry_id: Set(payment.journal_entry_id),
            paid_at: Set(payment.paid_at.into()), created_at: Set(payment.created_at.into()),
        };

        if PaymentEntity::find_by_id(payment.id).one(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?.is_some() {
            active.update(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        } else {
            active.insert(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        }
        Ok(())
    }
}

#[async_trait]
impl orbis_app::ports::repositories::CurrencyRepository for FinanceRepoImpl {
    async fn find_currency_by_id(&self, id: &str) -> Result<Option<DomainCurrency>, DomainError> {
        let model = CurrencyEntity::find_by_id(id).one(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(model.map(|m| DomainCurrency {
            id: m.id, name: m.name, name_ar: m.name_ar, symbol: m.symbol,
            decimal_places: m.decimal_places, is_active: m.is_active,
            created_at: m.created_at.into()
        }))
    }

    async fn get_exchange_rate(&self, org_id: Uuid, base_currency_id: &str, target_currency_id: &str) -> Result<Option<DomainExchangeRate>, DomainError> {
        use sea_orm::{ColumnTrait, QueryFilter};
        let model = ExchangeRateEntity::find()
            .filter(exchange_rate::Column::OrgId.eq(org_id))
            .filter(exchange_rate::Column::BaseCurrencyId.eq(base_currency_id))
            .filter(exchange_rate::Column::TargetCurrencyId.eq(target_currency_id))
            .one(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
            
        Ok(model.map(|m| DomainExchangeRate {
            id: m.id, org_id: m.org_id, base_currency_id: m.base_currency_id,
            target_currency_id: m.target_currency_id, rate: m.rate,
            effective_date: m.effective_date.into(), created_at: m.created_at.into()
        }))
    }
}
