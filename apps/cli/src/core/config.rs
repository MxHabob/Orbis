use anyhow::{Context, Result};
use orbis_infra::config::{ServerConfig, generate_secret, persist_server_config};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use std::time::Duration;

pub fn http_client() -> &'static reqwest::Client {
    static CLIENT: OnceLock<reqwest::Client> = OnceLock::new();
    CLIENT.get_or_init(|| {
        reqwest::Client::builder()
            .timeout(Duration::from_secs(2))
            .build()
            .unwrap_or_else(|_| reqwest::Client::new())
    })
}

pub const DEFAULT_API_IMAGE: &str = "ghcr.io/mxhabob/orbis-api:1.2.6";
pub const DEFAULT_POSTGRES_IMAGE: &str = "postgres:16-alpine";
pub const DEFAULT_API_PORT: u16 = 3000;

/// Supported deployment targets (extensible for future platforms).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeployTarget {
    Docker,
    Native,
}

impl DeployTarget {
    pub fn label(self) -> &'static str {
        match self {
            Self::Docker => "Docker Container",
            Self::Native => "Native Host (Windows / Local Binary)",
        }
    }
}

/// Supported database engines (extensible for future providers).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DbProvider {
    DockerPostgres,
    ExternalPostgres,
    Sqlite,
}

impl DbProvider {
    pub fn label(self) -> &'static str {
        match self {
            Self::DockerPostgres => "Managed PostgreSQL (Docker)",
            Self::ExternalPostgres => "External PostgreSQL",
            Self::Sqlite => "SQLite (Local File)",
        }
    }
}

/// Persistent CLI deployment profile so commands (`start`, `license`, `doctor`, `backup`)
/// know which target and database the user chose without re-asking every time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CliState {
    pub target: DeployTarget,
    pub db_provider: DbProvider,
    pub database_url: String,
    pub api_port: u16,
    pub api_image: String,
    pub container_name: String,
    pub postgres_container: String,
    pub docker_network: String,
}

impl Default for CliState {
    fn default() -> Self {
        Self {
            target: DeployTarget::Docker,
            db_provider: DbProvider::DockerPostgres,
            database_url: String::new(),
            api_port: DEFAULT_API_PORT,
            api_image: DEFAULT_API_IMAGE.to_string(),
            container_name: "orbis-api".to_string(),
            postgres_container: "orbis-postgres".to_string(),
            docker_network: "orbis-net".to_string(),
        }
    }
}

impl CliState {
    pub fn state_dir() -> PathBuf {
        if let Ok(dir) = std::env::var("ORBIS_CLI_DIR")
            && !dir.trim().is_empty()
        {
            return PathBuf::from(dir);
        }
        #[cfg(target_os = "windows")]
        if let Ok(appdata) = std::env::var("APPDATA") {
            return PathBuf::from(appdata).join("Orbis").join("cli");
        }
        #[cfg(not(target_os = "windows"))]
        if let Ok(home) = std::env::var("HOME") {
            return PathBuf::from(home).join(".config").join("orbis");
        }
        PathBuf::from(".orbis-cli")
    }

    pub fn state_file() -> PathBuf {
        Self::state_dir().join("cli_state.json")
    }

    pub fn load() -> Option<Self> {
        let path = Self::state_file();
        let data = fs::read_to_string(path).ok()?;
        serde_json::from_str(&data).ok()
    }

    pub fn save(&self) -> Result<()> {
        let dir = Self::state_dir();
        fs::create_dir_all(&dir)?;
        let path = Self::state_file();
        let json = serde_json::to_string_pretty(self)?;
        fs::write(&path, json).with_context(|| format!("Failed to save {}", path.display()))?;
        Ok(())
    }

    /// Host-reachable database URL for running `Migrator::up` / `doctor` from the CLI.
    pub fn host_database_url(&self) -> String {
        if self.target == DeployTarget::Docker {
            self.database_url
                .replace("host.docker.internal", "127.0.0.1")
                .replace(&format!("@{}:", self.postgres_container), "@127.0.0.1:")
        } else {
            self.database_url.clone()
        }
    }

    /// Container-reachable database URL written into `server.json`.
    pub fn container_database_url(&self) -> String {
        if self.target != DeployTarget::Docker {
            return self.database_url.clone();
        }
        if self.db_provider == DbProvider::DockerPostgres {
            return rewrite_pg_host(&self.database_url, &self.postgres_container);
        }
        rewrite_localhost_to_docker_host(&self.database_url)
    }

    pub fn api_base_url(&self) -> String {
        format!("http://127.0.0.1:{}", self.api_port)
    }
}

