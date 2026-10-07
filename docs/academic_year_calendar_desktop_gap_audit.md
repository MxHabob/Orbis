# Academic Year & Calendar Desktop Gap — Root-Cause Audit

> **Status**: `COMPLETE`
> **Date**: 2026-10-07
> **Scope**: Database → Backend → API → OpenAPI → SDK → Tauri → Local/Remote → Desktop Route → Navigation → Permissions → UI

---

## 1. Executive Summary

The Orbis Desktop currently has **no dedicated management page** for Academic Years or an Academic Calendar. However, the **entire backend stack is fully implemented** — from domain entities and database tables through API routes, OpenAPI schemas, SDK-generated types, Tauri IPC commands, and Desktop API client methods.

**The root cause is simple and singular:**

> **The Desktop integration was never completed at the UI/route/navigation layer.** The backend, API, SDK, and client code all exist. No SvelteKit route was created, no navigation item was added, and the only place academic years can be created is the **onboarding wizard** — which was designed as a one-time setup flow, not a permanent administration UI.

### Key Findings

| Layer | Academic Years | Academic Periods |
|-------|---------------|-----------------|
| Domain Entity | ✅ EXISTS | ✅ EXISTS |
| Database Table | ✅ EXISTS | ✅ EXISTS |
| Repository | ✅ EXISTS | ✅ EXISTS |
| Application Service | ✅ EXISTS | ✅ EXISTS |
| API Routes | ✅ EXISTS (CRUD) | ✅ EXISTS (CRUD + Lock/Unlock/Close) |
| OpenAPI Schema | ✅ EXISTS | ✅ EXISTS |
| SDK Types/Functions | ✅ EXISTS | ✅ EXISTS |
| Tauri Commands | ✅ EXISTS | ✅ EXISTS |
| LocalClient Methods | ✅ EXISTS | ✅ EXISTS |
| RemoteClient Methods | ✅ EXISTS | ✅ EXISTS |
| Desktop Route | ❌ **MISSING** | ⚠️ PARTIAL (Settings > Periods) |
| Desktop Navigation | ❌ **MISSING** | ⚠️ PARTIAL (Settings sidebar) |
| Desktop Permissions | ⚠️ Exists for periods only | ⚠️ No separate year permission |
| Desktop Management UI | ❌ **MISSING** | ⚠️ PARTIAL (periods-manager only) |
| Academic Calendar Screen | ❌ **MISSING** | ❌ **MISSING** |

---

## 2. Current Academic Domain

The Orbis academic domain is well-structured with a clear hierarchy:

```
Organization
  ↓
Academic Year (academic_years table)
  ↓
Academic Period — with parent_id forming hierarchy
  ├── Term (type: semester/trimester/quarter, parent_id: NULL)
  │   ├── Monthly Period (type: month, parent_id: term_id)
  │   └── Exam Period (kind: examination, parent_id: term_id)
  └── ...
```

### Domain Entities

