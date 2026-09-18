//! Connection Tauri IPC commands — manage connection setup before AppState exists.
//!
//! These commands can be called BEFORE the database is connected.
//! They read/write the connection config file and perform health checks.

use tauri::{State, Manager};
use std::sync::Arc;
use crate::connection::{ConnectionConfig, ConnectionMode, LocalConfig, RemoteConfig, ConnectionValidation, RemoteHealthResult};

/// Managed state for connection config — available before database init.
pub struct ConnectionState {
    pub app_data_dir: std::path::PathBuf,
    pub config: Arc<std::sync::RwLock<ConnectionConfig>>,
}

/// Get the current connection configuration.
#[tauri::command]
pub async fn connection_get_config(
    conn_state: State<'_, ConnectionState>,
) -> Result<ConnectionConfig, String> {
    let config = conn_state.config.read()
        .map_err(|e| format!("Lock error: {e}"))?;
    Ok(config.clone())
}

/// Update the connection configuration and persist to disk.
#[tauri::command]
pub async fn connection_set_config(
    conn_state: State<'_, ConnectionState>,
    mode: String,
    server_url: Option<String>,
    db_filename: Option<String>,
) -> Result<ConnectionConfig, String> {
    let new_config = match mode.as_str() {
        "local" => ConnectionConfig {
            mode: ConnectionMode::Local,
            local: Some(LocalConfig {
                db_filename: db_filename.unwrap_or_else(|| "orbis.db".to_string()),
            }),
            remote: None,
        },
        "remote" => {
            let url = server_url.ok_or("Server URL is required for remote mode")?;
            if url.trim().is_empty() {
                return Err("Server URL cannot be empty".to_string());
            }
            ConnectionConfig {
                mode: ConnectionMode::Remote,
                local: None,
                remote: Some(RemoteConfig {
                    server_url: url,
                    api_key: None,
                }),
            }
        }
        _ => return Err(format!("Invalid connection mode: {mode}")),
    };

    // Save to disk
    new_config.save(&conn_state.app_data_dir)?;

    // Update in-memory state
    let mut config = conn_state.config.write()
        .map_err(|e| format!("Lock error: {e}"))?;
    *config = new_config.clone();

    Ok(new_config)
}

/// Validate the current connection configuration.
#[tauri::command]
pub async fn connection_validate(
    conn_state: State<'_, ConnectionState>,
) -> Result<ConnectionValidation, String> {
    let config = conn_state.config.read()
        .map_err(|e| format!("Lock error: {e}"))?;
    Ok(config.validate(&conn_state.app_data_dir))
}

/// Check health of a remote server.
#[tauri::command]
pub async fn connection_check_remote_health(
    server_url: String,
) -> Result<RemoteHealthResult, String> {
    Ok(crate::connection::check_remote_health(&server_url).await)
}

/// Connect to the configured database and initialize AppState.
/// This is the key command that transitions from "connection setup" to "connected".
#[tauri::command]
pub async fn connection_connect(
    app: tauri::AppHandle,
    conn_state: State<'_, ConnectionState>,
) -> Result<serde_json::Value, String> {
    let config = conn_state.config.read()
        .map_err(|e| format!("Lock error: {e}"))?
        .clone();

    // Validate first
    let validation = config.validate(&conn_state.app_data_dir);
    if !validation.is_valid {
        return Err(validation.message);
    }

    match config.mode {
        ConnectionMode::Local => {
            // Set DATABASE_URL env var for AppState::init to pick up
            if let Some(db_url) = config.resolve_local_db_url(&conn_state.app_data_dir) {
                // SAFETY: Single-threaded at this point — AppState hasn't been created yet.
                unsafe { std::env::set_var("DATABASE_URL", &db_url); }
            }

            // Initialize AppState
            let app_state = crate::state::AppState::init(&conn_state.app_data_dir)
                .await
                .map_err(|e| format!("Failed to initialize local database: {e}"))?;

            // Manage the state in Tauri
            app.manage(app_state);

            Ok(serde_json::json!({
                "connected": true,
                "mode": "local",
                "message": "Connected to local database"
            }))
        }
        ConnectionMode::Remote => {
            let server_url = config.remote_server_url()
                .ok_or("Remote server URL not configured")?
                .to_string();

            // Health check first
            let health = crate::connection::check_remote_health(&server_url).await;
            if !health.reachable {
                return Err(format!(
                    "Cannot connect to remote server: {}",
                    health.error.unwrap_or_else(|| "Server unreachable".to_string())
                ));
            }

            // For remote mode, we still need a local cache database for offline support
            // Initialize with local SQLite as cache + remote as primary
            let app_state = crate::state::AppState::init(&conn_state.app_data_dir)
                .await
                .map_err(|e| format!("Failed to initialize cache database: {e}"))?;

            app.manage(app_state);

            Ok(serde_json::json!({
                "connected": true,
                "mode": "remote",
                "server_url": server_url,
                "server_version": health.version,
                "message": "Connected to remote server"
            }))
        }
    }
}