/// Ensures `server.json` exists with non-empty `jwt_secret` and `mfa_encryption_key`.
pub fn ensure_server_config_file(path: &Path, database_url: &str) -> Result<ServerConfig> {
    let existing: Option<ServerConfig> = fs::read_to_string(path)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok());

    let jwt_secret = existing
        .as_ref()
        .and_then(|c| c.jwt_secret.as_deref())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_owned)
        .unwrap_or_else(generate_secret);

    let mfa_encryption_key = existing
        .as_ref()
        .and_then(|c| c.mfa_encryption_key.as_deref())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_owned)
        .unwrap_or_else(generate_secret);

    let cfg = ServerConfig {
        database_url: database_url.to_string(),
        jwt_secret: Some(jwt_secret),
        mfa_encryption_key: Some(mfa_encryption_key),
    };
    persist_server_config(path, &cfg).map_err(anyhow::Error::msg)?;
    Ok(cfg)
}

pub fn rewrite_localhost_to_docker_host(url: &str) -> String {
    url.replace("@localhost:", "@host.docker.internal:")
        .replace("@localhost/", "@host.docker.internal/")
        .replace("@127.0.0.1:", "@host.docker.internal:")
        .replace("@127.0.0.1/", "@host.docker.internal/")
}

pub fn rewrite_pg_host(url: &str, new_host: &str) -> String {
    if let Some(at_idx) = url.find('@') {
        let prefix = &url[..=at_idx];
        let rest = &url[at_idx + 1..];
        if let Some(end_idx) = rest.find([':', '/']) {
            return format!("{prefix}{new_host}{}", &rest[end_idx..]);
        }
    }
    url.to_string()
}

pub fn default_sqlite_url() -> String {
    let path = CliState::state_dir().join("orbis.db");
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    format!(
        "sqlite://{}?mode=rwc",
        path.to_string_lossy().replace('\\', "/")
    )
}

pub fn native_license_dir() -> PathBuf {
    if let Ok(dir) = std::env::var("ORBIS_LICENSE_DATA_DIR")
        && !dir.trim().is_empty()
    {
        return PathBuf::from(dir);
    }
    #[cfg(target_os = "windows")]
    if let Ok(program_data) = std::env::var("PROGRAMDATA")
        && !program_data.trim().is_empty()
    {
        return PathBuf::from(program_data).join("Orbis").join("license");
    }
    #[cfg(target_os = "linux")]
    {
        return PathBuf::from("/var/lib/orbis/license");
    }
    #[allow(unreachable_code)]
    CliState::state_dir().join("license")
}

pub fn find_server_binary() -> Option<PathBuf> {
    let bin = if cfg!(windows) {
        "orbis-server.exe"
    } else {
        "orbis-server"
    };
    let mut candidates = Vec::new();

    #[cfg(target_os = "windows")]
    if let Ok(pf) = std::env::var("ProgramFiles") {
        candidates.push(PathBuf::from(pf).join("Orbis").join("API").join(bin));
    }
    #[cfg(target_os = "linux")]
    candidates.push(PathBuf::from("/opt/orbis/bin").join(bin));

    candidates.extend([
        PathBuf::from(bin),
        PathBuf::from("apps/api-rust/target/release").join(bin),
        PathBuf::from("apps/api-rust/target/debug").join(bin),
        PathBuf::from("../api-rust/target/release").join(bin),
        PathBuf::from("../api-rust/target/debug").join(bin),
    ]);
    candidates.into_iter().find(|p| p.exists())
}

pub fn get_app_executable_paths() -> Vec<PathBuf> {
    let app_name = if cfg!(windows) {
        "Orbis-Svelte.exe"
    } else {
        "Orbis-Svelte"
    };
    let product_name = if cfg!(windows) { "Orbis.exe" } else { "Orbis" };
    let mut paths = vec![PathBuf::from(app_name), PathBuf::from(product_name)];

    #[cfg(target_os = "windows")]
    if let Ok(local_app_data) = std::env::var("LOCALAPPDATA") {
        let base = Path::new(&local_app_data);
        paths.push(base.join("Programs").join("Orbis").join(product_name));
        paths.push(base.join("Orbis").join(product_name));
    }

    paths.extend([
        PathBuf::from("apps/desktop/src-tauri/target/release").join(app_name),
        PathBuf::from("apps/desktop/src-tauri/target/debug").join(app_name),
    ]);
    paths
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rewrites_localhost_for_docker_external_postgres() {
        let mut state = CliState::default();
        state.target = DeployTarget::Docker;
        state.db_provider = DbProvider::ExternalPostgres;
        state.database_url = "postgres://orbis:pass@localhost:5432/orbis".to_string();
        assert_eq!(
            state.container_database_url(),
            "postgres://orbis:pass@host.docker.internal:5432/orbis"
        );
        assert_eq!(
            state.host_database_url(),
            "postgres://orbis:pass@localhost:5432/orbis"
        );
    }

    #[test]
    fn rewrites_host_for_managed_docker_postgres() {
        let mut state = CliState::default();
        state.target = DeployTarget::Docker;
        state.db_provider = DbProvider::DockerPostgres;
        state.database_url = "postgres://orbis:pass@127.0.0.1:5432/orbis".to_string();
        assert_eq!(
            state.container_database_url(),
            "postgres://orbis:pass@orbis-postgres:5432/orbis"
        );
        assert_eq!(
            state.host_database_url(),
            "postgres://orbis:pass@127.0.0.1:5432/orbis"
        );
    }
}
