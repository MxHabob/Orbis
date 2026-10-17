use crate::models::mfa_factor::{self, Entity as MfaFactorEntity};
use crate::models::mfa_recovery_code::{self, Entity as RecoveryCodeEntity};
use async_trait::async_trait;
use orbis_app::ports::repositories::{MfaFactorRepository, MfaRecoveryCodeRepository};
use orbis_domain::entities::mfa::{MfaFactor, MfaRecoveryCode, MfaType};
use orbis_domain::errors::DomainError;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set,
    sea_query::Expr,
};
use uuid::Uuid;

pub struct MfaFactorRepoImpl {
    pub db: DatabaseConnection,
}

#[async_trait]
impl MfaFactorRepository for MfaFactorRepoImpl {
    async fn find_by_user(&self, user_id: Uuid) -> Result<Vec<MfaFactor>, DomainError> {
        let models = MfaFactorEntity::find()
            .filter(mfa_factor::Column::UserId.eq(user_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        Ok(models
            .into_iter()
            .map(|m| MfaFactor {
                id: m.id,
                user_id: m.user_id,
                factor_type: MfaType::from(m.factor_type.as_str()),
                factor_name: m.factor_name,
                secret_encrypted: m.secret_encrypted,
                is_verified: m.is_verified,
                is_default: m.is_default,
                created_at: m.created_at,
                last_used_at: m.last_used_at,
            })
            .collect())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<MfaFactor>, DomainError> {
        let model = MfaFactorEntity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        Ok(model.map(|m| MfaFactor {
            id: m.id,
            user_id: m.user_id,
            factor_type: MfaType::from(m.factor_type.as_str()),
            factor_name: m.factor_name,
            secret_encrypted: m.secret_encrypted,
            is_verified: m.is_verified,
            is_default: m.is_default,
            created_at: m.created_at,
            last_used_at: m.last_used_at,
        }))
    }

    async fn save(&self, factor: &MfaFactor) -> Result<(), DomainError> {
        let active_model = mfa_factor::ActiveModel {
            id: Set(factor.id),
            user_id: Set(factor.user_id),
            factor_type: Set(factor.factor_type.to_string()),
            factor_name: Set(factor.factor_name.clone()),
            secret_encrypted: Set(factor.secret_encrypted.clone()),
            is_verified: Set(factor.is_verified),
            is_default: Set(factor.is_default),
            created_at: Set(factor.created_at),
            last_used_at: Set(factor.last_used_at),
        };

        if MfaFactorEntity::find_by_id(factor.id)
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

    async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        MfaFactorEntity::delete_by_id(id)
            .exec(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(())
    }
}

pub struct MfaRecoveryCodeRepoImpl {
    pub db: DatabaseConnection,
}

#[async_trait]
impl MfaRecoveryCodeRepository for MfaRecoveryCodeRepoImpl {
    async fn find_active_by_user(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<MfaRecoveryCode>, DomainError> {
        let models = RecoveryCodeEntity::find()
            .filter(mfa_recovery_code::Column::UserId.eq(user_id))
            .filter(mfa_recovery_code::Column::IsUsed.eq(false))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        Ok(models
            .into_iter()
            .map(|m| MfaRecoveryCode {
                id: m.id,
                user_id: m.user_id,
                code_hash: m.code_hash,
                is_used: m.is_used,
                created_at: m.created_at,
                used_at: m.used_at,
            })
            .collect())
    }

    async fn save_all(&self, codes: &[MfaRecoveryCode]) -> Result<(), DomainError> {
        for code in codes {
            let active = mfa_recovery_code::ActiveModel {
                id: Set(code.id),
                user_id: Set(code.user_id),
                code_hash: Set(code.code_hash.clone()),
                is_used: Set(code.is_used),
                created_at: Set(code.created_at),
                used_at: Set(code.used_at),
            };
            active
                .insert(&self.db)
                .await
                .map_err(|e| DomainError::Internal(e.to_string()))?;
        }
        Ok(())
    }

    async fn mark_used(&self, id: Uuid) -> Result<bool, DomainError> {
        let result = RecoveryCodeEntity::update_many()
            .col_expr(mfa_recovery_code::Column::IsUsed, Expr::value(true))
            .col_expr(
                mfa_recovery_code::Column::UsedAt,
                Expr::value(Some(chrono::Utc::now())),
            )
            .filter(mfa_recovery_code::Column::Id.eq(id))
            .filter(mfa_recovery_code::Column::IsUsed.eq(false))
            .exec(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(result.rows_affected == 1)
    }
}
