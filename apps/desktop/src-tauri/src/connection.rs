//! Connection configuration — read/write/validate connection settings and database lifecycle.
//!
//! Supports both Local (SQLite) and Remote (PostgreSQL / Orbis Server) connection modes.
//! Configuration is persisted as JSON in AppData/connection.json.
//!
//! This module works before and during database connection lifecycle.

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tracing::info;

fn default_true() -> bool {
    true
}

fn default_false() -> bool {
    false
}

fn default_timeout() -> u64 {
    5000
}

fn default_backup_interval() -> u32 {
    24
}

fn default_max_backups() -> u32 {
    10
}

// ── Connection Configuration Types ──────────────────────────────────────────

/// Connection mode — Local SQLite or Remote Orbis Server / PostgreSQL.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ConnectionMode {
    Local,
    Remote,
}

/// Persisted connection configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionConfig {
    pub mode: ConnectionMode,
    /// Local: SQLite database file path (relative to AppData or absolute).
    pub local: Option<LocalConfig>,
    /// Remote: Orbis Server URL and credentials.
    pub remote: Option<RemoteConfig>,
    /// Database settings and backup policies.
    #[serde(default)]
    pub settings: DatabaseSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalConfig {
    /// Database file name (default: "orbis.db").
    pub db_filename: String,
    /// Optional custom directory or full absolute file path.
    pub custom_path: Option<String>,
    /// Enable SQLite WAL (Write-Ahead Logging) mode for high concurrency.
    #[serde(default = "default_true")]
    pub wal_mode: bool,
    /// Busy timeout in milliseconds.
    #[serde(default = "default_timeout")]
    pub busy_timeout_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteConfig {
    /// Server base URL (e.g. "https://api.orbis.school" or "http://localhost:3000").
    pub server_url: String,
    /// Database host (for direct Postgres connection if configured).
    pub host: Option<String>,
    /// Port (default: 5432 for Postgres, or 80/443 for HTTP).
    pub port: Option<u16>,
    /// Database name.
    pub database_name: Option<String>,
    /// Database username.
    pub username: Option<String>,
    /// Database password (stored securely).
    pub password: Option<String>,
    /// SSL Mode (e.g., "prefer", "require", "disable").
    pub ssl_mode: Option<String>,
    /// API key or token (optional — user may authenticate later).
    pub api_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseSettings {
    #[serde(default = "default_false")]
    pub auto_backup_enabled: bool,
    #[serde(default = "default_backup_interval")]
    pub auto_backup_interval_hours: u32,
    pub backup_dir: Option<String>,
    #[serde(default = "default_max_backups")]
    pub max_backup_files: u32,
    pub last_backup_timestamp: Option<String>,
    pub last_backup_path: Option<String>,
    #[serde(default = "default_true")]
    pub integrity_check_on_startup: bool,
}

impl Default for DatabaseSettings {
    fn default() -> Self {
        Self {
            auto_backup_enabled: false,
            auto_backup_interval_hours: 24,
            backup_dir: None,
            max_backup_files: 10,
            last_backup_timestamp: None,
            last_backup_path: None,
            integrity_check_on_startup: true,
        }
    }
}

/// Connection validation result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionValidation {
    pub is_valid: bool,
    pub mode: String,
    pub message: String,
    /// Resolved database URL for Local mode or endpoint for Remote mode.
    pub resolved_url: Option<String>,
    /// Resolved filesystem path for local SQLite database.
    pub resolved_path: Option<String>,
}

/// Health check result from a remote server.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteHealthResult {
    pub reachable: bool,
    pub status: Option<String>,
    pub version: Option<String>,
    pub api_version: Option<String>,
    pub latency_ms: u64,
    pub error: Option<String>,
}

// ── Configuration Persistence ───────────────────────────────────────────────

impl ConnectionConfig {
    /// Default configuration — Local mode with default database.
    pub fn default_local() -> Self {
        Self {
            mode: ConnectionMode::Local,
            local: Some(LocalConfig {
                db_filename: "orbis.db".to_string(),
                custom_path: None,
                wal_mode: true,
                busy_timeout_ms: 5000,
            }),
            remote: None,
            settings: DatabaseSettings::default(),
        }
    }

    /// Config file path within the app data directory.
    pub fn config_path(app_data_dir: &Path) -> PathBuf {
        app_data_dir.join("connection.json")
    }

