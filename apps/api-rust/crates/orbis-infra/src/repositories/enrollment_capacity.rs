use orbis_domain::errors::DomainError;
use sea_orm::{ConnectionTrait, DatabaseBackend, DatabaseTransaction, Statement};
use uuid::Uuid;

#[derive(Clone, Copy)]
pub enum CapacityLockTarget {
    ClassSection,
    CourseOffering,
}

pub async fn acquire_capacity_lock(
    transaction: &DatabaseTransaction,
    organization_id: Uuid,
    capacity_key: Uuid,
    target: CapacityLockTarget,
) -> Result<(), DomainError> {
    let backend = transaction.get_database_backend();
    let table = match target {
        CapacityLockTarget::ClassSection => "sections",
        CapacityLockTarget::CourseOffering => "section_subjects",
    };

    match backend {
        DatabaseBackend::Postgres => {
            let row = transaction
                .query_one(Statement::from_sql_and_values(
                    backend,
                    &format!("SELECT id FROM {table} WHERE id = $1 AND org_id = $2 FOR UPDATE"),
                    [capacity_key.into(), organization_id.into()],
                ))
                .await
                .map_err(|error| DomainError::Internal(error.to_string()))?;
            if row.is_none() {
                return Err(DomainError::NotFound(
                    "Enrollment capacity target not found".into(),
                ));
            }
        }
        DatabaseBackend::Sqlite => {
            let result = transaction
                .execute(Statement::from_sql_and_values(
                    backend,
                    &format!("UPDATE {table} SET id = id WHERE id = ? AND org_id = ?"),
                    [capacity_key.into(), organization_id.into()],
                ))
                .await
                .map_err(|error| DomainError::Internal(error.to_string()))?;
            if result.rows_affected() == 0 {
                return Err(DomainError::NotFound(
                    "Enrollment capacity target not found".into(),
                ));
            }
        }
        _ => {
            return Err(DomainError::Internal(
                "Enrollment capacity locking is not supported by this database backend".into(),
            ));
        }
    }

    Ok(())
}
