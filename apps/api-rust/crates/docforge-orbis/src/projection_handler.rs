use std::sync::Arc;

use async_trait::async_trait;
use sea_orm::DatabaseConnection;

use shared_kernel::event_bus::EventEnvelope;
use shared_kernel::event_loop::EventProjectionHandler;

use crate::event_handler;

pub struct DocForgeProjectionHandler;

#[async_trait]
impl EventProjectionHandler for DocForgeProjectionHandler {
    async fn on_event(&self, db: &DatabaseConnection, event: &EventEnvelope) {
        event_handler::handle_domain_event(db, event).await;
    }
}

pub fn handler() -> Arc<dyn EventProjectionHandler> {
    Arc::new(DocForgeProjectionHandler)
}
