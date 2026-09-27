# Orbis — Unified Bulk Entry & Excel Import Architecture Plan

> **Document Version:** 1.0.0-PROD  
> **Workspace Root:** `d:/projects/Orbis`  
> **Status:** `APPROVED`  
> **Target Subsystems:** Academic, Admissions/Students, HR/Employees, Grading/Assessments, Finance, Identity Provisioning  

---

## 1. Executive Summary & Repository Audit

The Bulk Entry / Excel Import System in Orbis is **not a standalone database insertion bypass**. It is a **first-class transactional ingestion, validation, and workflow orchestration layer** that passes all incoming records through canonical Orbis domain models, application services, invariants, Autopilot policies, financial obligations, identity provisioning, and append-only audit logging.

```
┌─────────────────────────────────────────────────────────────────────────────────────────────────┐
│                                 ORBIS BULK INGESTION PIPELINE                                   │
└─────────────────────────────────────────────────────────────────────────────────────────────────┘
 [Excel / CSV File] ──► [Parse & Normalize] ──► [Multi-Level Validation (L1-L4)]
                                                         │
                                                         ▼
                                                [OrbisGrid In-Memory]
                                      (Inline Edits, Error Badges, Next Error Nav)
                                                         │
                                                         ▼
                                             [Domain Pre-Validation (L5)]
                                      (Capacity, Duplicates, Academic Periods, Fees)
                                                         │
                                                         ▼
                                                [Interactive Preview]
                                      (Records to Create/Update, Fees, Users, Overrides)
                                                         │
                                                         ▼
                                       [Transactional Commit Pipeline (L6)]
                                                         │
             ┌───────────────────────────────────────────┴───────────────────────────────────────────┐
             ▼                                           ▼                                           ▼
   [Student Application Service]               [Employee Service]                         [Assessment Service]
  • Person creation/linking                   • Person creation                          • Subject / Class link
  • Student profile & numbering               • Employee contract & profile              • Range & Weighting check
  • Class capacity & Overrides                • Identity Provisioning (CSPRNG)           • Grading scheme evaluation
  • Auto-assigned fee templates               • Group membership assignments             • CalcEngine GPA calculation
  • Guardian accounts & linkage               • Payroll baseline setup                   • Academic standing updates
             │                                           │                                           │
             └───────────────────────────────────────────┬───────────────────────────────────────────┘
                                                         │
                                                         ▼
                                            [Append-Only Audit Trail]
                                     (`audit_events` + Domain Event Bus)
```

---

## 2. Existing OrbisGrid Capability Audit

The OrbisGrid package (`packages/svelte-grid`) was audited thoroughly. It provides production-grade capabilities that fully satisfy bulk entry requirements without creating parallel grid implementations:

| OrbisGrid Capability | Implementation in `packages/svelte-grid` | Status | Bulk Import Role |
| :--- | :--- | :--- | :--- |
| **Virtual Scrolling** | `visibleWindow` in `src/engine/virtual.ts` (O(1) DOM nodes) | `VERIFIED EXISTING` | Renders 10,000+ rows smoothly with 60 FPS |
| **Inline Editing** | `startEdit`, `commitEdit`, `editorEl` in `OrbisGrid.svelte` | `VERIFIED EXISTING` | Allows direct correction of invalid cells |
| **Clipboard / Paste** | `parseClipboard`, `applyPaste` in `src/engine/clipboard.ts` | `VERIFIED EXISTING` | Native multi-cell paste from Excel / Sheets |
| **Coercion & Normalization** | `coerceValue`, `normalizeDate` in `src/engine/coerce.ts` | `VERIFIED EXISTING` | Converts dates, numbers, enums, booleans |
| **Cell Error Highlighting** | `cellClass` checks `row.__errors.has(colId)` → `.is-error` | `VERIFIED EXISTING` | Displays red borders and tooltip error messages |
| **Range Selection** | `CellRange`, `SelectionSnapshot`, `handlePointer` | `VERIFIED EXISTING` | Allows multi-cell operations and inspection |
| **Fill Down / Fill Right** | `fillDown` (Ctrl+D), `fillRight` (Ctrl+R) in `clipboard.ts` | `VERIFIED EXISTING` | Rapid mass data correction across rows |
| **Keyboard Navigation** | `interpretKey` in `src/engine/keyboard.ts` (Tab, Enter, Esc) | `VERIFIED EXISTING` | Fast data entry without mouse reliance |
| **RTL Support** | Native `rtl` prop with coordinate mirroring | `VERIFIED EXISTING` | Complete Arabic layout fidelity |
| **Context Menu** | `contextMenuItems` in `src/engine/context-menu.ts` | `VERIFIED EXISTING` | Copy, Paste, Fill, Duplicate, Delete |
| **Client Querying** | `queryClient` (filtering, search, multi-column sort) | `VERIFIED EXISTING` | Instant filtering to invalid rows (`hasErrors: true`) |
| **Action Columns** | `ActionItem[]` with variant-based buttons | `VERIFIED EXISTING` | Row-level delete, fix, and inspector actions |
| **Error Navigation Bar** | Next/Previous Error quick jump | `REQUIRES EXTENSION` | Add lightweight toolbar helper to jump to next invalid cell |

