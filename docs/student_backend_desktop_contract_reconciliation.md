# Orbis — Student Domain: Backend-First Contract Reconciliation

> **Document Version:** 1.0.0-PROD  
> **Target Subsystem:** Student Domain & Lifecycle  
> **Source of Truth:** Rust Clean Architecture (`orbis-domain`, `orbis-app`, `orbis-infra`, `orbis-server`) + SQLite / Postgres Database Schema  
> **Date:** September 2026

---

## 1. Executive Summary & Source of Truth

The Student domain is a core pillar of Orbis. In accordance with the Cardinal Law **CONSOLIDATION BEFORE ADDITION** and the strict **BACKEND DATABASE → DOMAIN → APPLICATION → API → SDK → DESKTOP** pipeline, this document establishes the authoritative contract for the entire Student lifecycle.

Desktop components must never invent lifecycle states, fake user accounts for parents/guardians, synthetic identifiers, or frontend-only fallback logic.

```mermaid
graph TD
    DB[(Backend Database)] --> Domain[orbis-domain: Person / Student / StudentGuardian / Enrollment]
    Domain --> App[orbis-app: StudentService / Use Cases]
    App --> API[orbis-server & Tauri IPC: Student Routes / Commands]
    API --> SDK[@orbis/sdk & @orbis/schemas]
    SDK --> Desktop[Desktop Svelte 5 Feature: Runes, FormController, TanStack Query]
```

---

## 2. Current Backend Database Schema & Field Inventory

### 2.1 Table: `persons`
Authoritative owner of human identity, biometrics, vital records, and personal contact info.

| Column | Type | Nullable | Description / Semantics |
|---|---|---|---|
| `id` | `UUID` (PK) | No | Canonical Person Unique Identifier |
| `org_id` | `UUID` (FK) | Yes | Optional owning organization context |
| `first_name` | `VARCHAR(100)` | No | Given / First Name (Latin or Primary) |
| `middle_name` | `VARCHAR(100)` | Yes | Middle / Father's Name |
| `last_name` | `VARCHAR(100)` | No | Family / Last / Surname |
| `first_name_ar` | `VARCHAR(100)` | Yes | Given Name in Arabic script |
| `middle_name_ar` | `VARCHAR(100)` | Yes | Middle Name in Arabic script |
| `last_name_ar` | `VARCHAR(100)` | Yes | Family / Surname in Arabic script |
| `display_name` | `VARCHAR(255)` | Yes | Explicit display override name |
| `national_id` | `VARCHAR(50)` | Yes | Civil registration / National ID (indexed for deduplication) |
| `passport_number`| `VARCHAR(50)` | Yes | International Travel Document / Passport number |
| `gender` | `VARCHAR(20)` | Yes | Biological/Civil sex: `male`, `female`, `other` |
| `birth_date` | `DATE / TEXT` | Yes | Date of birth (`YYYY-MM-DD`) |
| `place_of_birth` | `VARCHAR(150)`| Yes | Birth location / city / governorate |
| `nationality` | `VARCHAR(100)` | Yes | ISO nationality / country of citizenship |
| `blood_type` | `VARCHAR(10)` | Yes | ABO/Rh blood group (`A+`, `O-`, etc.) |
| `medical_notes` | `TEXT` | Yes | Chronic conditions, allergies, emergency medical notes |
| `special_needs_notes` | `TEXT`| Yes | Accessibility and individualized learning requirements |
| `phone` | `VARCHAR(50)` | Yes | Primary contact phone number |
| `email` | `VARCHAR(255)` | Yes | Personal contact email |
| `address` | `TEXT` | Yes | Physical residential address |
| `avatar_url` | `TEXT` | Yes | Profile portrait asset path/URL |
| `created_at` | `TIMESTAMPTZ` | No | Record creation timestamp |
| `updated_at` | `TIMESTAMPTZ` | No | Record last modification timestamp |

### 2.2 Table: `students`
Authoritative domain representation of a Student enrolled within an educational institution.

