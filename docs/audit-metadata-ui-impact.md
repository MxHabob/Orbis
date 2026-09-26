# Orbis — Audit Metadata UI Impact Plan

> **Version:** 1.0.0
> **Companion:** `audit-metadata-architecture.md`, `audit-metadata-implementation-plan.md`

---

## 1. Overview

This document specifies every UI surface that must be created or modified to support audit metadata, audit trail, OrgUnit scoping, and concurrency control in the Orbis Desktop application.

---

## 2. Core Principles for UI Changes

### 2.1 Metadata Fields Are Server-Controlled

```
╔═══════════════════════════════════════════════════════════════════╗
║  RULE: The frontend NEVER sends created_by, updated_by,         ║
║  created_at, updated_at to the backend.                         ║
║  These are set by the backend from RequestContext (JWT-derived). ║
╚═══════════════════════════════════════════════════════════════════╝
```

### 2.2 Metadata Columns Are Hidden by Default

New grid columns (Created By, Updated By, Branch) are available in the column picker but NOT visible by default. This preserves the current UX for existing users.

### 2.3 Audit Timeline Is On-Demand

The Audit History tab in detail panels loads lazily and is permission-gated by `audit:read`.

### 2.4 Display Names, Not UUIDs

`created_by` and `updated_by` are UUID references. UI must resolve them to display names. Use a batch user name resolver to avoid N+1 lookups.

---

## 3. New Shared Components

### 3.1 `<ActorBadge>` Component

**Purpose:** Displays a user/system attribution inline.

**Location:** `packages/ui/` or `$lib/shared/components/`

**Props:**
```typescript
interface ActorBadgeProps {
  userId: string | null | undefined;
  showAvatar?: boolean;  // default: false
  size?: 'sm' | 'md';   // default: 'sm'
}
```

**Behavior:**
| Value | Display |
|---|---|
| Valid UUID | User's display name (from resolver cache) |
| `null` / `undefined` | "System" (or localized equivalent "النظام") |
| UUID not found | "Unknown User" / "مستخدم غير معروف" |

**Design:**
```
┌─────────────────────┐
│ 👤 Ahmed AlKhaldi  │   ← sm size, no avatar
└─────────────────────┘

┌──────────────────────────┐
│ [avatar] Ahmed AlKhaldi  │   ← md size, with avatar
└──────────────────────────┘

┌───────────┐
│ ⚙️ System │   ← null actor
└───────────┘
```

### 3.2 `<AuditTimeline>` Component

**Purpose:** Displays paginated audit history for an entity.

**Location:** `$lib/shared/components/audit-timeline.svelte`

**Props:**
```typescript
interface AuditTimelineProps {
  entityType: string;
  entityId: string;
  maxHeight?: string;  // default: '400px'
}
```

**States:**
| State | Display |
|---|---|
| Loading | Skeleton loader (3-4 pulse items) |
| Empty | "No audit history" with info icon |
| Error | Error message with retry button |
| Data | Paginated list of audit events |

**Design (LTR):**
```
Audit History
─────────────────────────────────────────
  ○ student.created — Ahmed AlKhaldi
  │ Sep 15, 2026 • 10:23 AM • API
  │
  ○ student.updated — Ahmed AlKhaldi
  │ Sep 20, 2026 • 2:15 PM • Desktop
  │ Changes: phone: "" → "055123456"
  │
  ○ student.status_changed — System
  │ Sep 22, 2026 • 8:00 AM • Autopilot
  │ Changes: status: "active" → "on_leave"
  │
  [Load More]
```

**Design (RTL — Arabic):**
```
سجل التعديلات
─────────────────────────────────────────
  ○ إنشاء طالب — أحمد الخالدي
  │ ١٥ سبتمبر ٢٠٢٦ • ١٠:٢٣ ص • API
  │
  ○ تعديل بيانات الطالب — أحمد الخالدي
  │ ٢٠ سبتمبر ٢٠٢٦ • ٢:١٥ م • سطح المكتب
  │ التغييرات: الهاتف: "" ← "055123456"
  │
  [تحميل المزيد]
```

