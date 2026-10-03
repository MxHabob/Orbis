//! Tauri IPC commands for Database Management subsystem.
//!
//! Exposes complete database lifecycle capabilities to the frontend:
//! - Health, schema, and migration status
//! - Create new SQLite databases
//! - Open/switch existing SQLite databases
//! - Online backups with PRAGMA integrity check
//! - Staged safe database restore
//! - Running pending migrations
//! - Database optimization (VACUUM, WAL checkpoint)
//! - Native OS file and directory dialogs
//! - Database settings persistence

use serde::Serialize;
use tauri::{AppHandle, Manager, State};
use tauri_plugin_dialog::DialogExt;

use crate::commands::connection_cmds::ConnectionState;
use crate::connection::{ConnectionConfig, DatabaseSettings};
use crate::features::database_manager::{
    self, BackupInspectionDto, BackupResultDto, DatabaseStatusDto, IntegrityResultDto,
    MigrationResultDto, OptimizeResultDto, RestoreResultDto,
};
use crate::state::AppState;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub data: T,
}

fn ok<T: Serialize>(data: T) -> ApiResponse<T> {
    ApiResponse { data }
}

async fn log_db_audit(state: &AppState, action: &str, details: serde_json::Value) {
    let ctx = crate::commands::make_ctx(state);
    if let Some(user_id) = ctx.actor.id() {
        let entry = orbis_domain::entities::report::ReportAuditEntry {
            id: uuid::Uuid::new_v4(),
            org_id: ctx.tenant_id,
            actor_id: user_id,
            action: action.to_string(),
            entity_type: "database".to_string(),
            entity_id: uuid::Uuid::new_v4(),
            details: Some(details),
            created_at: chrono::Utc::now(),
        };
        let _ = state.report_audit_repo.save(&entry).await;
    }
}

// ── Status & Diagnostics ───────────────────────────────────────────────────

#[tauri::command]
pub async fn database_get_status(
    app: AppHandle,
    conn_state: State<'_, ConnectionState>,
) -> Result<ApiResponse<DatabaseStatusDto>, String> {
    let config = conn_state
        .config
        .read()
        .map_err(|e| format!("Lock error: {e}"))?
        .clone();

    let app_state = app.try_state::<AppState>();
    let active_db = app_state.as_ref().map(|s| &s.db);

    let status =
        database_manager::get_database_status(&conn_state.app_data_dir, &config, active_db).await?;

    Ok(ok(status))
}

// ── Creation & Opening ─────────────────────────────────────────────────────

#[tauri::command]
pub async fn database_create_local(
    app: AppHandle,
    conn_state: State<'_, ConnectionState>,
    custom_dir: Option<String>,
    filename: String,
) -> Result<ApiResponse<DatabaseStatusDto>, String> {
    let status =
        database_manager::create_local_database(&conn_state.app_data_dir, custom_dir, filename)
            .await?;

    // Reconnect AppState to the new database
    let new_config = ConnectionConfig::load(&conn_state.app_data_dir);
    {
        let mut config_guard = conn_state
            .config
            .write()
            .map_err(|e| format!("Lock error: {e}"))?;
        *config_guard = new_config.clone();
    }

    // Initialize/replace AppState
    if let Some(db_url) = new_config.resolve_local_db_url(&conn_state.app_data_dir) {
        unsafe {
            std::env::set_var("DATABASE_URL", &db_url);
        }
    }
    let new_app_state = AppState::init(&conn_state.app_data_dir)
        .await
        .map_err(|e| format!("Failed to initialize AppState with new database: {e}"))?;

    log_db_audit(
        &new_app_state,
        "database_created",
        serde_json::json!({
            "database_name": status.database_name,
            "target": status.connection_target,
        }),
    )
    .await;

    app.manage(new_app_state);

    Ok(ok(status))
}

