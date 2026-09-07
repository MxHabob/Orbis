use async_trait::async_trait;
use orbis_app::ports::repositories::OrganizationRepository;
use orbis_domain::entities::organization::Organization as DomainOrganization;
use orbis_domain::errors::DomainError;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set, ColumnTrait, QueryFilter};
use uuid::Uuid;

use crate::models::organization::{self, Entity as OrganizationEntity};

pub struct OrgRepoImpl {
    pub db: DatabaseConnection,
}

#[async_trait]
impl OrganizationRepository for OrgRepoImpl {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<DomainOrganization>, DomainError> {
        let model = OrganizationEntity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        Ok(model.map(|m| DomainOrganization {
            id: m.id,
            name: m.name,
            name_ar: m.name_ar,
            slug: m.slug,
            code: m.code,
            timezone: m.timezone,
            locale: m.locale,
            date_format: m.date_format,
            hijri_enabled: m.hijri_enabled,
            max_users: m.max_users,
            max_students: m.max_students,
            max_storage_gb: m.max_storage_gb,
            base_currency_id: m.base_currency_id,
            settings: m.settings,
            features: m.features,
            logo: m.logo,
            favicon: m.favicon,
            primary_color: m.primary_color,
            is_active: m.is_active,
            is_verified: m.is_verified,
            is_deleted: m.is_deleted,
            deleted_at: m.deleted_at,
            deleted_by: m.deleted_by,
            current_period_id: m.current_period_id,
            metadata: m.metadata,
            created_at: m.created_at,
            updated_at: m.updated_at,
        }))
    }

    async fn find_by_slug(&self, slug: &str) -> Result<Option<DomainOrganization>, DomainError> {
        let model = OrganizationEntity::find()
            .filter(organization::Column::Slug.eq(slug))
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        Ok(model.map(|m| DomainOrganization {
            id: m.id,
            name: m.name,
            name_ar: m.name_ar,
            slug: m.slug,
            code: m.code,
            timezone: m.timezone,
            locale: m.locale,
            date_format: m.date_format,
            hijri_enabled: m.hijri_enabled,
            max_users: m.max_users,
            max_students: m.max_students,
            max_storage_gb: m.max_storage_gb,
            base_currency_id: m.base_currency_id,
            settings: m.settings,
            features: m.features,
            logo: m.logo,
            favicon: m.favicon,
            primary_color: m.primary_color,
            is_active: m.is_active,
            is_verified: m.is_verified,
            is_deleted: m.is_deleted,
            deleted_at: m.deleted_at,
            deleted_by: m.deleted_by,
            current_period_id: m.current_period_id,
            metadata: m.metadata,
            created_at: m.created_at,
            updated_at: m.updated_at,
        }))
    }

    async fn find_first(&self) -> Result<Option<DomainOrganization>, DomainError> {
        let model = OrganizationEntity::find()
            .filter(organization::Column::IsDeleted.eq(false))
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        Ok(model.map(|m| DomainOrganization {
            id: m.id,
            name: m.name,
            name_ar: m.name_ar,
            slug: m.slug,
            code: m.code,
            timezone: m.timezone,
            locale: m.locale,
            date_format: m.date_format,
            hijri_enabled: m.hijri_enabled,
            max_users: m.max_users,
            max_students: m.max_students,
            max_storage_gb: m.max_storage_gb,
            base_currency_id: m.base_currency_id,
            settings: m.settings,
            features: m.features,
            logo: m.logo,
            favicon: m.favicon,
            primary_color: m.primary_color,
            is_active: m.is_active,
            is_verified: m.is_verified,
            is_deleted: m.is_deleted,
            deleted_at: m.deleted_at,
            deleted_by: m.deleted_by,
            current_period_id: m.current_period_id,
            metadata: m.metadata,
            created_at: m.created_at,
            updated_at: m.updated_at,
        }))
    }

    async fn save(&self, org: &DomainOrganization) -> Result<(), DomainError> {
        let active_model = organization::ActiveModel {
            id: Set(org.id),
            name: Set(org.name.clone()),
            name_ar: Set(org.name_ar.clone()),
            slug: Set(org.slug.clone()),
            code: Set(org.code.clone()),
            timezone: Set(org.timezone.clone()),
            locale: Set(org.locale.clone()),
            date_format: Set(org.date_format.clone()),
            hijri_enabled: Set(org.hijri_enabled),
            max_users: Set(org.max_users),
            max_students: Set(org.max_students),
            max_storage_gb: Set(org.max_storage_gb),
            base_currency_id: Set(org.base_currency_id.clone()),
            settings: Set(org.settings.clone()),
            features: Set(org.features.clone()),
            logo: Set(org.logo.clone()),
            favicon: Set(org.favicon.clone()),
            primary_color: Set(org.primary_color.clone()),
            is_active: Set(org.is_active),
            is_verified: Set(org.is_verified),
            is_deleted: Set(org.is_deleted),
            deleted_at: Set(org.deleted_at),
            deleted_by: Set(org.deleted_by),
            current_period_id: Set(org.current_period_id.clone()),
            metadata: Set(org.metadata.clone()),
            created_at: Set(org.created_at),
            updated_at: Set(org.updated_at),
            ..Default::default()
        };

        if OrganizationEntity::find_by_id(org.id).one(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?.is_some() {
            active_model.update(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        } else {
            active_model.insert(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        }

        Ok(())
    }
}
