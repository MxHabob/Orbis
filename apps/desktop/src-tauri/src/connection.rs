//! Connection configuration — read/write/validate connection settings.
//!
//! Supports both Local (SQLite) and Remote (Orbis Server) connection modes.
//! Configuration is persisted as JSON in AppData/connection.json.
//!
//! This module does NOT depend on AppState — it works before the database is connected.

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tracing::info;

// ── Connection Configuration Types ──────────────────────────────────────────

/// Connection mode — Local SQLite or Remote Orbis Server.
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalConfig {
    /// Database file name (default: "orbis.db").
    pub db_filename: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteConfig {
    /// Server base URL (e.g. "https://api.orbis.school").
    pub server_url: String,
    /// API key or token (optional — user may authenticate later).
    pub api_key: Option<String>,
}

/// Connection validation result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionValidation {
    pub is_valid: bool,
    pub mode: String,
    pub message: String,
    /// Resolved database URL for Local mode.
    pub resolved_url: Option<String>,
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
            }),
            remote: None,
        }
    }

    /// Config file path within the app data directory.
    fn config_path(app_data_dir: &Path) -> PathBuf {
        app_data_dir.join("connection.json")
    }

    /// Load configuration from disk. Returns default if not found.
    pub fn load(app_data_dir: &Path) -> Self {
        let path = Self::config_path(app_data_dir);
        if path.exists() {
            match std::fs::read_to_string(&path) {
                Ok(contents) => {
                    match serde_json::from_str::<ConnectionConfig>(&contents) {
                        Ok(config) => {
                            info!("Loaded connection config from {:?}", path);
                            return config;
                        }
                        Err(e) => {
                            tracing::warn!("Failed to parse connection config: {e}, using default");
                        }
                    }
                }
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
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize connection config: {e}"))?;
        std::fs::write(&path, json)
            .map_err(|e| format!("Failed to write connection config to {:?}: {e}", path))?;
        info!("Saved connection config to {:?}", path);
        Ok(())
    }

    /// Resolve the full database URL for Local mode.
    pub fn resolve_local_db_url(&self, app_data_dir: &Path) -> Option<String> {
        if self.mode != ConnectionMode::Local {
            return None;
        }
        let filename = self.local.as_ref()
            .map(|l| l.db_filename.as_str())
            .unwrap_or("orbis.db");
        let db_path = app_data_dir.join(filename);
        Some(format!("sqlite://{}?mode=rwc", db_path.display()))
    }

    /// Get the remote server URL.
    pub fn remote_server_url(&self) -> Option<&str> {
        self.remote.as_ref().map(|r| r.server_url.as_str())
    }

    /// Validate the configuration for completeness.
    pub fn validate(&self, app_data_dir: &Path) -> ConnectionValidation {
        match self.mode {
            ConnectionMode::Local => {
                let resolved = self.resolve_local_db_url(app_data_dir);
                ConnectionValidation {
                    is_valid: true,
                    mode: "local".to_string(),
                    message: "Local SQLite connection is ready".to_string(),
                    resolved_url: resolved,
                }
            }
            ConnectionMode::Remote => {
                match &self.remote {
                    Some(remote) if !remote.server_url.trim().is_empty() => {
                        // Validate URL format
                        if remote.server_url.starts_with("http://") || remote.server_url.starts_with("https://") {
                            ConnectionValidation {
                                is_valid: true,
                                mode: "remote".to_string(),
                                message: format!("Remote server configured: {}", remote.server_url),
                                resolved_url: Some(remote.server_url.clone()),
                            }
                        } else {
                            ConnectionValidation {
                                is_valid: false,
                                mode: "remote".to_string(),
                                message: "Server URL must start with http:// or https://".to_string(),
                                resolved_url: None,
                            }
                        }
                    }
                    _ => ConnectionValidation {
                        is_valid: false,
                        mode: "remote".to_string(),
                        message: "Remote server URL is required".to_string(),
                        resolved_url: None,
                    },
                }
            }
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
                        status: body.get("status").and_then(|v| v.as_str()).map(|s| s.to_string()),
                        version: body.get("version").and_then(|v| v.as_str()).map(|s| s.to_string()),
                        api_version: body.get("api_version").and_then(|v| v.as_str()).map(|s| s.to_string()),
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
