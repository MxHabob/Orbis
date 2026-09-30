# Orbis File Storage & Archive Implementation Plan

**Author:** Antigravity Architect & Senior Reviewer  
**Date:** September 2026  
**Status:** Canonical Implementation Roadmap (Pre-Implementation)  
**Target System:** Orbis Education Platform

---

## 1. Phased Execution Roadmap

```
Phase 1: Domain & Ports Contract Definition (orbis-domain / orbis-app)
   ↓
Phase 2: Storage Infrastructure Adapters (Filesystem & S3-Compatible)
   ↓
Phase 3: Canonical File Storage Application Service
   ↓
Phase 4: Database Migration (Schema enhancements for `documents`)
   ↓
Phase 5: Repository Implementation Updates (Soft-delete & Checksums)
   ↓
Phase 6: Axum REST API Routes & OpenAPI Registration (orbis-server)
   ↓
Phase 7: SDK Re-export & `@orbis/storage` Client Driver Package
   ↓
Phase 8: Desktop Tauri IPC Commands & AppState Integration
   ↓
Phase 9: Desktop Client Parity (LocalClient & RemoteClient)
   ↓
Phase 10: Reports & DocForge Storage Migration
   ↓
Phase 11: Student & Entity Document UI Overhaul
   ↓
Phase 12: Shared Uploaders Overhaul (AvatarUpload & LogoUpload)
   ↓
Phase 13: Enterprise Archive Center Migration (ArchiveManager)
   ↓
Phase 14: Storage Administration Settings UI
   ↓
Phase 15: Orphan Reconciliation & Background Cleanup Worker
   ↓
Phase 16: Verification & Complete Test Matrix Execution
```

---

## 2. Phase-by-Phase Technical Specifications

### Phase 1: Storage Domain & Ports Contract Definition
- Define `StorageAdapter`, `StoredObjectMetadata`, `StorageCapabilities`, `PresignedUrlRequest` in `orbis-app::ports::storage`.
- Define `FileStorageError` in `orbis-domain::errors`.
- Define `DomainEvent::DocumentArchived`, `DomainEvent::DocumentRestored`, `DomainEvent::DocumentPurged` in `orbis-domain::events`.

### Phase 2: Storage Infrastructure Adapters
- Implement `FilesystemStorageAdapter` in `orbis-infra::adapters::storage::filesystem` with sandbox path verification, atomic staging, and quota checks.
- Implement `S3StorageAdapter` in `orbis-infra::adapters::storage::s3` using `aws-sdk-s3` (supporting AWS S3, MinIO, RustFS, R2, and Wasabi via configuration).
- Implement `StorageAdapterFactory` / `StorageResolver` to dynamically resolve active adapter per organization from `integration_configs`.

### Phase 3: Canonical File Storage Application Service
- Implement `FileStorageService` in `orbis-app::services::file_storage_service`:
  - Enforces MIME validation using magic numbers.
  - Generates deterministic multi-tenant storage keys.
  - Manages upload streaming with SHA-256 calculation.
  - Manages compensation delete on database transaction failure.
  - Generates download streams or secure signed URLs.

### Phase 4: Database Migration
- Add migration `m20240108_000001_storage_and_archive_enhancements.rs`:
  - Adds `org_unit_id`, `storage_key`, `checksum_sha256`, `is_archived`, `archived_at`, `archived_by`, `archive_reason`, `retention_until`, `is_legal_hold`, `deleted_by` to `documents` table.
  - Creates indexes on `(org_id, is_archived, deleted_at)` and `(org_id, entity_type, entity_id)`.

### Phase 5: Repository Implementation Updates
- Update `DocumentRepoImpl` in `orbis-infra`:
  - `save()` handles all new lifecycle and checksum fields.
  - `find_by_org()` and `find_by_entity()` filter `WHERE deleted_at IS NULL AND is_archived = false`.
  - Add `find_archived()`, `find_trash()`, `soft_delete()`, `restore()`, `purge()`.

### Phase 6: Axum REST API Routes & OpenAPI Registration
- Create `storage_routes.rs` in `orbis-server::api`:
  - `POST /api/v1/storage/upload` (multipart stream upload).
  - `GET /api/v1/storage/:id/file` (stream binary or redirect to signed URL).
  - `GET /api/v1/storage/:id/presigned-url` (get signed download/upload URL).
- Update `document_routes.rs` with archive, restore, and trash endpoints.
- Register all routes and DTO schemas in `ApiDoc` in `orbis-server/src/lib.rs`.

### Phase 7: SDK Re-export & Upload Transport Driver (`@orbis/sdk` / `@orbis/utils`)
- Run `export_openapi` to generate complete `packages/sdk/openapi.json`.
- Regenerate `@orbis/sdk` with canonical storage and document endpoints.
- Provide `OrbisStorageUploader` in `@orbis/utils` for unified XHR/fetch progress tracking and presigned URL streaming.

