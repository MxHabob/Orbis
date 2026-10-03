# Orbis Storage Integration & Adapter Architecture

**Author:** Antigravity Architect & Senior Reviewer  
**Date:** September 2026  
**Status:** Canonical Target Architecture Design (Pre-Implementation)  
**Target System:** Orbis Education Platform (Desktop / Web / Backend)

---

## 1. Architectural Philosophy & Principles

The storage subsystem in Orbis follows strict Clean Architecture and Hexagonal Principles:

```
┌─────────────────────────────────────────────────────────────────┐
│                    Application / UI Layer                       │
│      (Students, Employees, Reports, Archive, Templates, Avatars)│
└────────────────────────────────┬────────────────────────────────┘
                                 │
                                 ▼
┌─────────────────────────────────────────────────────────────────┐
│              Canonical File / Storage Service                   │
│        (Validation, Auth, Namespace Keying, Metadata Sync)      │
└────────────────────────────────┬────────────────────────────────┘
                                 │
                                 ▼
┌─────────────────────────────────────────────────────────────────┐
│                    Storage Port Interface                       │
│                     (StorageAdapter Trait)                      │
└────────────────┬───────────────────────────────┬────────────────┘
                 │                               │
                 ▼                               ▼
┌────────────────────────────────┐ ┌──────────────────────────────┐
│   FilesystemStorageAdapter     │ │       S3StorageAdapter       │
│  (Local AppData / Custom Dir / │ │ (MinIO / RustFS / AWS S3 /   │
│       Desktop Sandbox)         │ │     Cloudflare R2 / Wasabi)  │
└────────────────────────────────┘ └──────────────────────────────┘
```

### Core Invariants
1. **Zero Provider Leakage:** Feature modules (Students, Employees, Reports, DocForge, Finance, Archive) must never know whether physical bytes reside on a local NVMe drive, MinIO, RustFS, or AWS S3.
2. **Provider-Agnostic Storage Keys:** Business entities store canonical storage references (URIs/Keys), never absolute host paths or provider secrets.
3. **Strong Multi-Tenant Isolation:** Storage keys are strictly namespaced by Organization ID and Org-Unit ID. No tenant can ever address or read another tenant's files.
4. **Transport Parity:** The exact same API contract, validation rules, and metadata model apply whether running in **Desktop Local Mode** (Tauri IPC) or **Server Remote Mode** (REST HTTP).
5. **Fail-Safe Atomicity:** Failed metadata transactions must not leave orphaned physical files, and failed physical uploads must not commit phantom metadata records.

---

## 2. Core Storage Abstractions (`orbis-app` / `orbis-domain`)

### 2.1 The `StorageAdapter` Port Interface

The authoritative Rust port trait defining physical storage capabilities:

