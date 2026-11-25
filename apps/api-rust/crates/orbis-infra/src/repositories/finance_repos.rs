use async_trait::async_trait;
use orbis_app::ports::repositories::{
    AccountRepository, CurrencyRepository, DiscountTemplateRepository, FeeTemplateRepository,
    JournalEntryRepository, LedgerAccountBalances, PaymentRepository, StudentFeeRepository,
};
use orbis_domain::entities::account::{Account as DomainAccount, AccountType};
use orbis_domain::entities::currency::Currency as DomainCurrency;
use orbis_domain::entities::discount_template::{
    DiscountTemplate as DomainDiscountTemplate, DiscountType,
};
use orbis_domain::entities::exchange_rate::ExchangeRate as DomainExchangeRate;
use orbis_domain::entities::fee_template::{FeeFrequency, FeeTemplate as DomainFeeTemplate};
use orbis_domain::entities::journal_entry::{
    JournalEntry as DomainJournalEntry, JournalEntryLine, JournalPostingStatus,
};
use orbis_domain::entities::payment::{Payment as DomainPayment, PaymentMethod};
use orbis_domain::entities::student_fee::{
    FeeStatus, StudentFee as DomainStudentFee, StudentFeeCurrencySummary,
};
use orbis_domain::errors::DomainError;
use rust_decimal::Decimal;
use sea_orm::sea_query::Expr;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, ConnectionTrait, DatabaseConnection, EntityTrait,
    Order, PaginatorTrait, QueryFilter, QueryOrder, Set, Statement, TransactionTrait,
};
use std::collections::HashMap;
use uuid::Uuid;

use crate::models::{
    account::{self, Entity as AccountEntity},
    currency::{self, Entity as CurrencyEntity},
    discount_template::{self, Entity as DiscountTemplateEntity},
    exchange_rate::{self, Entity as ExchangeRateEntity},
    fee_template::{self, Entity as FeeTemplateEntity},
    journal_entry::{self, Entity as JournalEntryEntity},
    journal_entry_line::{self, Entity as JournalEntryLineEntity},
    payment::{self, Entity as PaymentEntity},
    student_fee::{self, Entity as StudentFeeEntity},
};

pub struct FinanceRepoImpl {
    pub db: DatabaseConnection,
}

#[async_trait]
impl AccountRepository for FinanceRepoImpl {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<DomainAccount>, DomainError> {
        let model = AccountEntity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(model.map(map_account_model_to_domain))
    }

