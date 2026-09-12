use async_trait::async_trait;
use orbis_app::ports::repositories::DocumentRepository;
use orbis_domain::entities::document::Document as DomainDocument;
use orbis_domain::errors::DomainError;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set, ColumnTrait, QueryFilter};
use uuid::Uuid;

use crate::models::document::{self, Entity as DocumentEntity};

pub struct DocumentRepoImpl {
    pub db: DatabaseConnection,
}

fn map_document_model_to_domain(m: document::Model) -> DomainDocument {
    DomainDocument {
        id: m.id,
        org_id: m.org_id,
        uploaded_by: m.uploaded_by,
        entity_type: m.entity_type,
        entity_id: m.entity_id,
        name: m.name,
        description: m.description,
        category: m.category,
        asset_type: m.asset_type,
        school_year: m.school_year,
        term: m.term,
        tags: m.tags,
        visibility: m.visibility,
        file_size_bytes: m.file_size_bytes,
        mime_type: m.mime_type,
        extension: m.extension,
        storage_provider: m.storage_provider,
        file_path: m.file_path,
        url: m.url,
        thumbnail_path: m.thumbnail_path,
        preview_path: m.preview_path,
        deleted_at: m.deleted_at.map(|d| d.into()),
        created_at: m.created_at.into(),
        updated_at: m.updated_at.into(),
    }
}

#[async_trait]
impl DocumentRepository for DocumentRepoImpl {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<DomainDocument>, DomainError> {
        let model = DocumentEntity::find_by_id(id).one(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(model.map(map_document_model_to_domain))
    }

    async fn find_by_org(&self, org_id: Uuid) -> Result<Vec<DomainDocument>, DomainError> {
        let models = DocumentEntity::find()
            .filter(document::Column::OrgId.eq(org_id))
            .filter(document::Column::DeletedAt.is_null())
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(models.into_iter().map(map_document_model_to_domain).collect())
    }

    async fn find_by_entity(&self, entity_type: &str, entity_id: Uuid) -> Result<Vec<DomainDocument>, DomainError> {
        let models = DocumentEntity::find()
            .filter(document::Column::EntityType.eq(entity_type))
            .filter(document::Column::EntityId.eq(entity_id))
            .filter(document::Column::DeletedAt.is_null())
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(models.into_iter().map(map_document_model_to_domain).collect())
    }

    async fn save(&self, document: &DomainDocument) -> Result<(), DomainError> {
        let active = document::ActiveModel {
            id: Set(document.id),
            org_id: Set(document.org_id),
            uploaded_by: Set(document.uploaded_by),
            entity_type: Set(document.entity_type.clone()),
            entity_id: Set(document.entity_id),
            name: Set(document.name.clone()),
            description: Set(document.description.clone()),
            category: Set(document.category.clone()),
            asset_type: Set(document.asset_type.clone()),
            school_year: Set(document.school_year.clone()),
            term: Set(document.term.clone()),
            tags: Set(document.tags.clone()),
            visibility: Set(document.visibility.clone()),
            file_size_bytes: Set(document.file_size_bytes),
            mime_type: Set(document.mime_type.clone()),
            extension: Set(document.extension.clone()),
            storage_provider: Set(document.storage_provider.clone()),
            file_path: Set(document.file_path.clone()),
            url: Set(document.url.clone()),
            thumbnail_path: Set(document.thumbnail_path.clone()),
            preview_path: Set(document.preview_path.clone()),
            deleted_at: Set(document.deleted_at.map(|d| d.into())),
            created_at: Set(document.created_at.into()),
            updated_at: Set(document.updated_at.into()),
        };

        if DocumentEntity::find_by_id(document.id).one(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?.is_some() {
            active.update(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        } else {
            active.insert(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        }
        Ok(())
    }
    
    async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        DocumentEntity::delete_by_id(id).exec(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(())
    }

    async fn delete_by_entity(&self, entity_type: &str, entity_id: Uuid) -> Result<(), DomainError> {
        DocumentEntity::delete_many()
            .filter(document::Column::EntityType.eq(entity_type))
            .filter(document::Column::EntityId.eq(entity_id))
            .exec(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(())
    }
}