**Data Flow:**
```
<AuditTimeline entityType="student" entityId="..." />
  ↓
api.getEntityAudit("student", entityId, limit=20, offset=0)
  ↓
GET /api/v1/audit/events/entity/student/{id}?limit=20
  ↓
audit_events WHERE entity_type = 'student' AND entity_id = ?
```

### 3.3 `<BranchBadge>` Component

**Purpose:** Displays OrgUnit name for branch-scoped entities.

**Location:** `packages/ui/` or `$lib/shared/components/`

**Props:**
```typescript
interface BranchBadgeProps {
  orgUnitId: string | null | undefined;
  size?: 'sm' | 'md';
}
```

**Behavior:**
| Value | Display |
|---|---|
| Valid UUID | OrgUnit name (from orgContext cache) |
| `null` | "—" (not assigned) |

### 3.4 `<VersionConflictDialog>` Component

**Purpose:** Handles optimistic locking conflicts gracefully.

**Location:** `$lib/shared/components/version-conflict-dialog.svelte`

**When:** Backend returns `ConcurrencyConflict` error.

**Design:**
```
╔═══════════════════════════════════════════════╗
║  ⚠️ Update Conflict                          ║
║                                               ║
║  This record was modified by another user     ║
║  while you were editing it.                   ║
║                                               ║
║  Your changes could not be saved.             ║
║                                               ║
║  ┌───────────┐  ┌──────────────────────┐     ║
║  │  Reload   │  │  Review & Retry      │     ║
║  └───────────┘  └──────────────────────┘     ║
╚═══════════════════════════════════════════════╝
```

---

## 4. Screen-by-Screen UI Changes

### 4.1 Employee Directory (`features/employees/`)

#### Grid Changes

**Current columns:** Selected, Index, Full Name, Employee Number, Job/Dept, Type, Status, Account, Hire Date, Actions

**New optional columns (hidden by default):**

| Column | Key | Source | Width |
|---|---|---|---|
| Created By | `created_by_display` | `<ActorBadge userId={row.created_by} />` | 150px |
| Updated By | `updated_by_display` | `<ActorBadge userId={row.updated_by} />` | 150px |
| Updated At | `updated_at_display` | `orgContext.formatDate(row.updated_at)` | 130px |

**User enables columns via:** Column picker (existing `usePersonalization` stores preference).

#### EmployeeSheet Changes

**Preview mode:** Add metadata section at bottom:
```
─────────────────────────────
Created by:  Ahmed AlKhaldi  •  Sep 15, 2026
Updated by:  Fatima Hassan   •  Sep 20, 2026
─────────────────────────────
```

**Audit tab (new):** Add `<AuditTimeline entityType="employee" entityId={...} />` wrapped in `<PermissionGate permission="audit:read">`.

#### Context Menu

Add divider + "📋 View Audit History" item after existing actions. Only visible with `audit:read` permission.

---

### 4.2 Student List (`features/students/`)

#### Grid Changes

**New optional columns:**

| Column | Key | Source |
|---|---|---|
| Created By | `created_by_display` | `<ActorBadge>` |
| Updated By | `updated_by_display` | `<ActorBadge>` |
| Branch | `org_unit_display` | `<BranchBadge>` |

#### Student Detail / Sheet

**Metadata section:** Created by, Updated by, Branch.

**Audit tab:** `<AuditTimeline entityType="student" entityId={...} />`

#### Student Create Form

**New field (if org has multiple OrgUnits):**
```
Branch / الفرع
[Select Branch ▾]        ← OrgUnit selector dropdown
```

This field is shown only when the organization has > 1 active OrgUnit.

#### Filters

Add Branch filter pill to student list filter bar.

---

### 4.3 Journal Entries (`features/finance/`)

#### Grid Changes

**New optional columns:**

