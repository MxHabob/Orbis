use async_trait::async_trait;
use orbis_app::ports::repositories::{
    AccountRepository, CurrencyRepository, DiscountTemplateRepository, FeeTemplateRepository,
    JournalEntryRepository, PaymentRepository, StudentFeeRepository,
};
use orbis_domain::entities::account::{Account as DomainAccount, AccountType};
use orbis_domain::entities::currency::Currency as DomainCurrency;
use orbis_domain::entities::discount_template::{
    DiscountTemplate as DomainDiscountTemplate, DiscountType,
};
use orbis_domain::entities::exchange_rate::ExchangeRate as DomainExchangeRate;
use orbis_domain::entities::fee_template::{FeeFrequency, FeeTemplate as DomainFeeTemplate};
use orbis_domain::entities::journal_entry::{
    JournalEntry as DomainJournalEntry, JournalPostingStatus,
};
use orbis_domain::entities::payment::{Payment as DomainPayment, PaymentMethod};
use orbis_domain::entities::student_fee::{FeeStatus, StudentFee as DomainStudentFee};
use orbis_domain::errors::DomainError;
use rust_decimal::Decimal;
use sea_orm::sea_query::Expr;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseConnection, EntityTrait, Order,
    PaginatorTrait, QueryFilter, QueryOrder, Set,
};
use uuid::Uuid;

use crate::models::{
    account::{self, Entity as AccountEntity},
    currency::{self, Entity as CurrencyEntity},
    discount_template::{self, Entity as DiscountTemplateEntity},
    exchange_rate::{self, Entity as ExchangeRateEntity},
    fee_template::{self, Entity as FeeTemplateEntity},
    journal_entry::{self, Entity as JournalEntryEntity},
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

fn map_journal_entry_model_to_domain(m: journal_entry::Model) -> DomainJournalEntry {
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
        lines: Vec::new(),
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
        Ok(model.map(map_journal_entry_model_to_domain))
    }

    async fn find_by_org(&self, org_id: Uuid) -> Result<Vec<DomainJournalEntry>, DomainError> {
        let models = JournalEntryEntity::find()
            .filter(journal_entry::Column::OrgId.eq(org_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(models
            .into_iter()
            .map(map_journal_entry_model_to_domain)
            .collect())
    }

    async fn save(&self, entry: &DomainJournalEntry) -> Result<(), DomainError> {
        let active = journal_entry::ActiveModel {
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
        };

        if JournalEntryEntity::find_by_id(entry.id)
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
        JournalEntryEntity::delete_by_id(id)
            .exec(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(())
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
    if let Some(period_id) = filter.period_id {
        query = query.filter(student_fee::Column::PeriodId.eq(Some(period_id)));
    }
    if let Some(status) = filter.status.as_deref() {
        query = query.filter(student_fee::Column::Status.eq(status));
    }
    if filter.only_unpaid == Some(true) {
        query = query
            .filter(Expr::cust_with_values(
                format!(
                    "COALESCE(balance_due, net_amount - COALESCE(paid_amount, 0)) > {}",
                    placeholder(1)
                ),
                [Decimal::ZERO],
            ))
            .filter(student_fee::Column::Status.is_in(["pending", "partially_paid", "overdue"]));
    }
    if filter.is_overdue == Some(true) {
        query = query
            .filter(student_fee::Column::DueDate.lt(chrono::Utc::now().date_naive()))
            .filter(Expr::cust_with_values(
                format!(
                    "COALESCE(balance_due, net_amount - COALESCE(paid_amount, 0)) > {}",
                    placeholder(1)
                ),
                [Decimal::ZERO],
            ));
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
    if let Some(fee_id) = filter.fee_id {
        query = query.filter(payment::Column::FeeId.eq(fee_id));
    }
    if let Some(method) = filter.method {
        query = query.filter(payment::Column::Method.eq(method.to_string()));
    }
    if let Some(from_date) = filter.from_date {
        query = query.filter(payment::Column::PaidAt.gte(from_date));
    }
    if let Some(to_date) = filter.to_date {
        query = query.filter(payment::Column::PaidAt.lte(to_date));
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

    async fn save(&self, fee: &DomainStudentFee) -> Result<(), DomainError> {
        let active = student_fee::ActiveModel {
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
        };

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

    async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        StudentFeeEntity::delete_by_id(id)
            .exec(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(())
    }
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
    use sea_orm::QueryTrait;

    #[test]
    fn student_fee_query_applies_organization_period_and_unpaid_filters() {
        let org_id = Uuid::new_v4();
        let period_id = Uuid::new_v4();
        let filter = orbis_domain::queries::filters::StudentFeeFilter {
            period_id: Some(period_id),
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
            assert!(statement.sql.contains("balance_due"));
            assert!(values.contains("partially_paid"));
            assert!(values.contains(&org_id.to_string()));
            assert!(values.contains(&period_id.to_string()));
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
        let filter = orbis_domain::queries::filters::PaymentFilter {
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
            assert!(statement.sql.contains("payments.receipt_number"));
            assert!(statement.sql.contains("students"));
            assert!(values.contains("cash"));
            assert!(values.contains("%receipt-7%"), "query values: {values}");
            assert!(values.contains(&org_id.to_string()));
        }
    }
}
