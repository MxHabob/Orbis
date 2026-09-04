use crate::ports::repositories::DocumentRepository;
use crate::ports::event_bus::EventBus;
use crate::context::RequestContext;
use orbis_domain::events::DomainEvent;
use orbis_domain::errors::DomainError;
use uuid::Uuid;
use chrono::Utc;
use std::sync::Arc;

pub struct DeleteDocumentUseCase {
    pub document_repo: Arc<dyn DocumentRepository>,
    pub event_bus: Arc<dyn EventBus>,
}

impl DeleteDocumentUseCase {
    pub async fn execute(
        &self,
        _ctx: &RequestContext,
        document_id: Uuid,
    ) -> Result<(), DomainError> {
        if let Some(mut doc) = self.document_repo.find_by_id(document_id).await? {
            // Soft delete
            doc.deleted_at = Some(Utc::now());
            self.document_repo.save(&doc).await?;

            let _ = self.event_bus.publish(_ctx, &DomainEvent::DocumentDeleted {
                document_id,
                timestamp: Utc::now(),
            }).await;
            Ok(())
        } else {
            Err(DomainError::NotFound("Document not found".to_string()))
        }
    }
}
