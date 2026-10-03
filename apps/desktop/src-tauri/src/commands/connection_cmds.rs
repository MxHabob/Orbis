//! Connection Tauri IPC commands — manage connection setup before AppState exists.
//!
//! These commands can be called BEFORE the database is connected.
//! They read/write the connection config file, validate endpoints, and perform health checks.

use crate::connection::{
    ConnectionConfig, ConnectionMode, ConnectionValidation, LocalConfig, RemoteConfig,
    RemoteHealthResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::{Manager, State};

/// Managed state for connection config — available before database init.
pub struct ConnectionState {
    pub app_data_dir: std::path::PathBuf,
    pub config: Arc<std::sync::RwLock<ConnectionConfig>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveConfigRequest {
    pub mode: String,
    pub server_url: Option<String>,
    pub host: Option<String>,
    pub port: Option<u16>,
    pub database_name: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub ssl_mode: Option<String>,
    pub db_filename: Option<String>,
    pub custom_path: Option<String>,
    pub wal_mode: Option<bool>,
    pub busy_timeout_ms: Option<u64>,
}

/// Get the current connection configuration (sanitized so passwords aren't exposed).
#[tauri::command]
pub async fn connection_get_config(
    conn_state: State<'_, ConnectionState>,
) -> Result<ConnectionConfig, String> {
    let config = conn_state
        .config
        .read()
        .map_err(|e| format!("Lock error: {e}"))?;
    Ok(config.sanitized())
}

/// Update the connection configuration and persist to disk.
#[tauri::command]
pub async fn connection_set_config(
    conn_state: State<'_, ConnectionState>,
    mode: String,
    server_url: Option<String>,
    db_filename: Option<String>,
    custom_path: Option<String>,
    host: Option<String>,
    port: Option<u16>,
    database_name: Option<String>,
    username: Option<String>,
    password: Option<String>,
    ssl_mode: Option<String>,
    wal_mode: Option<bool>,
    busy_timeout_ms: Option<u64>,
) -> Result<ConnectionConfig, String> {
    let existing = conn_state
        .config
        .read()
        .map_err(|e| format!("Lock error: {e}"))?
        .clone();

    let new_config = match mode.as_str() {
        "local" => ConnectionConfig {
            mode: ConnectionMode::Local,
            local: Some(LocalConfig {
                db_filename: db_filename.unwrap_or_else(|| "orbis.db".to_string()),
                custom_path: custom_path.filter(|p| !p.trim().is_empty()),
                wal_mode: wal_mode.unwrap_or(true),
                busy_timeout_ms: busy_timeout_ms.unwrap_or(5000),
            }),
            remote: None,
            settings: existing.settings,
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
                    host,
                    port,
                    database_name,
                    username,
                    password: password.filter(|p| p != "••••••••"),
                    ssl_mode,
                    api_key: None,
                }),
                settings: existing.settings,
            }
        }
        _ => return Err(format!("Invalid connection mode: {mode}")),
    };

    // Save to disk
    new_config.save(&conn_state.app_data_dir)?;

    // Update in-memory state
    let mut config = conn_state
        .config
        .write()
        .map_err(|e| format!("Lock error: {e}"))?;
    *config = new_config.clone();

    Ok(new_config.sanitized())
}

/// Reset connection to default local configuration.
#[tauri::command]
pub async fn connection_reset(
    conn_state: State<'_, ConnectionState>,
) -> Result<ConnectionConfig, String> {
    let default_config = ConnectionConfig::default_local();
    default_config.save(&conn_state.app_data_dir)?;

    let mut config = conn_state
        .config
        .write()
        .map_err(|e| format!("Lock error: {e}"))?;
    *config = default_config.clone();

    Ok(default_config)
}

/// Validate the current connection configuration.
#[tauri::command]
pub async fn connection_validate(
    conn_state: State<'_, ConnectionState>,
) -> Result<ConnectionValidation, String> {
    let config = conn_state
        .config
        .read()
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
    let config = conn_state
        .config
        .read()
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
                // SAFETY: Atomic configuration of the connection URL for this process.
                unsafe {
                    std::env::set_var("DATABASE_URL", &db_url);
                }
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
            let server_url = config
                .remote_server_url()
                .ok_or("Remote server URL not configured")?
                .to_string();

            // Health check first
            let health = crate::connection::check_remote_health(&server_url).await;
            if !health.reachable {
                return Err(format!(
                    "Cannot connect to remote server: {}",
                    health
                        .error
                        .unwrap_or_else(|| "Server unreachable".to_string())
                ));
            }

            // For remote mode, we still initialize local SQLite as cache
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
