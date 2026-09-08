use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;
use orbis_domain::entities::document::Document;
use orbis_domain::events::DomainEvent;
use orbis_domain::errors::DomainError;
use orbis_domain::iam::permission::Permission;

use crate::context::RequestContext;
use crate::ports::repositories::DocumentRepository;
use crate::ports::event_bus::EventBus;
use crate::ports::iam::PermissionCheckerPort;

#[derive(Debug, Clone)]
pub struct CreateDocumentInput {
    pub uploaded_by: Option<Uuid>,
    pub entity_type: Option<String>,
    pub entity_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub category: String,
    pub asset_type: String,
    pub school_year: Option<String>,
    pub term: Option<String>,
    pub tags: Option<String>,
    pub visibility: String,
    pub size: Option<i32>,
    pub mime_type: Option<String>,
    pub extension: Option<String>,
    pub storage_provider: String,
    pub path: String,
    pub url: Option<String>,
    pub thumbnail_path: Option<String>,
    pub preview_path: Option<String>,
}

pub struct DocumentService {
    pub document_repo: Arc<dyn DocumentRepository>,
    pub event_bus: Arc<dyn EventBus>,
    pub permission_checker: Arc<dyn PermissionCheckerPort>,
}

impl DocumentService {
    pub fn new(
        document_repo: Arc<dyn DocumentRepository>,
        event_bus: Arc<dyn EventBus>,
        permission_checker: Arc<dyn PermissionCheckerPort>,
    ) -> Self {
        Self {
            document_repo,
            event_bus,
            permission_checker,
        }
    }

    /// Upload/Record a document metadata record
    pub async fn create_document(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        input: CreateDocumentInput,
    ) -> Result<Document, DomainError> {
        self.permission_checker.check(ctx, Permission::DocumentWrite).await?;

        let now = Utc::now();
        let doc_id = Uuid::new_v4();

        let doc = Document {
            id: doc_id,
            org_id,
            uploaded_by: input.uploaded_by,
            entity_type: input.entity_type.clone(),
            entity_id: input.entity_id,
            name: input.name,
            description: input.description,
            category: input.category,
            asset_type: input.asset_type,
            school_year: input.school_year,
            term: input.term,
            tags: input.tags,
            visibility: input.visibility,
            size: input.size,
            mime_type: input.mime_type,
            extension: input.extension,
            storage_provider: input.storage_provider,
            path: input.path,
            url: input.url,
            thumbnail_path: input.thumbnail_path,
            preview_path: input.preview_path,
            deleted_at: None,
            created_at: now,
            updated_at: now,
        };

        self.document_repo.save(&doc).await?;

        let event = DomainEvent::DocumentUploaded {
            document_id: doc_id,
            entity_type: input.entity_type,
            entity_id: input.entity_id,
            timestamp: now,
        };
        self.event_bus.publish(ctx, &event).await?;

        Ok(doc)
    }

    /// Get document by ID
    pub async fn get_document(&self, id: Uuid) -> Result<Option<Document>, DomainError> {
        self.document_repo.find_by_id(id).await
    }

    /// List documents for an organization
    pub async fn list_documents_by_org(&self, org_id: Uuid) -> Result<Vec<Document>, DomainError> {
        self.document_repo.find_by_org(org_id).await
    }

    /// List documents attached to an entity (e.g., Student, Employee)
    pub async fn list_documents_by_entity(&self, entity_type: &str, entity_id: Uuid) -> Result<Vec<Document>, DomainError> {
        self.document_repo.find_by_entity(entity_type, entity_id).await
    }

    /// Delete document
    pub async fn delete_document(&self, ctx: &RequestContext, id: Uuid) -> Result<(), DomainError> {
        self.permission_checker.check(ctx, Permission::DocumentDelete).await?;

        self.document_repo.delete(id).await?;

        let event = DomainEvent::DocumentDeleted {
            document_id: id,
            timestamp: Utc::now(),
        };
        self.event_bus.publish(ctx, &event).await?;

        Ok(())
    }
}
