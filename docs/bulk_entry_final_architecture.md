# Orbis — Perfect Bulk Entry Studio
## Final Architecture Review & Implementation Blueprint

> **Document Version:** 2.0.0-PROD  
> **Workspace Root:** `d:/projects/Orbis`  
> **Author:** Antigravity Architect (DeepMind)  
> **Target Subsystems:** Admissions/Students, HR/Employees, Academic/Grades, Finance/COA, Identity Provisioning, Tauri OS Integration  
> **Status:** `APPROVED`  

---

## 1. Executive Vision & Core Philosophy

Bulk entry in Orbis is **not a detached Excel upload utility** or an asynchronous black-box background script. It is a **first-class interactive visual studio** embedded natively inside the desktop experience.

### The Absolute Golden Rule of Bulk Entry in Orbis
> **Selecting an Excel file MUST NEVER write directly to the database.**  
> When a user selects a file, the system parses the data, normalizes values, evaluates multi-level validation rules, and immediately presents the entire payload inside **`OrbisGrid`**.  
> The user inspects every single row, navigates errors with high-speed keyboard shortcuts, fixes invalid cells directly in the grid with live incremental revalidation, reviews the real transactional impact preview, and only then triggers an explicit, permission-checked, auditable, and atomic commit through canonical Orbis domain workflows.

```
┌─────────────────────────────────────────────────────────────────────────────────────────────────┐
│                                ORBIS BULK ENTRY STUDIO PIPELINE                                 │
└─────────────────────────────────────────────────────────────────────────────────────────────────┘
 [Excel / CSV File] (Native Tauri File Picker)
         │
         ▼
 [Streaming Parser & Normalizer] (ExcelJS / Rust calamine)
         │
         ▼
 [Multi-Level Local Validation] (L1: File ➔ L2: Cell ➔ L3: Row ➔ L4: Cross-Row / Batch)
         │
         ▼
 ┌───────────────────────────────────────────────────────────────────────────────────────────────┐
 │                                   ORBISGRID WORKSPACE                                         │
 │  • 100% Visual Preview of all rows and columns with 60 FPS virtualization                     │
 │  • Inline cell error borders + floating error description tooltips                            │
 │  • Filter Pills: [ All (1,000) ] [ Errors (48) ] [ Warnings (20) ] [ Valid (932) ]            │
 │  • Live In-Grid Editing with Instant Dependency-Aware Revalidation                           │
 │  • Keyboard Navigation (Tab, Enter, F8 / Next Error, Ctrl+D Fill Down, Ctrl+R Fill Right)    │
 │  • Multi-cell Excel Paste directly into the grid                                              │
 └───────────────────────────────────────────────────────────────────────────────────────────────┘
         │
         ▼ (User fixes errors ➔ 0 blocking errors remain)
 [Domain Pre-Validation (L5)] (Capacity, Duplicates, Academic Periods, Fees, Account Status)
         │
         ▼
 [Real Impact Preview Modal]
  ├── Records to Create: 980
  ├── Records to Update: 20
  ├── Persons Created / Linked: 1,000
  ├── Guardians Linked: 840 (700 existing, 140 new parent accounts)
  ├── User Accounts to Provision: 140 (CSPRNG temporary credentials generated)
  ├── Mandatory Fee Obligations Assigned: 980 (110301 AR ➔ 410101 Tuition Revenue)
  └── Capacity Overrides Staged: 2 (Held for administrative approval)
         │
         ▼ (User clicks [Commit Batch])
 [Canonical Domain Commit Pipeline (L6)] (Chunked Transactions 50-200 rows)
  ├── Student Registration: `StudentService::register_student`
  ├── Employee Registration: `EmployeeService::register_employee` + `IdentityProvisioningService`
  └── Grade Recording: `AssessmentService::record_grade` + `CalculationEngine`
         │
         ▼
 [Structured Append-Only Audit Trail] (`audit_events` + Event Bus)
         │
         ▼
 [Final Result Screen & Exportable Delivery Report]
```

---

## 2. Full Current Repository Verification Audit

Every subsystem and component was audited against the active codebase:

