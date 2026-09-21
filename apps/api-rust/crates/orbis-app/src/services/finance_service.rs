use crate::ports::repositories::{PaymentRepository, JournalEntryRepository, StudentFeeRepository, FeeTemplateRepository, CurrencyRepository, PolicyRepository, AccountRepository};
use crate::ports::event_bus::EventBus;
use crate::ports::gateways::{PaymentGatewayPort, NotificationPort};
use crate::ports::iam::PermissionCheckerPort;
use orbis_domain::iam::permission::Permission;
use crate::context::RequestContext;
use orbis_domain::entities::payment::{Payment, PaymentMethod};
use orbis_domain::entities::journal_entry::{JournalEntry, JournalEntryLine, JournalPostingStatus};
use orbis_domain::entities::account::Account;
use orbis_domain::entities::fee_template::FeeTemplate;
use orbis_domain::entities::student_fee::{StudentFee, FeeStatus};
use orbis_domain::queries::filters::{PaymentFilter, StudentFeeFilter, PaginatedList};
use orbis_domain::events::DomainEvent;
use orbis_domain::errors::DomainError;
use uuid::Uuid;
use chrono::Utc;
use rust_decimal::Decimal;
use std::sync::Arc;

pub struct FinanceService {
    pub payment_repo: Arc<dyn PaymentRepository>,
    pub journal_repo: Arc<dyn JournalEntryRepository>,
    pub student_fee_repo: Arc<dyn StudentFeeRepository>,
    pub fee_template_repo: Arc<dyn FeeTemplateRepository>,
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
        self.permission_checker.check(ctx, Permission::ProcessPayment).await?;

        if amount <= Decimal::ZERO {
            return Err(DomainError::Validation("Payment amount must be greater than zero".into()));
        }

        let now = Utc::now();
        let accounting_date = now.date_naive();
        let journal_entry_id = Uuid::new_v4();
        let payment_id = Uuid::new_v4();

        // 1. Fetch org settings to determine base currency
        let org_settings = self.policy_repo.get_settings(org_id).await?;
        let base_currency = org_settings.finance.currency;

        // 2. Fetch target account to determine transaction currency
        let account = self.account_repo.find_by_id(account_id).await?
            .ok_or_else(|| DomainError::Validation(format!("Account {} not found", account_id)))?;
        let transaction_currency = account.currency_code;

