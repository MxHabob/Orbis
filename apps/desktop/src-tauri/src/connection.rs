//! Connection configuration — read/write/validate connection settings and database lifecycle.
//!
//! Supports local SQLite, direct PostgreSQL, and Orbis API connection modes.
//! Configuration is persisted as JSON in AppData/connection.json.
//!
//! This module works before and during database connection lifecycle.

use keyring::{Entry, Error as KeyringError};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tracing::info;
use url::Url;

const CONNECTION_SECRET_SERVICE: &str = "com.mulverse.orbis.database";

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
    Direct,
    #[serde(alias = "remote")]
    Api,
}

/// Persisted connection configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionConfig {
    pub mode: ConnectionMode,
    /// Local: SQLite database file path (relative to AppData or absolute).
    pub local: Option<LocalConfig>,
    /// API server URL. Database credentials are never stored in this config.
    #[serde(rename = "api", alias = "remote")]
    pub api: Option<ApiConfig>,
    /// Direct database connection settings.
    #[serde(default)]
    pub direct: Option<DirectConfig>,
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
pub struct ApiConfig {
    /// Orbis API base URL (e.g. "https://api.orbis.school").
    pub server_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DirectDatabaseEngine {
    Postgres,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectConfig {
    pub engine: DirectDatabaseEngine,
    pub host: String,
    pub port: u16,
    pub database_name: String,
    pub username: String,
    pub ssl_mode: Option<String>,
    pub credential_id: String,
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
            api: None,
            direct: None,
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
                        if let Ok(normalized) = serde_json::to_string_pretty(&config) {
                            if normalized != contents {
                                if let Err(error) = std::fs::write(&path, normalized) {
                                    tracing::warn!(
                                        "Could not remove legacy connection fields from config: {error}"
                                    );
                                }
                            }
                        }
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

    /// Resolve the active database URL. API mode intentionally has no local database URL.
    pub fn resolve_database_url(&self, app_data_dir: &Path) -> Result<Option<String>, String> {
        match self.mode {
            ConnectionMode::Local => Ok(self.resolve_local_db_url(app_data_dir)),
            ConnectionMode::Api => Ok(None),
            ConnectionMode::Direct => {
                let direct = self
                    .direct
                    .as_ref()
                    .ok_or_else(|| "Direct database settings are missing".to_string())?;
                let password = Entry::new(CONNECTION_SECRET_SERVICE, &direct.credential_id)
                    .map_err(|e| format!("Unable to access the operating system credential store: {e}"))?
                    .get_password()
                    .map_err(|e| match e {
                        KeyringError::NoEntry => "Database password is not saved in the operating system credential store".to_string(),
                        other => format!("Unable to read database password from the operating system credential store: {other}"),
                    })?;

                let mut url = Url::parse("postgres://localhost")
                    .map_err(|e| format!("Unable to construct PostgreSQL connection URL: {e}"))?;
                url.set_host(Some(&direct.host))
                    .map_err(|_| "Invalid PostgreSQL host".to_string())?;
                url.set_port(Some(direct.port))
                    .map_err(|_| "Invalid PostgreSQL port".to_string())?;
                url.set_username(&direct.username)
                    .map_err(|_| "Invalid PostgreSQL username".to_string())?;
                url.set_password(Some(&password))
                    .map_err(|_| "Invalid PostgreSQL password".to_string())?;
                url.set_path(&format!("/{}", direct.database_name));
                if let Some(ssl_mode) = &direct.ssl_mode {
                    url.query_pairs_mut().append_pair("sslmode", ssl_mode);
                }
                Ok(Some(url.to_string()))
            }
        }
    }

    /// Get the configured Orbis API server URL.
    pub fn remote_server_url(&self) -> Option<&str> {
        self.api.as_ref().map(|r| r.server_url.as_str())
    }

    /// Store a direct-connection password in the operating system credential store.
    pub fn save_direct_password(credential_id: &str, password: &str) -> Result<(), String> {
        let entry = Entry::new(CONNECTION_SECRET_SERVICE, credential_id)
            .map_err(|e| format!("Unable to access the operating system credential store: {e}"))?;
        entry.set_password(password).map_err(|e| {
            format!(
                "Unable to save database password in the operating system credential store: {e}"
            )
        })
    }

    /// Sanitize configuration before returning it to the UI.
    pub fn sanitized(&self) -> Self {
        self.clone()
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
            ConnectionMode::Api => match &self.api {
                Some(remote) if !remote.server_url.trim().is_empty() => {
                    let url = remote.server_url.trim();
                    if url.starts_with("http://") || url.starts_with("https://") {
                        ConnectionValidation {
                            is_valid: true,
                            mode: "api".to_string(),
                            message: "Orbis API server is configured".to_string(),
                            resolved_url: Some(url.to_string()),
                            resolved_path: None,
                        }
                    } else {
                        ConnectionValidation {
                            is_valid: false,
                            mode: "api".to_string(),
                            message: "Server URL must start with http:// or https://".to_string(),
                            resolved_url: None,
                            resolved_path: None,
                        }
                    }
                }
                _ => ConnectionValidation {
                    is_valid: false,
                    mode: "api".to_string(),
                    message: "Remote server URL is required".to_string(),
                    resolved_url: None,
                    resolved_path: None,
                },
            },
            ConnectionMode::Direct => match &self.direct {
                Some(direct)
                    if !direct.host.trim().is_empty()
                        && !direct.database_name.trim().is_empty()
                        && !direct.username.trim().is_empty()
                        && direct.port > 0
                        && !direct.credential_id.trim().is_empty() =>
                {
                    ConnectionValidation {
                        is_valid: true,
                        mode: "direct".to_string(),
                        message: "Direct PostgreSQL connection is configured".to_string(),
                        resolved_url: None,
                        resolved_path: None,
                    }
                }
                _ => ConnectionValidation {
                    is_valid: false,
                    mode: "direct".to_string(),
                    message: "Host, port, database, username, and saved credentials are required"
                        .to_string(),
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
    fn test_api_config_does_not_contain_database_credentials() {
        let mut config = ConnectionConfig::default_local();
        config.mode = ConnectionMode::Api;
        config.api = Some(ApiConfig {
            server_url: "https://api.orbis.school".to_string(),
        });

        let sanitized = config.sanitized();
        let serialized = serde_json::to_string(&sanitized).unwrap();
        assert!(!serialized.contains("password"));
        assert!(!serialized.contains("username"));
        assert!(serialized.contains("https://api.orbis.school"));
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
        config.mode = ConnectionMode::Api;
        config.api = Some(ApiConfig {
            server_url: "invalid-url".to_string(),
        });

        let app_data = std::env::temp_dir();
        let val = config.validate(&app_data);
        assert!(!val.is_valid);

        config.api.as_mut().unwrap().server_url = "https://api.orbis.school".to_string();
        let val2 = config.validate(&app_data);
        assert!(val2.is_valid);
    }

    #[test]
    fn test_legacy_remote_config_migrates_to_api_without_database_credentials() {
        let legacy = r#"{
          "mode":"remote",
          "remote":{
            "server_url":"https://school.example",
            "host":"db.internal",
            "username":"desktop_user",
            "password":"should-not-be-retained"
          }
        }"#;
        let config: ConnectionConfig = serde_json::from_str(legacy).unwrap();
        assert_eq!(config.mode, ConnectionMode::Api);
        assert_eq!(config.remote_server_url(), Some("https://school.example"));
        let serialized = serde_json::to_string(&config).unwrap();
        assert!(!serialized.contains("should-not-be-retained"));
        assert!(!serialized.contains("desktop_user"));
        assert!(serialized.contains("\"api\""));
    }
}