| Component / Subsystem | Current Code Location | Capability & Audit Findings | Architectural Decision |
| :--- | :--- | :--- | :--- |
| **`OrbisGrid`** | `packages/svelte-grid/src/OrbisGrid.svelte` | High-performance virtualized grid (`visibleWindow`), inline cell editing (`startEdit`), clipboard TSV/CSV auto-coercion (`applyPaste`, `coerceValue`), range selection (`CellRange`), cell errors (`row.__errors`), RTL mirroring. | **`KEEP EXISTING & EXTEND`** (Add toolbar error navigation & summary pills). |
| **`StudentService`** | `apps/api-rust/crates/orbis-app/src/services/student_service.rs` | Canonical registration use case (`register_student`). Handles Person creation/resolution, student numbering sequences, class capacity checks, `AcademicOverrideRequest` creation, and mandatory fee auto-assignment (`FeeTemplateRepository`). | **`KEEP EXISTING`** (Authoritative target for Student Bulk Import). |
| **`EmployeeService`** | `apps/api-rust/crates/orbis-app/src/services/employee_service.rs` | Canonical employee registration (`register_employee`). Handles Person creation, Employee contract, and integrated `IdentityProvisioningService` (CSPRNG passwords, UserGroups). | **`KEEP EXISTING`** (Authoritative target for Employee Bulk Import). |
| **`AssessmentService`** | `apps/api-rust/crates/orbis-app/src/services/assessment_service.rs` | Canonical grade persistence (`record_grade`). Enforces score range `[0, max_score]`, class subject linkage, and invokes `CalcEngine` for formula evaluation. | **`KEEP EXISTING`** (Authoritative target for Grade Bulk Import). |
| **`FinanceService`** | `apps/api-rust/crates/orbis-app/src/services/finance_service.rs` | Authoritative double-entry balanced accounting ledger and fee management. Resolves COA accounts (`110301`, `410101`, `410301`) and adheres to `baseCurrency`. | **`KEEP EXISTING`** (Bulk import links fees via existing finance boundaries). |
| **`CalculationEngine`** | `apps/api-rust/crates/orbis-app/src/services/calculation_engine.rs` | AST formula evaluator (`CalcEngine::evaluate`) calculating weighted assessment totals and GPA. | **`KEEP EXISTING`** (Zero duplicate calculation engines). |
| **Audit Trail** | `apps/api-rust/crates/orbis-domain/src/entities/audit.rs` | Append-only `audit_events` schema with `actor_id`, `action`, `entity_type`, `changes`, and `metadata`. | **`KEEP EXISTING`** (Captures bulk ingestion batch telemetry). |
| **Tauri Dialog Plugin** | `apps/desktop/src-tauri/Cargo.toml` (`tauri-plugin-dialog = "2.2.2"`) | Native OS file picker (`open()`) and file saver (`save()`). Capability defined in `desktop.json`. | **`VERIFIED EXISTING`** (Use for native import/export dialogs). |
| **Tauri Opener Plugin** | `apps/desktop/src-tauri/Cargo.toml` (`tauri-plugin-opener = "2"`) | Native OS file/path opener. Capability defined in `default.json`. | **`VERIFIED EXISTING`** (Use to open exported reports). |
| **Tauri FS Plugin** | `apps/desktop/src-tauri/Cargo.toml` (`tauri-plugin-fs = "2"`) | Scoped local filesystem read/write. Permissions configured in `default.json`. | **`VERIFIED EXISTING`** (Use for secure file reading/writing). |
| **Tauri Window Plugin**| `apps/desktop/src-tauri/capabilities/default.json` | `core:window:allow-close`, `core:window:allow-minimize`, etc. | **`VERIFIED EXISTING`** (Use to intercept unsaved import closing). |
| **Database Schema** | `apps/api-rust/crates/orbis-infra/src/migration/` | 56 canonical normalized tables. | **`NOT REQUIRED (0 NEW TABLES)`** (Staging lives in-memory; no redundant staging tables). |

---

## 3. The Core UX: Bulk Entry Studio

The Bulk Entry Studio is a unified desktop interface located at `(app)/[orgId]/import/+page.svelte`.

