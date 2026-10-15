//! Configuration and Environment Resolution for Orbis Infrastructure.
//!
//! Resolves the API database configuration and the Desktop database in development.

use serde::Deserialize;
use std::path::{Path, PathBuf};

const DESKTOP_APP_IDENTIFIER: &str = "com.mulverse.orbis";
const DESKTOP_CONNECTION_FILE: &str = "connection.json";

/// Resolve the API database URL. During local development, use the Desktop's
/// configured database so both applications operate on the same data.
pub fn resolve_database_url() -> Result<String, String> {
    if std::env::var("APP_ENV").as_deref() == Ok("development") {
        if let Ok(url) = std::env::var("ORBIS_API_DATABASE_URL") {
            return validate_database_url(url, "ORBIS_API_DATABASE_URL");
        }
        return resolve_desktop_database_url(&desktop_app_data_dir()?);
    }

    let url = std::env::var("DATABASE_URL")
        .map_err(|_| "DATABASE_URL must be configured for the Orbis API server".to_string())?;
    validate_database_url(url, "DATABASE_URL")
}

fn validate_database_url(url: String, variable: &str) -> Result<String, String> {
    let url = url.trim();
    if url.is_empty() {
        return Err(format!(
            "{variable} cannot be empty for the Orbis API server"
        ));
    }
    Ok(url.to_string())
}

fn desktop_app_data_dir() -> Result<PathBuf, String> {
    #[cfg(target_os = "windows")]
    {
        let app_data = std::env::var_os("APPDATA")
            .ok_or_else(|| "APPDATA is not set; cannot locate the Desktop database".to_string())?;
        return Ok(PathBuf::from(app_data).join(DESKTOP_APP_IDENTIFIER));
    }

    #[cfg(target_os = "macos")]
    {
        let home = std::env::var_os("HOME")
            .ok_or_else(|| "HOME is not set; cannot locate the Desktop database".to_string())?;
        return Ok(PathBuf::from(home)
            .join("Library")
            .join("Application Support")
            .join(DESKTOP_APP_IDENTIFIER));
    }

    #[cfg(all(unix, not(target_os = "macos")))]
    {
        let data_dir = std::env::var_os("XDG_DATA_HOME")
            .map(PathBuf::from)
            .or_else(|| {
                std::env::var_os("HOME")
                    .map(PathBuf::from)
                    .map(|home| home.join(".local").join("share"))
            })
            .ok_or_else(|| {
                "Neither XDG_DATA_HOME nor HOME is set; cannot locate the Desktop database"
                    .to_string()
            })?;
        return Ok(data_dir.join(DESKTOP_APP_IDENTIFIER));
    }

    #[allow(unreachable_code)]
    Err("Unsupported operating system; cannot locate the Desktop database".to_string())
}

