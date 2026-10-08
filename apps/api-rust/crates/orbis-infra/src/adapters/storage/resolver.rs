use std::{collections::HashMap, path::PathBuf, sync::Arc};
use uuid::Uuid;

use orbis_app::ports::{
    repositories::IntegrationRepository,
    storage::{StorageAdapter, StorageAdapterResolver},
};
use orbis_domain::{
    entities::integration::{IntegrationConfig, IntegrationSecret},
    errors::StorageError,
};
use serde_json::Value;

use super::filesystem::FilesystemStorageAdapter;
use super::s3::{S3Config, S3StorageAdapter};

pub struct StorageResolver {
    base_path: PathBuf,
    integration_repo: Arc<dyn IntegrationRepository>,
}

impl StorageResolver {
    pub fn new(
        default_base_path: PathBuf,
        integration_repo: Arc<dyn IntegrationRepository>,
    ) -> Self {
        Self {
            base_path: default_base_path,
            integration_repo,
        }
    }

    pub async fn check_connection(
        &self,
        org_id: Uuid,
        provider_name: &str,
        category: &str,
        settings: &Value,
        secrets: &HashMap<String, String>,
    ) -> Result<(), StorageError> {
        if !category.eq_ignore_ascii_case("storage") {
            return Err(StorageError::NotConfigured(
                "The selected integration is not a storage provider".into(),
            ));
        }
        let existing = self
            .integration_repo
            .find_configs_by_org(org_id)
            .await
            .map_err(|error| StorageError::Connection(error.to_string()))?;
        if existing.iter().any(|config| {
            config.is_enabled
                && config.category.eq_ignore_ascii_case("storage")
                && config.provider_name != provider_name
        }) {
            return Err(StorageError::NotConfigured(
                "Another storage provider is already enabled. Disconnect it before connecting this provider"
                    .into(),
            ));
        }
        let secret_list: Vec<_> = secrets
            .iter()
            .map(|(key_name, secret_key)| IntegrationSecret {
                id: Uuid::new_v4(),
                config_id: Uuid::nil(),
                key_name: key_name.clone(),
                secret_key: secret_key.clone(),
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            })
            .collect();
        let config = IntegrationConfig {
            id: Uuid::nil(),
            org_id: Uuid::nil(),
            provider_name: provider_name.to_string(),
            category: category.to_string(),
            is_enabled: true,
            env: "production".to_string(),
            settings: Some(settings.clone()),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        self.build_adapter(&config, &secret_list)?
            .check_health()
            .await
    }

    pub async fn check_integration_connection(
        &self,
        org_id: Uuid,
        provider_name: &str,
    ) -> Result<(), StorageError> {
        let config = self
            .integration_repo
            .find_config_by_provider(org_id, provider_name)
            .await
            .map_err(|error| StorageError::Connection(error.to_string()))?
            .ok_or_else(|| {
                StorageError::NotConfigured("Storage integration is not connected".into())
            })?;
        if !config.is_enabled {
            return Err(StorageError::NotConfigured(
                "Storage integration is connected but disabled".into(),
            ));
        }
        let secrets = self
            .integration_repo
            .get_secrets_by_config(config.id)
            .await
            .map_err(|error| StorageError::Connection(error.to_string()))?;
        self.build_adapter(&config, &secrets)?.check_health().await
    }

    fn build_adapter(
        &self,
        config: &IntegrationConfig,
        secrets: &[IntegrationSecret],
    ) -> Result<Arc<dyn StorageAdapter>, StorageError> {
        if !config.category.eq_ignore_ascii_case("storage") {
            return Err(StorageError::NotConfigured(format!(
                "Integration '{}' is not categorized as storage",
                config.provider_name
            )));
        }
        let settings = config.settings.as_ref().ok_or_else(|| {
            StorageError::NotConfigured(format!(
                "Storage integration '{}' has no settings",
                config.provider_name
            ))
        })?;
        let provider = settings
            .get("provider")
            .and_then(Value::as_str)
            .filter(|provider| !provider.trim().is_empty())
            .ok_or_else(|| {
                StorageError::NotConfigured(
                    "Select a storage provider before connecting the integration".into(),
                )
            })?
            .trim()
            .to_ascii_lowercase();

        match provider.as_str() {
            "local" => Ok(Arc::new(FilesystemStorageAdapter::new(
                self.base_path.clone(),
            ))),
            "s3" | "minio" | "rustfs" => {
                let bucket = required_setting(settings, "bucket")?;
                let access_key_id =
                    required_secret(secrets, &["access_key", "access_key_id"], &provider)?;
                let secret_access_key =
                    required_secret(secrets, &["secret_key", "secret_access_key"], &provider)?;
                let endpoint = match optional_setting(settings, "endpoint") {
                    Some(endpoint) => endpoint,
                    None if provider == "s3" => "https://s3.amazonaws.com".to_string(),
                    None => {
                        return Err(StorageError::NotConfigured(format!(
                            "An endpoint is required for the {} storage provider",
                            provider
                        )));
                    }
                };
                validate_endpoint(&endpoint)?;
                let region =
                    optional_setting(settings, "region").unwrap_or_else(|| "us-east-1".to_string());
                let force_path_style = provider != "s3"
                    || settings
                        .get("force_path_style")
                        .and_then(Value::as_bool)
                        .unwrap_or(false);
                Ok(Arc::new(S3StorageAdapter::new(S3Config {
                    endpoint,
                    bucket,
                    region,
                    access_key_id,
                    secret_access_key,
                    force_path_style,
                })))
            }
            unsupported => Err(StorageError::NotConfigured(format!(
                "Storage provider '{}' is not supported by an installed adapter",
                unsupported
            ))),
        }
    }

    async fn resolve_for_org(&self, org_id: Uuid) -> Result<Arc<dyn StorageAdapter>, StorageError> {
        let configs = self
            .integration_repo
            .find_configs_by_org(org_id)
            .await
            .map_err(|error| StorageError::Connection(error.to_string()))?;
        let config = select_storage_integration(configs, org_id)?;
        let secrets = self
            .integration_repo
            .get_secrets_by_config(config.id)
            .await
            .map_err(|error| StorageError::Connection(error.to_string()))?;
        self.build_adapter(&config, &secrets)
    }
}

#[async_trait::async_trait]
impl StorageAdapterResolver for StorageResolver {
    async fn resolve(&self, org_id: Uuid) -> Result<Arc<dyn StorageAdapter>, StorageError> {
        self.resolve_for_org(org_id).await
    }
}

fn select_storage_integration(
    configs: Vec<IntegrationConfig>,
    org_id: Uuid,
) -> Result<IntegrationConfig, StorageError> {
    let org_storage: Vec<_> = configs
        .into_iter()
        .filter(|config| config.org_id == org_id && config.category.eq_ignore_ascii_case("storage"))
        .collect();
    let active: Vec<_> = org_storage
        .iter()
        .filter(|config| config.is_enabled)
        .cloned()
        .collect();
    match active.as_slice() {
        [] if org_storage.is_empty() => Err(StorageError::NotConfigured(
            "Connect and verify a storage integration before uploading or retrieving files"
                .into(),
        )),
        [] => Err(StorageError::NotConfigured(
            "The organization's storage integration is disabled".into(),
        )),
        [config] => Ok(config.clone()),
        _ => Err(StorageError::NotConfigured(
            "Multiple storage integrations are enabled. Keep only the organization's selected storage provider enabled"
                .into(),
        )),
    }
}

fn required_setting(settings: &Value, name: &str) -> Result<String, StorageError> {
    optional_setting(settings, name).ok_or_else(|| {
        StorageError::NotConfigured(format!(
            "The storage integration is missing the required '{}' setting",
            name
        ))
    })
}

fn optional_setting(settings: &Value, name: &str) -> Option<String> {
    settings
        .get(name)
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string)
}

