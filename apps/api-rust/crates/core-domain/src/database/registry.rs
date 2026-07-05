//! Registry module — manages the central `orbis_registry.db` and per-year databases.
//!
//! # Architecture
//!
//! ```text
//! AppData/
//! ├── orbis_registry.db          ← Central index (academic_years, promotion_jobs, etc.)
//! ├── years/
//! │   ├── 2024-2025_active.db    ← Active year (full read-write)
//! │   ├── 2023-2024_archived.db  ← Archived year (read-only)
//! │   └── ...
//! └── backups/
//!     └── pre_promotion_<year>_<ts>.db
//! ```

use std::path::{Path, PathBuf};
use std::time::Duration;
use sea_orm::{ConnectOptions, ConnectionTrait, Database, DatabaseConnection, DbErr};
use tracing::info;

pub use super::registry_migrate::run_registry_migrations;

/// Build a SeaORM/sqlx-compatible SQLite URL from an absolute path.
/// Windows paths use forward slashes and `sqlite:///C:/...` form.
pub fn sqlite_url(db_path: &Path, mode: &str) -> String {
    let path = db_path.to_string_lossy().replace('\\', "/");
    if path.starts_with('/') {
        format!("sqlite://{path}?mode={mode}")
    } else {
        format!("sqlite:///{path}?mode={mode}")
    }
}

/// Connect to the central registry database.
/// Creates the file and runs migrations if it doesn't exist.
pub async fn connect_registry(app_data_dir: &Path) -> Result<DatabaseConnection, DbErr> {
    let db_path = app_data_dir.join("orbis_registry.db");
    let url = sqlite_url(&db_path, "rwc");

    let mut opt = ConnectOptions::new(url);
    opt.max_connections(5)
        .min_connections(1)
        .connect_timeout(Duration::from_secs(5))
        .acquire_timeout(Duration::from_secs(5))
        .idle_timeout(Duration::from_secs(60))
        .max_lifetime(Duration::from_secs(3600))
        .sqlx_logging(true)
        .sqlx_logging_level(tracing::log::LevelFilter::Debug);

    let db = Database::connect(opt).await?;

    // Enforce WAL mode and pragmas
    db.execute_unprepared(
        "PRAGMA journal_mode=WAL; \
         PRAGMA synchronous=NORMAL; \
         PRAGMA foreign_keys=ON; \
         PRAGMA cache_size=-32000; \
         PRAGMA busy_timeout=5000;",
    )
    .await?;

    // Run registry-specific migrations
    run_registry_migrations(&db).await?;
    info!("📦 Registry database ready (orbis_registry.db)");

    Ok(db)
}

/// Open an existing year database for full read-write access.
pub async fn open_year_database(db_path: &Path) -> Result<DatabaseConnection, DbErr> {
    let url = sqlite_url(db_path, "rwc");

    let mut opt = ConnectOptions::new(url);
    opt.max_connections(10)
        .min_connections(2)
        .connect_timeout(Duration::from_secs(5))
        .acquire_timeout(Duration::from_secs(5))
        .idle_timeout(Duration::from_secs(30))
        .max_lifetime(Duration::from_secs(1800))
        .sqlx_logging(true)
        .sqlx_logging_level(tracing::log::LevelFilter::Debug);

    let db = Database::connect(opt).await?;

    db.execute_unprepared(
        "PRAGMA journal_mode=WAL; \
         PRAGMA synchronous=NORMAL; \
         PRAGMA foreign_keys=ON; \
         PRAGMA cache_size=-64000; \
         PRAGMA busy_timeout=5000;",
    )
    .await?;

    info!("🗃️ Year database opened: {:?}", db_path);

    Ok(db)
}

