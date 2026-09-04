use async_trait::async_trait;
use orbis_app::context::RequestContext;
use orbis_app::ports::event_bus::EventBus;
use orbis_domain::events::DomainEvent;
use orbis_domain::errors::DomainError;
use orbis_app::usecases::workflow::engine::WorkflowEngine;
use std::sync::Arc;

pub struct MemoryEventBus {
    pub workflow_engine: Option<Arc<WorkflowEngine>>,
}

#[async_trait]
impl EventBus for MemoryEventBus {
    async fn publish(&self, _ctx: &RequestContext, event: &DomainEvent) -> Result<(), DomainError> {
        println!("Publishing event (Outbox stub): {:?}", event);
        if let Some(engine) = &self.workflow_engine {
            let _ = engine.handle_event(_ctx, event).await;
        }
        Ok(())
    }
}
