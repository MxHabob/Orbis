use async_trait::async_trait;
use chrono::Utc;
use orbis_app::ports::repositories::DocumentRepository;
use orbis_domain::entities::document::Document as DomainDocument;
use orbis_domain::errors::DomainError;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

use crate::models::document::{self, Entity as DocumentEntity};

pub struct DocumentRepoImpl {
    pub db: DatabaseConnection,
}

fn map_document_model_to_domain(m: document::Model) -> DomainDocument {
    DomainDocument {
        id: m.id,
        org_id: m.org_id,
        org_unit_id: m.org_unit_id,
        folder_id: m.folder_id,
        uploaded_by: m.uploaded_by,
        entity_type: m.entity_type,
        entity_id: m.entity_id,
        name: m.name,
        description: m.description,
        category: m.category,
        asset_type: m.asset_type,
        academic_year_id: m.academic_year_id,
        term_id: m.term_id,
        tags: m.tags,
        visibility: m.visibility,
        file_size_bytes: m.file_size_bytes,
        mime_type: m.mime_type,
        extension: m.extension,
        checksum_sha256: m.checksum_sha256,
        storage_provider: m.storage_provider,
        file_path: m.file_path,
        storage_key: m.storage_key,
        url: m.url,
        thumbnail_path: m.thumbnail_path,
        preview_path: m.preview_path,
        is_archived: m.is_archived,
        archived_at: m.archived_at.map(|d| d.into()),
        archived_by: m.archived_by,
        archive_reason: m.archive_reason,
        is_legal_hold: m.is_legal_hold,
        deleted_at: m.deleted_at.map(|d| d.into()),
        deleted_by: m.deleted_by,
        created_at: m.created_at.into(),
        updated_at: m.updated_at.into(),
    }
}

#[async_trait]
impl DocumentRepository for DocumentRepoImpl {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<DomainDocument>, DomainError> {
        let model = DocumentEntity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(model.map(map_document_model_to_domain))
    }

    async fn find_by_org(&self, org_id: Uuid) -> Result<Vec<DomainDocument>, DomainError> {
        let models = DocumentEntity::find()
            .filter(document::Column::OrgId.eq(org_id))
            .filter(document::Column::DeletedAt.is_null())
            .filter(document::Column::IsArchived.eq(false))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(models
            .into_iter()
            .map(map_document_model_to_domain)
            .collect())
    }

    async fn find_by_entity(
        &self,
        entity_type: &str,
        entity_id: Uuid,
    ) -> Result<Vec<DomainDocument>, DomainError> {
        let models = DocumentEntity::find()
            .filter(document::Column::EntityType.eq(entity_type))
            .filter(document::Column::EntityId.eq(entity_id))
            .filter(document::Column::DeletedAt.is_null())
            .filter(document::Column::IsArchived.eq(false))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(models
            .into_iter()
            .map(map_document_model_to_domain)
            .collect())
    }

    async fn find_by_folder(
        &self,
        org_id: Uuid,
        folder_id: Uuid,
    ) -> Result<Vec<DomainDocument>, DomainError> {
        let models = DocumentEntity::find()
            .filter(document::Column::OrgId.eq(org_id))
            .filter(document::Column::FolderId.eq(Some(folder_id)))
            .filter(document::Column::DeletedAt.is_null())
            .filter(document::Column::IsArchived.eq(false))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(models
            .into_iter()
            .map(map_document_model_to_domain)
            .collect())
    }

    async fn find_by_curriculum(
        &self,
        org_id: Uuid,
        academic_year_id: Option<Uuid>,
        term_id: Option<Uuid>,
    ) -> Result<Vec<DomainDocument>, DomainError> {
        let mut query = DocumentEntity::find()
            .filter(document::Column::OrgId.eq(org_id))
            .filter(document::Column::Category.eq("curriculum"))
            .filter(document::Column::DeletedAt.is_null())
            .filter(document::Column::IsArchived.eq(false));

        if let Some(ay) = academic_year_id {
            query = query.filter(document::Column::AcademicYearId.eq(Some(ay)));
        }
        if let Some(t) = term_id {
            query = query.filter(document::Column::TermId.eq(Some(t)));
        }

        let models = query
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(models
            .into_iter()
            .map(map_document_model_to_domain)
            .collect())
    }

