use std::sync::Arc;
use std::time::Duration;
use sea_orm::{DatabaseConnection, DbBackend, Statement, ConnectionTrait};
use tracing::{error, trace, debug};
use tokio::time::sleep;

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
        // Fetch up to 100 events at a time to prevent memory issues and lock holding
        let sql = r#"
            SELECT id, payload
            FROM outbox_events
            ORDER BY created_at ASC
            LIMIT 100
        "#;

        let rows = self.db.query_all(Statement::from_sql_and_values(
            DbBackend::Sqlite,
            sql,
            vec![],
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

            // Deserialize the envelope
            match serde_json::from_str::<EventEnvelope>(&payload_str) {
                Ok(envelope) => {
                    // Publish to the event bus
                    self.event_bus.publish(envelope);
                    processed_ids.push(id);
                }
                Err(e) => {
                    error!("Failed to deserialize outbox event {}: {}", id, e);
                    // We still delete it to prevent poison messages from blocking the relay
                    processed_ids.push(id);
                }
            }
        }

        if !processed_ids.is_empty() {
            // Delete processed events
            let placeholders = processed_ids.iter().map(|_| "?").collect::<Vec<_>>().join(",");
            let delete_sql = format!("DELETE FROM outbox_events WHERE id IN ({})", placeholders);
            
            let values = processed_ids.into_iter().map(|id| id.into()).collect::<Vec<sea_orm::Value>>();
            
            self.db.execute(Statement::from_sql_and_values(
                DbBackend::Sqlite,
                &delete_sql,
                values,
            )).await?;
        }

        Ok(())
    }
}
