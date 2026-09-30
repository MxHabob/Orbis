use std::sync::Arc;
use std::path::PathBuf;
use uuid::Uuid;
use tokio::sync::RwLock;
use std::collections::HashMap;

use orbis_app::ports::storage::StorageAdapter;
use orbis_app::ports::repositories::IntegrationRepository;
use orbis_domain::errors::StorageError;

use super::filesystem::FilesystemStorageAdapter;
use super::s3::{S3Config, S3StorageAdapter};

/// Resolves the authoritative storage adapter instance for any organization.
pub struct StorageResolver {
    default_adapter: Arc<dyn StorageAdapter>,
    integration_repo: Arc<dyn IntegrationRepository>,
    cached_adapters: RwLock<HashMap<Uuid, Arc<dyn StorageAdapter>>>,
}

impl StorageResolver {
    pub fn new(
        default_base_path: PathBuf,
        integration_repo: Arc<dyn IntegrationRepository>,
    ) -> Self {
        let default_adapter = Arc::new(FilesystemStorageAdapter::new(default_base_path));
        Self {
            default_adapter,
            integration_repo,
            cached_adapters: RwLock::new(HashMap::new()),
        }
    }

    /// Return default filesystem storage adapter.
    pub fn default_adapter(&self) -> Arc<dyn StorageAdapter> {
        self.default_adapter.clone()
    }

    /// Resolve the storage adapter for a given tenant organization.
    pub async fn resolve(&self, org_id: Uuid) -> Result<Arc<dyn StorageAdapter>, StorageError> {
        // Check cache
        {
            let cache = self.cached_adapters.read().await;
            if let Some(adapter) = cache.get(&org_id) {
                return Ok(adapter.clone());
            }
        }

        // Query integration config
        let config_opt = self.integration_repo.find_config_by_provider(org_id, "cloud-storage").await
            .map_err(|e| StorageError::Connection(e.to_string()))?;

        if let Some(config) = config_opt {
            if config.is_enabled {
                if let Some(settings) = config.settings {
                    let provider = settings.get("provider").and_then(|v| v.as_str()).unwrap_or("local");
                    
                    if provider == "s3" || provider == "minio" || provider == "rustfs" {
                        let secrets = self.integration_repo.get_secrets_by_config(config.id).await
                            .map_err(|e| StorageError::Connection(e.to_string()))?;

                        let access_key = secrets.iter().find(|s| s.key_name == "access_key").map(|s| s.secret_key.clone())
                            .unwrap_or_default();
                        let secret_key = secrets.iter().find(|s| s.key_name == "secret_key").map(|s| s.secret_key.clone())
                            .unwrap_or_default();
                        let bucket = settings.get("bucket").and_then(|v| v.as_str()).unwrap_or("orbis-storage").to_string();
                        let endpoint = settings.get("endpoint").and_then(|v| v.as_str())
                            .unwrap_or(if provider == "minio" { "http://127.0.0.1:9000" } else { "https://s3.amazonaws.com" })
                            .to_string();
                        let region = settings.get("region").and_then(|v| v.as_str()).unwrap_or("us-east-1").to_string();
                        let force_path_style = provider == "minio" || provider == "rustfs" || settings.get("force_path_style").and_then(|v| v.as_bool()).unwrap_or(false);

                        let s3_cfg = S3Config {
                            endpoint,
                            bucket,
                            region,
                            access_key_id: access_key,
                            secret_access_key: secret_key,
                            force_path_style,
                        };

                        let adapter: Arc<dyn StorageAdapter> = Arc::new(S3StorageAdapter::new(s3_cfg));
                        let mut cache = self.cached_adapters.write().await;
                        cache.insert(org_id, adapter.clone());
                        return Ok(adapter);
                    }
                }
            }
        }

        // Fallback to default local filesystem storage
        Ok(self.default_adapter.clone())
    }

    /// Invalidate cache when integration settings change.
    pub async fn invalidate(&self, org_id: Uuid) {
        let mut cache = self.cached_adapters.write().await;
        cache.remove(&org_id);
    }
}
