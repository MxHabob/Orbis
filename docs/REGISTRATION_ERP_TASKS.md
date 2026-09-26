# Orbis ERP Registration Subsystem — Engineering Task Breakdown & Execution Tracker

> **Document Status:** Active Execution (`PHASE_1_IN_PROGRESS`)  
> **Master Reference Plan:** [`docs/registration_erp_architecture_plan.md`](file:///d:/projects/Orbis/docs/registration_erp_architecture_plan.md)  
> **Target Subsystem:** Core Registration, Admission Policies, Numbering Engine, Document Requirements & Verification Dossier  

---

## 1. Execution Overview & Progress Summary

| Phase | Description | Status | Total Tasks | Completed | Remaining |
|---|---|---|---|---|---|
| **Phase 1** | Core Domain Modeling, Sequence Counters & DB Migrations | 🟡 In Progress | 6 | 0 | 6 |
| **Phase 2** | Application Use Cases, Policy Enforcement & Service Layer | ⚪ Pending | 7 | 0 | 7 |
| **Phase 3** | REST API Routes, OpenAPI Spec & Tauri IPC Commands | ⚪ Pending | 6 | 0 | 6 |
| **Phase 4** | Desktop Svelte 5 / Runes UI Modernization & DocForge Vector Templates | ⚪ Pending | 8 | 0 | 8 |
| **Total** | | | **27 Tasks** | **0** | **27** |

---

## 2. Detailed Task Breakdown

### Phase 1: Core Domain Modeling, Sequence Engine & Database Normalization

- [ ] **TASK-REG-101: Define Typed Admission & Numbering Domain Models**
  - **Layer:** `apps/api-rust/crates/orbis-domain/src/entities/`
  - **Target Files:**
    - Create [`admission.rs`](file:///d:/projects/Orbis/apps/api-rust/crates/orbis-domain/src/entities/admission.rs)
    - Modify [`settings.rs`](file:///d:/projects/Orbis/apps/api-rust/crates/orbis-domain/src/entities/settings.rs)
    - Export in [`mod.rs`](file:///d:/projects/Orbis/apps/api-rust/crates/orbis-domain/src/entities/mod.rs)
  - **Scope:** 
    - Define `NumberingSchema` (prefix, year format, sequence padding, separator).
    - Define `AdmissionPolicy` (requires approval, allow independent enrollment, allow duplicate national ID, auto-assign fees, max class size).
    - Harmonize `OrganizationSettings` so Rust `admission` / `enrollment` align seamlessly with frontend payloads.
  - **Acceptance Criteria:** `cargo check -p orbis-domain` passes with unit tests for numbering format serialization and invariants.

- [ ] **TASK-REG-102: Define Document Requirement Domain Entities**
  - **Layer:** `apps/api-rust/crates/orbis-domain/src/entities/`
  - **Target Files:**
    - Create [`document_requirement.rs`](file:///d:/projects/Orbis/apps/api-rust/crates/orbis-domain/src/entities/document_requirement.rs)
    - Modify [`document.rs`](file:///d:/projects/Orbis/apps/api-rust/crates/orbis-domain/src/entities/document.rs)
  - **Scope:**
    - Define `DocumentRequirement` (target type: Stage/GradeLevel/Program, mandatory flag, blocks enrollment flag, conditional rules).
    - Extend `Document` or add `DocumentVerificationStatus` (`Pending`, `Verified`, `Rejected`, `Expired`) with verification audit metadata.
  - **Acceptance Criteria:** Types are strongly typed, serializable with serde, and implement necessary domain predicates.

- [ ] **TASK-REG-103: Extend Override Request Types for Registration & Admission**
  - **Layer:** `apps/api-rust/crates/orbis-domain/src/entities/`
  - **Target Files:**
    - Modify [`override_request.rs`](file:///d:/projects/Orbis/apps/api-rust/crates/orbis-domain/src/entities/override_request.rs)
  - **Scope:**
    - Add `OverrideType::AdmissionApproval` and refine `CapacityOverride`, `LateEnrollment`, `AgeBypass`.
  - **Acceptance Criteria:** `OverrideType` displays cleanly in Arabic/English with round-trip serde tests.

- [ ] **TASK-REG-104: Create SeaORM Migration for `document_requirements` and `sequence_counters`**
  - **Layer:** `apps/api-rust/crates/orbis-infra/src/migration/`
  - **Target Files:**
    - Create [`m20240105_000001_registration_schema.rs`](file:///d:/projects/Orbis/apps/api-rust/crates/orbis-infra/src/migration/m20240105_000001_registration_schema.rs)
    - Register in [`orbis-infra/src/migration/mod.rs`](file:///d:/projects/Orbis/apps/api-rust/crates/orbis-infra/src/migration/mod.rs)
  - **Scope:**
    - Create `document_requirements` table (columns: `id`, `org_id`, `target_type`, `target_id`, `code`, `name_ar`, `name_en`, `is_mandatory`, `blocks_enrollment`, `condition_json`, `created_at`, `updated_at`).
    - Create `sequence_counters` table (columns: `org_id`, `sequence_type`, `academic_year`, `current_val`, `updated_at`).
  - **Acceptance Criteria:** SeaORM migration compiles and executes cleanly in `up` and `down`.

- [ ] **TASK-REG-105: Implement SeaORM Models and Repository Ports**
  - **Layer:** `apps/api-rust/crates/orbis-infra/src/models/` & `repositories/`
  - **Target Files:**
    - Create [`document_requirement.rs`](file:///d:/projects/Orbis/apps/api-rust/crates/orbis-infra/src/models/document_requirement.rs)
    - Create [`sequence_counter.rs`](file:///d:/projects/Orbis/apps/api-rust/crates/orbis-infra/src/models/sequence_counter.rs)
    - Implement repositories in `orbis-infra/src/repositories/`
  - **Scope:** Concrete SeaORM entity definitions and repository implementations satisfying `DocumentRequirementRepository` and `SequenceCounterRepository`.
  - **Acceptance Criteria:** Clean compilation and unit tests against in-memory SQLite backend.

- [ ] **TASK-REG-106: Implement Atomic Numbering Sequence Generator in Infrastructure**
  - **Layer:** `apps/api-rust/crates/orbis-infra/`
  - **Target Files:**
    - Create [`sequence_generator.rs`](file:///d:/projects/Orbis/apps/api-rust/crates/orbis-infra/src/adapters/sequence_generator.rs)
  - **Scope:**
    - Concurrency-safe atomic increment of student numbering sequence counters per organization and academic year.
    - Seeding from `MAX(student_number)` on first run if counter row is missing.
  - **Acceptance Criteria:** Concurrent calls yield unique, sequentially ordered numbers without gaps or collisions.

---

### Phase 2: Application Use Cases, Policy Enforcement & Service Layer

- [ ] **TASK-REG-201: Refactor `StudentService::register_student` for Policy-Driven Numbering**
  - **Layer:** `apps/api-rust/crates/orbis-app/src/services/`
  - **Target Files:** [`student_service.rs`](file:///d:/projects/Orbis/apps/api-rust/crates/orbis-app/src/services/student_service.rs)
  - **Scope:** Replace hardcoded `format!("STD-...")` with call to `SequenceGenerator` evaluated against the effective `NumberingSchema`.
  - **Acceptance Criteria:** Student number format reflects configured prefix, year format, and digit length.

- [ ] **TASK-REG-202: Implement Admission Governance & Approval State Handling**
  - **Layer:** `apps/api-rust/crates/orbis-app/src/services/`
  - **Target Files:** [`student_service.rs`](file:///d:/projects/Orbis/apps/api-rust/crates/orbis-app/src/services/student_service.rs)
  - **Scope:**
    - Check `AdmissionPolicy.requires_approval`.
    - If required, create `Enrollment` in `PendingApproval` status and create an `AcademicOverrideRequest` of type `AdmissionApproval`.
    - If approval not required, activate `Enrollment` immediately.
  - **Acceptance Criteria:** New applicant registration enters `PendingApproval` when policy is active.

- [ ] **TASK-REG-203: Implement Class Capacity & Late Enrollment Verification Invariants**
  - **Layer:** `apps/api-rust/crates/orbis-app/src/services/`
  - **Target Files:** [`student_service.rs`](file:///d:/projects/Orbis/apps/api-rust/crates/orbis-app/src/services/student_service.rs)
  - **Scope:**
    - Query active enrollment count for target `class_id`.
    - Compare against `class.max_capacity` or `AdmissionPolicy.max_class_size`.
    - If capacity exceeded without active `CapacityOverride`, reject with domain error or create override request.
    - Verify against `AcademicPeriod` start/end dates.
  - **Acceptance Criteria:** Over-capacity registrations are blocked or routed to override review.

- [ ] **TASK-REG-204: Implement Dedicated Server-Side Duplicate Check Query**
  - **Layer:** `apps/api-rust/crates/orbis-app/src/services/`
  - **Target Files:** [`student_service.rs`](file:///d:/projects/Orbis/apps/api-rust/crates/orbis-app/src/services/student_service.rs)
  - **Scope:**
    - Add `check_duplicate_student(org_id, national_id, phone, email, current_student_id)`.
    - Perform efficient $O(1)$ SQL index lookup across `person` and `student` tables.
  - **Acceptance Criteria:** Fast, secure detection of duplicate identity without sending all users to client.

- [ ] **TASK-REG-205: Implement Document Requirement Evaluation Service**
  - **Layer:** `apps/api-rust/crates/orbis-app/src/services/`
  - **Target Files:** [`document_service.rs`](file:///d:/projects/Orbis/apps/api-rust/crates/orbis-app/src/services/document_service.rs)
  - **Scope:**
    - Add `get_required_documents_for_student(org_id, student_id, grade_id)`.
    - Return checklist of mandatory vs optional documents with current fulfillment status (`Missing`, `Uploaded`, `Verified`).
  - **Acceptance Criteria:** Accurate checklist returned for any student based on their grade and profile conditions.

- [ ] **TASK-REG-206: Implement Atomic Enrollment Fee Generator**
  - **Layer:** `apps/api-rust/crates/orbis-app/src/services/`
  - **Target Files:** [`finance_service.rs`](file:///d:/projects/Orbis/apps/api-rust/crates/orbis-app/src/services/finance_service.rs) & [`student_service.rs`](file:///d:/projects/Orbis/apps/api-rust/crates/orbis-app/src/services/student_service.rs)
  - **Scope:**
    - In `StudentService::register_student`, if `auto_assign_fees` is enabled, atomically map mandatory and selected optional fee templates to `student_fees` in the same database transaction.
  - **Acceptance Criteria:** No partial fee assignment; all line items created cleanly upon enrollment activation.

- [ ] **TASK-REG-207: Harmonize Organization Settings Service Serialization**
  - **Layer:** `apps/api-rust/crates/orbis-app/src/services/`
  - **Target Files:** [`organization_service.rs`](file:///d:/projects/Orbis/apps/api-rust/crates/orbis-app/src/services/organization_service.rs)
  - **Scope:** Ensure `get_settings` and `update_settings` preserve typed `AdmissionPolicy` and custom UI field matrix without dropping data.
  - **Acceptance Criteria:** Settings saved via API or Tauri round-trip with 100% fidelity.

---

### Phase 3: REST API Routes, OpenAPI Spec & Tauri IPC Commands

- [ ] **TASK-REG-301: Add Duplicate Check & Registration API Routes**
  - **Layer:** `apps/api-rust/crates/orbis-server/src/api/`
  - **Target Files:** [`student_routes.rs`](file:///d:/projects/Orbis/apps/api-rust/crates/orbis-server/src/api/student_routes.rs)
  - **Scope:** Expose `POST /api/v1/students/check-duplicate` and updated `POST /api/v1/students/register`.
  - **Acceptance Criteria:** Validated with utoipa OpenAPI annotations.

- [ ] **TASK-REG-302: Add Document Requirements API Routes**
  - **Layer:** `apps/api-rust/crates/orbis-server/src/api/`
  - **Target Files:** Create [`document_requirement_routes.rs`](file:///d:/projects/Orbis/apps/api-rust/crates/orbis-server/src/api/document_requirement_routes.rs)
  - **Scope:** CRUD endpoints for document requirement rules: `GET/POST/PUT/DELETE /api/v1/document-requirements`.
  - **Acceptance Criteria:** Clean REST endpoints with tenant security checks.

- [ ] **TASK-REG-303: Update Tauri Student & Document IPC Commands**
  - **Layer:** `apps/desktop/src-tauri/src/commands/`
  - **Target Files:** [`student_cmds.rs`](file:///d:/projects/Orbis/apps/desktop/src-tauri/src/commands/student_cmds.rs), [`document_cmds.rs`](file:///d:/projects/Orbis/apps/desktop/src-tauri/src/commands/document_cmds.rs)
  - **Scope:** Add Tauri command handlers: `student_check_duplicate`, `document_requirements_list`, `document_requirements_save`.
  - **Acceptance Criteria:** `cargo check -p orbis-desktop-tauri` passes cleanly.

- [ ] **TASK-REG-304: Export Canonical OpenAPI 3.1 & Regenerate TypeScript SDK**
  - **Layer:** `packages/sdk/` & `apps/api-rust/crates/orbis-server/`
  - **Target Files:** Run `cargo run --bin export_openapi` and `bun run build` in `packages/sdk`.
  - **Scope:** Deterministically generate TypeScript client types and functions for all new routes.
  - **Acceptance Criteria:** `@orbis/sdk` exports `checkDuplicateStudent`, `listDocumentRequirements`, `createDocumentRequirement` without type errors.

- [ ] **TASK-REG-305: Update Desktop API Client Wrapper**
  - **Layer:** `apps/desktop/src/lib/`
  - **Target Files:** [`api.ts`](file:///d:/projects/Orbis/apps/desktop/src/lib/api.ts)
  - **Scope:** Wire new SDK methods into the desktop unified API bridge.
  - **Acceptance Criteria:** Type-safe access in Svelte components.

- [ ] **TASK-REG-306: Integration Verification for IPC & REST**
  - **Layer:** Full Stack
  - **Scope:** Automated verification tests comparing Tauri IPC and Axum REST responses for identical inputs.
  - **Acceptance Criteria:** Exact parity between desktop native and web API responses.

---

### Phase 4: Desktop Svelte 5 / Runes UI Modernization & DocForge Vector Templates

- [ ] **TASK-REG-401: Rebuild `RegistrationSettingsForm` with Strict Typing & Preview**
  - **Layer:** `apps/desktop/src/lib/features/settings/`
  - **Target Files:**
    - [`registration-settings.svelte`](file:///d:/projects/Orbis/apps/desktop/src/lib/features/settings/registration-settings.svelte)
    - [`registration-settings-form.svelte`](file:///d:/projects/Orbis/apps/desktop/src/lib/features/settings/components/registration-settings-form.svelte)
  - **Scope:**
    - Align payload keys directly with backend `AdmissionPolicy`.
    - Live dynamic student number preview reacting to prefix/year/digit modifications.
    - Connect Document Requirements tab directly to `DocumentRequirement` API.
  - **Acceptance Criteria:** Saving settings persists to backend and loads correctly on refresh.

- [ ] **TASK-REG-402: Rebuild Student Registration Form State Manager (`form-state.svelte.ts`)**
  - **Layer:** `apps/desktop/src/lib/features/students/`
  - **Target Files:** [`form-state.svelte.ts`](file:///d:/projects/Orbis/apps/desktop/src/lib/features/students/form-state.svelte.ts)
  - **Scope:**
    - Remove insecure `api.listUsers()` local search; replace with `api.checkDuplicateStudent`.
    - Connect document verification tab to real required document rules.
    - Implement state transitions: Draft, Submit for Review, Direct Enroll.
  - **Acceptance Criteria:** Debounced backend duplicate check warning with zero client-side memory leakage.

- [ ] **TASK-REG-403: Implement Document Requirements Management Window (`FormGridView`)**
  - **Layer:** `apps/desktop/src/lib/features/settings/`
  - **Target Files:** Create [`document-requirements-manager.svelte`](file:///d:/projects/Orbis/apps/desktop/src/lib/features/settings/document-requirements-manager.svelte)
  - **Scope:** Standard `FormGridView` matching Competitor Archetype 2: Form on top (Document Title Ar/En, Target Grade, Mandatory switch), CRUD toolbar, data grid below with stage filtering.
  - **Acceptance Criteria:** Full CRUD operations on document requirements with instant grid updates.

- [ ] **TASK-REG-404: Modernize `StudentForm` into `EntityWorkspace360` Dossier**
  - **Layer:** `apps/desktop/src/lib/features/students/`
  - **Target Files:** [`student-form.svelte`](file:///d:/projects/Orbis/apps/desktop/src/lib/features/students/student-form.svelte)
  - **Scope:**
    - Top header displaying student photo, admission status pill (`Draft`, `Pending Approval`, `Enrolled`), and live student number.
    - Clean horizontal/vertical stepper across Personal, Guardian, Academic, Financials, and Documents.
  - **Acceptance Criteria:** Smooth Svelte 5 Rune-based reactive state transitions with zero lag.

- [ ] **TASK-REG-405: Rebuild Student Document Checklist Section (`documents-section.svelte`)**
  - **Layer:** `apps/desktop/src/lib/features/students/sections/`
  - **Target Files:** [`documents-section.svelte`](file:///d:/projects/Orbis/apps/desktop/src/lib/features/students/sections/documents-section.svelte)
  - **Scope:**
    - Display checklist of required documents for selected grade.
    - Show badge indicators: `مكتمل` (Complete), `مفقود` (Missing), `اختياري` (Optional).
    - Allow immediate file drag-and-drop or scanning attachment.
  - **Acceptance Criteria:** Mandatory document missing warning prevents direct enrollment unless overridden.

- [ ] **TASK-REG-406: Integrate Registration Approval Flow in `ApprovalsManager`**
  - **Layer:** `apps/desktop/src/lib/features/settings/`
  - **Target Files:** [`approvals-manager.svelte`](file:///d:/projects/Orbis/apps/desktop/src/lib/features/settings/approvals-manager.svelte)
  - **Scope:**
    - Add filter tab for `Admission Approvals` & `Capacity Overrides`.
    - Provide single-click Approve / Reject with resolution notes.
    - On approval, automatically activate `Enrollment` and trigger fee creation.
  - **Acceptance Criteria:** Supervisor can approve pending student from the centralized inbox.

- [ ] **TASK-REG-407: Create Official Blank Admission Form & Student Card DocForge AST Templates**
  - **Layer:** `docforge-engine` / Templates
  - **Target Files:** Create [`blank_registration_application.json`](file:///d:/projects/Orbis/packages/ui/templates/blank_registration_application.json)
  - **Scope:** Standalone 18-node DocForge AST template with Arabic HarfBuzz shaping for printing blank admission forms and official student registration cards.
  - **Acceptance Criteria:** High-resolution vector PDF export rendered through DocForge without external dependencies.

- [ ] **TASK-REG-408: Complete Legacy Cleanup Pass & Section Completion Verification**
  - **Layer:** Full Stack
  - **Scope:**
    - Remove dead code, orphan interfaces, and hardcoded test heuristics.
    - Verify Biome linting, TypeScript compilation, and Rust compiler checks.
  - **Acceptance Criteria:** Zero lint/type errors across Rust and Svelte workspaces.

---

> **Tracker Status:** Active. Proceeding to execute Phase 1 tasks sequentially.