```rust
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::io::SeekFrom;
use std::pin::Pin;
use tokio::io::{AsyncRead, AsyncSeek};
use uuid::Uuid;

/// Metadata describing a stored physical object.
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

/// Read stream abstraction for streaming files without unbounded memory consumption.
pub type ObjectStream = Pin<Box<dyn AsyncRead + Send + Sync>>;

/// Request parameters for presigned URL generation.
#[derive(Debug, Clone)]
pub struct PresignedUrlRequest {
    pub key: String,
    pub expires_in_seconds: u64,
    pub method: PresignMethod,
    pub content_type: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PresignMethod {
    Get,
    Put,
}

/// The authoritative Storage Adapter interface.
#[async_trait]
pub trait StorageAdapter: Send + Sync {
    /// Identifier of the provider ("filesystem", "s3", "minio", "rustfs").
    fn provider_id(&self) -> &'static str;

    /// Inspect the capabilities of this adapter instance.
    fn capabilities(&self) -> StorageCapabilities;

    /// Health check / connectivity validation.
    async fn check_health(&self) -> Result<(), StorageError>;

    /// Store a binary payload using an async reader.
    async fn put_object(
        &self,
        key: &str,
        stream: ObjectStream,
        size_bytes: u64,
        mime_type: &str,
    ) -> Result<StoredObjectMetadata, StorageError>;

    /// Store a binary buffer directly (convenience for small files/thumbnails).
    async fn put_bytes(
        &self,
        key: &str,
        bytes: &[u8],
        mime_type: &str,
    ) -> Result<StoredObjectMetadata, StorageError> {
        let size = bytes.len() as u64;
        let cursor = std::io::Cursor::new(bytes.to_vec());
        self.put_object(key, Box::pin(cursor), size, mime_type).await
    }

    /// Retrieve an object stream for reading.
    async fn get_object(&self, key: &str) -> Result<ObjectStream, StorageError>;

    /// Read the full binary contents of an object (for small files/reports).
    async fn get_bytes(&self, key: &str) -> Result<Vec<u8>, StorageError>;

    /// Fetch metadata and verify existence without reading payload.
    async fn head_object(&self, key: &str) -> Result<StoredObjectMetadata, StorageError>;

    /// Check if an object exists.
    async fn exists(&self, key: &str) -> Result<bool, StorageError>;

    /// Delete an object physically from storage.
    async fn delete_object(&self, key: &str) -> Result<(), StorageError>;

    /// Copy an object within the storage provider.
    async fn copy_object(&self, source_key: &str, target_key: &str) -> Result<StoredObjectMetadata, StorageError>;

    /// Move/Rename an object.
    async fn move_object(&self, source_key: &str, target_key: &str) -> Result<StoredObjectMetadata, StorageError>;

    /// Generate a secure, time-limited presigned URL (returns Unsupported if not capable).
    async fn generate_presigned_url(
        &self,
        request: PresignedUrlRequest,
    ) -> Result<String, StorageError>;
}
```

---

## 3. Storage Providers & Concrete Implementations (`orbis-infra`)

### 3.1 Filesystem Storage Adapter (`FilesystemStorageAdapter`)
Used for Desktop local-first operation and local backend deployments.

**Key Technical Features:**
- **Base Directory Sandboxing:** Configured to an application data directory (`$APPDATA/Orbis/storage`) or an administrator-chosen custom path.
- **Path Normalization & Traversal Defense:** Strips `..`, leading slashes, and illegal Windows/POSIX characters. Enforces that all resolved paths reside strictly inside the configured base directory.
- **Atomic Writes via Staging:** Writes incoming uploads to `.tmp_{uuid}` files before atomically renaming to the target key.
- **Disk Space Pre-Allocation Check:** Verifies available volume capacity before streaming large files.

```rust
pub struct FilesystemStorageAdapter {
    base_path: std::path::PathBuf,
}
```

### 3.2 S3-Compatible Object Storage Adapter (`S3StorageAdapter`)
A single, high-performance adapter supporting:
- **Amazon S3**
- **MinIO**
- **RustFS**
- **Cloudflare R2**
- **Wasabi / Scaleway / Ceph**

**Why ONE Adapter for all S3 Providers?**
All of the above implement standard AWS S3 REST API semantics (SigV4 signing, PutObject, GetObject, HeadObject, DeleteObject, Presigned URLs). Introducing distinct adapters for MinIO and RustFS creates code duplication. Differences are handled through configuration flags:
- `force_path_style: bool` (true for MinIO and RustFS, false for AWS S3 virtual-hosted buckets).
- `custom_endpoint: Option<String>` (e.g. `http://127.0.0.1:9000` for MinIO, `http://localhost:8000` for RustFS).
- `region: String` (e.g. `us-east-1` or `auto`).

```rust
pub struct S3StorageAdapter {
    client: aws_sdk_s3::Client,
    bucket: String,
    key_prefix: Option<String>,
    capabilities: StorageCapabilities,
}
```

---

## 4. Canonical Storage Reference & Key Generation Strategy

Storage keys must be **deterministic, collision-free, provider-neutral, and tenancy-isolated**.

### 4.1 Canonical Key Hierarchy

```
org_{org_id}/[unit_{unit_id}/]{domain}/{entity_type}/{entity_id}/{file_id}_{sanitized_name}.{ext}
```

**Examples:**
- Student Passport: `org_9a1c.../unit_3b8f.../students/student/4e7d.../1a2b3c4d_passport.pdf`
- School Logo: `org_9a1c.../core/organization/9a1c.../logo_main.png`
- Generated Report: `org_9a1c.../reporting/report_job/8f1e.../grade_sheet_2026_q1.pdf`
- Template Asset: `org_9a1c.../docforge/template/5c2a.../stamp_certified.png`
- Archive Backup: `org_9a1c.../archive/yearly_archive/2025_2026/cumulative_records.zip`

