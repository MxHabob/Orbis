use crate::context::RequestContext;
use async_trait::async_trait;
use orbis_domain::errors::DomainError;

#[async_trait]
pub trait PaymentPort: Send + Sync {
    async fn process_payment(
        &self,
        ctx: &RequestContext,
        amount: u64,
        currency: &str,
    ) -> Result<String, DomainError>;
}
