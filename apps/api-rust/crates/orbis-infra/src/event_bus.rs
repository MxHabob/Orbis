use async_trait::async_trait;
use orbis_app::autopilot::engine::AutopilotEngine;
use orbis_app::context::RequestContext;
use orbis_app::ports::event_bus::EventBus;
use orbis_app::services::NotificationService;
use orbis_domain::errors::DomainError;
use orbis_domain::events::DomainEvent;
use std::sync::{Arc, RwLock};

pub struct MemoryEventBus {
    pub workflow_engine: RwLock<Option<Arc<AutopilotEngine>>>,
    pub notification_service: RwLock<Option<Arc<NotificationService>>>,
}

impl Default for MemoryEventBus {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryEventBus {
    pub fn new() -> Self {
        Self {
            workflow_engine: RwLock::new(None),
            notification_service: RwLock::new(None),
        }
    }

    pub fn set_workflow_engine(&self, engine: Arc<AutopilotEngine>) {
        if let Ok(mut lock) = self.workflow_engine.write() {
            *lock = Some(engine);
        }
    }

    pub fn set_notification_service(&self, service: Arc<NotificationService>) {
        if let Ok(mut lock) = self.notification_service.write() {
            *lock = Some(service);
        }
    }
}

#[async_trait]
impl EventBus for MemoryEventBus {
    async fn publish(&self, ctx: &RequestContext, event: &DomainEvent) -> Result<(), DomainError> {
        let engine = self
            .workflow_engine
            .read()
            .ok()
            .and_then(|guard| guard.clone());
        if let Some(engine) = engine {
            let _ = engine.handle_event(ctx, event).await;
        }

        let notif_service = self
            .notification_service
            .read()
            .ok()
            .and_then(|guard| guard.clone());
        if let Some(service) = notif_service {
            let _ = service.handle_domain_event(ctx, event).await;
        }

        Ok(())
    }
}
