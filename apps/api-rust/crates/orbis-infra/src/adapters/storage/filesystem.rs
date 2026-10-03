use async_trait::async_trait;
use chrono::Utc;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tokio::fs::{self, File};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use uuid::Uuid;

use orbis_app::ports::storage::{ObjectStream, StorageAdapter};
use orbis_domain::entities::storage::{
    PresignedUrlRequest, StorageCapabilities, StoredObjectMetadata,
};
use orbis_domain::errors::StorageError;

/// High-performance, sandboxed local filesystem storage adapter.
pub struct FilesystemStorageAdapter {
    base_path: PathBuf,
}

impl FilesystemStorageAdapter {
    pub fn new<P: AsRef<Path>>(base_path: P) -> Self {
        let path = base_path.as_ref().to_path_buf();
        Self { base_path: path }
    }

    /// Resolves and validates that a relative storage key does not escape the sandbox base path.
    fn resolve_path(&self, key: &str) -> Result<PathBuf, StorageError> {
        let clean_key = key.trim().trim_start_matches('/').trim_start_matches('\\');

        // Prevent path traversal
        if clean_key.contains("..") || clean_key.contains(':') {
            return Err(StorageError::SecurityViolation(format!(
                "Path traversal detected in key: {key}"
            )));
        }

        let full_path = self.base_path.join(clean_key);
        Ok(full_path)
    }

    async fn ensure_parent_dir(&self, file_path: &Path) -> Result<(), StorageError> {
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent).await.map_err(|e| {
                StorageError::Io(format!("Failed to create storage directory: {e}"))
            })?;
        }
        Ok(())
    }
}

