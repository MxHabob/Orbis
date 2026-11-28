//! Connection configuration — read/write/validate connection settings and database lifecycle.
//!
//! Supports local SQLite and Orbis API connection modes.
//! Configuration is persisted as JSON in AppData/connection.json.
//!
//! This module works before and during database connection lifecycle.

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tracing::info;
use url::Url;

fn default_true() -> bool {
    true
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

/// Connection mode — Local SQLite or Remote Orbis API Server.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ConnectionMode {
    Local,
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
pub struct DatabaseSettings {
    #[serde(default = "default_true")]
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
            auto_backup_enabled: true,
            auto_backup_interval_hours: 24,
            backup_dir: None,
            max_backup_files: 10,
            last_backup_timestamp: None,
            last_backup_path: None,
            integrity_check_on_startup: true,
        }
    }
}

impl DatabaseSettings {
    pub fn enforce_autopilot(&mut self) {
        self.auto_backup_enabled = true;
        self.integrity_check_on_startup = true;
        self.auto_backup_interval_hours = self.auto_backup_interval_hours.clamp(1, 168);
        self.max_backup_files = self.max_backup_files.clamp(1, 100);
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
                    Ok(mut config) => {
                        info!("Loaded connection config from {:?}", path);
                        let was_autopilot_disabled = !config.settings.auto_backup_enabled;
                        config.settings.enforce_autopilot();
                        if was_autopilot_disabled {
                            tracing::info!("Backup autopilot policy enforced at startup");
                        }
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
        }
    }

    /// Get the configured Orbis API server URL.
    pub fn remote_server_url(&self) -> Option<&str> {
        self.api.as_ref().map(|r| r.server_url.as_str())
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
            ConnectionMode::Api => {
                let result = self
                    .api
                    .as_ref()
                    .ok_or("Remote server URL is required")
                    .and_then(|remote| normalize_server_url(&remote.server_url));
                let (message, resolved_url) = match result {
                    Ok(url) => ("Orbis API server is configured", Some(url)),
                    Err(error) => (error, None),
                };
                ConnectionValidation {
                    is_valid: resolved_url.is_some(),
                    mode: "api".to_string(),
                    message: message.to_string(),
                    resolved_url,
                    resolved_path: None,
                }
            }
        }
    }
}

// ── Remote Health Check ─────────────────────────────────────────────────────

pub fn normalize_server_url(input: &str) -> Result<String, &'static str> {
    let url = Url::parse(input.trim()).map_err(|_| "Enter a valid Orbis server URL")?;
    let host = url.host().ok_or("The Orbis server URL requires a host")?;
    if !url.username().is_empty()
        || url.password().is_some()
        || url.query().is_some()
        || url.fragment().is_some()
    {
        return Err("The Orbis server URL must omit credentials, query parameters, and fragments");
    }
    let loopback = match host {
        url::Host::Domain(name) => name.eq_ignore_ascii_case("localhost"),
        url::Host::Ipv4(address) => address == std::net::Ipv4Addr::LOCALHOST,
        url::Host::Ipv6(address) => address.is_loopback(),
    };
    if url.scheme() != "https" && !(url.scheme() == "http" && loopback) {
        return Err("Use HTTPS for an Orbis server outside this computer");
    }

    let base = url.as_str().trim_end_matches('/');
    Ok(base.strip_suffix("/api/v1").unwrap_or(base).to_owned())
}

#[derive(Deserialize)]
struct ServerHealth {
    status: String,
    version: String,
    api_version: String,
    database_connected: bool,
}

/// Check whether the configured endpoint can serve Orbis requests.
pub async fn check_remote_health(server_url: &str) -> RemoteHealthResult {
    static CLIENT: std::sync::OnceLock<Result<reqwest::Client, reqwest::Error>> =
        std::sync::OnceLock::new();
    let start = std::time::Instant::now();
    let result = async {
        let base = normalize_server_url(server_url).map_err(str::to_owned)?;
        let client = CLIENT
            .get_or_init(|| {
                reqwest::Client::builder()
                    .redirect(reqwest::redirect::Policy::none())
                    .connect_timeout(std::time::Duration::from_secs(3))
                    .timeout(std::time::Duration::from_secs(5))
                    .build()
            })
            .as_ref()
            .map_err(|_| "Could not initialize the HTTP client".to_string())?;
        let response = client
            .get(format!("{base}/api/v1/health"))
            .send()
            .await
            .map_err(|error| {
                if error.is_timeout() {
                    "The server did not respond before the connection deadline"
                } else {
                    "Could not connect to the Orbis server. Check its address and TLS certificate"
                }
                .to_string()
            })?;
        if !response.status().is_success() {
            return Err(format!(
                "Orbis readiness check returned HTTP {}",
                response.status()
            ));
        }
        let health = response
            .json::<ServerHealth>()
            .await
            .map_err(|_| "This endpoint did not return an Orbis readiness response".to_string())?;
        if health.status != "ok" || !health.database_connected || health.api_version.is_empty() {
            return Err("The Orbis API is not ready to serve requests".to_string());
        }
        Ok(health)
    }
    .await;
    let latency_ms = start.elapsed().as_millis() as u64;
    match result {
        Ok(health) => RemoteHealthResult {
            reachable: true,
            status: Some(health.status),
            version: Some(health.version),
            api_version: Some(health.api_version),
            latency_ms,
            error: None,
        },
        Err(error) => RemoteHealthResult {
            reachable: false,
            status: None,
            version: None,
            api_version: None,
            latency_ms,
            error: Some(error),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_api_endpoint_and_requires_tls_for_network_connections() {
        assert_eq!(
            normalize_server_url(" https://school.example/orbis/api/v1/ ").unwrap(),
            "https://school.example/orbis",
        );
        for local in [
            "http://localhost:3000",
            "http://127.0.0.1:3000",
            "http://[::1]:3000",
        ] {
            assert_eq!(normalize_server_url(local).unwrap(), local);
        }
        for invalid in [
            "http://school.example",
            "http://192.168.1.5:3000",
            "https://user:secret@school.example",
            "https://school.example?token=secret",
            "https://school.example/#fragment",
        ] {
            assert!(normalize_server_url(invalid).is_err());
        }
    }

    #[test]
    fn test_default_local_config() {
        let config = ConnectionConfig::default_local();
        assert_eq!(config.mode, ConnectionMode::Local);
        assert!(config.local.is_some());
        assert_eq!(config.local.unwrap().db_filename, "orbis.db");
        assert!(config.settings.auto_backup_enabled);
    }

    #[test]
    fn test_backup_autopilot_policy_is_always_enforced() {
        let mut settings = DatabaseSettings {
            auto_backup_enabled: false,
            auto_backup_interval_hours: 0,
            backup_dir: None,
            max_backup_files: 500,
            last_backup_timestamp: None,
            last_backup_path: None,
            integrity_check_on_startup: false,
        };

        settings.enforce_autopilot();

        assert!(settings.auto_backup_enabled);
        assert!(settings.integrity_check_on_startup);
        assert_eq!(settings.auto_backup_interval_hours, 1);
        assert_eq!(settings.max_backup_files, 100);
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