#[tauri::command]
pub async fn database_open_existing(
    app: AppHandle,
    conn_state: State<'_, ConnectionState>,
    file_path: String,
) -> Result<ApiResponse<DatabaseStatusDto>, String> {
    let inspection = database_manager::inspect_database_file(&file_path).await?;
    if !inspection.is_valid {
        return Err(format!(
            "Cannot open database: {}",
            inspection
                .error
                .unwrap_or_else(|| "Invalid SQLite database file".to_string())
        ));
    }

    let path = std::path::PathBuf::from(&file_path);
    let filename = path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "orbis.db".to_string());

    // Update connection config
    let mut config = conn_state
        .config
        .read()
        .map_err(|e| format!("Lock error: {e}"))?
        .clone();

    config.mode = crate::connection::ConnectionMode::Local;
    config.local = Some(crate::connection::LocalConfig {
        db_filename: filename,
        custom_path: Some(file_path.clone()),
        wal_mode: true,
        busy_timeout_ms: 5000,
    });

    config.save(&conn_state.app_data_dir)?;

    {
        let mut config_guard = conn_state
            .config
            .write()
            .map_err(|e| format!("Lock error: {e}"))?;
        *config_guard = config.clone();
    }

    // Connect AppState to this DB
    if let Some(db_url) = config.resolve_local_db_url(&conn_state.app_data_dir) {
        unsafe {
            std::env::set_var("DATABASE_URL", &db_url);
        }
    }
    let new_app_state = AppState::init(&conn_state.app_data_dir)
        .await
        .map_err(|e| format!("Failed to initialize connection to selected database: {e}"))?;

    log_db_audit(
        &new_app_state,
        "database_switched",
        serde_json::json!({
            "file_path": file_path,
            "schema_version": inspection.schema_version,
        }),
    )
    .await;

    app.manage(new_app_state);

    let status = database_manager::get_database_status(
        &conn_state.app_data_dir,
        &config,
        app.try_state::<AppState>().as_ref().map(|s| &s.db),
    )
    .await?;

    Ok(ok(status))
}

// ── Backup & Restore ───────────────────────────────────────────────────────

#[tauri::command]
pub async fn database_backup(
    state: State<'_, AppState>,
    conn_state: State<'_, ConnectionState>,
    destination_path: Option<String>,
) -> Result<ApiResponse<BackupResultDto>, String> {
    let config = conn_state
        .config
        .read()
        .map_err(|e| format!("Lock error: {e}"))?
        .clone();

    let result = database_manager::execute_backup(
        &conn_state.app_data_dir,
        &config,
        &state.db,
        destination_path,
    )
    .await?;

    // Update in-memory config settings
    {
        let mut config_guard = conn_state
            .config
            .write()
            .map_err(|e| format!("Lock error: {e}"))?;
        config_guard.settings.last_backup_timestamp = Some(result.timestamp.clone());
        config_guard.settings.last_backup_path = Some(result.backup_path.clone());
    }

    log_db_audit(
        &state,
        "database_backup_created",
        serde_json::json!({
            "backup_path": result.backup_path,
            "file_size": result.file_size_formatted,
            "duration_ms": result.duration_ms,
        }),
    )
    .await;

    Ok(ok(result))
}

#[tauri::command]
pub async fn database_validate_backup_file(
    file_path: String,
) -> Result<ApiResponse<BackupInspectionDto>, String> {
    let result = database_manager::inspect_database_file(&file_path).await?;
    Ok(ok(result))
}

#[tauri::command]
pub async fn database_restore(
    app: AppHandle,
    conn_state: State<'_, ConnectionState>,
    backup_file_path: String,
) -> Result<ApiResponse<RestoreResultDto>, String> {
    let config = conn_state
        .config
        .read()
        .map_err(|e| format!("Lock error: {e}"))?
        .clone();

    let result =
        database_manager::execute_restore(&conn_state.app_data_dir, &config, &backup_file_path)
            .await?;

    // Reinitialize AppState with restored database
    let new_app_state = AppState::init(&conn_state.app_data_dir)
        .await
        .map_err(|e| format!("Failed to reconnect to restored database: {e}"))?;

    log_db_audit(
        &new_app_state,
        "database_restored",
        serde_json::json!({
            "restored_from": backup_file_path,
            "pre_restore_backup": result.pre_restore_backup_path,
            "applied_migrations": result.applied_migrations,
        }),
    )
    .await;

    app.manage(new_app_state);

    Ok(ok(result))
}

// ── Maintenance, Migrations & Integrity ────────────────────────────────────

#[tauri::command]
pub async fn database_run_migrations(
    state: State<'_, AppState>,
) -> Result<ApiResponse<MigrationResultDto>, String> {
    let result = database_manager::run_pending_migrations(&state.db).await?;

    log_db_audit(
        &state,
        "database_migrations_applied",
        serde_json::json!({
            "applied": result.applied_migrations,
            "count": result.total_applied,
        }),
    )
    .await;

    Ok(ok(result))
}

