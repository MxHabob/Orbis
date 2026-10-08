use crate::context::RequestContext;
use crate::ports::event_bus::EventBus;
use crate::ports::gateways::{NotificationPort, PaymentGatewayPort};
use crate::ports::iam::PermissionCheckerPort;
use crate::ports::repositories::{
    AccountRepository, CurrencyRepository, DiscountTemplateRepository, FeeTemplateRepository,
    JournalEntryRepository, PaymentRepository, PolicyRepository, StudentFeeRepository,
};
use chrono::Utc;
use orbis_domain::entities::account::Account;
use orbis_domain::entities::discount_template::DiscountTemplate;
use orbis_domain::entities::fee_template::FeeTemplate;
use orbis_domain::entities::journal_entry::{JournalEntry, JournalEntryLine, JournalPostingStatus};
use orbis_domain::entities::payment::{Payment, PaymentMethod};
use orbis_domain::entities::student_fee::{FeeStatus, StudentFee};
use orbis_domain::errors::DomainError;
use orbis_domain::events::DomainEvent;
use orbis_domain::iam::permission::Permission;
use orbis_domain::queries::filters::{PaginatedList, PaymentFilter, StudentFeeFilter};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountPurpose {
    MainCashbox,                 // 110101
    MainBank,                    // 110102
    PettyCash,                   // 110103
    ElectronicGatewayClearing,   // 110106
    StudentReceivableTuition,    // 110301
    StudentReceivableTransport,  // 110302
    StudentReceivableActivities, // 110304
    CorporateReceivable,         // 110305
    EmployeeAdvance,             // 110306
    VatInputRecoverable,         // 110401
    VendorPayable,               // 210101
    ServiceContractorPayable,    // 210102
    GeneralSupplierPayable,      // 210103
    SalariesPayable,             // 210201
    AllowancesPayable,           // 210202
    SocialInsuranceGosiPayable,  // 210203
    EndOfServiceProvision,       // 210204
    UnearnedTuition,             // 210301
    VatOutputPayable,            // 210401
    TuitionRevenue,              // 410101
    TransportRevenue,            // 410201
    BooksRevenue,                // 410202
    RegistrationRevenue,         // 410203
    SiblingDiscount,             // 410301
    MeritScholarship,            // 410302
    StaffChildDiscount,          // 410303
    EarlyPaymentDiscount,        // 410304
    AcademicSalariesExpense,     // 510101
    AdminSalariesExpense,        // 520101
    BankGatewayFeesExpense,      // 520301
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisbursementInput {
    pub treasury_account_id: Uuid,
    pub beneficiary_account_id: Uuid,
    pub amount: Decimal,
    pub currency_code: Option<String>,
    pub payment_method: PaymentMethod,
    pub reference: Option<String>,
    pub beneficiary_name: Option<String>,
    pub beneficiary_type: Option<String>, // e.g. "vendor", "employee", "student", "general"
    pub beneficiary_id: Option<Uuid>,
    pub notes: Option<String>,
    pub voucher_date: Option<chrono::NaiveDate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSubAccountInput {
    pub parent_id: Option<Uuid>,
    pub parent_code: Option<String>,
    pub code: Option<String>,
    pub name: String,
    pub name_ar: Option<String>,
    pub entity_type: Option<String>, // "student", "vendor", "employee", "sponsor", "cashbox", "bank"
    pub entity_id: Option<Uuid>,
    pub currency_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatementFilter {
    pub account_id: Option<Uuid>,
    pub entity_type: Option<String>,
    pub entity_id: Option<Uuid>,
    pub start_date: Option<chrono::NaiveDate>,
    pub end_date: Option<chrono::NaiveDate>,
    pub currency_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatementTransactionItem {
    pub id: Uuid,
    pub date: chrono::NaiveDate,
    pub reference_number: String,
    pub source_type: String,
    pub source_id: Option<Uuid>,
    pub description: String,
    pub description_ar: Option<String>,
    pub debit: Decimal,
    pub credit: Decimal,
    pub running_balance: Decimal,
    pub contra_account_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatementResponse {
    pub account_id: Option<Uuid>,
    pub account_code: Option<String>,
    pub account_name: Option<String>,
    pub account_name_ar: Option<String>,
    pub entity_type: Option<String>,
    pub entity_id: Option<Uuid>,
    pub currency: String,
    pub opening_balance: Decimal,
    pub total_debit: Decimal,
    pub total_credit: Decimal,
    pub closing_balance: Decimal,
    pub transactions: Vec<StatementTransactionItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayrollItemInput {
    pub employee_id: Uuid,
    pub employee_name: String,
    pub is_academic: bool,
    pub base_salary: Decimal,
    pub allowances: Decimal,
    pub deductions: Decimal,
    pub gosi_deduction: Decimal,
    pub net_salary: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostPayrollInput {
    pub payroll_period: String, // e.g. "2026-10"
    pub accounting_date: chrono::NaiveDate,
    pub items: Vec<PayrollItemInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettleGatewayPayoutInput {
    pub clearing_account_id: Option<Uuid>,
    pub bank_account_id: Uuid,
    pub gross_amount: Decimal,
    pub fee_amount: Decimal,
    pub net_amount: Decimal,
    pub payout_reference: String,
    pub accounting_date: Option<chrono::NaiveDate>,
}

pub struct FinanceService {
    pub payment_repo: Arc<dyn PaymentRepository>,
    pub journal_repo: Arc<dyn JournalEntryRepository>,
    pub student_fee_repo: Arc<dyn StudentFeeRepository>,
    pub fee_template_repo: Arc<dyn FeeTemplateRepository>,
    pub discount_template_repo: Arc<dyn DiscountTemplateRepository>,
    pub account_repo: Arc<dyn AccountRepository>,
    pub currency_repo: Arc<dyn CurrencyRepository>,
    pub policy_repo: Arc<dyn PolicyRepository>,
    pub event_bus: Arc<dyn EventBus>,
    pub payment_gateway: Arc<dyn PaymentGatewayPort>,
    pub notification_gateway: Arc<dyn NotificationPort>,
    pub permission_checker: Arc<dyn PermissionCheckerPort>,
}

impl FinanceService {
    pub fn new(
        payment_repo: Arc<dyn PaymentRepository>,
        journal_repo: Arc<dyn JournalEntryRepository>,
        student_fee_repo: Arc<dyn StudentFeeRepository>,
        fee_template_repo: Arc<dyn FeeTemplateRepository>,
        discount_template_repo: Arc<dyn DiscountTemplateRepository>,
        account_repo: Arc<dyn AccountRepository>,
        currency_repo: Arc<dyn CurrencyRepository>,
        policy_repo: Arc<dyn PolicyRepository>,
        event_bus: Arc<dyn EventBus>,
        payment_gateway: Arc<dyn PaymentGatewayPort>,
        notification_gateway: Arc<dyn NotificationPort>,
        permission_checker: Arc<dyn PermissionCheckerPort>,
    ) -> Self {
        Self {
            payment_repo,
            journal_repo,
            student_fee_repo,
            fee_template_repo,
            discount_template_repo,
            account_repo,
            currency_repo,
            policy_repo,
            event_bus,
            payment_gateway,
            notification_gateway,
            permission_checker,
        }
    }

    /// Process student payment with double-entry balanced accounting ledger entry
    pub async fn process_payment(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        fee_id: Option<Uuid>,
        student_id: Option<Uuid>,
        enrollment_id: Option<Uuid>,
        account_id: Uuid,
        amount: Decimal,
        method: PaymentMethod,
        payer_name: Option<String>,
        payer_phone: Option<String>,
        reference: Option<String>,
        source: Option<String>,
    ) -> Result<Payment, DomainError> {
        self.permission_checker
            .check(ctx, Permission::ProcessPayment)
            .await?;

        if amount <= Decimal::ZERO {
            return Err(DomainError::Validation(
                "Payment amount must be greater than zero".into(),
            ));
        }

        let now = Utc::now();
        let accounting_date = now.date_naive();
        let journal_entry_id = Uuid::new_v4();
        let payment_id = Uuid::new_v4();

        // 1. Fetch org settings to determine base currency
        let org_settings = self.policy_repo.get_settings(org_id).await?;
        let base_currency = org_settings.finance.currency;

        // 2. Fetch target account to determine transaction currency
        let account = self
            .account_repo
            .find_by_id(account_id)
            .await?
            .ok_or_else(|| DomainError::Validation(format!("Account {} not found", account_id)))?;
        let transaction_currency = account.currency_code;

        let mut exchange_rate_value = Decimal::from(1);
        if transaction_currency != base_currency {
            let rate_opt = self
                .currency_repo
                .get_exchange_rate(org_id, &base_currency, &transaction_currency)
                .await?;
            if let Some(rate) = rate_opt {
                exchange_rate_value = rate.rate;
            } else {
                return Err(DomainError::Validation(format!(
                    "Exchange rate not found from {} to {}",
                    base_currency, transaction_currency
                )));
            }
        }

        // 3. Electronic Gateway Processing if applicable
        let mut final_reference = reference.clone();
        if matches!(
            method,
            PaymentMethod::CreditCard | PaymentMethod::Mada | PaymentMethod::ApplePay
        ) {
            if let Some(src) = source {
                let amount_f64 = rust_decimal::prelude::ToPrimitive::to_f64(&amount).unwrap_or(0.0);
                let gateway_ref = self
                    .payment_gateway
                    .process_payment(ctx, amount_f64, &transaction_currency, &src)
                    .await?;
                final_reference = Some(gateway_ref);
            } else {
                return Err(DomainError::Validation(format!(
                    "Payment method '{}' requires a source token",
                    method
                )));
            }
        }

        // 4. Update StudentFee if fee_id is provided
        let mut receivable_acct = account_id;
        let mut student_resolved_id = student_id;
        let mut enrollment_resolved_id = enrollment_id;

        if let Some(f_id) = fee_id {
            if let Some(mut fee) = self.student_fee_repo.find_by_id(f_id).await? {
                if amount > fee.balance_due {
                    return Err(DomainError::Validation(format!(
                        "Payment amount ({}) exceeds fee balance due ({})",
                        amount, fee.balance_due
                    )));
                }
                if !org_settings.finance.allow_partial_payments && amount < fee.balance_due {
                    return Err(DomainError::Validation(format!(
                        "Partial payments are not permitted by organization policy. Full balance of {} is required.",
                        fee.balance_due
                    )));
                }
                student_resolved_id = Some(fee.student_id);
                enrollment_resolved_id = fee.enrollment_id;
                fee.apply_payment(amount);
                self.student_fee_repo.save(&fee).await?;

                if let Some(template) = self
                    .fee_template_repo
                    .find_by_id(fee.fee_template_id)
                    .await?
                {
                    receivable_acct = template.receivable_account_id;
                }
            }
        }

        let base_amount = amount * exchange_rate_value;

        // 5. Construct balanced multi-line double entry voucher
        let lines = vec![
            JournalEntryLine {
                id: Uuid::new_v4(),
                account_id, // Debit Cash/Bank
                debit_amount: amount,
                credit_amount: Decimal::ZERO,
                description: Some("Payment received for student fee".into()),
                entity_type: Some("student".into()),
                entity_id: student_resolved_id,
            },
            JournalEntryLine {
                id: Uuid::new_v4(),
                account_id: receivable_acct, // Credit Accounts Receivable / Revenue
                debit_amount: Decimal::ZERO,
                credit_amount: amount,
                description: Some("AR settlement".into()),
                entity_type: Some("student".into()),
                entity_id: student_resolved_id,
            },
        ];

        let entry = JournalEntry {
            id: journal_entry_id,
            org_id,
            reference_number: format!(
                "PAY-{}",
                payment_id.to_string().chars().take(8).collect::<String>()
            ),
            accounting_date,
            amount,
            currency_code: transaction_currency.clone(),
            exchange_rate: exchange_rate_value,
            base_amount,
            debit_account_id: account_id,
            credit_account_id: receivable_acct,
            description: "Payment Receipt for Student".into(),
            description_ar: Some("إيصال قبض دفعة رسوم دراسية".into()),
            source_type: "student_payment".into(),
            source_id: Some(payment_id),
            status: JournalPostingStatus::Posted,
            lines,
            is_reversed: false,
            reversed_by: None,
            reversal_reason: None,
            created_by: ctx.user_id(),
            created_at: now,
            updated_at: now,
            updated_by: ctx.user_id(),
            org_unit_id: None,
            version: 1,
        };

        if let Err(e) = entry.validate_balance() {
            return Err(DomainError::Validation(e));
        }

        self.journal_repo.save(&entry).await?;

        // 6. Create Payment Record
        let payment = Payment {
            id: payment_id,
            org_id,
            fee_id,
            enrollment_id: enrollment_resolved_id,
            student_id: student_resolved_id,
            account_id,
            amount,
            currency_code: transaction_currency,
            exchange_rate: exchange_rate_value,
            base_amount,
            method,
            reference: final_reference.clone(),
            receipt_number: Some(format!(
                "REC-{}",
                payment_id.to_string().chars().take(8).collect::<String>()
            )),
            journal_entry_id: Some(journal_entry_id),
            payer_name,
            payer_phone,
            notes: None,
            paid_at: now,
            created_by: ctx.user_id(),
            created_at: now,
            updated_at: now,
            org_unit_id: None,
        };
        self.payment_repo.save(&payment).await?;

        // 7. Publish Events
        let _ = self
            .event_bus
            .publish(
                ctx,
                &DomainEvent::JournalEntryPosted {
                    entry_id: journal_entry_id,
                    amount,
                    timestamp: now,
                },
            )
            .await;

        let _ = self
            .event_bus
            .publish(
                ctx,
                &DomainEvent::PaymentReceived {
                    payment_id,
                    fee_id,
                    amount,
                    timestamp: now,
                },
            )
            .await;

        // 8. Notification — receipt dispatch is handled by the PaymentReceived event subscriber,
        // which has access to guardian/person repos for email lookup.
        // The event was already published above.

        Ok(payment)
    }

    /// Generate fee invoice for student
    pub async fn generate_student_fee(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        period_id: Uuid,
        student_id: Uuid,
        enrollment_id: Option<Uuid>,
        template_id: Uuid,
        discount_amount: Option<Decimal>,
        due_date: Option<chrono::NaiveDate>,
    ) -> Result<StudentFee, DomainError> {
        self.permission_checker
            .check(ctx, Permission::FinanceWrite)
            .await?;

        let template = self
            .fee_template_repo
            .find_by_id(template_id)
            .await?
            .ok_or_else(|| {
                DomainError::NotFound(format!("Fee Template {} not found", template_id))
            })?;

        let discount = discount_amount.unwrap_or(Decimal::ZERO);
        let net_amount = (template.default_amount - discount).max(Decimal::ZERO);

        let now = Utc::now();
        let fee_id = Uuid::new_v4();

        let org_settings = self.policy_repo.get_settings(org_id).await?;
        let due_days = if org_settings.finance.invoice_due_days > 0 {
            org_settings.finance.invoice_due_days as i64
        } else {
            30
        };

        let fee = StudentFee {
            id: fee_id,
            org_id,
            period_id,
            student_id,
            enrollment_id,
            fee_template_id: template_id,
            original_amount: template.default_amount,
            discount_amount: discount,
            net_amount,
            paid_amount: Decimal::ZERO,
            balance_due: net_amount,
            status: FeeStatus::Pending,
            due_date: due_date
                .unwrap_or_else(|| (now + chrono::Duration::days(due_days)).date_naive()),
            discount_details: None,
            installment_number: None,
            notes: None,
            created_by: ctx.user_id(),
            created_at: now,
            updated_at: now,
        };

        self.student_fee_repo.save(&fee).await?;

        // Double-entry ledger accrual for student fee invoice (قيد استحقاق الرسوم)
        let journal_entry_id = Uuid::new_v4();
        let mut lines = vec![JournalEntryLine {
            id: Uuid::new_v4(),
            account_id: template.receivable_account_id,
            debit_amount: net_amount,
            credit_amount: Decimal::ZERO,
            description: Some(format!("Student fee invoice: {}", template.name)),
            entity_type: Some("student".into()),
            entity_id: Some(student_id),
        }];

        if discount > Decimal::ZERO {
            let discount_acct = self
                .resolve_default_account(org_id, AccountPurpose::SiblingDiscount)
                .await
                .map(|a| a.id)
                .unwrap_or(template.revenue_account_id);

            lines.push(JournalEntryLine {
                id: Uuid::new_v4(),
                account_id: discount_acct,
                debit_amount: discount,
                credit_amount: Decimal::ZERO,
                description: Some("Fee concession / discount applied".into()),
                entity_type: Some("student".into()),
                entity_id: Some(student_id),
            });
        }

        lines.push(JournalEntryLine {
            id: Uuid::new_v4(),
            account_id: template.revenue_account_id,
            debit_amount: Decimal::ZERO,
            credit_amount: template.default_amount,
            description: Some(format!("Educational revenue: {}", template.name)),
            entity_type: Some("student".into()),
            entity_id: Some(student_id),
        });

        let invoice_entry = JournalEntry {
            id: journal_entry_id,
            org_id,
            reference_number: format!(
                "INV-{}",
                fee_id.to_string().chars().take(8).collect::<String>()
            ),
            accounting_date: now.date_naive(),
            amount: template.default_amount,
            currency_code: template.currency_code.clone(),
            exchange_rate: Decimal::from(1),
            base_amount: template.default_amount,
            debit_account_id: template.receivable_account_id,
            credit_account_id: template.revenue_account_id,
            description: format!("Tuition fee invoice for student: {}", template.name),
            description_ar: Some(format!(
                "فاتورة رسوم دراسية: {}",
                template
                    .name_ar
                    .clone()
                    .unwrap_or_else(|| template.name.clone())
            )),
            source_type: "student_fee_invoice".into(),
            source_id: Some(fee_id),
            status: JournalPostingStatus::Posted,
            lines,
            is_reversed: false,
            reversed_by: None,
            reversal_reason: None,
            created_by: ctx.user_id(),
            created_at: now,
            updated_at: now,
            updated_by: ctx.user_id(),
            org_unit_id: None,
            version: 1,
        };

        if invoice_entry.validate_balance().is_ok() {
            let _ = self.journal_repo.save(&invoice_entry).await;
            let _ = self
                .event_bus
                .publish(
                    ctx,
                    &DomainEvent::JournalEntryPosted {
                        entry_id: journal_entry_id,
                        amount: template.default_amount,
                        timestamp: now,
                    },
                )
                .await;
        }

        let _ = self
            .event_bus
            .publish(
                ctx,
                &DomainEvent::StudentFeeGenerated {
                    fee_id,
                    student_id,
                    enrollment_id,
                    net_amount,
                    timestamp: now,
                },
            )
            .await;

        Ok(fee)
    }

    pub async fn create_fee_template(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        code: Option<String>,
        name: String,
        name_ar: Option<String>,
        category: Option<String>,
        amount: Decimal,
        currency_code: String,
        receivable_account_id: Option<Uuid>,
        revenue_account_id: Option<Uuid>,
        applicable_levels: Option<Vec<String>>,
        frequency: Option<String>,
        is_mandatory: Option<bool>,
    ) -> Result<FeeTemplate, DomainError> {
        self.permission_checker
            .check(ctx, Permission::FinanceWrite)
            .await?;

        if name.trim().is_empty() {
            return Err(DomainError::Validation(
                "Fee template name is required".into(),
            ));
        }
        if amount <= Decimal::ZERO {
            return Err(DomainError::Validation(
                "Fee template amount must be greater than zero".into(),
            ));
        }

        let category = category.unwrap_or_else(|| "tuition".to_string());
        let receivable_purpose = match category.as_str() {
            "transport" => AccountPurpose::StudentReceivableTransport,
            "activity" => AccountPurpose::StudentReceivableActivities,
            _ => AccountPurpose::StudentReceivableTuition,
        };
        let revenue_purpose = match category.as_str() {
            "registration" => AccountPurpose::RegistrationRevenue,
            "transport" => AccountPurpose::TransportRevenue,
            "books" => AccountPurpose::BooksRevenue,
            _ => AccountPurpose::TuitionRevenue,
        };
        let receivable_account_id = match receivable_account_id {
            Some(id) => id,
            None => {
                self.resolve_default_account(org_id, receivable_purpose)
                    .await?
                    .id
            }
        };
        let revenue_account_id = match revenue_account_id {
            Some(id) => id,
            None => {
                self.resolve_default_account(org_id, revenue_purpose)
                    .await?
                    .id
            }
        };
        let receivable_account = self
            .account_repo
            .find_by_id(receivable_account_id)
            .await?
            .ok_or_else(|| {
                DomainError::NotFound(format!(
                    "Receivable account {} not found",
                    receivable_account_id
                ))
            })?;
        if receivable_account.org_id != org_id
            || receivable_account.account_type
                != orbis_domain::entities::account::AccountType::Asset
        {
            return Err(DomainError::Validation(
                "The receivable account must be an asset account in this organization".into(),
            ));
        }
        let revenue_account = self
            .account_repo
            .find_by_id(revenue_account_id)
            .await?
            .ok_or_else(|| {
                DomainError::NotFound(format!("Revenue account {} not found", revenue_account_id))
            })?;
        if revenue_account.org_id != org_id
            || revenue_account.account_type != orbis_domain::entities::account::AccountType::Revenue
        {
            return Err(DomainError::Validation(
                "The revenue account must be a revenue account in this organization".into(),
            ));
        }

        let frequency = match frequency.as_deref().unwrap_or("yearly") {
            "once" => orbis_domain::entities::fee_template::FeeFrequency::Once,
            "monthly" => orbis_domain::entities::fee_template::FeeFrequency::Monthly,
            "termly" | "per_term" => orbis_domain::entities::fee_template::FeeFrequency::PerTerm,
            "yearly" | "annual" => orbis_domain::entities::fee_template::FeeFrequency::Annual,
            value => {
                return Err(DomainError::Validation(format!(
                    "Unsupported fee frequency '{}'",
                    value
                )));
            }
        };
        let now = Utc::now();
        let tmpl_id = Uuid::new_v4();

        let tmpl = FeeTemplate {
            id: tmpl_id,
            org_id,
            code: code.unwrap_or_else(|| {
                format!(
                    "FEE-{}",
                    tmpl_id.to_string().chars().take(6).collect::<String>()
                )
            }),
            name,
            name_ar,
            category,
            default_amount: amount,
            currency_code,
            frequency,
            receivable_account_id,
            revenue_account_id,
            applicable_levels,
            applicable_study_modes: None,
            is_mandatory: is_mandatory.unwrap_or(true),
            is_active: true,
            created_at: now,
            updated_at: now,
        };

        self.fee_template_repo.save(&tmpl).await?;
        Ok(tmpl)
    }

    /// List fee templates
    pub async fn list_fee_templates(&self, org_id: Uuid) -> Result<Vec<FeeTemplate>, DomainError> {
        self.fee_template_repo.find_by_org(org_id).await
    }

    /// Get fee template
    pub async fn get_fee_template(
        &self,
        org_id: Uuid,
        id: Uuid,
    ) -> Result<Option<FeeTemplate>, DomainError> {
        let tmpl = self.fee_template_repo.find_by_id(id).await?;
        if let Some(ref t) = tmpl {
            if t.org_id != org_id {
                return Err(DomainError::Unauthorized("Organization mismatch".into()));
            }
        }
        Ok(tmpl)
    }

    /// Update fee template
    pub async fn update_fee_template(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        id: Uuid,
        code: Option<String>,
        name: String,
        name_ar: Option<String>,
        category: Option<String>,
        default_amount: Decimal,
        currency_code: Option<String>,
        revenue_account_id: Option<Uuid>,
        receivable_account_id: Option<Uuid>,
        applicable_levels: Option<Vec<String>>,
        frequency: Option<String>,
        is_mandatory: Option<bool>,
        is_active: Option<bool>,
    ) -> Result<FeeTemplate, DomainError> {
        self.permission_checker
            .check(ctx, Permission::FinanceWrite)
            .await?;

        let mut tmpl = self
            .fee_template_repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| DomainError::NotFound(format!("Fee template {} not found", id)))?;

        if tmpl.org_id != org_id {
            return Err(DomainError::Unauthorized("Organization mismatch".into()));
        }

        tmpl.name = name;
        tmpl.name_ar = name_ar;
        if let Some(c) = code {
            if !c.trim().is_empty() {
                tmpl.code = c;
            }
        }
        if let Some(cat) = category {
            tmpl.category = cat;
        }
        tmpl.default_amount = default_amount;
        if let Some(cur) = currency_code {
            tmpl.currency_code = cur;
        }
        if let Some(rev) = revenue_account_id {
            tmpl.revenue_account_id = rev;
        }
        if let Some(rec) = receivable_account_id {
            tmpl.receivable_account_id = rec;
        }
        if let Some(freq) = frequency {
            tmpl.frequency = match freq.to_lowercase().as_str() {
                "once" => orbis_domain::entities::fee_template::FeeFrequency::Once,
                "monthly" => orbis_domain::entities::fee_template::FeeFrequency::Monthly,
                "annual" => orbis_domain::entities::fee_template::FeeFrequency::Annual,
                _ => orbis_domain::entities::fee_template::FeeFrequency::PerTerm,
            };
        }
        if let Some(mand) = is_mandatory {
            tmpl.is_mandatory = mand;
        }
        if let Some(act) = is_active {
            tmpl.is_active = act;
        }
        tmpl.applicable_levels = applicable_levels;
        tmpl.updated_at = Utc::now();

        self.fee_template_repo.save(&tmpl).await?;

        let _ = self
            .event_bus
            .publish(
                ctx,
                &DomainEvent::FeeTemplateUpdated {
                    template_id: id,
                    org_id,
                    timestamp: tmpl.updated_at,
                },
            )
            .await;

        Ok(tmpl)
    }

    /// Delete fee template
    pub async fn delete_fee_template(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        id: Uuid,
    ) -> Result<(), DomainError> {
        self.permission_checker
            .check(ctx, Permission::FinanceWrite)
            .await?;

        let tmpl = self
            .fee_template_repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| DomainError::NotFound(format!("Fee template {} not found", id)))?;

        if tmpl.org_id != org_id {
            return Err(DomainError::Unauthorized("Organization mismatch".into()));
        }

        self.fee_template_repo.delete(id).await?;

        let _ = self
            .event_bus
            .publish(
                ctx,
                &DomainEvent::FeeTemplateDeleted {
                    template_id: id,
                    org_id,
                    timestamp: Utc::now(),
                },
            )
            .await;

        Ok(())
    }

    /// Create general ledger account
    pub async fn create_account(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        code: String,
        name: String,
        name_ar: Option<String>,
        account_type: orbis_domain::entities::account::AccountType,
        sub_type: Option<String>,
        currency_code: String,
        parent_id: Option<Uuid>,
    ) -> Result<Account, DomainError> {
        self.permission_checker
            .check(ctx, Permission::FinanceWrite)
            .await?;

        let now = Utc::now();
        let path = format!("/{}", code);
        let account = Account {
            id: Uuid::new_v4(),
            org_id,
            code,
            name,
            name_ar,
            account_type,
            sub_type,
            parent_id,
            path,
            depth: 1,
            currency_code,
            entity_type: None,
            entity_id: None,
            is_control_account: false,
            is_system_account: false,
            is_active: true,
            balances: serde_json::json!({}),
            created_by: ctx.user_id(),
            updated_by: ctx.user_id(),
            version: 1,
            created_at: now,
            updated_at: now,
        };

        self.account_repo.save(&account).await?;
        Ok(account)
    }

    /// Get general ledger account
    pub async fn get_account(
        &self,
        org_id: Uuid,
        id: Uuid,
    ) -> Result<Option<Account>, DomainError> {
        let acct = self.account_repo.find_by_id(id).await?;
        if let Some(ref a) = acct {
            if a.org_id != org_id {
                return Err(DomainError::Unauthorized("Organization mismatch".into()));
            }
        }
        Ok(acct)
    }

    /// Update general ledger account
    pub async fn update_account(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        id: Uuid,
        code: String,
        name: String,
        name_ar: Option<String>,
        account_type: orbis_domain::entities::account::AccountType,
        sub_type: Option<String>,
        parent_id: Option<Uuid>,
        currency_code: String,
        is_active: bool,
    ) -> Result<Account, DomainError> {
        self.permission_checker
            .check(ctx, Permission::FinanceWrite)
            .await?;

        let mut account = self
            .account_repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| DomainError::NotFound(format!("Account {} not found", id)))?;

        if account.org_id != org_id {
            return Err(DomainError::Unauthorized("Organization mismatch".into()));
        }

        account.code = code.clone();
        account.name = name;
        account.name_ar = name_ar;
        account.account_type = account_type;
        account.sub_type = sub_type;
        account.parent_id = parent_id;
        account.path = format!("/{}", code);
        account.currency_code = currency_code;
        account.is_active = is_active;
        account.updated_by = ctx.user_id();
        account.updated_at = Utc::now();
        account.version += 1;

        self.account_repo.save(&account).await?;

        let _ = self
            .event_bus
            .publish(
                ctx,
                &DomainEvent::AccountUpdated {
                    account_id: id,
                    org_id,
                    timestamp: account.updated_at,
                },
            )
            .await;

        Ok(account)
    }

    /// Delete general ledger account
    pub async fn delete_account(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        id: Uuid,
    ) -> Result<(), DomainError> {
        self.permission_checker
            .check(ctx, Permission::FinanceWrite)
            .await?;

        let account = self
            .account_repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| DomainError::NotFound(format!("Account {} not found", id)))?;

        if account.org_id != org_id {
            return Err(DomainError::Unauthorized("Organization mismatch".into()));
        }

        if account.is_system_account {
            return Err(DomainError::Validation(
                "Cannot delete system protected account".into(),
            ));
        }

        self.account_repo.delete(id).await?;

        let _ = self
            .event_bus
            .publish(
                ctx,
                &DomainEvent::AccountDeleted {
                    account_id: id,
                    org_id,
                    timestamp: Utc::now(),
                },
            )
            .await;

        Ok(())
    }

    /// List general ledger accounts
    pub async fn list_accounts(&self, org_id: Uuid) -> Result<Vec<Account>, DomainError> {
        self.account_repo.find_by_org(org_id).await
    }

    /// List student fees
    pub async fn list_student_fees(
        &self,
        org_id: Uuid,
        filter: StudentFeeFilter,
    ) -> Result<PaginatedList<StudentFee>, DomainError> {
        self.student_fee_repo.list(org_id, filter).await
    }

    /// List payments
    pub async fn list_payments(
        &self,
        org_id: Uuid,
        filter: PaymentFilter,
    ) -> Result<PaginatedList<Payment>, DomainError> {
        self.payment_repo.list(org_id, filter).await
    }

    /// Reverse an existing posted journal entry
    pub async fn reverse_journal_entry(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        entry_id: Uuid,
        reason: String,
    ) -> Result<JournalEntry, DomainError> {
        self.permission_checker
            .check(ctx, Permission::FinanceWrite)
            .await?;

        let mut original_entry =
            self.journal_repo
                .find_by_id(entry_id)
                .await?
                .ok_or_else(|| {
                    DomainError::NotFound(format!("Journal entry {} not found", entry_id))
                })?;

        if original_entry.org_id != org_id {
            return Err(DomainError::Unauthorized("Organization mismatch".into()));
        }

        if original_entry.is_reversed {
            return Err(DomainError::Validation(
                "Journal entry is already reversed".into(),
            ));
        }

        let now = Utc::now();
        let user_id = ctx.user_id().unwrap_or(Uuid::nil());

        // Mark original as reversed
        original_entry.is_reversed = true;
        original_entry.reversed_by = Some(user_id);
        original_entry.reversal_reason = Some(reason.clone());
        self.journal_repo.save(&original_entry).await?;

        // Create opposing reversing entry
        let reversal_id = Uuid::new_v4();
        let reversal_lines: Vec<JournalEntryLine> = original_entry
            .lines
            .into_iter()
            .map(|l| JournalEntryLine {
                id: Uuid::new_v4(),
                account_id: l.account_id,
                debit_amount: l.credit_amount,
                credit_amount: l.debit_amount,
                description: Some(format!("Reversal: {}", l.description.unwrap_or_default())),
                entity_type: l.entity_type,
                entity_id: l.entity_id,
            })
            .collect();

        let reversing_entry = JournalEntry {
            id: reversal_id,
            org_id,
            reference_number: format!("REV-{}", original_entry.reference_number),
            accounting_date: now.date_naive(),
            amount: original_entry.amount,
            currency_code: original_entry.currency_code,
            exchange_rate: original_entry.exchange_rate,
            base_amount: original_entry.base_amount,
            debit_account_id: original_entry.credit_account_id,
            credit_account_id: original_entry.debit_account_id,
            description: format!(
                "Reversal of {}: {}",
                original_entry.reference_number, reason
            ),
            description_ar: None,
            source_type: "reversal".into(),
            source_id: Some(original_entry.id),
            status: JournalPostingStatus::Posted,
            lines: reversal_lines,
            is_reversed: false,
            reversed_by: None,
            reversal_reason: None,
            created_by: Some(user_id),
            created_at: now,
            updated_at: now,
            updated_by: Some(user_id),
            org_unit_id: original_entry.org_unit_id,
            version: 1,
        };

        self.journal_repo.save(&reversing_entry).await?;
        Ok(reversing_entry)
    }

    /// Process a refund for a student fee
    pub async fn refund_student_fee(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        fee_id: Uuid,
        refund_amount: Decimal,
        notes: Option<String>,
    ) -> Result<StudentFee, DomainError> {
        self.permission_checker
            .check(ctx, Permission::ProcessPayment)
            .await?;

        let mut fee = self
            .student_fee_repo
            .find_by_id(fee_id)
            .await?
            .ok_or_else(|| DomainError::NotFound(format!("Student fee {} not found", fee_id)))?;

        if fee.org_id != org_id {
            return Err(DomainError::Unauthorized("Organization mismatch".into()));
        }

        if refund_amount <= Decimal::ZERO || refund_amount > fee.paid_amount {
            return Err(DomainError::Validation(format!(
                "Invalid refund amount: {} (Paid: {})",
                refund_amount, fee.paid_amount
            )));
        }

        fee.paid_amount -= refund_amount;
        fee.balance_due += refund_amount;
        if fee.paid_amount == Decimal::ZERO {
            fee.status = FeeStatus::Refunded;
        } else {
            fee.status = FeeStatus::PartiallyPaid;
        }
        if let Some(n) = notes {
            fee.notes = Some(match fee.notes {
                Some(existing) => format!("{}; Refund note: {}", existing, n),
                None => format!("Refund note: {}", n),
            });
        }
        fee.updated_at = Utc::now();

        self.student_fee_repo.save(&fee).await?;
        Ok(fee)
    }

    /// Split a single student fee into multiple installment schedule records
    pub async fn split_fee_into_installments(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        fee_id: Uuid,
        num_installments: u32,
        due_dates: Vec<chrono::NaiveDate>,
    ) -> Result<Vec<StudentFee>, DomainError> {
        self.permission_checker
            .check(ctx, Permission::FinanceWrite)
            .await?;

        if num_installments < 2 || due_dates.len() != num_installments as usize {
            return Err(DomainError::Validation(
                "Installment count must match number of due dates".into(),
            ));
        }

        let base_fee = self
            .student_fee_repo
            .find_by_id(fee_id)
            .await?
            .ok_or_else(|| DomainError::NotFound(format!("Student fee {} not found", fee_id)))?;

        if base_fee.org_id != org_id {
            return Err(DomainError::Unauthorized("Organization mismatch".into()));
        }

        if base_fee.paid_amount > Decimal::ZERO {
            return Err(DomainError::Validation(
                "Cannot split fee that already has payments applied".into(),
            ));
        }

        let installment_amount =
            (base_fee.net_amount / Decimal::from(num_installments)).round_dp(2);
        let mut created_fees = Vec::new();
        let now = Utc::now();

        for (i, due_date) in due_dates.into_iter().enumerate() {
            let inst_fee = StudentFee {
                id: if i == 0 { base_fee.id } else { Uuid::new_v4() },
                org_id: base_fee.org_id,
                period_id: base_fee.period_id,
                student_id: base_fee.student_id,
                enrollment_id: base_fee.enrollment_id,
                fee_template_id: base_fee.fee_template_id,
                original_amount: installment_amount,
                discount_amount: Decimal::ZERO,
                net_amount: installment_amount,
                paid_amount: Decimal::ZERO,
                balance_due: installment_amount,
                status: FeeStatus::Pending,
                due_date,
                discount_details: None,
                installment_number: Some((i + 1) as u32),
                notes: Some(format!("Installment {} of {}", i + 1, num_installments)),
                created_by: ctx.user_id(),
                created_at: now,
                updated_at: now,
            };
            self.student_fee_repo.save(&inst_fee).await?;
            created_fees.push(inst_fee);
        }

        Ok(created_fees)
    }

    /// Create discount template
    pub async fn create_discount_template(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        code: Option<String>,
        name: String,
        name_ar: Option<String>,
        discount_type: orbis_domain::entities::discount_template::DiscountType,
        value: Decimal,
        eligibility_rules: Option<serde_json::Value>,
        is_approval_required: bool,
        is_auto_apply: bool,
        is_stackable: bool,
        priority: u32,
    ) -> Result<DiscountTemplate, DomainError> {
        self.permission_checker
            .check(ctx, Permission::FinanceWrite)
            .await?;

        let now = Utc::now();
        let tmpl_id = Uuid::new_v4();

        let tmpl = DiscountTemplate {
            id: tmpl_id,
            org_id,
            code: code.unwrap_or_else(|| {
                format!(
                    "DISC-{}",
                    tmpl_id.to_string().chars().take(6).collect::<String>()
                )
            }),
            name,
            name_ar,
            discount_type,
            value,
            eligibility_rules,
            is_approval_required,
            is_auto_apply,
            is_stackable,
            priority,
            is_active: true,
            created_at: now,
            updated_at: now,
        };

        self.discount_template_repo.save(&tmpl).await?;

        let _ = self
            .event_bus
            .publish(
                ctx,
                &DomainEvent::DiscountTemplateCreated {
                    template_id: tmpl_id,
                    org_id,
                    code: tmpl.code.clone(),
                    timestamp: now,
                },
            )
            .await;

        Ok(tmpl)
    }

    /// List discount templates
    pub async fn list_discount_templates(
        &self,
        org_id: Uuid,
    ) -> Result<Vec<DiscountTemplate>, DomainError> {
        self.discount_template_repo.find_by_org(org_id).await
    }

    /// Delete discount template
    pub async fn delete_discount_template(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        id: Uuid,
    ) -> Result<(), DomainError> {
        self.permission_checker
            .check(ctx, Permission::FinanceWrite)
            .await?;

        let tmpl = self
            .discount_template_repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| DomainError::NotFound(format!("Discount template {} not found", id)))?;

        if tmpl.org_id != org_id {
            return Err(DomainError::Unauthorized("Organization mismatch".into()));
        }

        self.discount_template_repo.delete(id).await?;

        let _ = self
            .event_bus
            .publish(
                ctx,
                &DomainEvent::DiscountTemplateDeleted {
                    template_id: id,
                    org_id,
                    timestamp: Utc::now(),
                },
            )
            .await;

        Ok(())
    }

    /// Delete student fee
    pub async fn delete_student_fee(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        id: Uuid,
    ) -> Result<(), DomainError> {
        self.permission_checker
            .check(ctx, Permission::FinanceWrite)
            .await?;

        let fee = self
            .student_fee_repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| DomainError::NotFound(format!("Student fee {} not found", id)))?;

        if fee.org_id != org_id {
            return Err(DomainError::Unauthorized("Organization mismatch".into()));
        }

        if fee.paid_amount > Decimal::ZERO {
            return Err(DomainError::Validation(
                "Cannot delete fee with applied payments. Refund or reverse payments first.".into(),
            ));
        }

        self.student_fee_repo.delete(id).await?;

        let _ = self
            .event_bus
            .publish(
                ctx,
                &DomainEvent::StudentFeeDeleted {
                    fee_id: id,
                    org_id,
                    timestamp: Utc::now(),
                },
            )
            .await;

        Ok(())
    }

    /// Delete payment record
    pub async fn delete_payment(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        id: Uuid,
    ) -> Result<(), DomainError> {
        self.permission_checker
            .check(ctx, Permission::ProcessPayment)
            .await?;

        let payment = self
            .payment_repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| DomainError::NotFound(format!("Payment {} not found", id)))?;

        if payment.org_id != org_id {
            return Err(DomainError::Unauthorized("Organization mismatch".into()));
        }

        // If linked to a student fee, adjust the fee's paid_amount and balance_due
        if let Some(fee_id) = payment.fee_id {
            if let Some(mut fee) = self.student_fee_repo.find_by_id(fee_id).await? {
                fee.paid_amount = (fee.paid_amount - payment.amount).max(Decimal::ZERO);
                fee.balance_due = (fee.net_amount - fee.paid_amount).max(Decimal::ZERO);
                if fee.paid_amount == Decimal::ZERO {
                    fee.status = FeeStatus::Pending;
                } else {
                    fee.status = FeeStatus::PartiallyPaid;
                }
                fee.updated_at = Utc::now();
                self.student_fee_repo.save(&fee).await?;
            }
        }

        self.payment_repo.delete(id).await?;

        let _ = self
            .event_bus
            .publish(
                ctx,
                &DomainEvent::PaymentDeleted {
                    payment_id: id,
                    org_id,
                    timestamp: Utc::now(),
                },
            )
            .await;

        Ok(())
    }

    /// Delete journal entry
    pub async fn delete_journal_entry(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        id: Uuid,
    ) -> Result<(), DomainError> {
        self.permission_checker
            .check(ctx, Permission::FinanceWrite)
            .await?;

        let entry = self
            .journal_repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| DomainError::NotFound(format!("Journal entry {} not found", id)))?;

        if entry.org_id != org_id {
            return Err(DomainError::Unauthorized("Organization mismatch".into()));
        }

        self.journal_repo.delete(id).await?;

        let _ = self
            .event_bus
            .publish(
                ctx,
                &DomainEvent::JournalEntryDeleted {
                    entry_id: id,
                    org_id,
                    timestamp: Utc::now(),
                },
            )
            .await;

        Ok(())
    }

    /// Resolve canonical account from the Chart of Accounts tree
    pub async fn resolve_default_account(
        &self,
        org_id: Uuid,
        purpose: AccountPurpose,
    ) -> Result<Account, DomainError> {
        let accounts = self.account_repo.find_by_org(org_id).await?;

        let matching = match purpose {
            AccountPurpose::MainCashbox => accounts.iter().find(|a| {
                a.code == "110101"
                    || a.code.starts_with("1101")
                    || a.sub_type.as_deref() == Some("cash")
            }),
            AccountPurpose::MainBank => accounts
                .iter()
                .find(|a| a.code == "110102" || a.sub_type.as_deref() == Some("bank")),
            AccountPurpose::PettyCash => accounts.iter().find(|a| a.code == "110103"),
            AccountPurpose::ElectronicGatewayClearing => accounts
                .iter()
                .find(|a| a.code == "110106" || a.code == "110102"),
            AccountPurpose::StudentReceivableTuition => accounts.iter().find(|a| {
                a.code == "110301"
                    || a.code == "1103"
                    || (a.account_type == orbis_domain::entities::account::AccountType::Asset
                        && a.code.starts_with("1103"))
            }),
            AccountPurpose::StudentReceivableTransport => accounts
                .iter()
                .find(|a| a.code == "110302" || a.code == "110301"),
            AccountPurpose::StudentReceivableActivities => accounts
                .iter()
                .find(|a| a.code == "110304" || a.code == "110301"),
            AccountPurpose::CorporateReceivable => accounts
                .iter()
                .find(|a| a.code == "110305" || a.code == "110301"),
            AccountPurpose::EmployeeAdvance => accounts
                .iter()
                .find(|a| a.code == "110306" || a.code == "110301"),
            AccountPurpose::VatInputRecoverable => accounts.iter().find(|a| a.code == "110401"),
            AccountPurpose::VendorPayable => accounts.iter().find(|a| {
                a.code == "210101"
                    || a.code == "2101"
                    || (a.account_type == orbis_domain::entities::account::AccountType::Liability
                        && a.code.starts_with("2101"))
            }),
            AccountPurpose::ServiceContractorPayable => accounts
                .iter()
                .find(|a| a.code == "210102" || a.code == "210101"),
            AccountPurpose::GeneralSupplierPayable => accounts
                .iter()
                .find(|a| a.code == "210103" || a.code == "210101"),
            AccountPurpose::SalariesPayable => accounts
                .iter()
                .find(|a| a.code == "210201" || a.code.starts_with("2102")),
            AccountPurpose::AllowancesPayable => accounts
                .iter()
                .find(|a| a.code == "210202" || a.code == "210201"),
            AccountPurpose::SocialInsuranceGosiPayable => accounts
                .iter()
                .find(|a| a.code == "210203" || a.code == "210201"),
            AccountPurpose::EndOfServiceProvision => accounts
                .iter()
                .find(|a| a.code == "210204" || a.code == "210201"),
            AccountPurpose::UnearnedTuition => accounts
                .iter()
                .find(|a| a.code == "210301" || a.code.starts_with("2103")),
            AccountPurpose::VatOutputPayable => accounts.iter().find(|a| a.code == "210401"),
            AccountPurpose::TuitionRevenue => accounts.iter().find(|a| {
                a.code == "410101"
                    || a.code == "4101"
                    || (a.account_type == orbis_domain::entities::account::AccountType::Revenue
                        && a.code.starts_with("4101"))
            }),
            AccountPurpose::TransportRevenue => accounts
                .iter()
                .find(|a| a.code == "410201" || a.code == "410101"),
            AccountPurpose::BooksRevenue => accounts
                .iter()
                .find(|a| a.code == "410202" || a.code == "410101"),
            AccountPurpose::RegistrationRevenue => accounts
                .iter()
                .find(|a| a.code == "410203" || a.code == "410101"),
            AccountPurpose::SiblingDiscount => accounts
                .iter()
                .find(|a| a.code == "410301" || a.code.starts_with("4103")),
            AccountPurpose::MeritScholarship => accounts
                .iter()
                .find(|a| a.code == "410302" || a.code == "410301"),
            AccountPurpose::StaffChildDiscount => accounts
                .iter()
                .find(|a| a.code == "410303" || a.code == "410301"),
            AccountPurpose::EarlyPaymentDiscount => accounts
                .iter()
                .find(|a| a.code == "410304" || a.code == "410301"),
            AccountPurpose::AcademicSalariesExpense => accounts.iter().find(|a| {
                a.code == "510101"
                    || a.code == "5101"
                    || (a.account_type == orbis_domain::entities::account::AccountType::Expense
                        && a.code.starts_with("51"))
            }),
            AccountPurpose::AdminSalariesExpense => accounts.iter().find(|a| {
                a.code == "520101"
                    || a.code == "5201"
                    || (a.account_type == orbis_domain::entities::account::AccountType::Expense
                        && a.code.starts_with("52"))
            }),
            AccountPurpose::BankGatewayFeesExpense => accounts
                .iter()
                .find(|a| a.code == "520301" || a.code.starts_with("52")),
        };

        matching.cloned().ok_or_else(|| {
            DomainError::NotFound(format!(
                "No matching account found in Chart of Accounts for purpose '{:?}'",
                purpose
            ))
        })
    }

    /// Process a disbursement voucher (سند صرف)
    pub async fn process_disbursement(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        input: DisbursementInput,
    ) -> Result<JournalEntry, DomainError> {
        self.permission_checker
            .check(ctx, Permission::FinanceWrite)
            .await?;

        if input.amount <= Decimal::ZERO {
            return Err(DomainError::Validation(
                "Disbursement amount must be greater than zero".into(),
            ));
        }

        let now = Utc::now();
        let accounting_date = input.voucher_date.unwrap_or_else(|| now.date_naive());
        let journal_entry_id = Uuid::new_v4();

        // 1. Fetch treasury (credit) and beneficiary (debit) accounts
        let treasury = self
            .account_repo
            .find_by_id(input.treasury_account_id)
            .await?
            .ok_or_else(|| {
                DomainError::NotFound(format!(
                    "Treasury account {} not found",
                    input.treasury_account_id
                ))
            })?;
        let beneficiary = self
            .account_repo
            .find_by_id(input.beneficiary_account_id)
            .await?
            .ok_or_else(|| {
                DomainError::NotFound(format!(
                    "Beneficiary account {} not found",
                    input.beneficiary_account_id
                ))
            })?;

        if treasury.org_id != org_id || beneficiary.org_id != org_id {
            return Err(DomainError::Unauthorized(
                "Organization mismatch for accounts".into(),
            ));
        }

        let currency = input
            .currency_code
            .unwrap_or_else(|| treasury.currency_code.clone());

        // 2. Multi-line balanced entry
        let lines = vec![
            JournalEntryLine {
                id: Uuid::new_v4(),
                account_id: input.beneficiary_account_id,
                debit_amount: input.amount,
                credit_amount: Decimal::ZERO,
                description: input
                    .notes
                    .clone()
                    .or_else(|| Some("Disbursement - Beneficiary / Expense".into())),
                entity_type: input.beneficiary_type.clone(),
                entity_id: input.beneficiary_id,
            },
            JournalEntryLine {
                id: Uuid::new_v4(),
                account_id: input.treasury_account_id,
                debit_amount: Decimal::ZERO,
                credit_amount: input.amount,
                description: input
                    .notes
                    .clone()
                    .or_else(|| Some("Disbursement - Treasury / Cashbox".into())),
                entity_type: Some("treasury".into()),
                entity_id: Some(input.treasury_account_id),
            },
        ];

        let ref_num = input.reference.unwrap_or_else(|| {
            format!(
                "DISB-{}",
                journal_entry_id
                    .to_string()
                    .chars()
                    .take(8)
                    .collect::<String>()
            )
        });

        let entry = JournalEntry {
            id: journal_entry_id,
            org_id,
            reference_number: ref_num,
            accounting_date,
            amount: input.amount,
            currency_code: currency,
            exchange_rate: Decimal::from(1),
            base_amount: input.amount,
            debit_account_id: input.beneficiary_account_id,
            credit_account_id: input.treasury_account_id,
            description: input
                .notes
                .clone()
                .unwrap_or_else(|| format!("Disbursement to {}", beneficiary.name)),
            description_ar: Some(input.notes.unwrap_or_else(|| {
                format!(
                    "سند صرف إلى {}",
                    beneficiary.name_ar.unwrap_or(beneficiary.name)
                )
            })),
            source_type: "disbursement_voucher".into(),
            source_id: Some(journal_entry_id),
            status: JournalPostingStatus::Posted,
            lines,
            is_reversed: false,
            reversed_by: None,
            reversal_reason: None,
            created_by: ctx.user_id(),
            created_at: now,
            updated_at: now,
            updated_by: ctx.user_id(),
            org_unit_id: None,
            version: 1,
        };

        if let Err(e) = entry.validate_balance() {
            return Err(DomainError::Validation(e));
        }

        self.journal_repo.save(&entry).await?;

        let _ = self
            .event_bus
            .publish(
                ctx,
                &DomainEvent::JournalEntryPosted {
                    entry_id: journal_entry_id,
                    amount: input.amount,
                    timestamp: now,
                },
            )
            .await;

        Ok(entry)
    }

    /// Settle electronic gateway payout to bank account (تسوية مبالغ بوابة الدفع)
    pub async fn settle_gateway_payout(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        input: SettleGatewayPayoutInput,
    ) -> Result<JournalEntry, DomainError> {
        self.permission_checker
            .check(ctx, Permission::FinanceWrite)
            .await?;

        if input.gross_amount <= Decimal::ZERO {
            return Err(DomainError::Validation(
                "Gross settlement amount must be greater than zero".into(),
            ));
        }

        if input.net_amount + input.fee_amount != input.gross_amount {
            return Err(DomainError::Validation(format!(
                "Net amount ({}) + Fee amount ({}) != Gross amount ({})",
                input.net_amount, input.fee_amount, input.gross_amount
            )));
        }

        let now = Utc::now();
        let accounting_date = input.accounting_date.unwrap_or_else(|| now.date_naive());
        let journal_entry_id = Uuid::new_v4();

        // 1. Resolve accounts
        let clearing_acct_id = match input.clearing_account_id {
            Some(id) => id,
            None => {
                self.resolve_default_account(org_id, AccountPurpose::ElectronicGatewayClearing)
                    .await?
                    .id
            }
        };

        let bank = self
            .account_repo
            .find_by_id(input.bank_account_id)
            .await?
            .ok_or_else(|| {
                DomainError::NotFound(format!("Bank account {} not found", input.bank_account_id))
            })?;

        let fee_acct = self
            .resolve_default_account(org_id, AccountPurpose::BankGatewayFeesExpense)
            .await
            .unwrap_or_else(|_| bank.clone());

        let mut lines = vec![
            // Dr Bank Account (Net received)
            JournalEntryLine {
                id: Uuid::new_v4(),
                account_id: bank.id,
                debit_amount: input.net_amount,
                credit_amount: Decimal::ZERO,
                description: Some(format!(
                    "Gateway settlement deposit: {}",
                    input.payout_reference
                )),
                entity_type: Some("bank".into()),
                entity_id: Some(bank.id),
            },
        ];

        // Dr Fee Account if any
        if input.fee_amount > Decimal::ZERO {
            lines.push(JournalEntryLine {
                id: Uuid::new_v4(),
                account_id: fee_acct.id,
                debit_amount: input.fee_amount,
                credit_amount: Decimal::ZERO,
                description: Some(format!(
                    "Gateway processing commission: {}",
                    input.payout_reference
                )),
                entity_type: Some("gateway".into()),
                entity_id: None,
            });
        }

        // Cr Electronic Gateway Clearing Account (Gross collected)
        lines.push(JournalEntryLine {
            id: Uuid::new_v4(),
            account_id: clearing_acct_id,
            debit_amount: Decimal::ZERO,
            credit_amount: input.gross_amount,
            description: Some(format!(
                "Electronic clearing settlement: {}",
                input.payout_reference
            )),
            entity_type: Some("gateway_clearing".into()),
            entity_id: Some(clearing_acct_id),
        });

        let entry = JournalEntry {
            id: journal_entry_id,
            org_id,
            reference_number: format!("PAYOUT-{}", input.payout_reference),
            accounting_date,
            amount: input.gross_amount,
            currency_code: bank.currency_code.clone(),
            exchange_rate: Decimal::from(1),
            base_amount: input.gross_amount,
            debit_account_id: bank.id,
            credit_account_id: clearing_acct_id,
            description: format!(
                "Payment Gateway Payout Settlement: {}",
                input.payout_reference
            ),
            description_ar: Some(format!(
                "تسوية وإيداع مبالغ بوابة الدفع: {}",
                input.payout_reference
            )),
            source_type: "gateway_settlement".into(),
            source_id: Some(journal_entry_id),
            status: JournalPostingStatus::Posted,
            lines,
            is_reversed: false,
            reversed_by: None,
            reversal_reason: None,
            created_by: ctx.user_id(),
            created_at: now,
            updated_at: now,
            updated_by: ctx.user_id(),
            org_unit_id: None,
            version: 1,
        };

        if let Err(e) = entry.validate_balance() {
            return Err(DomainError::Validation(e));
        }

        self.journal_repo.save(&entry).await?;

        let _ = self
            .event_bus
            .publish(
                ctx,
                &DomainEvent::JournalEntryPosted {
                    entry_id: journal_entry_id,
                    amount: input.gross_amount,
                    timestamp: now,
                },
            )
            .await;

        Ok(entry)
    }

    /// Post monthly employee payroll journal entry (مسير استحقاق الرواتب والأجور)
    pub async fn post_payroll_journal(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        input: PostPayrollInput,
    ) -> Result<JournalEntry, DomainError> {
        self.permission_checker
            .check(ctx, Permission::FinanceWrite)
            .await?;

        if input.items.is_empty() {
            return Err(DomainError::Validation(
                "Payroll must contain at least one employee item".into(),
            ));
        }

        let now = Utc::now();
        let journal_entry_id = Uuid::new_v4();

        let academic_expense = self
            .resolve_default_account(org_id, AccountPurpose::AcademicSalariesExpense)
            .await?;
        let admin_expense = self
            .resolve_default_account(org_id, AccountPurpose::AdminSalariesExpense)
            .await?;
        let salaries_payable = self
            .resolve_default_account(org_id, AccountPurpose::SalariesPayable)
            .await?;
        let gosi_payable = self
            .resolve_default_account(org_id, AccountPurpose::SocialInsuranceGosiPayable)
            .await
            .unwrap_or_else(|_| salaries_payable.clone());
        let advance_receivable = self
            .resolve_default_account(org_id, AccountPurpose::EmployeeAdvance)
            .await
            .unwrap_or_else(|_| salaries_payable.clone());

        let mut lines = Vec::new();
        let mut total_gross = Decimal::ZERO;

        for item in &input.items {
            let gross = item.base_salary + item.allowances;
            total_gross += gross;

            // Dr Salaries Expense
            let expense_acct = if item.is_academic {
                &academic_expense
            } else {
                &admin_expense
            };
            lines.push(JournalEntryLine {
                id: Uuid::new_v4(),
                account_id: expense_acct.id,
                debit_amount: gross,
                credit_amount: Decimal::ZERO,
                description: Some(format!(
                    "Salary: {} ({})",
                    item.employee_name, input.payroll_period
                )),
                entity_type: Some("employee".into()),
                entity_id: Some(item.employee_id),
            });

            // Cr GOSI deduction if any
            if item.gosi_deduction > Decimal::ZERO {
                lines.push(JournalEntryLine {
                    id: Uuid::new_v4(),
                    account_id: gosi_payable.id,
                    debit_amount: Decimal::ZERO,
                    credit_amount: item.gosi_deduction,
                    description: Some(format!(
                        "GOSI Social Security deduction: {}",
                        item.employee_name
                    )),
                    entity_type: Some("employee".into()),
                    entity_id: Some(item.employee_id),
                });
            }

            // Cr Advance recovery if any
            if item.deductions > Decimal::ZERO {
                lines.push(JournalEntryLine {
                    id: Uuid::new_v4(),
                    account_id: advance_receivable.id,
                    debit_amount: Decimal::ZERO,
                    credit_amount: item.deductions,
                    description: Some(format!(
                        "Salary loan / advance deduction: {}",
                        item.employee_name
                    )),
                    entity_type: Some("employee".into()),
                    entity_id: Some(item.employee_id),
                });
            }

            // Cr Net Salary Payable
            lines.push(JournalEntryLine {
                id: Uuid::new_v4(),
                account_id: salaries_payable.id,
                debit_amount: Decimal::ZERO,
                credit_amount: item.net_salary,
                description: Some(format!("Net salary payable: {}", item.employee_name)),
                entity_type: Some("employee".into()),
                entity_id: Some(item.employee_id),
            });
        }

        let entry = JournalEntry {
            id: journal_entry_id,
            org_id,
            reference_number: format!("PAYROLL-{}", input.payroll_period),
            accounting_date: input.accounting_date,
            amount: total_gross,
            currency_code: academic_expense.currency_code.clone(),
            exchange_rate: Decimal::from(1),
            base_amount: total_gross,
            debit_account_id: academic_expense.id,
            credit_account_id: salaries_payable.id,
            description: format!("Monthly Payroll Accrual: {}", input.payroll_period),
            description_ar: Some(format!(
                "مسير استحقاق الرواتب والأجور لشهر: {}",
                input.payroll_period
            )),
            source_type: "payroll_accrual".into(),
            source_id: Some(journal_entry_id),
            status: JournalPostingStatus::Posted,
            lines,
            is_reversed: false,
            reversed_by: None,
            reversal_reason: None,
            created_by: ctx.user_id(),
            created_at: now,
            updated_at: now,
            updated_by: ctx.user_id(),
            org_unit_id: None,
            version: 1,
        };

        if let Err(e) = entry.validate_balance() {
            return Err(DomainError::Validation(e));
        }

        self.journal_repo.save(&entry).await?;

        let _ = self
            .event_bus
            .publish(
                ctx,
                &DomainEvent::JournalEntryPosted {
                    entry_id: journal_entry_id,
                    amount: total_gross,
                    timestamp: now,
                },
            )
            .await;

        Ok(entry)
    }

    /// Disburse payroll from a bank or cash account
    pub async fn disburse_payroll(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        bank_account_id: Uuid,
        payroll_period: String,
        amount: Decimal,
        notes: Option<String>,
    ) -> Result<JournalEntry, DomainError> {
        self.permission_checker
            .check(ctx, Permission::FinanceWrite)
            .await?;

        if amount <= Decimal::ZERO {
            return Err(DomainError::Validation(
                "Payroll disbursement amount must be greater than zero".into(),
            ));
        }

        let salaries_payable = self
            .resolve_default_account(org_id, AccountPurpose::SalariesPayable)
            .await?;
        let bank = self
            .account_repo
            .find_by_id(bank_account_id)
            .await?
            .ok_or_else(|| {
                DomainError::NotFound(format!("Bank account {} not found", bank_account_id))
            })?;

        let input = DisbursementInput {
            treasury_account_id: bank.id,
            beneficiary_account_id: salaries_payable.id,
            amount,
            currency_code: Some(bank.currency_code),
            payment_method: PaymentMethod::BankTransfer,
            reference: Some(format!("PAYROLL-DISB-{}", payroll_period)),
            beneficiary_name: Some("All Employees (Net Salaries)".into()),
            beneficiary_type: Some("employee_payroll".into()),
            beneficiary_id: None,
            notes: notes.or_else(|| {
                Some(format!(
                    "Disbursement of payroll for period: {}",
                    payroll_period
                ))
            }),
            voucher_date: None,
        };

        self.process_disbursement(ctx, org_id, input).await
    }

    /// Retrieve detailed general ledger or subsidiary account statement (كشف حساب)
    pub async fn get_account_statement(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        filter: StatementFilter,
    ) -> Result<StatementResponse, DomainError> {
        self.permission_checker
            .check(ctx, Permission::FinanceRead)
            .await?;

        let mut target_account = None;
        if let Some(acct_id) = filter.account_id {
            target_account = self.account_repo.find_by_id(acct_id).await?;
        }

        // If no specific account_id but entity_type is provided:
        if target_account.is_none() {
            if let Some(ref etype) = filter.entity_type {
                match etype.as_str() {
                    "student" => {
                        target_account = self
                            .resolve_default_account(
                                org_id,
                                AccountPurpose::StudentReceivableTuition,
                            )
                            .await
                            .ok();
                    }
                    "vendor" => {
                        target_account = self
                            .resolve_default_account(org_id, AccountPurpose::VendorPayable)
                            .await
                            .ok();
                    }
                    "employee" => {
                        target_account = self
                            .resolve_default_account(org_id, AccountPurpose::SalariesPayable)
                            .await
                            .ok();
                    }
                    "sponsor" | "corporate" => {
                        target_account = self
                            .resolve_default_account(org_id, AccountPurpose::CorporateReceivable)
                            .await
                            .ok();
                    }
                    _ => {}
                }
            }
        }

        let all_entries = self.journal_repo.find_by_org(org_id).await?;
        let org_settings = self.policy_repo.get_settings(org_id).await?;
        let currency = filter
            .currency_code
            .unwrap_or(org_settings.finance.currency);

        let mut opening_balance = Decimal::ZERO;
        let mut total_debit = Decimal::ZERO;
        let mut total_credit = Decimal::ZERO;
        let mut transactions = Vec::new();

        let acct_id_filter = filter.account_id;
        let entity_type_filter = filter.entity_type.as_deref();
        let entity_id_filter = filter.entity_id;
        let start_date = filter.start_date;
        let end_date = filter.end_date;

        // Sort chronological
        let mut sorted_entries = all_entries;
        sorted_entries.sort_by(|a, b| {
            a.accounting_date
                .cmp(&b.accounting_date)
                .then(a.created_at.cmp(&b.created_at))
        });

        let mut running_balance = Decimal::ZERO;

        for entry in sorted_entries {
            if entry.is_reversed || entry.status == JournalPostingStatus::Draft {
                continue;
            }

            for line in &entry.lines {
                let mut matches = true;

                if let Some(target_id) = acct_id_filter {
                    if line.account_id != target_id {
                        matches = false;
                    }
                }

                if let Some(etype) = entity_type_filter {
                    if line.entity_type.as_deref() != Some(etype) {
                        matches = false;
                    }
                }

                if let Some(eid) = entity_id_filter {
                    if line.entity_id != Some(eid) {
                        matches = false;
                    }
                }

                if !matches {
                    continue;
                }

                let net_movement = line.debit_amount - line.credit_amount;

                // Check date window
                if let Some(start) = start_date {
                    if entry.accounting_date < start {
                        opening_balance += net_movement;
                        running_balance += net_movement;
                        continue;
                    }
                }

                if let Some(end) = end_date {
                    if entry.accounting_date > end {
                        continue;
                    }
                }

                running_balance += net_movement;
                total_debit += line.debit_amount;
                total_credit += line.credit_amount;

                transactions.push(StatementTransactionItem {
                    id: line.id,
                    date: entry.accounting_date,
                    reference_number: entry.reference_number.clone(),
                    source_type: entry.source_type.clone(),
                    source_id: entry.source_id,
                    description: line
                        .description
                        .clone()
                        .unwrap_or(entry.description.clone()),
                    description_ar: entry.description_ar.clone(),
                    debit: line.debit_amount,
                    credit: line.credit_amount,
                    running_balance,
                    contra_account_name: None,
                });
            }
        }

        let closing_balance = opening_balance + total_debit - total_credit;

        Ok(StatementResponse {
            account_id: target_account.as_ref().map(|a| a.id),
            account_code: target_account.as_ref().map(|a| a.code.clone()),
            account_name: target_account.as_ref().map(|a| a.name.clone()),
            account_name_ar: target_account.as_ref().and_then(|a| a.name_ar.clone()),
            entity_type: filter.entity_type,
            entity_id: filter.entity_id,
            currency,
            opening_balance,
            total_debit,
            total_credit,
            closing_balance,
            transactions,
        })
    }

    /// Create a structured child sub-account in the Chart of Accounts tree
    pub async fn create_sub_account(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        input: CreateSubAccountInput,
    ) -> Result<Account, DomainError> {
        self.permission_checker
            .check(ctx, Permission::FinanceWrite)
            .await?;

        // 1. Find parent account
        let parent = if let Some(pid) = input.parent_id {
            self.account_repo
                .find_by_id(pid)
                .await?
                .ok_or_else(|| DomainError::NotFound(format!("Parent account {} not found", pid)))?
        } else if let Some(ref pcode) = input.parent_code {
            self.account_repo
                .find_by_code(org_id, pcode)
                .await?
                .ok_or_else(|| {
                    DomainError::NotFound(format!("Parent account with code '{}' not found", pcode))
                })?
        } else {
            return Err(DomainError::Validation(
                "Must provide parent_id or parent_code to create sub-account".into(),
            ));
        };

        if parent.org_id != org_id {
            return Err(DomainError::Unauthorized("Organization mismatch".into()));
        }

        let now = Utc::now();
        let sub_id = Uuid::new_v4();

        // Compute code if not supplied
        let code = input.code.unwrap_or_else(|| {
            format!(
                "{}-{}",
                parent.code,
                sub_id.to_string().chars().take(4).collect::<String>()
            )
        });

        let path = format!("{}{}/", parent.path, code);
        let depth = parent.depth + 1;
        let currency_code = input.currency_code.unwrap_or(parent.currency_code);

        let sub_account = Account {
            id: sub_id,
            org_id,
            code: code.clone(),
            name: input.name,
            name_ar: input.name_ar,
            account_type: parent.account_type,
            sub_type: parent.sub_type,
            parent_id: Some(parent.id),
            path,
            depth,
            currency_code,
            entity_type: input.entity_type,
            entity_id: input.entity_id,
            is_control_account: false,
            is_system_account: false,
            is_active: true,
            balances: serde_json::json!({}),
            created_by: ctx.user_id(),
            updated_by: ctx.user_id(),
            version: 1,
            created_at: now,
            updated_at: now,
        };

        self.account_repo.save(&sub_account).await?;

        let _ = self
            .event_bus
            .publish(
                ctx,
                &DomainEvent::AccountCreated {
                    account_id: sub_id,
                    org_id,
                    code,
                    timestamp: now,
                },
            )
            .await;

        Ok(sub_account)
    }
}
