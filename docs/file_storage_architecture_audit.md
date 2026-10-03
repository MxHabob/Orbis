# Orbis File Storage, Upload & Archive System — Deep Architecture Audit

**Author:** Antigravity Architect & Senior Reviewer  
**Date:** September 2026  
**Status:** Audit & Analysis Complete (Pre-Implementation)  
**Target System:** Orbis Education Platform (Desktop / Web / Backend)

---

## 1. Executive Summary

This document presents a comprehensive, repository-wide architectural audit of all file storage, file upload, document management, attachment, reporting artifacts, bulk import/export, and archive subsystems in Orbis.

### Core Audit Finding
Orbis currently possesses **fragmented, metadata-only file abstractions** without a unified binary storage pipeline. Specifically:
1. **Metadata vs Binary Disconnect:** The backend `DocumentService` and database `documents` table record metadata only (e.g. `file_path`, `storage_provider`, `mime_type`), assuming the physical file was already transferred out-of-band.
2. **Frontend Mock/Direct Path Assumptions:** Desktop components (`AvatarUpload`, `LogoUpload`, `ArchiveManager`) pass raw `File` objects expecting a multipart upload API, while `StudentDocuments` prompts the user for arbitrary path strings.
3. **OpenAPI & SDK Gaps:** `document_routes` exists in `orbis-server` but was omitted from the OpenAPI spec macro (`ApiDoc`), preventing `@orbis/sdk` and `LocalClient`/`RemoteClient` from providing typed document transport methods.
4. **Report & Export Storage Isolation:** Report generation in `report_routes.rs` and `report_cmds.rs` directly writes files to host OS temp directories (`std::env::temp_dir().join("orbis_reports")`), bypassing all storage provider abstractions.
5. **Archive Conceptual Confusion:** The "Archive" feature in `archive-manager.svelte` is currently a UI filter over `documents` with `entity_type: "archive"`, lacking domain lifecycle management, soft-deletion policies, physical storage tiering, or legal retention rules.
6. **Hard Deletion & Orphan File Risk:** Deletion in `DocumentRepoImpl` executes immediate database `DELETE`, disregarding the existing `deleted_at` column and creating guaranteed physical file orphans.

This audit establishes the blueprint to consolidate all storage operations into **one canonical Storage Integration + Storage Adapter architecture**.

---

## 2. Comprehensive Current-State System Map

### 2.1 Component Flow & Ownership Matrix

| Feature / Subsystem | UI Component / Caller | Desktop Command (Tauri) | Client Interface (`ApiClient`) | Backend Route (Axum) | Application Service | Domain Entity / Port | Database Table | Physical Storage Mechanism |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **Student / Entity Documents** | `StudentDocuments` / `DocumentsSection` | `document_upload`, `document_get`, `document_list_by_entity`, `document_delete` | `api.listDocumentsByEntity`, `api.uploadDocument` *(unimplemented in client)* | `POST /api/v1/documents`, `GET /api/v1/documents` | `DocumentService` (`orbis-app`) | `Document`, `DocumentRepository` | `documents` | None (stores user-provided path string) |
| **Document Requirements** | `StudentDocuments`, `RegistrationSettingsForm` | `document_requirement_*` | `api.listDocumentRequirements`, `api.evaluateStudentDocumentRequirements` | `/api/v1/document-requirements`, `/api/v1/students/:id/document-requirements` | `DocumentService` (`orbis-app`) | `DocumentRequirement`, `DocumentRequirementRepository` | `document_requirements` | N/A (Rules Engine only) |
| **Digital Archive Repository** | `ArchiveManager`, `archive/+page.svelte` | `document_upload`, `document_delete`, `document_list` | `api.uploadDocument`, `api.listDocuments` | `POST /api/v1/documents`, `GET /api/v1/documents` | `DocumentService` (`orbis-app`) | `Document`, `DocumentRepository` | `documents` (filtered by `entity_type: "archive"`) | None (orphaned binary in UI) |
| **User & Student Avatars** | `AvatarUpload` (`avatar-upload.svelte`) | `document_upload` | `api.uploadDocument` | `POST /api/v1/documents` | `DocumentService` (`orbis-app`) | `Document` | `documents` | Unhandled (FileReader dataURL local preview only) |
| **Organization Logos** | `LogoUpload` (`logo-upload.svelte`) | `document_upload` | `api.uploadDocument` | `POST /api/v1/documents` | `DocumentService` (`orbis-app`) | `Document` | `documents` | Unhandled (FileReader dataURL local preview only) |
| **Report Generation & Print Artifacts** | `ReportingWorkspace`, `PrintDialog` | `generate_report`, `download_report_file` | `api.generateReport`, `api.downloadReportFile` | `POST /api/v1/reports/generate`, `GET /api/v1/reports/jobs/:id/download` | `ReportService` (`orbis-app`) | `ReportJob`, `ReportJobRepository` | `report_jobs` (`output_path`) | Hardcoded OS Temp Directory (`std::env::temp_dir().join("orbis_reports")`) |
| **Template Canvas Assets** | `AssetPanel` (`asset-panel.svelte`) | `document_upload`, `document_list` | `api.listDocuments` (`entityType: "template"`) | `GET /api/v1/documents` | `DocumentService` (`orbis-app`) | `Document` | `documents` | `assetRef` to document ID; URL resolved via `convertFileSrc` |
| **Bulk Data Import / Export** | `BulkEntry` (`file-parser.ts`, `exporter.ts`) | Direct Tauri Plugins (`dialog`, `fs`, `opener`) | Bypasses `ApiClient` | None (pure client-side execution) | None (Tauri client file I/O) | None | None | User-selected filesystem paths |
| **Database Backups & Restore** | `DatabaseManager`, `DatabaseSettings` | `database_backup_online`, `database_restore_from_backup`, `database_pick_*` | `api.databaseBackupOnline`, `api.databasePickSavePath` | None (Desktop local-only) | `database_manager` (`src-tauri`) | `ReportAuditEntry` (audit only) | `report_audit_log` | Host filesystem path selected via `tauri-plugin-dialog` |
| **Storage Helper Package** | `packages/storage/uploader.ts` | None | None | None | None | None | None | Standalone XHR/Fetch uploader with progress (unused) |