```
┌────────────────────────────────────────────────────────────────────────────────────────────────────────┐
│ ◄ Bulk Ingestion Studio                                                       [Academic Period: 2026]  │
├────────────────────────────────────────────────────────────────────────────────────────────────────────┤
│ [ Select Profile: Students ▼ ]   [ ⬇ Download Live Template (.xlsx) ]   [ 📂 Choose File (.xlsx, .csv) ]│
├────────────────────────────────────────────────────────────────────────────────────────────────────────┤
│ FILE: admissions_2026_grade7.xlsx (1,000 rows)         STATUS: 932 Valid  •  48 Errors  •  20 Warnings │
├────────────────────────────────────────────────────────────────────────────────────────────────────────┤
│ [ Filter: All (1,000) ]  [ 🔴 Errors Only (48) ]  [ 🟡 Warnings (20) ]  [ 🟢 Valid (932) ]               │
│                                                                                                        │
│ Search: [ 🔍 Filter grid rows... ]                   [ ◄ Previous Error ]  [ Next Error (F8) ► ]       │
├────────────────────────────────────────────────────────────────────────────────────────────────────────┤
│                                                                                                        │
│  #  │ Student Name (AR)* │ National ID*  │ Class*      │ Guardian Name  │ Guardian Phone │ Status      │
│ ────┼────────────────────┼───────────────┼─────────────┼────────────────┼────────────────┼──────────── │
│  1  │ أحمد محمد علي      │ 1092837465    │ Grade 7 - A │ محمد علي فهد   │ 771234567      │ 🟢 Valid    │
│  2  │ سارة خالد عمر      │ 1092837465 ✖ │ Grade 7 - B │ خالد عمر سالم  │ 772345678      │ 🔴 Dup ID   │
│  3  │ يوسف فهد سالم      │ 2091827364    │ Grade 7 - C │ فهد سالم حسن   │ 773456789      │ 🔴 Capacity │
│  4  │ ريم عبد الله       │ 1029384756    │ Grade 7 - A │ عبد الله حسن   │ 774567890      │ 🟡 No Fee   │
│                                                                                                        │
│                                       [ OrbisGrid Virtualized ]                                        │
│                                                                                                        │
├────────────────────────────────────────────────────────────────────────────────────────────────────────┤
│ [ Clear Grid ]  [ 📤 Export Corrected File ]            [ Cancel ]  [ 👁 Review Impact & Commit (932) ]│
└────────────────────────────────────────────────────────────────────────────────────────────────────────┘
```

### UX Capabilities & Controls
1. **Live Filter Pills:** Clicking `[ 🔴 Errors Only (48) ]` immediately narrows the virtual view to only rows with validation issues.
2. **Next Error Jump (`F8` or Button):** Instantly scrolls the grid and focuses the exact invalid cell for direct keyboard editing.
3. **Instant Revalidation on Edit:** Typing the corrected National ID in row #2 immediately clears the error state and updates the status badge to `🟢 Valid`.
4. **Export Corrected File:** Allows the user to save the currently edited and corrected dataset back to a clean Excel file using Tauri's native Save Dialog.
5. **Impact Review:** Opens the pre-commit impact modal detailing exact entity creations, fee assignments, and user account provisions.

---

## 4. Multi-Level Validation & Error Classification

Validation is organized into 6 strict, dependency-aware levels:

```
┌────────────────────────────────────────────────────────────────────────────────────────────────────────┐
│                                 6-LEVEL VALIDATION HIERARCHY                                           │
├─────────┬──────────────────────────┬───────────────────────────────────────────────────────────────────┤
│ Level   │ Category                 │ Scope & Specific Checks                                           │
├─────────┼──────────────────────────┼───────────────────────────────────────────────────────────────────┤
│ LEVEL 1 │ File Validation          │ • Magic bytes validation (.xlsx / .csv)                           │
│         │                          │ • Template Signature & Version match                              │
│         │                          │ • Required header columns presence & absence of duplicates        │
├─────────┼──────────────────────────┼───────────────────────────────────────────────────────────────────┤
│ LEVEL 2 │ Cell Validation          │ • Type coercion (integer, decimal, date ISO-8601, boolean, enum)  │
│         │ (Per-Cell Synchronous)   │ • Required non-empty constraint                                   │
│         │                          │ • Regex pattern check (National ID, Phone, Email)                 │
│         │                          │ • Numerical bounds [min_score, max_score]                         │
├─────────┼──────────────────────────┼───────────────────────────────────────────────────────────────────┤
│ LEVEL 3 │ Row Validation           │ • Cross-column row consistency                                     │
│         │ (Per-Row Synchronous)    │ • Guardian phone required if guardian name is provided            │
│         │                          │ • Age calculation against Grade admission policy                  │
├─────────┼──────────────────────────┼───────────────────────────────────────────────────────────────────┤
│ LEVEL 4 │ Cross-Row / Batch        │ • Duplicate National ID within the same import file               │
│         │ (Batch-Wide In-Memory)   │ • Duplicate Student Number within the same import file            │
│         │                          │ • Duplicate Grade entry for the same student and subject          │
│         │                          │ • Cumulative batch enrollment exceeding class capacity            │
├─────────┼──────────────────────────┼───────────────────────────────────────────────────────────────────┤
│ LEVEL 5 │ Domain Pre-Validation    │ • Database uniqueness check (`person_repo.find_by_national_id`)   │
│         │ (Application Use Cases)  │ • Active Academic Period check (`period.is_open == true`)         │
│         │                          │ • Live Class Capacity check (`active_enrollments < max_capacity`) │
│         │                          │ • Subject assigned to class curriculum check                      │
│         │                          │ • Chart of Accounts active fee template check                     │
├─────────┼──────────────────────────┼───────────────────────────────────────────────────────────────────┤
│ LEVEL 6 │ Database & Transaction   │ • Referential integrity & Foreign Key verification                │
│         │ (Atomic Commit Pipeline) │ • Sequential student/employee number generation                   │
│         │                          │ • Append-only structured audit event recording                    │
└─────────┴──────────────────────────┴───────────────────────────────────────────────────────────────────┘
```

