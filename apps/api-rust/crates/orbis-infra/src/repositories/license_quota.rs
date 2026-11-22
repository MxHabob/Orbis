use orbis_domain::errors::DomainError;
use sea_orm::{ConnectionTrait, DatabaseBackend, DatabaseTransaction, Statement};
use uuid::Uuid;

pub async fn acquire_organization_quota_lock(
    transaction: &DatabaseTransaction,
    organization_id: Uuid,
) -> Result<(), DomainError> {
    let backend = transaction.get_database_backend();
    match backend {
        DatabaseBackend::Postgres => {
            let organization = transaction
                .query_one(Statement::from_sql_and_values(
                    backend,
                    "SELECT id FROM organizations WHERE id = $1 FOR UPDATE",
                    [organization_id.into()],
                ))
                .await
                .map_err(|error| DomainError::Internal(error.to_string()))?;
            if organization.is_none() {
                return Err(DomainError::NotFound("Organization not found".into()));
            }
        }
        DatabaseBackend::Sqlite => {
            let result = transaction
                .execute(Statement::from_sql_and_values(
                    backend,
                    "UPDATE organizations SET id = id WHERE id = ?",
                    [organization_id.into()],
                ))
                .await
                .map_err(|error| DomainError::Internal(error.to_string()))?;
            if result.rows_affected() == 0 {
                return Err(DomainError::NotFound("Organization not found".into()));
            }
        }
        _ => {
            return Err(DomainError::Internal(
                "License quota locking is not supported by this database backend".to_string(),
            ));
        }
    }

    Ok(())
}
