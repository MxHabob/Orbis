use crate::ports::repositories::DocumentRepository;
use crate::ports::event_bus::EventBus;
use crate::context::RequestContext;
use orbis_domain::entities::document::Document;
use orbis_domain::events::DomainEvent;
use orbis_domain::errors::DomainError;
use uuid::Uuid;
use chrono::Utc;
use std::sync::Arc;

pub struct UploadDocumentUseCase {
    pub document_repo: Arc<dyn DocumentRepository>,
    pub event_bus: Arc<dyn EventBus>,
}

impl UploadDocumentUseCase {
    pub async fn execute(
        &self,
        _ctx: &RequestContext,
        org_id: Uuid,
        uploaded_by: Option<Uuid>,
        entity_type: Option<String>,
        entity_id: Option<Uuid>,
        name: String,
        description: Option<String>,
        category: String,
        asset_type: String,
        school_year: Option<String>,
        term: Option<String>,
        tags: Option<String>,
        visibility: String,
        size: Option<i32>,
        mime_type: Option<String>,
        extension: Option<String>,
        storage_provider: String,
        path: String,
        url: Option<String>,
        thumbnail_path: Option<String>,
        preview_path: Option<String>,
    ) -> Result<Document, DomainError> {
        let document = Document {
            id: Uuid::new_v4(),
            org_id,
            uploaded_by,
            entity_type: entity_type.clone(),
            entity_id,
            name,
            description,
            category,
            asset_type,
            school_year,
            term,
            tags,
            visibility,
            size,
            mime_type,
            extension,
            storage_provider,
            path,
            url,
            thumbnail_path,
            preview_path,
            deleted_at: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        self.document_repo.save(&document).await?;

        let _ = self.event_bus.publish(_ctx, &DomainEvent::DocumentUploaded {
            document_id: document.id,
            entity_type,
            entity_id,
            timestamp: Utc::now(),
        }).await;

        Ok(document)
    }
}
