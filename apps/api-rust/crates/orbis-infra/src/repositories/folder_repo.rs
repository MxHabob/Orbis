use async_trait::async_trait;
use chrono::Utc;
use orbis_app::ports::repositories::FolderRepository;
use orbis_domain::entities::folder::Folder as DomainFolder;
use orbis_domain::errors::DomainError;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, Set,
};
use uuid::Uuid;

use crate::models::folder::{self, Entity as FolderEntity};

pub struct FolderRepoImpl {
    pub db: DatabaseConnection,
}

impl FolderRepoImpl {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

fn map_folder_model_to_domain(m: folder::Model) -> DomainFolder {
    DomainFolder {
        id: m.id,
        org_id: m.org_id,
        parent_id: m.parent_id,
        name: m.name,
        path: m.path,
        folder_type: m.folder_type,
        entity_type: m.entity_type,
        entity_id: m.entity_id,
        org_unit_id: m.org_unit_id,
        academic_year_id: m.academic_year_id,
        term_id: m.term_id,
        is_system: m.is_system,
        metadata: m.metadata,
        created_by: m.created_by,
        created_at: m.created_at.into(),
        updated_at: m.updated_at.into(),
        deleted_at: m.deleted_at.map(|d| d.into()),
    }
}

#[async_trait]
impl FolderRepository for FolderRepoImpl {
    async fn find_by_id(
        &self,
        org_id: Uuid,
        id: Uuid,
    ) -> Result<Option<DomainFolder>, DomainError> {
        let model = FolderEntity::find()
            .filter(folder::Column::OrgId.eq(org_id))
            .filter(folder::Column::Id.eq(id))
            .filter(folder::Column::DeletedAt.is_null())
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(model.map(map_folder_model_to_domain))
    }

    async fn find_by_path(
        &self,
        org_id: Uuid,
        path: &str,
    ) -> Result<Option<DomainFolder>, DomainError> {
        let model = FolderEntity::find()
            .filter(folder::Column::OrgId.eq(org_id))
            .filter(folder::Column::Path.eq(path))
            .filter(folder::Column::DeletedAt.is_null())
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(model.map(map_folder_model_to_domain))
    }

    async fn find_by_entity(
        &self,
        org_id: Uuid,
        entity_type: &str,
        entity_id: Uuid,
    ) -> Result<Option<DomainFolder>, DomainError> {
        let model = FolderEntity::find()
            .filter(folder::Column::OrgId.eq(org_id))
            .filter(folder::Column::EntityType.eq(entity_type))
            .filter(folder::Column::EntityId.eq(entity_id))
            .filter(folder::Column::DeletedAt.is_null())
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(model.map(map_folder_model_to_domain))
    }

    async fn list_children(
        &self,
        org_id: Uuid,
        parent_id: Option<Uuid>,
    ) -> Result<Vec<DomainFolder>, DomainError> {
        let mut query = FolderEntity::find()
            .filter(folder::Column::OrgId.eq(org_id))
            .filter(folder::Column::DeletedAt.is_null());

        if let Some(pid) = parent_id {
            query = query.filter(folder::Column::ParentId.eq(Some(pid)));
        } else {
            query = query.filter(folder::Column::ParentId.is_null());
        }

        let models = query
            .order_by_asc(folder::Column::Name)
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(models.into_iter().map(map_folder_model_to_domain).collect())
    }

    async fn list_by_type(
        &self,
        org_id: Uuid,
        folder_type: &str,
    ) -> Result<Vec<DomainFolder>, DomainError> {
        let models = FolderEntity::find()
            .filter(folder::Column::OrgId.eq(org_id))
            .filter(folder::Column::FolderType.eq(folder_type))
            .filter(folder::Column::DeletedAt.is_null())
            .order_by_asc(folder::Column::Name)
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(models.into_iter().map(map_folder_model_to_domain).collect())
    }

    async fn list_all(&self, org_id: Uuid) -> Result<Vec<DomainFolder>, DomainError> {
        let models = FolderEntity::find()
            .filter(folder::Column::OrgId.eq(org_id))
            .filter(folder::Column::DeletedAt.is_null())
            .order_by_asc(folder::Column::Path)
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(models.into_iter().map(map_folder_model_to_domain).collect())
    }

    async fn save(&self, folder: &DomainFolder) -> Result<(), DomainError> {
        let active = folder::ActiveModel {
            id: Set(folder.id),
            org_id: Set(folder.org_id),
            parent_id: Set(folder.parent_id),
            name: Set(folder.name.clone()),
            path: Set(folder.path.clone()),
            folder_type: Set(folder.folder_type.clone()),
            entity_type: Set(folder.entity_type.clone()),
            entity_id: Set(folder.entity_id),
            org_unit_id: Set(folder.org_unit_id),
            academic_year_id: Set(folder.academic_year_id),
            term_id: Set(folder.term_id),
            is_system: Set(folder.is_system),
            metadata: Set(folder.metadata.clone()),
            created_by: Set(folder.created_by),
            created_at: Set(folder.created_at.into()),
            updated_at: Set(folder.updated_at.into()),
            deleted_at: Set(folder.deleted_at.map(|d| d.into())),
        };

        let exists = FolderEntity::find_by_id(folder.id)
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

    async fn delete(&self, org_id: Uuid, id: Uuid) -> Result<(), DomainError> {
        let folder = FolderEntity::find()
            .filter(folder::Column::OrgId.eq(org_id))
            .filter(folder::Column::Id.eq(id))
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?
            .ok_or_else(|| DomainError::NotFound(format!("Folder {id} not found")))?;

        let mut active: folder::ActiveModel = folder.into();
        active.deleted_at = Set(Some(Utc::now().into()));
        active.updated_at = Set(Utc::now().into());
        active
            .update(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(())
    }
}
