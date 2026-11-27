//! Configuration and Environment Resolution for Orbis Infrastructure.
//!
//! Resolves the API database configuration and the Desktop database in development.

use rand::RngCore;
use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

const SERVER_CONFIG_FILE: &str = "server.json";
const SERVER_CONFIG_ENV: &str = "ORBIS_SERVER_CONFIG_FILE";
const EMPTY_SERVER_CONFIG: &str = "{\n  \"database_url\": \"\"\n}\n";
const JWT_SECRET_ENV: &str = "JWT_SECRET";
const MFA_ENCRYPTION_KEY_ENV: &str = "MFA_ENCRYPTION_KEY";

pub struct ServerSecrets {
    pub jwt_secret: String,
    pub mfa_encryption_key: String,
}

/// Resolve the API database URL from the protected server-side configuration file.
pub fn resolve_database_url() -> Result<String, String> {
    let path = server_config_path()?;
    resolve_database_url_from_config(&path)
}

/// Resolve explicit environment overrides or create durable per-installation keys.
pub fn resolve_server_secrets() -> Result<ServerSecrets, String> {
    let path = server_config_path()?;
    let jwt_override = std::env::var(JWT_SECRET_ENV).ok();
    let mfa_override = std::env::var(MFA_ENCRYPTION_KEY_ENV).ok();
    resolve_server_secrets_from_config(&path, jwt_override, mfa_override)
}

pub fn server_config_path() -> Result<PathBuf, String> {
    match std::env::var_os(SERVER_CONFIG_ENV) {
        Some(path) if !path.is_empty() => Ok(PathBuf::from(path)),
        _ => default_server_config_path(),
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServerConfig {
    pub database_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jwt_secret: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mfa_encryption_key: Option<String>,
}

fn resolve_database_url_from_config(path: &Path) -> Result<String, String> {
    let contents = match std::fs::read_to_string(path) {
        Ok(contents) => contents,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            if let Some(parent) = path
                .parent()
                .filter(|parent| !parent.as_os_str().is_empty())
            {
                std::fs::create_dir_all(parent).map_err(|error| {
                    format!(
                        "Unable to create Orbis server configuration directory at {}: {error}",
                        parent.display()
                    )
                })?;
            }

            let mut options = OpenOptions::new();
            options.write(true).create_new(true);
            #[cfg(unix)]
            {
                use std::os::unix::fs::OpenOptionsExt;
                options.mode(0o640);
            }

            match options.open(path) {
                Ok(mut file) => {
                    file.write_all(EMPTY_SERVER_CONFIG.as_bytes())
                        .map_err(|error| {
                            format!(
                                "Unable to initialize Orbis server configuration at {}: {error}",
                                path.display()
                            )
                        })?;
                    return Err(format!(
                        "Created Orbis server configuration at {}. Set database_url to the customer PostgreSQL connection string, then restart the API.",
                        path.display()
                    ));
                }
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
                    std::fs::read_to_string(path).map_err(|error| {
                        format!(
                            "Unable to read Orbis server configuration at {}: {error}",
                            path.display()
                        )
                    })?
                }
                Err(error) => {
                    return Err(format!(
                        "Unable to create Orbis server configuration at {}: {error}",
                        path.display()
                    ));
                }
            }
        }
        Err(error) => {
            return Err(format!(
                "Unable to read Orbis server configuration at {}: {error}",
                path.display()
            ));
        }
    };
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

fn resolve_server_secrets_from_config(
    path: &Path,
    jwt_override: Option<String>,
    mfa_override: Option<String>,
) -> Result<ServerSecrets, String> {
    let contents = fs::read_to_string(path).map_err(|error| {
        format!(
            "Unable to read Orbis server configuration at {}: {error}",
            path.display()
        )
    })?;
    let mut config: ServerConfig = serde_json::from_str(&contents).map_err(|error| {
        format!(
            "Unable to parse Orbis server configuration at {}: {error}",
            path.display()
        )
    })?;

    let jwt_override = non_empty(jwt_override);
    let mfa_override = non_empty(mfa_override);
    let mut changed = false;
    let jwt_secret = match jwt_override.as_ref() {
        Some(secret) => secret.clone(),
        None => match config
            .jwt_secret
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty())
        {
            Some(secret) => secret.to_owned(),
            None => {
                changed = true;
                generate_secret()
            }
        },
    };
    let mfa_encryption_key = match mfa_override.as_ref() {
        Some(secret) => secret.clone(),
        None => match config
            .mfa_encryption_key
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty())
        {
            Some(secret) => secret.to_owned(),
            None => {
                changed = true;
                generate_secret()
            }
        },
    };

    if changed {
        if jwt_override.is_none() {
            config.jwt_secret = Some(jwt_secret.clone());
        }
        if mfa_override.is_none() {
            config.mfa_encryption_key = Some(mfa_encryption_key.clone());
        }
        persist_server_config(path, &config)?;
    }

    Ok(ServerSecrets {
        jwt_secret,
        mfa_encryption_key,
    })
}