    async fn find_archived(&self, org_id: Uuid) -> Result<Vec<DomainDocument>, DomainError> {
        let models = DocumentEntity::find()
            .filter(document::Column::OrgId.eq(org_id))
            .filter(document::Column::DeletedAt.is_null())
            .filter(document::Column::IsArchived.eq(true))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(models
            .into_iter()
            .map(map_document_model_to_domain)
            .collect())
    }

    async fn find_trash(&self, org_id: Uuid) -> Result<Vec<DomainDocument>, DomainError> {
        let models = DocumentEntity::find()
            .filter(document::Column::OrgId.eq(org_id))
            .filter(document::Column::DeletedAt.is_not_null())
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(models
            .into_iter()
            .map(map_document_model_to_domain)
            .collect())
    }

    async fn save(&self, document: &DomainDocument) -> Result<(), DomainError> {
        let active = document::ActiveModel {
            id: Set(document.id),
            org_id: Set(document.org_id),
            org_unit_id: Set(document.org_unit_id),
            folder_id: Set(document.folder_id),
            uploaded_by: Set(document.uploaded_by),
            entity_type: Set(document.entity_type.clone()),
            entity_id: Set(document.entity_id),
            name: Set(document.name.clone()),
            description: Set(document.description.clone()),
            category: Set(document.category.clone()),
            asset_type: Set(document.asset_type.clone()),
            academic_year_id: Set(document.academic_year_id),
            term_id: Set(document.term_id),
            tags: Set(document.tags.clone()),
            visibility: Set(document.visibility.clone()),
            file_size_bytes: Set(document.file_size_bytes),
            mime_type: Set(document.mime_type.clone()),
            extension: Set(document.extension.clone()),
            checksum_sha256: Set(document.checksum_sha256.clone()),
            storage_provider: Set(document.storage_provider.clone()),
            file_path: Set(document.file_path.clone()),
            storage_key: Set(document.storage_key.clone()),
            url: Set(document.url.clone()),
            thumbnail_path: Set(document.thumbnail_path.clone()),
            preview_path: Set(document.preview_path.clone()),
            is_archived: Set(document.is_archived),
            archived_at: Set(document.archived_at.map(|d| d.into())),
            archived_by: Set(document.archived_by),
            archive_reason: Set(document.archive_reason.clone()),
            is_legal_hold: Set(document.is_legal_hold),
            deleted_at: Set(document.deleted_at.map(|d| d.into())),
            deleted_by: Set(document.deleted_by),
            created_at: Set(document.created_at.into()),
            updated_at: Set(document.updated_at.into()),
        };

        if DocumentEntity::find_by_id(document.id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?
            .is_some()
        {
            active
                .update(&self.db)
                .await
                .map_err(|e| DomainError::Internal(e.to_string()))?;
        } else {
            active
                .insert(&self.db)
                .await
                .map_err(|e| DomainError::Internal(e.to_string()))?;
        }
        Ok(())
    }

    async fn soft_delete(&self, id: Uuid, deleted_by: Option<Uuid>) -> Result<(), DomainError> {
        let doc = DocumentEntity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?
            .ok_or_else(|| DomainError::NotFound(format!("Document {id} not found")))?;

        if doc.is_legal_hold {
            return Err(DomainError::BusinessRule(
                "Cannot delete a document with active legal hold".to_string(),
            ));
        }

        let mut active: document::ActiveModel = doc.into();
        active.deleted_at = Set(Some(Utc::now().into()));
        active.deleted_by = Set(deleted_by);
        active.updated_at = Set(Utc::now().into());
        active
            .update(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(())
    }

    async fn restore(&self, id: Uuid) -> Result<(), DomainError> {
        let doc = DocumentEntity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?
            .ok_or_else(|| DomainError::NotFound(format!("Document {id} not found")))?;

        let mut active: document::ActiveModel = doc.into();
        active.deleted_at = Set(None);
        active.deleted_by = Set(None);
        active.is_archived = Set(false);
        active.archived_at = Set(None);
        active.archived_by = Set(None);
        active.archive_reason = Set(None);
        active.updated_at = Set(Utc::now().into());
        active
            .update(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(())
    }

    async fn purge(&self, id: Uuid) -> Result<(), DomainError> {
        let doc = DocumentEntity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?
            .ok_or_else(|| DomainError::NotFound(format!("Document {id} not found")))?;

        if doc.is_legal_hold {
            return Err(DomainError::BusinessRule(
                "Cannot purge a document with active legal hold".to_string(),
            ));
        }

        DocumentEntity::delete_by_id(id)
            .exec(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(())
    }

    async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        self.soft_delete(id, None).await
    }

    async fn delete_by_entity(
        &self,
        entity_type: &str,
        entity_id: Uuid,
    ) -> Result<(), DomainError> {
        let docs = DocumentEntity::find()
            .filter(document::Column::EntityType.eq(entity_type))
            .filter(document::Column::EntityId.eq(entity_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        let now = Utc::now();
        for doc in docs {
            if !doc.is_legal_hold {
                let mut active: document::ActiveModel = doc.into();
                active.deleted_at = Set(Some(now.into()));
                active.updated_at = Set(now.into());
                let _ = active.update(&self.db).await;
            }
        }
        Ok(())
    }
}

use crate::models::document_requirement::{self as doc_req_model, Entity as DocRequirementEntity};
use orbis_app::ports::repositories::DocumentRequirementRepository;
use orbis_domain::entities::document_requirement::DocumentRequirement as DomainDocRequirement;

pub struct DocumentRequirementRepoImpl {
    pub db: DatabaseConnection,
}

impl DocumentRequirementRepoImpl {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl DocumentRequirementRepository for DocumentRequirementRepoImpl {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<DomainDocRequirement>, DomainError> {
        let model = DocRequirementEntity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(model.map(|m| m.into()))
    }

    async fn find_by_org(&self, org_id: Uuid) -> Result<Vec<DomainDocRequirement>, DomainError> {
        let models = DocRequirementEntity::find()
            .filter(doc_req_model::Column::OrgId.eq(org_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(models.into_iter().map(|m| m.into()).collect())
    }

    async fn find_by_target(
        &self,
        org_id: Uuid,
        target_type: &str,
        target_id: Option<Uuid>,
    ) -> Result<Vec<DomainDocRequirement>, DomainError> {
        let mut query = DocRequirementEntity::find()
            .filter(doc_req_model::Column::OrgId.eq(org_id))
            .filter(doc_req_model::Column::TargetType.eq(target_type));

        if let Some(tid) = target_id {
            query = query.filter(doc_req_model::Column::TargetId.eq(tid));
        }

        let models = query
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(models.into_iter().map(|m| m.into()).collect())
    }

    async fn save(&self, req: &DomainDocRequirement) -> Result<(), DomainError> {
        let active: doc_req_model::ActiveModel = req.into();
        let exists = DocRequirementEntity::find_by_id(req.id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        if exists.is_some() {
            active
                .update(&self.db)
                .await
                .map_err(|e| DomainError::Internal(e.to_string()))?;
        } else {
            active
                .insert(&self.db)
                .await
                .map_err(|e| DomainError::Internal(e.to_string()))?;
        }
        Ok(())
    }

    async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        DocRequirementEntity::delete_by_id(id)
            .exec(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(())
    }
}
