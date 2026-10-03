use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set, ColumnTrait, QueryFilter};
use uuid::Uuid;
use orbis_app::ports::repositories::UserIdentityRepository;
use orbis_domain::entities::identity::{UserIdentity, IdentityProvider};
use orbis_domain::errors::DomainError;
use crate::models::user_identity::{self, Entity as IdentityEntity};

pub struct UserIdentityRepoImpl {
    pub db: DatabaseConnection,
}

#[async_trait]
impl UserIdentityRepository for UserIdentityRepoImpl {
    async fn find_by_provider(&self, provider: &str, provider_user_id: &str) -> Result<Option<UserIdentity>, DomainError> {
        let model = IdentityEntity::find()
            .filter(user_identity::Column::Provider.eq(provider))
            .filter(user_identity::Column::ProviderUserId.eq(provider_user_id))
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        Ok(model.map(|m| UserIdentity {
            id: m.id,
            user_id: m.user_id,
            provider: IdentityProvider::from(m.provider.as_str()),
            provider_user_id: m.provider_user_id,
            email: m.email,
            is_primary: m.is_primary,
            profile_data: m.profile_data,
            connected_at: m.connected_at,
            last_used_at: m.last_used_at,
        }))
    }

    async fn find_by_user(&self, user_id: Uuid) -> Result<Vec<UserIdentity>, DomainError> {
        let models = IdentityEntity::find()
            .filter(user_identity::Column::UserId.eq(user_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        Ok(models.into_iter().map(|m| UserIdentity {
            id: m.id,
            user_id: m.user_id,
            provider: IdentityProvider::from(m.provider.as_str()),
            provider_user_id: m.provider_user_id,
            email: m.email,
            is_primary: m.is_primary,
            profile_data: m.profile_data,
            connected_at: m.connected_at,
            last_used_at: m.last_used_at,
        }).collect())
    }

    async fn save(&self, identity: &UserIdentity) -> Result<(), DomainError> {
        let active_model = user_identity::ActiveModel {
            id: Set(identity.id),
            user_id: Set(identity.user_id),
            provider: Set(identity.provider.to_string()),
            provider_user_id: Set(identity.provider_user_id.clone()),
            email: Set(identity.email.clone()),
            is_primary: Set(identity.is_primary),
            profile_data: Set(identity.profile_data.clone()),
            connected_at: Set(identity.connected_at),
            last_used_at: Set(identity.last_used_at),
        };

        if IdentityEntity::find_by_id(identity.id).one(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?.is_some() {
            active_model.update(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        } else {
            active_model.insert(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        }

        Ok(())
    }

    async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        IdentityEntity::delete_by_id(id)
            .exec(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(())
    }
}