| Column | Type | Nullable | Description / Semantics |
|---|---|---|---|
| `id` | `UUID` (PK) | No | Canonical Student Identifier |
| `org_id` | `UUID` (FK) | No | Institutional tenant context |
| `person_id` | `UUID` (FK) | No | 1:1 Reference to `persons.id` |
| `user_id` | `UUID` (FK) | Yes | Optional 1:1 Reference to IAM account `users.id` for portal login |
| `student_number`| `VARCHAR(50)` | No | Sequential institutional student admission number |
| `status` | `VARCHAR(30)` | No | Lifecycle status: `active`, `on_leave`, `suspended`, `withdrawn`, `transferred_out`, `graduated`, `expelled` |
| `standing` | `VARCHAR(30)` | No | Academic standing: `good_standing`, `honor_list`, `warning_1`, `warning_2`, `warning_3`, `probation`, `dismissed` |
| `admitted_at` | `TIMESTAMPTZ` | No | Initial institutional admission timestamp |
| `graduated_at` | `TIMESTAMPTZ` | Yes | Official graduation completion timestamp |
| `is_deleted` | `BOOLEAN` | No | Soft-delete flag (default `false`) |
| `deleted_at` | `TIMESTAMPTZ` | Yes | Soft-delete audit timestamp |
| `meta` | `JSONB / TEXT` | Yes | Extensible key-value metadata |
| `org_unit_id` | `UUID` (FK) | Yes | Branch / campus / organizational unit reference |
| `created_by` | `UUID` (FK) | Yes | Audit user who registered the student |
| `updated_by` | `UUID` (FK) | Yes | Audit user who last updated the student |
| `created_at` | `TIMESTAMPTZ` | No | Creation timestamp |
| `updated_at` | `TIMESTAMPTZ` | No | Update timestamp |

### 2.3 Table: `student_guardians`
Authoritative M:N relationship linking Students with Guardians (who are canonical Persons).

| Column | Type | Nullable | Description / Semantics |
|---|---|---|---|
| `id` | `UUID` (PK) | No | Linkage record unique identifier |
| `student_id` | `UUID` (FK) | No | References `students.id` |
| `guardian_id` | `UUID` (FK) | No | References `persons.id` (The guardian as a Person) |
| `relationship` | `VARCHAR(50)` | No | English canonical relationship (`father`, `mother`, `brother`, `sister`, `uncle`, `grandparent`, `legal_guardian`) |
| `relationship_ar`| `VARCHAR(50)` | Yes | Arabic localized relationship (`أب`, `أم`, `أخ`, etc.) |
| `is_primary` | `BOOLEAN` | No | Primary custodian flag |
| `is_emergency_contact`| `BOOLEAN` | No | Authorized emergency contact notification target |
| `is_authorized_pickup` | `BOOLEAN` | No | Legally authorized for school dismissal pickup |
| `is_payer` | `BOOLEAN` | No | Financial debtor / billing recipient |
| `payment_share_percent` | `DOUBLE PRECISION` | Yes | Financial obligation split percentage (0-100) |
| `occupation` | `VARCHAR(100)` | Yes | Guardian professional occupation |
| `employer` | `VARCHAR(150)` | Yes | Employer / company name |
| `work_phone` | `VARCHAR(50)` | Yes | Workplace landline / phone |
| `notes` | `TEXT` | Yes | Legal or administrative guardianship notes |
| `created_at` | `TIMESTAMPTZ` | No | Linkage creation timestamp |
| `updated_at` | `TIMESTAMPTZ` | No | Linkage update timestamp |

### 2.4 Table: `enrollments`
Academic term/period placement entity.

| Column | Type | Nullable | Description / Semantics |
|---|---|---|---|
| `id` | `UUID` (PK) | No | Enrollment record unique identifier |
| `org_id` | `UUID` (FK) | No | Owning organization ID |
| `period_id` | `UUID` (FK) | No | Academic Period (`academic_periods.id`) |
| `student_id` | `UUID` (FK) | No | References `students.id` |
| `class_id` | `UUID` (FK) | Yes | Assigned classroom section (`classes.id`) |
| `student_number`| `VARCHAR(50)` | No | Snapshot of student number at enrollment |
| `status` | `VARCHAR(30)` | No | `active`, `pending_approval`, `on_hold`, `withdrawn`, `transferred`, `graduated`, `suspended` |
| `study_mode` | `VARCHAR(30)` | No | `regular`, `homeschooled`, `auditor`, `visiting`, `distance_learning`, `part_time`, `exchange`, `special_needs` |
| `is_attendance_exempt` | `BOOLEAN` | No | Flag indicating exemption from attendance rules |
| `is_grade_exempt` | `BOOLEAN` | No | Flag indicating exemption from grading evaluations |
| `enrolled_at` | `TIMESTAMPTZ` | No | Enrollment activation timestamp |
| `withdrawn_at`| `TIMESTAMPTZ` | Yes | Withdrawal timestamp if applicable |
| `withdrawal_reason` | `TEXT` | Yes | Justification for withdrawal |
| `meta` | `JSONB` | Yes | Dynamic configuration metadata |

---

## 3. Person vs. Student vs. User vs. Guardian Resolution

