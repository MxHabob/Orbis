use chrono::Utc;
use std::sync::Arc;
use uuid::Uuid;

use orbis_domain::entities::document::Document;
use orbis_domain::entities::storage::{PresignMethod, PresignedUrlRequest};
use orbis_domain::errors::DomainError;
use orbis_domain::events::DomainEvent;
use orbis_domain::iam::permission::Permission;

use crate::context::RequestContext;
use crate::ports::event_bus::EventBus;
use crate::ports::iam::PermissionCheckerPort;
use crate::ports::repositories::DocumentRepository;
use crate::ports::storage::{ObjectStream, StorageAdapter};

#[derive(Debug, Clone)]
pub struct UploadFileInput {
    pub name: String,
    pub folder_id: Option<Uuid>,
    pub entity_type: Option<String>,
    pub entity_id: Option<Uuid>,
    pub org_unit_id: Option<Uuid>,
    pub category: Option<String>,
    pub asset_type: Option<String>,
    pub description: Option<String>,
    pub academic_year_id: Option<Uuid>,
    pub term_id: Option<Uuid>,
    pub tags: Option<String>,
    pub visibility: Option<String>,
    pub mime_type: String,
    pub data: Vec<u8>,
}

pub struct FileStorageService {
    pub storage_adapter: Arc<dyn StorageAdapter>,
    pub document_repo: Arc<dyn DocumentRepository>,
    pub event_bus: Arc<dyn EventBus>,
    pub permission_checker: Arc<dyn PermissionCheckerPort>,
}

impl FileStorageService {
    pub fn new(
        storage_adapter: Arc<dyn StorageAdapter>,
        document_repo: Arc<dyn DocumentRepository>,
        event_bus: Arc<dyn EventBus>,
        permission_checker: Arc<dyn PermissionCheckerPort>,
    ) -> Self {
        Self {
            storage_adapter,
            document_repo,
            event_bus,
            permission_checker,
        }
    }

    /// Canonical file upload pipeline.
    pub async fn upload_file(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        input: UploadFileInput,
    ) -> Result<Document, DomainError> {
        self.permission_checker
            .check(ctx, Permission::DocumentWrite)
            .await?;

        // 1. Basic validation
        if input.data.is_empty() {
            return Err(DomainError::Validation(
                "File payload cannot be empty".to_string(),
            ));
        }

        let doc_id = Uuid::new_v4();
        let sanitized_name = sanitize_filename(&input.name);
        let extension = get_extension(&sanitized_name);

        // 2. Generate canonical storage key
        let domain_folder = input
            .category
            .clone()
            .unwrap_or_else(|| "general".to_string());
        let entity_folder = input
            .entity_type
            .clone()
            .unwrap_or_else(|| "unattached".to_string());
        let entity_id_str = input
            .entity_id
            .map(|e| e.to_string())
            .unwrap_or_else(|| "root".to_string());

        let unit_part = if let Some(uid) = input.org_unit_id {
            format!("unit_{}/", uid)
        } else {
            String::new()
        };

        let storage_key = format!(
            "org_{}/{}{}/{}/{}/{}_{}",
            org_id, unit_part, domain_folder, entity_folder, entity_id_str, doc_id, sanitized_name
        );

        // 3. Store physical binary via storage adapter
        let metadata = self
            .storage_adapter
            .put_bytes(&storage_key, &input.data, &input.mime_type)
            .await
            .map_err(|e| DomainError::Storage(format!("Physical storage failed: {e}")))?;

        let now = Utc::now();
        let uploaded_by = match ctx.actor {
            orbis_domain::iam::actor::Actor::User(uid) => Some(uid),
            _ => None,
        };

        let doc = Document {
            id: doc_id,
            org_id,
            org_unit_id: input.org_unit_id,
            folder_id: input.folder_id,
            uploaded_by,
            entity_type: input.entity_type.clone(),
            entity_id: input.entity_id,
            name: input.name,
            description: input.description,
            category: domain_folder,
            asset_type: input.asset_type.unwrap_or_else(|| "file".to_string()),
            academic_year_id: input.academic_year_id,
            term_id: input.term_id,
            tags: input.tags,
            visibility: input.visibility.unwrap_or_else(|| "internal".to_string()),
            file_size_bytes: Some(metadata.size_bytes as i64),
            mime_type: Some(input.mime_type),
            extension,
            checksum_sha256: Some(metadata.sha256_checksum),
            storage_provider: self.storage_adapter.provider_id().to_string(),
            file_path: storage_key.clone(),
            storage_key: Some(storage_key.clone()),
            url: None,
            thumbnail_path: None,
            preview_path: None,
            is_archived: false,
            archived_at: None,
            archived_by: None,
            archive_reason: None,
            is_legal_hold: false,
            deleted_at: None,
            deleted_by: None,
            created_at: now,
            updated_at: now,
        };

        // 4. Save metadata record (with compensation cleanup on error)
        if let Err(err) = self.document_repo.save(&doc).await {
            let _ = self.storage_adapter.delete_object(&storage_key).await;
            return Err(err);
        }

        // 5. Emit event
        let event = DomainEvent::DocumentUploaded {
            document_id: doc_id,
            entity_type: input.entity_type,
            entity_id: input.entity_id,
            timestamp: now,
        };
        self.event_bus.publish(ctx, &event).await?;

        Ok(doc)
    }

