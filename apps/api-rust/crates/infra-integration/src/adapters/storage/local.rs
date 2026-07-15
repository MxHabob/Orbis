use async_trait::async_trait;
use std::path::{Path, PathBuf};
use tokio::fs;

use crate::traits::StorageProvider;

pub struct LocalStorageProvider {
    pub base_path: PathBuf,
}

impl LocalStorageProvider {
    pub fn new(base_path: impl AsRef<Path>) -> Self {
        Self {
            base_path: base_path.as_ref().to_path_buf(),
        }
    }
}

#[async_trait]
impl StorageProvider for LocalStorageProvider {
    fn provider_name(&self) -> &str {
        "local"
    }

    async fn upload(&self, bucket: &str, key: &str, data: &[u8]) -> Result<String, String> {
        let file_path = self.base_path.join(bucket).join(key);
        
        // Ensure parent directories exist
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)
                .await
                .map_err(|e| format!("Failed to create directories: {}", e))?;
        }

        // Write the file to disk
        fs::write(&file_path, data)
            .await
            .map_err(|e| format!("Failed to write file: {}", e))?;

        // Return the relative path starting from the bucket
        let relative_path = Path::new(bucket).join(key);
        Ok(relative_path.to_string_lossy().into_owned().replace('\\', "/"))
    }

    async fn delete(&self, bucket: &str, key: &str) -> Result<(), String> {
        let file_path = self.base_path.join(bucket).join(key);
        if file_path.exists() {
            fs::remove_file(&file_path)
                .await
                .map_err(|e| format!("Failed to delete file: {}", e))?;
        }
        Ok(())
    }

    fn get_url(&self, bucket: &str, key: &str) -> Option<String> {
        // Since it's local, we just return the path relative to the API's static file handler
        // The API Gateway will host the base directory.
        Some(format!("/storage/{}/{}", bucket, key))
    }
}
