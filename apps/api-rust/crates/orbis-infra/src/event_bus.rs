use async_trait::async_trait;
use orbis_app::context::RequestContext;
use orbis_app::ports::event_bus::EventBus;
use orbis_domain::events::DomainEvent;
use orbis_domain::errors::DomainError;

pub struct MemoryEventBus;

#[async_trait]
impl EventBus for MemoryEventBus {
    async fn publish(&self, _ctx: &RequestContext, event: &DomainEvent) -> Result<(), DomainError> {
        println!("Publishing event (Outbox stub): {:?}", event);
        Ok(())
    }
}
