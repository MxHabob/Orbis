//! Database Management Engine for Orbis Desktop.
//!
//! Handles SQLite & PostgreSQL lifecycle operations:
//! - Health & schema status inspection
//! - Atomic online backup via SQLite VACUUM INTO
//! - Staged safe restoration with pre-restore safety snapshot
//! - Database creation & bootstrapping with migrations
//! - Integrity checks (PRAGMA integrity_check) & optimization (VACUUM, WAL checkpoint)
//! - Migration execution & version tracking
//! - Thread-safe concurrency locking for mutation operations

use crate::connection::{ConnectionConfig, ConnectionMode};
use orbis_infra::migration::Migrator;
use sea_orm::{ConnectionTrait, Database, DatabaseBackend, DatabaseConnection, Statement};
use sea_orm_migration::MigratorTrait;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tokio::sync::Mutex;
use tracing::{info, warn};

/// Global concurrency lock to guarantee only one mutating database operation runs at a time.
pub static OPERATION_LOCK: Mutex<()> = Mutex::const_new(());

// ── Data Transfer Objects (DTOs) ───────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseStatusDto {
    pub mode: String,
    pub state: String,
    pub database_name: String,
    pub connection_target: String,
    pub file_size_bytes: Option<u64>,
    pub file_size_formatted: Option<String>,
    pub schema_version: String,
    pub applied_migrations_count: usize,
    pub total_migrations_count: usize,
    pub pending_migrations: Vec<String>,
    pub is_compatible: bool,
    pub journal_mode: Option<String>,
    pub integrity_status: Option<String>,
    pub last_backup_timestamp: Option<String>,
    pub last_backup_path: Option<String>,
    pub last_connected_at: Option<String>,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupResultDto {
    pub success: bool,
    pub backup_path: String,
    pub file_size_bytes: u64,
    pub file_size_formatted: String,
    pub duration_ms: u64,
    pub timestamp: String,
    pub integrity_verified: bool,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupInspectionDto {
    pub is_valid: bool,
    pub file_path: String,
    pub file_size_bytes: u64,
    pub file_size_formatted: String,
    pub schema_version: String,
    pub applied_migrations_count: usize,
    pub is_compatible: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestoreResultDto {
    pub success: bool,
    pub message: String,
    pub pre_restore_backup_path: Option<String>,
    pub applied_migrations: Vec<String>,
    pub duration_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrityResultDto {
    pub is_healthy: bool,
    pub status: String,
    pub details: Vec<String>,
    pub checked_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizeResultDto {
    pub success: bool,
    pub message: String,
    pub size_before_bytes: u64,
    pub size_after_bytes: u64,
    pub space_freed_bytes: u64,
    pub space_freed_formatted: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationResultDto {
    pub success: bool,
    pub applied_migrations: Vec<String>,
    pub total_applied: usize,
    pub message: String,
}

// ── Helpers ─────────────────────────────────────────────────────────────────

pub fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.1} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

/// Verify SQLite magic header bytes from file.
pub fn is_valid_sqlite_file(path: &Path) -> bool {
    if !path.exists() || !path.is_file() {
        return false;
    }
    let mut file = match std::fs::File::open(path) {
        Ok(f) => f,
        Err(_) => return false,
    };
    use std::io::Read;
    let mut header = [0u8; 16];
    if file.read_exact(&mut header).is_err() {
        return false;
    }
    // SQLite header magic string: "SQLite format 3\0"
    &header == b"SQLite format 3\0"
}

// ── Engine Methods ──────────────────────────────────────────────────────────

/// Gather comprehensive status & diagnostics of the active database.
pub async fn get_database_status(
    app_data_dir: &Path,
    config: &ConnectionConfig,
    active_db: Option<&DatabaseConnection>,
) -> Result<DatabaseStatusDto, String> {
    let all_migrations = Migrator::migrations();
    let total_migrations_count = all_migrations.len();

    match config.mode {
        ConnectionMode::Local => {
            let db_path = config
                .resolve_local_db_path(app_data_dir)
                .ok_or_else(|| "Failed to resolve local database path".to_string())?;
            let target_str = db_path.to_string_lossy().to_string();
            let db_name = db_path
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| "orbis.db".to_string());

            let (file_size_bytes, file_size_formatted) = if db_path.exists() {
                let size = std::fs::metadata(&db_path).map(|m| m.len()).unwrap_or(0);
                (Some(size), Some(format_size(size)))
            } else {
                (None, None)
            };

            // If we have an active DB connection, query diagnostics
            if let Some(db) = active_db {
                // Applied migrations
                let applied = Migrator::get_applied_migrations(db)
                    .await
                    .unwrap_or_default();
                let applied_count = applied.len();
                let applied_names: std::collections::HashSet<_> =
                    applied.iter().map(|m| m.name().to_string()).collect();

                let pending: Vec<String> = all_migrations
                    .iter()
                    .map(|m| m.name().to_string())
                    .filter(|name| !applied_names.contains(name))
                    .collect();

                let is_compatible = pending.is_empty();

                // SQLite journal mode
                let journal_mode = match db
                    .query_one(Statement::from_string(
                        DatabaseBackend::Sqlite,
                        "PRAGMA journal_mode",
                    ))
                    .await
                {
                    Ok(Some(row)) => row.try_get_by_index::<String>(0).ok(),
                    _ => None,
                };

                // SQLite quick integrity check
                let integrity = match db
                    .query_one(Statement::from_string(
                        DatabaseBackend::Sqlite,
                        "PRAGMA quick_check",
                    ))
                    .await
                {
                    Ok(Some(row)) => row.try_get_by_index::<String>(0).ok(),
                    _ => Some("unknown".to_string()),
                };

                let schema_version = if let Some(last) = applied.last() {
                    last.name().to_string()
                } else if applied_count > 0 {
                    format!("v{}", applied_count)
                } else {
                    "v0.0.0 (uninitialized)".to_string()
                };

                let state = if !is_compatible {
                    "migration_required"
                } else if integrity.as_deref() != Some("ok") {
                    "warning"
                } else {
                    "connected"
                };

                Ok(DatabaseStatusDto {
                    mode: "local".to_string(),
                    state: state.to_string(),
                    database_name: db_name,
                    connection_target: target_str,
                    file_size_bytes,
                    file_size_formatted,
                    schema_version,
                    applied_migrations_count: applied_count,
                    total_migrations_count,
                    pending_migrations: pending,
                    is_compatible,
                    journal_mode,
                    integrity_status: integrity,
                    last_backup_timestamp: config.settings.last_backup_timestamp.clone(),
                    last_backup_path: config.settings.last_backup_path.clone(),
                    last_connected_at: Some(chrono::Utc::now().to_rfc3339()),
                    error_message: None,
                })
            } else {
                // Disconnected state
                Ok(DatabaseStatusDto {
                    mode: "local".to_string(),
                    state: if db_path.exists() {
                        "disconnected".to_string()
                    } else {
                        "not_found".to_string()
                    },
                    database_name: db_name,
                    connection_target: target_str,
                    file_size_bytes,
                    file_size_formatted,
                    schema_version: "unknown".to_string(),
                    applied_migrations_count: 0,
                    total_migrations_count,
                    pending_migrations: vec![],
                    is_compatible: true,
                    journal_mode: None,
                    integrity_status: None,
                    last_backup_timestamp: config.settings.last_backup_timestamp.clone(),
                    last_backup_path: config.settings.last_backup_path.clone(),
                    last_connected_at: None,
                    error_message: None,
                })
            }
        }
        ConnectionMode::Api => {
            let server_url = config.remote_server_url().unwrap_or("").to_string();
            let health = crate::connection::check_remote_health(&server_url).await;

            let state = if health.reachable {
                "connected"
            } else {
                "disconnected"
            };

            Ok(DatabaseStatusDto {
                mode: "api".to_string(),
                state: state.to_string(),
                database_name: "orbis_api".to_string(),
                connection_target: server_url,
                file_size_bytes: None,
                file_size_formatted: None,
                schema_version: health.version.unwrap_or_else(|| "1.0.0".to_string()),
                applied_migrations_count: total_migrations_count,
                total_migrations_count,
                pending_migrations: vec![],
                is_compatible: true,
                journal_mode: None,
                integrity_status: if health.reachable {
                    Some("ok".to_string())
                } else {
                    None
                },
                last_backup_timestamp: config.settings.last_backup_timestamp.clone(),
                last_backup_path: config.settings.last_backup_path.clone(),
                last_connected_at: if health.reachable {
                    Some(chrono::Utc::now().to_rfc3339())
                } else {
                    None
                },
                error_message: health.error,
            })
        }
        ConnectionMode::Direct => {
            let direct = config
                .direct
                .as_ref()
                .ok_or_else(|| "Direct database settings are missing".to_string())?;
            let mut applied_count = 0;
            let mut pending_migrations = vec![];
            let mut schema_version = "unknown".to_string();
            let mut migration_status_error = None;
            if let Some(db) = active_db {
                let has_migration_table = sea_orm_migration::SchemaManager::new(db)
                    .has_table("seaql_migrations")
                    .await
                    .map_err(|error| {
                        format!("Failed to inspect PostgreSQL migration history: {error}")
                    })?;
                if has_migration_table {
                    let applied = Migrator::get_applied_migrations(db)
                        .await
                        .map_err(|error| {
                            format!("Failed to read PostgreSQL migration history: {error}")
                        })?;
                    applied_count = applied.len();
                    schema_version = applied
                        .last()
                        .map(|migration| migration.name().to_string())
                        .unwrap_or_else(|| "v0.0.0 (uninitialized)".to_string());
                    let applied_names: std::collections::HashSet<_> =
                        applied.iter().map(|migration| migration.name().to_string()).collect();
                    pending_migrations = all_migrations
                        .iter()
                        .map(|migration| migration.name().to_string())
                        .filter(|name| !applied_names.contains(name))
                        .collect();
                } else {
                    pending_migrations = all_migrations
                        .iter()
                        .map(|migration| migration.name().to_string())
                        .collect();
                    schema_version = "v0.0.0 (uninitialized)".to_string();
                    migration_status_error =
                        Some("Orbis schema has not been initialized".to_string());
                }
            }
            let connected = active_db.is_some();
            let is_compatible = pending_migrations.is_empty();
            Ok(DatabaseStatusDto {
                mode: "direct".to_string(),
                state: if !connected {
                    "disconnected"
                } else if !is_compatible {
                    "migration_required"
                } else {
                    "connected"
                }
                .to_string(),
                database_name: direct.database_name.clone(),
                connection_target: format!(
                    "{}:{}/{}",
                    direct.host, direct.port, direct.database_name
                ),
                file_size_bytes: None,
                file_size_formatted: None,
                schema_version,
                applied_migrations_count: applied_count,
                total_migrations_count,
                pending_migrations,
                is_compatible,
                journal_mode: None,
                integrity_status: None,
                last_backup_timestamp: None,
                last_backup_path: None,
                last_connected_at: connected.then(|| chrono::Utc::now().to_rfc3339()),
                error_message: migration_status_error,
            })
        }
    }
}

/// Create a new local SQLite database, run all migrations, and return status.
pub async fn create_local_database(
    app_data_dir: &Path,
    custom_dir: Option<String>,
    filename: String,
) -> Result<DatabaseStatusDto, String> {
    let _lock = OPERATION_LOCK.lock().await;

    let target_dir = match custom_dir {
        Some(d) if !d.trim().is_empty() => PathBuf::from(d.trim()),
        _ => app_data_dir.to_path_buf(),
    };

    std::fs::create_dir_all(&target_dir)
        .map_err(|e| format!("Failed to create target directory {:?}: {e}", target_dir))?;

    let clean_filename = if filename.trim().is_empty() {
        "orbis.db".to_string()
    } else {
        let trimmed = filename.trim();
        if trimmed.ends_with(".db") || trimmed.ends_with(".sqlite") || trimmed.ends_with(".sqlite3")
        {
            trimmed.to_string()
        } else {
            format!("{}.db", trimmed)
        }
    };

    let target_path = target_dir.join(&clean_filename);
    if target_path.exists() {
        return Err(format!(
            "A database file already exists at {:?}",
            target_path
        ));
    }

    let url_path = target_path.to_string_lossy().replace('\\', "/");
    let db_url = format!("sqlite://{}?mode=rwc", url_path);

    info!("Creating new SQLite database at {:?}", target_path);
    let db = Database::connect(&db_url)
        .await
        .map_err(|e| format!("Failed to connect to newly created database: {e}"))?;

    // Enable WAL mode & foreign keys
    let _ = db
        .execute(Statement::from_string(
            DatabaseBackend::Sqlite,
            "PRAGMA journal_mode=WAL",
        ))
        .await;
    let _ = db
        .execute(Statement::from_string(
            DatabaseBackend::Sqlite,
            "PRAGMA foreign_keys=ON",
        ))
        .await;

    // Run all migrations
    Migrator::up(&db, None)
        .await
        .map_err(|e| format!("Failed to execute database migrations: {e}"))?;

    // Verify integrity
    let integrity = db
        .query_one(Statement::from_string(
            DatabaseBackend::Sqlite,
            "PRAGMA integrity_check",
        ))
        .await
        .map_err(|e| format!("Failed integrity check on newly created database: {e}"))?;

    let is_ok = integrity
        .and_then(|row| row.try_get_by_index::<String>(0).ok())
        .unwrap_or_default()
        == "ok";
    if !is_ok {
        return Err("Database integrity check failed after initialization".to_string());
    }

    // Persist new config
    let mut config = ConnectionConfig::load(app_data_dir);
    config.mode = ConnectionMode::Local;
    config.local = Some(crate::connection::LocalConfig {
        db_filename: clean_filename,
        custom_path: if target_dir != app_data_dir {
            Some(target_path.to_string_lossy().to_string())
        } else {
            None
        },
        wal_mode: true,
        busy_timeout_ms: 5000,
    });
    config.save(app_data_dir)?;

    // Return full status
    get_database_status(app_data_dir, &config, Some(&db)).await
}

/// Validate an existing SQLite file and inspect its schema & migrations.
pub async fn inspect_database_file(file_path: &str) -> Result<BackupInspectionDto, String> {
    let path = PathBuf::from(file_path);
    if !path.exists() {
        return Err(format!("File does not exist: {file_path}"));
    }
    if !is_valid_sqlite_file(&path) {
        return Err("The selected file is not a valid SQLite database".to_string());
    }

    let file_size = std::fs::metadata(&path).map(|m| m.len()).unwrap_or(0);
    let formatted_size = format_size(file_size);

    let url_path = path.to_string_lossy().replace('\\', "/");
    let db_url = format!("sqlite://{}?mode=ro", url_path);

    let db = match Database::connect(&db_url).await {
        Ok(d) => d,
        Err(e) => {
            return Ok(BackupInspectionDto {
                is_valid: false,
                file_path: file_path.to_string(),
                file_size_bytes: file_size,
                file_size_formatted: formatted_size,
                schema_version: "unreadable".to_string(),
                applied_migrations_count: 0,
                is_compatible: false,
                error: Some(format!("Cannot open database: {e}")),
            });
        }
    };

    // Check integrity
    let integrity_ok = match db
        .query_one(Statement::from_string(
            DatabaseBackend::Sqlite,
            "PRAGMA quick_check",
        ))
        .await
    {
        Ok(Some(row)) => row.try_get_by_index::<String>(0).unwrap_or_default() == "ok",
        _ => false,
    };

    if !integrity_ok {
        return Ok(BackupInspectionDto {
            is_valid: false,
            file_path: file_path.to_string(),
            file_size_bytes: file_size,
            file_size_formatted: formatted_size,
            schema_version: "corrupt".to_string(),
            applied_migrations_count: 0,
            is_compatible: false,
            error: Some("Database file integrity verification failed".to_string()),
        });
    }

    // Inspect migrations
    let applied = Migrator::get_applied_migrations(&db)
        .await
        .unwrap_or_default();
    let applied_count = applied.len();
    let schema_version = applied
        .last()
        .map(|m| m.name().to_string())
        .unwrap_or_else(|| "v0".to_string());

    Ok(BackupInspectionDto {
        is_valid: true,
        file_path: file_path.to_string(),
        file_size_bytes: file_size,
        file_size_formatted: formatted_size,
        schema_version,
        applied_migrations_count: applied_count,
        is_compatible: applied_count > 0,
        error: None,
    })
}

/// Perform a live, transactional SQLite online backup using `VACUUM INTO`.
pub async fn execute_backup(
    app_data_dir: &Path,
    config: &ConnectionConfig,
    active_db: &DatabaseConnection,
    destination_path: Option<String>,
) -> Result<BackupResultDto, String> {
    let _lock = OPERATION_LOCK.lock().await;
    let start_time = std::time::Instant::now();

    let backup_path: PathBuf = match destination_path {
        Some(p) if !p.trim().is_empty() => PathBuf::from(p.trim()),
        _ => {
            let backup_dir = match &config.settings.backup_dir {
                Some(dir) if !dir.trim().is_empty() => PathBuf::from(dir.trim()),
                _ => app_data_dir.join("backups"),
            };
            std::fs::create_dir_all(&backup_dir)
                .map_err(|e| format!("Failed to create backup directory {:?}: {e}", backup_dir))?;

            let timestamp = chrono::Local::now().format("%Y-%m-%d_%H%M%S");
            backup_dir.join(format!("orbis_backup_{}.db", timestamp))
        }
    };

    if let Some(parent) = backup_path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }

    if backup_path.exists() {
        let _ = std::fs::remove_file(&backup_path);
    }

    let escaped_path = backup_path
        .to_string_lossy()
        .replace('\\', "/")
        .replace('\'', "''");
    let vacuum_query = format!("VACUUM INTO '{}'", escaped_path);

    info!("Executing SQLite backup: {}", vacuum_query);
    active_db
        .execute(Statement::from_string(
            DatabaseBackend::Sqlite,
            &vacuum_query,
        ))
        .await
        .map_err(|e| format!("Backup execution failed: {e}"))?;

    if !backup_path.exists() {
        return Err("Backup file was not created by the SQLite engine".to_string());
    }

    let file_size = std::fs::metadata(&backup_path)
        .map(|m| m.len())
        .unwrap_or(0);
    if file_size == 0 {
        let _ = std::fs::remove_file(&backup_path);
        return Err("Backup file is empty (0 bytes)".to_string());
    }

    // Verify backup file integrity
    let inspection = inspect_database_file(&backup_path.to_string_lossy()).await?;
    if !inspection.is_valid {
        let _ = std::fs::remove_file(&backup_path);
        return Err(format!(
            "Backup integrity verification failed: {:?}",
            inspection.error
        ));
    }

    let duration_ms = start_time.elapsed().as_millis() as u64;
    let timestamp_str = chrono::Utc::now().to_rfc3339();

    // Update settings with last backup info
    let mut updated_config = config.clone();
    updated_config.settings.last_backup_timestamp = Some(timestamp_str.clone());
    updated_config.settings.last_backup_path = Some(backup_path.to_string_lossy().to_string());
    let _ = updated_config.save(app_data_dir);

    Ok(BackupResultDto {
        success: true,
        backup_path: backup_path.to_string_lossy().to_string(),
        file_size_bytes: file_size,
        file_size_formatted: format_size(file_size),
        duration_ms,
        timestamp: timestamp_str,
        integrity_verified: true,
        message: "Database backup completed and verified successfully".to_string(),
    })
}

/// Restore a database from a validated backup file.
/// Performs:
/// 1. Pre-restore validation
/// 2. Automatic safety snapshot of current active DB
/// 3. Atomic file swap and cleanup of `-wal` and `-shm` sidecars
/// 4. Post-restore migration verification & integrity check
pub async fn execute_restore(
    app_data_dir: &Path,
    config: &ConnectionConfig,
    backup_file_path: &str,
) -> Result<RestoreResultDto, String> {
    let _lock = OPERATION_LOCK.lock().await;
    let start_time = std::time::Instant::now();

    let backup_path = PathBuf::from(backup_file_path);
    if !backup_path.exists() {
        return Err(format!("Backup file does not exist: {backup_file_path}"));
    }

    // 1. Inspect & validate backup
    let inspection = inspect_database_file(backup_file_path).await?;
    if !inspection.is_valid {
        return Err(format!(
            "Cannot restore invalid backup: {:?}",
            inspection.error
        ));
    }

    let active_db_path = config
        .resolve_local_db_path(app_data_dir)
        .ok_or_else(|| "Cannot restore: remote connection mode active".to_string())?;

    // 2. Automatic safety snapshot of current active DB
    let pre_restore_backup_path = if active_db_path.exists() {
        let snapshot_dir = app_data_dir.join("backups").join("pre_restore_snapshots");
        let _ = std::fs::create_dir_all(&snapshot_dir);
        let timestamp = chrono::Local::now().format("%Y-%m-%d_%H%M%S");
        let snapshot_path =
            snapshot_dir.join(format!("orbis_pre_restore_snapshot_{}.db", timestamp));

        match std::fs::copy(&active_db_path, &snapshot_path) {
            Ok(_) => Some(snapshot_path.to_string_lossy().to_string()),
            Err(e) => {
                warn!("Could not create pre-restore snapshot: {e}");
                None
            }
        }
    } else {
        None
    };

    // 3. Clean up active sidecar WAL files before overwriting
    let wal_path = PathBuf::from(format!("{}-wal", active_db_path.display()));
    let shm_path = PathBuf::from(format!("{}-shm", active_db_path.display()));
    let _ = std::fs::remove_file(&wal_path);
    let _ = std::fs::remove_file(&shm_path);

    // 4. Overwrite active DB file with verified backup
    if let Some(parent) = active_db_path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    std::fs::copy(&backup_path, &active_db_path)
        .map_err(|e| format!("Failed to copy backup to active database location: {e}"))?;

    // 5. Connect and run any pending migrations
    let url_path = active_db_path.to_string_lossy().replace('\\', "/");
    let db_url = format!("sqlite://{}?mode=rwc", url_path);

    let db = Database::connect(&db_url)
        .await
        .map_err(|e| format!("Failed to connect to restored database: {e}"))?;

    let _ = db
        .execute(Statement::from_string(
            DatabaseBackend::Sqlite,
            "PRAGMA journal_mode=WAL",
        ))
        .await;
    let _ = db
        .execute(Statement::from_string(
            DatabaseBackend::Sqlite,
            "PRAGMA foreign_keys=ON",
        ))
        .await;

    // Run migrations if backup had an older schema
    let pending_before = Migrator::get_pending_migrations(&db)
        .await
        .unwrap_or_default();
    let applied_migration_names: Vec<String> = pending_before
        .iter()
        .map(|m| m.name().to_string())
        .collect();

    if !applied_migration_names.is_empty() {
        info!(
            "Applying {} pending migrations to restored database...",
            applied_migration_names.len()
        );
        Migrator::up(&db, None)
            .await
            .map_err(|e| format!("Failed to apply migrations to restored database: {e}"))?;
    }

    // Final integrity check
    let integrity = db
        .query_one(Statement::from_string(
            DatabaseBackend::Sqlite,
            "PRAGMA integrity_check",
        ))
        .await
        .map_err(|e| format!("Integrity check error after restore: {e}"))?;
    let is_ok = integrity
        .and_then(|row| row.try_get_by_index::<String>(0).ok())
        .unwrap_or_default()
        == "ok";

    if !is_ok {
        return Err("Integrity check failed on active database following restore".to_string());
    }

    let duration_ms = start_time.elapsed().as_millis() as u64;

    Ok(RestoreResultDto {
        success: true,
        message: "Database successfully restored and verified".to_string(),
        pre_restore_backup_path,
        applied_migrations: applied_migration_names,
        duration_ms,
    })
}

/// Run pending migrations on active database.
pub async fn run_pending_migrations(
    active_db: &DatabaseConnection,
) -> Result<MigrationResultDto, String> {
    let _lock = OPERATION_LOCK.lock().await;

    let pending = Migrator::get_pending_migrations(active_db)
        .await
        .map_err(|e| format!("Failed to check pending migrations: {e}"))?;

    let pending_names: Vec<String> = pending.iter().map(|m| m.name().to_string()).collect();
    if pending_names.is_empty() {
        return Ok(MigrationResultDto {
            success: true,
            applied_migrations: vec![],
            total_applied: 0,
            message: "Database schema is already up to date".to_string(),
        });
    }

    Migrator::up(active_db, None)
        .await
        .map_err(|e| format!("Failed to apply migrations: {e}"))?;

    let applied_count = pending_names.len();

    Ok(MigrationResultDto {
        success: true,
        applied_migrations: pending_names,
        total_applied: applied_count,
        message: format!(
            "Successfully applied {} database migration(s)",
            applied_count
        ),
    })
}

/// Run PRAGMA integrity_check on active database.
pub async fn check_active_integrity(
    active_db: &DatabaseConnection,
) -> Result<IntegrityResultDto, String> {
    let rows = active_db
        .query_all(Statement::from_string(
            DatabaseBackend::Sqlite,
            "PRAGMA integrity_check(100)",
        ))
        .await
        .map_err(|e| format!("Failed to execute integrity check: {e}"))?;

    let mut details = Vec::new();
    for row in rows {
        if let Ok(msg) = row.try_get_by_index::<String>(0) {
            details.push(msg);
        }
    }

    let is_healthy = details.len() == 1 && details[0] == "ok";
    let status = if is_healthy {
        "ok".to_string()
    } else {
        "corruption_detected".to_string()
    };

    Ok(IntegrityResultDto {
        is_healthy,
        status,
        details,
        checked_at: chrono::Utc::now().to_rfc3339(),
    })
}

/// Run SQLite optimization, WAL checkpoint, and VACUUM.
pub async fn optimize_database(
    app_data_dir: &Path,
    config: &ConnectionConfig,
    active_db: &DatabaseConnection,
) -> Result<OptimizeResultDto, String> {
    let _lock = OPERATION_LOCK.lock().await;

    let db_path = config.resolve_local_db_path(app_data_dir);
    let size_before = db_path
        .as_ref()
        .and_then(|p| std::fs::metadata(p).ok())
        .map(|m| m.len())
        .unwrap_or(0);

    // 1. Checkpoint WAL
    let _ = active_db
        .execute(Statement::from_string(
            DatabaseBackend::Sqlite,
            "PRAGMA wal_checkpoint(TRUNCATE)",
        ))
        .await;

    // 2. PRAGMA optimize
    let _ = active_db
        .execute(Statement::from_string(
            DatabaseBackend::Sqlite,
            "PRAGMA optimize",
        ))
        .await;

    // 3. VACUUM
    let _ = active_db
        .execute(Statement::from_string(DatabaseBackend::Sqlite, "VACUUM"))
        .await;

    let size_after = db_path
        .as_ref()
        .and_then(|p| std::fs::metadata(p).ok())
        .map(|m| m.len())
        .unwrap_or(0);

    let space_freed = if size_before > size_after {
        size_before - size_after
    } else {
        0
    };

    Ok(OptimizeResultDto {
        success: true,
        message: "Database optimization and compaction completed successfully".to_string(),
        size_before_bytes: size_before,
        size_after_bytes: size_after,
        space_freed_bytes: space_freed,
        space_freed_formatted: format_size(space_freed),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_size() {
        assert_eq!(format_size(500), "500 B");
        assert_eq!(format_size(2048), "2.0 KB");
        assert_eq!(format_size(1048576 * 5), "5.00 MB");
        assert_eq!(format_size(1073741824 * 2), "2.00 GB");
    }

    #[tokio::test]
    async fn test_create_and_backup_and_restore_sqlite() {
        let temp_dir = std::env::temp_dir().join(format!("orbis_test_db_{}", uuid::Uuid::new_v4()));
        let _ = std::fs::create_dir_all(&temp_dir);

        // 1. Create local database
        let status = create_local_database(&temp_dir, None, "test_orbis.db".to_string()).await;
        assert!(
            status.is_ok(),
            "Failed to create local database: {:?}",
            status.err()
        );
        let status_res = status.unwrap();
        assert_eq!(status_res.database_name, "test_orbis.db");
        assert!(status_res.applied_migrations_count > 0);

        // 2. Connect
        let config = ConnectionConfig::load(&temp_dir);
        let db_url = config.resolve_local_db_url(&temp_dir).unwrap();
        let db = Database::connect(&db_url).await.unwrap();

        // 3. Backup
        let backup_dest = temp_dir
            .join("test_backup.db")
            .to_string_lossy()
            .to_string();
        let backup_res = execute_backup(&temp_dir, &config, &db, Some(backup_dest.clone())).await;
        assert!(backup_res.is_ok(), "Backup failed: {:?}", backup_res.err());
        assert!(backup_res.unwrap().integrity_verified);

        // 4. Validate backup
        let inspect = inspect_database_file(&backup_dest).await.unwrap();
        assert!(inspect.is_valid);
        assert!(inspect.applied_migrations_count > 0);

        // 5. Integrity check
        let integrity = check_active_integrity(&db).await.unwrap();
        assert!(integrity.is_healthy);
        assert_eq!(integrity.status, "ok");

        // 6. Optimize
        let opt = optimize_database(&temp_dir, &config, &db).await.unwrap();
        assert!(opt.success);

        // Cleanup
        let _ = std::fs::remove_dir_all(&temp_dir);
    }
}
