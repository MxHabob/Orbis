# Orbis File & Storage Reconciliation Matrix

**Author:** Antigravity Architect & Senior Reviewer  
**Date:** September 2026  
**Status:** Architecture Specification (Pre-Implementation)  
**Target System:** Orbis Education Platform

---

## 1. Subsystem Reconciliation Table

| Current Implementation | Current Owner | Current Storage Mechanism | Problem Identified | Target Owner | Target Contract | Action |
| :--- | :--- | :--- | :--- | :--- | :--- | :---: |
| **`DocumentService`** (`orbis-app`) | `orbis-app::services` | Database metadata record only | Does not handle physical binary payload; hard-deletes DB rows leaving orphan files. | `FileStorageService` + `DocumentService` | `FileStorageService::upload_file()`, `DocumentService::create_document()` with storage key | **REBUILD** |
| **`DocumentRepoImpl`** (`orbis-infra`) | `orbis-infra::repositories` | SeaORM SQL queries | Uses `Entity::delete_by_id(id)` ignoring `deleted_at` soft deletion. Missing `checksum_sha256`, `storage_key`, `org_unit_id`. | `DocumentRepoImpl` | Updates schema & queries to support soft delete and checksum fields | **REBUILD** |
| **`document_routes.rs`** (`orbis-server`) | `orbis-server::api` | Axum routes (`POST /api/v1/documents`) | Receives metadata-only JSON; omitted from `ApiDoc::openapi()` macro; no binary upload/download endpoints. | `storage_routes.rs` + `document_routes.rs` | Multipart/Stream upload (`POST /api/v1/storage/upload`), Download (`GET /api/v1/storage/:id/file`), OpenAPI registered | **REBUILD** |
| **`document_cmds.rs`** (`src-tauri`) | `apps/desktop/src-tauri` | Tauri IPC commands | Accepts JSON metadata only; no binary chunking or stream command. | `storage_cmds.rs` + `document_cmds.rs` | Tauri commands for upload bytes/file, get file stream, and register in `LocalClient` | **REBUILD** |
| **Report Temp File Generation** | `ReportService` & `report_routes.rs` / `report_cmds.rs` | Direct writes to `std::env::temp_dir().join("orbis_reports")` | Hardcodes host OS temp path; unportable across distributed nodes; bypasses storage provider abstraction. | `ReportService` + `StorageAdapter` | Generates report bytes and stores via canonical `StorageAdapter::put_bytes()` under `reporting/` prefix | **REPLACE** |
| **`AvatarUpload`** (`avatar-upload.svelte`) | `apps/desktop/src/lib/shared` | Raw `File` object passed to metadata mutation | Expects multipart upload endpoint returning `{ url }`; fails at runtime against metadata-only API. | `AvatarUpload` | Calls canonical `api.uploadFile()` / `useUploadFile()`, stores `document_id` reference | **REBUILD** |
| **`LogoUpload`** (`logo-upload.svelte`) | `apps/desktop/src/lib/shared` | Raw `File` object passed to metadata mutation | Fails at runtime against metadata-only API. | `LogoUpload` | Calls canonical `api.uploadFile()` with `category: "logo"`, stores `document_id` | **REBUILD** |
| **`StudentDocuments`** (`student-documents.svelte`) | `apps/desktop/src/lib/features/students` | User prompted to type path strings into text inputs | Anti-pattern: expects users to manually type URLs/paths; no physical file selection or upload. | `StudentDocuments` | Native OS file picker dialog, progress bar, canonical upload pipeline | **REBUILD** |
| **`ArchiveManager`** (`archive-manager.svelte`) | `apps/desktop/src/lib/features/settings` | Document query filtered by `entity_type == "archive"` | Lacks retention rules, soft delete vs purge, domain archival workflow, and legal hold. | `ArchiveManager` | Dual-view Enterprise Archive (Records Archive + Recovery Bin) backed by `ArchiveService` | **REBUILD** |
| **`AssetPanel`** (`asset-panel.svelte`) | `apps/desktop/src/lib/features/templates` | `useDocuments({ entityType: "template" })` | Manually formats URLs using fragile string slicing; no binary upload handler. | `AssetPanel` | Integrates with canonical `useUploadFile()`, drag-drops `assetRef` with canonical URL resolution | **REPAIR** |
| **Bulk Import / Export** (`file-parser.ts`, `exporter.ts`) | `apps/desktop/src/lib/features/bulk-entry` | Direct Tauri dialog/fs/opener plugins | Client-side user spreadsheet imports/exports. | `BulkEntryEngine` | Valid client-side file interaction; encapsulate under typed `DesktopFileDriver` | **KEEP** |
| **Database Backups** (`database_cmds.rs`) | `apps/desktop/src-tauri` | SQLite backup via `tauri_plugin_dialog` | Specific to database lifecycle management. | `DatabaseManager` | Verified healthy; separate from general file storage domain. | **KEEP** |
| **`packages/storage/uploader.ts`** | `packages/storage` | Standalone XHR/Fetch uploader | Disconnected from backend contracts and SDK; creates unnecessary package overhead. | `@orbis/utils` | Consolidated into `@orbis/utils` (`OrbisStorageUploader`); redundant package removed | **REMOVE** |
| **`resolveDocumentUrl`** (`queries.ts`) | `apps/desktop/src/lib/features/documents` | Heuristic path resolution (`convertFileSrc`, `appDataDir`) | Leaks physical filesystem details to frontend; breaks on remote S3 storage. | `storage-resolver.ts` | Server-issued signed URL (Remote) or Tauri Asset Protocol URI (Local) | **REPLACE** |
| **`ApiClient` Transport** (`local-client.ts`, `remote-client.ts`) | `apps/desktop/src/lib/api` | Missing document/storage methods | Calls return `undefined` causing runtime errors. | `ApiClient` | Implement complete typed storage and document methods on both `LocalClient` and `RemoteClient` | **ADD** |

---

## 2. Action Summary
- **REBUILD:** 7 items (Core domain, repositories, routes, commands, UI uploaders, archive center)
- **REPLACE:** 2 items (Report temp file generation, client-side path resolution)
- **ADD:** 1 item (`ApiClient` storage & document methods)
- **REPAIR:** 1 item (`AssetPanel`)
- **KEEP:** 2 items (Bulk entry spreadsheet parser, Database backups)
- **REMOVE:** 1 item (`packages/storage` redundant package - consolidated into `@orbis/utils`)
