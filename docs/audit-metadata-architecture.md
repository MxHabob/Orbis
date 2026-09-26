# Orbis — Audit Metadata & Trail Architecture

> **Version:** 1.0.0
> **Status:** PLAN — NOT FOR EXECUTION
> **Based on:** Full repository audit of `d:/projects/Orbis` on 2026-09-26

---

## Table of Contents

1. [Current State Analysis](#1-current-state-analysis)
2. [Target Architecture](#2-target-architecture)
3. [Entity Metadata Matrix](#3-entity-metadata-matrix)
4. [Audit Metadata Design](#4-audit-metadata-design)
5. [Audit Trail Architecture](#5-audit-trail-architecture)
6. [Execution Context Design](#6-execution-context-design)
7. [Data Scope Strategy](#7-data-scope-strategy)
8. [Permission Integration](#8-permission-integration)
9. [Concurrency Control](#9-concurrency-control)
10. [Soft Delete Strategy](#10-soft-delete-strategy)
11. [Notes Strategy](#11-notes-strategy)
12. [Security Considerations](#12-security-considerations)
13. [Performance Strategy](#13-performance-strategy)

---

## 1. Current State Analysis

### 1.1 What Exists

#### RequestContext (`orbis-app/src/context.rs`)
```rust
pub struct RequestContext {
    pub tenant_id: Uuid,    // org_id from JWT claims
    pub actor: Actor,       // User(Uuid) | Agent(Uuid) | Autopilot | System
    pub request_id: String, // Generated per-request (UUID)
}
```

**Assessment:** Minimal but sound foundation. Missing: OrgUnit scope, source indicator.

#### Actor Model (`orbis-domain/src/iam/actor.rs`)
```rust
pub enum Actor {
    User(Uuid),      // Regular authenticated user
    Agent(Uuid),     // AI Agent acting on behalf of user
    Autopilot,       // System daemon
    System,          // Migrations, internal triggers
}
```

**Assessment:** Correctly distinguishes four actor types. Well-designed for audit attribution.

#### Domain Events (`orbis-domain/src/events.rs`)
- 43 event variants across Organization, User, Academic, Student, Finance, Employee, Group, Document.
- Events include `timestamp`, `org_id`, entity IDs, and contextual fields.
- Published through `MemoryEventBus` → delegates to `AutopilotEngine`.
- **Events are NOT persisted.** Bus is a stub (prints + forwards to Autopilot only).

#### Existing Audit Infrastructure

| Component | Status | Notes |
|---|---|---|
| `ReportAuditEntry` domain entity | **EXISTS** | Report/template operations only |
| `report_audit_log` table | **EXISTS** | `id, org_id, actor_id, action, entity_type, entity_id, details(JSON), created_at` |
| `ReportAuditRepository` port | **EXISTS** | `save()`, `list_by_org()` |
| `AuditRead` permission | **EXISTS** | Defined in Permission enum |
| `/api/v1/reports/audit-logs` endpoint | **EXISTS** | Lists report audit entries |
| General system-wide audit trail | **DOES NOT EXIST** | No table, no service, no API |

#### Metadata Fields Across Domain Entities

| Field | Count | Notes |
|---|---|---|
| `created_at` | **ALL** (~50+) | Universally present |
| `updated_at` | ~42 entities | Missing from `JournalEntry`, `Payment`, some join tables |
| `created_by` | **3 entities** | `JournalEntry`, `Payment`, `Document.uploaded_by` |
| `updated_by` | **0 entities** | Does not exist anywhere |
| `org_id` | ~45 entities | Some use `Option<Uuid>` (Person) |
| `org_unit_id` | **1 entity** | Employee only |
| `deleted_at` | 5 entities | Organization, Student, Class, Document, ReportTemplate |
| `deleted_by` | 1 entity | Organization only |
| `is_deleted` | 4 entities | Organization, Student, Employee, User |
| `version` | 3 entities | ReportDefinition, ReportTemplate, ReportTemplateVersion only |
| `notes` | 3 entities | Employee, Payment, ApprovalStep |

### 1.2 Critical Gaps

1. **No `created_by` on sensitive entities**: Student, Employee, Enrollment, Grade, Attendance, Account, Fee, Settings, OrgUnit, Group, Role have no creator tracking.
2. **No `updated_by` anywhere**: Zero entities track who last modified them.
3. **No system-wide audit trail**: Only `report_audit_log` exists (report subsystem specific).
4. **OrgUnit scope limited to Employee**: Student, Enrollment, Class, JournalEntry, Payment, Grade — all lack `org_unit_id` despite OrgUnit tree existing.
5. **DomainEvents are fire-and-forget**: `MemoryEventBus` prints to stdout; no persistence.
6. **RequestContext lacks OrgUnit**: Backend cannot enforce branch-level data isolation.
7. **No concurrency control**: Zero entities have version/row-version fields for optimistic locking outside reporting.
8. **Inconsistent soft delete**: Mix of `is_deleted`, `deleted_at`, both, or neither across entities.

### 1.3 What Works Well

- **Actor model** correctly distinguishes User/System/Agent/Autopilot.
- **RequestContext** flows through all services, extracted from JWT.
- **Permission system** is mature: 88 permissions, DB-driven roles, group-based assignment.
- **PermissionCheckerPort** properly bypasses for System/Autopilot actors.
- **DomainEvent enum** is comprehensive with good granularity.
- **`report_audit_log`** provides a proven reference architecture for audit logging.
- **AuthContext extractor** in Axum correctly populates RequestContext from JWT.

---

## 2. Target Architecture

### 2.1 Core Design Principles

```
Current Record Metadata   → INSIDE the entity (created_at/by, updated_at/by)
Historical Audit           → SEPARATE audit_events table (append-only)
Business Lifecycle         → Domain status + transition rules in application layer
Data Ownership             → org_id always; org_unit_id where branch isolation matters
UI Display                 → Read projection from entity fields, NOT from audit trail
```

### 2.2 Architecture Decision Records

#### ADR-1: Entity-level metadata vs. Audit Trail
- `created_by`, `updated_by` on entities that need them → for **display** in grids/details.
- `audit_events` table → for **investigation**, **compliance**, **history**.
- **Rationale:** Grid showing "Updated by: Ahmed" should query entity table, not join audit trail. No N+1.

#### ADR-2: Single unified `audit_events` table
- Generalize the existing `report_audit_log` pattern to the entire system.
- Do NOT create per-entity audit tables.
- Do NOT duplicate the `DomainEvent` enum as audit entries.
- **Rationale:** One table, one query pattern, one retention policy, one index strategy.

#### ADR-3: EventBus → Audit Trail integration
- `MemoryEventBus.publish()` persists to `audit_events` in the same transaction boundary.
- DomainEvents become the **trigger** for audit log entries.
- **Rationale:** Events already contain actor, entity, timestamp — reuse, don't duplicate.

#### ADR-4: RequestContext enrichment (minimal)
- Add `org_unit_path: Option<String>` (materialized path e.g., `/1/4/12/`).
- Add `source: ActionSource` enum.
- Do NOT add locale/timezone/device — those come from OrgSettings/Session.
- **Rationale:** Minimal context for authorization and audit, not a God object.

#### ADR-5: OrgUnit scope is selective
- Only entities logically belonging to a branch get `org_unit_id`.
- Org-wide entities (Subjects, Currencies, Roles, Settings) do NOT get `org_unit_id`.
- **Rationale:** OrgUnit is branch-level data isolation, not universal tagging.

#### ADR-6: No generic metadata engine
- Each entity explicitly declares which fields it needs.
- No Rust trait-based "Auditable" or "SoftDeletable" generic abstraction.
- **Rationale:** Explicitness over generics. Clear per-entity decisions are simpler than framework magic.

---

## 3. Entity Metadata Matrix

### Legend
- ✅ = exists | ❌→✅ = needs adding | ❌ = not needed | ✅* = exists under different name

| # | Entity | C_at | C_by | U_at | U_by | org_id | org_unit | Audit | Workflow | Soft Del | Version | Action |
|---|--------|------|------|------|------|--------|----------|-------|----------|----------|---------|--------|
| 1 | Organization | ✅ | ❌→✅ | ✅ | ❌→✅ | N/A | N/A | ✅ | ❌ | ✅ | ❌→✅ | **EXTEND** |
| 2 | OrgUnit | ✅ | ❌→✅ | ✅ | ❌→✅ | ✅ | N/A | ✅ | ❌ | ❌ | ❌ | **EXTEND** |
| 3 | User | ✅ | ❌→✅ | ✅ | ❌→✅ | ✅ | ❌ | ✅ | ✅ | ✅ | ❌ | **EXTEND** |
| 4 | Person | ✅ | ❌ | ✅ | ❌ | ✅? | ❌ | ❌ | ❌ | ❌ | ❌ | NO CHANGE |
| 5 | Role | ✅ | ❌ | ✅ | ❌ | ✅ | ❌ | ✅ | ❌ | ❌ | ❌ | NO CHANGE |
| 6 | RolePermission | ✅ | ❌ | ❌ | ❌ | ✅ | ❌ | ✅ | ❌ | ❌ | ❌ | NO CHANGE |
| 7 | UserRoleAssignment | ✅ | ❌ | ❌ | ❌ | ✅ | ❌ | ✅ | ❌ | ❌ | ❌ | NO CHANGE |
| 8 | **Student** | ✅ | ❌→✅ | ✅ | ❌→✅ | ✅ | ❌→✅ | ✅ | ✅ | ✅ | ❌ | **EXTEND** |
| 9 | **Enrollment** | ✅ | ❌→✅ | ✅ | ❌→✅ | ✅ | ❌ | ✅ | ✅ | ❌ | ❌ | **EXTEND** |
| 10 | **Employee** | ✅ | ❌→✅ | ✅ | ❌→✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ❌ | **EXTEND** |
| 11 | Group | ✅ | ❌→✅ | ✅ | ❌→✅ | ✅ | ❌ | ✅ | ❌ | ❌ | ❌ | **EXTEND** |
| 12 | GroupPermission | ✅ | ❌ | ❌ | ❌ | ✅ | ❌ | ✅ | ❌ | ❌ | ❌ | NO CHANGE |
| 13 | UserGroupMembership | ✅ | ❌ | ❌ | ❌ | ✅ | ❌ | ✅ | ❌ | ❌ | ❌ | NO CHANGE |
| 14 | AcademicYear | ✅ | ❌ | ✅ | ❌ | ✅ | ❌ | ✅ | ❌ | ❌ | ❌ | NO CHANGE |
| 15 | AcademicPeriod | ✅ | ❌ | ✅ | ❌ | ✅ | ❌ | ✅ | ✅ | ❌ | ❌ | NO CHANGE |
| 16 | Subject | ✅ | ❌ | ✅ | ❌ | ✅ | ❌ | ❌ | ❌ | ❌ | ❌ | NO CHANGE |
| 17 | **Class** | ✅ | ❌ | ✅ | ❌ | ✅ | ❌→✅ | ❌ | ❌ | ✅ | ❌ | **EXTEND** |
| 18 | ClassSubject | ✅ | ❌ | ✅ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | NO CHANGE |
| 19 | **Account** | ✅ | ❌→✅ | ✅ | ❌→✅ | ✅ | ❌ | ✅ | ❌ | ❌ | ❌→✅ | **EXTEND** |
| 20 | **JournalEntry** | ✅ | ✅ | ❌→✅ | ❌→✅ | ✅ | ❌→✅ | ✅ | ✅ | ❌ | ❌→✅ | **EXTEND** |
| 21 | **Payment** | ✅ | ✅ | ❌→✅ | ❌ | ✅ | ❌→✅ | ✅ | ❌ | ❌ | ❌ | **EXTEND** |
| 22 | FeeTemplate | ✅ | ❌ | ✅ | ❌ | ✅ | ❌ | ❌ | ❌ | ❌ | ❌ | NO CHANGE |
| 23 | **StudentFee** | ✅ | ❌→✅ | ✅ | ❌ | ✅ | ❌ | ✅ | ✅ | ❌ | ❌ | **EXTEND** |
| 24 | DiscountTemplate | ✅ | ❌ | ✅ | ❌ | ✅ | ❌ | ❌ | ❌ | ❌ | ❌ | NO CHANGE |
| 25 | **Grade** | ✅ | ❌→✅ | ✅ | ❌→✅ | ✅ | ❌ | ✅ | ❌ | ❌ | ❌ | **EXTEND** |
| 26 | Attendance | ✅ | ❌→✅ | ✅ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | EXTEND: `marked_by` |
| 27 | BehaviorRecord | ✅ | ❌→✅ | ✅ | ❌ | ✅ | ❌ | ❌ | ❌ | ❌ | ❌ | EXTEND: `recorded_by` |
| 28 | AssessmentScheme | ✅ | ❌ | ✅ | ❌ | ✅ | ❌ | ❌ | ❌ | ❌ | ❌ | NO CHANGE |
| 29 | **Room** | ✅ | ❌ | ✅ | ❌ | ✅ | ❌→✅ | ❌ | ❌ | ❌ | ❌ | **EXTEND** |
| 30 | OverrideRequest | ✅ | ✅* | ✅ | ❌ | ✅ | ❌ | ✅ | ✅ | ❌ | ❌ | NO CHANGE |
| 31 | Document | ✅ | ✅* | ✅ | ❌ | ✅ | ❌ | ✅ | ❌ | ✅ | ❌ | NO CHANGE |
| 32 | Currency | ✅ | ❌ | ✅ | ❌ | ✅ | ❌ | ❌ | ❌ | ❌ | ❌ | NO CHANGE |
| 33 | ExchangeRate | ✅ | ❌ | ❌ | ❌ | ✅ | ❌ | ❌ | ❌ | ❌ | ❌ | NO CHANGE |
| 34 | IntegrationConfig | ✅ | ❌ | ✅ | ❌ | ✅ | ❌ | ✅ | ❌ | ❌ | ❌ | NO CHANGE |
| 35 | WorkflowRule | ✅ | ❌ | ✅ | ❌ | ✅ | ❌ | ❌ | ❌ | ❌ | ❌ | NO CHANGE |
| 36 | WorkflowExecution | ✅* | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ✅ | ❌ | ❌ | NO CHANGE |
| 37 | AutopilotTask | ✅ | ❌ | ✅ | ❌ | ✅ | ❌ | ❌ | ✅ | ❌ | ❌ | NO CHANGE |
| 38 | Session | ✅ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | NO CHANGE |
| 39 | MfaFactor | ✅ | ❌ | ❌ | ❌ | ❌ | ❌ | ✅ | ❌ | ❌ | ❌ | NO CHANGE |
| 40 | PeriodTemplate | ✅ | ❌ | ✅ | ❌ | ✅ | ❌ | ❌ | ❌ | ❌ | ❌ | NO CHANGE |
| 41 | ReportDefinition | ✅ | ✅* | ✅ | ❌ | ✅ | ❌ | ✅ | ❌ | ❌ | ✅ | NO CHANGE |
| 42 | ReportTemplate | ✅ | ❌ | ✅ | ❌ | ✅ | ❌ | ✅ | ✅ | ✅ | ✅ | NO CHANGE |
| 43 | PromotionJob | ✅ | ❌ | ✅ | ❌ | ✅ | ❌ | ✅ | ✅ | ❌ | ❌ | NO CHANGE |
| 44 | StudentGuardian | ✅ | ❌ | ✅ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | NO CHANGE |
| 45 | GpaRecord | ✅* | ❌ | ❌ | ❌ | ✅ | ❌ | ❌ | ❌ | ❌ | ❌ | NO CHANGE |
| 46 | Exam entities | ✅ | ❌ | ✅ | ❌ | ✅* | ❌ | ❌ | ❌ | ❌ | ❌ | NO CHANGE |
| 47 | Settings (JSON) | N/A | N/A | N/A | N/A | ✅ | N/A | ✅ | ❌ | N/A | ❌→✅ | **EXTEND** |

### Summary of Required Changes

| Change | Entities Affected | Count |
|---|---|---|
| Add `created_by: Option<Uuid>` | Organization, OrgUnit, User, Student, Enrollment, Employee, Group, Account, Grade, StudentFee, Attendance(`marked_by`), BehaviorRecord(`recorded_by`) | 12 |
| Add `updated_by: Option<Uuid>` | Organization, OrgUnit, User, Student, Enrollment, Employee, Group, Account, Grade, JournalEntry | 10 |
| Add `updated_at` | JournalEntry, Payment | 2 |
| Add `org_unit_id: Option<Uuid>` | Student, Class, JournalEntry, Payment, Room | 5 |
| Add `version: i32` | Organization, Account, JournalEntry, Settings | 4 |
| **Total entities unchanged** | | **35 of 47** |

---

## 4. Audit Metadata Design

### 4.1 Field Semantics

| Field | Type | Source | Mutable | Purpose |
|---|---|---|---|---|
| `created_at` | `DateTime<Utc>` | `Utc::now()` in domain/service | No | When record was created |
| `created_by` | `Option<Uuid>` | `ctx.actor.id()` in service layer | No | Who created (None for System/Autopilot) |
| `updated_at` | `DateTime<Utc>` | `Utc::now()` on every save | Yes | When last modified |
| `updated_by` | `Option<Uuid>` | `ctx.actor.id()` in service layer | Yes | Who last modified |

### 4.2 Population Flow

```
Frontend (Svelte/Tauri)
  │
  │  ← Does NOT send created_by/updated_by
  │     Only sends business payload
  │
  ▼
API Handler (Axum extractors.rs)
  │
  │  ← AuthContext extracts JWT → RequestContext { tenant_id, actor, request_id }
  │
  ▼
Application Service
  │
  │  ← service.create_entity(ctx, input) {
  │        entity.created_by = ctx.actor.id();  // None for System
  │        entity.created_at = Utc::now();
  │     }
  │
  │  ← service.update_entity(ctx, id, input) {
  │        entity.updated_by = ctx.actor.id();
  │        entity.updated_at = Utc::now();
  │     }
  │
  ▼
Repository.save(entity)
```

### 4.3 Local vs. Remote Parity

| Transport | Context Source | Result |
|---|---|---|
| **LocalClient** (Tauri IPC) | Tauri `state.rs` builds RequestContext from local authenticated user | `Actor::User(uuid)` |
| **RemoteClient** (HTTP) | `AuthContext` extractor reads JWT Bearer token | `Actor::User(uuid)` |
| **Autopilot** | Service creates RequestContext with `Actor::Autopilot` | `actor.id() = None` |

Both paths → Service layer receives identical RequestContext → sets `created_by`/`updated_by` identically.

### 4.4 Actor Attribution

| Actor | `created_by` / `updated_by` | Audit `actor_type` |
|---|---|---|
| `Actor::User(uuid)` | `Some(uuid)` | `"user"` |
| `Actor::Agent(uuid)` | `Some(uuid)` | `"agent"` |
| `Actor::Autopilot` | `None` | `"autopilot"` |
| `Actor::System` | `None` | `"system"` |

### 4.5 UI Trust Boundary

```
               TRUST BOUNDARY
                    │
  Frontend          │         Backend
  ─────────         │         ───────
  user_id in store  │    ← Ignored. ctx.actor.id() from JWT.
  org_id in store   │    ← Ignored. ctx.tenant_id from JWT.
  created_by field  │    ← NEVER sent by frontend. Service sets it.
  updated_by field  │    ← NEVER sent by frontend. Service sets it.
  created_at field  │    ← NEVER sent by frontend. Service sets it.
```

---

## 5. Audit Trail Architecture

### 5.1 Design: Generalize Existing `report_audit_log`

The `report_audit_log` table already implements the correct audit pattern:
```sql
-- EXISTING TABLE (report subsystem only)
report_audit_log (
    id UUID PK,
    org_id UUID,
    actor_id UUID,
    action TEXT,
    entity_type TEXT,
    entity_id UUID,
    details JSONB,
    created_at TIMESTAMPTZ
)
```

**Decision:** Generalize this into a system-wide `audit_events` table. Migrate existing `report_audit_log` data into it. Then remove the old table.

### 5.2 `audit_events` Table Schema

```sql
CREATE TABLE audit_events (
    id              UUID PRIMARY KEY,
    org_id          UUID NOT NULL,

    -- Who
    actor_id        UUID,                  -- NULL for System/Autopilot
    actor_type      TEXT NOT NULL,          -- 'user', 'agent', 'autopilot', 'system'

    -- What
    action          TEXT NOT NULL,          -- 'student.created', 'journal_entry.posted'
    entity_type     TEXT NOT NULL,          -- 'student', 'journal_entry', 'user'
    entity_id       UUID NOT NULL,          -- PK of affected entity

    -- Where (scope)
    org_unit_id     UUID,                  -- branch context if applicable

    -- Context
    request_id      TEXT,                  -- correlation ID from RequestContext
    source          TEXT NOT NULL DEFAULT 'api',  -- 'api', 'tauri_ipc', 'autopilot', 'system', 'integration'

    -- Details (optional, structured)
    changes         JSONB,                 -- {"field": {"old": ..., "new": ...}} for updates only
    metadata        JSONB,                 -- extra context (reason, notes, etc.)

    -- When
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT fk_audit_org FOREIGN KEY (org_id) REFERENCES organizations(id)
);

-- Primary query indexes
CREATE INDEX idx_audit_org_created ON audit_events(org_id, created_at DESC);
CREATE INDEX idx_audit_entity ON audit_events(entity_type, entity_id, created_at DESC);
CREATE INDEX idx_audit_actor ON audit_events(actor_id, created_at DESC) WHERE actor_id IS NOT NULL;
```

### 5.3 Audit Event Triggers

| Trigger | Mechanism | Example |
|---|---|---|
| DomainEvent published | `EventBus.publish()` → writes audit row | `StudentRegistered` → `"student.created"` |
| Sensitive operation | Explicit `AuditService.log()` call | Password reset, permission change |
| Status transition | Service publishes event after state change | `JournalEntryPosted` → `"journal_entry.posted"` |

### 5.4 Changes Field Strategy

| Action | `changes` Content |
|---|---|
| Create | `null` (no "before" state) |
| Update | `{"field": {"old": "val", "new": "val"}}` — only changed fields |
| Delete | `null`, optional `metadata: {"reason": "..."}` |
| Status change | `{"status": {"old": "draft", "new": "posted"}}` |

**Limits:**
- Do NOT store full entity snapshots. Only field-level diffs.
- **NEVER** store: `password_hash`, `secrets`, `tokens`, `recovery_codes`.
- Average `changes` size: ~200 bytes per event.

### 5.5 What NOT to Audit

| Category | Reason |
|---|---|
| Read operations (list, get) | Enormous volume; use access logs |
| Auth (login/logout) | Tracked via Session entity + UserLoggedIn events |
| Batch calculations (GPA) | Derived data; audit trigger, not each result |
| Lesson generation | Bulk system op; audit solver job, not each lesson |
| Report job execution | Already tracked in report_audit_log (will migrate) |

### 5.6 Grid vs. Audit Separation

```
Students Grid:
┌──────────┬──────────┬────────────┬─────────────┐
│ Name     │ Status   │ Created By │ Updated At  │
├──────────┼──────────┼────────────┼─────────────┤
│ Ahmed    │ Active   │ admin_user │ 2026-09-20  │  ← FROM students TABLE (direct)
└──────────┴──────────┴────────────┴─────────────┘

Student Detail → "Audit History" tab (on-demand):
┌──────────────────┬────────┬───────────┬────────────────────┐
│ Action           │ Actor  │ Timestamp │ Changes            │
├──────────────────┼────────┼───────────┼────────────────────┤
│ student.created  │ Ahmed  │ 09/15     │ —                  │  ← FROM audit_events
│ student.updated  │ Ahmed  │ 09/20     │ phone → 055...     │  ← FROM audit_events
│ student.status   │ System │ 09/22     │ active → on_leave  │  ← FROM audit_events
└──────────────────┴────────┴───────────┴────────────────────┘
```

**Rule: Grid queries NEVER join audit_events. Audit loaded only on-demand, paginated.**

---

## 6. Execution Context Design

### 6.1 Enriched RequestContext

```rust
pub struct RequestContext {
    pub tenant_id: Uuid,                    // Organization ID (from JWT)
    pub actor: Actor,                       // User/Agent/Autopilot/System
    pub request_id: String,                 // Correlation ID (UUID)
    pub source: ActionSource,               // NEW: Api | TauriIpc | Autopilot | System | Integration
    pub org_unit_path: Option<String>,       // NEW: e.g., "/1/4/12/" — resolved per-request
}

#[derive(Debug, Clone)]
pub enum ActionSource {
    Api,
    TauriIpc,
    Autopilot,
    System,
    Integration(String),  // provider name
}
```

### 6.2 OrgUnit Path in Context

- Currently, repositories query by `org_id` only.
- With OrgUnit scope: A branch admin sees only data in their branch subtree.
- `org_unit_path` enables `WHERE path LIKE '/1/4/%'` queries.
- Source: JWT custom claim (branch-scoped users) OR organization root path (org admins).

### 6.3 What Is NOT in RequestContext

| Candidate | Decision | Reason |
|---|---|---|
| Locale | OUT | `Organization.locale` resolved at query time |
| Timezone | OUT | `Organization.timezone` |
| Permissions list | OUT | Checked via `PermissionCheckerPort` per-action |
| Device/Session info | OUT | From `Session` entity when needed |
| User name | OUT | Resolved by joining `Person` for display |

---

## 7. Data Scope Strategy

### 7.1 Scope Classification

| Scope | Meaning | Entities |
|---|---|---|
| **Organization** | Filtered by `org_id` | ALL entities with `org_id` |
| **OrgUnit** | Further scoped to branch/department | Student, Class, Employee, JournalEntry, Payment, Room |
| **Global** | System-level | Person (`org_id: Option`) |
| **Period-scoped** | Within academic period | Enrollment, Grade, Attendance, ExamSession, ScheduledLesson |
| **User-scoped** | Belongs to user | Session, MfaFactor, UserDevice, UserIdentity |

### 7.2 OrgUnit Addition Plan

| Entity | Why | Nullable? | Backfill |
|---|---|---|---|
| Student | Students belong to branches | `Option<Uuid>` | Infer from enrollment class → class org_unit, or NULL |
| Class | Classes run at specific branches | `Option<Uuid>` | NULL for existing records |
| JournalEntry | Financial entries per branch | `Option<Uuid>` | NULL for existing entries |
| Payment | Payments collected at branches | `Option<Uuid>` | NULL |
| Room | Rooms are in physical branches | `Option<Uuid>` | NULL |

### 7.3 Entities That Do NOT Get OrgUnit

- Subject (org-wide curriculum)
- Account (org-wide chart of accounts)
- Currency, ExchangeRate (org-wide config)
- FeeTemplate, DiscountTemplate (org-wide policy)
- Role, Group (org-wide IAM)
- Settings (org-wide; branch overrides via `OrgUnit.local_policies`)
- AcademicYear, AcademicPeriod (org-wide calendar)

---

## 8. Permission Integration

### 8.1 Permissions

| Permission | Status | Purpose |
|---|---|---|
| `audit:read` | **EXISTS** | View audit trail entries |
| `audit:export` | **NEW** | Export audit data |

No other new permissions needed. Audit trail reads operations already authorized by existing permissions.

### 8.2 OrgUnit Scope Enforcement on Audit

- Audit events inherit `org_unit_id` from the referenced entity.
- Branch admin with path `/1/4/` sees only audit events where:
  ```sql
  org_unit_id IN (SELECT id FROM org_units WHERE path LIKE '/1/4/%')
  ```

---

## 9. Concurrency Control

### 9.1 Entities Requiring Optimistic Locking

| Entity | Why | Mechanism |
|---|---|---|
| Organization | Settings changes by multiple admins | `version: i32` + check on update |
| Account | Chart of accounts concurrent edits | `version: i32` |
| JournalEntry | Financial data integrity | `version: i32` |
| Settings (JSON) | Multiple admins editing simultaneously | `version: i32` on settings table/field |

### 9.2 Implementation Pattern

```rust
fn update_account(ctx: &RequestContext, id: Uuid, input: UpdateAccountInput) -> Result<...> {
    let account = repo.find_by_id(id).await?.ok_or(NotFound)?;
    if account.version != input.expected_version {
        return Err(DomainError::ConcurrencyConflict("Account modified by another user"));
    }
    account.version += 1;
    account.updated_by = ctx.actor.id();
    account.updated_at = Utc::now();
    repo.save(&account).await
}
```

### 9.3 Entities NOT Needing Concurrency

- Student, Employee: Status changes are discrete operations.
- Grade: Teacher enters for specific student/subject — low conflict risk.
- Enrollment: Created in batch, not concurrently edited.

---

## 10. Soft Delete Strategy

| Entity | Current | Decision | Reason |
|---|---|---|---|
| Organization | `is_deleted + deleted_at + deleted_by` | **KEEP** | Data recovery |
| Student | `is_deleted + deleted_at` | **KEEP** | Regulatory: immutable records |
| Employee | `is_deleted` only | **EXTEND**: add `deleted_at` | Track deletion time |
| User | `is_deleted` | **KEEP** as-is | Account deactivation |
| Class | `deleted_at` | **KEEP** | Academic history |
| Document | `deleted_at` | **KEEP** | Recoverability |
| ReportTemplate | `deleted_at` | **KEEP** | Version history |
| JournalEntry | — | **IMMUTABLE**: no delete | Financial records → use reversal |
| Payment | — | **IMMUTABLE**: no delete | Financial records |
| Grade | Hard delete | **KEEP** | Can be re-entered |
| Enrollment | — | Status-based (`Withdrawn`) | Not deleted, status changed |

---

## 11. Notes Strategy

| Entity | Field | Type | Decision |
|---|---|---|---|
| Employee | `notes: Option<String>` | Business field (HR notes) | KEEP as-is |
| Payment | `notes: Option<String>` | Business field (payment memo) | KEEP as-is |
| ApprovalStep | `notes: Option<String>` | Decision comment | KEEP as-is |

Do NOT add generic notes to other entities. If timeline/discussion needed later → separate entity comments feature.

---

## 12. Security Considerations

### 12.1 PII in Audit Events

| Data | Audit Behavior |
|---|---|
| Person name, email, phone | Record `entity_id` only, NOT PII values in `changes` |
| Password hash | **NEVER** store |
| MFA secrets, recovery codes | **NEVER** store |
| Integration secrets/tokens | **NEVER** store |
| Financial amounts | OK in changes (amount changed from X to Y) |
| Student/Employee status | OK in changes |
| Grade scores | OK in changes |

### 12.2 Tenant Isolation

- `audit_events.org_id` always set from `ctx.tenant_id` (JWT-derived).
- ALL queries include `WHERE org_id = ?`.
- No cross-tenant audit visibility possible.

### 12.3 Audit Integrity

- `audit_events` is **append-only**. No UPDATE or DELETE endpoints.
- API exposes only `GET /api/v1/audit/events` (read-only).
- No admin can modify or delete audit records through any client.

---

## 13. Performance Strategy

### 13.1 Write Path

- Single INSERT per business operation.
- No triggers, no stored procedures, no async queue (Phase 1).
- Bulk operations (batch grade entry for 200 students) → **one** audit event with `metadata: {"count": 200}`.

### 13.2 Read Path

- Grid/table queries → entity tables (fast, indexed by `org_id`).
- Audit timeline → loaded only on-demand (detail panel "History" tab).
- Pagination: max 50 events per page.

### 13.3 Storage Projection

- ~100 audit events/day per active org (SIS with 500 students, 50 staff).
- ~36,500 rows/year per org. Manageable in single table for years.
- Average `changes` JSON: ~200 bytes per event.
- Retention: No auto-cleanup in Phase 1. Add archiving if needed later.

### 13.4 Index Plan

| Index | Purpose | Phase |
|---|---|---|
| `(org_id, created_at DESC)` | Default list view | 1 |
| `(entity_type, entity_id, created_at DESC)` | Entity detail audit tab | 1 |
| `(actor_id, created_at DESC) WHERE NOT NULL` | "What did user X do?" | 1 |
| `(org_id, action, created_at DESC)` | Filter by action type | 2 |
| `(org_unit_id, created_at DESC)` | Branch-scoped views | 2 |

### 13.5 Impact of New Columns

- `created_by`, `updated_by` (`Option<Uuid>`) → nullable, no impact on existing queries.
- `org_unit_id` (`Option<Uuid>`) → nullable, no impact on existing queries.
- `version` (`i32 DEFAULT 1`) → default value, no impact on existing reads.
- No existing indexes need modification.