| Column | Key | Source |
|---|---|---|
| Updated By | `updated_by_display` | `<ActorBadge>` |
| Branch | `org_unit_display` | `<BranchBadge>` |
| Version | `version` | Direct number |

#### Journal Entry Form

**New field:**
```
Branch / الفرع
[Select Branch ▾]
```

**Concurrency:** Form includes hidden `version` field. On submit, sends `expected_version`. If backend returns `ConcurrencyConflict`, shows `<VersionConflictDialog>`.

#### Context Menu

Add "📋 View Audit History" item.

---

### 4.4 Payments (`features/finance/`)

#### Grid Changes

**New optional column:**

| Column | Key | Source |
|---|---|---|
| Branch | `org_unit_display` | `<BranchBadge>` |

---

### 4.5 Grades (`features/assessment/`)

#### Grid Changes

**New optional columns:**

| Column | Key | Source |
|---|---|---|
| Entered By | `created_by_display` | `<ActorBadge>` |
| Modified By | `updated_by_display` | `<ActorBadge>` |

**Note:** Column labels are domain-specific ("Entered By" instead of "Created By" for grades).

---

### 4.6 Groups (`features/groups/`)

#### Grid Changes

**New optional columns:** Created By, Updated By.

#### Context Menu

Add "📋 View Audit History" item.

---

### 4.7 Classes (`features/academics/`)

#### Grid Changes

**New optional column:** Branch.

#### Class Create/Edit Form

**New field:** Branch selector (OrgUnit dropdown).

---

### 4.8 Chart of Accounts (`features/finance/`)

#### Account Edit Form

**Concurrency:** Include `version` field for optimistic locking. Show `<VersionConflictDialog>` on conflict.

---

### 4.9 Organization Settings (`features/settings/`)

#### Settings Form

**Concurrency:** Settings form includes version. On save, if conflict detected:
```
⚠️ Settings were updated by another administrator.
Please refresh and try again.
```

---

### 4.10 Attendance (`features/attendance/`)

#### Grid Changes

**New optional column:**

| Column | Key | Source |
|---|---|---|
| Marked By | `marked_by_display` | `<ActorBadge>` |

---

## 5. Global Audit Dashboard (Optional — Phase 2)

**Route:** `/audit` or within Settings section.

**Purpose:** System-wide audit event viewer for administrators.

**Design:**
```
┌─────────────────────────────────────────────────────────┐
│ Audit Log                                        🔍     │
├─────────────────────────────────────────────────────────┤
│ Filters: [Entity Type ▾] [Action ▾] [Actor ▾] [Date ↔] │
│ Branch:  [All ▾]                                        │
├─────────────────────────────────────────────────────────┤
│ ○ student.created • Ahmed AlKhaldi • Sep 26, 10:23 AM  │
│   Entity: Student/abc-123 • Source: Desktop             │
│                                                         │
│ ○ journal_entry.posted • System • Sep 26, 10:20 AM     │
│   Entity: JournalEntry/def-456 • Source: Autopilot     │
│   Changes: status: draft → posted                       │
│                                                         │
│ ○ employee.updated • Fatima Hassan • Sep 26, 10:15 AM  │
│   Entity: Employee/ghi-789 • Source: API                │
│   Changes: department: "IT" → "Finance"                 │
├─────────────────────────────────────────────────────────┤
│ Showing 1-20 of 1,247  │ [◀ Prev] [Page 1] [Next ▶]  │
└─────────────────────────────────────────────────────────┘
```

**Permission:** `audit:read` required.

**Implementation:** Uses `api.listAuditEvents(filter)` with server-side filtering and pagination.

---

## 6. Localization Keys (Paraglide)

All new UI text must use Paraglide message keys. No hardcoded strings.

### New Keys Required

