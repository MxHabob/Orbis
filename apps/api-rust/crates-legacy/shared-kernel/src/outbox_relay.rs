use std::sync::Arc;
use std::time::Duration;
use sea_orm::{DatabaseConnection, DbBackend, Statement, ConnectionTrait};
use tracing::{error, trace, debug};
use tokio::time::sleep;
use uuid::Uuid;

use crate::event_bus::{EventBus, EventEnvelope};

pub struct OutboxRelay {
    pub db: DatabaseConnection,
    pub event_bus: Arc<EventBus>,
    pub poll_interval: Duration,
}

impl OutboxRelay {
    pub fn new(db: DatabaseConnection, event_bus: Arc<EventBus>) -> Self {
        Self {
            db,
            event_bus,
            poll_interval: Duration::from_secs(2), // Poll every 2 seconds
        }
    }

    /// Spawns the relay task in the background.
    pub fn spawn(self) {
        tokio::spawn(async move {
            self.run().await;
        });
    }

    async fn run(&self) {
        loop {
            if let Err(e) = self.process_outbox().await {
                error!("Error processing outbox events: {}", e);
            }
            sleep(self.poll_interval).await;
        }
    }

    async fn process_outbox(&self) -> Result<(), sea_orm::DbErr> {
        let worker_id = Uuid::new_v4().to_string();
        let lease_until = (chrono::Utc::now() + chrono::Duration::minutes(2)).to_rfc3339();
        let sql = r#"
            SELECT id, payload
            FROM outbox_events
            WHERE locked_until IS NULL OR locked_until < $1
            ORDER BY created_at ASC
            LIMIT 100
        "#;

        let backend = self.db.get_database_backend();
        let rows = self.db.query_all(Statement::from_sql_and_values(
            backend,
            sql,
            vec![chrono::Utc::now().to_rfc3339().into()],
        )).await?;

        if rows.is_empty() {
            trace!("No outbox events to process");
            return Ok(());
        }

        debug!("Processing {} outbox events", rows.len());

        let mut processed_ids = Vec::new();

        for row in rows {
            let id: String = row.try_get_by_index(0)?;
            let payload_str: String = row.try_get_by_index(1)?;

            let claimed = self.db.execute(Statement::from_sql_and_values(
                backend,
                "UPDATE outbox_events SET locked_by = $1, locked_until = $2, attempts = attempts + 1 WHERE id = $3 AND (locked_until IS NULL OR locked_until < $4)",
                vec![worker_id.clone().into(), lease_until.clone().into(), id.clone().into(), chrono::Utc::now().to_rfc3339().into()],
            )).await?.rows_affected();
            if claimed == 0 {
                continue;
            }

            // Deserialize the envelope
            match serde_json::from_str::<EventEnvelope>(&payload_str) {
                Ok(envelope) => {
                    // Publish to the event bus
                    self.event_bus.publish(envelope);
                    processed_ids.push(id);
                }
                Err(e) => {
                    error!("Failed to deserialize outbox event {}: {}", id, e);
                    let _ = self.db.execute(Statement::from_sql_and_values(
                        backend,
                        "UPDATE outbox_events SET last_error = $1, locked_by = NULL, locked_until = NULL WHERE id = $2 AND locked_by = $3",
                        vec![e.to_string().into(), id.into(), worker_id.clone().into()],
                    )).await?;
                }
            }
        }

        if !processed_ids.is_empty() {
            // Delete processed events
            let placeholders = processed_ids
                .iter()
                .enumerate()
                .map(|(index, _)| if backend == DbBackend::Postgres { format!("${}", index + 2) } else { "?".to_string() })
                .collect::<Vec<_>>().join(",");
            let delete_sql = format!("DELETE FROM outbox_events WHERE locked_by = $1 AND id IN ({})", placeholders);
            
            let mut values = vec![worker_id.into()];
            values.extend(processed_ids.into_iter().map(|id| id.into()));
            
            self.db.execute(Statement::from_sql_and_values(
                backend,
                &delete_sql,
                values,
            )).await?;
        }

        Ok(())
    }
}