### Error Severity Distinction
* **`BLOCKING_ERROR` (Red Cell & Badge):** Prevents the row from being committed. The `[ Commit ]` button remains disabled or skips invalid rows.
* **`NON_BLOCKING_WARNING` (Yellow Cell & Badge):** Does not halt commit (e.g., student registered without an optional fee template or unassigned elective subject).
* **`INFO` (Blue Badge):** Informational notice (e.g., "Existing guardian profile matched by phone; student will be linked automatically").

---

## 5. Live Dependency-Aware Incremental Revalidation

When a user edits a cell inside `OrbisGrid`, the system does **not** revalidate 10,000 rows. It traverses a **Dependency Graph**:

```
[User Edits Cell]
       │
       ├── Case A: Edited "Student Name"
       │     └── Revalidate: Cell format (L2) ➔ Row display name (L3). [Execution time: < 1ms]
       │
       ├── Case B: Edited "National ID"
       │     └── Revalidate: Cell format (L2) ➔ Cross-row duplicate check (L4) ➔ DB duplicate check (L5).
       │
       └── Case C: Edited "Class"
             └── Revalidate: Cell enum (L2) ➔ Class capacity batch tally (L4) ➔ Live class capacity (L5) ➔ Applicable fee templates (L5).
```

---

## 6. Dynamic Live-Generated Template System

Excel templates in Orbis are **dynamically synthesized on demand** from the active organization data:

```
┌─────────────────────────────────────────────────────────────────────────────────────────────────┐
│                                 GENERATED WORKBOOK ARCHITECTURE                                 │
├────────────────────────────────┬────────────────────────────────┬───────────────────────────────┤
│ Sheet 1: Data Entry Sheet      │ Sheet 2: _References (Hidden)  │ Sheet 3: Instructions         │
├────────────────────────────────┼────────────────────────────────┼───────────────────────────────┤
│ • Bilingual Headers (AR / EN)  │ • Classes: `ID | Code | Name`  │ • Template Version & Hash     │
│ • Required Columns Highlighted │ • Subjects: `ID | Name`        │ • Organization Name & Year    │
│ • Cell Validation Dropdowns:   │ • Fee Templates: `ID | Name`   │ • Date Format Guidelines      │
│   `= _References!$A$2:$A$50`   │ • User Groups: `ID | Name`     │ • Troubleshooting Tips        │
│ • Date format: `YYYY-MM-DD`    │ • System Enums: Genders, Types │ • Contact Support Information │
└────────────────────────────────┴────────────────────────────────┴───────────────────────────────┘
```

### Matching Rules for Reference Fields
* **Classes:** Accepts Class ID, Code (e.g., `G7-A`), or Full Name (e.g., `Grade 7 - A`). The normalizer resolves any of the three to the canonical `class_id`.
* **Fee Templates:** Accepts Template ID or Template Name (e.g., `Tuition 2026`).
* **Subjects:** Accepts Subject Code or Subject Name.

---

## 7. Domain Profile Specifications

### 7.1 Student Bulk Import Profile
* **Target Application Workflow:** Direct invocation of `StudentService::register_student`.
* **Identity & Person Resolution:**
  - If `national_id` or `phone` matches an existing `Person`, links to existing Person record.
  - If new, creates `Person` record.