    /// Load configuration from disk. Returns default if not found.
    pub fn load(app_data_dir: &Path) -> Self {
        let path = Self::config_path(app_data_dir);
        if path.exists() {
            match std::fs::read_to_string(&path) {
                Ok(contents) => match serde_json::from_str::<ConnectionConfig>(&contents) {
                    Ok(config) => {
                        info!("Loaded connection config from {:?}", path);
                        return config;
                    }
                    Err(e) => {
                        tracing::warn!("Failed to parse connection config: {e}, using default");
                    }
                },
                Err(e) => {
                    tracing::warn!("Failed to read connection config: {e}, using default");
                }
            }
        }
        Self::default_local()
    }

    /// Save configuration to disk.
    pub fn save(&self, app_data_dir: &Path) -> Result<(), String> {
        let path = Self::config_path(app_data_dir);
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize connection config: {e}"))?;
        std::fs::write(&path, json)
            .map_err(|e| format!("Failed to write connection config to {:?}: {e}", path))?;
        info!("Saved connection config to {:?}", path);
        Ok(())
    }

    /// Resolve the full filesystem path for Local mode.
    pub fn resolve_local_db_path(&self, app_data_dir: &Path) -> Option<PathBuf> {
        if self.mode != ConnectionMode::Local {
            return None;
        }
        if let Some(local) = &self.local {
            if let Some(custom) = &local.custom_path {
                let trimmed = custom.trim();
                if !trimmed.is_empty() {
                    let path = PathBuf::from(trimmed);
                    if path.is_absolute() {
                        return Some(path);
                    } else {
                        return Some(app_data_dir.join(path));
                    }
                }
            }
            let filename = if local.db_filename.trim().is_empty() {
                "orbis.db"
            } else {
                local.db_filename.trim()
            };
            return Some(app_data_dir.join(filename));
        }
        Some(app_data_dir.join("orbis.db"))
    }

    /// Resolve the full database URL for Local mode.
    pub fn resolve_local_db_url(&self, app_data_dir: &Path) -> Option<String> {
        let db_path = self.resolve_local_db_path(app_data_dir)?;
        let path_str = db_path.to_string_lossy().replace('\\', "/");
        Some(format!("sqlite://{}?mode=rwc", path_str))
    }

    /// Get the remote server URL.
    pub fn remote_server_url(&self) -> Option<&str> {
        self.remote.as_ref().map(|r| r.server_url.as_str())
    }

    /// Sanitize configuration for returning to the UI (strip password if sensitive).
    pub fn sanitized(&self) -> Self {
        let mut cloned = self.clone();
        if let Some(remote) = &mut cloned.remote {
            if remote.password.is_some() {
                remote.password = Some("••••••••".to_string());
            }
        }
        cloned
    }

    /// Validate the configuration for completeness and file system accessibility.
    pub fn validate(&self, app_data_dir: &Path) -> ConnectionValidation {
        match self.mode {
            ConnectionMode::Local => {
                let resolved_path = self.resolve_local_db_path(app_data_dir);
                let resolved_url = self.resolve_local_db_url(app_data_dir);

                if let Some(path) = &resolved_path {
                    if let Some(parent) = path.parent() {
                        if !parent.exists() {
                            if let Err(e) = std::fs::create_dir_all(parent) {
                                return ConnectionValidation {
                                    is_valid: false,
                                    mode: "local".to_string(),
                                    message: format!("Cannot create directory for database: {e}"),
                                    resolved_url,
                                    resolved_path: Some(path.to_string_lossy().to_string()),
                                };
                            }
                        }
                    }
                }

                ConnectionValidation {
                    is_valid: true,
                    mode: "local".to_string(),
                    message: "Local SQLite connection is ready".to_string(),
                    resolved_url,
                    resolved_path: resolved_path.map(|p| p.to_string_lossy().to_string()),
                }
            }
            ConnectionMode::Remote => match &self.remote {
                Some(remote) if !remote.server_url.trim().is_empty() => {
                    let url = remote.server_url.trim();
                    if url.starts_with("http://") || url.starts_with("https://") {
                        ConnectionValidation {
                            is_valid: true,
                            mode: "remote".to_string(),
                            message: format!("Remote server configured: {}", url),
                            resolved_url: Some(url.to_string()),
                            resolved_path: None,
                        }
                    } else {
                        ConnectionValidation {
                            is_valid: false,
                            mode: "remote".to_string(),
                            message: "Server URL must start with http:// or https://".to_string(),
                            resolved_url: None,
                            resolved_path: None,
                        }
                    }
                }
                _ => ConnectionValidation {
                    is_valid: false,
                    mode: "remote".to_string(),
                    message: "Remote server URL is required".to_string(),
                    resolved_url: None,
                    resolved_path: None,
                },
            },
        }
    }
}