| Entity | File | Status |
|--------|------|--------|
| `AcademicYear` | [academic_year.rs](file:///d:/projects/Orbis/apps/api-rust/crates/orbis-domain/src/entities/academic_year.rs) | ✅ Complete |
| `AcademicPeriod` | [academic_period.rs](file:///d:/projects/Orbis/apps/api-rust/crates/orbis-domain/src/entities/academic_period.rs) | ✅ Complete |
| `PeriodTemplate` | [period_template.rs](file:///d:/projects/Orbis/apps/api-rust/crates/orbis-domain/src/entities/period_template.rs) | ✅ Complete |

### Academic Year Entity Fields
- `id`, `org_id`, `name`, `name_ar`
- `start_date`, `end_date`, `hijri_start`, `hijri_end`
- `status` ("planning", "active", "closing", "archived")
- `is_locked`, `locked_at`, `locked_by`, `lock_reason`
- `current_period_id`, `archived_at`, `archived_by`
- `notes`, `created_at`, `updated_at`

### Academic Period Entity Fields
- `id`, `org_id`, `year_id`, `parent_id` (for tree structure)
- `name`, `name_ar`, `code`
- `period_type` — structural: Semester, Trimester, Quarter, Month, Module, Block, Custom
- `period_kind` — semantic: Teaching, Examination, Break, Registration, Preparatory
- `state` — lifecycle: Planning, EnrollmentOpen, ActiveTeaching, FinalExams, GradeSubmission, Closed, Archived
- Enrollment windows: `enrollment_starts_at/ends_at`, `is_enrollment_open`
- Grade windows: `grade_entry_starts_at/ends_at`, `is_grade_entry_open`
- Exam windows: `exam_starts_at`, `exam_ends_at`
- `is_locked`, `locked_at`, `locked_by`
- `sort_order`, `metadata`

> [!IMPORTANT]
> The backend model **already supports** the `Year → Term → Monthly Period / Exam Period` hierarchy via `parent_id` + `period_type` + `period_kind`. The distinction between teaching months and exam periods is correctly modeled through `PeriodKind::Teaching` vs `PeriodKind::Examination`.

---

## 3. Academic Year Backend Audit

### Domain Layer — ✅ EXISTS

- **Entity**: [`AcademicYear`](file:///d:/projects/Orbis/apps/api-rust/crates/orbis-domain/src/entities/academic_year.rs) — complete with status lifecycle and business methods (`is_writable()`, `can_enroll()`, `can_enter_grades()`)

### Repository Layer — ✅ EXISTS

- **Port**: [`AcademicYearRepository`](file:///d:/projects/Orbis/apps/api-rust/crates/orbis-app/src/ports/repositories.rs) — `find_by_id`, `find_by_org`, `find_current`, `save`
- **Implementation**: [`CalendarRepoImpl`](file:///d:/projects/Orbis/apps/api-rust/crates/orbis-infra/src/repositories/calendar_repos.rs#L23) — full upsert via SeaORM

### Application Service — ✅ EXISTS

[`CalendarService`](file:///d:/projects/Orbis/apps/api-rust/crates/orbis-app/src/services/calendar_service.rs) provides:
- `create_year()` — with `Permission::PeriodWrite` check
- `list_years()`
- `get_current_year()`
- `get_year()`
- `activate_year()` — ⚠️ **service method exists but NO API route exposed**
- `start_year_closing()` — ⚠️ **service method exists but NO API route exposed**
- `close_year()` — ⚠️ **service method exists but NO API route exposed**

> [!WARNING]
> The `CalendarService` has year lifecycle methods (`activate_year`, `start_year_closing`, `close_year`) that are **never exposed through API routes**. These are internal-only and cannot be invoked from the Desktop.

### API Routes — ✅ EXISTS (CRUD only)

[`calendar_routes.rs`](file:///d:/projects/Orbis/apps/api-rust/crates/orbis-server/src/api/calendar_routes.rs) exposes:

| Route | Method | Description |
|-------|--------|-------------|
| `/api/v1/academic-years` | POST | Create year |
| `/api/v1/academic-years` | GET | List years |
| `/api/v1/academic-years/current` | GET | Get current context |
| `/api/v1/academic-years/:id` | GET | Get year by ID |

**Missing API routes:**
- ❌ `PUT /api/v1/academic-years/:id` — update year
- ❌ `DELETE /api/v1/academic-years/:id` — delete year
- ❌ `POST /api/v1/academic-years/:id/activate` — activate year
- ❌ `POST /api/v1/academic-years/:id/close` — close year
- ❌ `POST /api/v1/academic-years/:id/archive` — archive year

---

## 4. Academic Calendar Backend Audit

**There is NO separate "calendar" domain entity in Orbis.** The concept of "Academic Calendar" is represented by:
- `AcademicYear` (date boundaries)
- `AcademicPeriod` tree (terms, months, exam periods)
- `PeriodKind::Break` (holidays/vacations)

### Calendar Capabilities

| Capability | Status | Evidence |
|-----------|--------|----------|
| Academic year boundaries | ✅ EXISTS | `AcademicYear.start_date/end_date` |
| Term boundaries | ✅ EXISTS | `AcademicPeriod` with `parent_id: NULL`, `period_type: semester` |
| Period date ranges | ✅ EXISTS | `AcademicPeriod.start_date/end_date` |
| School days | ❌ MISSING | No dedicated table/entity |
| Holidays | ⚠️ PARTIAL | `PeriodKind::Break` exists, but no dedicated holiday table |
| Vacations | ⚠️ PARTIAL | Modeled as `Break` periods |
| Academic events | ❌ MISSING | No events/calendar-events entity |
| Registration periods | ✅ EXISTS | `enrollment_starts_at/enrollment_ends_at` |
| Period opening/closing | ✅ EXISTS | `AcademicPeriodState` state machine |
| Academic locks | ✅ EXISTS | `is_locked`, `locked_at`, `locked_by` |

> [!NOTE]
> The "Academic Calendar" in Orbis is NOT a separate domain — it is the **visualization** of Academic Year + Periods + their states. No additional backend entities are needed for a calendar management screen. A calendar UI should render the existing data from the `getAcademicContext()` endpoint.

---

## 5. Term/Period Hierarchy Audit

### Period Hierarchy Model — ✅ CORRECT

```
Academic Year 1447/1448
├── Term 1 (type: semester, kind: teaching, parent_id: null)
│   ├── Month 1 (type: month, kind: teaching, parent_id: term1)
│   ├── Month 2 (type: month, kind: teaching, parent_id: term1)
│   ├── Month 3 (type: month, kind: teaching, parent_id: term1)
│   └── Term 1 Exam (type: month, kind: examination, parent_id: term1)
│
└── Term 2 (type: semester, kind: teaching, parent_id: null)
    ├── Month 4 (type: month, kind: teaching, parent_id: term2)
    ├── Month 5 (type: month, kind: teaching, parent_id: term2)
    ├── Month 6 (type: month, kind: teaching, parent_id: term2)
    └── Term 2 Exam (type: month, kind: examination, parent_id: term2)
```

### Period State Machine — ✅ CORRECT

```
Planning → EnrollmentOpen → ActiveTeaching → FinalExams → GradeSubmission → Closed → Archived
```

---

## 6. Database/Migration Audit

### Tables — ✅ EXISTS

Created in [`m20240101_000001_initial_schema.rs`](file:///d:/projects/Orbis/apps/api-rust/crates/orbis-infra/src/migration/m20240101_000001_initial_schema.rs):
- `academic_years` — via `academic_year::Entity`
- `academic_periods` — via `academic_period::Entity`
- `period_templates` — via `period_template::Entity`

### Schema Observations
- ⚠️ No `UNIQUE(org_id, name)` constraint on academic_years — could allow duplicate year names
- ⚠️ No `SeaORM Relation` defined between academic_year and academic_periods (empty `Relation` enums)
- ⚠️ No cascading delete defined — deleting a year won't cascade to periods

---

## 7. OpenAPI Audit — ✅ EXISTS

In [`lib.rs`](file:///d:/projects/Orbis/apps/api-rust/crates/orbis-server/src/lib.rs#L363-L372), registered schemas:
- `AcademicYearDto`, `AcademicPeriodDto`
- `PeriodKind`, `PeriodType`, `AcademicPeriodState`
- `AcademicContextResponse`
- `CreateYearRequest`, `CreatePeriodRequest`
- `PeriodTemplateDto`

---

## 8. SDK Audit — ✅ EXISTS

[`types.gen.ts`](file:///d:/projects/Orbis/packages/sdk/src/gen/types.gen.ts#L67-L80) — `AcademicYearDto` type generated.

[`sdk.gen.ts`](file:///d:/projects/Orbis/packages/sdk/src/gen/sdk.gen.ts) — Functions: `listYears`, `createYear`, `getCurrentContext`, `getYear`, `createPeriod`, `listPeriods`, `getPeriod`, `lockPeriod`, `unlockPeriod`, `closePeriod`, `listPeriodTemplates`.

---

## 9. Tauri IPC Commands — ✅ EXISTS

[`calendar_cmds.rs`](file:///d:/projects/Orbis/apps/desktop/src-tauri/src/commands/calendar_cmds.rs) — 11 commands registered in [`mod.rs`](file:///d:/projects/Orbis/apps/desktop/src-tauri/src/commands/mod.rs#L85-L94):

| Command | Maps To |
|---------|---------|
| `calendar_create_year` | `CalendarService::create_year` |
| `calendar_list_years` | `CalendarService::list_years` |
| `calendar_get_year` | `CalendarService::get_year` |
| `calendar_get_context` | Builds `AcademicContextResponse` |
| `calendar_create_period` | `CalendarService::create_period` |
| `calendar_list_periods` | `CalendarService::list_periods_by_year/org` |
| `calendar_get_period` | `CalendarService::get_period` |
| `calendar_lock_period` | `CalendarService::lock_period` |
| `calendar_unlock_period` | `CalendarService::unlock_period` |
| `calendar_close_period` | `CalendarService::close_period` |
| `calendar_list_period_templates` | `PeriodTemplateRepo::find_by_org` |

---

## 10. Desktop API Client Audit — ✅ EXISTS

### OrbisApi Interface ([`types.ts`](file:///d:/projects/Orbis/apps/desktop/src/lib/api/types.ts#L1363-L1374))

```typescript
// Calendar section of OrbisApi interface
createAcademicYear(req: CreateYearRequest): Promise<AcademicYearDto>;
listAcademicYears(): Promise<AcademicYearDto[]>;
getAcademicContext(): Promise<AcademicContextResponse>;
getAcademicYear(id: string): Promise<AcademicYearDto>;
createAcademicPeriod(req: CreatePeriodRequest): Promise<AcademicPeriodDto>;
listAcademicPeriods(): Promise<AcademicPeriodDto[]>;
getAcademicPeriod(id: string): Promise<AcademicPeriodDto>;
lockAcademicPeriod(id: string): Promise<AcademicPeriodDto>;
unlockAcademicPeriod(id: string): Promise<AcademicPeriodDto>;
closeAcademicPeriod(id: string): Promise<AcademicPeriodDto>;
listPeriodTemplates(): Promise<PeriodTemplateDto[]>;
```

Both [`LocalClient`](file:///d:/projects/Orbis/apps/desktop/src/lib/api/local-client.ts#L359-L395) (Tauri IPC) and [`RemoteClient`](file:///d:/projects/Orbis/apps/desktop/src/lib/api/remote-client.ts#L465-L500) (SDK HTTP) implement all methods identically.

### Missing from both clients
- ❌ `updateAcademicYear` — no update route exists in backend
- ❌ `deleteAcademicYear` — no delete route exists
- ❌ `activateYear` — service method exists but no route
- ❌ `closeYear` — service method exists but no route
- ❌ `updateAcademicPeriod` — no update route exists
- ❌ `deleteAcademicPeriod` — no delete route exists

---

## 11. Desktop Route Audit — ❌ MISSING

### Existing Routes Under `/(app)/[orgId]/`

| Route | Exists | Academic Year Related |
|-------|--------|---------------------|
| `/academics` | ✅ | Overview page only |
| `/academics/classes` | ✅ | No |
| `/academics/schedule` | ✅ | No |
| `/academics/exams` | ✅ | No |
| `/academics/promotion` | ✅ | No |
| `/academics/reports` | ✅ | No |
| `/settings/periods` | ✅ | Manages periods only, not years |
| `/settings/academic-policy` | ✅ | Policy settings, not year management |
| `/onboarding` | ✅ | Year creation during onboarding only |
| **`/academic-years`** | ❌ **MISSING** | — |
| **`/academic-calendar`** | ❌ **MISSING** | — |

---

## 12. Navigation Audit — ❌ MISSING

### Main Sidebar ([`app-sidebar.svelte`](file:///d:/projects/Orbis/apps/desktop/src/lib/shell/app-sidebar.svelte))

8 top-level items: Overview, Students, Employees, **Academics**, Finance, Templates, AI Center, Settings.

**No Academic Year or Calendar entry exists.**

### Academics Sub-Navigation ([`academics/+layout.svelte`](file:///d:/projects/Orbis/apps/desktop/src/routes/%28app%29/%5BorgId%5D/academics/+layout.svelte))

6 items: Overview, Classes, Schedule, Exams, Promotion, Reports.

**No Academic Year or Calendar entry exists.**

### Settings Sub-Navigation ([`settings/+layout.svelte`](file:///d:/projects/Orbis/apps/desktop/src/routes/%28app%29/%5BorgId%5D/settings/+layout.svelte))

Under "Academic Rules" group: **Periods**, Subjects, Rooms, Academic Policy, Assessment Schemes, Registration, Approvals.

**"Periods" manages periods only — no year management or calendar visualization.**

---

## 13. Permission Audit

### Backend Permissions ([`permission.rs`](file:///d:/projects/Orbis/apps/api-rust/crates/orbis-domain/src/iam/permission.rs))

| Permission | Used For |
|-----------|----------|
| `period:write` | Create year, create period |
| `period:lock` | Lock period |
| `period:unlock` | Unlock period |

> [!WARNING]
> **No dedicated `academic_year:*` permission exists.** All year operations reuse `Permission::PeriodWrite`.

### Desktop Permission Constants ([`permissions.ts`](file:///d:/projects/Orbis/apps/desktop/src/lib/core/iam/permissions.ts#L80-L85))

```typescript
PeriodRead: "period:read",
PeriodWrite: "period:write",
PeriodActivate: "period:activate",  // ← defined but NOT used by backend
PeriodLock: "period:lock",
PeriodUnlock: "period:unlock",
```

---

## 14. Onboarding Audit — ⚠️ ONLY PLACE FOR YEAR CREATION

### Calendar Step ([`step-3-calendar.svelte`](file:///d:/projects/Orbis/apps/desktop/src/lib/features/onboarding-wizard/steps/step-3-calendar.svelte))

The onboarding wizard's step 3 ("Calendar") provides:
- Academic year creation form (name, name_ar, start_date, end_date)
- Semester/trimester/quarter generation
- Hijri calendar integration

### Onboarding Steps Config ([`onboarding-steps.ts`](file:///d:/projects/Orbis/apps/desktop/src/lib/features/onboarding-wizard/onboarding-steps.ts#L68-L75))

```typescript
{ id: "calendar", route: "calendar", taskIds: ["academic_year", "academic_periods"], order: 3 }
```

> [!CAUTION]
> **Onboarding is the ONLY place a user can create an academic year.** After onboarding, there is no UI to:
> - Create additional academic years
> - Edit an existing academic year
> - Activate/deactivate/close/archive a year
> - View year-to-year comparison
> - Plan the next academic year

---

## 15. Settings vs Domain Audit — ✅ CORRECT

[`OrganizationSettings`](file:///d:/projects/Orbis/apps/api-rust/crates/orbis-domain/src/entities/settings.rs) contains only configuration:
- `AcademicSettings` — grading system, passing thresholds, ranking
- `EnrollmentSettings`, `FinanceSettings`, `NotificationSettings`, `AutopilotSettings`

**No academic year data leaks into settings.** Correct architecture.

---

## 16. Existing Periods Manager Audit — ⚠️ PARTIAL

[`periods-manager.svelte`](file:///d:/projects/Orbis/apps/desktop/src/lib/features/settings/periods-manager.svelte) provides:
- Period listing (flat, not grouped by year)
- Period creation via sheet
- Period lock/unlock/activate lifecycle
- ⚠️ Delete button exists but **doesn't actually delete** (shows toast only, no API call)
- ❌ No year management
- ❌ No year grouping/filtering
- ❌ No year creation

---

## 17. Academic Context Store — ✅ EXISTS (READ-ONLY)

[`academic-context.svelte.ts`](file:///d:/projects/Orbis/apps/desktop/src/lib/stores/academic-context.svelte.ts) is extensively used (40+ components):
- Provides `currentYear`, `allYears`, `periods`, `yearStatus`
- Supports `switchYear()` — year switching works at store level
- Used by: grades, attendance, exams, scheduling, reporting, AI sidebar
- **Read-only consumption** — no mutation operations exposed to UI

---

## 18. Root-Cause Matrix

| Capability | Backend | API | SDK | Tauri | Client | Route | Nav | UI | Root Cause |
|------------|---------|-----|-----|-------|--------|-------|-----|----|------------|
| **Create Year** | ✅ | ✅ | ✅ | ✅ | ✅ | ❌ | ❌ | ❌ | `ROUTE+NAV+UI MISSING` — Only in onboarding |
| **List Years** | ✅ | ✅ | ✅ | ✅ | ✅ | ❌ | ❌ | ❌ | `ROUTE+UI MISSING` |
| **View Year** | ✅ | ✅ | ✅ | ✅ | ✅ | ❌ | ❌ | ❌ | `ROUTE+UI MISSING` |
| **Update Year** | ⚠️ save() | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | `API_ROUTE MISSING` |
| **Activate Year** | ✅ service | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | `API_ROUTE MISSING` |
| **Close Year** | ✅ service | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | `API_ROUTE MISSING` |
| **Delete Year** | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | `BACKEND MISSING` |
| **Manage Periods** | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | **WORKS** (partial) |
| **Update Period** | ⚠️ save() | ❌ | ❌ | ❌ | ❌ | — | — | ❌ | `API_ROUTE MISSING` |
| **Delete Period** | ❌ | ❌ | ❌ | ❌ | ❌ | — | — | ❌ | `BACKEND MISSING` |
| **Calendar View** | ✅ data | ✅ | ✅ | ✅ | ✅ | ❌ | ❌ | ❌ | `ROUTE+UI MISSING` |

---

## 19. Recommended Desktop Information Architecture

### Option A (Recommended): Extend Settings > Academic Rules

```
Settings
├── General
├── Organization
├── Academic Rules               ← EXTEND THIS GROUP
│   ├── ⭐ Academic Years       ← NEW: Year CRUD + lifecycle + period tree
│   ├── Periods                  ← EXISTING: Enhance with year grouping
│   ├── Subjects
│   ├── Rooms
│   ├── Academic Policy
│   ├── Assessment Schemes
│   ├── Registration
│   └── Approvals
├── Finance
└── Operations
```

### Option B: Add Calendar to Academics Section

```
Academics
├── Overview
├── ⭐ Calendar                 ← NEW: Visual timeline of year + periods
├── Classes
├── Schedule
├── Exams
├── Promotion
└── Reports
```

### Recommendation

**Do both**, with clear separation:
- **Settings > Academic Years** = **Administration** (create/edit/lifecycle/period tree management)
- **Academics > Calendar** = **Visualization** (read-only timeline/Gantt of current year, links to Settings for editing)

This avoids scattered pages and keeps editing in Settings (where all configuration lives) while giving academics users a calendar view.

---

## 20. Required Implementation Phases

### Phase 1: Backend API Completion

1. Add `PUT /api/v1/academic-years/:id` route (update year)
2. Add `POST /api/v1/academic-years/:id/activate` route
3. Add `POST /api/v1/academic-years/:id/close` route
4. Add `POST /api/v1/academic-years/:id/archive` route
5. Add `PUT /api/v1/academic-periods/:id` route (update period)
6. Regenerate SDK
7. Add corresponding Tauri commands
8. Add LocalClient/RemoteClient methods

### Phase 2: Academic Year Management Page

1. Create route `/(app)/[orgId]/settings/academic-years/`
2. Add navigation item to Settings sidebar under "Academic Rules"
3. Build Academic Year management component:
   - Year list with status badges
   - Create year dialog/sheet
   - Year detail panel with edit
   - Period tree viewer (terms → months → exams)
   - Lifecycle buttons (activate, close, archive)
4. Enhance `periods-manager` to group by year

### Phase 3: Academic Calendar Visualization

1. Create route `/(app)/[orgId]/academics/calendar/`
2. Add navigation item to Academics sidebar
3. Build calendar visualization (timeline/Gantt of current year)

---

## 21. Risks

| Risk | Severity | Description |
|------|----------|-------------|
| Onboarding as only admin UI | **HIGH** | Users who complete onboarding have no way to manage years afterward |
| Missing update/delete APIs | MEDIUM | Backend `save()` supports upsert but no explicit routes exist |
| Year lifecycle routes unexposed | MEDIUM | `activate_year()`, `close_year()` exist in service but have no API routes |
| No year-level permission | LOW | All year operations reuse `period:write` |
| No overlapping year validation | LOW | Backend doesn't prevent duplicate/overlapping year dates |
| Period delete is fake | MEDIUM | `periods-manager` delete button shows toast but doesn't call any API |

---

## 22. Correct Domain Boundaries (DO NOT CHANGE)

```
CalendarService (orbis-app)
├── AcademicYear management
├── AcademicPeriod management (CRUD + state transitions)
└── PeriodTemplate management

OrganizationSettings (separate — config only)
├── Academic grading/ranking policies
├── Enrollment settings
└── ...
```

**DO NOT:**
- Create a separate `AcademicYearService` — use existing `CalendarService`
- Move year data into `OrganizationSettings`
- Create duplicate period services
- Introduce a separate "calendar events" domain until requirements justify it

---

## 23. Files Examined

### Backend (apps/api-rust/)
| File | Purpose |
|------|---------|
| [academic_year.rs](file:///d:/projects/Orbis/apps/api-rust/crates/orbis-domain/src/entities/academic_year.rs) | Domain entity |
| [academic_period.rs](file:///d:/projects/Orbis/apps/api-rust/crates/orbis-domain/src/entities/academic_period.rs) | Domain entity |
| [settings.rs](file:///d:/projects/Orbis/apps/api-rust/crates/orbis-domain/src/entities/settings.rs) | Org settings |
| [permission.rs](file:///d:/projects/Orbis/apps/api-rust/crates/orbis-domain/src/iam/permission.rs) | IAM permissions |
| [repositories.rs](file:///d:/projects/Orbis/apps/api-rust/crates/orbis-app/src/ports/repositories.rs) | Port traits |
| [calendar_service.rs](file:///d:/projects/Orbis/apps/api-rust/crates/orbis-app/src/services/calendar_service.rs) | App service |
| [calendar_repos.rs](file:///d:/projects/Orbis/apps/api-rust/crates/orbis-infra/src/repositories/calendar_repos.rs) | Repo impl |
| [academic_year model](file:///d:/projects/Orbis/apps/api-rust/crates/orbis-infra/src/models/academic_year.rs) | SeaORM model |
| [academic_period model](file:///d:/projects/Orbis/apps/api-rust/crates/orbis-infra/src/models/academic_period.rs) | SeaORM model |
| [calendar_routes.rs](file:///d:/projects/Orbis/apps/api-rust/crates/orbis-server/src/api/calendar_routes.rs) | HTTP routes |
| [lib.rs](file:///d:/projects/Orbis/apps/api-rust/crates/orbis-server/src/lib.rs) | OpenAPI config |
| [initial_schema.rs](file:///d:/projects/Orbis/apps/api-rust/crates/orbis-infra/src/migration/m20240101_000001_initial_schema.rs) | DB migration |

### Desktop
| File | Purpose |
|------|---------|
| [calendar_cmds.rs](file:///d:/projects/Orbis/apps/desktop/src-tauri/src/commands/calendar_cmds.rs) | Tauri IPC |
| [types.ts](file:///d:/projects/Orbis/apps/desktop/src/lib/api/types.ts) | API contract |
| [local-client.ts](file:///d:/projects/Orbis/apps/desktop/src/lib/api/local-client.ts) | Local client |
| [remote-client.ts](file:///d:/projects/Orbis/apps/desktop/src/lib/api/remote-client.ts) | Remote client |
| [academic-context.svelte.ts](file:///d:/projects/Orbis/apps/desktop/src/lib/stores/academic-context.svelte.ts) | Store |
| [permissions.ts](file:///d:/projects/Orbis/apps/desktop/src/lib/core/iam/permissions.ts) | IAM |
| [app-sidebar.svelte](file:///d:/projects/Orbis/apps/desktop/src/lib/shell/app-sidebar.svelte) | Main nav |
| [academics/+layout.svelte](file:///d:/projects/Orbis/apps/desktop/src/routes/%28app%29/%5BorgId%5D/academics/+layout.svelte) | Academics nav |
| [settings/+layout.svelte](file:///d:/projects/Orbis/apps/desktop/src/routes/%28app%29/%5BorgId%5D/settings/+layout.svelte) | Settings nav |
| [periods-manager.svelte](file:///d:/projects/Orbis/apps/desktop/src/lib/features/settings/periods-manager.svelte) | Period UI |
| [onboarding-steps.ts](file:///d:/projects/Orbis/apps/desktop/src/lib/features/onboarding-wizard/onboarding-steps.ts) | Onboarding |
| [step-3-calendar.svelte](file:///d:/projects/Orbis/apps/desktop/src/lib/features/onboarding-wizard/steps/step-3-calendar.svelte) | Year creation |

### SDK
| File | Purpose |
|------|---------|
| [types.gen.ts](file:///d:/projects/Orbis/packages/sdk/src/gen/types.gen.ts) | Generated types |
| [sdk.gen.ts](file:///d:/projects/Orbis/packages/sdk/src/gen/sdk.gen.ts) | Generated functions |

---

## 24. Final Summary

### Root Causes (ordered by impact)

1. **`ROUTE_MISSING` + `NAVIGATION_MISSING` + `UI_MISSING`** — The entire backend → client chain is complete. No SvelteKit route, navigation entry, or management component was ever created for academic years.

2. **`API_ROUTE_MISSING`** — Year lifecycle operations (`activate`, `close`, `archive`, `update`) exist as service methods but were never exposed as HTTP/Tauri routes.

3. **`ONBOARDING_ONLY`** — Academic year creation is exclusively available through the one-time onboarding wizard.

4. **`PARTIAL_UI`** — `periods-manager` manages periods but doesn't group by year, doesn't allow year creation, and has non-functional delete buttons.

**The fix is NOT to redesign the domain** — the domain is correct. **The fix is to complete the Desktop integration** by adding the missing API routes, regenerating SDK, adding Tauri commands, creating the SvelteKit route, and adding navigation entries.
