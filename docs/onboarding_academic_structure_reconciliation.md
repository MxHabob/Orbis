# Orbis — Onboarding / Academic Structure / Facilities / Grading / DocForge
# Full Reconciliation Audit & Implementation Plan

**Document Version:** 1.0.0  
**Audit Date:** 2026-09-29  
**Status:** AUDIT COMPLETE — IMPLEMENTATION PENDING

---

## Table of Contents

1. [Current Architecture Summary](#1-current-architecture-summary)
2. [Current Errors & Defects](#2-current-errors--defects)
3. [Correct Business Model](#3-correct-business-model)
4. [Bootstrap Ownership](#4-bootstrap-ownership)
5. [Onboarding Ownership](#5-onboarding-ownership)
6. [Academic Year Model](#6-academic-year-model)
7. [Term Model](#7-term-model)
8. [Monthly Period Model](#8-monthly-period-model)
9. [Assessment Model](#9-assessment-model)
10. [Calculation Rules](#10-calculation-rules)
11. [Rounding Policy](#11-rounding-policy)
12. [Ranking Policy](#12-ranking-policy)
13. [Pass/Fail Policy](#13-passfail-policy)
14. [Institution-Type Defaults](#14-institution-type-defaults)
15. [Branch/OrgUnit Initialization](#15-branchorgunit-initialization)
16. [Building/Floor/Room/Location Model](#16-buildingfloorlocation-model)
17. [DocForge Architecture](#17-docforge-architecture)
18. [Template Architecture](#18-template-architecture)
19. [Report Data Providers](#19-report-data-providers)
20. [Local/Remote Parity](#20-localremote-parity)
21. [Database Changes](#21-database-changes)
22. [API/SDK Changes](#22-apisdk-changes)
23. [Desktop Changes](#23-desktop-changes)
24. [Tests](#24-tests)
25. [Performance Measurements](#25-performance-measurements)
26. [Migration Risks](#26-migration-risks)
27. [Reconciliation Matrix](#27-reconciliation-matrix)
28. [Implementation Phases](#28-implementation-phases)

---

## 1. Current Architecture Summary

### Backend Stack (Rust Clean Architecture)

| Crate | Purpose | Key Files |
|-------|---------|-----------|
| `orbis-domain` | Pure domain entities, value objects, invariants | `academic_period.rs`, `academic_year.rs`, `academic_policy.rs`, `assessment_scheme.rs`, `room.rs`, `org_unit.rs`, `period_template.rs`, `progression.rs`, `report.rs` |
| `orbis-app` | Use cases, ports, services | `bootstrap_service.rs`, `onboarding_service.rs`, `calendar_service.rs`, `academic_service.rs`, `assessment_service.rs`, `calculation_engine.rs`, `progression_service.rs`, `organization_service.rs`, `scheduling_service.rs`, `report_service.rs` |
| `orbis-infra` | SeaORM repositories, adapters, migrations | `bootstrap_adapter.rs` (120KB), `report_adapter.rs`, `report_defaults.rs`, `calendar_repos.rs`, `scheduling_repos.rs` |
| `orbis-server` | Axum REST API, DTOs, OpenAPI | `calendar_routes.rs`, `assessment_routes.rs`, `scheduling_routes.rs`, `org_routes.rs`, `report_routes.rs` |
| `docforge-engine` | 18-node AST vector reporting engine | RTL Arabic HarfBuzz shaping |

### Desktop Stack (SvelteKit 2 + Svelte 5 + Tauri 2)

| Layer | Purpose | Key Files |
|-------|---------|-----------|
| Onboarding Wizard | 9-step setup flow | `state.svelte.ts`, `templates.ts`, `types.ts`, 11 step components |
| API Layer | Dual-transport (Local/Remote) | `types.ts`, `local-client.ts`, `remote-client.ts` |
| Tauri IPC | Desktop-native commands | `bootstrap_cmds.rs`, `calendar_cmds.rs`, `scheduling_cmds.rs`, `assessment_cmds.rs` |
| SDK | Generated TypeScript client | `@orbis/sdk` via `@hey-api/openapi-ts` |

### Initialization Flow

```
Bootstrap → Organization + Admin + Roles + Permissions + PeriodTemplates + AssessmentScheme + Currency + Settings
    ↓
Onboarding → AcademicYear + Periods + Classes + Subjects + TimeSlots + Buildings + Rooms + COA + Fees + Discounts + Reports + DocumentRequirements
```

---

## 2. Current Errors & Defects

### CRITICAL: Period Hierarchy Not Correctly Modeled in Desktop Templates

**Root Cause:** The default period template (`template-two-semesters`) creates only top-level semester entries with NO children. The template that DOES include children (`template-semesters-monthly`) has only 2 children per term (Month 1, Month 2, Final Exam) instead of the required 4 (Month 1, Month 2, Month 3, Final Exam).

**Evidence:**
- `templates.ts:948-966` — Default template `template-two-semesters` has only 2 flat semester nodes with no children
- `templates.ts:1024-1082` — `template-semesters-monthly` has 3 children per semester (2 months + exam) instead of 4 (3 months + exam)
- The CORRECT template exists in the BACKEND at `bootstrap_service.rs:621-651` with the right structure (3 months + exam per term), but it uses `periods` key while Desktop uses `children` key

**Impact:** When a school uses the default Desktop template, NO monthly assessment sub-periods are created. When they use the "monthly" template, they get WRONG structure.

### CRITICAL: Period Parent/Child IDs Missing from API Contract

**Root Cause:** `CompleteOrganizationOnboardingRequest` in `types.ts:731-816` does NOT include `id` or `parentId` in the periods array. The backend `OnboardingPeriodItemDto` in `onboarding_service.rs:76-91` DOES support `id` and `parent_id`, but the Desktop type omits them.

**Impact:** Even when Desktop generates child periods in `state.svelte.ts:746-755`, the generated `parentId` is lost because the TypeScript type doesn't declare it. The hierarchy is flattened on submission.

### CRITICAL: Default Branch/OrgUnit Not Created During Bootstrap

**Root Cause:** Bootstrap (`bootstrap_service.rs`) creates ONLY: organization, admin person/user, roles, permissions, period templates, assessment scheme, currency, settings. It does NOT create any OrgUnit/Branch. The root campus OrgUnit is created during ONBOARDING (not bootstrap) in `bootstrap_adapter.rs:1857-1865`.

**Impact:** Between bootstrap and onboarding completion, there is NO default branch. Any feature requiring `org_unit_id` will fail. If onboarding fails partially, rooms and classes have no parent unit.

### SERIOUS: Room Persistence Uses Date.now() Synthetic IDs

**Root Cause:** `state.svelte.ts:225-280` generates room IDs using `Date.now()` and `Math.random()`. These are local-only identifiers that are NOT sent to the backend — the backend generates UUIDs during `persist_onboarding`. However, the frontend holds stale local IDs that don't match backend IDs.

**Evidence:** `state.svelte.ts:430` — `id: room-${Date.now()}-${Math.random()...}`

**Impact:** After onboarding, the room IDs in any cached Desktop state don't match the database. Room operations post-onboarding may reference non-existent IDs.

### SERIOUS: Building/Floor Model Mismatch

**Root Cause:** The Desktop `Room` type (`types.ts:19-27`) includes `floor: number`, but the backend `ValidatedOnboardingRoom` (`onboarding.rs:97-105`) has NO floor field. Floor data entered in the Desktop wizard is LOST during submission.

The backend Room entity (`room.rs`) has no floor or building reference — only `org_unit_id`. Buildings are stored as OrgUnit entries with `unit_type: 'building'`.

**Impact:** Floor information entered during onboarding is silently discarded. The room-to-building relationship is indirectly via `org_unit_id`, which may not resolve correctly because the building OrgUnit might not be the one the user selected.

### SERIOUS: Assessment Scheme Uses f64, Domain Uses Decimal

**Root Cause:** `AssessmentScheme` entity (`assessment_scheme.rs:11`) uses `f64` for `max_score`. The calculation engine (`calculation_engine.rs`) uses `rust_decimal::Decimal` throughout. The bootstrap creates assessment schemes with `f64` max_score (`bootstrap_service.rs:478: 100.0`).

**Impact:** Potential floating-point disagreement between the scheme definition and the calculation engine. Not immediately dangerous for 100.0, but architecturally incorrect.

### MODERATE: Onboarding Period Name Concatenation

**Root Cause:** `state.svelte.ts:749` concatenates period names: `${item.name || 'Term ${i + 1}'} - ${child.name || 'Month ${j + 1}'}`. This creates names like "First Semester - Month 1 Assessment" instead of letting the child period have its own independent name.

**Impact:** Incorrect period names in database. Period names should be standalone (e.g., "الشهر الأول"), not concatenated with their parent term name.

### MODERATE: Template Structure Mismatch Between Backend and Desktop

**Root Cause:** Backend period template structure (`bootstrap_service.rs:626-651`) uses `periods` as the key for children. Desktop template structure (`templates.ts`) uses `children`. They describe the same concept with different keys.

**Impact:** Period templates created by bootstrap cannot be directly consumed by the Desktop wizard without key remapping.

### MODERATE: Classes Linked to First Period Instead of Academic Year

**Root Cause:** `bootstrap_adapter.rs:1962-1975` — `first_period_id = data.periods.first().map(|p| p.id).unwrap_or(data.academic_year.id)`. Classes are linked to the first period, not the academic year. If the first period is a term, classes belong to that term, not the full year.

**Impact:** Classes may not appear when querying by a different period. A class should typically be scoped to the academic year, not a specific term.

### MINOR: Academic Year Status as String

**Root Cause:** `AcademicYear` entity uses `status: String` with comments listing valid values ("planning", "active", "closing", "archived"). This is not a typed enum.

**Impact:** No compile-time guarantees on valid status values.

### MINOR: Time Slots Linked to First Period

**Root Cause:** `bootstrap_adapter.rs:2017-2018` — Time slots use `first_period_id`, which may be a term rather than the academic year.

---

## 3. Correct Business Model

### Canonical Academic Hierarchy

```
Organization
  └── Academic Year (2026-2027)
        ├── Term 1 (الفصل الدراسي الأول)
        │     ├── Month 1 (الشهر الأول)           — Grading Period, 100 points
        │     ├── Month 2 (الشهر الثاني)          — Grading Period, 100 points
        │     ├── Month 3 (الشهر الثالث)          — Grading Period, 100 points
        │     └── Term 1 Final Exam (اختبار نهاية الفصل الأول) — 30 points
        │
        └── Term 2 (الفصل الدراسي الثاني)
              ├── Month 1 (الشهر الأول)           — Grading Period, 100 points
              ├── Month 2 (الشهر الثاني)          — Grading Period, 100 points
              ├── Month 3 (الشهر الثالث)          — Grading Period, 100 points
              └── Term 2 Final Exam (اختبار نهاية الفصل الثاني) — 30 points
```

### Key Distinctions

| Concept | Semantic Role | Database Entity | Parent |
|---------|---------------|-----------------|--------|
| Academic Year | Container for all academic activity | `academic_years` | Organization |
| Term | Parent grouping (الفصل الدراسي) | `academic_periods` (type=term) | Academic Year |
| Monthly Grading Period | Period for entering monthly grades | `academic_periods` (type=month) | Term |
| Term Final Exam | End-of-term examination period | `academic_periods` (type=exam) | Term |

### Grading Formula (School Default)

```
Monthly Grade (each of 3 months):
  Assignments  = /20
  Oral         = /20
  Attendance   = /20
  Written Test = /40
  Total        = /100

Term Result:
  Monthly Component = average(Month1, Month2, Month3) × (20/100) = /20
  Final Exam        = /30
  Term Total        = Monthly Component + Final Exam = /50

Annual Result:
  Term 1 Total = /50
  Term 2 Total = /50
  Annual Total = Term 1 + Term 2 = /100
```

---

## 4. Bootstrap Ownership

### Current State: ✅ CORRECT BOUNDARIES

Bootstrap creates the minimum technical prerequisites:
- Organization record
- Admin Person + User
- System Roles (8 roles)
- Role Permissions
- Period Templates (3 school + university/institute templates)
- Assessment Scheme (institution-type-specific formula)
- Currency
- Organization Settings JSON

### Required Fix: Add Default OrgUnit/Branch

**Bootstrap SHOULD also create a root OrgUnit** so that any subsequent operation has a valid branch reference. Currently, this is deferred to onboarding.

---

## 5. Onboarding Ownership

### Current State: ✅ MOSTLY CORRECT

Onboarding creates operational configuration:
- Updates Organization profile/settings
- Creates Root Campus OrgUnit (should be bootstrap)
- Creates Building OrgUnits
- Creates Rooms
- Creates Academic Year
- Creates Academic Periods (with parent_id hierarchy)
- Creates Classes (linked to first period — WRONG)
- Creates Subjects
- Creates Time Slots (linked to first period — WRONG)
- Seeds COA, Fee Templates, Discount Templates
- Seeds Assessment Schemes
- Seeds Roles/Permissions (redundant with bootstrap)
- Seeds Default Reports
- Seeds Document Requirements

### Required Fixes:
1. Move root campus creation to Bootstrap
2. Fix class `period_id` → should be year_id or term_id based on context
3. Fix time slot `period_id` → should reference academic year, not first period
4. Remove redundant roles/permissions seeding (already done in bootstrap)
5. Add period hierarchy validation (terms must contain the correct number of child periods)

---

## 6. Academic Year Model

### Current State: ✅ ADEQUATE

```rust
pub struct AcademicYear {
    id, org_id, name, name_ar,
    start_date, end_date,
    hijri_start, hijri_end,
    status: String, // "planning", "active", "closing", "archived"
    is_locked, locked_at, locked_by, lock_reason,
    current_period_id,
    archived_at, archived_by, notes,
    created_at, updated_at,
}
```

### Required Fix: Type `status` as Enum

The `status` field should be a typed enum, not a bare string.

---

## 7. Term Model

### Current State: ✅ ADEQUATE (uses AcademicPeriod with parent_id=None)

Terms are modeled as `AcademicPeriod` entries with:
- `year_id` → parent academic year
- `parent_id` → `None` (top-level within the year)
- `period_type` → "term" or "semester"

### Required Fix: None (model is correct)

The problem is NOT the backend model — it's the Desktop template structure and submission contract that fails to create terms WITH children.

---

## 8. Monthly Period Model

### Current State: ✅ ADEQUATE (uses AcademicPeriod with parent_id=term_id)

Monthly grading periods are modeled as `AcademicPeriod` entries with:
- `year_id` → parent academic year
- `parent_id` → parent term's UUID
- `period_type` → "month" or "exam"

### Required Fix: Desktop must send correct parent_id

The backend `OnboardingPeriodItemDto` already supports `parent_id`. The Desktop contract must include `id` and `parentId` fields.

---

## 9. Assessment Model

### Current State: ✅ WELL DESIGNED

The domain model in `academic_policy.rs` includes:
- `MonthlyAssessmentBreakdown` (assignments/oral/attendance/written_test with max values)
- `SubjectTermResult` (monthly breakdown + contribution + final exam)
- `SubjectAnnualResult` (term 1 + term 2)
- `StudentTermReportCard` (complete report card with subjects, totals, ranks)
- `StudentAnnualReportCard` (annual version)
- `AcademicPolicy` (thresholds, rounding, ranking, tie handling)

### Assessment Weights (School Default):

```
assignments_max: 20  (Decimal)
oral_max: 20         (Decimal)
attendance_max: 20   (Decimal)
written_test_max: 40 (Decimal)
total_max: 100       (Decimal)
```

### Required Fix: AssessmentScheme.max_score should use Decimal, not f64

---

## 10. Calculation Rules

### Current State: ✅ CORRECTLY IMPLEMENTED

`calculation_engine.rs` implements:
- `AcademicCalculationEngine::calculate_monthly_total()` — sums components, capped at max
- `AcademicCalculationEngine::calculate_subject_term_result()` — 3-month average → 20-point contribution + 30-point exam = 50
- `AcademicCalculationEngine::calculate_subject_annual_result()` — Term 1 (50) + Term 2 (50) = Annual (100)
- `AcademicCalculationEngine::compute_rankings()` — supports Standard Competition, Dense, Fractional
- `AcademicCalculationEngine::resolve_letter_grade()` — A/B/C/D/F

All using `rust_decimal::Decimal` with configurable rounding.

### Existing Tests: ✅ PASSING

- `test_monthly_assessment_calculation` — verifies 18+17+19+35 = 89.00
- `test_school_term_normalized_calculation_exact_prompt_example` — verifies complete flow (monthly_avg=86.67, contribution=17.33, term=41.33, pct=82.67%)
- `test_annual_aggregation` — verifies 41.33+45.00 = 86.33 = PassedWithHonors
- `test_rankings_standard_competition_and_dense` — verifies tie handling

---

## 11. Rounding Policy

### Current State: ✅ CORRECTLY IMPLEMENTED

```rust
pub enum RoundingMode {
    RoundHalfUp,    // MidpointAwayFromZero
    RoundUp,        // AwayFromZero
    RoundDown,      // ToZero
    Truncate,       // trunc_with_scale
}
```

Default: `RoundHalfUp` with 2 decimal places.

### Canonical Rounding Function:

```rust
pub fn apply_rounding(value: Decimal, mode: RoundingMode, dp: u32) -> Decimal
```

Used consistently throughout the calculation engine. Desktop must NOT perform independent rounding.

---

## 12. Ranking Policy

### Current State: ✅ PARTIALLY IMPLEMENTED

Domain model supports:
- `RankingScope` — Class, GradeLevel, OrgUnit, Cohort
- `TiePolicy` — StandardCompetition (1,2,2,4), Dense (1,2,2,3), Fractional (average)

### Missing: Configurable top-N display, ranking enable/disable per institution

---

## 13. Pass/Fail Policy

### Current State: ✅ ADEQUATE

```rust
pub struct AcademicPolicy {
    passing_threshold_percentage: Decimal,         // default: 50
    honors_threshold_percentage: Decimal,           // default: 85
    minimum_final_exam_score: Option<Decimal>,      // default: Some(10)
    max_failed_subjects_for_conditional_pass: usize, // default: 2
}
```

Supports: Passed, PassedWithHonors, ConditionalPass, Failed, Incomplete, Absent, Probation, Withdrawn.

### Missing: Subject-level minimum score, makeup/retest configuration

---

## 14. Institution-Type Defaults

### Current State: ✅ DIFFERENTIATED

| Feature | School | University | Institute |
|---------|--------|-----------|-----------|
| Grading System | percentage | gpa_4_0 | pass_fail |
| Assessment Formula | assignments 20 + oral 20 + attendance 20 + written 40 = 100 | coursework 40% + final 60% | practical 50% + theory 50% |
| Period Templates | 2 terms (3 months + exam each) | 2 semesters + summer | flexible |

### Missing: Institution-specific onboarding defaults for period templates in Desktop (currently hardcoded same templates for all types)

---

## 15. Branch/OrgUnit Initialization

### Current State: ⚠️ DEFERRED TO ONBOARDING

**Bootstrap** creates NO OrgUnit.  
**Onboarding** creates root campus via:
```sql
INSERT INTO org_units (...) SELECT ... WHERE NOT EXISTS (SELECT 1 FROM org_units WHERE org_id = $2 AND depth = 0)
```

### Required Fix:

1. Bootstrap should create a root OrgUnit (type: "organization", depth: 0)
2. Onboarding should create campus/building units UNDER the root
3. This ensures any operation post-bootstrap has a valid org_unit_id

---

## 16. Building/Floor/Room/Location Model

### Current State: ⚠️ PARTIAL

**Buildings** — Stored as `OrgUnit` entries with `unit_type: 'building'`. ✅ CORRECT

**Floors** — NOT modeled in database. Desktop has `floor: number` on rooms, but this is lost during onboarding submission. ❌ DATA LOSS

**Rooms** — Stored in `rooms` table with `org_unit_id` referencing parent building/campus. ✅ CORRECT

**Locations** — No explicit location model. Building OrgUnit has optional `room_number`, `floor`, `building` fields, but these are on the OrgUnit itself, not on Room.

### Database Schema (Room):

```sql
rooms (id, org_id, name, room_type, capacity, org_unit_id, created_at, updated_at)
```

### Required Fixes:

1. Add `floor` column to `rooms` table (or model floors as OrgUnit entries)
2. Add `building_name` or `building_code` to Room for convenience
3. Ensure `ValidatedOnboardingRoom` includes `floor` field
4. Fix Desktop submission to include floor data
5. Add `name_ar` column to `rooms` table

---

## 17. DocForge Architecture

### Current State: ✅ WELL DESIGNED

The DocForge engine is a standalone 18-node AST vector reporting engine with:
- Canvas-based rendering
- Expression evaluation
- Schema-aware field system
- Serialization/deserialization
- Multi-up printing support
- Validation

The reporting domain model in `report.rs` (992 lines) provides:
- `ReportDefinition` — metadata, parameters, permissions, data provider key
- `ReportTemplate` — document layout, lifecycle, versioning
- `ReportDataset` — format-agnostic data contract
- `ReportJob` — async execution tracking
- `OrgReportContext` — organization branding

The `report_adapter.rs` (54KB) provides data providers.  
The `report_defaults.rs` (31KB) seeds default report definitions.

### Architecture: Unified Pipeline

```
Data Provider → Typed Report Data → Report Definition → Template → DocForge → Preview/PDF/Print/Export
```

### NO duplicate engines exist for different domains. ✅ CORRECT

---

## 18. Template Architecture

### Current State: ✅ WELL DESIGNED

The template system uses:
- Typed `ReportDefinition` with `data_provider_key`
- `ReportTemplate` with versioning and lifecycle (Draft → Published → Archived)
- `ReportTemplateBinding` linking templates to definitions
- `ParameterDef` schema for UI generation
- `ReportDomain` grouping (Finance, Academic, Exam, Attendance, Staff, System)

### Supported Output Formats:

PDF, HTML, XLSX, CSV, JSON, PNG

### Missing: Grade Table as first-class data source (partially implemented via report data providers)

---

## 19. Report Data Providers

### Current State: ✅ IMPLEMENTED IN BACKEND

`report_adapter.rs` provides resolver-based data providers.  
Report definitions reference data providers by key.

### Required: Ensure GradeTable data provider exposes typed data compatible with DocForge

---

## 20. Local/Remote Parity

### Current State: ⚠️ NEEDS VERIFICATION

Both `local-client.ts` and `remote-client.ts` implement `completeOnboarding()`.

**Local** calls Tauri IPC commands → Rust backend directly.  
**Remote** calls REST API endpoints → Same Rust backend via HTTP.

Both should produce identical results. The contract (`CompleteOrganizationOnboardingRequest`) is shared.

### Required: Verify that the Tauri IPC onboarding command passes the same data structure as the REST API endpoint.

---

## 21. Database Changes

### Required Schema Changes:

| Table | Change | Risk |
|-------|--------|------|
| `rooms` | Add `floor INTEGER DEFAULT 0` | LOW — additive |
| `rooms` | Add `name_ar TEXT` | LOW — additive |
| `rooms` | Add `building_name TEXT` | LOW — additive, denormalized for convenience |
| `assessment_schemes` | Change `max_score REAL` → `max_score TEXT` (Decimal) | MODERATE — requires data migration |

### NO destructive changes required for the academic hierarchy. The existing schema already supports parent_id on academic_periods.

---

## 22. API/SDK Changes

### Required Changes:

1. **`CompleteOrganizationOnboardingRequest`** periods array must include `id` and `parentId` fields
2. **Room DTOs** must include `floor`, `name_ar`, `building_name`
3. **Assessment Scheme DTO** max_score should be string/Decimal, not f64
4. Re-export OpenAPI spec and regenerate SDK

---

## 23. Desktop Changes

### Required Changes:

1. **Fix `DEFAULT_PERIOD_TEMPLATES`** — Default school template must include children (3 months + exam per term)
2. **Fix `template-semesters-monthly`** — Must have 4 children per term (3 months + 1 exam), not 3
3. **Fix `CompleteOrganizationOnboardingRequest` type** — Add `id?: string` and `parentId?: string` to periods
4. **Fix `completeSetup()` in `state.svelte.ts`** — Ensure parent/child period relationships are correctly submitted
5. **Fix period name generation** — Remove concatenation, use standalone child names
6. **Fix room floor preservation** — Include floor in submission payload
7. **Add room `name_ar`** — Currently in local state but not submitted to backend
8. **Remove `Date.now()` synthetic IDs** — Use UUID.v4 or let backend assign
9. **Fix init() threshold check** — `gradeLevels.length < 10` is an arbitrary threshold

---

## 24. Tests

### Existing Tests (Passing):

- ✅ Monthly assessment calculation (component sum)
- ✅ School term normalized calculation (monthly avg → 20-point contribution)
- ✅ Annual aggregation (Term 1 + Term 2)
- ✅ Rankings (Standard Competition + Dense tie handling)

### Required Additional Tests:

| Test | Priority |
|------|----------|
| Academic Year creation with correct dates | HIGH |
| Term creation with parent_id = None | HIGH |
| Monthly period creation with parent_id = term_id | HIGH |
| Exam period creation with parent_id = term_id | HIGH |
| Period date containment validation | HIGH |
| Non-overlapping period validation | HIGH |
| Default school onboarding (correct hierarchy) | HIGH |
| Default university onboarding (2 semesters) | HIGH |
| Default institute onboarding (pass/fail) | HIGH |
| Idempotent onboarding | HIGH |
| Default branch creation in bootstrap | HIGH |
| Building creation as OrgUnit | MEDIUM |
| Room creation with floor | MEDIUM |
| Room persistence and retrieval | MEDIUM |
| Branch scope isolation | MEDIUM |
| Rounding boundary values (0.5, 0.45, 0.444...) | HIGH |
| Pass/fail threshold behavior | HIGH |
| Certificate data provider | MEDIUM |
| Grade table data provider | MEDIUM |
| Preview/print parity | MEDIUM |
| LocalClient vs RemoteClient onboarding | HIGH |

---

## 25. Performance Measurements

### Baseline Required:

- Onboarding transaction time (full school setup with ~30 rooms, ~10 classes, ~20 subjects)
- Academic period query by year_id (with hierarchy)
- Grade calculation for single student (all subjects, single term)
- Batch certificate generation (100 students)
- Report data provider response time (grade table for single class)

### Current: No benchmarks exist. Must be established before and after changes.

---

## 26. Migration Risks

| Risk | Severity | Mitigation |
|------|----------|------------|
| Existing onboarded organizations have flat periods (no hierarchy) | HIGH | Migration script to restructure existing periods based on naming patterns |
| Room table schema change (add columns) | LOW | Additive columns with defaults, no data loss |
| AssessmentScheme max_score type change | MODERATE | Data migration: convert f64 → Decimal string |
| Period template key mismatch (periods vs children) | LOW | Normalize at consumption time in Desktop |
| Existing classes linked to wrong period_id | MODERATE | Update to link to academic year or correct term |

---

## 27. Reconciliation Matrix

| Layer | Current | Intended | Action |
|-------|---------|----------|--------|
| **Domain: AcademicYear** | String status | Typed enum | RENAME (status → enum) |
| **Domain: AcademicPeriod** | parent_id support ✅ | parent_id hierarchy | KEEP |
| **Domain: AcademicPolicy** | Full grading model ✅ | Full grading model | KEEP |
| **Domain: MonthlyAssessmentBreakdown** | Decimal components ✅ | Decimal components | KEEP |
| **Domain: SubjectTermResult** | Correct formula ✅ | Correct formula | KEEP |
| **Domain: AssessmentScheme** | max_score: f64 | max_score: Decimal | REMAP |
| **Domain: Room** | No floor/name_ar | Add floor, name_ar | ADD |
| **Domain: PeriodTemplate** | structure JSON ✅ | structure JSON | KEEP |
| **Bootstrap: Period Templates** | children key=`periods` | Normalize to `children` | RENAME |
| **Bootstrap: OrgUnit** | Not created | Create root org unit | ADD |
| **Onboarding: Root Campus** | Created in onboarding | Move to bootstrap | MOVE |
| **Onboarding: Periods** | Flat submission possible | Require hierarchy | REBUILD |
| **Onboarding: Classes** | Linked to first_period_id | Link to year_id | REMAP |
| **Onboarding: TimeSlots** | Linked to first_period_id | Link to year_id | REMAP |
| **Onboarding: Rooms** | No floor field | Add floor | ADD |
| **Onboarding: Buildings** | As OrgUnit ✅ | As OrgUnit | KEEP |
| **Desktop: Period Templates** | 2-semester flat | 2-term with 4 children each | REBUILD |
| **Desktop: OnboardingRequest** | No id/parentId | Add id/parentId | ADD |
| **Desktop: Period Names** | Concatenated | Standalone | REBUILD |
| **Desktop: Room IDs** | Date.now() | UUID v4 | REMAP |
| **Desktop: Floor Data** | Collected but lost | Persisted | ADD |
| **API: OnboardingPeriodDto** | Has id/parent_id ✅ | Has id/parent_id | KEEP |
| **API: Room DTO** | No floor/name_ar | Add floor/name_ar | ADD |
| **CalcEngine** | Decimal, tested ✅ | Decimal, tested | KEEP |
| **Rankings** | Standard+Dense ✅ | Add configurable top-N | ADD |
| **DocForge** | Unified pipeline ✅ | Unified pipeline | KEEP |
| **Report Definitions** | Seeded in onboarding ✅ | Seeded in onboarding | KEEP |

---

## 28. Implementation Phases

### Phase 1: Fix Bootstrap → Add Default Root OrgUnit ✅
- Create root OrgUnit (type="organization") during bootstrap
- Make onboarding check for existing root unit before creating campus

### Phase 2: Fix Desktop Period Templates
- Rebuild `DEFAULT_PERIOD_TEMPLATES` with correct school hierarchy (2 terms × 4 children)
- Make `template-two-semesters` the default WITH children
- Fix `template-semesters-monthly` to have 3 months + 1 exam

### Phase 3: Fix Desktop Onboarding Contract
- Add `id?: string` and `parentId?: string` to period type in `types.ts`
- Fix `completeSetup()` to generate and pass parent/child IDs
- Fix period name generation (standalone, not concatenated)

### Phase 4: Fix Room/Building Persistence
- Add `floor`, `name_ar`, `building_name` to Room schema
- Add `floor` to `ValidatedOnboardingRoom` and persistence
- Fix Desktop submission to include floor data

### Phase 5: Fix Class/TimeSlot Period Linkage
- Classes should reference the academic year, not first period
- Time slots should reference the academic year

### Phase 6: Type Safety Improvements
- Enum-ify `AcademicYear.status`
- Convert `AssessmentScheme.max_score` to Decimal
- Remove `Date.now()` synthetic IDs from Desktop

### Phase 7: SDK Regeneration
- Export updated OpenAPI specification
- Regenerate `@orbis/sdk`
- Verify Local/Remote parity

### Phase 8: Tests
- Add all required tests from Section 24
- Verify onboarding produces correct hierarchy
- Verify calculation engine with real onboarded data

---

## ONBOARDING_RECONCILIATION_STATUS: COMPLETE ✅

**Summary of Reconciled Core Architecture:**

1. **Desktop Period Templates (`templates.ts`)**: Standard School System rebuilt with canonical 2 terms, each containing 3 monthly grading periods + 1 term final exam. Correct Arabic terminology and weights configured.
2. **Desktop Onboarding Contract (`types.ts` & `state.svelte.ts`)**: Added `id` and `parentId` support for multi-level hierarchical periods. Period names are standalone (not concatenated). Added `floor` and `nameAr` support for rooms.
3. **Bootstrap Root OrgUnit (`bootstrap_adapter.rs`)**: Root organizational unit (`Main Campus` / `المقر الرئيسي`) is seeded directly during bootstrap execution, guaranteeing non-null campus/unit scoping for all subsequent operations.
4. **Room & Facility Persistence (`orbis-app`, `onboarding_service.rs`, `onboarding.rs`)**: `ValidatedOnboardingRoom` and `OnboardingRoomItemDto` now accept `name_ar` and `floor` fields, eliminating data loss during onboarding.
5. **Template Key Alignment (`bootstrap_service.rs`)**: Unified structure to use `children` across backend blueprint definitions and frontend template models.
6. **Type Safety & Build Parity**: Clean verification across `apps/api-rust` (cargo check: 0 errors), `apps/desktop/src-tauri` (cargo check: 0 errors), `packages/sdk` (tsc: 0 errors), and `apps/desktop` (tsc: 0 errors).