```
// Column headers
audit.column.created_by = "Created By" / "أنشئ بواسطة"
audit.column.updated_by = "Updated By" / "عُدّل بواسطة"
audit.column.updated_at = "Updated At" / "تاريخ التعديل"
audit.column.branch = "Branch" / "الفرع"
audit.column.version = "Version" / "الإصدار"
audit.column.marked_by = "Marked By" / "سُجّل بواسطة"
audit.column.entered_by = "Entered By" / "أُدخل بواسطة"
audit.column.recorded_by = "Recorded By" / "سُجّل بواسطة"

// Audit timeline
audit.timeline.title = "Audit History" / "سجل التعديلات"
audit.timeline.empty = "No audit history" / "لا توجد سجلات تعديل"
audit.timeline.load_more = "Load More" / "تحميل المزيد"
audit.timeline.changes = "Changes" / "التغييرات"

// Actor display
audit.actor.system = "System" / "النظام"
audit.actor.autopilot = "Autopilot" / "النظام الآلي"
audit.actor.unknown = "Unknown User" / "مستخدم غير معروف"

// Actions (for audit timeline display)
audit.action.created = "Created" / "إنشاء"
audit.action.updated = "Updated" / "تعديل"
audit.action.deleted = "Deleted" / "حذف"
audit.action.status_changed = "Status Changed" / "تغيير الحالة"
audit.action.posted = "Posted" / "ترحيل"

// Concurrency
audit.conflict.title = "Update Conflict" / "تعارض في التحديث"
audit.conflict.message = "This record was modified by another user" / "تم تعديل هذا السجل بواسطة مستخدم آخر"
audit.conflict.reload = "Reload" / "إعادة تحميل"
audit.conflict.retry = "Review & Retry" / "مراجعة وإعادة المحاولة"

// Context menu
audit.context_menu.view_history = "View Audit History" / "عرض سجل التعديلات"

// Filters
audit.filter.entity_type = "Entity Type" / "نوع الكيان"
audit.filter.action = "Action" / "الإجراء"
audit.filter.actor = "Actor" / "المستخدم"
audit.filter.date_range = "Date Range" / "نطاق التاريخ"
audit.filter.branch = "Branch" / "الفرع"

// Branch selector
audit.branch.select = "Select Branch" / "اختر الفرع"
audit.branch.all = "All Branches" / "جميع الفروع"
audit.branch.none = "Not Assigned" / "غير محدد"
```

---

## 7. RTL Considerations

### Timeline Direction

In RTL mode (Arabic), the audit timeline flows right-to-left:
```
سجل التعديلات
─────────────────────────────────────────
                        إنشاء طالب — أحمد الخالدي  ○
                  ١٥ سبتمبر ٢٠٢٦ • ١٠:٢٣ ص • API  │
                                                      │
                   تعديل بيانات الطالب — أحمد الخالدي  ○
            ٢٠ سبتمبر ٢٠٢٦ • ٢:١٥ م • سطح المكتب  │
                       التغييرات: الهاتف: "" ← "055"  │
```

### Column Order

In RTL grids, new metadata columns follow the same mirroring as existing columns.

### Date/Time Formatting

Use `orgContext.formatDate()` and `orgContext.formatDateTime()` which already handle locale-aware formatting.

---

## 8. Accessibility

- All new columns have proper `aria-label` attributes.
- Audit timeline uses semantic list markup (`<ol>` with `<li>`).
- Version conflict dialog is keyboard-navigable and screen-reader accessible.
- ActorBadge includes `title` attribute with full user name.
- Color is not the only indicator of status (icons + text).

---

## 9. Error Handling

### Audit Timeline Errors

| Error | UI Response |
|---|---|
| Network error | "Could not load audit history. Tap to retry." + Retry button |
| Permission denied | Component not rendered (hidden by PermissionGate) |
| No data | "No audit history for this record." |

### Version Conflict Errors

| Error | UI Response |
|---|---|
| `ConcurrencyConflict` from backend | `<VersionConflictDialog>` with Reload / Retry options |
| Reload action | Refreshes entity data, re-enables form with updated version |
| Retry action | Keeps user changes, attempts save with fresh version |