---

## 3. Existing Excel / Import / Export Audit

| Module | Location | Findings & Decision |
| :--- | :--- | :--- |
| **Table Exporter** | `packages/ui/src/components/ui/table/create-table-export.ts` | Supports CSV, TSV, JSON; dynamic XLSX support. |
| **Rust CSV Engine** | `apps/api-rust/Cargo.toml` (`csv = "1.3"`) | High-performance streaming parser already in workspace dependencies. |
| **Spreadsheet Engine** | Rust `calamine` + Frontend `exceljs` / `xlsx` | Calamine for server-side sheet inspection; frontend parser for instant desktop validation. |
| **Duplicate Database Tables** | Proposed `import_sessions`, `staging_rows`, etc. | **`NOT REQUIRED`**. Ephemeral staging lives in frontend memory + OrbisGrid state. Execution passes directly through domain use cases in atomic transactions. |

---

## 4. One Bulk Engine, Multiple Domain Profiles

To maintain strict typing and avoid untyped generic blobs (`Record<string, any>`), Orbis uses a **Typed Bulk Engine Architecture** composed of a core workflow orchestrator and domain-specific profiles:

```
                  ┌──────────────────────────────────────────────┐
                  │          Unified Bulk Import Engine          │
                  │  (File Parsing, Grid State, Batch Progress)  │
                  └──────────────────────┬───────────────────────┘
                                         │
        ┌────────────────────────────────┼────────────────────────────────┐
        ▼                                ▼                                ▼
┌──────────────────────┐  ┌──────────────────────┐  ┌──────────────────────┐
│ StudentImportProfile │  │ EmployeeImportProfile│  │   GradeImportProfile │
│ • Schema & Columns   │  │ • Schema & Columns   │  │ • Schema & Columns   │
│ • L1-L5 Validators   │  │ • L1-L5 Validators   │  │ • L1-L5 Validators   │
│ • Ref Sheets Builder │  │ • Ref Sheets Builder │  │ • Ref Sheets Builder │
│ • Application Target │  │ • Application Target │  │ • Application Target │
└──────────────────────┘  └──────────────────────┘  └──────────────────────┘
```

### Core Bulk Profile Contract (TypeScript / Rust)
```typescript
export interface BulkImportProfile<TRow, TSummary> {
  id: "students" | "employees" | "grades";
  displayName: { ar: string; en: string };
  templateVersion: string;
  columns: OrbisColumnDef[];
  buildReferenceData: (orgId: string) => Promise<BulkReferenceData>;
  validateCell: (colId: string, value: unknown, row: TRow) => ColumnValidateResult;
  validateRow: (row: TRow, allRows: TRow[], refData: BulkReferenceData) => Map<string, string>;
  generatePreview: (rows: TRow[], refData: BulkReferenceData) => BulkPreviewSummary<TSummary>;
  commitBatch: (validRows: TRow[], options: BulkCommitOptions) => Promise<BulkCommitResult>;
}
```

---

## 5. Dynamic Template Generation System

Excel templates in Orbis are **dynamically synthesized from the live organization configuration, active academic period, and Chart of Accounts**, rather than static pre-written files.

