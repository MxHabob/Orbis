use async_trait::async_trait;
use orbis_domain::entities::storage::{
    PresignedUrlRequest, StorageCapabilities, StoredObjectMetadata,
};
use orbis_domain::errors::StorageError;
use std::pin::Pin;
use std::sync::Arc;
use tokio::io::AsyncRead;
use uuid::Uuid;

/// Type alias for an async read stream.
pub type ObjectStream = Pin<Box<dyn AsyncRead + Send + Sync>>;

/// Authoritative port trait for physical storage adapters.
#[async_trait]
pub trait StorageAdapter: Send + Sync {
    /// Provider identifier (e.g. "filesystem", "s3", "minio", "rustfs").
    fn provider_id(&self) -> &'static str;

    /// Provider capability flags.
    fn capabilities(&self) -> StorageCapabilities;

    /// Connection health check.
    async fn check_health(&self) -> Result<(), StorageError>;

    /// Store a binary payload using an async reader stream.
    async fn put_object(
        &self,
        key: &str,
        stream: ObjectStream,
        size_bytes: u64,
        mime_type: &str,
    ) -> Result<StoredObjectMetadata, StorageError>;

    /// Store a binary byte slice directly.
    async fn put_bytes(
        &self,
        key: &str,
        bytes: &[u8],
        mime_type: &str,
    ) -> Result<StoredObjectMetadata, StorageError> {
        let size = bytes.len() as u64;
        let cursor = std::io::Cursor::new(bytes.to_vec());
        self.put_object(key, Box::pin(cursor), size, mime_type)
            .await
    }

    /// Retrieve an object stream for reading.
    async fn get_object(&self, key: &str) -> Result<ObjectStream, StorageError>;

    /// Read the full binary contents of an object.
    async fn get_bytes(&self, key: &str) -> Result<Vec<u8>, StorageError>;

    /// Fetch metadata and verify existence without loading object content.
    async fn head_object(&self, key: &str) -> Result<StoredObjectMetadata, StorageError>;

    /// Check whether an object exists.
    async fn exists(&self, key: &str) -> Result<bool, StorageError>;

    /// Physically delete an object from storage.
    async fn delete_object(&self, key: &str) -> Result<(), StorageError>;

    /// Copy an object within storage.
    async fn copy_object(
        &self,
        source_key: &str,
        target_key: &str,
    ) -> Result<StoredObjectMetadata, StorageError>;

    /// Move/Rename an object within storage.
    async fn move_object(
        &self,
        source_key: &str,
        target_key: &str,
    ) -> Result<StoredObjectMetadata, StorageError>;

    /// Generate a time-limited presigned URL for direct client access.
    async fn generate_presigned_url(
        &self,
        request: PresignedUrlRequest,
    ) -> Result<String, StorageError>;
}

#[async_trait]
pub trait StorageAdapterResolver: Send + Sync {
    async fn resolve(&self, org_id: Uuid) -> Result<Arc<dyn StorageAdapter>, StorageError>;
}
