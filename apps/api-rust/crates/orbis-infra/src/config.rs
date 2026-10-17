//! Configuration and Environment Resolution for Orbis Infrastructure.
//!
//! Resolves the API database configuration and the Desktop database in development.

use serde::Deserialize;
use std::path::{Path, PathBuf};

const SERVER_CONFIG_FILE: &str = "server.json";
const SERVER_CONFIG_ENV: &str = "ORBIS_SERVER_CONFIG_FILE";

/// Resolve the API database URL from the protected server-side configuration file.
pub fn resolve_database_url() -> Result<String, String> {
    let path = match std::env::var_os(SERVER_CONFIG_ENV) {
        Some(path) if !path.is_empty() => PathBuf::from(path),
        _ => default_server_config_path()?,
    };
    resolve_database_url_from_config(&path)
}

#[derive(Deserialize)]
struct ServerConfig {
    database_url: String,
}

fn resolve_database_url_from_config(path: &Path) -> Result<String, String> {
    let contents = std::fs::read_to_string(path).map_err(|error| {
        format!(
            "Unable to read Orbis server configuration at {}: {error}",
            path.display()
        )
    })?;
    let config: ServerConfig = serde_json::from_str(&contents).map_err(|error| {
        format!(
            "Unable to parse Orbis server configuration at {}: {error}",
            path.display()
        )
    })?;
    let database_url = config.database_url.trim();
    if database_url.is_empty() {
        return Err(format!(
            "database_url cannot be empty in Orbis server configuration at {}",
            path.display()
        ));
    }
    Ok(database_url.to_string())
}

fn default_server_config_path() -> Result<PathBuf, String> {
    #[cfg(target_os = "windows")]
    {
        let program_data = std::env::var_os("PROGRAMDATA")
            .ok_or_else(|| "PROGRAMDATA is not set; cannot locate Orbis server configuration".to_string())?;
        return Ok(PathBuf::from(program_data).join("Orbis").join(SERVER_CONFIG_FILE));
    }

    #[cfg(target_os = "macos")]
    {
        return Ok(PathBuf::from("/Library/Application Support/Orbis").join(SERVER_CONFIG_FILE));
    }

    #[cfg(target_os = "linux")]
    {
        return Ok(PathBuf::from("/etc/orbis").join(SERVER_CONFIG_FILE));
    }

    #[allow(unreachable_code)]
    Err("Unsupported operating system; set ORBIS_SERVER_CONFIG_FILE to the server config path".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_database_url_from_server_config_file() {
        let config_path = std::env::temp_dir().join(format!(
            "orbis-server-config-{}.json",
            uuid::Uuid::new_v4()
        ));
        std::fs::write(&config_path, r#"{"database_url":"postgres://localhost/orbis"}"#)
            .unwrap();
        assert_eq!(
            resolve_database_url_from_config(&config_path).unwrap(),
            "postgres://localhost/orbis"
        );
        std::fs::remove_file(config_path).unwrap();
    }

    #[test]
    fn rejects_empty_database_url_in_server_config_file() {
        let config_path = std::env::temp_dir().join(format!(
            "orbis-server-config-{}.json",
            uuid::Uuid::new_v4()
        ));
        std::fs::write(&config_path, r#"{"database_url":"  "}"#).unwrap();
        let error = resolve_database_url_from_config(&config_path).unwrap_err();
        assert!(error.contains("database_url cannot be empty"));
        std::fs::remove_file(config_path).unwrap();
    }
}
