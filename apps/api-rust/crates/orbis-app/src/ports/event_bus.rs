use async_trait::async_trait;
use orbis_domain::events::DomainEvent;
use orbis_domain::errors::DomainError;
use crate::context::RequestContext;

#[async_trait]
pub trait EventBus: Send + Sync {
    async fn publish(&self, ctx: &RequestContext, event: &DomainEvent) -> Result<(), DomainError>;
}
