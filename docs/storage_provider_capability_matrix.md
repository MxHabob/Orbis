# Orbis Storage Provider Capability Matrix & Protocol Analysis

**Author:** Antigravity Architect & Senior Reviewer  
**Date:** September 2026  
**Status:** Architecture Specification (Pre-Implementation)  
**Target System:** Orbis Education Platform

---

## 1. Storage Provider Capability Matrix

| Storage Provider | Adapter Implementation | Presigned GET URLs | Presigned PUT Uploads | Streaming / Range Reads | Multipart Chunking | Atomic Rename / Move | Direct Local Path (Tauri Asset) | Force Path Style Required |
| :--- | :--- | :---: | :---: | :---: | :---: | :---: | :---: | :---: |
| **Local Filesystem** (Windows/Linux/macOS) | `FilesystemStorageAdapter` | ❌ (N/A) | ❌ (N/A) | ✅ Direct async I/O | ✅ Via staging chunks | ✅ `fs::rename` (OS atomic) | ✅ `asset://` or file stream | N/A |
| **Amazon S3** | `S3StorageAdapter` | ✅ (SigV4) | ✅ (SigV4) | ✅ HTTP Range | ✅ S3 Multipart API | ❌ (Copy + Delete) | ❌ (Streamed/Signed) | ❌ (DNS Virtual Bucket) |
| **MinIO** (Self-hosted / Private Cloud) | `S3StorageAdapter` | ✅ (SigV4) | ✅ (SigV4) | ✅ HTTP Range | ✅ S3 Multipart API | ❌ (Copy + Delete) | ❌ (Streamed/Signed) | ✅ (`force_path_style: true`) |
| **RustFS** (High-perf Rust Object Store) | `S3StorageAdapter` | ✅ (SigV4) | ✅ (SigV4) | ✅ HTTP Range | ✅ S3 Multipart API | ❌ (Copy + Delete) | ❌ (Streamed/Signed) | ✅ (`force_path_style: true`) |
| **Cloudflare R2** | `S3StorageAdapter` | ✅ (SigV4) | ✅ (SigV4) | ✅ HTTP Range | ✅ S3 Multipart API | ❌ (Copy + Delete) | ❌ (Streamed/Signed) | ❌ (Account subdomain) |
| **Wasabi / Scaleway / Ceph** | `S3StorageAdapter` | ✅ (SigV4) | ✅ (SigV4) | ✅ HTTP Range | ✅ S3 Multipart API | ❌ (Copy + Delete) | ❌ (Streamed/Signed) | Configurable |

---

## 2. MinIO and RustFS S3-Compatibility Evaluation

### 2.1 Protocol Analysis: MinIO
MinIO is a high-performance, S3-API compatible object storage suite. It strictly implements the AWS S3 REST API specification, including:
- Amazon S3 Signature Version 4 (SigV4) authentication.
- Standard S3 XML error responses and HTTP headers.
- Standard Multipart Upload (`CreateMultipartUpload`, `UploadPart`, `CompleteMultipartUpload`).
- Standard Presigned GET/PUT URLs.

**Conclusion:** MinIO requires **no custom adapter logic**. It is 100% covered by `S3StorageAdapter` using:
- `custom_endpoint: "http://<minio-host>:9000"`
- `force_path_style: true`
- `region: "us-east-1"` (or configured MinIO region).

### 2.2 Protocol Analysis: RustFS
RustFS is an ultra-lightweight, memory-safe S3-compatible object store written in Rust. It exposes standard S3 REST endpoints for standalone and embedded edge deployments.
- RustFS implements standard S3 PutObject, GetObject, HeadObject, DeleteObject, and SigV4 authentication.

**Conclusion:** RustFS requires **no separate business implementation**. It runs natively through `S3StorageAdapter` using:
- `custom_endpoint: "http://<rustfs-host>:<port>"`
- `force_path_style: true`
- `region: "auto"`.

---

## 3. Platform & Filesystem Nuances (Windows / macOS / Linux)

The `FilesystemStorageAdapter` accounts for OS-level variations:

### 3.1 Windows Path Specifics
- **Case Insensitivity:** Windows paths are case-insensitive; the adapter enforces lowercased UUID keys to guarantee cross-platform consistency.
- **Path Separators:** All internal storage keys strictly use POSIX `/` forward slashes. The filesystem adapter converts to OS-native separators (`\`) exclusively at the physical I/O boundary via `PathBuf::join()`.
- **Locked File Handles:** On Windows, open file descriptors prevent file deletion or renaming. The adapter uses RAII stream drops and `tokio::fs` handle management before attempting atomic replacements.
- **Drive Letters & UNC Paths:** Sandboxing enforces that the target path begins with the validated base directory prefix and forbids arbitrary drive switches (`D:\`, `\\server\share`).

### 3.2 POSIX Specifics (macOS & Linux)
- Permissions: Newly created directories use `0o750` permissions (restricted to the service user/group).
- Atomic File Replacement: Utilizes standard POSIX `rename(2)` atomicity.