---

## 3. Deep Architectural Inspection by Layer

### 3.1 Database Schema & Data Models (`orbis-infra`)

#### The `documents` Table
Inspected in [models/document.rs](file:///d:/projects/Orbis/apps/api-rust/crates/orbis-infra/src/models/document.rs) and [migration/m20240101_000001_initial_schema.rs](file:///d:/projects/Orbis/apps/api-rust/crates/orbis-infra/src/migration/m20240101_000001_initial_schema.rs#L74):

```rust
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "documents")]
pub struct Model {
    pub id: Uuid,
    pub org_id: Uuid,
    pub uploaded_by: Option<Uuid>,
    pub entity_type: Option<String>,
    pub entity_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub category: String,
    pub asset_type: String,
    pub school_year: Option<String>,
    pub term: Option<String>,
    pub tags: Option<String>,
    pub visibility: String,
    pub file_size_bytes: Option<i64>,
    pub mime_type: Option<String>,
    pub extension: Option<String>,
    pub storage_provider: String,
    pub file_path: String,
    pub url: Option<String>,
    pub thumbnail_path: Option<String>,
    pub preview_path: Option<String>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}
```

**Schema Strengths:**
- Multi-tenancy is enforced via `org_id`.
- Polymorphic association via `entity_type` + `entity_id` accommodates students, employees, organizations, templates, and archives.
- Tracks MIME type, file size, extensions, and timestamps.

**Schema Flaws & Gaps:**
1. **Missing Checksum/Hash:** No `sha256` or `checksum` column to ensure binary integrity, detect duplicates, or verify physical object consistency.
2. **Ambiguous `file_path` vs `url`:** `file_path` contains raw host filesystem paths, relative paths, or arbitrary strings with no canonical URI schema.
3. **No Storage Key / Namespace Separation:** Does not distinguish between physical object key, bucket/container name, and logical storage namespace.
4. **No Versioning or Retention Fields:** Lacks `version_id`, `is_current`, `retention_until`, `is_archived`, `archived_at`, `archived_by`.
5. **Missing Org-Unit Scope:** No `org_unit_id` column for branch-level document scoping and permission isolation.

#### The `document_requirements` Table
Inspected in [models/document_requirement.rs](file:///d:/projects/Orbis/apps/api-rust/crates/orbis-infra/src/models/document_requirement.rs) and [migration/m20240105_000001_registration_schema.rs](file:///d:/projects/Orbis/apps/api-rust/crates/orbis-infra/src/migration/m20240105_000001_registration_schema.rs):
- Cleanly models organizational prerequisites (e.g. Birth Certificate, National ID, Transfer Certificate) targeted by `stage`, `grade_level`, `program`, or `global` with conditional rules (`non_national_only`, `transfer_students_only`, etc.).
- Evaluated properly by `DocumentService::evaluate_student_requirements()`.

#### The `report_jobs` Table
Inspected in [models/report.rs](file:///d:/projects/Orbis/apps/api-rust/crates/orbis-infra/src/models/report.rs#L131-L160):
- Stores `output_path: Option<String>` and `output_size_bytes: Option<i64>`.
- Stores host-specific absolute paths such as `/tmp/orbis_reports/8e4...pdf`, making report files unportable and non-scalable across distributed nodes or object storage.

---

### 3.2 Application Services & Ports (`orbis-app`)

#### `DocumentService`
Inspected in [services/document_service.rs](file:///d:/projects/Orbis/apps/api-rust/crates/orbis-app/src/services/document_service.rs):
- `create_document()`: Validates permissions (`Permission::DocumentWrite`), generates a new `Uuid`, populates `Document` struct from `CreateDocumentInput`, and calls `document_repo.save()`.
- Publishes `DomainEvent::DocumentUploaded`, `DomainEvent::DocumentUpdated`, `DomainEvent::DocumentDeleted`.
- **Architectural Defect:** `DocumentService` does not have access to any `StorageAdapter` or binary stream handler. It cannot read, write, verify, or stream actual file bytes.
- **Deletion Defect:** `delete_document()` calls `self.document_repo.delete(id)` which executes an SQL hard delete:
  ```rust
  DocumentEntity::delete_by_id(id).exec(&self.db).await...
  ```
  This ignores `doc.deleted_at` soft-delete and never cleans up the physical storage file.

---

### 3.3 REST API & OpenAPI Contract (`orbis-server`)

#### `document_routes.rs`
Inspected in [api/document_routes.rs](file:///d:/projects/Orbis/apps/api-rust/crates/orbis-server/src/api/document_routes.rs):
- `POST /api/v1/documents` expects `UploadDocumentRequest` JSON body.
- It is NOT a multipart upload endpoint. It cannot receive binary file streams or file buffers.
- **OpenAPI Exclusion Bug:** In [lib.rs](file:///d:/projects/Orbis/apps/api-rust/crates/orbis-server/src/lib.rs#L13-L570):
  - `document_routes` is merged in the Axum router (`.merge(document_routes::routes())`),
  - BUT is omitted from `#[derive(OpenApi)] paths(...)` and `components(schemas(...))`.
  - When `export_openapi` runs to generate `packages/sdk/openapi.json`, all document routes are absent from the spec.

---

### 3.4 Desktop & Tauri Integration (`apps/desktop/src-tauri`)

#### `document_cmds.rs`
Inspected in [commands/document_cmds.rs](file:///d:/projects/Orbis/apps/desktop/src-tauri/src/commands/document_cmds.rs):
- Implements `document_upload`, `document_update`, `document_list`, `document_get`, `document_delete`, `document_list_by_entity`.
- Like the REST API, it accepts JSON metadata and delegates to `AppState.document_service`.
- No binary byte stream or file chunking IPC command exists.

#### Tauri Capabilities & Permissions
Inspected in [tauri.conf.json](file:///d:/projects/Orbis/apps/desktop/src-tauri/tauri.conf.json) and [capabilities/default.json](file:///d:/projects/Orbis/apps/desktop/src-tauri/capabilities/default.json):
- `fs:allow-read-file`, `fs:allow-write-file`, `fs:allow-exists` are enabled in `default.json`.
- `assetProtocol` is enabled with scope `["$APPDATA/**", "$LOCALAPPDATA/**", "$RESOURCE/**", "$TEMP/**"]`.
- `dialog:allow-ask`, `dialog:allow-message` are enabled in `desktop.json`.
- **Finding:** Tauri has the necessary capabilities, but individual UI components currently call `@tauri-apps/plugin-fs` and `@tauri-apps/plugin-dialog` directly in an ad-hoc fashion instead of going through a centralized storage driver.

---

### 3.5 Desktop Frontend Client & Queries (`apps/desktop/src`)

#### `ApiClient` Transport Gap
Inspected in [lib/api/types.ts](file:///d:/projects/Orbis/apps/desktop/src/lib/api/types.ts), [lib/api/local-client.ts](file:///d:/projects/Orbis/apps/desktop/src/lib/api/local-client.ts), [lib/api/remote-client.ts](file:///d:/projects/Orbis/apps/desktop/src/lib/api/remote-client.ts):
- Because OpenAPI lacked document routes, `LocalClient` and `RemoteClient` had no generated document methods.
- The `api` proxy in [lib/api/client.ts](file:///d:/projects/Orbis/apps/desktop/src/lib/api/client.ts) returns `undefined` for `api.uploadDocument`, `api.listDocuments`, etc., creating runtime failures when called from `queries.ts`.

#### Path Resolution in `queries.ts`
Inspected in [features/documents/queries.ts](file:///d:/projects/Orbis/apps/desktop/src/lib/features/documents/queries.ts#L30-L90):
- `resolveDocumentUrl()` attempts manual URL formatting:
  - If HTTP/HTTPS/data/blob/asset URL, returns as-is.
  - If absolute Windows (`C:\...`) or POSIX (`/...`) path, runs `convertFileSrc(path)`.
  - If relative storage path, prepends `join(appDataDir(), "storage", relativePath)` and converts.
- **Architectural Flaw:** Exposes physical filesystem structure to frontend client code and fails if storage is remote or object-based.

---

## 4. Current Archive System Audit

### 4.1 What "Archive" Currently Means in Orbis
In the current codebase, "Archive" is merely a UI page ([archive-manager.svelte](file:///d:/projects/Orbis/apps/desktop/src/lib/features/settings/archive-manager.svelte)) that lists `documents` where `entity_type == "archive"` or `entity_type == "student"`.

### 4.2 Identified Archive Deficiencies
1. **No Domain Archival Lifecycle:** No aggregate root in `orbis-domain` tracks archival events, retention schedules, reason for archiving, or archival authority.
2. **Mixed Concepts:** Conflates:
   - *Document storage* (uploading school year photos, event videos, certificates).
   - *Entity lifecycle state* (archiving inactive students or former employees).
   - *System backup / cold storage* (moving data off primary database).
3. **No Restoration Workflow:** No API or command exists to restore an archived document, unarchive an entity, or inspect previous entity versions.
4. **No Storage Tiering:** Files in the "archive" reside in the same physical storage and same database table as active records with no migration to colder/cheaper storage classes.
5. **No Audit Trail for Archival Actions:** Archiving or deleting archive items does not log legal retention audit events.

---

## 5. Security & Isolation Audit

| Security Domain | Current Status | Risk Level | Finding & Vulnerability |
| :--- | :--- | :--- | :--- |
| **Tenant Isolation** | Partial | **HIGH** | `documents` table has `org_id`, but `report_jobs.output_path` and `resolveDocumentUrl` allow arbitrary local file paths across organization boundaries. |
| **Org-Unit Isolation** | Missing | **HIGH** | `documents` table has no `org_unit_id`. Users with access to one branch can view all documents across the entire organization. |
| **Path Traversal** | Vulnerable | **CRITICAL** | `UploadDocumentRequest` and `CreateDocumentInput` accept raw user-supplied `file_path` without sanitization, directory traversal checking (`../`), or normalization. |
| **Malicious File Execution** | Vulnerable | **HIGH** | No MIME type sniffing / magic number verification. File extension is trusted directly from client payload. |
| **Credential Safety** | Good (for existing integrations) | **LOW** | `integration_secrets` encrypts third-party keys. Storage provider credentials must follow this existing pattern. |
| **Storage Access Authorization** | Incomplete | **HIGH** | Files served via Tauri asset protocol (`asset://`) or direct file paths bypass domain permission checks once a path is obtained. |

---

## 6. Audit Verdict

```
CURRENT FILE STORAGE SYSTEM STATUS:
[X] FRAGMENTED & INCONSISTENT
[X] MISSING BINARY TRANSPORT ADAPTERS
[X] OPENAPI SPEC REGISTRATION GAPS
[X] ORPHAN FILE RISK ON DELETION
[X] ARBITRARY PATH SECURITY VULNERABILITIES
```

**Conclusion:** The codebase requires a clean, canonical Storage Integration and Storage Adapter layer implemented across `orbis-domain`, `orbis-app`, `orbis-infra`, `orbis-server`, `apps/desktop`, and `@orbis/sdk`.