    async fn find_by_org(&self, org_id: Uuid) -> Result<Vec<DomainAccount>, DomainError> {
        let models = AccountEntity::find()
            .filter(account::Column::OrgId.eq(org_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(models
            .into_iter()
            .map(map_account_model_to_domain)
            .collect())
    }

    async fn find_by_code(
        &self,
        org_id: Uuid,
        code: &str,
    ) -> Result<Option<DomainAccount>, DomainError> {
        let model = AccountEntity::find()
            .filter(account::Column::OrgId.eq(org_id))
            .filter(account::Column::Code.eq(code))
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(model.map(map_account_model_to_domain))
    }

    async fn save(&self, a: &DomainAccount) -> Result<(), DomainError> {
        let active = account::ActiveModel {
            id: Set(a.id),
            org_id: Set(a.org_id),
            code: Set(a.code.clone()),
            name: Set(a.name.clone()),
            name_ar: Set(a.name_ar.clone()),
            account_type: Set(a.account_type.to_string()),
            sub_type: Set(a.sub_type.clone()),
            parent_id: Set(a.parent_id),
            currency_code: Set(a.currency_code.clone()),
            is_active: Set(a.is_active),
            balances: Set(a.balances.clone()),
            created_by: Set(a.created_by),
            updated_by: Set(a.updated_by),
            version: Set(a.version),
            created_at: Set(a.created_at.into()),
            updated_at: Set(a.updated_at.into()),
        };

        if AccountEntity::find_by_id(a.id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?
            .is_some()
        {
            active
                .update(&self.db)
                .await
                .map_err(|e| DomainError::Internal(e.to_string()))?;
        } else {
            active
                .insert(&self.db)
                .await
                .map_err(|e| DomainError::Internal(e.to_string()))?;
        }
        Ok(())
    }

    async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        AccountEntity::delete_by_id(id)
            .exec(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(())
    }
}

fn map_account_model_to_domain(m: account::Model) -> DomainAccount {
    DomainAccount {
        id: m.id,
        org_id: m.org_id,
        code: m.code,
        name: m.name,
        name_ar: m.name_ar,
        account_type: match m.account_type.to_lowercase().as_str() {
            "liability" => AccountType::Liability,
            "equity" => AccountType::Equity,
            "revenue" => AccountType::Revenue,
            "expense" => AccountType::Expense,
            _ => AccountType::Asset,
        },
        sub_type: m.sub_type,
        parent_id: m.parent_id,
        path: "/".to_string(),
        depth: 0,
        currency_code: m.currency_code,
        entity_type: None,
        entity_id: None,
        is_control_account: false,
        is_system_account: false,
        is_active: m.is_active,
        balances: m.balances,
        created_by: m.created_by,
        updated_by: m.updated_by,
        version: m.version,
        created_at: m.created_at.into(),
        updated_at: m.updated_at.into(),
    }
}

async fn insert_journal_lines(
    db: &impl ConnectionTrait,
    entry: &DomainJournalEntry,
) -> Result<(), DomainError> {
    if entry.lines.is_empty() {
        return Ok(());
    }
    JournalEntryLineEntity::insert_many(entry.lines.iter().map(|line| {
        journal_entry_line::ActiveModel {
            id: Set(line.id),
            journal_entry_id: Set(entry.id),
            account_id: Set(line.account_id),
            debit_amount: Set(line.debit_amount),
            credit_amount: Set(line.credit_amount),
            description: Set(line.description.clone()),
            entity_type: Set(line.entity_type.clone()),
            entity_id: Set(line.entity_id),
        }
    }))
    .exec(db)
    .await
    .map_err(|error| DomainError::Internal(error.to_string()))?;
    Ok(())
}

async fn load_journal_lines(
    db: &impl ConnectionTrait,
    headers: &[journal_entry::Model],
) -> Result<HashMap<Uuid, Vec<JournalEntryLine>>, DomainError> {
    let mut by_entry: HashMap<Uuid, Vec<JournalEntryLine>> = HashMap::new();
    for batch in headers.chunks(500) {
        let lines = JournalEntryLineEntity::find()
            .filter(
                journal_entry_line::Column::JournalEntryId
                    .is_in(batch.iter().map(|header| header.id)),
            )
            .order_by_asc(journal_entry_line::Column::Id)
            .all(db)
            .await
            .map_err(|error| DomainError::Internal(error.to_string()))?;
        for line in lines {
            by_entry
                .entry(line.journal_entry_id)
                .or_default()
                .push(JournalEntryLine {
                    id: line.id,
                    account_id: line.account_id,
                    debit_amount: line.debit_amount,
                    credit_amount: line.credit_amount,
                    description: line.description,
                    entity_type: line.entity_type,
                    entity_id: line.entity_id,
                });
        }
    }
    Ok(by_entry)
}

fn journal_header(entry: &DomainJournalEntry) -> journal_entry::ActiveModel {
    journal_entry::ActiveModel {
        id: Set(entry.id),
        org_id: Set(entry.org_id),
        reference_number: Set(entry.reference_number.clone()),
        accounting_date: Set(entry.accounting_date),
        amount: Set(entry.amount),
        base_amount: Set(entry.base_amount),
        currency_code: Set(entry.currency_code.clone()),
        exchange_rate: Set(entry.exchange_rate),
        debit_account_id: Set(Some(entry.debit_account_id)),
        credit_account_id: Set(Some(entry.credit_account_id)),
        description: Set(entry.description.clone()),
        description_ar: Set(entry.description_ar.clone()),
        source_type: Set(Some(entry.source_type.clone())),
        source_id: Set(entry.source_id),
        status: Set(Some(entry.status.to_string())),
        is_reversed: Set(Some(entry.is_reversed)),
        reversed_by: Set(entry.reversed_by),
        reversal_reason: Set(entry.reversal_reason.clone()),
        created_by: Set(entry.created_by),
        updated_by: Set(entry.updated_by),
        org_unit_id: Set(entry.org_unit_id),
        version: Set(entry.version),
        created_at: Set(entry.created_at.into()),
        updated_at: Set(Some(entry.updated_at.into())),
    }
}

fn map_journal_entry_model_to_domain(
    m: journal_entry::Model,
    mut lines: Vec<JournalEntryLine>,
) -> DomainJournalEntry {
    if lines.is_empty() && m.amount > Decimal::ZERO {
        if let (Some(debit), Some(credit)) = (m.debit_account_id, m.credit_account_id) {
            if debit != credit {
                for (account_id, debit_amount, credit_amount) in [
                    (debit, m.amount, Decimal::ZERO),
                    (credit, Decimal::ZERO, m.amount),
                ] {
                    lines.push(JournalEntryLine {
                        id: account_id,
                        account_id,
                        debit_amount,
                        credit_amount,
                        description: Some(m.description.clone()),
                        entity_type: None,
                        entity_id: None,
                    });
                }
            }
        }
    }
    DomainJournalEntry {
        id: m.id,
        org_id: m.org_id,
        reference_number: m.reference_number,
        accounting_date: m.accounting_date,
        amount: m.amount,
        base_amount: m.base_amount,
        currency_code: m.currency_code,
        exchange_rate: m.exchange_rate,
        debit_account_id: m.debit_account_id.unwrap_or_default(),
        credit_account_id: m.credit_account_id.unwrap_or_default(),
        description: m.description,
        description_ar: m.description_ar,
        source_type: m.source_type.unwrap_or_else(|| "manual".into()),
        source_id: m.source_id,
        status: match m.status.as_deref().unwrap_or("posted") {
            "draft" => JournalPostingStatus::Draft,
            "reconciled" => JournalPostingStatus::Reconciled,
            "reversed" => JournalPostingStatus::Reversed,
            _ => JournalPostingStatus::Posted,
        },
        lines,
        is_reversed: m.is_reversed.unwrap_or(false),
        reversed_by: m.reversed_by,
        reversal_reason: m.reversal_reason,
        created_by: m.created_by,
        created_at: m.created_at.into(),
        updated_at: m
            .updated_at
            .map(Into::into)
            .unwrap_or_else(|| m.created_at.into()),
        updated_by: m.updated_by,
        org_unit_id: m.org_unit_id,
        version: m.version,
    }
}

#[async_trait]
impl JournalEntryRepository for FinanceRepoImpl {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<DomainJournalEntry>, DomainError> {
        let model = JournalEntryEntity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        let Some(model) = model else {
            return Ok(None);
        };
        let mut lines = load_journal_lines(&self.db, std::slice::from_ref(&model)).await?;
        Ok(Some(map_journal_entry_model_to_domain(
            model,
            lines.remove(&id).unwrap_or_default(),
        )))
    }

    async fn find_by_org(&self, org_id: Uuid) -> Result<Vec<DomainJournalEntry>, DomainError> {
        let models = JournalEntryEntity::find()
            .filter(journal_entry::Column::OrgId.eq(org_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        let mut lines = load_journal_lines(&self.db, &models).await?;
        Ok(models
            .into_iter()
            .map(|model| {
                let entry_lines = lines.remove(&model.id).unwrap_or_default();
                map_journal_entry_model_to_domain(model, entry_lines)
            })
            .collect())
    }

    async fn account_balances_at(
        &self,
        org_id: Uuid,
        date: chrono::NaiveDate,
    ) -> Result<LedgerAccountBalances, DomainError> {
        let backend = self.db.get_database_backend();
        let postgres = backend == sea_orm::DatabaseBackend::Postgres;
        let (org_param, date_param) = if postgres { ("$1", "$2") } else { ("?", "?") };
        let (debit, credit, group) = if postgres {
            (
                "SUM(debit)",
                "SUM(credit)",
                "GROUP BY account_id, currency_code",
            )
        } else {
            ("debit", "credit", "")
        };
        // SQLite sums NUMERIC values using floating point, so sum its decimal rows in Rust.
        let sql = format!(
            "WITH posted AS (
                SELECT id, currency_code, amount, debit_account_id, credit_account_id
                FROM journal_entries
                WHERE org_id = {org_param} AND accounting_date <= {date_param}
                    AND COALESCE(status, 'posted') <> 'draft'
             ), movements AS (
                SELECT line.account_id, p.currency_code, line.debit_amount AS debit, line.credit_amount AS credit
                FROM posted p JOIN journal_entry_lines line ON line.journal_entry_id = p.id
                UNION ALL
                SELECT p.debit_account_id, p.currency_code, p.amount, 0 FROM posted p
                WHERE p.debit_account_id <> p.credit_account_id AND p.amount > 0
                    AND NOT EXISTS (SELECT 1 FROM journal_entry_lines l WHERE l.journal_entry_id = p.id)
                UNION ALL
                SELECT p.credit_account_id, p.currency_code, 0, p.amount FROM posted p
                WHERE p.debit_account_id <> p.credit_account_id AND p.amount > 0
                    AND NOT EXISTS (SELECT 1 FROM journal_entry_lines l WHERE l.journal_entry_id = p.id)
             )
             SELECT account_id, currency_code, CAST({debit} AS TEXT) AS debit,
                CAST({credit} AS TEXT) AS credit FROM movements {group}"
        );
        let rows = self
            .db
            .query_all(Statement::from_sql_and_values(
                backend,
                sql,
                [org_id.into(), date.into()],
            ))
            .await
            .map_err(|error| DomainError::Internal(error.to_string()))?;
        let mut balances = LedgerAccountBalances::new();
        for row in rows {
            let account_id = row
                .try_get::<Uuid>("", "account_id")
                .map_err(|error| DomainError::Internal(error.to_string()))?;
            let currency = row
                .try_get::<String>("", "currency_code")
                .map_err(|error| DomainError::Internal(error.to_string()))?;
            let amount = |column| -> Result<Decimal, DomainError> {
                let text = row
                    .try_get::<String>("", column)
                    .map_err(|error| DomainError::Internal(error.to_string()))?;
                text.parse()
                    .or_else(|_| Decimal::from_scientific(&text))
                    .map_err(|error| DomainError::Internal(error.to_string()))
            };
            *balances
                .entry(account_id)
                .or_default()
                .entry(currency)
                .or_default() += amount("debit")? - amount("credit")?;
        }
        Ok(balances)
    }

    async fn save(&self, entry: &DomainJournalEntry) -> Result<(), DomainError> {
        entry.validate_balance().map_err(DomainError::Validation)?;
        let txn = self
            .db
            .begin()
            .await
            .map_err(|error| DomainError::Internal(error.to_string()))?;
        let active = journal_header(entry);
        if JournalEntryEntity::find_by_id(entry.id)
            .one(&txn)
            .await
            .map_err(|error| DomainError::Internal(error.to_string()))?
            .is_some()
        {
            active
                .update(&txn)
                .await
                .map_err(|error| DomainError::Internal(error.to_string()))?;
            JournalEntryLineEntity::delete_many()
                .filter(journal_entry_line::Column::JournalEntryId.eq(entry.id))
                .exec(&txn)
                .await
                .map_err(|error| DomainError::Internal(error.to_string()))?;
        } else {
            active
                .insert(&txn)
                .await
                .map_err(|error| DomainError::Internal(error.to_string()))?;
        }
        insert_journal_lines(&txn, entry).await?;
        txn.commit()
            .await
            .map_err(|error| DomainError::Internal(error.to_string()))
    }

    async fn save_reversal(
        &self,
        entry: &DomainJournalEntry,
        original_id: Uuid,
        reason: &str,
    ) -> Result<(), DomainError> {
        entry.validate_balance().map_err(DomainError::Validation)?;
        let txn = self
            .db
            .begin()
            .await
            .map_err(|error| DomainError::Internal(error.to_string()))?;
        let update = JournalEntryEntity::update_many()
            .col_expr(journal_entry::Column::IsReversed, Expr::value(true))
            .col_expr(
                journal_entry::Column::ReversedBy,
                Expr::value(entry.created_by),
            )
            .col_expr(
                journal_entry::Column::ReversalReason,
                Expr::value(reason.to_owned()),
            )
            .col_expr(
                journal_entry::Column::UpdatedAt,
                Expr::value(entry.updated_at),
            )
            .col_expr(
                journal_entry::Column::Version,
                Expr::col(journal_entry::Column::Version).add(1),
            )
            .filter(journal_entry::Column::Id.eq(original_id))
            .filter(journal_entry::Column::OrgId.eq(entry.org_id))
            .filter(
                Condition::any()
                    .add(journal_entry::Column::IsReversed.eq(false))
                    .add(journal_entry::Column::IsReversed.is_null()),
            )
            .exec(&txn)
            .await
            .map_err(|error| DomainError::Internal(error.to_string()))?;
        if update.rows_affected != 1 {
            return Err(DomainError::ConcurrencyConflict(
                "Journal entry was already reversed".into(),
            ));
        }
        journal_header(entry)
            .insert(&txn)
            .await
            .map_err(|error| DomainError::Internal(error.to_string()))?;
        insert_journal_lines(&txn, entry).await?;
        txn.commit()
            .await
            .map_err(|error| DomainError::Internal(error.to_string()))
    }

    async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        let txn = self
            .db
            .begin()
            .await
            .map_err(|error| DomainError::Internal(error.to_string()))?;
        JournalEntryLineEntity::delete_many()
            .filter(journal_entry_line::Column::JournalEntryId.eq(id))
            .exec(&txn)
            .await
            .map_err(|error| DomainError::Internal(error.to_string()))?;
        JournalEntryEntity::delete_by_id(id)
            .exec(&txn)
            .await
            .map_err(|error| DomainError::Internal(error.to_string()))?;
        txn.commit()
            .await
            .map_err(|error| DomainError::Internal(error.to_string()))
    }
}

fn map_fee_template_model_to_domain(m: fee_template::Model) -> DomainFeeTemplate {
    DomainFeeTemplate {
        id: m.id,
        org_id: m.org_id,
        code: m.code.unwrap_or_default(),
        name: m.name,
        name_ar: m.name_ar,
        category: m.category.unwrap_or_else(|| "tuition".into()),
        default_amount: m.default_amount,
        currency_code: m.currency_code.unwrap_or_else(|| "SAR".into()),
        revenue_account_id: m.revenue_account_id,
        receivable_account_id: m.receivable_account_id,
        applicable_levels: m
            .applicable_levels
            .and_then(|v| serde_json::from_value(v).ok()),
        applicable_study_modes: m
            .applicable_study_modes
            .and_then(|v| serde_json::from_value(v).ok()),
        frequency: match m.frequency.as_deref().unwrap_or("per_term") {
            "once" => FeeFrequency::Once,
            "monthly" => FeeFrequency::Monthly,
            "annual" => FeeFrequency::Annual,
            _ => FeeFrequency::PerTerm,
        },
        is_mandatory: m.is_mandatory,
        is_active: m.is_active.unwrap_or(true),
        created_at: m.created_at.into(),
        updated_at: m.updated_at.into(),
    }
}

#[async_trait]
impl FeeTemplateRepository for FinanceRepoImpl {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<DomainFeeTemplate>, DomainError> {
        let model = FeeTemplateEntity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(model.map(map_fee_template_model_to_domain))
    }

    async fn find_by_org(&self, org_id: Uuid) -> Result<Vec<DomainFeeTemplate>, DomainError> {
        let models = FeeTemplateEntity::find()
            .filter(fee_template::Column::OrgId.eq(org_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(models
            .into_iter()
            .map(map_fee_template_model_to_domain)
            .collect())
    }

    async fn save(&self, template: &DomainFeeTemplate) -> Result<(), DomainError> {
        let active = fee_template::ActiveModel {
            id: Set(template.id),
            org_id: Set(template.org_id),
            code: Set(Some(template.code.clone())),
            name: Set(template.name.clone()),
            name_ar: Set(template.name_ar.clone()),
            category: Set(Some(template.category.clone())),
            default_amount: Set(template.default_amount),
            currency_code: Set(Some(template.currency_code.clone())),
            revenue_account_id: Set(template.revenue_account_id),
            receivable_account_id: Set(template.receivable_account_id),
            applicable_levels: Set(template
                .applicable_levels
                .as_ref()
                .and_then(|v| serde_json::to_value(v).ok())),
            applicable_study_modes: Set(template
                .applicable_study_modes
                .as_ref()
                .and_then(|v| serde_json::to_value(v).ok())),
            frequency: Set(Some(template.frequency.to_string())),
            is_mandatory: Set(template.is_mandatory),
            is_active: Set(Some(template.is_active)),
            created_at: Set(template.created_at.into()),
            updated_at: Set(template.updated_at.into()),
        };

        if FeeTemplateEntity::find_by_id(template.id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?
            .is_some()
        {
            active
                .update(&self.db)
                .await
                .map_err(|e| DomainError::Internal(e.to_string()))?;
        } else {
            active
                .insert(&self.db)
                .await
                .map_err(|e| DomainError::Internal(e.to_string()))?;
        }
        Ok(())
    }

    async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        FeeTemplateEntity::delete_by_id(id)
            .exec(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(())
    }
}

fn map_discount_template_model_to_domain(m: discount_template::Model) -> DomainDiscountTemplate {
    DomainDiscountTemplate {
        id: m.id,
        org_id: m.org_id,
        code: m.code,
        name: m.name,
        name_ar: m.name_ar,
        discount_type: match m.discount_type.as_str() {
            "fixed_amount" => DiscountType::FixedAmount,
            _ => DiscountType::Percentage,
        },
        value: m.value,
        eligibility_rules: m.eligibility_rules,
        is_approval_required: m.is_approval_required,
        is_auto_apply: m.is_auto_apply,
        is_stackable: m.is_stackable,
        priority: m.priority as u32,
        is_active: m.is_active,
        created_at: m.created_at.into(),
        updated_at: m.updated_at.into(),
    }
}

#[async_trait]
impl DiscountTemplateRepository for FinanceRepoImpl {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<DomainDiscountTemplate>, DomainError> {
        let model = DiscountTemplateEntity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(model.map(map_discount_template_model_to_domain))
    }

    async fn find_by_org(&self, org_id: Uuid) -> Result<Vec<DomainDiscountTemplate>, DomainError> {
        let models = DiscountTemplateEntity::find()
            .filter(discount_template::Column::OrgId.eq(org_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(models
            .into_iter()
            .map(map_discount_template_model_to_domain)
            .collect())
    }

    async fn save(&self, d: &DomainDiscountTemplate) -> Result<(), DomainError> {
        let active = discount_template::ActiveModel {
            id: Set(d.id),
            org_id: Set(d.org_id),
            code: Set(d.code.clone()),
            name: Set(d.name.clone()),
            name_ar: Set(d.name_ar.clone()),
            discount_type: Set(match d.discount_type {
                DiscountType::FixedAmount => "fixed_amount".into(),
                DiscountType::Percentage => "percentage".into(),
            }),
            value: Set(d.value),
            eligibility_rules: Set(d.eligibility_rules.clone()),
            is_approval_required: Set(d.is_approval_required),
            is_auto_apply: Set(d.is_auto_apply),
            is_stackable: Set(d.is_stackable),
            priority: Set(d.priority as i32),
            is_active: Set(d.is_active),
            created_at: Set(d.created_at.into()),
            updated_at: Set(d.updated_at.into()),
        };

        if DiscountTemplateEntity::find_by_id(d.id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?
            .is_some()
        {
            active
                .update(&self.db)
                .await
                .map_err(|e| DomainError::Internal(e.to_string()))?;
        } else {
            active
                .insert(&self.db)
                .await
                .map_err(|e| DomainError::Internal(e.to_string()))?;
        }
        Ok(())
    }

    async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        DiscountTemplateEntity::delete_by_id(id)
            .exec(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(())
    }
}

fn build_student_fee_list_query(
    org_id: Uuid,
    filter: &orbis_domain::queries::filters::StudentFeeFilter,
    backend: sea_orm::DatabaseBackend,
) -> sea_orm::Select<StudentFeeEntity> {
    let placeholder = |index| match backend {
        sea_orm::DatabaseBackend::Postgres => format!("${index}"),
        _ => "?".to_string(),
    };
    let mut query = StudentFeeEntity::find().filter(student_fee::Column::OrgId.eq(org_id));
    if let Some(student_id) = filter.student_id {
        query = query.filter(student_fee::Column::StudentId.eq(student_id));
    }
    if let Some(org_unit_id) = filter.org_unit_id {
        query = query.filter(Expr::cust_with_values(
            format!(
                "EXISTS (SELECT 1 FROM students s \
             WHERE s.id = student_fees.student_id \
               AND s.org_id = student_fees.org_id \
               AND s.org_unit_id = {})",
                placeholder(1)
            ),
            [org_unit_id],
        ));
    }
    if let Some(currency_code) = filter
        .currency_code
        .as_deref()
        .map(str::trim)
        .filter(|currency| !currency.is_empty())
    {
        query = query.filter(Expr::cust_with_values(
            format!(
                "EXISTS (SELECT 1 FROM fee_templates ft \
                 WHERE ft.id = student_fees.fee_template_id \
                   AND ft.org_id = student_fees.org_id \
                   AND ft.currency_code = {})",
                placeholder(1)
            ),
            [currency_code.to_string()],
        ));
    }
    if let Some(period_id) = filter.period_id {
        query = query.filter(student_fee::Column::PeriodId.eq(Some(period_id)));
    }
    if let Some(from_date) = filter.from_date {
        query = query.filter(student_fee::Column::DueDate.gte(from_date));
    }
    if let Some(to_date) = filter.to_date {
        query = query.filter(student_fee::Column::DueDate.lte(to_date));
    }
    if let Some(status) = filter.status.as_deref() {
        query = query.filter(student_fee::Column::Status.eq(status));
    }
    if filter.only_unpaid == Some(true) || filter.is_overdue == Some(true) {
        query = query
            .filter(Expr::cust(
                "COALESCE(balance_due, net_amount - COALESCE(paid_amount, 0)) > 0",
            ))
            .filter(student_fee::Column::Status.is_in(["pending", "partially_paid", "overdue"]));
    }
    if let Some(search) = filter
        .search
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        let pattern = format!("%{search}%");
        query = query.filter(Expr::cust_with_values(
            format!(
                "(LOWER(CAST(student_fees.id AS TEXT)) LIKE LOWER({}) \
             OR LOWER(CAST(student_fees.student_id AS TEXT)) LIKE LOWER({}) \
             OR LOWER(CAST(student_fees.fee_template_id AS TEXT)) LIKE LOWER({}) \
             OR LOWER(CAST(student_fees.status AS TEXT)) LIKE LOWER({}) \
             OR EXISTS (SELECT 1 FROM students s \
               INNER JOIN persons p ON s.person_id = p.id \
               WHERE s.id = student_fees.student_id \
                 AND s.org_id = student_fees.org_id \
                 AND (LOWER(s.student_number) LIKE LOWER({}) \
                   OR LOWER(COALESCE(p.first_name, '') || ' ' || COALESCE(p.last_name, '')) LIKE LOWER({}) \
                   OR LOWER(COALESCE(p.first_name_ar, '') || ' ' || COALESCE(p.last_name_ar, '')) LIKE LOWER({}))) \
             OR EXISTS (SELECT 1 FROM fee_templates ft \
               WHERE ft.id = student_fees.fee_template_id \
                 AND ft.org_id = student_fees.org_id \
                 AND (LOWER(ft.name) LIKE LOWER({}) OR LOWER(ft.name_ar) LIKE LOWER({}))))",
                placeholder(1),
                placeholder(2),
                placeholder(3),
                placeholder(4),
                placeholder(5),
                placeholder(6),
                placeholder(7),
                placeholder(8),
                placeholder(9),
            ),
            [
                pattern.clone(),
                pattern.clone(),
                pattern.clone(),
                pattern.clone(),
                pattern.clone(),
                pattern.clone(),
                pattern.clone(),
                pattern.clone(),
                pattern,
            ],
        ));
    }
    query
}

fn build_payment_list_query(
    org_id: Uuid,
    filter: &orbis_domain::queries::filters::PaymentFilter,
    backend: sea_orm::DatabaseBackend,
) -> sea_orm::Select<PaymentEntity> {
    let placeholder = |index| match backend {
        sea_orm::DatabaseBackend::Postgres => format!("${index}"),
        _ => "?".to_string(),
    };
    let mut query = PaymentEntity::find().filter(payment::Column::OrgId.eq(org_id));
    if let Some(student_id) = filter.student_id {
        query = query.filter(payment::Column::StudentId.eq(student_id));
    }
    if let Some(org_unit_id) = filter.org_unit_id {
        query = query.filter(Expr::cust_with_values(
            format!(
                "COALESCE(payments.org_unit_id, (SELECT s.org_unit_id FROM students s +                 WHERE s.id = payments.student_id AND s.org_id = payments.org_id)) = {}",
                placeholder(1),
            ),
            [org_unit_id],
        ));
    }
    if let Some(currency_code) = filter
        .currency_code
        .as_deref()
        .map(str::trim)
        .filter(|currency| !currency.is_empty())
    {
        query = query.filter(payment::Column::CurrencyCode.eq(currency_code));
    }
    if let Some(enrollment_id) = filter.enrollment_id {
        query = query.filter(payment::Column::EnrollmentId.eq(enrollment_id));
    }
    if let Some(fee_id) = filter.fee_id {
        query = query.filter(payment::Column::FeeId.eq(fee_id));
    }
    if let Some(method) = filter.method {
        query = query.filter(payment::Column::Method.eq(method.to_string()));
    }
    if let Some(from_date) = filter.from_date {
        query = query.filter(payment::Column::PaidAt.gte(from_date));
    }
    if let Some(to_date) = filter.to_date_exclusive {
        query = query.filter(payment::Column::PaidAt.lt(to_date));
    }
    if let Some(search) = filter
        .search
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        let pattern = format!("%{search}%");
        query = query.filter(Expr::cust_with_values(
            format!(
                "(LOWER(CAST(payments.id AS TEXT)) LIKE LOWER({}) \
             OR LOWER(CAST(payments.student_id AS TEXT)) LIKE LOWER({}) \
             OR LOWER(payments.receipt_number) LIKE LOWER({}) \
             OR LOWER(payments.reference) LIKE LOWER({}) \
             OR LOWER(payments.payer_name) LIKE LOWER({}) \
             OR LOWER(payments.payer_phone) LIKE LOWER({}) \
             OR LOWER(payments.notes) LIKE LOWER({}) \
             OR LOWER(payments.method) LIKE LOWER({}) \
             OR EXISTS (SELECT 1 FROM students s \
               INNER JOIN persons p ON s.person_id = p.id \
               WHERE s.id = payments.student_id \
                 AND s.org_id = payments.org_id \
                 AND (LOWER(s.student_number) LIKE LOWER({}) \
                   OR LOWER(COALESCE(p.first_name, '') || ' ' || COALESCE(p.last_name, '')) LIKE LOWER({}) \
                   OR LOWER(COALESCE(p.first_name_ar, '') || ' ' || COALESCE(p.last_name_ar, '')) LIKE LOWER({}))))",
                placeholder(1),
                placeholder(2),
                placeholder(3),
                placeholder(4),
                placeholder(5),
                placeholder(6),
                placeholder(7),
                placeholder(8),
                placeholder(9),
                placeholder(10),
                placeholder(11),
            ),
            [
                pattern.clone(),
                pattern.clone(),
                pattern.clone(),
                pattern.clone(),
                pattern.clone(),
                pattern.clone(),
                pattern.clone(),
                pattern.clone(),
                pattern.clone(),
                pattern.clone(),
                pattern,
            ],
        ));
    }
    query
}

fn fee_model(fee: &DomainStudentFee) -> student_fee::ActiveModel {
    student_fee::ActiveModel {
        id: Set(fee.id),
        org_id: Set(fee.org_id),
        period_id: Set(Some(fee.period_id)),
        student_id: Set(fee.student_id),
        enrollment_id: Set(fee.enrollment_id),
        fee_template_id: Set(fee.fee_template_id),
        original_amount: Set(fee.original_amount),
        discount_amount: Set(fee.discount_amount),
        net_amount: Set(fee.net_amount),
        paid_amount: Set(Some(fee.paid_amount)),
        balance_due: Set(Some(fee.balance_due)),
        status: Set(fee.status.to_string()),
        due_date: Set(fee.due_date),
        discount_details: Set(fee.discount_details.clone()),
        installment_number: Set(fee.installment_number.map(|n| n as i32)),
        notes: Set(fee.notes.clone()),
        created_by: Set(fee.created_by),
        created_at: Set(fee.created_at.into()),
        updated_at: Set(fee.updated_at.into()),
    }
}

#[async_trait]
impl StudentFeeRepository for FinanceRepoImpl {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<DomainStudentFee>, DomainError> {
        let model = StudentFeeEntity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(model.map(|m| DomainStudentFee {
            id: m.id,
            org_id: m.org_id,
            period_id: m.period_id.unwrap_or_default(),
            student_id: m.student_id,
            enrollment_id: m.enrollment_id,
            fee_template_id: m.fee_template_id,
            original_amount: m.original_amount,
            discount_amount: m.discount_amount,
            net_amount: m.net_amount,
            paid_amount: m.paid_amount.unwrap_or(Decimal::ZERO),
            balance_due: m.balance_due.unwrap_or(m.net_amount),
            status: match m.status.as_str() {
                "partially_paid" => FeeStatus::PartiallyPaid,
                "paid" => FeeStatus::Paid,
                "overdue" => FeeStatus::Overdue,
                "cancelled" => FeeStatus::Cancelled,
                "refunded" => FeeStatus::Refunded,
                _ => FeeStatus::Pending,
            },
            due_date: m.due_date,
            discount_details: m.discount_details,
            installment_number: m.installment_number.map(|n| n as u32),
            notes: m.notes,
            created_by: m.created_by,
            created_at: m.created_at.into(),
            updated_at: m.updated_at.into(),
        }))
    }

    async fn find_by_org(&self, org_id: Uuid) -> Result<Vec<DomainStudentFee>, DomainError> {
        let models = StudentFeeEntity::find()
            .filter(student_fee::Column::OrgId.eq(org_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(models
            .into_iter()
            .map(map_student_fee_model_to_domain)
            .collect())
    }

    async fn find_by_student(
        &self,
        student_id: Uuid,
    ) -> Result<Vec<DomainStudentFee>, DomainError> {
        let models = StudentFeeEntity::find()
            .filter(student_fee::Column::StudentId.eq(student_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(models
            .into_iter()
            .map(map_student_fee_model_to_domain)
            .collect())
    }

    async fn list(
        &self,
        org_id: Uuid,
        filter: orbis_domain::queries::filters::StudentFeeFilter,
    ) -> Result<orbis_domain::queries::filters::PaginatedList<DomainStudentFee>, DomainError> {
        let mut query =
            build_student_fee_list_query(org_id, &filter, self.db.get_database_backend());
        let pagination = filter.pagination.unwrap_or_default();
        let order = if pagination
            .sort_dir
            .as_deref()
            .is_some_and(|direction| direction.eq_ignore_ascii_case("desc"))
        {
            Order::Desc
        } else {
            Order::Asc
        };
        query = match pagination.sort_by.as_deref() {
            Some("due_date") => query.order_by(student_fee::Column::DueDate, order),
            Some("original_amount") => query.order_by(student_fee::Column::OriginalAmount, order),
            Some("net_amount") => query.order_by(student_fee::Column::NetAmount, order),
            Some("paid_amount") => query.order_by(student_fee::Column::PaidAmount, order),
            Some("balance_due") => query.order_by(student_fee::Column::BalanceDue, order),
            Some("status") => query.order_by(student_fee::Column::Status, order),
            Some("created_at") => query.order_by(student_fee::Column::CreatedAt, order),
            _ => query,
        };
        let page = pagination.page();
        let page_size = pagination.page_size();
        let paginator = query.paginate(&self.db, page_size);
        let total_count = paginator
            .num_items()
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        let rows = paginator
            .fetch_page(page - 1)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        let items = rows
            .into_iter()
            .map(map_student_fee_model_to_domain)
            .collect();
        Ok(orbis_domain::queries::filters::PaginatedList::new(
            items,
            total_count,
            page,
            page_size,
        ))
    }

    async fn summarize_by_currency(
        &self,
        org_id: Uuid,
        filter: orbis_domain::queries::filters::StudentFeeFilter,
    ) -> Result<Vec<StudentFeeCurrencySummary>, DomainError> {
        let backend = self.db.get_database_backend();
        let mut values = vec![org_id.into()];
        let mut conditions = vec!["sf.org_id = $1".to_string()];
        let mut add_value = |value: sea_orm::Value| {
            values.push(value);
            match backend {
                sea_orm::DatabaseBackend::Postgres => format!("${}", values.len()),
                _ => "?".to_string(),
            }
        };
        if let Some(from_date) = filter.from_date {
            let placeholder = add_value(from_date.into());
            conditions.push(format!("sf.due_date >= {placeholder}"));
        }
        if let Some(to_date) = filter.to_date {
            let placeholder = add_value(to_date.into());
            conditions.push(format!("sf.due_date <= {placeholder}"));
        }
        if let Some(org_unit_id) = filter.org_unit_id {
            let placeholder = add_value(org_unit_id.into());
            conditions.push(format!(
                "EXISTS (SELECT 1 FROM students s \
                 WHERE s.id = sf.student_id AND s.org_id = sf.org_id \
                   AND s.org_unit_id = {placeholder})"
            ));
        }
        if let Some(currency_code) = filter
            .currency_code
            .as_deref()
            .map(str::trim)
            .filter(|currency| !currency.is_empty())
        {
            let placeholder = add_value(currency_code.to_string().into());
            conditions.push(format!("COALESCE(ft.currency_code, '') = {placeholder}"));
        }
        let filter_clause = if backend == sea_orm::DatabaseBackend::Postgres {
            conditions.join(" AND ")
        } else {
            let mut sqlite_conditions = vec!["sf.org_id = ?".to_string()];
            if filter.from_date.is_some() {
                sqlite_conditions.push("sf.due_date >= ?".to_string());
            }
            if filter.to_date.is_some() {
                sqlite_conditions.push("sf.due_date <= ?".to_string());
            }
            if filter.org_unit_id.is_some() {
                sqlite_conditions.push(
                    "EXISTS (SELECT 1 FROM students s \
                     WHERE s.id = sf.student_id AND s.org_id = sf.org_id \
                       AND s.org_unit_id = ?)"
                        .to_string(),
                );
            }
            if filter
                .currency_code
                .as_deref()
                .map(str::trim)
                .is_some_and(|currency| !currency.is_empty())
            {
                sqlite_conditions.push("COALESCE(ft.currency_code, '') = ?".to_string());
            }
            sqlite_conditions.join(" AND ")
        };
        let sql = format!(
            "SELECT COALESCE(ft.currency_code, '') AS currency, \
                    CAST(COALESCE(SUM(sf.net_amount), 0) AS TEXT) AS due, \
                    CAST(COALESCE(SUM(COALESCE(sf.paid_amount, 0)), 0) AS TEXT) AS paid, \
                    CAST(COALESCE(SUM(COALESCE(sf.balance_due, \
                        sf.net_amount - COALESCE(sf.paid_amount, 0))), 0) AS TEXT) AS balance \
             FROM student_fees sf \
             LEFT JOIN fee_templates ft \
               ON ft.id = sf.fee_template_id AND ft.org_id = sf.org_id \
             WHERE {filter_clause} \
             GROUP BY ft.currency_code \
             ORDER BY ft.currency_code"
        );
        let rows = self
            .db
            .query_all(Statement::from_sql_and_values(backend, sql, values))
            .await
            .map_err(|error| {
                DomainError::Internal(format!("Student fee summary query failed: {error}"))
            })?;

        rows.into_iter()
            .map(|row| {
                let parse_total = |column| -> Result<Decimal, DomainError> {
                    let value: String = row
                        .try_get("", column)
                        .map_err(|error| DomainError::Internal(error.to_string()))?;
                    value.parse().map_err(|error| {
                        DomainError::Internal(format!(
                            "Invalid student fee summary {column}: {error}"
                        ))
                    })
                };
                Ok(StudentFeeCurrencySummary {
                    currency: row
                        .try_get("", "currency")
                        .map_err(|error| DomainError::Internal(error.to_string()))?,
                    due: parse_total("due")?,
                    paid: parse_total("paid")?,
                    balance: parse_total("balance")?,
                })
            })
            .collect()
    }

    async fn save(&self, fee: &DomainStudentFee) -> Result<(), DomainError> {
        let active = fee_model(fee);

        if StudentFeeEntity::find_by_id(fee.id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?
            .is_some()
        {
            active
                .update(&self.db)
                .await
                .map_err(|e| DomainError::Internal(e.to_string()))?;
        } else {
            active
                .insert(&self.db)
                .await
                .map_err(|e| DomainError::Internal(e.to_string()))?;
        }
        Ok(())
    }

    async fn delete_fee_posting(&self, fee: &DomainStudentFee) -> Result<(), DomainError> {
        let txn = self
            .db
            .begin()
            .await
            .map_err(|error| DomainError::Internal(error.to_string()))?;
        let deleted = StudentFeeEntity::delete_many()
            .filter(student_fee::Column::Id.eq(fee.id))
            .filter(student_fee::Column::OrgId.eq(fee.org_id))
            .filter(student_fee::Column::InstallmentNumber.is_null())
            .filter(
                Condition::any()
                    .add(student_fee::Column::PaidAmount.eq(Decimal::ZERO))
                    .add(student_fee::Column::PaidAmount.is_null()),
            )
            .exec(&txn)
            .await
            .map_err(|error| DomainError::Internal(error.to_string()))?;
        if deleted.rows_affected != 1 {
            return Err(DomainError::ConcurrencyConflict(
                "Fee has changed or is part of an installment schedule".into(),
            ));
        }
        if PaymentEntity::find()
            .filter(payment::Column::OrgId.eq(fee.org_id))
            .filter(payment::Column::FeeId.eq(fee.id))
            .one(&txn)
            .await
            .map_err(|error| DomainError::Internal(error.to_string()))?
            .is_some()
        {
            return Err(DomainError::Validation(
                "A fee with payment history cannot be deleted".into(),
            ));
        }
        let invoices = JournalEntryEntity::find()
            .filter(journal_entry::Column::OrgId.eq(fee.org_id))
            .filter(journal_entry::Column::SourceType.eq("student_fee_invoice"))
            .filter(journal_entry::Column::SourceId.eq(fee.id))
            .all(&txn)
            .await
            .map_err(|error| DomainError::Internal(error.to_string()))?;
        for invoice in invoices {
            delete_unreconciled_journal(&txn, invoice.id, fee.org_id).await?;
        }
        txn.commit()
            .await
            .map_err(|error| DomainError::Internal(error.to_string()))
    }

    async fn save_fee_installments(
        &self,
        original: &DomainStudentFee,
        installments: &[DomainStudentFee],
    ) -> Result<(), DomainError> {
        let Some((first, rest)) = installments.split_first() else {
            return Err(DomainError::Validation("Missing installments".into()));
        };
        let txn = self
            .db
            .begin()
            .await
            .map_err(|error| DomainError::Internal(error.to_string()))?;
        let reserved = StudentFeeEntity::update_many()
            .col_expr(
                student_fee::Column::InstallmentNumber,
                Expr::value(first.installment_number.map(|number| number as i32)),
            )
            .filter(student_fee::Column::Id.eq(original.id))
            .filter(student_fee::Column::OrgId.eq(original.org_id))
            .filter(student_fee::Column::NetAmount.eq(original.net_amount))
            .filter(student_fee::Column::Status.eq(original.status.to_string()))
            .filter(student_fee::Column::InstallmentNumber.is_null())
            .filter(
                Condition::any()
                    .add(student_fee::Column::PaidAmount.eq(Decimal::ZERO))
                    .add(student_fee::Column::PaidAmount.is_null()),
            )
            .exec(&txn)
            .await
            .map_err(|error| DomainError::Internal(error.to_string()))?;
        if reserved.rows_affected != 1 {
            return Err(DomainError::ConcurrencyConflict(
                "Fee has changed or is already split".into(),
            ));
        }
        fee_model(first)
            .update(&txn)
            .await
            .map_err(|error| DomainError::Internal(error.to_string()))?;
        StudentFeeEntity::insert_many(rest.iter().map(fee_model))
            .exec(&txn)
            .await
            .map_err(|error| DomainError::Internal(error.to_string()))?;
        txn.commit()
            .await
            .map_err(|error| DomainError::Internal(error.to_string()))
    }

    async fn save_fee_refund(
        &self,
        previous: &DomainStudentFee,
        updated: &DomainStudentFee,
        entry: &DomainJournalEntry,
    ) -> Result<(), DomainError> {
        entry.validate_balance().map_err(DomainError::Validation)?;
        let txn = self
            .db
            .begin()
            .await
            .map_err(|error| DomainError::Internal(error.to_string()))?;
        update_fee_balance(&txn, previous, updated).await?;
        journal_header(entry)
            .insert(&txn)
            .await
            .map_err(|error| DomainError::Internal(error.to_string()))?;
        insert_journal_lines(&txn, entry).await?;
        txn.commit()
            .await
            .map_err(|error| DomainError::Internal(error.to_string()))
    }

    async fn save_fee_invoice(
        &self,
        fee: &DomainStudentFee,
        entry: &DomainJournalEntry,
    ) -> Result<(), DomainError> {
        entry.validate_balance().map_err(DomainError::Validation)?;
        let txn = self
            .db
            .begin()
            .await
            .map_err(|error| DomainError::Internal(error.to_string()))?;
        fee_model(fee)
            .insert(&txn)
            .await
            .map_err(|error| DomainError::Internal(error.to_string()))?;
        journal_header(entry)
            .insert(&txn)
            .await
            .map_err(|error| DomainError::Internal(error.to_string()))?;
        insert_journal_lines(&txn, entry).await?;
        txn.commit()
            .await
            .map_err(|error| DomainError::Internal(error.to_string()))
    }

    async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        StudentFeeEntity::delete_by_id(id)
            .exec(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(())
    }
}

async fn delete_unreconciled_journal(
    db: &impl ConnectionTrait,
    journal_id: Uuid,
    org_id: Uuid,
) -> Result<(), DomainError> {
    JournalEntryLineEntity::delete_many()
        .filter(journal_entry_line::Column::JournalEntryId.eq(journal_id))
        .exec(db)
        .await
        .map_err(|error| DomainError::Internal(error.to_string()))?;
    let deleted = JournalEntryEntity::delete_many()
        .filter(journal_entry::Column::Id.eq(journal_id))
        .filter(journal_entry::Column::OrgId.eq(org_id))
        .filter(
            Condition::any()
                .add(journal_entry::Column::IsReversed.eq(false))
                .add(journal_entry::Column::IsReversed.is_null()),
        )
        .filter(
            Condition::any()
                .add(journal_entry::Column::Status.ne("reconciled"))
                .add(journal_entry::Column::Status.is_null()),
        )
        .exec(db)
        .await
        .map_err(|error| DomainError::Internal(error.to_string()))?;
    if deleted.rows_affected != 1 {
        return Err(DomainError::ConcurrencyConflict(
            "Journal is missing, reversed or reconciled".into(),
        ));
    }
    Ok(())
}

async fn update_fee_balance(
    db: &impl ConnectionTrait,
    previous: &DomainStudentFee,
    updated: &DomainStudentFee,
) -> Result<(), DomainError> {
    let paid_amount_filter = if previous.paid_amount == Decimal::ZERO {
        Condition::any()
            .add(student_fee::Column::PaidAmount.eq(Some(Decimal::ZERO)))
            .add(student_fee::Column::PaidAmount.is_null())
    } else {
        Condition::all().add(student_fee::Column::PaidAmount.eq(Some(previous.paid_amount)))
    };
    let balance_due_filter = if previous.balance_due == previous.net_amount {
        Condition::any()
            .add(student_fee::Column::BalanceDue.eq(Some(previous.balance_due)))
            .add(student_fee::Column::BalanceDue.is_null())
    } else {
        Condition::all().add(student_fee::Column::BalanceDue.eq(Some(previous.balance_due)))
    };
    let update = StudentFeeEntity::update_many()
        .col_expr(
            student_fee::Column::PaidAmount,
            Expr::value(Some(updated.paid_amount)),
        )
        .col_expr(
            student_fee::Column::BalanceDue,
            Expr::value(Some(updated.balance_due)),
        )
        .col_expr(
            student_fee::Column::Status,
            Expr::value(updated.status.to_string()),
        )
        .col_expr(
            student_fee::Column::Notes,
            Expr::value(updated.notes.clone()),
        )
        .col_expr(
            student_fee::Column::UpdatedAt,
            Expr::value(Some(updated.updated_at)),
        )
        .filter(student_fee::Column::Id.eq(previous.id))
        .filter(student_fee::Column::OrgId.eq(previous.org_id))
        .filter(paid_amount_filter)
        .filter(balance_due_filter)
        .exec(db)
        .await
        .map_err(|error| DomainError::Internal(error.to_string()))?;

    if update.rows_affected != 1 {
        return Err(DomainError::ConcurrencyConflict(
            "Student fee changed while the payment was being processed".into(),
        ));
    }
    Ok(())
}

#[async_trait]
impl PaymentRepository for FinanceRepoImpl {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<DomainPayment>, DomainError> {
        let model = PaymentEntity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(model.map(map_payment_model_to_domain))
    }

    async fn find_by_org(&self, org_id: Uuid) -> Result<Vec<DomainPayment>, DomainError> {
        let models = PaymentEntity::find()
            .filter(payment::Column::OrgId.eq(org_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(models
            .into_iter()
            .map(map_payment_model_to_domain)
            .collect())
    }

    async fn find_by_student(&self, student_id: Uuid) -> Result<Vec<DomainPayment>, DomainError> {
        let models = PaymentEntity::find()
            .filter(payment::Column::StudentId.eq(student_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(models
            .into_iter()
            .map(map_payment_model_to_domain)
            .collect())
    }

    async fn list(
        &self,
        org_id: Uuid,
        filter: orbis_domain::queries::filters::PaymentFilter,
    ) -> Result<orbis_domain::queries::filters::PaginatedList<DomainPayment>, DomainError> {
        let mut query = build_payment_list_query(org_id, &filter, self.db.get_database_backend());
        let pagination = filter.pagination.unwrap_or_default();
        let order = if pagination
            .sort_dir
            .as_deref()
            .is_some_and(|direction| direction.eq_ignore_ascii_case("desc"))
        {
            Order::Desc
        } else {
            Order::Asc
        };
        query = match pagination.sort_by.as_deref() {
            Some("paid_at") => query.order_by(payment::Column::PaidAt, order),
            Some("amount") => query.order_by(payment::Column::Amount, order),
            Some("method") => query.order_by(payment::Column::Method, order),
            Some("receipt_number") => query.order_by(payment::Column::ReceiptNumber, order),
            Some("currency_code") => query.order_by(payment::Column::CurrencyCode, order),
            Some("created_at") => query.order_by(payment::Column::CreatedAt, order),
            _ => query.order_by_desc(payment::Column::PaidAt),
        }
        .order_by_desc(payment::Column::CreatedAt)
        .order_by_asc(payment::Column::Id);
        let page = pagination.page();
        let page_size = pagination.page_size();
        let paginator = query.paginate(&self.db, page_size);
        let total_count = paginator
            .num_items()
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        let rows = paginator
            .fetch_page(page - 1)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        let items = rows.into_iter().map(map_payment_model_to_domain).collect();
        Ok(orbis_domain::queries::filters::PaginatedList::new(
            items,
            total_count,
            page,
            page_size,
        ))
    }

    async fn save(&self, p: &DomainPayment) -> Result<(), DomainError> {
        let active = payment::ActiveModel {
            id: Set(p.id),
            org_id: Set(p.org_id),
            fee_id: Set(p.fee_id),
            enrollment_id: Set(p.enrollment_id),
            student_id: Set(p.student_id),
            account_id: Set(p.account_id),
            amount: Set(p.amount),
            currency_code: Set(Some(p.currency_code.clone())),
            exchange_rate: Set(Some(p.exchange_rate)),
            base_amount: Set(Some(p.base_amount)),
            method: Set(p.method.to_string()),
            reference: Set(p.reference.clone()),
            receipt_number: Set(p.receipt_number.clone()),
            journal_entry_id: Set(p.journal_entry_id),
            payer_name: Set(p.payer_name.clone()),
            payer_phone: Set(p.payer_phone.clone()),
            notes: Set(p.notes.clone()),
            paid_at: Set(p.paid_at.into()),
            created_by: Set(p.created_by),
            org_unit_id: Set(p.org_unit_id),
            created_at: Set(p.created_at.into()),
            updated_at: Set(Some(p.updated_at.into())),
        };

        if PaymentEntity::find_by_id(p.id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?
            .is_some()
        {
            active
                .update(&self.db)
                .await
                .map_err(|e| DomainError::Internal(e.to_string()))?;
        } else {
            active
                .insert(&self.db)
                .await
                .map_err(|e| DomainError::Internal(e.to_string()))?;
        }
        Ok(())
    }

    async fn delete_payment_posting(
        &self,
        payment: &DomainPayment,
        fee_update: Option<(&DomainStudentFee, &DomainStudentFee)>,
    ) -> Result<(), DomainError> {
        let txn = self
            .db
            .begin()
            .await
            .map_err(|error| DomainError::Internal(error.to_string()))?;
        let deleted = PaymentEntity::delete_many()
            .filter(payment::Column::Id.eq(payment.id))
            .filter(payment::Column::OrgId.eq(payment.org_id))
            .exec(&txn)
            .await
            .map_err(|error| DomainError::Internal(error.to_string()))?;
        if deleted.rows_affected != 1 {
            return Err(DomainError::ConcurrencyConflict(
                "Payment was already removed".into(),
            ));
        }
        if let Some((previous, updated)) = fee_update {
            update_fee_balance(&txn, previous, updated).await?;
        }
        if JournalEntryEntity::find()
            .filter(journal_entry::Column::OrgId.eq(payment.org_id))
            .filter(journal_entry::Column::SourceId.eq(payment.id))
            .filter(journal_entry::Column::SourceType.eq("student_fee_refund"))
            .one(&txn)
            .await
            .map_err(|error| DomainError::Internal(error.to_string()))?
            .is_some()
        {
            return Err(DomainError::Validation(
                "A payment with refunds cannot be deleted".into(),
            ));
        }
        if let Some(journal_id) = payment.journal_entry_id {
            delete_unreconciled_journal(&txn, journal_id, payment.org_id).await?;
        }
        txn.commit()
            .await
            .map_err(|error| DomainError::Internal(error.to_string()))
    }

    async fn save_payment_posting(
        &self,
        payment_record: &DomainPayment,
        entry: &DomainJournalEntry,
        fee_update: Option<(&DomainStudentFee, &DomainStudentFee)>,
    ) -> Result<(), DomainError> {
        entry.validate_balance().map_err(DomainError::Validation)?;
        let txn = self
            .db
            .begin()
            .await
            .map_err(|error| DomainError::Internal(error.to_string()))?;

        let result = async {
            if let Some((previous, updated)) = fee_update {
                update_fee_balance(&txn, previous, updated).await?;
            }

            journal_header(entry)
                .insert(&txn)
                .await
                .map_err(|error| DomainError::Internal(error.to_string()))?;

            insert_journal_lines(&txn, entry).await?;

            let payment_active = payment::ActiveModel {
                id: Set(payment_record.id),
                org_id: Set(payment_record.org_id),
                fee_id: Set(payment_record.fee_id),
                enrollment_id: Set(payment_record.enrollment_id),
                student_id: Set(payment_record.student_id),
                account_id: Set(payment_record.account_id),
                amount: Set(payment_record.amount),
                currency_code: Set(Some(payment_record.currency_code.clone())),
                exchange_rate: Set(Some(payment_record.exchange_rate)),
                base_amount: Set(Some(payment_record.base_amount)),
                method: Set(payment_record.method.to_string()),
                reference: Set(payment_record.reference.clone()),
                receipt_number: Set(payment_record.receipt_number.clone()),
                journal_entry_id: Set(payment_record.journal_entry_id),
                payer_name: Set(payment_record.payer_name.clone()),
                payer_phone: Set(payment_record.payer_phone.clone()),
                notes: Set(payment_record.notes.clone()),
                paid_at: Set(payment_record.paid_at.into()),
                created_by: Set(payment_record.created_by),
                org_unit_id: Set(payment_record.org_unit_id),
                created_at: Set(payment_record.created_at.into()),
                updated_at: Set(Some(payment_record.updated_at.into())),
            };
            payment_active
                .insert(&txn)
                .await
                .map_err(|error| DomainError::Internal(error.to_string()))?;

            Ok::<(), DomainError>(())
        }
        .await;

        match result {
            Ok(()) => txn
                .commit()
                .await
                .map_err(|error| DomainError::Internal(error.to_string())),
            Err(error) => {
                if let Err(rollback_error) = txn.rollback().await {
                    return Err(DomainError::Internal(format!(
                        "{error}; transaction rollback failed: {rollback_error}"
                    )));
                }
                Err(error)
            }
        }
    }

    async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        PaymentEntity::delete_by_id(id)
            .exec(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(())
    }
}

#[async_trait]
impl CurrencyRepository for FinanceRepoImpl {
    async fn find_by_code(
        &self,
        _org_id: Uuid,
        code: &str,
    ) -> Result<Option<DomainCurrency>, DomainError> {
        let base_currency_row = self
            .db
            .query_one(sea_orm::Statement::from_sql_and_values(
                self.db.get_database_backend(),
                "SELECT base_currency_code FROM organizations WHERE id = $1 LIMIT 1",
                vec![_org_id.into()],
            ))
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        let base_code = base_currency_row
            .and_then(|row| row.try_get("", "base_currency_code").ok())
            .unwrap_or_else(|| "YER".to_string());

        let model = CurrencyEntity::find_by_id(code.to_string())
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(model.map(|m| DomainCurrency {
            code: m.id.clone(),
            org_id: _org_id,
            name: m.name,
            name_ar: m.name_ar,
            symbol: m.symbol,
            symbol_position: m.symbol_position,
            decimal_places: m.decimal_places as u32,
            is_base: m.id.eq_ignore_ascii_case(&base_code),
            is_active: m.is_active,
            updated_at: m.created_at,
        }))
    }

    async fn find_by_org(&self, _org_id: Uuid) -> Result<Vec<DomainCurrency>, DomainError> {
        let base_currency_row = self
            .db
            .query_one(sea_orm::Statement::from_sql_and_values(
                self.db.get_database_backend(),
                "SELECT base_currency_code FROM organizations WHERE id = $1 LIMIT 1",
                vec![_org_id.into()],
            ))
            .await
            .ok()
            .flatten();

        let base_code: String = base_currency_row
            .and_then(|r| r.try_get("", "base_currency_code").ok())
            .unwrap_or_else(|| "YER".into());

        let models = CurrencyEntity::find()
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(models
            .into_iter()
            .map(|m| {
                let is_base = m.id.to_uppercase() == base_code.to_uppercase();
                DomainCurrency {
                    code: m.id.clone(),
                    org_id: _org_id,
                    name: m.name,
                    name_ar: m.name_ar,
                    symbol: m.symbol,
                    symbol_position: m.symbol_position,
                    decimal_places: m.decimal_places as u32,
                    is_base,
                    is_active: is_base || m.is_active,
                    updated_at: m.created_at,
                }
            })
            .collect())
    }

    async fn save(&self, c: &DomainCurrency) -> Result<(), DomainError> {
        let active = currency::ActiveModel {
            id: Set(c.code.clone()),
            name: Set(c.name.clone()),
            name_ar: Set(c.name_ar.clone()),
            symbol: Set(c.symbol.clone()),
            symbol_position: Set(c.symbol_position.clone()),
            decimal_places: Set(c.decimal_places as i32),
            is_active: Set(c.is_active),
            created_at: Set(c.updated_at),
        };

        if CurrencyEntity::find_by_id(c.code.clone())
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?
            .is_some()
        {
            active
                .update(&self.db)
                .await
                .map_err(|e| DomainError::Internal(e.to_string()))?;
        } else {
            active
                .insert(&self.db)
                .await
                .map_err(|e| DomainError::Internal(e.to_string()))?;
        }
        Ok(())
    }

    async fn get_rate(
        &self,
        from_id: Uuid,
        _to_id: Uuid,
    ) -> Result<Option<DomainExchangeRate>, DomainError> {
        let model = ExchangeRateEntity::find_by_id(from_id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(model.map(|m| DomainExchangeRate {
            id: m.id,
            org_id: m.org_id,
            from_currency: m.base_currency_id,
            to_currency: m.target_currency_id,
            rate: m.rate,
            effective_date: m.effective_date,
            created_at: m.created_at,
        }))
    }

    async fn get_exchange_rate(
        &self,
        org_id: Uuid,
        from: &str,
        to: &str,
    ) -> Result<Option<DomainExchangeRate>, DomainError> {
        let model = ExchangeRateEntity::find()
            .filter(exchange_rate::Column::OrgId.eq(org_id))
            .filter(exchange_rate::Column::BaseCurrencyId.eq(from))
            .filter(exchange_rate::Column::TargetCurrencyId.eq(to))
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(model.map(|m| DomainExchangeRate {
            id: m.id,
            org_id: m.org_id,
            from_currency: m.base_currency_id,
            to_currency: m.target_currency_id,
            rate: m.rate,
            effective_date: m.effective_date,
            created_at: m.created_at,
        }))
    }

    async fn save_rate(&self, rate: &DomainExchangeRate) -> Result<(), DomainError> {
        let active = exchange_rate::ActiveModel {
            id: Set(rate.id),
            org_id: Set(rate.org_id),
            base_currency_id: Set(rate.from_currency.clone()),
            target_currency_id: Set(rate.to_currency.clone()),
            rate: Set(rate.rate),
            effective_date: Set(rate.effective_date),
            created_at: Set(rate.created_at),
        };

        if ExchangeRateEntity::find_by_id(rate.id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?
            .is_some()
        {
            active
                .update(&self.db)
                .await
                .map_err(|e| DomainError::Internal(e.to_string()))?;
        } else {
            active
                .insert(&self.db)
                .await
                .map_err(|e| DomainError::Internal(e.to_string()))?;
        }
        Ok(())
    }
}

fn map_student_fee_model_to_domain(m: student_fee::Model) -> DomainStudentFee {
    DomainStudentFee {
        id: m.id,
        org_id: m.org_id,
        period_id: m.period_id.unwrap_or_default(),
        student_id: m.student_id,
        enrollment_id: m.enrollment_id,
        fee_template_id: m.fee_template_id,
        original_amount: m.original_amount,
        discount_amount: m.discount_amount,
        net_amount: m.net_amount,
        paid_amount: m.paid_amount.unwrap_or(Decimal::ZERO),
        balance_due: m.balance_due.unwrap_or(m.net_amount),
        status: match m.status.as_str() {
            "partially_paid" => FeeStatus::PartiallyPaid,
            "paid" => FeeStatus::Paid,
            "overdue" => FeeStatus::Overdue,
            "cancelled" => FeeStatus::Cancelled,
            "refunded" => FeeStatus::Refunded,
            _ => FeeStatus::Pending,
        },
        due_date: m.due_date,
        discount_details: m.discount_details,
        installment_number: m.installment_number.map(|n| n as u32),
        notes: m.notes,
        created_by: m.created_by,
        created_at: m.created_at.into(),
        updated_at: m.updated_at.into(),
    }
}

fn map_payment_model_to_domain(m: payment::Model) -> DomainPayment {
    DomainPayment {
        id: m.id,
        org_id: m.org_id,
        fee_id: m.fee_id,
        enrollment_id: m.enrollment_id,
        student_id: m.student_id,
        account_id: m.account_id,
        amount: m.amount,
        currency_code: m.currency_code.unwrap_or_else(|| "SAR".into()),
        exchange_rate: m.exchange_rate.unwrap_or(Decimal::ONE),
        base_amount: m.base_amount.unwrap_or(m.amount),
        method: match m.method.to_lowercase().as_str() {
            "bank_transfer" => PaymentMethod::BankTransfer,
            "check" => PaymentMethod::Check,
            "credit_card" => PaymentMethod::CreditCard,
            "mada" => PaymentMethod::Mada,
            "apple_pay" => PaymentMethod::ApplePay,
            _ => PaymentMethod::Cash,
        },
        reference: m.reference,
        receipt_number: m.receipt_number,
        journal_entry_id: m.journal_entry_id,
        payer_name: m.payer_name,
        payer_phone: m.payer_phone,
        notes: m.notes,
        paid_at: m.paid_at.into(),
        created_by: m.created_by,
        created_at: m.created_at.into(),
        updated_at: m
            .updated_at
            .map(Into::into)
            .unwrap_or_else(|| m.created_at.into()),
        org_unit_id: m.org_unit_id,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use sea_orm::{Database, QueryTrait, Schema};

    fn test_fee(org_id: Uuid, now: chrono::DateTime<Utc>) -> DomainStudentFee {
        DomainStudentFee {
            id: Uuid::new_v4(),
            org_id,
            period_id: Uuid::new_v4(),
            student_id: Uuid::new_v4(),
            enrollment_id: None,
            fee_template_id: Uuid::new_v4(),
            original_amount: Decimal::new(10000, 2),
            discount_amount: Decimal::ZERO,
            net_amount: Decimal::new(10000, 2),
            paid_amount: Decimal::ZERO,
            balance_due: Decimal::new(10000, 2),
            status: FeeStatus::Pending,
            due_date: chrono::NaiveDate::from_ymd_opt(2026, 12, 31).unwrap(),
            discount_details: None,
            installment_number: None,
            notes: None,
            created_by: None,
            created_at: now,
            updated_at: now,
        }
    }

    fn test_entry(org_id: Uuid, id: Uuid, now: chrono::DateTime<Utc>) -> DomainJournalEntry {
        DomainJournalEntry {
            id,
            org_id,
            reference_number: format!("PAY-{}", id),
            accounting_date: now.date_naive(),
            amount: Decimal::new(4000, 2),
            currency_code: "SAR".into(),
            exchange_rate: Decimal::ONE,
            base_amount: Decimal::new(4000, 2),
            debit_account_id: Uuid::new_v4(),
            credit_account_id: Uuid::new_v4(),
            description: "Test payment".into(),
            description_ar: None,
            source_type: "student_payment".into(),
            source_id: Some(id),
            status: JournalPostingStatus::Posted,
            lines: Vec::new(),
            is_reversed: false,
            reversed_by: None,
            reversal_reason: None,
            created_by: None,
            created_at: now,
            updated_at: now,
            updated_by: None,
            org_unit_id: None,
            version: 1,
        }
    }

    fn test_payment(
        org_id: Uuid,
        fee_id: Uuid,
        id: Uuid,
        journal_id: Uuid,
        now: chrono::DateTime<Utc>,
    ) -> DomainPayment {
        DomainPayment {
            id,
            org_id,
            fee_id: Some(fee_id),
            enrollment_id: None,
            student_id: None,
            account_id: Uuid::new_v4(),
            amount: Decimal::new(4000, 2),
            currency_code: "SAR".into(),
            exchange_rate: Decimal::ONE,
            base_amount: Decimal::new(4000, 2),
            method: PaymentMethod::Cash,
            reference: None,
            receipt_number: None,
            journal_entry_id: Some(journal_id),
            payer_name: None,
            payer_phone: None,
            notes: None,
            paid_at: now,
            created_by: None,
            created_at: now,
            updated_at: now,
            org_unit_id: None,
        }
    }

    async fn create_payment_tables(db: &DatabaseConnection) {
        let schema = Schema::new(db.get_database_backend());
        for statement in [
            schema
                .create_table_from_entity(StudentFeeEntity)
                .if_not_exists()
                .to_owned(),
            schema
                .create_table_from_entity(JournalEntryEntity)
                .if_not_exists()
                .to_owned(),
            schema
                .create_table_from_entity(JournalEntryLineEntity)
                .if_not_exists()
                .to_owned(),
            schema
                .create_table_from_entity(PaymentEntity)
                .if_not_exists()
                .to_owned(),
        ] {
            db.execute(db.get_database_backend().build(&statement))
                .await
                .unwrap();
        }
    }

    #[test]
    fn student_fee_query_applies_organization_period_and_unpaid_filters() {
        let org_id = Uuid::new_v4();
        let period_id = Uuid::new_v4();
        let org_unit_id = Uuid::new_v4();
        let filter = orbis_domain::queries::filters::StudentFeeFilter {
            period_id: Some(period_id),
            org_unit_id: Some(org_unit_id),
            currency_code: Some("SAR".to_string()),
            from_date: Some(chrono::NaiveDate::from_ymd_opt(2026, 11, 1).unwrap()),
            to_date: Some(chrono::NaiveDate::from_ymd_opt(2026, 11, 30).unwrap()),
            only_unpaid: Some(true),
            ..Default::default()
        };

        for backend in [
            sea_orm::DatabaseBackend::Postgres,
            sea_orm::DatabaseBackend::Sqlite,
        ] {
            let statement = build_student_fee_list_query(org_id, &filter, backend).build(backend);
            let values = format!("{:?}", statement.values);
            assert!(statement.sql.contains("org_id"));
            assert!(statement.sql.contains("period_id"));
            assert!(statement.sql.contains("due_date"));
            assert!(statement.sql.contains("org_unit_id"));
            assert!(statement.sql.contains("fee_templates"));
            assert!(statement.sql.contains("balance_due"));
            assert!(values.contains("partially_paid"));
            assert!(values.contains(&org_id.to_string()));
            assert!(values.contains(&period_id.to_string()));
            assert!(values.contains(&org_unit_id.to_string()));
            assert!(values.contains("SAR"));
        }
    }

    #[test]
    fn student_fee_query_searches_student_and_fee_template_fields() {
        let org_id = Uuid::new_v4();
        let filter = orbis_domain::queries::filters::StudentFeeFilter {
            search: Some("Rana".to_string()),
            ..Default::default()
        };

        for backend in [
            sea_orm::DatabaseBackend::Postgres,
            sea_orm::DatabaseBackend::Sqlite,
        ] {
            let statement = build_student_fee_list_query(org_id, &filter, backend).build(backend);
            let values = format!("{:?}", statement.values);
            assert!(statement.sql.contains("student_fees"));
            assert!(statement.sql.contains("students"));
            assert!(statement.sql.contains("fee_templates"));
            assert!(values.contains("%Rana%"), "query values: {values}");
            assert!(values.contains(&org_id.to_string()));
        }
    }

    #[test]
    fn payment_query_applies_search_and_method_filters_before_pagination() {
        let org_id = Uuid::new_v4();
        let org_unit_id = Uuid::new_v4();
        let filter = orbis_domain::queries::filters::PaymentFilter {
            org_unit_id: Some(org_unit_id),
            currency_code: Some("SAR".to_string()),
            method: Some(PaymentMethod::Cash),
            search: Some("receipt-7".to_string()),
            ..Default::default()
        };

        for backend in [
            sea_orm::DatabaseBackend::Postgres,
            sea_orm::DatabaseBackend::Sqlite,
        ] {
            let statement = build_payment_list_query(org_id, &filter, backend).build(backend);
            let values = format!("{:?}", statement.values);

            assert!(statement.sql.contains("payments.method"));
            assert!(statement.sql.contains("org_unit_id"));
            assert!(statement.sql.contains("currency_code"));
            assert!(statement.sql.contains("payments.receipt_number"));
            assert!(statement.sql.contains("students"));
            assert!(values.contains("cash"));
            assert!(values.contains("%receipt-7%"), "query values: {values}");
            assert!(values.contains(&org_unit_id.to_string()));
            assert!(values.contains("SAR"));
            assert!(values.contains(&org_id.to_string()));
        }
    }

    #[tokio::test]
    async fn payment_day_filter_includes_local_midnight_and_fractional_last_second() {
        let db = Database::connect("sqlite::memory:").await.unwrap();
        create_payment_tables(&db).await;
        let repo = FinanceRepoImpl { db };
        let org_id = Uuid::new_v4();
        let start = "2026-11-24T21:00:00Z"
            .parse::<chrono::DateTime<Utc>>()
            .unwrap();
        let end = start + chrono::Duration::days(1);
        let entries = [
            (start - chrono::Duration::seconds(1), false),
            (start, true),
            (end - chrono::Duration::milliseconds(100), true),
            (end, false),
        ];
        let mut expected = Vec::new();
        for (paid_at, included) in entries {
            let id = Uuid::new_v4();
            let mut payment = test_payment(org_id, Uuid::new_v4(), id, Uuid::new_v4(), paid_at);
            payment.fee_id = None;
            payment.journal_entry_id = None;
            PaymentRepository::save(&repo, &payment).await.unwrap();
            if included {
                expected.push(id);
            }
        }
        expected.reverse();
        let page = PaymentRepository::list(
            &repo,
            org_id,
            orbis_domain::queries::filters::PaymentFilter {
                from_date: Some(start),
                to_date_exclusive: Some(end),
                ..Default::default()
            },
        )
        .await
        .unwrap();
        assert_eq!(
            page.items
                .into_iter()
                .map(|payment| payment.id)
                .collect::<Vec<_>>(),
            expected
        );
    }

    #[tokio::test]
    async fn student_fee_summary_aggregates_by_currency_in_database() {
        let db = Database::connect("sqlite::memory:").await.unwrap();
        db.execute_unprepared(
            "CREATE TABLE student_fees (
                org_id TEXT NOT NULL,
                student_id TEXT NOT NULL,
                fee_template_id TEXT NOT NULL,
                due_date TEXT NOT NULL,
                net_amount DECIMAL NOT NULL,
                paid_amount DECIMAL,
                balance_due DECIMAL
            )",
        )
        .await
        .unwrap();
        db.execute_unprepared(
            "CREATE TABLE fee_templates (
                id TEXT NOT NULL,
                org_id TEXT NOT NULL,
                currency_code TEXT NOT NULL
            )",
        )
        .await
        .unwrap();
        db.execute_unprepared(
            "CREATE TABLE students (
            id TEXT NOT NULL,
            org_id TEXT NOT NULL,
            org_unit_id TEXT
        )",
        )
        .await
        .unwrap();

        let org_id = Uuid::new_v4();
        let other_org_id = Uuid::new_v4();
        let branch_one_id = Uuid::new_v4();
        let branch_two_id = Uuid::new_v4();
        let yer_template_id = Uuid::new_v4();
        let sar_template_id = Uuid::new_v4();
        let students = [
            (Uuid::new_v4(), org_id, branch_one_id),
            (Uuid::new_v4(), org_id, branch_two_id),
            (Uuid::new_v4(), org_id, branch_one_id),
            (Uuid::new_v4(), other_org_id, branch_one_id),
        ];
        let backend = db.get_database_backend();
        for (student_id, student_org_id, org_unit_id) in students {
            db.execute(Statement::from_sql_and_values(
                backend,
                "INSERT INTO students (id, org_id, org_unit_id) VALUES (?, ?, ?)",
                vec![student_id.into(), student_org_id.into(), org_unit_id.into()],
            ))
            .await
            .unwrap();
        }
        for (template_id, currency) in [(yer_template_id, "YER"), (sar_template_id, "SAR")] {
            db.execute(Statement::from_sql_and_values(
                backend,
                "INSERT INTO fee_templates (id, org_id, currency_code) VALUES (?, ?, ?)",
                vec![template_id.into(), org_id.into(), currency.into()],
            ))
            .await
            .unwrap();
        }
        for (fee_org_id, student, template_id, date, due, paid, balance) in [
            (
                org_id,
                students[0].0,
                yer_template_id,
                "2026-01-15",
                100,
                25,
                75,
            ),
            (
                org_id,
                students[1].0,
                yer_template_id,
                "2026-02-15",
                50,
                10,
                40,
            ),
            (
                org_id,
                students[2].0,
                sar_template_id,
                "2026-02-20",
                200,
                80,
                120,
            ),
            (
                other_org_id,
                students[3].0,
                yer_template_id,
                "2026-01-10",
                1000,
                0,
                1000,
            ),
        ] {
            db.execute(Statement::from_sql_and_values(
                backend,
                "INSERT INTO student_fees
                    (org_id, student_id, fee_template_id, due_date, net_amount, paid_amount, balance_due)
                    VALUES (?, ?, ?, ?, ?, ?, ?)",
                vec![
                    fee_org_id.into(),
                    student.into(),
                    template_id.into(),
                    date.into(),
                    Decimal::from(due).into(),
                    Decimal::from(paid).into(),
                    Decimal::from(balance).into(),
                ],
            ))
            .await
            .unwrap();
        }

        let repo = FinanceRepoImpl { db };
        let summary = repo
            .summarize_by_currency(
                org_id,
                orbis_domain::queries::filters::StudentFeeFilter::default(),
            )
            .await
            .unwrap();

        assert_eq!(summary.len(), 2);
        assert_eq!(summary[0].currency, "SAR");
        assert_eq!(summary[0].due, Decimal::from(200));
        assert_eq!(summary[0].paid, Decimal::from(80));
        assert_eq!(summary[0].balance, Decimal::from(120));
        assert_eq!(summary[1].currency, "YER");
        assert_eq!(summary[1].due, Decimal::from(150));
        assert_eq!(summary[1].paid, Decimal::from(35));
        assert_eq!(summary[1].balance, Decimal::from(115));

        let branch_summary = repo
            .summarize_by_currency(
                org_id,
                orbis_domain::queries::filters::StudentFeeFilter {
                    org_unit_id: Some(branch_one_id),
                    from_date: Some(chrono::NaiveDate::from_ymd_opt(2026, 1, 1).unwrap()),
                    to_date: Some(chrono::NaiveDate::from_ymd_opt(2026, 1, 31).unwrap()),
                    ..Default::default()
                },
            )
            .await
            .unwrap();
        assert_eq!(branch_summary.len(), 1);
        assert_eq!(branch_summary[0].currency, "YER");
        assert_eq!(branch_summary[0].due, Decimal::from(100));
        assert_eq!(branch_summary[0].paid, Decimal::from(25));
        assert_eq!(branch_summary[0].balance, Decimal::from(75));
    }

    #[tokio::test]
    async fn journal_lines_and_reversal_are_committed_together() {
        let db = Database::connect("sqlite::memory:").await.unwrap();
        create_payment_tables(&db).await;
        let repo = FinanceRepoImpl { db };
        let now = Utc::now();
        let org_id = Uuid::new_v4();
        let mut original = test_entry(org_id, Uuid::new_v4(), now);
        let debit_a = original.debit_account_id;
        let debit_b = Uuid::new_v4();
        let credit = original.credit_account_id;
        let line = |account_id, debit_amount, credit_amount| JournalEntryLine {
            id: Uuid::new_v4(),
            account_id,
            debit_amount,
            credit_amount,
            description: None,
            entity_type: None,
            entity_id: None,
        };
        original.lines = vec![
            line(debit_a, Decimal::ONE, Decimal::ZERO),
            line(debit_b, original.amount - Decimal::ONE, Decimal::ZERO),
            line(credit, Decimal::ZERO, original.amount),
        ];
        JournalEntryRepository::save(&repo, &original)
            .await
            .unwrap();
        let stored = JournalEntryRepository::find_by_id(&repo, original.id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(stored.lines.len(), 3);
        let balances = repo
            .account_balances_at(org_id, now.date_naive())
            .await
            .unwrap();
        assert_eq!(balances[&debit_a]["SAR"], Decimal::ONE);
        assert_eq!(balances[&credit]["SAR"], -original.amount);

        let mut reversal = original.clone();
        reversal.lines = original
            .lines
            .iter()
            .map(|item| line(item.account_id, item.credit_amount, item.debit_amount))
            .collect();
        assert!(
            repo.save_reversal(&reversal, original.id, "duplicate ID")
                .await
                .is_err()
        );
        let stored = JournalEntryRepository::find_by_id(&repo, original.id)
            .await
            .unwrap()
            .unwrap();
        assert!(!stored.is_reversed);

        reversal.id = Uuid::new_v4();
        repo.save_reversal(&reversal, original.id, "test")
            .await
            .unwrap();
        assert!(
            repo.save_reversal(&reversal, original.id, "second reversal")
                .await
                .is_err()
        );
        let entries = JournalEntryRepository::find_by_org(&repo, org_id)
            .await
            .unwrap();
        assert_eq!(entries.len(), 2);
        let balances = repo
            .account_balances_at(org_id, now.date_naive())
            .await
            .unwrap();
        assert!(
            balances
                .values()
                .flatten()
                .all(|(_, amount)| amount.is_zero())
        );
        assert!(
            entries
                .iter()
                .any(|entry| entry.id == original.id && entry.is_reversed)
        );
        assert!(
            entries
                .iter()
                .any(|entry| entry.id == reversal.id && entry.lines.len() == 3)
        );
    }

    #[tokio::test]
    async fn fee_invoice_and_installments_reject_partial_writes() {
        let db = Database::connect("sqlite::memory:").await.unwrap();
        create_payment_tables(&db).await;
        let repo = FinanceRepoImpl { db };
        let now = Utc::now();
        let org_id = Uuid::new_v4();
        let mut invoice = test_entry(org_id, Uuid::new_v4(), now);
        let fee = test_fee(org_id, now);
        invoice.source_type = "student_fee_invoice".into();
        invoice.source_id = Some(fee.id);
        repo.save_fee_invoice(&fee, &invoice).await.unwrap();

        let duplicate = test_fee(org_id, now);
        assert!(repo.save_fee_invoice(&duplicate, &invoice).await.is_err());
        assert!(
            StudentFeeRepository::find_by_id(&repo, duplicate.id)
                .await
                .unwrap()
                .is_none()
        );

        let mut first = fee.clone();
        first.installment_number = Some(1);
        first.net_amount = fee.net_amount / Decimal::from(2);
        first.balance_due = first.net_amount;
        let mut second = first.clone();
        second.installment_number = Some(2);
        assert!(
            repo.save_fee_installments(&fee, &[first.clone(), second.clone()])
                .await
                .is_err()
        );
        let stored = StudentFeeRepository::find_by_id(&repo, fee.id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(stored.net_amount, fee.net_amount);
        assert_eq!(stored.installment_number, None);
        second.id = Uuid::new_v4();
        repo.save_fee_installments(&fee, &[first, second])
            .await
            .unwrap();
        assert_eq!(
            StudentFeeRepository::find_by_org(&repo, org_id)
                .await
                .unwrap()
                .len(),
            2
        );
    }

    #[tokio::test]
    async fn fee_refund_rolls_back_when_the_journal_cannot_be_written() {
        let db = Database::connect("sqlite::memory:").await.unwrap();
        create_payment_tables(&db).await;
        let repo = FinanceRepoImpl { db };
        let now = Utc::now();
        let org_id = Uuid::new_v4();
        let mut paid_fee = test_fee(org_id, now);
        paid_fee.apply_payment(Decimal::from(40));
        StudentFeeRepository::save(&repo, &paid_fee).await.unwrap();
        let mut refund = test_entry(org_id, Uuid::new_v4(), now);
        JournalEntryRepository::save(&repo, &refund).await.unwrap();
        let mut updated = paid_fee.clone();
        updated.paid_amount = Decimal::from(20);
        updated.balance_due = updated.net_amount - updated.paid_amount;
        assert!(
            repo.save_fee_refund(&paid_fee, &updated, &refund)
                .await
                .is_err()
        );
        let stored = StudentFeeRepository::find_by_id(&repo, paid_fee.id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(stored.paid_amount, paid_fee.paid_amount);
        refund.id = Uuid::new_v4();
        repo.save_fee_refund(&paid_fee, &updated, &refund)
            .await
            .unwrap();
        let stored = StudentFeeRepository::find_by_id(&repo, paid_fee.id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(stored.paid_amount, updated.paid_amount);
    }

    #[tokio::test]
    async fn payment_posting_rolls_back_as_a_unit_and_rejects_stale_fee_balance() {
        let db = Database::connect("sqlite::memory:").await.unwrap();
        create_payment_tables(&db).await;
        let repo = FinanceRepoImpl { db: db.clone() };
        let now = Utc::now();
        let org_id = Uuid::new_v4();
        let fee = test_fee(org_id, now);
        StudentFeeRepository::save(&repo, &fee).await.unwrap();

        let failed_entry = test_entry(org_id, Uuid::new_v4(), now);
        let failed_payment = test_payment(org_id, fee.id, Uuid::new_v4(), Uuid::new_v4(), now);
        PaymentRepository::save(&repo, &failed_payment)
            .await
            .unwrap();
        let mut updated_fee = fee.clone();
        updated_fee.apply_payment(failed_payment.amount);
        assert!(
            repo.save_payment_posting(&failed_payment, &failed_entry, Some((&fee, &updated_fee)),)
                .await
                .is_err()
        );
        let unchanged_fee = StudentFeeRepository::find_by_id(&repo, fee.id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(unchanged_fee.paid_amount, Decimal::ZERO);
        assert_eq!(unchanged_fee.balance_due, fee.balance_due);
        assert!(
            JournalEntryEntity::find_by_id(failed_entry.id)
                .one(&db)
                .await
                .unwrap()
                .is_none()
        );

        let entry = test_entry(org_id, Uuid::new_v4(), now);
        let payment = test_payment(org_id, fee.id, Uuid::new_v4(), entry.id, now);
        let mut updated_fee = fee.clone();
        updated_fee.apply_payment(payment.amount);
        repo.save_payment_posting(&payment, &entry, Some((&fee, &updated_fee)))
            .await
            .unwrap();

        let stale_entry = test_entry(org_id, Uuid::new_v4(), now);
        let stale_payment = test_payment(org_id, fee.id, Uuid::new_v4(), stale_entry.id, now);
        let mut stale_updated_fee = fee.clone();
        stale_updated_fee.apply_payment(stale_payment.amount);
        assert!(matches!(
            repo.save_payment_posting(
                &stale_payment,
                &stale_entry,
                Some((&fee, &stale_updated_fee)),
            )
            .await,
            Err(DomainError::ConcurrencyConflict(_))
        ));
        let final_fee = StudentFeeRepository::find_by_id(&repo, fee.id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(final_fee.paid_amount, payment.amount);
        assert_eq!(PaymentEntity::find().all(&db).await.unwrap().len(), 2);
    }
}
