use crate::ports::repositories::{PaymentRepository, JournalEntryRepository, StudentFeeRepository, FeeTemplateRepository, CurrencyRepository, PolicyRepository, AccountRepository};
use crate::ports::event_bus::EventBus;
use crate::ports::gateways::{PaymentGatewayPort, NotificationPort};
use crate::ports::iam::PermissionCheckerPort;
use orbis_domain::iam::permission::Permission;
use crate::context::RequestContext;
use orbis_domain::entities::payment::{Payment, PaymentMethod};
use orbis_domain::entities::journal_entry::{JournalEntry, JournalEntryLine, JournalPostingStatus};
use orbis_domain::events::DomainEvent;
use orbis_domain::errors::DomainError;
use uuid::Uuid;
use chrono::Utc;
use rust_decimal::Decimal;
use std::sync::Arc;

pub struct ProcessPaymentUseCase {
    pub payment_repo: Arc<dyn PaymentRepository>,
    pub journal_repo: Arc<dyn JournalEntryRepository>,
    pub student_fee_repo: Arc<dyn StudentFeeRepository>,
    pub fee_template_repo: Arc<dyn FeeTemplateRepository>,
    pub event_bus: Arc<dyn EventBus>,
    pub payment_gateway: Arc<dyn PaymentGatewayPort>,
    pub notification_gateway: Arc<dyn NotificationPort>,
    pub permission_checker: Arc<dyn PermissionCheckerPort>,
    pub currency_repo: Arc<dyn CurrencyRepository>,
    pub policy_repo: Arc<dyn PolicyRepository>,
    pub account_repo: Arc<dyn AccountRepository>,
}

impl ProcessPaymentUseCase {
    pub async fn execute(
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

        // Fetch org settings to determine base currency
        let org_settings = self.policy_repo.get_settings(org_id).await?;
        let base_currency = org_settings.finance.currency;
        
        // Fetch target account to determine transaction currency
        let account = self.account_repo.find_by_id(account_id).await?
            .ok_or_else(|| DomainError::Validation(format!("Account {} not found", account_id)))?;
        let transaction_currency = account.currency;
        
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

        // Gateway processing for electronic payments
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
        
        // 1. Update StudentFee if fee_id is provided
        let mut receivable_acct = account_id;
        let mut student_resolved_id = student_id;
        let mut enrollment_resolved_id = enrollment_id;
        
        if let Some(f_id) = fee_id {
            if let Some(mut fee) = self.student_fee_repo.find_by_id(f_id).await? {
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

        // 2. Construct balanced multi-line double entry voucher
        let lines = vec![
            JournalEntryLine {
                id: Uuid::new_v4(),
                account_id, // Debit Cash/Bank
                debit: amount,
                credit: Decimal::ZERO,
                description: Some(format!("Payment received for student fee")),
                entity_type: Some("student".into()),
                entity_id: student_resolved_id,
            },
            JournalEntryLine {
                id: Uuid::new_v4(),
                account_id: receivable_acct, // Credit Accounts Receivable / Revenue
                debit: Decimal::ZERO,
                credit: amount,
                description: Some(format!("AR settlement")),
                entity_type: Some("student".into()),
                entity_id: student_resolved_id,
            },
        ];

        let entry = JournalEntry {
            id: journal_entry_id,
            org_id,
            transaction_ref: format!("PAY-{}", payment_id.to_string().chars().take(8).collect::<String>()),
            accounting_date,
            amount,
            currency_id: transaction_currency.clone(),
            exchange_rate: exchange_rate_value,
            base_amount,
            debit_account_id: account_id,
            credit_account_id: receivable_acct,
            description: format!("Payment Receipt for Student"),
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

        // 3. Create Payment
        let payment = Payment {
            id: payment_id,
            org_id,
            fee_id,
            enrollment_id: enrollment_resolved_id,
            student_id: student_resolved_id,
            account_id,
            amount,
            currency: transaction_currency,
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

        // 4. Publish Events
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

        // 5. Send Notification
        let receipt_body = format!("Payment of {} received successfully. Reference: {:?}", amount, payment.reference);
        let _ = self.notification_gateway.send_email(ctx, "parent@example.com", "Payment Receipt", &receipt_body).await;

        Ok(payment)
    }
}