### 4.2 Security Rules for Key Generation
1. Keys are generated exclusively by `FileStorageService` in the backend/Tauri core.
2. User-provided filenames are stripped of non-ASCII, path separators (`/`, `\`), and control characters.
3. Every file receives a unique `file_id` (UUIDv4) prefix to prevent overwriting files with identical names.

---

## 5. Canonical Upload Pipeline & Transaction Consistency

```
┌────────────────────────────────────────────────────────────────────────┐
│                        Canonical Upload Pipeline                       │
└────────────────────────────────────────────────────────────────────────┘
  1. Receive Upload Request (FileStream, Metadata, Tenant Context)
  2. Authorize Actor (Check IAM Permission & Tenant Ownership)
  3. Validate Magic Bytes & Sniff MIME Type (Reject spoofed binaries)
  4. Validate Size Constraints against Quotas & Limits
  5. Generate Deterministic Storage Key
  6. Stream Binary to Storage Adapter (Staged / Temp Key)
  7. Compute SHA-256 Checksum on the fly during stream
  8. Begin Database Transaction:
     - Insert Canonical Document / File Record
     - Link to Parent Entity
  9. Commit Storage Object (Atomic rename / S3 complete)
 10. Commit Database Transaction
 11. Emit Domain Event (`DomainEvent::DocumentUploaded`)
 12. Return Canonical `DocumentDto` with signed/resolved access URL
```

### 5.1 Failure Recovery & Orphan Prevention
- **If storage fails during upload:** Database transaction is never opened. Temp file deleted. No orphan records.
- **If DB transaction fails:** Storage adapter deletes the uploaded object using `delete_object(key)` in compensation handler.
- **Reconciliation Background Worker:** A low-priority maintenance job compares storage keys against active database records and removes unreferenced physical files older than 24 hours.

---

## 6. Canonical Download & Streaming Pipeline

```
┌────────────────────────────────────────────────────────────┐
│                    Client Request Access                   │
└─────────────────────────────┬──────────────────────────────┘
                              │
               ┌──────────────┴──────────────┐
               ▼                             ▼
       [Local Desktop Mode]          [Remote Server Mode]
               │                             │
    Authorize Tenant & Perms      Authorize JWT & Perms
               │                             │
    Resolve Local Storage Path     Provider Capability Check
               │                             │
    Tauri Asset Protocol /        ┌──────────┴──────────┐
    Direct Stream Reader          ▼                     ▼
                           [S3 / MinIO /      [Filesystem /
                               RustFS]        Non-Presigned]
                                  │                 │
                             Generate          Stream Bytes
                           Presigned URL      Through Backend
                           (TTL: 15 min)         via Axum
```

---

## 7. Storage Integration Configuration & Secure Credential Storage

Storage configuration is managed via the existing `IntegrationConfig` and `IntegrationSecret` entities.

### 7.1 Configuration Entity Mapping

```rust
// Stored in `integration_configs`
// category = "storage"
// provider_name = "s3" | "minio" | "rustfs" | "filesystem"
pub struct StorageSettings {
    pub provider_type: String, // "filesystem" | "s3_compatible" | "aws_s3"
    pub local_base_path: Option<String>,
    pub endpoint: Option<String>,
    pub bucket: Option<String>,
    pub region: Option<String>,
    pub force_path_style: bool,
    pub use_ssl: bool,
    pub max_upload_size_mb: u64,
}

// Stored in `integration_secrets` (Encrypted)
// config_id -> links to storage integration config
// key_name = "access_key_id", "secret_access_key"
```

### 7.2 Desktop Administration UX
- **For Local Filesystem:** Uses native OS folder picker (`app.dialog().file().pick_folder()`) to choose storage location.
- **For Remote/Object Storage:** Admin enters endpoint, bucket, credentials, and clicks "Test Connection" (`check_health()`).
- **Secret Safety:** Credentials never leak to frontend state; the UI displays masked secrets (`••••••••`).