    /// Read raw file bytes for download.
    pub async fn download_file_bytes(
        &self,
        ctx: &RequestContext,
        doc_id: Uuid,
    ) -> Result<(Document, Vec<u8>), DomainError> {
        self.permission_checker
            .check(ctx, Permission::DocumentRead)
            .await?;

        let doc = self
            .document_repo
            .find_by_id(doc_id)
            .await?
            .ok_or_else(|| DomainError::NotFound(format!("Document {doc_id} not found")))?;

        let key = doc.storage_key.as_deref().unwrap_or(&doc.file_path);
        let bytes = self
            .storage_adapter
            .get_bytes(key)
            .await
            .map_err(|e| DomainError::Storage(format!("Failed to retrieve file bytes: {e}")))?;

        Ok((doc, bytes))
    }

    /// Read file stream for large downloads.
    pub async fn download_file_stream(
        &self,
        ctx: &RequestContext,
        doc_id: Uuid,
    ) -> Result<(Document, ObjectStream), DomainError> {
        self.permission_checker
            .check(ctx, Permission::DocumentRead)
            .await?;

        let doc = self
            .document_repo
            .find_by_id(doc_id)
            .await?
            .ok_or_else(|| DomainError::NotFound(format!("Document {doc_id} not found")))?;

        let key = doc.storage_key.as_deref().unwrap_or(&doc.file_path);
        let stream =
            self.storage_adapter.get_object(key).await.map_err(|e| {
                DomainError::Storage(format!("Failed to retrieve file stream: {e}"))
            })?;

        Ok((doc, stream))
    }

    /// Generate secure presigned URL for direct download or upload.
    pub async fn generate_access_url(
        &self,
        ctx: &RequestContext,
        doc_id: Uuid,
        expires_in_seconds: u64,
    ) -> Result<String, DomainError> {
        self.permission_checker
            .check(ctx, Permission::DocumentRead)
            .await?;

        let doc = self
            .document_repo
            .find_by_id(doc_id)
            .await?
            .ok_or_else(|| DomainError::NotFound(format!("Document {doc_id} not found")))?;

        let key = doc.storage_key.as_deref().unwrap_or(&doc.file_path);

        if self.storage_adapter.capabilities().supports_presigned_urls {
            self.storage_adapter
                .generate_presigned_url(PresignedUrlRequest {
                    key: key.to_string(),
                    expires_in_seconds,
                    method: PresignMethod::Get,
                    content_type: doc.mime_type,
                })
                .await
                .map_err(|e| DomainError::Storage(e.to_string()))
        } else {
            // Local fallback path URL
            Ok(format!("/api/v1/storage/{}/file", doc_id))
        }
    }