#[async_trait]
impl StorageAdapter for FilesystemStorageAdapter {
    fn provider_id(&self) -> &'static str {
        "filesystem"
    }

    fn capabilities(&self) -> StorageCapabilities {
        StorageCapabilities {
            supports_presigned_urls: false,
            supports_multipart_upload: true,
            supports_direct_streaming: true,
            supports_range_reads: true,
            supports_atomic_move: true,
            supports_local_path: true,
        }
    }

    async fn check_health(&self) -> Result<(), StorageError> {
        fs::create_dir_all(&self.base_path).await.map_err(|e| {
            StorageError::Connection(format!("Filesystem storage base path inaccessible: {e}"))
        })?;
        Ok(())
    }

    async fn put_object(
        &self,
        key: &str,
        mut stream: ObjectStream,
        size_bytes: u64,
        mime_type: &str,
    ) -> Result<StoredObjectMetadata, StorageError> {
        let target_path = self.resolve_path(key)?;
        self.ensure_parent_dir(&target_path).await?;

        // Stage write to a temporary file in the same directory for atomic rename
        let temp_filename = format!(
            ".tmp_{}_{}",
            Uuid::new_v4(),
            target_path
                .file_name()
                .and_then(|f| f.to_str())
                .unwrap_or("file")
        );
        let temp_path = target_path
            .parent()
            .unwrap_or(&self.base_path)
            .join(temp_filename);

        let mut file = File::create(&temp_path)
            .await
            .map_err(|e| StorageError::Io(format!("Failed to create temp storage file: {e}")))?;

        let mut hasher = Sha256::new();
        let mut buffer = vec![0u8; 64 * 1024]; // 64 KB streaming buffer
        let mut total_written: u64 = 0;

        loop {
            let read_bytes = stream
                .read(&mut buffer)
                .await
                .map_err(|e| StorageError::Io(format!("Failed to read source stream: {e}")))?;

            if read_bytes == 0 {
                break;
            }

            file.write_all(&buffer[..read_bytes])
                .await
                .map_err(|e| StorageError::Io(format!("Failed to write to temp file: {e}")))?;

            hasher.update(&buffer[..read_bytes]);
            total_written += read_bytes as u64;
        }

        file.flush()
            .await
            .map_err(|e| StorageError::Io(format!("Failed to flush temp storage file: {e}")))?;
        drop(file);

        // Atomic replace to target path
        if let Err(e) = fs::rename(&temp_path, &target_path).await {
            let _ = fs::remove_file(&temp_path).await;
            return Err(StorageError::Io(format!(
                "Failed to finalize storage file: {e}"
            )));
        }

        let checksum = hex::encode(hasher.finalize());

        Ok(StoredObjectMetadata {
            key: key.to_string(),
            size_bytes: if size_bytes > 0 {
                size_bytes
            } else {
                total_written
            },
            mime_type: mime_type.to_string(),
            sha256_checksum: checksum,
            etag: None,
            last_modified: Utc::now(),
            custom_metadata: HashMap::new(),
        })
    }

    async fn get_object(&self, key: &str) -> Result<ObjectStream, StorageError> {
        let path = self.resolve_path(key)?;
        if !path.exists() {
            return Err(StorageError::NotFound(format!(
                "File not found in storage: {key}"
            )));
        }
        let file = File::open(path)
            .await
            .map_err(|e| StorageError::Io(format!("Failed to open storage file: {e}")))?;
        Ok(Box::pin(file))
    }

    async fn get_bytes(&self, key: &str) -> Result<Vec<u8>, StorageError> {
        let path = self.resolve_path(key)?;
        if !path.exists() {
            return Err(StorageError::NotFound(format!(
                "File not found in storage: {key}"
            )));
        }
        fs::read(path)
            .await
            .map_err(|e| StorageError::Io(format!("Failed to read storage bytes: {e}")))
    }

    async fn head_object(&self, key: &str) -> Result<StoredObjectMetadata, StorageError> {
        let path = self.resolve_path(key)?;
        let metadata = fs::metadata(&path)
            .await
            .map_err(|_| StorageError::NotFound(format!("File not found: {key}")))?;

        let modified: chrono::DateTime<Utc> = metadata
            .modified()
            .map(|t| t.into())
            .unwrap_or_else(|_| Utc::now());

        Ok(StoredObjectMetadata {
            key: key.to_string(),
            size_bytes: metadata.len(),
            mime_type: "application/octet-stream".to_string(),
            sha256_checksum: String::new(),
            etag: None,
            last_modified: modified,
            custom_metadata: HashMap::new(),
        })
    }

    async fn exists(&self, key: &str) -> Result<bool, StorageError> {
        let path = self.resolve_path(key)?;
        Ok(path.exists())
    }

    async fn delete_object(&self, key: &str) -> Result<(), StorageError> {
        let path = self.resolve_path(key)?;
        if path.exists() {
            fs::remove_file(path)
                .await
                .map_err(|e| StorageError::Io(format!("Failed to delete file: {e}")))?;
        }
        Ok(())
    }

    async fn copy_object(
        &self,
        source_key: &str,
        target_key: &str,
    ) -> Result<StoredObjectMetadata, StorageError> {
        let src_path = self.resolve_path(source_key)?;
        let dst_path = self.resolve_path(target_key)?;

        if !src_path.exists() {
            return Err(StorageError::NotFound(format!(
                "Source file not found: {source_key}"
            )));
        }

        self.ensure_parent_dir(&dst_path).await?;
        fs::copy(&src_path, &dst_path)
            .await
            .map_err(|e| StorageError::Io(format!("Failed to copy storage file: {e}")))?;

        self.head_object(target_key).await
    }

    async fn move_object(
        &self,
        source_key: &str,
        target_key: &str,
    ) -> Result<StoredObjectMetadata, StorageError> {
        let src_path = self.resolve_path(source_key)?;
        let dst_path = self.resolve_path(target_key)?;

        if !src_path.exists() {
            return Err(StorageError::NotFound(format!(
                "Source file not found: {source_key}"
            )));
        }

        self.ensure_parent_dir(&dst_path).await?;
        fs::rename(&src_path, &dst_path)
            .await
            .map_err(|e| StorageError::Io(format!("Failed to move storage file: {e}")))?;

        self.head_object(target_key).await
    }

    async fn generate_presigned_url(
        &self,
        _request: PresignedUrlRequest,
    ) -> Result<String, StorageError> {
        Err(StorageError::Unsupported(
            "Presigned URLs are not supported by the local filesystem adapter".to_string(),
        ))
    }
}
