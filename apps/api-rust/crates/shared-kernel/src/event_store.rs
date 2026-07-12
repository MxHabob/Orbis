//! Event Store — append-only event log for critical aggregates.
//!
//! Provides Event Sourcing for aggregates where full history matters:
//!   • Enrollment (registration, transfer, withdrawal, promotion)
//!   • Grades (submission, modification, publication)
//!   • Payments (immutable financial transactions)
//!   • Attendance (patterns over time for AI analysis)
//!
//! The event store is *complementary* to the existing SeaORM models.
//! Aggregates keep their current state in regular tables (for fast queries)
//! AND append events here (for full audit trail + AI analysis).
//!
//! Schema: `domain_events` table (see migration).

use sea_orm::{DatabaseConnection, ConnectionTrait, Statement, DbBackend};
use serde::Serialize;

use crate::errors::AppError;
use crate::traits::Actor;

/// A stored domain event — immutable record of something that happened.
#[derive(Debug, Clone, Serialize)]
pub struct StoredEvent {
    pub id: i64,
    pub aggregate_type: String,
    pub aggregate_id: String,
    pub event_type: String,
    pub payload: serde_json::Value,
    pub actor_id: String,
    pub actor_type: String,
    pub correlation_id: Option<String>,
    pub version: i64,
    pub created_at: String,
}

/// Append-only event store backed by the `domain_events` table.
pub struct EventStore {
    db: DatabaseConnection,
}

impl EventStore {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// Get a reference to the database connection.
    pub fn db(&self) -> &DatabaseConnection {
        &self.db
    }

    /// Append a new event for an aggregate.
    ///
    /// The `version` is auto-incremented per aggregate (aggregate_type + aggregate_id).
    pub async fn append(
        &self,
        aggregate_type: &str,
        aggregate_id: &str,
        event_type: &str,
        payload: &serde_json::Value,
        actor: &Actor,
        correlation_id: Option<&str>,
    ) -> Result<i64, AppError> {
        // Get next version for this aggregate
        let next_version = self.next_version(aggregate_type, aggregate_id).await?;
        let now = chrono::Utc::now().to_rfc3339();
        let payload_str = serde_json::to_string(payload)
            .map_err(|e| AppError::internal(format!("Event payload serialization failed: {}", e)))?;

        let sql = r#"
            INSERT INTO domain_events
                (aggregate_type, aggregate_id, event_type, payload, actor_id, actor_type, correlation_id, version, created_at)
            VALUES
                ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        "#;

        self.db.execute(Statement::from_sql_and_values(
            DbBackend::Sqlite,
            sql,
            vec![
                aggregate_type.into(),
                aggregate_id.into(),
                event_type.into(),
                payload_str.into(),
                actor.as_str().to_string().into(),
                actor.actor_type().to_string().into(),
                correlation_id.map(|s| s.to_string()).unwrap_or_default().into(),
                next_version.into(),
                now.into(),
            ],
        )).await?;

        Ok(next_version)
    }

    /// Get all events for an aggregate, ordered by version.
    pub async fn get_history(
        &self,
        aggregate_type: &str,
        aggregate_id: &str,
    ) -> Result<Vec<StoredEvent>, AppError> {
        let sql = r#"
            SELECT id, aggregate_type, aggregate_id, event_type, payload,
                   actor_id, actor_type, correlation_id, version, created_at
            FROM domain_events
            WHERE aggregate_type = $1 AND aggregate_id = $2
            ORDER BY version ASC
        "#;

        let rows = self.db.query_all(Statement::from_sql_and_values(
            DbBackend::Sqlite,
            sql,
            vec![aggregate_type.into(), aggregate_id.into()],
        )).await?;

        rows.iter().map(|row| {
        Ok(StoredEvent {
                id: row.try_get_by_index::<i64>(0)?,
                aggregate_type: row.try_get_by_index::<String>(1)?,
                aggregate_id: row.try_get_by_index::<String>(2)?,
                event_type: row.try_get_by_index::<String>(3)?,
                payload: {
                    let s: String = row.try_get_by_index(4)?;
                    serde_json::from_str(&s).unwrap_or_default()
                },
                actor_id: row.try_get_by_index::<String>(5)?,
                actor_type: row.try_get_by_index::<String>(6)?,
                correlation_id: row.try_get_by_index::<Option<String>>(7)?,
                version: row.try_get_by_index::<i64>(8)?,
                created_at: row.try_get_by_index::<String>(9)?,
            })
        }).collect()
    }

