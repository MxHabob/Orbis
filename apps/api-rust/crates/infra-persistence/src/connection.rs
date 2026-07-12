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
use std::path::Path;

/// Standard SQLite pragmas for read-write databases.
const DB_PRAGMAS: &str = "\
    PRAGMA journal_mode=WAL; \
    PRAGMA synchronous=NORMAL; \
    PRAGMA foreign_keys=ON; \
    PRAGMA cache_size=-64000; \
    PRAGMA busy_timeout=5000; \
    PRAGMA mmap_size=268435456; \
    PRAGMA temp_store=MEMORY;";

/// Connection pool presets.
pub struct PoolConfig {
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout_secs: u64,
    pub idle_timeout_secs: u64,
    pub max_lifetime_secs: u64,
}

impl PoolConfig {
    pub fn default() -> Self {
        Self {
            max_connections: 10,
            min_connections: 2,
            connect_timeout_secs: 5,
            idle_timeout_secs: 30,
            max_lifetime_secs: 1800,
        }
    }
}

/// Format the SQLite connection string correctly for SeaORM, with read/write/create mode.
pub fn sqlite_url_rwc(db_path: &Path) -> String {
    format!("sqlite://{}?mode=rwc", db_path.display())
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

/// Connect to a database with full read-write access and optimized pragmas.
pub async fn connect_db(url: &str) -> Result<DatabaseConnection, DbErr> {
    let db = connect_with(url, &PoolConfig::default(), DB_PRAGMAS).await?;
    info!("✅ Database connected with optimized pragmas");
    Ok(db)
}

/// Run PRAGMA optimize — call before closing the database for best performance
/// on next open. This analyzes query patterns and updates index statistics.
pub async fn optimize(db: &DatabaseConnection) -> Result<(), DbErr> {
    db.execute_unprepared("PRAGMA optimize;").await?;
    Ok(())
}