fn non_empty(value: Option<String>) -> Option<String> {
    value
        .map(|secret| secret.trim().to_owned())
        .filter(|s| !s.is_empty())
}

pub fn generate_secret() -> String {
    let mut bytes = [0u8; 32];
    rand::rngs::OsRng.fill_bytes(&mut bytes);
    hex::encode(bytes)
}

pub fn persist_server_config(path: &Path, config: &ServerConfig) -> Result<(), String> {
    if let Some(parent) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
        fs::create_dir_all(parent).map_err(|error| {
            format!(
                "Unable to create Orbis server configuration directory at {}: {error}",
                parent.display()
            )
        })?;
    }
    let temporary_path =
        path.with_file_name(format!(".server-{}.tmp", uuid::Uuid::new_v4().simple()));
    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o640);
    }

    let result = (|| {
        let mut file = options.open(&temporary_path).map_err(|error| {
            format!(
                "Unable to create a temporary Orbis server configuration beside {}: {error}",
                path.display()
            )
        })?;
        serde_json::to_writer_pretty(&mut file, config).map_err(|error| {
            format!(
                "Unable to serialize Orbis server configuration at {}: {error}",
                path.display()
            )
        })?;
        file.write_all(b"\n").map_err(|error| {
            format!(
                "Unable to write Orbis server configuration at {}: {error}",
                path.display()
            )
        })?;
        file.sync_all().map_err(|error| {
            format!(
                "Unable to flush Orbis server configuration at {}: {error}",
                path.display()
            )
        })?;
        fs::rename(&temporary_path, path).map_err(|error| {
            format!(
                "Unable to persist Orbis server configuration at {}: {error}",
                path.display()
            )
        })
    })();

    if result.is_err() {
        let _ = fs::remove_file(&temporary_path);
    }
    result
}