    /// Get events by type since a given timestamp (for AI analysis).
    pub async fn get_events_by_type(
        &self,
        event_type: &str,
        since: &str,
        limit: u64,
    ) -> Result<Vec<StoredEvent>, AppError> {
        let sql = r#"
            SELECT id, aggregate_type, aggregate_id, event_type, payload,
                   actor_id, actor_type, correlation_id, version, created_at
            FROM domain_events
            WHERE event_type = $1 AND created_at >= $2
            ORDER BY created_at ASC
            LIMIT $3
        "#;

        let rows = self.db.query_all(Statement::from_sql_and_values(
            DbBackend::Sqlite,
            sql,
            vec![event_type.into(), since.into(), (limit as i64).into()],
        )).await?;

        rows.iter().map(|row| {
        Ok(StoredEvent {
                id: row.try_get_by_index::<i64>(0)?,
                aggregate_type: row.try_get_by_index::<String>(1)?,
                aggregate_id: row.try_get_by_index::<String>(2)?,
                event_type: row.try_get_by_index::<String>(3)?,
                payload: {
                    let s: String = row.try_get_by_index(4)?;
                    serde_json::from_str(&s).unwrap_or_default()
                },
                actor_id: row.try_get_by_index::<String>(5)?,
                actor_type: row.try_get_by_index::<String>(6)?,
                correlation_id: row.try_get_by_index::<Option<String>>(7)?,
                version: row.try_get_by_index::<i64>(8)?,
                created_at: row.try_get_by_index::<String>(9)?,
            })
        }).collect()
    }

    /// Get the next version number for an aggregate.
    async fn next_version(&self, aggregate_type: &str, aggregate_id: &str) -> Result<i64, AppError> {
        let sql = r#"
            SELECT COALESCE(MAX(version), 0) + 1 as next_version
            FROM domain_events
            WHERE aggregate_type = $1 AND aggregate_id = $2
        "#;

        let result = self.db.query_one(Statement::from_sql_and_values(
            DbBackend::Sqlite,
            sql,
            vec![aggregate_type.into(), aggregate_id.into()],
        )).await?;

        match result {
            Some(row) => {
                Ok(row.try_get_by_index::<i64>(0)?)
            }
            None => Ok(1),
        }
    }

    /// Persist a new AI insight into the `ai_insights` table.
    pub async fn persist_insight(
        &self,
        org_id: &str,
        insight_type: &str,
        entity_type: Option<&str>,
        entity_id: Option<&str>,
        title: &str,
        title_ar: Option<&str>,
        summary: &str,
        summary_ar: Option<&str>,
        severity: &str,
        confidence: f64,
        suggested_action: Option<&str>,
        generated_by: &str,
    ) -> Result<String, AppError> {
        let id = format!("insight_{}", uuid::Uuid::new_v4().to_string().replace("-", "")).chars().take(24).collect::<String>();
        let now = chrono::Utc::now().to_rfc3339();

        let sql = r#"
            INSERT INTO ai_insights
                (id, org_id, insight_type, entity_type, entity_id, title, title_ar, summary, summary_ar, severity, confidence, suggested_action, generated_by, valid_from, created_at, updated_at)
            VALUES
                ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
        "#;

        self.db.execute(Statement::from_sql_and_values(
            DbBackend::Sqlite,
            sql,
            vec![
                id.clone().into(),
                org_id.to_string().into(),
                insight_type.to_string().into(),
                entity_type.map(|s| s.to_string()).into(),
                entity_id.map(|s| s.to_string()).into(),
                title.to_string().into(),
                title_ar.map(|s| s.to_string()).into(),
                summary.to_string().into(),
                summary_ar.map(|s| s.to_string()).into(),
                severity.to_string().into(),
                confidence.into(),
                suggested_action.map(|s| s.to_string()).into(),
                generated_by.to_string().into(),
                now.clone().into(),
                now.clone().into(),
                now.into(),
            ],
        )).await?;

        Ok(id)
    }
}
