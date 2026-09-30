# Orbis File Archive & Retention Architecture

**Author:** Antigravity Architect & Senior Reviewer  
**Date:** September 2026  
**Status:** Canonical Target Architecture Design (Pre-Implementation)  
**Target System:** Orbis Education Platform (Desktop / Web / Backend)

---

## 1. Domain Separation of Archival Concepts

In enterprise educational software, "Archive" cannot be a single conflated feature. Orbis establishes a rigorous 4-tier separation:

```
┌────────────────────────────────────────────────────────────────────────┐
│ 1. Business / Domain Entity Archiving                                 │
│    • Inactive students (graduated, transferred, expelled)              │
│    • Closed academic years & historical periods                        │
│    • Historical fee structures & grade books                           │
│    • State: Entity is marked `is_archived: true`                       │
└──────────────────────────────────┬─────────────────────────────────────┘
                                   │
┌──────────────────────────────────▼─────────────────────────────────────┐
│ 2. Document & File Archiving                                           │
│    • Permanent student portfolios, transcripts, health records         │
│    • Certified graduation certificates and government submissions      │
│    • Immutable historical snapshots with retention schedules           │
│    • State: Document is marked `is_archived: true`, has retention policy│
└──────────────────────────────────┬─────────────────────────────────────┘
                                   │
┌──────────────────────────────────▼─────────────────────────────────────┐
│ 3. Soft Deletion & Trash Lifecycle                                     │
│    • Erroneous uploads or user-deleted files                           │
│    • State: `deleted_at: Some(timestamp)`                              │
│    • 30-day recovery grace period before permanent physical purge      │
└──────────────────────────────────┬─────────────────────────────────────┘
                                   │
┌──────────────────────────────────▼─────────────────────────────────────┐
│ 4. Physical Storage Lifecycle & Tiering                                │
│    • Hot Storage (Active local disk / S3 Standard)                     │
│    • Warm/Cold Storage (Infrequent Access / S3 Glacier / MinIO cold)   │
│    • Orchestrated by infrastructure, transparent to business logic     │
└────────────────────────────────────────────────────────────────────────┘
```

---

## 2. Archive Domain Model & Entity Extensions

### 2.1 Canonical Document Model Enhancements

To support enterprise archival and retention, the `documents` table and `Document` domain entity are enhanced with formal lifecycle fields:

```rust
pub struct Document {
    pub id: Uuid,
    pub org_id: Uuid,
    pub org_unit_id: Option<Uuid>,      // Branch/Campus scoping
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
    pub checksum_sha256: Option<String>, // Binary integrity verification
    pub storage_provider: String,        // "filesystem", "s3", etc.
    pub storage_key: String,             // Canonical provider-neutral storage key
    pub url: Option<String>,
    pub thumbnail_path: Option<String>,
    pub preview_path: Option<String>,

    // ── Archival & Retention State ─────────────────────────────
    pub is_archived: bool,
    pub archived_at: Option<DateTime<Utc>>,
    pub archived_by: Option<Uuid>,
    pub archive_reason: Option<String>,
    pub retention_policy_id: Option<Uuid>,
    pub retention_until: Option<DateTime<Utc>>,
    pub is_legal_hold: bool,

    // ── Soft Deletion State ────────────────────────────────────
    pub deleted_at: Option<DateTime<Utc>>,
    pub deleted_by: Option<Uuid>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

### 2.2 Retention Policies (`retention_policies`)

Organizations can define automated retention schedules aligned with national educational regulations:

```rust
pub struct RetentionPolicy {
    pub id: Uuid,
    pub org_id: Uuid,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub category: String,               // "student_records", "financial", "exam_papers"
    pub retention_years: i32,           // e.g. 5, 10, 50 (permanent)
    pub auto_archive_after_days: i32,   // automatically transition to archive
    pub auto_delete_after_retention: bool,
    pub requires_legal_review: bool,
    pub is_system_default: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

---

## 3. Archive & Restoration Workflow

### 3.1 Archiving a Document
1. User with `Permission::DocumentArchive` requests document archival (`archive_document(doc_id, reason)`).
2. Service validates that the document is not currently locked or in draft state.
3. Sets `is_archived = true`, `archived_at = Utc::now()`, `archived_by = actor.id()`, `archive_reason`.
4. Calculates `retention_until` based on the document category retention policy.
5. Emits `DomainEvent::DocumentArchived`.
6. Document remains searchable in Archive views with full audit metadata.

### 3.2 Restoring an Archived Document
1. User with `Permission::DocumentRestore` requests unarchiving.
2. If `is_legal_hold` is active, operation is blocked unless user has legal hold authority.
3. Sets `is_archived = false`, resets archival timestamps, logs unarchive event.
4. Emits `DomainEvent::DocumentUnarchived`.
5. Document returns to active views.

### 3.3 Soft Deletion vs Permanent Purge
- **Soft Deletion (`delete_document`):**
  - Sets `deleted_at = Some(now)`, `deleted_by = Some(actor)`.
  - Document disappears from normal queries (`WHERE deleted_at IS NULL`).
  - Stored in "Trash" view for 30 days.
  - Can be restored via `restore_document()`.
- **Permanent Purge (`purge_document`):**
  - Only permitted after soft-delete grace period or by SuperAdmin with `Permission::DocumentPurge`.
  - Blocks purge if `is_legal_hold == true` or `retention_until > now()`.
  - Physically deletes binary from storage adapter: `storage_adapter.delete_object(doc.storage_key)`.
  - Permanently removes database record.
  - Records immutable audit entry in `report_audit_log`.

---

## 4. Archive UI & Workspace Experience

The current `archive-manager.svelte` is upgraded to the **Enterprise Archive Center**:

1. **Dual Facets:**
   - **Records Archive:** Official student folders, graduation portfolios, staff files, past academic year books.
   - **Trash / Recovery Bin:** Soft-deleted items pending permanent deletion.
2. **Authoritative Filters:**
   - Filter by Academic Year, Cohort, Grade Level, and Class.
   - Filter by Archival Reason and Retention Expiration Date.
   - Legal Hold badges with lock indicators.
3. **Actions:**
   - Preview, Download, Batch Export to ZIP, Restore to Active, Apply Legal Hold.
