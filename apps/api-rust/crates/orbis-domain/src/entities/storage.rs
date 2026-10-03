use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Canonical metadata describing a stored object across providers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredObjectMetadata {
    pub key: String,
    pub size_bytes: u64,
    pub mime_type: String,
    pub sha256_checksum: String,
    pub etag: Option<String>,
    pub last_modified: DateTime<Utc>,
    pub custom_metadata: std::collections::HashMap<String, String>,
}

/// Dynamic capabilities supported by a concrete storage adapter instance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct StorageCapabilities {
    pub supports_presigned_urls: bool,
    pub supports_multipart_upload: bool,
    pub supports_direct_streaming: bool,
    pub supports_range_reads: bool,
    pub supports_atomic_move: bool,
    pub supports_local_path: bool,
}

impl Default for StorageCapabilities {
    fn default() -> Self {
        Self {
            supports_presigned_urls: false,
            supports_multipart_upload: false,
            supports_direct_streaming: true,
            supports_range_reads: false,
            supports_atomic_move: false,
            supports_local_path: false,
        }
    }
}

/// Request parameters for presigned URL generation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresignedUrlRequest {
    pub key: String,
    pub expires_in_seconds: u64,
    pub method: PresignMethod,
    pub content_type: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PresignMethod {
    Get,
    Put,
}
