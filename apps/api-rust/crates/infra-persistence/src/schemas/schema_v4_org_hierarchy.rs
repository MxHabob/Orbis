//! Orbis v4.0 Schema Migration — Organization Hierarchy Upgrade.
//!
//! Upgrades:
//!   - `org_units`: Materialized Path, settings/policy inheritance, data isolation
//!   - `academic_policies`: Branch-level policy support (`org_unit_id`)
//!   - Performance indexes for common query patterns
//!
//! Safe to run multiple times (uses `CREATE INDEX IF NOT EXISTS` and migration guard).

use sea_orm::{ConnectionTrait, DbErr};
use tracing::info;

use crate::migrations::run_sql_migration;

/// The v4.0 organization hierarchy schema SQL.
const V4_ORG_HIERARCHY_SQL: &str = include_str!("schema_v4_org_hierarchy.sql");

/// Apply the v4.0 organization hierarchy schema — safe to run multiple times.
pub async fn apply_v4_org_hierarchy<C: ConnectionTrait>(db: &C) -> Result<(), DbErr> {
    info!("🔄 Applying Orbis v4.0 Organization Hierarchy schema...");
    run_sql_migration(db, V4_ORG_HIERARCHY_SQL).await?;
    info!("✅ Organization Hierarchy schema applied successfully");
    Ok(())
}