### 5.1 Workbook Architecture
Every generated template workbook contains:
1. **Data Entry Sheet (`Data` / `البيانات`):**
   - **Header Row:** Clear bilingual labels with `*` marking required fields.
   - **Color Coding:** Required columns highlighted with subtle primary tint; optional columns neutral.
   - **Data Validation:** Dropdown constraints linked to reference formulas (e.g., `=Lists!$A$2:$A$20`).
   - **Format Rules:** Dates formatted as `YYYY-MM-DD`, numbers formatted as decimals/integers.
   - **Instruction Comments:** Cell notes explaining exact requirements and formats.
2. **Reference Data Sheet (`_References` / `_البيانات_المرجعية`) (Hidden or Protected):**
   - Live list of Classes & Sections (`ID`, `Code`, `Name`).
   - Live list of Subjects & Grading Schemes.
   - Live list of Active Fee Templates & Discounts.
   - Live list of Employee Departments, Job Titles, and Groups.
   - System allowed Enums (Genders, Blood Types, Study Modes, Employment Types).
3. **Metadata / Instructions Sheet (`Instructions` / `التعليمات`):**
   - Template Version Hash (e.g., `ORBIS-TPL-STU-2026.1-e4f9`).
   - Organization Name & Academic Year.
   - Guidelines for filling data, date formats, and error resolution.

---

## 6. Multi-Level Validation Hierarchy

