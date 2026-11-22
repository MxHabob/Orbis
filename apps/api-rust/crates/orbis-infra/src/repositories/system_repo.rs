//! System Repository — SeaORM implementation for system-level queries.
//!
//! Provides health check and readiness data by querying core tables.

use async_trait::async_trait;
use sea_orm::{
    ColumnTrait, Condition, ConnectionTrait, DatabaseBackend, DatabaseConnection,
    DatabaseTransaction, EntityTrait, PaginatorTrait, QueryFilter, Statement, TransactionTrait,
};
use std::fs::{File, OpenOptions};
use uuid::Uuid;

use orbis_app::ports::repositories::{BootstrapLockGuard, SystemRepository};
use orbis_domain::errors::DomainError;

use crate::models::{academic_calendar, class, organization, subject, user_role_assignment};

pub struct SystemRepoImpl {
    pub db: DatabaseConnection,
}

struct PostgresBootstrapLockGuard {
    _transaction: DatabaseTransaction,
}

impl BootstrapLockGuard for PostgresBootstrapLockGuard {}

struct SqliteBootstrapLockGuard {
    _file: File,
}

impl BootstrapLockGuard for SqliteBootstrapLockGuard {}

struct InMemoryBootstrapLockGuard;

impl BootstrapLockGuard for InMemoryBootstrapLockGuard {}

#[async_trait]
impl SystemRepository for SystemRepoImpl {
    async fn acquire_bootstrap_lock(&self) -> Result<Box<dyn BootstrapLockGuard>, DomainError> {
        match self.db.get_database_backend() {
            DatabaseBackend::Postgres => {
                let transaction = self
                    .db
                    .begin()
                    .await
                    .map_err(|error| DomainError::Internal(error.to_string()))?;
                transaction
                    .query_one(Statement::from_sql_and_values(
                        DatabaseBackend::Postgres,
                        "SELECT pg_advisory_xact_lock($1)",
                        [0x4F52424953424F4Fi64.into()],
                    ))
                    .await
                    .map_err(|error| DomainError::Internal(error.to_string()))?;
                Ok(Box::new(PostgresBootstrapLockGuard {
                    _transaction: transaction,
                }))
            }
            DatabaseBackend::Sqlite => {
                let rows = self
                    .db
                    .query_all(Statement::from_string(
                        DatabaseBackend::Sqlite,
                        "PRAGMA database_list".to_string(),
                    ))
                    .await
                    .map_err(|error| DomainError::Internal(error.to_string()))?;
                let database_path = rows
                    .first()
                    .and_then(|row| row.try_get::<String>("", "file").ok())
                    .filter(|path| !path.is_empty());

                if let Some(database_path) = database_path {
                    let lock_path = format!("{database_path}.bootstrap.lock");
                    let file = tokio::task::spawn_blocking(move || {
                        let file = OpenOptions::new()
                            .create(true)
                            .read(true)
                            .write(true)
                            .open(lock_path)
                            .map_err(|error| DomainError::Internal(error.to_string()))?;
                        fs2::FileExt::lock_exclusive(&file)
                            .map_err(|error| DomainError::Internal(error.to_string()))?;
                        Ok::<_, DomainError>(file)
                    })
                    .await
                    .map_err(|error| DomainError::Internal(error.to_string()))??;
                    Ok(Box::new(SqliteBootstrapLockGuard { _file: file }))
                } else {
                    Ok(Box::new(InMemoryBootstrapLockGuard))
                }
            }

            _ => Err(DomainError::Internal(
                "Bootstrap locking is not supported by this database backend".into(),
            )),
        }
    }

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
                    .add(user_role_assignment::Column::RoleId.eq("admin")),
            )
            .count(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(count > 0)
    }

    async fn has_academic_calendar(&self, org_id: Uuid) -> Result<bool, DomainError> {
        let count = academic_calendar::Entity::find()
            .filter(academic_calendar::Column::OrgId.eq(org_id))
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

#[cfg(test)]
mod tests {
    use super::*;
    use sea_orm::Database;
    use std::time::Duration;

    #[tokio::test]
    async fn sqlite_bootstrap_lock_serializes_processes_using_the_same_database() {
        let database_file =
            std::env::temp_dir().join(format!("orbis-bootstrap-{}.db", Uuid::new_v4()));
        let database_url = format!("sqlite://{}?mode=rwc", database_file.display());
        let first_db = Database::connect(&database_url).await.unwrap();
        let second_db = Database::connect(&database_url).await.unwrap();
        let lock_file =
            std::path::PathBuf::from(format!("{}.bootstrap.lock", database_file.display()));

        {
            let first_repo = SystemRepoImpl {
                db: first_db.clone(),
            };
            let first_guard = first_repo.acquire_bootstrap_lock().await.unwrap();
            let second_repo = SystemRepoImpl {
                db: second_db.clone(),
            };
            let second_task =
                tokio::spawn(async move { second_repo.acquire_bootstrap_lock().await });

            tokio::time::sleep(Duration::from_millis(100)).await;
            assert!(!second_task.is_finished());

            drop(first_guard);
            let second_guard = tokio::time::timeout(Duration::from_secs(2), second_task)
                .await
                .unwrap()
                .unwrap()
                .unwrap();
            drop(second_guard);
        }

        first_db.close().await.unwrap();
        second_db.close().await.unwrap();
        std::fs::remove_file(&database_file).unwrap();
        std::fs::remove_file(lock_file).unwrap();
    }
}