fn default_server_config_path() -> Result<PathBuf, String> {
    #[cfg(target_os = "windows")]
    {
        let program_data = std::env::var_os("PROGRAMDATA").ok_or_else(|| {
            "PROGRAMDATA is not set; cannot locate Orbis server configuration".to_string()
        })?;
        return Ok(PathBuf::from(program_data)
            .join("Orbis")
            .join(SERVER_CONFIG_FILE));
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
    Err(
        "Unsupported operating system; set ORBIS_SERVER_CONFIG_FILE to the server config path"
            .to_string(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write_config(path: &Path, contents: &str) {
        std::fs::write(path, contents).unwrap();
    }

    #[test]
    fn reads_database_url_from_server_config_file() {
        let config_path =
            std::env::temp_dir().join(format!("orbis-server-config-{}.json", uuid::Uuid::new_v4()));
        std::fs::write(
            &config_path,
            r#"{"database_url":"postgres://localhost/orbis"}"#,
        )
        .unwrap();
        assert_eq!(
            resolve_database_url_from_config(&config_path).unwrap(),
            "postgres://localhost/orbis"
        );
        std::fs::remove_file(config_path).unwrap();
    }

    #[test]
    fn generates_and_persists_independent_server_secrets_once() {
        let config_path = std::env::temp_dir().join(format!(
            "orbis-server-secrets-{}.json",
            uuid::Uuid::new_v4()
        ));
        write_config(
            &config_path,
            r#"{"database_url":"postgres://localhost/orbis"}"#,
        );

        let first = resolve_server_secrets_from_config(&config_path, None, None).unwrap();
        let persisted = std::fs::read_to_string(&config_path).unwrap();
        let second = resolve_server_secrets_from_config(&config_path, None, None).unwrap();

        assert_eq!(first.jwt_secret.len(), 64);
        assert_eq!(first.mfa_encryption_key.len(), 64);
        assert_ne!(first.jwt_secret, first.mfa_encryption_key);
        assert_eq!(first.jwt_secret, second.jwt_secret);
        assert_eq!(first.mfa_encryption_key, second.mfa_encryption_key);
        assert!(persisted.contains("postgres://localhost/orbis"));
        assert!(persisted.contains(&first.jwt_secret));

        let parsed: ServerConfig = serde_json::from_str(&persisted).unwrap();
        assert_eq!(
            parsed.jwt_secret.as_deref(),
            Some(first.jwt_secret.as_str())
        );
        assert_eq!(
            parsed.mfa_encryption_key.as_deref(),
            Some(first.mfa_encryption_key.as_str())
        );
        std::fs::remove_file(config_path).unwrap();
    }

    #[test]
    fn environment_secrets_override_persisted_secrets_without_rewriting_them() {
        let config_path = std::env::temp_dir().join(format!(
            "orbis-server-overrides-{}.json",
            uuid::Uuid::new_v4()
        ));
        let original = r#"{"database_url":"postgres://localhost/orbis","jwt_secret":"persisted-jwt-secret","mfa_encryption_key":"persisted-mfa-secret"}"#;
        write_config(&config_path, original);

        let resolved = resolve_server_secrets_from_config(
            &config_path,
            Some("  external-jwt-secret  ".to_owned()),
            Some("external-mfa-secret".to_owned()),
        )
        .unwrap();

        assert_eq!(resolved.jwt_secret, "external-jwt-secret");
        assert_eq!(resolved.mfa_encryption_key, "external-mfa-secret");
        assert_eq!(std::fs::read_to_string(&config_path).unwrap(), original);
        std::fs::remove_file(config_path).unwrap();
    }

    #[test]
    fn persists_generated_key_without_persisting_environment_override() {
        let config_path = std::env::temp_dir().join(format!(
            "orbis-server-partial-override-{}.json",
            uuid::Uuid::new_v4()
        ));
        write_config(
            &config_path,
            r#"{"database_url":"postgres://localhost/orbis"}"#,
        );

        let resolved = resolve_server_secrets_from_config(
            &config_path,
            Some("external-jwt-secret".to_owned()),
            None,
        )
        .unwrap();
        let persisted: ServerConfig =
            serde_json::from_str(&std::fs::read_to_string(&config_path).unwrap()).unwrap();

        assert_eq!(resolved.jwt_secret, "external-jwt-secret");
        assert_eq!(persisted.jwt_secret, None);
        assert_eq!(
            persisted.mfa_encryption_key.as_deref(),
            Some(resolved.mfa_encryption_key.as_str())
        );
        std::fs::remove_file(config_path).unwrap();
    }

    #[test]
    fn rejects_empty_database_url_in_server_config_file() {
        let config_path =
            std::env::temp_dir().join(format!("orbis-server-config-{}.json", uuid::Uuid::new_v4()));
        std::fs::write(&config_path, r#"{"database_url":"  "}"#).unwrap();
        let error = resolve_database_url_from_config(&config_path).unwrap_err();
        assert!(error.contains("database_url cannot be empty"));
        std::fs::remove_file(config_path).unwrap();
    }

    #[test]
    fn creates_server_config_with_empty_database_url_when_missing() {
        let config_path = std::env::temp_dir()
            .join(format!("orbis-missing-config-{}", uuid::Uuid::new_v4()))
            .join(SERVER_CONFIG_FILE);

        let error = resolve_database_url_from_config(&config_path).unwrap_err();

        assert!(error.contains("Created Orbis server configuration"));
        assert!(error.contains("Set database_url"));
        assert_eq!(
            std::fs::read_to_string(&config_path).unwrap(),
            EMPTY_SERVER_CONFIG
        );
        std::fs::remove_dir_all(config_path.parent().unwrap()).unwrap();
    }
}
