//! Registry utilities — path helpers, URL builders, backup management.
//!
//! Pure utility functions with no database connections. The actual registry
//! connection and year switching logic stays in `core-domain::database::provider`
//! during Phase 0 (it depends on Service/Repository which are domain types).

use std::path::{Path, PathBuf};

/// Build a SeaORM/sqlx-compatible SQLite URL from an absolute path.
///
/// Windows paths use forward slashes and `sqlite:///C:/...` form.
pub fn sqlite_url(db_path: &Path, mode: &str) -> String {
    let path = db_path.to_string_lossy().replace('\\', "/");
    if path.starts_with('/') {
        format!("sqlite://{path}?mode={mode}")
    } else {
        format!("sqlite:///{path}?mode={mode}")
    }
}

/// Build a SQLite URL for read-write-create mode.
pub fn sqlite_url_rwc(db_path: &Path) -> String {
    sqlite_url(db_path, "rwc")
}

/// Build a SQLite URL for read-only mode.
pub fn sqlite_url_readonly(db_path: &Path) -> String {
    sqlite_url(db_path, "ro")
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
///
/// Example: `"2024-2025_active.db"`
pub fn year_db_filename(year_name: &str) -> String {
    let sanitized = year_name
        .chars()
        .map(|c| if c.is_alphanumeric() || c == '-' { c } else { '_' })
        .collect::<String>();
    format!("{}_active.db", sanitized)
}

/// Generate a backup filename with timestamp.
///
/// Example: `"pre_promotion_2024-2025_20260705T120000.db"`
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

/// Create an atomic backup of a SQLite database.
///
/// Safe to call while the source database is open (WAL mode).
/// Runs on a blocking thread to avoid blocking the async runtime.
pub async fn backup_database(source_path: &Path, backup_path: &Path) -> Result<(), String> {
    // Ensure backup directory exists
    if let Some(parent) = backup_path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create backup dir {:?}: {}", parent, e))?;
    }

    let source = source_path.to_path_buf();
    let dest = backup_path.to_path_buf();

    tokio::task::spawn_blocking(move || {
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
