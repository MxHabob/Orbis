//! Orbis v3.0 Schema Migration — Event Store tables.
//!
//! Adds:
//!   - `domain_events` — append-only event log for Event Sourcing
//!   - `process_log`   — placeholder for future Process Manager persistence
//!
//! Safe to run multiple times (uses `CREATE TABLE IF NOT EXISTS`).

use sea_orm::{ConnectionTrait, DbErr};
use tracing::info;

use crate::migrations::run_sql_migration;

/// The v3.0 event store schema SQL.
const V3_EVENTS_SQL: &str = include_str!("schema_v3_events.sql");

/// Apply the v3.0 event store schema — safe to run multiple times.
pub async fn apply_v3_event_store<C: ConnectionTrait>(db: &C) -> Result<(), DbErr> {
    info!("🔄 Applying Orbis v3.0 Event Store schema...");
    run_sql_migration(db, V3_EVENTS_SQL).await?;
    info!("✅ Event Store schema applied successfully");
    Ok(())
}