### User Name Resolution Errors

| Error | UI Response |
|---|---|
| UUID not in cache | Show UUID truncated (`abc-12...`) with tooltip showing full ID |
| Resolver fails | Show "Unknown User" / "مستخدم غير معروف" |

---

## 10. User Name Resolution Architecture

### Problem

`created_by` and `updated_by` are UUIDs. Grids displaying 50 rows with Created By column would need 50 user lookups → N+1 problem.

### Solution: Batch User Name Resolver

```typescript
// $lib/shared/services/user-name-resolver.svelte.ts
class UserNameResolver {
  private cache = new Map<string, { name: string; nameAr?: string }>();
  private pending = new Set<string>();

  // Called by grid before render with all user IDs visible
  async prefetch(userIds: string[]): Promise<void> {
    const missing = userIds.filter(id => id && !this.cache.has(id));
    if (missing.length === 0) return;
    const users = await api.batchGetUserNames(missing);
    users.forEach(u => this.cache.set(u.id, { name: u.name, nameAr: u.nameAr }));
  }

  // Called by <ActorBadge> for display
  resolve(userId: string | null): string {
    if (!userId) return m.audit_actor_system();
    return this.cache.get(userId)?.name ?? m.audit_actor_unknown();
  }
}
```

### Backend Support

Add batch user name endpoint:
```
POST /api/v1/users/names
Body: { "user_ids": ["uuid1", "uuid2", ...] }
Response: [{ "id": "uuid1", "name": "Ahmed", "name_ar": "أحمد" }, ...]
```

This resolves names without exposing sensitive user data.

---

## 11. Personalization Persistence

New columns and audit preferences are persisted via `usePersonalization`:

| Key | Type | Default | Purpose |
|---|---|---|---|
| `grid.{entity}.columns` | `string[]` | Existing columns only | Which columns are visible |
| `grid.{entity}.columnWidths` | `Record<string, number>` | Defaults | Column widths |
| `audit.timeline.pageSize` | `number` | 20 | Audit timeline page size |

When a user enables "Created By" column in the student grid, it's persisted and restored on next visit.

---

## 12. Implementation Checklist

### New Shared Components
- [ ] `<ActorBadge>` — User/System attribution display
- [ ] `<AuditTimeline>` — Paginated audit history viewer
- [ ] `<BranchBadge>` — OrgUnit display
- [ ] `<VersionConflictDialog>` — Optimistic locking conflict handler
- [ ] `UserNameResolver` — Batch user name resolution service

### Per-Feature Changes
- [ ] Employee directory — optional columns + audit tab in sheet
- [ ] Student list — optional columns + audit tab + branch selector in form
- [ ] Journal entries — optional columns + branch selector + version conflict
- [ ] Payments — branch column
- [ ] Grades — entered by / modified by columns
- [ ] Groups — optional columns + audit tab
- [ ] Classes — branch column + branch selector in form
- [ ] Chart of accounts — version conflict on edit
- [ ] Organization settings — version conflict on save
- [ ] Attendance — marked by column
- [ ] Rooms — branch column

### Context Menus
- [ ] Employee actions — "View Audit History"
- [ ] Student actions — "View Audit History"
- [ ] Journal entry actions — "View Audit History"
- [ ] Group actions — "View Audit History"
- [ ] Account actions — "View Audit History"

### Localization
- [ ] All Paraglide message keys defined (English + Arabic)
- [ ] Date/time formatting uses `orgContext.formatDate()`
- [ ] Number formatting uses `orgContext.formatNumber()`

### RTL
- [ ] Audit timeline layout verified in RTL
- [ ] Grid columns render correctly in RTL
- [ ] Version conflict dialog layout verified in RTL

### Accessibility
- [ ] All new components keyboard-navigable
- [ ] Screen reader labels present
- [ ] No color-only status indicators

### Testing
- [ ] TypeScript compiles with zero errors
- [ ] All new components have loading/empty/error states
- [ ] Permission gates verified