        let mut exchange_rate_value = Decimal::from(1);
        if transaction_currency != base_currency {
            let rate_opt = self.currency_repo.get_exchange_rate(org_id, &base_currency, &transaction_currency).await?;
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
        if matches!(method, PaymentMethod::CreditCard | PaymentMethod::Mada | PaymentMethod::ApplePay) {
            if let Some(src) = source {
                let amount_f64 = rust_decimal::prelude::ToPrimitive::to_f64(&amount).unwrap_or(0.0);
                let gateway_ref = self.payment_gateway.process_payment(ctx, amount_f64, &transaction_currency, &src).await?;
                final_reference = Some(gateway_ref);
            } else {
                return Err(DomainError::Validation(format!("Payment method '{}' requires a source token", method)));
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
                        "Payment amount ({}) exceeds fee balance due ({})", amount, fee.balance_due
                    )));
                }
                student_resolved_id = Some(fee.student_id);
                enrollment_resolved_id = fee.enrollment_id;
                fee.apply_payment(amount);
                self.student_fee_repo.save(&fee).await?;

                if let Some(template) = self.fee_template_repo.find_by_id(fee.fee_template_id).await? {
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
            reference_number: format!("PAY-{}", payment_id.to_string().chars().take(8).collect::<String>()),
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
            receipt_number: Some(format!("REC-{}", payment_id.to_string().chars().take(8).collect::<String>())),
            journal_entry_id: Some(journal_entry_id),
            payer_name,
            payer_phone,
            notes: None,
            paid_at: now,
            created_by: ctx.user_id(),
            created_at: now,
        };
        self.payment_repo.save(&payment).await?;

        // 7. Publish Events
        let _ = self.event_bus.publish(ctx, &DomainEvent::JournalEntryPosted {
            entry_id: journal_entry_id,
            amount,
            timestamp: now,
        }).await;

        let _ = self.event_bus.publish(ctx, &DomainEvent::PaymentReceived {
            payment_id,
            fee_id,
            amount,
            timestamp: now,
        }).await;

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
        self.permission_checker.check(ctx, Permission::FinanceWrite).await?;

        let template = self.fee_template_repo.find_by_id(template_id).await?
            .ok_or_else(|| DomainError::NotFound(format!("Fee Template {} not found", template_id)))?;

        let discount = discount_amount.unwrap_or(Decimal::ZERO);
        let net_amount = (template.default_amount - discount).max(Decimal::ZERO);

        let now = Utc::now();
        let fee_id = Uuid::new_v4();

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
            due_date: due_date.unwrap_or_else(|| (now + chrono::Duration::days(30)).date_naive()),
            discount_details: None,
            installment_number: None,
            notes: None,
            created_at: now,
            updated_at: now,
        };

        self.student_fee_repo.save(&fee).await?;

        let _ = self.event_bus.publish(ctx, &DomainEvent::StudentFeeGenerated {
            fee_id,
            student_id,
            enrollment_id,
            net_amount,
            timestamp: now,
        }).await;

        Ok(fee)
    }

    pub async fn create_fee_template(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        name: String,
        amount: Decimal,
        currency_code: String,
        receivable_account_id: Uuid,
        revenue_account_id: Uuid,
    ) -> Result<FeeTemplate, DomainError> {
        self.permission_checker.check(ctx, Permission::FinanceWrite).await?;

        let now = Utc::now();
        let tmpl_id = Uuid::new_v4();

        let tmpl = FeeTemplate {
            id: tmpl_id,
            org_id,
            code: format!("FEE-{}", tmpl_id.to_string().chars().take(6).collect::<String>()),
            name,
            name_ar: None,
            category: "tuition".into(),
            default_amount: amount,
            currency_code,
            frequency: orbis_domain::entities::fee_template::FeeFrequency::PerTerm,
            receivable_account_id,
            revenue_account_id,
            applicable_levels: None,
            applicable_study_modes: None,
            is_mandatory: true,
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

    /// Create general ledger account
    pub async fn create_account(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        code: String,
        name: String,
        account_type: orbis_domain::entities::account::AccountType,
        currency_code: String,
        parent_id: Option<Uuid>,
    ) -> Result<Account, DomainError> {
        self.permission_checker.check(ctx, Permission::FinanceWrite).await?;

        let now = Utc::now();
        let path = format!("/{}", code);
        let account = Account {
            id: Uuid::new_v4(),
            org_id,
            code,
            name,
            name_ar: None,
            account_type,
            sub_type: None,
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
            created_at: now,
            updated_at: now,
        };

        self.account_repo.save(&account).await?;
        Ok(account)
    }

    /// List general ledger accounts
    pub async fn list_accounts(&self, org_id: Uuid) -> Result<Vec<Account>, DomainError> {
        self.account_repo.find_by_org(org_id).await
    }

    /// List student fees
    pub async fn list_student_fees(&self, org_id: Uuid, filter: StudentFeeFilter) -> Result<PaginatedList<StudentFee>, DomainError> {
        self.student_fee_repo.list(org_id, filter).await
    }

    /// List payments
    pub async fn list_payments(&self, org_id: Uuid, filter: PaymentFilter) -> Result<PaginatedList<Payment>, DomainError> {
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
        self.permission_checker.check(ctx, Permission::FinanceWrite).await?;

        let mut original_entry = self.journal_repo.find_by_id(entry_id).await?
            .ok_or_else(|| DomainError::NotFound(format!("Journal entry {} not found", entry_id)))?;

        if original_entry.org_id != org_id {
            return Err(DomainError::Unauthorized("Organization mismatch".into()));
        }

        if original_entry.is_reversed {
            return Err(DomainError::Validation("Journal entry is already reversed".into()));
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
        let reversal_lines: Vec<JournalEntryLine> = original_entry.lines.into_iter().map(|l| {
            JournalEntryLine {
                id: Uuid::new_v4(),
                account_id: l.account_id,
                debit_amount: l.credit_amount,
                credit_amount: l.debit_amount,
                description: Some(format!("Reversal: {}", l.description.unwrap_or_default())),
                entity_type: l.entity_type,
                entity_id: l.entity_id,
            }
        }).collect();

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
            description: format!("Reversal of {}: {}", original_entry.reference_number, reason),
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
        self.permission_checker.check(ctx, Permission::ProcessPayment).await?;

        let mut fee = self.student_fee_repo.find_by_id(fee_id).await?
            .ok_or_else(|| DomainError::NotFound(format!("Student fee {} not found", fee_id)))?;

        if fee.org_id != org_id {
            return Err(DomainError::Unauthorized("Organization mismatch".into()));
        }

        if refund_amount <= Decimal::ZERO || refund_amount > fee.paid_amount {
            return Err(DomainError::Validation(format!(
                "Invalid refund amount: {} (Paid: {})", refund_amount, fee.paid_amount
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
        self.permission_checker.check(ctx, Permission::FinanceWrite).await?;

        if num_installments < 2 || due_dates.len() != num_installments as usize {
            return Err(DomainError::Validation("Installment count must match number of due dates".into()));
        }

        let base_fee = self.student_fee_repo.find_by_id(fee_id).await?
            .ok_or_else(|| DomainError::NotFound(format!("Student fee {} not found", fee_id)))?;

        if base_fee.org_id != org_id {
            return Err(DomainError::Unauthorized("Organization mismatch".into()));
        }

        if base_fee.paid_amount > Decimal::ZERO {
            return Err(DomainError::Validation("Cannot split fee that already has payments applied".into()));
        }

        let installment_amount = (base_fee.net_amount / Decimal::from(num_installments)).round_dp(2);
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
                created_at: now,
                updated_at: now,
            };
            self.student_fee_repo.save(&inst_fee).await?;
            created_fees.push(inst_fee);
        }

        Ok(created_fees)
    }
}
