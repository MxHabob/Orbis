use crate::context::RequestContext;
use async_trait::async_trait;
use orbis_domain::errors::DomainError;
use orbis_domain::events::DomainEvent;

#[async_trait]
pub trait EventBus: Send + Sync {
    async fn publish(&self, ctx: &RequestContext, event: &DomainEvent) -> Result<(), DomainError>;
}
