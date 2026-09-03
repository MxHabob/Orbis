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

    fn safe_path(&self, bucket: &str, key: &str) -> Result<PathBuf, String> {
        let mut path = self.base_path.clone();
        for component in [bucket, key] {
            let candidate = Path::new(component);
            if candidate.is_absolute()
                || candidate.components().any(|part| matches!(part, std::path::Component::ParentDir | std::path::Component::RootDir | std::path::Component::Prefix(_)))
            {
                return Err("Storage path must remain within the configured base directory".to_string());
            }
            path.push(candidate);
        }
        Ok(path)
    }
}

#[async_trait]
impl StorageProvider for LocalStorageProvider {
    fn provider_name(&self) -> &str {
        "local"
    }

    async fn upload(&self, bucket: &str, key: &str, data: &[u8]) -> Result<String, String> {
        let file_path = self.safe_path(bucket, key)?;
        
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
        let file_path = self.safe_path(bucket, key)?;
        if file_path.exists() {
            fs::remove_file(&file_path)
                .await
                .map_err(|e| format!("Failed to delete file: {}", e))?;
        }
        Ok(())
    }

    async fn read(&self, bucket: &str, key: &str) -> Result<Vec<u8>, String> {
        let file_path = self.safe_path(bucket, key)?;
        fs::read(&file_path)
            .await
            .map_err(|e| format!("Failed to read file: {}", e))
    }

    fn get_url(&self, bucket: &str, key: &str) -> Option<String> {
        self.safe_path(bucket, key).ok()?;
        // Local objects must be read through an authenticated document endpoint.
        None
    }
}
