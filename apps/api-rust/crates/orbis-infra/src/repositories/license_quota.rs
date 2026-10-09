use orbis_domain::errors::DomainError;
use sea_orm::{ConnectionTrait, DatabaseBackend, DatabaseTransaction, Statement};
use uuid::Uuid;

pub async fn acquire_organization_quota_lock(
    transaction: &DatabaseTransaction,
    organization_id: Uuid,
) -> Result<(), DomainError> {
    let backend = transaction.get_database_backend();
    let insert_sql = match backend {
        DatabaseBackend::Postgres => {
            "INSERT INTO license_quota_locks (org_id) VALUES ($1) ON CONFLICT (org_id) DO NOTHING"
        }
        DatabaseBackend::Sqlite => "INSERT OR IGNORE INTO license_quota_locks (org_id) VALUES (?)",
        _ => {
            return Err(DomainError::Internal(
                "License quota locking is not supported by this database backend".to_string(),
            ));
        }
    };

    transaction
        .execute(Statement::from_sql_and_values(
            backend,
            insert_sql,
            [organization_id.into()],
        ))
        .await
        .map_err(|error| DomainError::Internal(error.to_string()))?;

    if backend == DatabaseBackend::Postgres {
        transaction
            .query_one(Statement::from_sql_and_values(
                backend,
                "SELECT org_id FROM license_quota_locks WHERE org_id = $1 FOR UPDATE",
                [organization_id.into()],
            ))
            .await
            .map_err(|error| DomainError::Internal(error.to_string()))?;
    }

    Ok(())
}