| Question | Authoritative Backend Truth | Desktop Implementation Mandate |
|---|---|---|
| **What represents a Person?** | `Person` entity (`persons` table). Holds names (AR/EN), National ID, Passport, gender, birth date, blood type, address, contacts. | Forms editing personal data write directly to `Person` attributes via `update_student_profile`. |
| **What represents a Student?** | `Student` entity (`students` table). Holds institutional status, standing, student number, admission date, and links to `person_id`. | Represents institutional tenure; displayed in header badge and status controls. |
| **What represents a Guardian?** | A `Person` entity linked to a `Student` via `student_guardians`. | Guardian forms MUST NOT invoke `registerUser` or expect a `user_id`. They reference or create a canonical `Person`. |
| **What represents an IAM User?** | `User` entity (`users` table). Holds login credentials, email/phone username, password hash, MFA, and RBAC roles. | Account provisioning is separate and optional. Students and Guardians do NOT require IAM accounts for educational record operations. |
| **Can a Guardian exist without an account?** | **YES**. 100% of guardians exist primarily as `Person` entities with contact information. | Eliminate synthetic email/password generation in Guardian dialogs. |
| **Can one Guardian link to multiple Students?** | **YES**. M:N relationship through `student_guardians`. | Searching existing Persons allows fast linkage to siblings. |

---

## 4. Student Lifecycle & Transaction Boundaries

```mermaid
stateDiagram-v2
    [*] --> Admission_Registration: StudentService::register_student
    Admission_Registration --> Active: Automatic sequence number + Fee generation
    Active --> OnLeave: Leave request approved
    OnLeave --> Active: Return from leave
    Active --> Suspended: Disciplinary action
    Suspended --> Active: Reinstatement
    Active --> Graduated: Completion of curriculum
    Active --> Withdrawn: Formal departure
    Active --> TransferredOut: Transfer to another institution
    Active --> Expelled: Disciplinary dismissal
```

### Registration Transaction Boundary
`StudentService::register_student` provides atomic orchestration:
1. Validates unique constraints (National ID, Email, Phone deduplication).
2. Creates or links the `Person` record.
3. Generates the sequential `student_number` via `SequenceCounterRepository`.
4. Creates the `Student` record.
5. If `initial_period_id` is supplied: creates the initial `Enrollment` record, optionally evaluating class capacity or raising an `AcademicOverrideRequest`.
6. Evaluates applicable `FeeTemplate` rules and automatically provisions initial `StudentFee` records.

Desktop MUST invoke `api.registerStudent(input)` directly as a single canonical mutation, rather than orchestrating multi-step manual calls.

---

## 5. Field Mapping Matrix & Obsolete Contract Removal

| Current Desktop Field | Backend Field | Backend Entity | Type | Nullable | Meaning / Rule | Action |
|---|---|---|---|---|---|---|
| `guardianUserId` | `guardian_id` | `StudentGuardian` | `UUID` | No | ID of the Guardian `Person` record | **REPLACE** (`guardian_id` referencing Person) |
| `password` | *None* | `None` | `String` | N/A | Obsolete synthetic IAM password in student form | **REMOVE** |
| `name` | `first_name` + `last_name` | `Person` | `String` | No | English / Latin name components | **REMAP** |
| `name_ar` | `first_name_ar` + `last_name_ar` | `Person` | `String` | Yes | Arabic name components | **REMAP** |
| `student_number` | `student_number` | `Student` | `String` | Yes (autogen) | Sequential institutional admission number | **BACKEND-GENERATED** (optional override) |
| `sex` | `gender` | `Person` | `String` | Yes | `male`, `female`, `other` | **RENAME** to `gender` |
| `study_mode` | `study_mode` | `Enrollment` | `Enum` | No | Study mode per term | **KEEP** |
| `standing` | `standing` | `Student` | `Enum` | No | `good_standing`, `probation`, etc. | **KEEP** |
| `is_primary` | `is_primary` | `StudentGuardian` | `Boolean` | No | Primary guardian flag | **KEEP** |
| `is_emergency_contact`| `is_emergency_contact`| `StudentGuardian`| `Boolean` | No | Emergency contact flag | **KEEP** |
| `is_authorized_pickup`| `is_authorized_pickup` | `StudentGuardian`| `Boolean` | No | Authorized pickup flag | **KEEP** |
| `is_payer` | `is_payer` | `StudentGuardian` | `Boolean` | No | Financial debtor flag | **KEEP** |

---

## 6. Svelte 5 Runes & Desktop Architecture Mandate

1. **Zero `$effect` Usages:**
   - All derived state (e.g. date formatting, document compliance percentages, dirty checks) MUST use `$derived` or `$derived.by`.
   - Dynamic form population must be initialized in `onMount` or passed declaratively through props, never by watching reactive props in cascading `$effect` triggers.
2. **Form Controller & Zod Validation:**
   - Use `FormController` from `$lib/shared/forms/form-controller.svelte` backed by `@orbis/schemas`.
   - Never use loose `$state<any[]>` arrays for typed domain data.
3. **TanStack Query & Invalidation:**
   - All data fetching uses `createQuery` and `createMutation` with canonical `queryKeys.student.*`.
   - Mutations systematically invalidate affected queries (`queryKeys.student.list`, `queryKeys.student.detail`, `queryKeys.student.guardians`).