fn required_secret(
    secrets: &[IntegrationSecret],
    names: &[&str],
    provider: &str,
) -> Result<String, StorageError> {
    secrets
        .iter()
        .find(|secret| names.iter().any(|name| secret.key_name == *name))
        .map(|secret| secret.secret_key.trim().to_string())
        .filter(|secret| !secret.is_empty())
        .ok_or_else(|| {
            StorageError::NotConfigured(format!(
                "The {} storage integration is missing a required credential",
                provider
            ))
        })
}

fn validate_endpoint(endpoint: &str) -> Result<(), StorageError> {
    let parsed = reqwest::Url::parse(endpoint).map_err(|error| {
        StorageError::NotConfigured(format!("Invalid storage endpoint: {error}"))
    })?;
    if !matches!(parsed.scheme(), "http" | "https") || parsed.host_str().is_none() {
        return Err(StorageError::NotConfigured(
            "Storage endpoint must be an absolute HTTP or HTTPS URL".into(),
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn integration(org_id: Uuid, provider_name: &str, is_enabled: bool) -> IntegrationConfig {
        let now = chrono::Utc::now();
        IntegrationConfig {
            id: Uuid::new_v4(),
            org_id,
            provider_name: provider_name.into(),
            category: "storage".into(),
            is_enabled,
            env: "production".into(),
            settings: Some(serde_json::json!({"provider": "local"})),
            created_at: now,
            updated_at: now,
        }
    }

    #[test]
    fn storage_resolution_requires_a_connected_integration() {
        let org_id = Uuid::new_v4();
        assert!(matches!(
            select_storage_integration(Vec::new(), org_id),
            Err(StorageError::NotConfigured(_))
        ));
    }

    #[test]
    fn storage_resolution_rejects_disabled_or_ambiguous_integrations() {
        let org_id = Uuid::new_v4();
        assert!(matches!(
            select_storage_integration(vec![integration(org_id, "cloud-storage", false)], org_id),
            Err(StorageError::NotConfigured(_))
        ));
        assert!(matches!(
            select_storage_integration(
                vec![
                    integration(org_id, "cloud-storage", true),
                    integration(org_id, "another-storage", true),
                ],
                org_id
            ),
            Err(StorageError::NotConfigured(_))
        ));
    }

    #[test]
    fn storage_resolution_selects_the_single_enabled_organization_integration() {
        let org_id = Uuid::new_v4();
        let other_org_id = Uuid::new_v4();
        let selected_config = integration(org_id, "cloud-storage", true);
        let selected = select_storage_integration(
            vec![
                selected_config.clone(),
                integration(other_org_id, "other-org-storage", true),
            ],
            org_id,
        )
        .expect("one active organization storage integration should resolve");
        assert_eq!(selected.id, selected_config.id);
    }
}
