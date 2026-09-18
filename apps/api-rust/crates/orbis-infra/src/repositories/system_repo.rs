//! System Repository — SeaORM implementation for system-level queries.
//!
//! Provides health check and readiness data by querying core tables.

use async_trait::async_trait;
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, PaginatorTrait, Condition};
use uuid::Uuid;

use orbis_app::ports::repositories::SystemRepository;
use orbis_domain::errors::DomainError;

use crate::models::{organization, academic_year, class, subject, user_role_assignment};

pub struct SystemRepoImpl {
    pub db: DatabaseConnection,
}

#[async_trait]
impl SystemRepository for SystemRepoImpl {
    async fn has_organization(&self) -> Result<bool, DomainError> {
        let count = organization::Entity::find()
            .count(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(count > 0)
    }

    async fn has_admin(&self) -> Result<bool, DomainError> {
        let count = user_role_assignment::Entity::find()
            .filter(
                Condition::any()
                    .add(user_role_assignment::Column::RoleId.eq("super_admin"))
                    .add(user_role_assignment::Column::RoleId.eq("admin"))
            )
            .count(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(count > 0)
    }

    async fn has_academic_year(&self, org_id: Uuid) -> Result<bool, DomainError> {
        let count = academic_year::Entity::find()
            .filter(academic_year::Column::OrgId.eq(org_id))
            .count(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(count > 0)
    }

    async fn has_classes(&self, org_id: Uuid) -> Result<bool, DomainError> {
        let count = class::Entity::find()
            .filter(class::Column::OrgId.eq(org_id))
            .count(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(count > 0)
    }

    async fn has_subjects(&self, org_id: Uuid) -> Result<bool, DomainError> {
        let count = subject::Entity::find()
            .filter(subject::Column::OrgId.eq(org_id))
            .count(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(count > 0)
    }

    async fn get_org_name(&self) -> Result<Option<String>, DomainError> {
        let org = organization::Entity::find()
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(org.map(|o| o.name))
    }

    async fn get_first_org_id(&self) -> Result<Option<Uuid>, DomainError> {
        let org = organization::Entity::find()
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(org.map(|o| o.id))
    }
}
