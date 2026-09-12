use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set, ColumnTrait, QueryFilter};
use uuid::Uuid;

use orbis_app::ports::repositories::UserRepository;
use orbis_domain::entities::user::User;
use orbis_domain::errors::DomainError;
use crate::models::user::{self, Entity as UserEntity};

pub struct UserRepoImpl {
    pub db: DatabaseConnection,
}

#[async_trait]
impl UserRepository for UserRepoImpl {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, DomainError> {
        let model = UserEntity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        Ok(model.map(|m| User {
            id: m.id,
            person_id: m.person_id,
            org_id: m.org_id,
            email: m.email,
            is_email_verified: m.is_email_verified,
            phone: m.phone,
            is_phone_verified: m.is_phone_verified,
            password_hash: m.password_hash,
            is_mfa_enabled: m.is_mfa_enabled,
            status: m.status,
            login_count: m.login_count,
            failed_login_attempts: m.failed_login_attempts,
            locked_until: m.locked_until,
            is_deleted: m.is_deleted,
            created_at: m.created_at,
            updated_at: m.updated_at,
        }))
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, DomainError> {
        let model = UserEntity::find()
            .filter(user::Column::Email.eq(email))
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        Ok(model.map(|m| User {
            id: m.id,
            person_id: m.person_id,
            org_id: m.org_id,
            email: m.email,
            is_email_verified: m.is_email_verified,
            phone: m.phone,
            is_phone_verified: m.is_phone_verified,
            password_hash: m.password_hash,
            is_mfa_enabled: m.is_mfa_enabled,
            status: m.status,
            login_count: m.login_count,
            failed_login_attempts: m.failed_login_attempts,
            locked_until: m.locked_until,
            is_deleted: m.is_deleted,
            created_at: m.created_at,
            updated_at: m.updated_at,
        }))
    }

    async fn find_by_org(&self, org_id: Uuid) -> Result<Vec<User>, DomainError> {
        let models = UserEntity::find()
            .filter(user::Column::OrgId.eq(org_id))
            .filter(user::Column::IsDeleted.eq(false))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        Ok(models.into_iter().map(|m| User {
            id: m.id,
            person_id: m.person_id,
            org_id: m.org_id,
            email: m.email,
            is_email_verified: m.is_email_verified,
            phone: m.phone,
            is_phone_verified: m.is_phone_verified,
            password_hash: m.password_hash,
            is_mfa_enabled: m.is_mfa_enabled,
            status: m.status,
            login_count: m.login_count,
            failed_login_attempts: m.failed_login_attempts,
            locked_until: m.locked_until,
            is_deleted: m.is_deleted,
            created_at: m.created_at,
            updated_at: m.updated_at,
        }).collect())
    }

    async fn save(&self, user: &User) -> Result<(), DomainError> {
        let active_model = user::ActiveModel {
            id: Set(user.id),
            person_id: Set(user.person_id),
            org_id: Set(user.org_id),
            email: Set(user.email.clone()),
            is_email_verified: Set(user.is_email_verified),
            phone: Set(user.phone.clone()),
            is_phone_verified: Set(user.is_phone_verified),
            password_hash: Set(user.password_hash.clone()),
            is_mfa_enabled: Set(user.is_mfa_enabled),
            status: Set(user.status.clone()),
            login_count: Set(user.login_count),
            failed_login_attempts: Set(user.failed_login_attempts),
            locked_until: Set(user.locked_until),
            is_deleted: Set(user.is_deleted),
            created_at: Set(user.created_at),
            updated_at: Set(user.updated_at),
        };

        if UserEntity::find_by_id(user.id).one(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?.is_some() {
            active_model.update(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        } else {
            active_model.insert(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        }

        Ok(())
    }
}
