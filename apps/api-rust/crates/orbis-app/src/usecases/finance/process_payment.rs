use crate::ports::repositories::{PaymentRepository, JournalEntryRepository, StudentFeeRepository, FeeTemplateRepository};
use crate::ports::event_bus::EventBus;
use crate::context::RequestContext;
use orbis_domain::entities::payment::Payment;
use orbis_domain::entities::journal_entry::JournalEntry;
use orbis_domain::events::DomainEvent;
use orbis_domain::errors::DomainError;
use uuid::Uuid;
use chrono::Utc;
use std::sync::Arc;

pub struct ProcessPaymentUseCase {
    pub payment_repo: Arc<dyn PaymentRepository>,
    pub journal_repo: Arc<dyn JournalEntryRepository>,
    pub student_fee_repo: Arc<dyn StudentFeeRepository>,
    pub fee_template_repo: Arc<dyn FeeTemplateRepository>,
    pub event_bus: Arc<dyn EventBus>,
}

impl ProcessPaymentUseCase {
    pub async fn execute(
        &self,
        _ctx: &RequestContext,
        org_id: Uuid,
        fee_id: Option<Uuid>,
        account_id: Uuid,
        amount: f64,
        method: String,
        reference: Option<String>,
    ) -> Result<Payment, DomainError> {
        let now = Utc::now();
        let accounting_date = now.date_naive();
        
        let journal_entry_id = Uuid::new_v4();
        let payment_id = Uuid::new_v4();
        
        // 1. Update StudentFee if fee_id is provided
        let mut revenue_acct = Uuid::new_v4(); // Default fallback
        let mut receivable_acct = Uuid::new_v4();
        
        if let Some(f_id) = fee_id {
            if let Some(mut fee) = self.student_fee_repo.find_by_id(f_id).await? {
                fee.status = "Paid".to_string(); // Simplified for now
                fee.updated_at = now;
                self.student_fee_repo.save(&fee).await?;
                
                if let Some(template) = self.fee_template_repo.find_by_id(fee.fee_template_id).await? {
                    revenue_acct = template.revenue_account_id;
                    receivable_acct = template.receivable_account_id;
                }
            }
        }

        // 2. Create Journal Entry
        let entry = JournalEntry {
            id: journal_entry_id,
            org_id,
            transaction_ref: format!("PAY-{}", payment_id.to_string().chars().take(8).collect::<String>()),
            accounting_date,
            amount,
            debit_account_id: account_id, // Cash/Bank
            credit_account_id: receivable_acct, // AR
            description: "Payment Received".to_string(),
            created_at: now,
        };
        self.journal_repo.save(&entry).await?;

        // 3. Create Payment
        let payment = Payment {
            id: payment_id,
            org_id,
            fee_id,
            account_id,
            amount,
            method,
            reference,
            journal_entry_id: Some(journal_entry_id),
            paid_at: now,
            created_at: now,
        };
        self.payment_repo.save(&payment).await?;

        // 4. Publish Events
        self.event_bus.publish(_ctx, &DomainEvent::JournalEntryPosted {
            entry_id: journal_entry_id,
            amount,
            timestamp: now,
        }).await;

        self.event_bus.publish(_ctx, &DomainEvent::PaymentReceived {
            payment_id,
            fee_id,
            amount,
            timestamp: now,
        }).await;

        Ok(payment)
    }
}