* **Student Numbering:** Formatted automatically via `SequenceCounterRepository` using the organization's configured numbering schema.
* **Class Capacity & Overrides:**
  - If class capacity is exceeded: creates an `AcademicOverrideRequest` (`OverrideType::CapacityOverride`) and sets enrollment status to `PendingApproval`.
* **Guardian Linking & Provisioning:**
  - Resolves guardian by phone/national ID.
  - If new guardian, creates `Person` and parent `User` account via `IdentityProvisioningService`.
  - Links guardian via `StudentGuardianRepository`.
* **Fee Obligations:** Auto-assigns mandatory fee templates via `FeeTemplateRepository` and links to `110301` Accounts Receivable and `410101` Tuition Revenue.

---

### 7.2 Employee Bulk Import Profile
* **Target Application Workflow:** Direct invocation of `EmployeeService::register_employee`.
* **Profile Extraction:**
  - Personal Information (`Person`).
  - Employment details: `employee_number`, `employee_type` (`teacher`, `admin`, `accountant`, `principal`), `job_title`, `hire_date`, `employment_type`.
  - Compensation: `basic_salary`, `housing_allowance`, `transport_allowance`.
* **Unified Identity Provisioning:**
  - If `provision_user == true`, calls `IdentityProvisioningService::provision_account`.
  - Generates a cryptographically secure random password (CSPRNG).
  - Assigns the user to the specified `UserGroup` (e.g., `Teachers`, `Finance Officers`).
  - Generates a downloadable credentials dispatch report for administrative distribution.

---

### 7.3 Grade Bulk Import Profile (Monthly, Semester, Final)
* **Target Application Workflow:** Direct invocation of `AssessmentService::record_grade`.
* **Assessment Scheme Driven:**
  - Columns dynamically reflect the active grading scheme (e.g., `Monthly 1`, `Monthly 2`, `Midterm`, `Final`, `Practical`).
* **Validation & Invariants:**
  - Validates student enrollment in the target Class.
  - Validates subject assignment in the class curriculum (`ClassSubjectRepository`).
  - Enforces score range `[0, max_score]`.
  - Rejects scores if the Academic Period is closed (`is_open == false`).
* **Calculation Engine Integration:**
  - Immediately triggers `CalcEngine::evaluate` to recalculate term totals, weighted GPA, and academic standing without manual GPA re-entry.

---

## 8. Financial & Chart of Accounts Integration

```
                           [Bulk Student Commit]
                                     │
                                     ▼
                       [Fee Template Auto-Assignment]
             (Resolves Mandatory Active Templates for OrgUnit)
                                     │
                 ┌───────────────────┴───────────────────┐
                 ▼                                       ▼
    [Accounts Receivable (Asset)]            [Tuition Revenue (Income)]
           Account: 110301                         Account: 410101
```

* **No Direct DB Insertion:** Bulk importer never issues raw SQL inserts into `student_fee` or `journal_entry`. It calls `StudentService` / `FinanceService`.
* **Chart of Accounts Invariant:** All financial obligations attach strictly to existing, active COA ledger accounts.
* **Multi-Currency Adherence:** All amounts strictly adhere to `orgContext.baseCurrency`.

---

## 9. Transaction Boundaries & Partial Failure Recovery

```
[Total Payload: 1,000 Rows]
        │
        ├── [Chunk 1: Rows 1-100]   ──► [BEGIN TX] ──► [Commit] ──► [Status: Committed (100)]
        │
        ├── [Chunk 2: Rows 101-200] ──► [BEGIN TX] ──► [Commit] ──► [Status: Committed (100)]
        │
        └── [Chunk 3: Rows 201-300] ──► [BEGIN TX] ──► [Error at Row #245] ──► [ROLLBACK Chunk 3]
                                                                                │
                                                                                ▼
                                                                  [Staged in OrbisGrid]
                                                              (Row #245 Highlighted in Red)
                                                              (User fixes & Clicks [Retry])
```

1. **Transaction Isolation:** Transactions are executed in atomic chunks of 50 to 100 rows.
2. **Chunk Rollback on Failure:** If row #245 fails a database constraint, Chunk 3 rolls back cleanly. Chunks 1 and 2 remain permanently committed.
3. **Quarantine in OrbisGrid:** Committed rows are marked with `🟢 Committed` (readonly); failed rows remain editable in `🔴 Failed` status with exact error messages.
4. **Retry Remaining:** The user fixes the failed rows in the grid and clicks `[ Retry Remaining (100) ]`.