// ── Remote Health Check ─────────────────────────────────────────────────────

/// Check health of a remote Orbis Server.
pub async fn check_remote_health(server_url: &str) -> RemoteHealthResult {
    let health_url = format!("{}/api/v1/health", server_url.trim_end_matches('/'));
    let start = std::time::Instant::now();

    match reqwest::Client::new()
        .get(&health_url)
        .timeout(std::time::Duration::from_secs(10))
        .send()
        .await
    {
        Ok(response) => {
            let latency = start.elapsed().as_millis() as u64;
            if response.status().is_success() {
                match response.json::<serde_json::Value>().await {
                    Ok(body) => RemoteHealthResult {
                        reachable: true,
                        status: body
                            .get("status")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string()),
                        version: body
                            .get("version")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string()),
                        api_version: body
                            .get("api_version")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string()),
                        latency_ms: latency,
                        error: None,
                    },
                    Err(e) => RemoteHealthResult {
                        reachable: true,
                        status: Some("ok".to_string()),
                        version: None,
                        api_version: None,
                        latency_ms: latency,
                        error: Some(format!("Failed to parse health response: {e}")),
                    },
                }
            } else {
                RemoteHealthResult {
                    reachable: true,
                    status: Some(format!("HTTP {}", response.status())),
                    version: None,
                    api_version: None,
                    latency_ms: latency,
                    error: Some(format!("Server returned HTTP {}", response.status())),
                }
            }
        }
        Err(e) => {
            let latency = start.elapsed().as_millis() as u64;
            RemoteHealthResult {
                reachable: false,
                status: None,
                version: None,
                api_version: None,
                latency_ms: latency,
                error: Some(format!("Connection failed: {e}")),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_local_config() {
        let config = ConnectionConfig::default_local();
        assert_eq!(config.mode, ConnectionMode::Local);
        assert!(config.local.is_some());
        assert_eq!(config.local.unwrap().db_filename, "orbis.db");
    }

    #[test]
    fn test_resolve_local_db_path() {
        let config = ConnectionConfig::default_local();
        let app_data = PathBuf::from("/tmp/orbis_data");
        let path = config.resolve_local_db_path(&app_data).unwrap();
        assert_eq!(path, app_data.join("orbis.db"));
    }

    #[test]
    fn test_sanitize_password() {
        let mut config = ConnectionConfig::default_local();
        config.mode = ConnectionMode::Remote;
        config.remote = Some(RemoteConfig {
            server_url: "https://api.orbis.school".to_string(),
            host: Some("localhost".to_string()),
            port: Some(5432),
            database_name: Some("orbis_db".to_string()),
            username: Some("admin".to_string()),
            password: Some("super_secret".to_string()),
            ssl_mode: Some("require".to_string()),
            api_key: None,
        });

        let sanitized = config.sanitized();
        assert_eq!(
            sanitized.remote.as_ref().unwrap().password,
            Some("••••••••".to_string())
        );
    }

    #[test]
    fn test_validate_local() {
        let config = ConnectionConfig::default_local();
        let app_data = std::env::temp_dir().join("orbis_test_appdata");
        let val = config.validate(&app_data);
        assert!(val.is_valid);
        assert_eq!(val.mode, "local");
    }

    #[test]
    fn test_validate_remote_url() {
        let mut config = ConnectionConfig::default_local();
        config.mode = ConnectionMode::Remote;
        config.remote = Some(RemoteConfig {
            server_url: "invalid-url".to_string(),
            host: None,
            port: None,
            database_name: None,
            username: None,
            password: None,
            ssl_mode: None,
            api_key: None,
        });

        let app_data = std::env::temp_dir();
        let val = config.validate(&app_data);
        assert!(!val.is_valid);

        config.remote.as_mut().unwrap().server_url = "https://api.orbis.school".to_string();
        let val2 = config.validate(&app_data);
        assert!(val2.is_valid);
    }
}