### Phase 8: Desktop Tauri IPC Commands & AppState Integration
- Implement `storage_cmds.rs` in `apps/desktop/src-tauri/src/commands`:
  - `storage_upload_file`, `storage_read_file_bytes`, `storage_get_url`, `storage_pick_and_upload`.
- Update `AppState` to include `storage_adapter` and `file_storage_service`.
- Register commands in `main.rs` / `lib.rs`.

### Phase 9: Desktop Client Parity (`LocalClient` & `RemoteClient`)
- Implement all storage and document methods on `LocalClient` (via Tauri IPC) and `RemoteClient` (via `@orbis/sdk` HTTP fetch).
- Ensure identical TypeScript return types and error envelopes.

### Phase 10: Reports & DocForge Storage Migration
- Update `ReportService` to persist generated PDF/Excel documents via `FileStorageService` instead of writing raw paths to `std::env::temp_dir()`.
- Update `download_report_file` to stream from the canonical storage adapter.

### Phase 11: Student & Entity Document UI Overhaul
- Rewrite `student-documents.svelte`:
  - Replace text path input with native file picker and drag-and-drop.
  - Add upload progress indicators.
  - Display file size, MIME type badge, and preview button.

### Phase 12: Shared Uploaders Overhaul (`AvatarUpload` & `LogoUpload`)
- Connect `AvatarUpload` and `LogoUpload` to `useUploadFile()`.
- Store uploaded `document_id` and display thumbnail/preview via canonical URL resolver.

### Phase 13: Enterprise Archive Center Migration (`ArchiveManager`)
- Upgrade `archive-manager.svelte` into a dual-facet view:
  - Tab 1: **Permanent Records Archive** (searchable by cohort, year, retention status).
  - Tab 2: **Trash & Recovery Bin** (soft-deleted files with restore and purge actions).
- Add Legal Hold toggle and retention schedule indicators.

### Phase 14: Storage Administration Settings UI
- Add "Storage & Integrations" panel in Organization Settings:
  - Local Storage directory selector with native folder picker.
  - Remote S3/MinIO/RustFS configuration with "Test Connection" button.

### Phase 15: Background Orphan Reconciliation Worker
- Create a scheduled maintenance job (`OrphanReconciliationWorker`) that detects and cleans unreferenced physical files older than 24 hours.

### Phase 16: Verification & Complete Test Matrix Execution
- Run unit, integration, end-to-end, and security tests as defined in the Test Matrix below.

---

## 3. Comprehensive Verification & Test Matrix

### 3.1 Filesystem Provider Tests
- [ ] Direct file upload with progress tracking.
- [ ] Atomic file write verification (no corrupt files on aborted upload).
- [ ] Path traversal protection (rejection of `../../etc/passwd` or `..\Windows`).
- [ ] Disk quota overflow handling.
- [ ] Read and streaming verification for large files (100MB+).

### 3.2 S3 / MinIO / RustFS Provider Tests
- [ ] S3 upload and download via standard SigV4.
- [ ] MinIO integration with `force_path_style: true`.
- [ ] RustFS standalone integration.
- [ ] Presigned GET and PUT URL expiration testing.
- [ ] Connection failure recovery and retry backoff.

### 3.3 Desktop & UI Tests
- [ ] Native OS file picker dialog integration.
- [ ] Native OS folder picker for local storage configuration.
- [ ] Drag-and-drop file upload in `StudentDocuments` and `ArchiveManager`.
- [ ] Avatar and logo instant local preview + background upload.
- [ ] Multi-file concurrent upload with cancellation support.

### 3.4 Local vs Remote Parity Tests
- [ ] Exact same metadata returned in Local mode (Tauri) and Remote mode (HTTP).
- [ ] Exact same validation rules enforced across both transports.
- [ ] Exact same error codes and error messages.

### 3.5 Archive & Retention Tests
- [ ] Archiving an active document preserves searchability in Archive view.
- [ ] Restoring an archived document returns it to active views.
- [ ] Soft deletion hides document from regular queries and shows in Trash.
- [ ] Restoring from Trash restores document to active state.
- [ ] Purging permanently deletes database row and physical storage object.
- [ ] Legal hold prevents deletion and purging.

### 3.6 Security & Tenancy Tests
- [ ] Tenant A cannot read, write, or delete Tenant B's files.
- [ ] Org-unit scoped user cannot access other org-unit documents.
- [ ] Malicious file extension spoofing (e.g. `.exe` renamed to `.pdf`) detected and rejected via magic bytes inspection.
- [ ] Storage credentials never leaked in logs, API responses, or frontend state.
