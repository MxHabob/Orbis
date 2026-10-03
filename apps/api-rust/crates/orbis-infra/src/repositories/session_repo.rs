use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set, ColumnTrait, QueryFilter};

use orbis_app::ports::repositories::SessionRepository;
use orbis_domain::entities::session::Session;
use orbis_domain::errors::DomainError;
use crate::models::user_session::{self, Entity as SessionEntity};

pub struct SessionRepoImpl {
    pub db: DatabaseConnection,
}

#[async_trait]
impl SessionRepository for SessionRepoImpl {
    async fn find_by_token(&self, token: &str) -> Result<Option<Session>, DomainError> {
        let model = SessionEntity::find()
            .filter(user_session::Column::Token.eq(token))
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        Ok(model.map(|m| Session {
            id: m.id,
            user_id: m.user_id,
            token: m.token,
            refresh_token: m.refresh_token,
            device_id: m.device_id,
            family_id: m.family_id,
            is_revoked: m.is_revoked,
            is_current: m.is_current,
            is_trusted: m.is_trusted,
            is_mfa_verified: m.is_mfa_verified,
            expires_at: m.expires_at,
            refresh_expires_at: m.refresh_expires_at,
            last_active_at: m.last_active_at,
            created_at: m.created_at,
        }))
    }

    async fn save(&self, session: &Session) -> Result<(), DomainError> {
        let active_model = user_session::ActiveModel {
            id: Set(session.id),
            user_id: Set(session.user_id),
            token: Set(session.token.clone()),
            refresh_token: Set(session.refresh_token.clone()),
            device_id: Set(session.device_id.clone()),
            family_id: Set(session.family_id.clone()),
            is_revoked: Set(session.is_revoked),
            is_current: Set(session.is_current),
            is_trusted: Set(session.is_trusted),
            is_mfa_verified: Set(session.is_mfa_verified),
            expires_at: Set(session.expires_at),
            refresh_expires_at: Set(session.refresh_expires_at),
            last_active_at: Set(session.last_active_at),
            created_at: Set(session.created_at),
        };

        if SessionEntity::find_by_id(session.id).one(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?.is_some() {
            active_model.update(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        } else {
            active_model.insert(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        }

        Ok(())
    }

    async fn invalidate(&self, token: &str) -> Result<(), DomainError> {
        SessionEntity::delete_many()
            .filter(user_session::Column::Token.eq(token))
            .exec(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(())
    }
}