Validation occurs across 6 distinct architectural levels to prevent any corrupted or inconsistent data from entering domain workflows:

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                       6-LEVEL VALIDATION ARCHITECTURE                       │
├─────────┬──────────────────────────┬────────────────────────────────────────┤
│ Level   │ Name                     │ Scope & Verification Checks            │
├─────────┼──────────────────────────┼────────────────────────────────────────┤
│ LEVEL 1 │ File Validation          │ • Magic bytes / format (.xlsx, .csv)   │
│         │                          │ • Template Version & Signature match   │
│         │                          │ • Required header columns present      │
│         │                          │ • Sheet naming integrity               │
├─────────┼──────────────────────────┼────────────────────────────────────────┤
│ LEVEL 2 │ Cell Validation          │ • Type coercion (int, decimal, date)   │
│         │                          │ • Required non-empty check             │
│         │                          │ • Regex pattern (National ID, Phone)   │
│         │                          │ • Range limits [min, max]              │
│         │                          │ • Enum membership check                │
├─────────┼──────────────────────────┼────────────────────────────────────────┤
│ LEVEL 3 │ Row Validation           │ • Cross-column consistency             │
│         │                          │ • Guardian required if student < 18    │
│         │                          │ • Class belongs to selected Level      │
│         │                          │ • Hire date <= today                   │
├─────────┼──────────────────────────┼────────────────────────────────────────┤
│ LEVEL 4 │ Cross-Row Validation     │ • Duplicate National ID in same file   │
│         │ (Batch Scope)            │ • Duplicate Student Number in file     │
│         │                          │ • Duplicate Grade entry for same class │
│         │                          │ • Class capacity overflow within batch │
├─────────┼──────────────────────────┼────────────────────────────────────────┤
│ LEVEL 5 │ Domain Validation        │ • Database duplicate check             │
│         │ (Application Services)   │ • Academic Period isOpen check         │
│         │                          │ • Real-time Class Capacity check       │
│         │                          │ • COA Account active check             │
│         │                          │ • Assessment Scheme formula validity   │
├─────────┼──────────────────────────┼────────────────────────────────────────┤
│ LEVEL 6 │ Transaction & DB         │ • Unique constraints                   │
│         │ (Atomic Persistence)     │ • Foreign key referential integrity    │
│         │                          │ • Append-only audit record creation    │
│         │                          │ • Domain event bus emission            │
└─────────┴──────────────────────────┴────────────────────────────────────────┘
```

---

## 7. Domain Profile Specifications

### 7.1 Student Bulk Import Profile

#### Field Mapping & Extraction
| Column Name (AR / EN) | Required | Target Entity | Domain Mapping |
| :--- | :--- | :--- | :--- |
| **الاسم الأول / First Name** | Yes | `Person` | `person.first_name` / `first_name_ar` |
| **اسم الأب / Middle Name** | Optional | `Person` | `person.middle_name` / `middle_name_ar` |
| **اسم العائلة / Last Name** | Yes | `Person` | `person.last_name` / `last_name_ar` |
| **الرقم الوطني / National ID** | Policy | `Person` | `person.national_id` (Checked via duplicate check) |
| **الجنس / Gender** | Yes | `Person` | `person.gender` (`male` \| `female`) |
| **تاريخ الميلاد / Birth Date** | Yes | `Person` | `person.birth_date` (`YYYY-MM-DD`) |
| **رقم الهاتف / Phone** | Optional | `Person` | `person.phone` |
| **البريد الإلكتروني / Email** | Optional | `Person` | `person.email` |
| **رقم الطالب / Student Number** | Optional | `Student` | If empty, generated via `SequenceCounterRepository` |
| **الصف / الشعبة / Class & Section**| Yes | `Enrollment` | Resolved by Name/Code to `class_id` |
| **نمط الدراسة / Study Mode** | Optional | `Enrollment` | Defaults to `regular` |
| **اسم ولي الأمر / Guardian Name** | Optional | `Person` / `User` | Links to existing or creates parent user |
| **هاتف ولي الأمر / Guardian Phone**| Optional | `Person` / `User` | Used for parent identity resolution |
| **صلة القرابة / Relationship** | Optional | `StudentGuardian` | `father` \| `mother` \| `guardian` |
| **قالب الرسوم / Fee Template** | Optional | `StudentFee` | Mandatory fees auto-assigned + optional template |

#### Application Service Invocations
- Executes `StudentService::register_student` for each row.
- Invariants strictly enforced:
  - If class capacity is reached: creates `AcademicOverrideRequest` (CapacityOverride) and flags row.
  - If mandatory fee templates exist: auto-assigns `StudentFee` records.
  - If guardian provided: invokes `StudentGuardianRepository` and `IdentityProvisioningService`.

---

### 7.2 Employee Bulk Import Profile

#### Field Mapping & Extraction
| Column Name (AR / EN) | Required | Target Entity | Domain Mapping |
| :--- | :--- | :--- | :--- |
| **الاسم بالكامل / Full Name** | Yes | `Person` | Split into first, middle, last (AR/EN) |
| **الرقم الوظيفي / Employee Number** | Optional | `Employee` | Auto-generated or validated |
| **النوع الوظيفي / Employee Type** | Yes | `Employee` | `teacher` \| `admin` \| `accountant` \| `principal` |
| **المسمى الوظيفي / Job Title** | Yes | `Employee` | `employee.job_title` |
| **القسم / Department** | Optional | `Employee` | `employee.department` |
| **تاريخ التعيين / Hire Date** | Yes | `Employee` | `employee.hire_date` |
| **نوع العقد / Employment Type** | Yes | `Employee` | `full_time` \| `part_time` \| `contractor` |
| **الراتب الأساسي / Basic Salary** | Optional | `Employee` / `Payroll` | `employee.meta.payroll.basic_salary` |
| **بدل سكن / Housing Allowance** | Optional | `Employee` / `Payroll` | `employee.meta.payroll.housing_allowance` |
| **بدل نقل / Transport Allowance**| Optional | `Employee` / `Payroll` | `employee.meta.payroll.transport_allowance` |
| **إنشاء حساب / Provision User** | Optional | `User` / `Identity` | `true` \| `false` (Triggers CSPRNG credential) |
| **المجموعة / User Group** | Optional | `UserGroup` | Assigns roles & permissions via `GroupRepository` |

#### Application Service Invocations
- Executes `EmployeeService::register_employee` for each row.
- If `Provision User = true`: calls `IdentityProvisioningService::provision_account`, creates `User`, assigns role/groups, and stages CSPRNG temporary credentials in exportable delivery report.

---

### 7.3 Grade Bulk Import Profile (Monthly, Semester, Final)

#### Field Mapping & Extraction
| Column Name (AR / EN) | Required | Target Entity | Domain Mapping |
| :--- | :--- | :--- | :--- |
| **رقم الطالب / Student Number** | Yes | `Student` | Resolved to `student_id` in current Class |
| **اسم الطالب / Student Name** | Readonly | Reference | Visual cross-verification in OrbisGrid |
| **المادة / Subject** | Yes | `ClassSubject` | Resolved to `class_subject_id` |
| **الفترة الأكاديمية / Academic Period**| Yes | `AcademicPeriod` | Verified `is_open == true` |
| **نوع التقييم / Assessment Type** | Yes | `AssessmentScheme`| `monthly` \| `midterm` \| `final` \| `practical` |
| **الدرجة / Score** | Yes | `Grade` | Validated `0 <= score <= max_score` |
| **الدرجة العظمى / Max Score** | Optional | `Grade` | Defaults from `AssessmentScheme` |
| **معذور / Is Excused** | Optional | `Grade` | `boolean` (excludes from GPA penalty) |
| **ملاحظات / Remarks** | Optional | `Grade` | `grade.notes` |

#### Application Service Invocations
- Executes `AssessmentService::record_grade`.
- Invariants strictly enforced:
  - Verifies student is actively enrolled in the specified Class and Period.
  - Verifies subject is part of class curriculum (`ClassSubjectRepository`).
  - Verifies academic period is not closed/locked.
  - Recalculates term GPA & ranking via `CalculationEngine` (`CalcEngine::evaluate`).

---

## 8. Financial & Chart of Accounts Integration

Bulk Import strictly respects Orbis double-entry accounting rules and does **not** insert arbitrary ledger records:

```
                      [Bulk Student Ingestion]
                                 │
                                 ▼
                     [Student Fee Assignment]
        (Tuition Fee Template -> 110301 Accounts Receivable)
                                 │
                     ┌───────────┴───────────┐
                     ▼                       ▼
            [Mandatory Policy]      [Optional Add-on]
            (Tuition Revenue)       (Transport/Activities)
            Account: 410101          Account: 410301
