//! Concrete `AuditLogger` implementation — writes to `audit_log` table.

use async_trait::async_trait;
use sea_orm::{DatabaseConnection, ConnectionTrait, Statement, DbBackend};

use shared_kernel::errors::AppError;
use shared_kernel::traits::{AuditEntry, AuditLogger};

/// Concrete `AuditLogger` that writes entries to the `audit_log` table.
pub struct DbAuditLogger {
    db: DatabaseConnection,
}

impl DbAuditLogger {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl AuditLogger for DbAuditLogger {
    async fn log(&self, entry: AuditEntry) -> Result<(), AppError> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().naive_utc().to_string();
        let old_data = entry.old_data.map(|v| v.to_string());
        let new_data = entry.new_data.map(|v| v.to_string());

        let sql = r#"
            INSERT INTO audit_log
                (id, org_id, user_id, actor_type, action, entity_type, entity_id,
                 old_data, new_data, correlation_id, created_at)
            VALUES
                ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        "#;

        self.db.execute(Statement::from_sql_and_values(
            DbBackend::Sqlite,
            sql,
            vec![
                id.into(),
                entry.org_id.into(),
                entry.actor.as_str().to_string().into(),
                entry.actor.actor_type().to_string().into(),
                entry.action.into(),
                entry.entity_type.into(),
                entry.entity_id.into(),
                old_data.unwrap_or_default().into(),
                new_data.unwrap_or_default().into(),
                entry.correlation_id.unwrap_or_default().into(),
                now.into(),
            ],
        )).await?;

        Ok(())
    }
}