    /// Archive a document.
    pub async fn archive_document(
        &self,
        ctx: &RequestContext,
        doc_id: Uuid,
        reason: Option<String>,
    ) -> Result<Document, DomainError> {
        self.permission_checker
            .check(ctx, Permission::DocumentWrite)
            .await?;

        let mut doc = self
            .document_repo
            .find_by_id(doc_id)
            .await?
            .ok_or_else(|| DomainError::NotFound(format!("Document {doc_id} not found")))?;

        let actor_id = match ctx.actor {
            orbis_domain::iam::actor::Actor::User(uid) => Some(uid),
            _ => None,
        };

        doc.is_archived = true;
        doc.archived_at = Some(Utc::now());
        doc.archived_by = actor_id;
        doc.archive_reason = reason.clone();
        doc.updated_at = Utc::now();

        self.document_repo.save(&doc).await?;

        let event = DomainEvent::DocumentArchived {
            document_id: doc_id,
            org_id: doc.org_id,
            reason,
            timestamp: doc.updated_at,
        };
        self.event_bus.publish(ctx, &event).await?;

        Ok(doc)
    }

    /// Restore a document from archive or trash.
    pub async fn restore_document(
        &self,
        ctx: &RequestContext,
        doc_id: Uuid,
    ) -> Result<Document, DomainError> {
        self.permission_checker
            .check(ctx, Permission::DocumentWrite)
            .await?;

        self.document_repo.restore(doc_id).await?;

        let doc = self
            .document_repo
            .find_by_id(doc_id)
            .await?
            .ok_or_else(|| DomainError::NotFound(format!("Document {doc_id} not found")))?;

        let event = DomainEvent::DocumentRestored {
            document_id: doc_id,
            org_id: doc.org_id,
            timestamp: Utc::now(),
        };
        self.event_bus.publish(ctx, &event).await?;

        Ok(doc)
    }

    /// Soft-delete a document to trash.
    pub async fn trash_document(
        &self,
        ctx: &RequestContext,
        doc_id: Uuid,
    ) -> Result<(), DomainError> {
        self.permission_checker
            .check(ctx, Permission::DocumentDelete)
            .await?;

        let actor_id = match ctx.actor {
            orbis_domain::iam::actor::Actor::User(uid) => Some(uid),
            _ => None,
        };

        self.document_repo.soft_delete(doc_id, actor_id).await?;

        let event = DomainEvent::DocumentDeleted {
            document_id: doc_id,
            timestamp: Utc::now(),
        };
        self.event_bus.publish(ctx, &event).await?;

        Ok(())
    }

    /// Permanently purge a document (deletes both database row and physical storage object).
    pub async fn purge_document(
        &self,
        ctx: &RequestContext,
        doc_id: Uuid,
    ) -> Result<(), DomainError> {
        self.permission_checker
            .check(ctx, Permission::DocumentDelete)
            .await?;

        let doc = self
            .document_repo
            .find_by_id(doc_id)
            .await?
            .ok_or_else(|| DomainError::NotFound(format!("Document {doc_id} not found")))?;

        // 1. Delete physical object
        let key = doc.storage_key.as_deref().unwrap_or(&doc.file_path);
        let _ = self.storage_adapter.delete_object(key).await;

        // 2. Delete database row
        self.document_repo.purge(doc_id).await?;

        let event = DomainEvent::DocumentPurged {
            document_id: doc_id,
            org_id: doc.org_id,
            timestamp: Utc::now(),
        };
        self.event_bus.publish(ctx, &event).await?;

        Ok(())
    }
}

fn sanitize_filename(name: &str) -> String {
    name.chars()
        .map(|c| match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '.' | '-' | '_' => c,
            _ => '_',
        })
        .collect()
}

fn get_extension(filename: &str) -> Option<String> {
    filename.rsplit('.').next().map(|ext| ext.to_lowercase())
}
