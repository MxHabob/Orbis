//! Database connection management with optimized SQLite pragmas.
//!
//! # Performance Pragmas (Enterprise Architecture 2026)
//!
//! | Pragma            | Value     | Effect                                       |
//! |--------------------|-----------|----------------------------------------------|
//! | journal_mode       | WAL       | Concurrent readers, non-blocking writes       |
//! | synchronous        | NORMAL    | Safe with WAL, ~2x faster than FULL           |
//! | foreign_keys       | ON        | Referential integrity enforced                |
//! | cache_size         | -64000    | 64MB page cache (negative = KiB)              |
//! | busy_timeout       | 5000      | Wait 5s before returning SQLITE_BUSY          |
//! | mmap_size          | 268435456 | 256MB memory-mapped I/O                       |
//! | temp_store         | MEMORY    | Temp tables in RAM                            |

use std::time::Duration;
use sea_orm::{ConnectOptions, ConnectionTrait, Database, DatabaseConnection, DbErr};
use tracing::info;

/// Standard SQLite pragmas for read-write year databases.
const YEAR_DB_PRAGMAS: &str = "\
    PRAGMA journal_mode=WAL; \
    PRAGMA synchronous=NORMAL; \
    PRAGMA foreign_keys=ON; \
    PRAGMA cache_size=-64000; \
    PRAGMA busy_timeout=5000; \
    PRAGMA mmap_size=268435456; \
    PRAGMA temp_store=MEMORY;";

/// Lighter pragmas for the registry database (small, low traffic).
const REGISTRY_DB_PRAGMAS: &str = "\
    PRAGMA journal_mode=WAL; \
    PRAGMA synchronous=NORMAL; \
    PRAGMA foreign_keys=ON; \
    PRAGMA cache_size=-32000; \
    PRAGMA busy_timeout=5000;";

/// Pragmas for read-only archived databases.
const ARCHIVED_DB_PRAGMAS: &str = "\
    PRAGMA foreign_keys=ON; \
    PRAGMA cache_size=-16000; \
    PRAGMA query_only=ON;";

/// Connection pool presets.
pub struct PoolConfig {
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout_secs: u64,
    pub idle_timeout_secs: u64,
    pub max_lifetime_secs: u64,
}

impl PoolConfig {
    /// Active year database — high concurrency.
    pub fn year() -> Self {
        Self {
            max_connections: 10,
            min_connections: 2,
            connect_timeout_secs: 5,
            idle_timeout_secs: 30,
            max_lifetime_secs: 1800,
        }
    }

    /// Registry database — low concurrency.
    pub fn registry() -> Self {
        Self {
            max_connections: 5,
            min_connections: 1,
            connect_timeout_secs: 5,
            idle_timeout_secs: 60,
            max_lifetime_secs: 3600,
        }
    }

    /// Archived year — read-only, minimal connections.
    pub fn archived() -> Self {
        Self {
            max_connections: 3,
            min_connections: 1,
            connect_timeout_secs: 5,
            idle_timeout_secs: 30,
            max_lifetime_secs: 1800,
        }
    }
}

/// Connect to a SQLite database with the given URL, pool config, and pragmas.
async fn connect_with(
    url: &str,
    pool: &PoolConfig,
    pragmas: &str,
) -> Result<DatabaseConnection, DbErr> {
    let mut opt = ConnectOptions::new(url.to_owned());
    opt.max_connections(pool.max_connections)
        .min_connections(pool.min_connections)
        .connect_timeout(Duration::from_secs(pool.connect_timeout_secs))
        .acquire_timeout(Duration::from_secs(pool.connect_timeout_secs))
        .idle_timeout(Duration::from_secs(pool.idle_timeout_secs))
        .max_lifetime(Duration::from_secs(pool.max_lifetime_secs))
        .sqlx_logging(true)
        .sqlx_logging_level(tracing::log::LevelFilter::Debug);

    let db = Database::connect(opt).await?;
    db.execute_unprepared(pragmas).await?;

    Ok(db)
}

/// Connect to a year database with full read-write access and optimized pragmas.
pub async fn connect_year(url: &str) -> Result<DatabaseConnection, DbErr> {
    let db = connect_with(url, &PoolConfig::year(), YEAR_DB_PRAGMAS).await?;
    info!("✅ Year database connected with optimized pragmas");
    Ok(db)
}

/// Connect to the registry database.
pub async fn connect_registry_db(url: &str) -> Result<DatabaseConnection, DbErr> {
    let db = connect_with(url, &PoolConfig::registry(), REGISTRY_DB_PRAGMAS).await?;
    info!("✅ Registry database connected");
    Ok(db)
}

/// Connect to an archived year database in read-only mode.
pub async fn connect_archived(url: &str) -> Result<DatabaseConnection, DbErr> {
    let db = connect_with(url, &PoolConfig::archived(), ARCHIVED_DB_PRAGMAS).await?;
    info!("📚 Archived database connected (read-only)");
    Ok(db)
}

/// Run PRAGMA optimize — call before closing the database for best performance
/// on next open. This analyzes query patterns and updates index statistics.
pub async fn optimize(db: &DatabaseConnection) -> Result<(), DbErr> {
    db.execute_unprepared("PRAGMA optimize;").await?;
    Ok(())
}
