//! Orbis v2.0 Schema Migration — enterprise architecture upgrade.
//!
//! Provides embedded SQL migrations for:
//!   - Adding 7 new tables (autopilot, AI, data quality)
//!   - Modifying existing tables (audit_log actor tracking)
//!   - Dropping duplicate tables (cleanup phase)
//!
//! # Usage
//! ```rust
//! use infra_persistence::schema_v2;
//!
//! // Phase 1: Create new tables + modify existing (safe — idempotent)
//! schema_v2::apply_v2_schema(&db).await?;
//!
//! // Phase 2: Drop duplicates (destructive — run after data migration)
//! schema_v2::apply_v2_cleanup(&db).await;
//! ```

use sea_orm::{ConnectionTrait, DbErr};
use tracing::{info, warn};

use crate::migrations::{run_sql_migration, run_sql_cleanup};

/// The v2.0 schema migration SQL (7 new tables + audit_log modifications).
const V2_SCHEMA_SQL: &str = include_str!("schema_v2.sql");

/// The v2.0 cleanup SQL (drop duplicate tables).
const V2_CLEANUP_SQL: &str = include_str!("schema_v2_cleanup.sql");

/// Apply the v2.0 schema additions — safe to run multiple times.
///
/// Creates:
///   - autopilot_config, autopilot_tasks, autopilot_execution_log
///   - ai_agent_sessions, ai_action_log, ai_insights
///   - data_quality_findings
///
/// Modifies:
///   - audit_log: adds actor_type, actor_session_id, correlation_id
pub async fn apply_v2_schema<C: ConnectionTrait>(db: &C) -> Result<(), DbErr> {
    info!("🔄 Applying Orbis v2.0 schema migration...");

    // Split on ALTER TABLE since those may fail if columns already exist
    let parts: Vec<&str> = V2_SCHEMA_SQL.split("-- ═══════════════════════════════════════════════════════════════").collect();

    // Part 1: CREATE TABLE statements (idempotent via IF NOT EXISTS)
    if let Some(create_part) = parts.get(1) {
        run_sql_migration(db, create_part).await?;
        info!("  ✅ 7 new tables created");
    }

    // Part 2: ALTER TABLE statements (may fail if columns exist — that's OK)
    if let Some(alter_part) = parts.get(2) {
        for line in alter_part.split(';') {
            let stmt = line.trim();
            if stmt.is_empty() || stmt.starts_with("--") {
                continue;
            }
            match db.execute_unprepared(stmt).await {
                Ok(_) => {}
                Err(e) => {
                    // Expected: "duplicate column name" if already applied
                    let msg = e.to_string();
                    if msg.contains("duplicate column") {
                        // Already applied — ignore
                    } else {
                        warn!("  ⚠️ ALTER TABLE warning: {}", msg);
                    }
                }
            }
        }
        info!("  ✅ audit_log columns updated");
    }

    info!("✅ Orbis v2.0 schema migration complete");
    Ok(())
}

/// Apply the v2.0 cleanup — DROP duplicate tables.
///
/// ⚠️  **Destructive** — only run after confirming data has been migrated.
///
/// Drops:
///   - attendance (replaced by attendance_daily)
///   - attendance_summary (computed on-the-fly)
///   - system_log (replaced by audit_log)
///   - registry_audit_log (merged into audit_log)
///   - permissions (replaced by role_permissions)
pub async fn apply_v2_cleanup<C: ConnectionTrait>(db: &C) {
    info!("🗑️  Applying v2.0 cleanup (dropping duplicate tables)...");
    run_sql_cleanup(db, V2_CLEANUP_SQL).await;
    info!("✅ v2.0 cleanup complete — duplicate tables dropped");
}