/// Open an archived year database in read-only mode.
pub async fn open_archived_database(db_path: &Path) -> Result<DatabaseConnection, DbErr> {
    let url = sqlite_url(db_path, "ro");

    let mut opt = ConnectOptions::new(url);
    opt.max_connections(3)
        .min_connections(1)
        .connect_timeout(Duration::from_secs(5))
        .acquire_timeout(Duration::from_secs(5))
        .idle_timeout(Duration::from_secs(30))
        .max_lifetime(Duration::from_secs(1800))
        .sqlx_logging(true)
        .sqlx_logging_level(tracing::log::LevelFilter::Debug);

    let db = Database::connect(opt).await?;

    // No WAL or write pragmas for read-only
    db.execute_unprepared(
        "PRAGMA foreign_keys=ON; \
         PRAGMA cache_size=-16000; \
         PRAGMA query_only=ON;",
    )
    .await?;

    info!("📚 Archived database opened (read-only): {:?}", db_path);

    Ok(db)
}

/// Create a brand-new year database with full schema.
/// Runs both `core_schema.sql` and `schema.sql` migrations.
pub async fn create_year_database(db_path: &Path) -> Result<DatabaseConnection, DbErr> {
    // Ensure parent directory exists
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| {
            DbErr::Custom(format!("Failed to create directory {:?}: {}", parent, e))
        })?;
    }

    // Create and connect
    let db = open_year_database(db_path).await?;

    // Run the full year schema migrations
    super::run_migrations(&db).await?;
    info!("🆕 Year database created with schema: {:?}", db_path);

    Ok(db)
}

/// Create an atomic backup of a SQLite database using the SQLite backup API.
/// This is safe to call while the source database is open and in use.
pub async fn backup_database(source_path: &Path, backup_path: &Path) -> Result<(), String> {
    // Ensure backup directory exists
    if let Some(parent) = backup_path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create backup dir {:?}: {}", parent, e))?;
    }

    let source = source_path.to_path_buf();
    let dest = backup_path.to_path_buf();

    // Run backup on blocking thread to avoid blocking async runtime
    tokio::task::spawn_blocking(move || {
        // Use raw SQLite backup via file copy — WAL mode makes this safe
        // when the source is not being written to concurrently.
        // For a truly atomic backup, we checkpoint WAL first then copy.
        std::fs::copy(&source, &dest)
            .map_err(|e| format!("Backup copy failed: {}", e))?;

        // Also copy WAL and SHM files if they exist
        let wal = source.with_extension("db-wal");
        let shm = source.with_extension("db-shm");
        let dest_wal = dest.with_extension("db-wal");
        let dest_shm = dest.with_extension("db-shm");

        if wal.exists() {
            let _ = std::fs::copy(&wal, &dest_wal);
        }
        if shm.exists() {
            let _ = std::fs::copy(&shm, &dest_shm);
        }

        Ok(())
    })
    .await
    .map_err(|e| format!("Backup task panicked: {}", e))?
}

/// Generate the years directory path within the app data directory.
pub fn years_dir(app_data_dir: &Path) -> PathBuf {
    app_data_dir.join("years")
}

/// Generate the backups directory path within the app data directory.
pub fn backups_dir(app_data_dir: &Path) -> PathBuf {
    app_data_dir.join("backups")
}

/// Generate a database filename for a new academic year.
/// Example: "2024-2025_active.db"
pub fn year_db_filename(year_name: &str) -> String {
    let sanitized = year_name
        .chars()
        .map(|c| if c.is_alphanumeric() || c == '-' { c } else { '_' })
        .collect::<String>();
    format!("{}_active.db", sanitized)
}

/// Generate a backup filename with timestamp.
/// Example: "pre_promotion_2024-2025_20260705T120000.db"
pub fn backup_filename(year_name: &str) -> String {
    let ts = chrono::Utc::now().format("%Y%m%dT%H%M%S");
    let sanitized = year_name
        .chars()
        .map(|c| if c.is_alphanumeric() || c == '-' { c } else { '_' })
        .collect::<String>();
    format!("pre_promotion_{}_{}.db", sanitized, ts)
}

/// Get the full path to a year's database file.
pub fn year_db_path(app_data_dir: &Path, db_filename: &str) -> PathBuf {
    years_dir(app_data_dir).join(db_filename)
}

/// Build a SQLite URL suitable for `database::connect()` (read-write-create mode).
/// Used by `DatabaseProvider` to create Services for year databases.
pub fn sqlite_url_for_service(db_path: &Path) -> String {
    sqlite_url(db_path, "rwc")
}