```

1. **No Root Accounts Created:** Bulk import cannot create Chart of Accounts root nodes or alter the accounting tree.
2. **Authoritative COA Linkage:** Student fees link exclusively to pre-seeded, active asset (`110301`) and revenue (`410101`, `410301`) accounts.
3. **Currency Invariant:** All amounts strictly adhere to `orgContext.baseCurrency`.
4. **Discount Processing:** Discounts specified in import files must match valid `DiscountTemplate` records.

---

## 9. Transaction & Atomicity Strategy

To guarantee zero database corruption during large bulk operations (e.g., 5,000 students), Orbis employs a **Chunked-Transaction Pipeline with Comprehensive State Tracking**:

```
[Total Rows: 1,000] ──► [Chunk 1: Rows 1-100]   ──► [BEGIN TX] ──► [Commit] ──► [Success: 100]
                    ──► [Chunk 2: Rows 101-200] ──► [BEGIN TX] ──► [Commit] ──► [Success: 100]
                    ──► [Chunk 3: Rows 201-300] ──► [BEGIN TX] ──► [Error!]  ──► [Rollback Chunk]
                                                                              └──► Staged in Grid for Retry
```

### Transaction Invariants
- **Chunk Size:** Configurable 50 to 200 records per transaction chunk.
- **Rollback Isolation:** If row #145 fails a database constraint within Chunk 2, Chunk 2 rolls back atomically. Chunk 1 remains committed.
- **Failure Quarantine:** Failed rows are highlighted in red in OrbisGrid with exact backend error messages; successful rows are marked as `Committed`. The user can fix only the failed rows and click **[Retry Remaining]**.
- **Idempotency via Content Hash:** Each batch generates a SHA-256 payload hash stored in the audit context, preventing accidental duplicate double-imports.

---

## 10. Desktop UI & OrbisGrid UX Specification

### 10.1 UI Layout & Component Structure
The bulk import screen adheres to the native Orbis dark/light design system:

```
┌─────────────────────────────────────────────────────────────────────────────────────────────────┐
│ ◄ Bulk Ingestion Studio                                                [Academic Period: 2026]  │
├─────────────────────────────────────────────────────────────────────────────────────────────────┤
│ [1. Select Profile: Students ▼]  [Download Template (.xlsx)]  [Choose File / Drag & Drop]       │
├─────────────────────────────────────────────────────────────────────────────────────────────────┤
│ FILE: students_grade7_admissions.xlsx (487 rows)   STATUS: 462 Valid | 25 Errors | 12 Warnings  │
│ [Filter: All (487) | Errors Only (25) | Warnings (12)]   [Jump to Next Error ◄ ►]               │
├─────────────────────────────────────────────────────────────────────────────────────────────────┤
│                                                                                                 │
│   #   │ Name (AR)*       │ National ID*   │ Class*        │ Fee Template      │ Status          │
│ ──────┼──────────────────┼────────────────┼───────────────┼───────────────────┼──────────────── │
│   1   │ أحمد محمد علي    │ 1092837465     │ Grade 7 - A   │ Tuition 2026      │ ✓ Valid         │
│   2   │ سارة خالد عمر    │ 1092837465 ✖  │ Grade 7 - B   │ Tuition 2026      │ ✖ Duplicate ID  │
│   3   │ يوسف فهد سالم    │ 2091827364     │ Grade 7 - C ✖ │ Tuition 2026      │ ✖ Capacity Full │
│   4   │ ريم عبد الله     │ 1029384756     │ Grade 7 - A   │ None              │ ⚠ No Fee Tpl    │
│                                                                                                 │
│                                      [OrbisGrid Virtualized]                                    │
│                                                                                                 │
├─────────────────────────────────────────────────────────────────────────────────────────────────┤
│ [Clear All]  [Export Corrected File]                     [Cancel]  [Review Preview & Commit (462)]│
└─────────────────────────────────────────────────────────────────────────────────────────────────┘
```

### 10.2 Interactive Correction Tools
- **Live Cell Revalidation:** Editing any cell in OrbisGrid immediately triggers `validateCell` and `validateRow`.
- **Error Popover & Tooltips:** Hovering or focusing an invalid cell shows a floating badge with the specific validation rule violated.
- **Filter by Error State:** A single toggle isolates only invalid rows so the user can quickly resolve 25 errors out of 1,000 rows.
- **Next Error Shortcut (F8 / Button):** Automatically scrolls and focuses the next invalid cell in the grid.
- **Batch Fill Down / Fill Right:** Selecting a valid class and pressing `Ctrl+D` copies it down the entire selected range.

---

## 11. Security, Permissions & OrgUnit Isolation

Bulk Import enforces identical security checks as manual operations:

| Import Action | Required RBAC Permission | Scope Constraint |
| :--- | :--- | :--- |
| **Student Bulk Import** | `Permission::StudentCreate` (`student:write`) | Restricted to user's authorized `org_id` and `org_unit_id` |
| **Employee Bulk Import**| `Permission::EmployeeCreate` (`employee:write`) | Superadmin or HR Administrator role required |
| **Grade Bulk Import** | `Permission::GradeWrite` (`grade:write`) | Teacher restricted to assigned `ClassSubject` instances |
| **Fee Assignment** | `Permission::ProcessPayment` (`finance:write`) | Finance officer scope |

---

## 12. Audit Trail & Verification

Every bulk operation generates structured audit logs in `audit_events`:
- **Actor:** `actor_id`, `actor_type: "User"`.
- **Action:** `action: "bulk_import_commit"`.
- **Entity:** `entity_type: "Student" | "Employee" | "Grade"`.
- **Metadata:**
  ```json
  {
    "batch_id": "c1f7a83d-3b52-47ef-8d69-823908f0a12e",
    "profile": "students",
    "filename": "students_grade7_admissions.xlsx",
    "template_version": "ORBIS-TPL-STU-2026.1",
    "total_rows": 487,
    "committed_count": 462,
    "failed_count": 25,
    "created_users_count": 462,
    "created_fees_count": 462
  }
  ```

---

## 13. Existing-vs-New Consolidation Matrix

| Architectural Element | Classification | Rationale & Mapping |
| :--- | :--- | :--- |
| **OrbisGrid (`packages/svelte-grid`)** | `KEEP EXISTING` | Already supports virtualization, inline editing, paste, validation, RTL, and selection. |
| **StudentService (`orbis-app`)** | `KEEP EXISTING` | Authoritative source for student numbering, person creation, overrides, and fee links. |
| **EmployeeService (`orbis-app`)** | `KEEP EXISTING` | Authoritative source for employee creation and unified identity provisioning. |
| **AssessmentService (`orbis-app`)** | `KEEP EXISTING` | Authoritative source for grade persistence, max score checks, and CalcEngine formulas. |
| **FinanceService (`orbis-app`)** | `KEEP EXISTING` | Authoritative source for COA accounting and fee templates. |
| **Audit Repository (`audit_events`)** | `KEEP EXISTING` | Built-in append-only audit trail captures all bulk actions. |
| **New Database Tables** | `NOT REQUIRED` | Staging is in-memory; persistence uses existing canonical 56 tables. |
| **Bulk Import UI View** | `NEW ONLY IF PROVEN` | Dedicated Studio route `(app)/[orgId]/import/+page.svelte` using OrbisGrid. |
| **Excel Template Generator** | `NEW ONLY IF PROVEN` | Utility module in `orbis-infra` / desktop generating styled dynamic workbooks. |

---

## 14. E2E Verification & Test Suite

The implementation will be verified against the following end-to-end test scenarios:

1. **TEST 1 (1,000 Students Import):** Verify atomic creation of 1,000 Persons, Students, Enrollments, Parent Users, and Auto-assigned Fees with zero orphaned records.
2. **TEST 2 (50 Employees with Accounts):** Verify Employee creation, CSPRNG password generation, and User Group assignments.
3. **TEST 3 (Monthly & Semester Grades):** Verify Grade entries against class subjects, max score validation, and live GPA calculation.
4. **TEST 4 (Mixed Valid/Invalid Excel):** Verify invalid rows are highlighted in red in OrbisGrid; valid rows pass preview.
5. **TEST 5 (Duplicate National ID):** Verify file-level and DB-level duplicate detection flags invalid cells immediately.
6. **TEST 6 (Class Capacity Overflow):** Verify capacity breach triggers `CapacityOverride` requests and halts direct enrollment.
7. **TEST 7 (Closed Academic Period):** Verify grade import is rejected with clear error when period `is_open == false`.
8. **TEST 8 (Chunked Rollback on Partial Failure):** Verify failure in chunk 3 rolls back chunk 3 cleanly without altering chunks 1 and 2.
9. **TEST 9 (Idempotent Double-Upload):** Verify uploading identical file twice detects content hash collision and warns user.
10. **TEST 10 (Large Import 10k+ Rows):** Verify OrbisGrid maintains 60 FPS scrolling and memory footprint under 150 MB.
11. **TEST 11 (RTL & Arabic Localization):** Verify Arabic column headers, RTL layout, and Arabic error messages.
12. **TEST 12 (Unauthorized User Access):** Verify user without `student:write` permission receives `403 Forbidden`.

---

## 15. Implementation Phases & Roadmap

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                       BULK IMPORT IMPLEMENTATION ROADMAP                    │
├──────────┬─────────────────────────────┬────────────────────────────────────┤
│ Phase    │ Deliverable                 │ Key Components                     │
├──────────┼─────────────────────────────┼────────────────────────────────────┤
│ Phase 1  │ Dynamic Template Generator  │ • Rust/TS Excel builder            │
│          │ & Reference Exporters       │ • Formula-driven reference sheets  │
│          │                             │ • Column definitions with schemas  │
├──────────┼─────────────────────────────┼────────────────────────────────────┤
│ Phase 2  │ Multi-Level Validation      │ • L1-L4 frontend validators        │
│          │ & Coercion Engine           │ • L5 domain validation IPC calls   │
│          │                             │ • OrbisGrid error map integration  │
├──────────┼─────────────────────────────┼────────────────────────────────────┤
│ Phase 3  │ Desktop Bulk Import Studio  │ • `(app)/[orgId]/import/+page`     │
│          │ (OrbisGrid Integration)     │ • Error navigation toolbar         │
│          │                             │ • Real-time preview summary cards  │
├──────────┼─────────────────────────────┼────────────────────────────────────┤
│ Phase 4  │ Domain Profile Hooks &      │ • Student bulk registration hook   │
│          │ Transaction Pipelines       │ • Employee & Identity hook         │
│          │                             │ • Grade & CalcEngine hook          │
├──────────┼─────────────────────────────┼────────────────────────────────────┤
│ Phase 5  │ E2E Testing & Audit Pass    │ • 12 E2E test executions           │
│          │                             │ • Memory & performance benchmarks  │
│          │                             │ • Verification pass & sign-off     │
└──────────┴─────────────────────────────┴────────────────────────────────────┘
```

---

## 16. Architectural Sign-Off

```
BULK_ENTRY_ARCHITECTURE_STATUS: APPROVED
```
All domain boundaries, application use case interactions, Chart of Accounts mappings, validation layers, transaction boundaries, and OrbisGrid integrations have been verified against active repository code without introducing redundant database tables or bypassing canonical ERP business rules.