#[tauri::command]
pub async fn database_check_integrity(
    state: State<'_, AppState>,
) -> Result<ApiResponse<IntegrityResultDto>, String> {
    let result = database_manager::check_active_integrity(&state.db).await?;
    Ok(ok(result))
}

#[tauri::command]
pub async fn database_optimize(
    state: State<'_, AppState>,
    conn_state: State<'_, ConnectionState>,
) -> Result<ApiResponse<OptimizeResultDto>, String> {
    let config = conn_state
        .config
        .read()
        .map_err(|e| format!("Lock error: {e}"))?
        .clone();

    let result =
        database_manager::optimize_database(&conn_state.app_data_dir, &config, &state.db).await?;

    log_db_audit(
        &state,
        "database_optimized",
        serde_json::json!({
            "freed_bytes": result.space_freed_bytes,
            "freed_formatted": result.space_freed_formatted,
        }),
    )
    .await;

    Ok(ok(result))
}

// ── Settings ───────────────────────────────────────────────────────────────

#[tauri::command]
pub async fn database_get_settings(
    conn_state: State<'_, ConnectionState>,
) -> Result<ApiResponse<DatabaseSettings>, String> {
    let config = conn_state
        .config
        .read()
        .map_err(|e| format!("Lock error: {e}"))?;
    Ok(ok(config.settings.clone()))
}

#[tauri::command]
pub async fn database_update_settings(
    conn_state: State<'_, ConnectionState>,
    settings: DatabaseSettings,
) -> Result<ApiResponse<DatabaseSettings>, String> {
    let mut config = conn_state
        .config
        .read()
        .map_err(|e| format!("Lock error: {e}"))?
        .clone();

    config.settings = settings.clone();
    config.save(&conn_state.app_data_dir)?;

    {
        let mut config_guard = conn_state
            .config
            .write()
            .map_err(|e| format!("Lock error: {e}"))?;
        *config_guard = config;
    }

    Ok(ok(settings))
}

// ── Native OS File & Folder Pickers ────────────────────────────────────────

#[tauri::command]
pub async fn database_pick_file(
    app: AppHandle,
    filter_name: Option<String>,
    extensions: Option<Vec<String>>,
) -> Result<Option<String>, String> {
    let (tx, rx) = tokio::sync::oneshot::channel();

    let mut builder = app.dialog().file();
    let exts: Vec<String> =
        extensions.unwrap_or_else(|| vec!["db".into(), "sqlite".into(), "sqlite3".into()]);
    let ext_slices: Vec<&str> = exts.iter().map(|s| s.as_str()).collect();
    let name = filter_name.unwrap_or_else(|| "SQLite Database (*.db, *.sqlite)".into());

    builder = builder.add_filter(&name, &ext_slices);

    builder.pick_file(move |file_path| {
        let result = file_path.map(|p| p.to_string());
        let _ = tx.send(result);
    });

    rx.await.map_err(|e| format!("File picker error: {e}"))
}

#[tauri::command]
pub async fn database_pick_folder(app: AppHandle) -> Result<Option<String>, String> {
    let (tx, rx) = tokio::sync::oneshot::channel();

    app.dialog().file().pick_folder(move |folder_path| {
        let result = folder_path.map(|p| p.to_string());
        let _ = tx.send(result);
    });

    rx.await.map_err(|e| format!("Folder picker error: {e}"))
}

#[tauri::command]
pub async fn database_pick_save_path(
    app: AppHandle,
    default_filename: Option<String>,
    filter_name: Option<String>,
    extensions: Option<Vec<String>>,
) -> Result<Option<String>, String> {
    let (tx, rx) = tokio::sync::oneshot::channel();

    let mut builder = app.dialog().file();
    let exts: Vec<String> =
        extensions.unwrap_or_else(|| vec!["db".into(), "sqlite".into(), "bak".into()]);
    let ext_slices: Vec<&str> = exts.iter().map(|s| s.as_str()).collect();
    let name = filter_name.unwrap_or_else(|| "Database Backup (*.db, *.bak)".into());

    builder = builder.add_filter(&name, &ext_slices);

    if let Some(def_name) = default_filename {
        builder = builder.set_file_name(&def_name);
    }

    builder.save_file(move |file_path| {
        let result = file_path.map(|p| p.to_string());
        let _ = tx.send(result);
    });

    rx.await.map_err(|e| format!("Save file dialog error: {e}"))
}