fn resolve_desktop_database_url(app_data_dir: &Path) -> Result<String, String> {
    let config_path = app_data_dir.join(DESKTOP_CONNECTION_FILE);
    let config = if config_path.exists() {
        let contents = std::fs::read_to_string(&config_path).map_err(|error| {
            format!(
                "Unable to read Desktop connection config at {}: {error}",
                config_path.display()
            )
        })?;
        serde_json::from_str::<DesktopConnectionConfig>(&contents).map_err(|error| {
            format!(
                "Unable to parse Desktop connection config at {}: {error}",
                config_path.display()
            )
        })?
    } else {
        DesktopConnectionConfig::default_local()
    };

    match config.mode {
        DesktopConnectionMode::Local => {
            let local = config.local.unwrap_or_default();
            let custom_path = local.custom_path.as_deref().map(str::trim).unwrap_or_default();
            let filename = local.db_filename.trim();
            let filename = if filename.is_empty() {
                "orbis.db"
            } else {
                filename
            };
            let db_path = if custom_path.is_empty() {
                app_data_dir.join(filename)
            } else {
                let custom_path = PathBuf::from(custom_path);
                if custom_path.is_absolute() {
                    custom_path
                } else {
                    app_data_dir.join(custom_path)
                }
            };

            if let Some(parent) = db_path.parent() {
                std::fs::create_dir_all(parent).map_err(|error| {
                    format!(
                        "Unable to create Desktop database directory {}: {error}",
                        parent.display()
                    )
                })?;
            }

            let path = db_path.to_string_lossy().replace('\\', "/");
            Ok(format!("sqlite://{path}?mode=rwc"))
        }
        DesktopConnectionMode::Direct => {
            let direct = config
                .direct
                .ok_or_else(|| "Desktop direct database settings are missing".to_string())?;
            direct.database_url()
        }
        DesktopConnectionMode::Api => Err(
            "Desktop is configured for API mode and has no local database; set ORBIS_API_DATABASE_URL to run the API server"
                .to_string(),
        ),
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
enum DesktopConnectionMode {
    Local,
    Direct,
    #[serde(alias = "remote")]
    Api,
}

#[derive(Deserialize)]
struct DesktopConnectionConfig {
    mode: DesktopConnectionMode,
    #[serde(default)]
    local: Option<DesktopLocalConfig>,
    #[serde(default)]
    direct: Option<DesktopDirectConfig>,
}

impl DesktopConnectionConfig {
    fn default_local() -> Self {
        Self {
            mode: DesktopConnectionMode::Local,
            local: Some(DesktopLocalConfig::default()),
            direct: None,
        }
    }
}

#[derive(Deserialize)]
struct DesktopLocalConfig {
    #[serde(default = "default_desktop_db_filename")]
    db_filename: String,
    #[serde(default)]
    custom_path: Option<String>,
}

impl Default for DesktopLocalConfig {
    fn default() -> Self {
        Self {
            db_filename: default_desktop_db_filename(),
            custom_path: None,
        }
    }
}

fn default_desktop_db_filename() -> String {
    "orbis.db".to_string()
}

#[derive(Deserialize)]
struct DesktopDirectConfig {
    host: String,
    port: u16,
    database_name: String,
    username: String,
    #[serde(default)]
    ssl_mode: Option<String>,
    credential_id: String,
}

impl DesktopDirectConfig {
    fn database_url(self) -> Result<String, String> {
        use keyring::{Entry, Error as KeyringError};
        use url::Url;

        let password = Entry::new("com.mulverse.orbis.database", &self.credential_id)
            .map_err(|error| {
                format!("Unable to access the Desktop database credential store: {error}")
            })?
            .get_password()
            .map_err(|error| match error {
                KeyringError::NoEntry => {
                    "Desktop database password is not saved in the credential store".to_string()
                }
                other => format!(
                    "Unable to read the Desktop database password from the credential store: {other}"
                ),
            })?;

        let mut url = Url::parse("postgres://localhost")
            .map_err(|error| format!("Unable to construct PostgreSQL connection URL: {error}"))?;
        url.set_host(Some(&self.host))
            .map_err(|_| "Invalid PostgreSQL host".to_string())?;
        url.set_port(Some(self.port))
            .map_err(|_| "Invalid PostgreSQL port".to_string())?;
        url.set_username(&self.username)
            .map_err(|_| "Invalid PostgreSQL username".to_string())?;
        url.set_password(Some(&password))
            .map_err(|_| "Invalid PostgreSQL password".to_string())?;
        url.set_path(&format!("/{}", self.database_name));
        if let Some(ssl_mode) = self.ssl_mode {
            url.query_pairs_mut().append_pair("sslmode", &ssl_mode);
        }
        Ok(url.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn development_uses_the_desktop_default_database() {
        let app_data_dir = std::env::temp_dir().join("orbis-api-config-default");
        let url = resolve_desktop_database_url(&app_data_dir).unwrap();
        assert_eq!(
            url,
            format!(
                "sqlite://{}/orbis.db?mode=rwc",
                app_data_dir.to_string_lossy().replace('\\', "/")
            )
        );
        std::fs::remove_dir_all(app_data_dir).unwrap();
    }

    #[test]
    fn development_uses_the_desktop_custom_database_path() {
        let app_data_dir = std::env::temp_dir().join("orbis-api-config-custom");
        std::fs::create_dir_all(&app_data_dir).unwrap();
        std::fs::write(
            app_data_dir.join(DESKTOP_CONNECTION_FILE),
            r#"{"mode":"local","local":{"db_filename":"other.db","custom_path":"data/custom.db"}}"#,
        )
        .unwrap();

        let url = resolve_desktop_database_url(&app_data_dir).unwrap();
        assert_eq!(
            url,
            format!(
                "sqlite://{}/data/custom.db?mode=rwc",
                app_data_dir.to_string_lossy().replace('\\', "/")
            )
        );
        std::fs::remove_dir_all(app_data_dir).unwrap();
    }

    #[test]
    fn development_requires_an_explicit_database_in_desktop_api_mode() {
        let app_data_dir = std::env::temp_dir().join("orbis-api-config-api-mode");
        std::fs::create_dir_all(&app_data_dir).unwrap();
        std::fs::write(
            app_data_dir.join(DESKTOP_CONNECTION_FILE),
            r#"{"mode":"api","api":{"server_url":"http://localhost:3000"}}"#,
        )
        .unwrap();

        let error = resolve_desktop_database_url(&app_data_dir).unwrap_err();
        assert!(error.contains("set ORBIS_API_DATABASE_URL"));
        std::fs::remove_dir_all(app_data_dir).unwrap();
    }
}