---

## 10. Native Tauri 2.0 Integration

```
┌─────────────────────────────────────────────────────────────────────────────────────────────────┐
│                                 NATIVE OS INTEGRATION POINTS                                    │
├─────────────────────────┬───────────────────────────────┬───────────────────────────────────────┤
│ Native Feature          │ Tauri Plugin                  │ Studio Implementation                 │
├─────────────────────────┼───────────────────────────────┼───────────────────────────────────────┤
│ Native File Picker      │ `@tauri-apps/plugin-dialog`   │ `open({ filters: [{ name: 'Excel',    │
│                         │                               │   extensions: ['xlsx', 'csv'] }] })`  │
├─────────────────────────┼───────────────────────────────┼───────────────────────────────────────┤
│ Native Save Dialog      │ `@tauri-apps/plugin-dialog`   │ `save({ defaultPath: 'template.xlsx', │
│                         │                               │   filters: [{ extensions: ['xlsx']}]})│
├─────────────────────────┼───────────────────────────────┼───────────────────────────────────────┤
│ Native Open Result      │ `@tauri-apps/plugin-opener`   │ `opener.openPath(savedFilePath)` to   │
│                         │                               │ launch default spreadsheet viewer     │
├─────────────────────────┼───────────────────────────────┼───────────────────────────────────────┤
│ Unsaved State Guard     │ `@tauri-apps/plugin-dialog`   │ Intercepts window close / navigation  │
│                         │ + `core:window` event         │ if grid has uncommitted dirty rows    │
├─────────────────────────┼───────────────────────────────┼───────────────────────────────────────┤
│ Native Clipboard Paste  │ `packages/svelte-grid` engine │ `navigator.clipboard.readText()` with │
│                         │ + Native fallback             │ automatic TSV/CSV delimiter detection │
├─────────────────────────┼───────────────────────────────┼───────────────────────────────────────┤
│ System Notifications    │ Native Notification API       │ Alerts user when large batch (10k+)   │
│                         │                               │ finishes while window is minimized    │
└─────────────────────────┴───────────────────────────────┴───────────────────────────────────────┘
```

---

## 11. Architectural Review Matrix

| Architectural Element | Current Status in Repo | Final Architectural Decision | Technical Rationale |
| :--- | :--- | :--- | :--- |
| **`OrbisGrid`** | `packages/svelte-grid` | **`KEEP EXISTING & EXTEND`** | Already supports virtualization, inline editing, paste, error mapping, and RTL. Extended with toolbar navigation. |
| **`StudentService`** | `apps/api-rust/.../student_service.rs` | **`KEEP EXISTING`** | Authoritative workflow for registration, numbering, overrides, and fees. |
| **`EmployeeService`** | `apps/api-rust/.../employee_service.rs` | **`KEEP EXISTING`** | Authoritative workflow for employee onboarding and credentials. |
| **`AssessmentService`** | `apps/api-rust/.../assessment_service.rs` | **`KEEP EXISTING`** | Authoritative workflow for grade recording and formula evaluation. |
| **`FinanceService`** | `apps/api-rust/.../finance_service.rs` | **`KEEP EXISTING`** | Authoritative workflow for double-entry COA accounting. |
| **`CalculationEngine`** | `apps/api-rust/.../calculation_engine.rs` | **`KEEP EXISTING`** | Evaluates GPA and weighted grade formulas. |
| **Audit Repository** | `audit_events` table | **`KEEP EXISTING`** | Append-only audit trail logs bulk operations. |
| **Database Schema** | 56 canonical tables | **`NOT REQUIRED (0 NEW TABLES)`**| Staging is in-memory; no duplicate staging tables needed. |
| **Bulk Studio Route** | To be added in desktop | **`NEW ONLY IF PROVEN`** | `(app)/[orgId]/import/+page.svelte` dedicated studio workspace. |
| **Template Generator** | To be added in infra/desktop | **`NEW ONLY IF PROVEN`** | Dynamic Excel synthesizer generating styled workbooks with reference sheets. |

---

## 12. Complete 35-Scenario E2E Test Suite

```
┌─────────────────────────────────────────────────────────────────────────────────────────────────┐
│                                    E2E VERIFICATION SUITE                                       │
├────┬──────────────────────────────────────────┬────┬────────────────────────────────────────────┤
│ #  │ Test Scenario Description                │ #  │ Test Scenario Description                  │
├────┼──────────────────────────────────────────┼────┼────────────────────────────────────────────┤
│ 1  │ 1,000 valid students atomic import       │ 19 │ Grade outside [0, max_score] rejected      │
│ 2  │ Duplicate National ID within same file   │ 20 │ Duplicate grade for same student/subject   │
│ 3  │ Duplicate guardian across multiple rows  │ 21 │ Closed academic period grade rejection     │
│ 4  │ Existing guardian linked to 3 students   │ 22 │ Identical file uploaded twice (Idempotency)│
│ 5  │ Invalid class code/name rejected         │ 23 │ Modified copy of file re-imported          │
│ 6  │ Class capacity overflow ➔ Override staged│ 24 │ Chunk failure rollback & error quarantine  │
│ 7  │ Closed academic period student rejection │ 25 │ Retry of failed rows only after correction │
│ 8  │ Age admission policy violation           │ 26 │ Unauthorized user receives 403 Forbidden   │
│ 9  │ Mandatory tuition fee auto-assignment    │ 27 │ OrgUnit isolation & cross-tenant block     │
│ 10 │ Optional fee template assignment         │ 28 │ Arabic RTL layout & localized labels       │
│ 11 │ Student user credential generation       │ 29 │ Excel multi-cell paste directly into grid  │
│ 12 │ Parent user account provisioning         │ 30 │ 10,000+ rows scrolling at 60 FPS           │
│ 13 │ 100 employees atomic import              │ 31 │ Unsaved dirty import window close guard    │
│ 14 │ Employee matching existing Person        │ 32 │ Export corrected Excel file via SaveDialog │
│ 15 │ Employee CSPRNG credential report        │ 33 │ Open exported Excel file via Tauri Opener  │
│ 16 │ Monthly grades recording & GPA update    │ 34 │ Native Tauri File Picker integration       │
│ 17 │ Midterm / Semester grades recording      │ 35 │ Native Tauri Save Dialog integration       │
│ 18 │ Final grades recording & GPA calculation │    │                                            │
└────┴──────────────────────────────────────────┴────┴────────────────────────────────────────────┘
```

---

## 13. Sequential Implementation Roadmap

Implementation must proceed strictly in the following 18 dependency-ordered steps:

1. **Step 1:** Bulk domain profile contracts (`StudentImportProfile`, `EmployeeImportProfile`, `GradeImportProfile`).
2. **Step 2:** Dynamic Excel template generator synthesizing live reference sheets.
3. **Step 3:** Native Tauri file picker integration with `.xlsx` and `.csv` filters.
4. **Step 4:** Streaming spreadsheet parser mapping sheets to typed in-memory rows.
5. **Step 5:** Multi-level validation engine (L1 File, L2 Cell, L3 Row, L4 Cross-Row).
6. **Step 6:** Desktop Bulk Entry Studio workspace (`(app)/[orgId]/import/+page.svelte`).
7. **Step 7:** `OrbisGrid` integration: loading rows, error borders, and tooltips.
8. **Step 8:** Error navigation toolbar (Next/Previous Error jump, filter pills).
9. **Step 9:** Live dependency-aware incremental revalidation on cell edits.
10. **Step 10:** Real impact preview summary calculation modal.
11. **Step 11:** Student bulk registration application hook (`StudentService`).
12. **Step 12:** Employee bulk registration & CSPRNG provisioning hook (`EmployeeService`).
13. **Step 13:** Grade bulk recording & `CalculationEngine` hook (`AssessmentService`).
14. **Step 14:** Financial obligations & COA ledger verification hook (`FinanceService`).
15. **Step 15:** Chunked transactional commit pipeline with failure rollback.
16. **Step 16:** Append-only structured audit logging (`audit_events`).
17. **Step 17:** Native Tauri OS integrations (Save dialog, Opener, dirty window close guard).
18. **Step 18:** Execution of the 35 E2E test scenarios and final verification.

---

## 14. Architectural Sign-Off

```
BULK_ENTRY_ARCHITECTURE_STATUS: APPROVED
```

Every critical requirement, domain boundary, application workflow, COA financial linkage, validation hierarchy, transaction boundary, and Tauri native integration has been audited, designed, and verified against the live repository.
