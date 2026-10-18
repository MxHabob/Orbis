use async_trait::async_trait;
use orbis_app::ports::repositories::OrganizationRepository;
use orbis_domain::entities::organization::Organization as DomainOrganization;
use orbis_domain::errors::DomainError;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

use crate::models::organization::{self, Entity as OrganizationEntity};

pub struct OrgRepoImpl {
    pub db: DatabaseConnection,
}

fn map_org_model_to_domain(m: organization::Model) -> DomainOrganization {
    DomainOrganization {
        id: m.id,
        name: m.name,
        name_ar: m.name_ar,
        slug: m.slug,
        code: m.code,
        timezone: m.timezone,
        locale: m.locale,
        date_format: m.date_format,
        format_locale: m.format_locale,
        time_format: m.time_format,
        is_hijri_enabled: m.is_hijri_enabled,
        base_currency_code: m.base_currency_code,
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
        current_period_id: m.current_period_id.map(|u| u.to_string()),
        institution_type: m.institution_type,
        metadata: m.metadata,
        created_by: m.created_by,
        updated_by: m.updated_by,
        version: m.version,
        created_at: m.created_at,
        updated_at: m.updated_at,
    }
}

#[async_trait]
impl OrganizationRepository for OrgRepoImpl {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<DomainOrganization>, DomainError> {
        let model = OrganizationEntity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        Ok(model.map(map_org_model_to_domain))
    }

    async fn find_by_slug(&self, slug: &str) -> Result<Option<DomainOrganization>, DomainError> {
        let model = OrganizationEntity::find()
            .filter(organization::Column::Slug.eq(slug))
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        Ok(model.map(map_org_model_to_domain))
    }

    async fn find_first(&self) -> Result<Option<DomainOrganization>, DomainError> {
        let model = OrganizationEntity::find()
            .filter(organization::Column::IsDeleted.eq(false))
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        Ok(model.map(map_org_model_to_domain))
    }

    async fn list_all(&self) -> Result<Vec<DomainOrganization>, DomainError> {
        let models = OrganizationEntity::find()
            .filter(organization::Column::IsDeleted.eq(false))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        Ok(models.into_iter().map(map_org_model_to_domain).collect())
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
            format_locale: Set(org.format_locale.clone()),
            time_format: Set(org.time_format.clone()),
            is_hijri_enabled: Set(org.is_hijri_enabled),
            base_currency_code: Set(org.base_currency_code.clone()),
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
            current_period_id: Set(org
                .current_period_id
                .as_deref()
                .and_then(|s| Uuid::parse_str(s).ok())),
            institution_type: Set(org.institution_type.clone()),
            metadata: Set(org.metadata.clone()),
            created_by: Set(org.created_by),
            updated_by: Set(org.updated_by),
            version: Set(org.version),
            created_at: Set(org.created_at),
            updated_at: Set(org.updated_at),
            ..Default::default()
        };

        if OrganizationEntity::find_by_id(org.id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?
            .is_some()
        {
            active_model
                .update(&self.db)
                .await
                .map_err(|e| DomainError::Internal(e.to_string()))?;
        } else {
            active_model
                .insert(&self.db)
                .await
                .map_err(|e| DomainError::Internal(e.to_string()))?;
        }

        Ok(())
    }
}
