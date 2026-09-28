# Orbis — Enterprise Educational Operating System (Agent Execution Contract)

> **Document Version:** 2.0.0-PROD  
> **Workspace Root:** `d:/projects/Orbis`  
> **Master Execution Plan:** `docs/ORBIS_ERP_TRANSFORMATION_EXECUTION_PLAN.md`  

---

## 1. Executive Instructions for AI Coding Agents

Every agent starting a session in this repository MUST:
1. **Read this AGENTS.md file first.**
2. **Read the current CONTINUATION STATE at the bottom of this file.**
3. **Inspect the existing implementation before writing any code.**
4. **Follow the Cardinal Law: CONSOLIDATION BEFORE ADDITION.**
5. **Never create mock data, placeholder buttons, empty handlers, or parallel duplicate components.**
6. **Never use `as any` casts to hide architectural mismatches.**
7. **Perform a Legacy Removal Pass after modifying any vertical section.**
8. **Verify the Section Completion Contract before declaring any section complete.**
9. **Update the CONTINUATION STATE section at the bottom of this file before ending the turn.**

---

## 2. Technology Stack & Workspace Structure

- **Backend:** Rust Clean Architecture (`apps/api-rust/crates/`):
  - `orbis-domain`: Pure domain entities, value objects, invariants.
  - `orbis-app`: Use cases, inbound/outbound ports, security services.
  - `orbis-infra`: Concrete repositories via SeaORM (SQLite for desktop; PostgreSQL for cloud).
  - `orbis-server`: Axum 0.8 REST API exposing canonical OpenAPI 3.1.
  - `docforge-engine`: Standalone 18-node AST vector reporting engine with cosmic-text RTL Arabic HarfBuzz shaping.
- **Frontend / Desktop:** SvelteKit 2 + Svelte 5 Runes + Tauri 2.0 (`apps/desktop/`):
  - `apps/desktop/src-tauri`: Native desktop wrapper and IPC command handlers calling `orbis-app` use cases.
  - `packages/ui`: Shared UI components.
  - `packages/svelte-grid`: High-performance virtualized canvas/DOM hybrid grid (`OrbisGrid`).
  - `packages/sdk`: Pure TypeScript client generated deterministically via `@hey-api/openapi-ts`.

---

## 3. CONTINUATION STATE

- **Last Session Date:** 2026-09-28
- **Active Phase:** Phase 6 — Student Lifecycle, Admissions, Guardians & Enrollments Reconciliation (Completed)
- **Completed Scopes:**
  - **Organization & Settings (Phase 3):**
    - Expanded `UpdateOrgRequest` (`name_ar`, `base_currency_code`, `date_format`, `is_hijri_enabled`, `logo`, `favicon`, `code`) in `orbis-server` & `orbis-app`.
    - Added `AdmissionConfigurationDto` & `admission` to `OrganizationSettingsDto` in `orbis-server`.
    - Aligned `OrgUnitDto` with domain fields (`head_id`, `data_isolation`, `children_count`, `descendant_count`).
    - Aligned Tauri IPC commands (`org_cmds.rs`) with full serialization/deserialization.
    - Reconciled desktop settings cards (`branch-settings-card`, `notifications-manager`, `registration-settings`, `enrollment-settings-card`, `finance-settings-card`, `autopilot-settings-card`, `general-settings-form`).
  - **IAM, Users, Roles & Permissions (Phase 4):**
    - Audited full parity across REST API (`iam_routes.rs`, `group_routes.rs`), Tauri IPC (`iam_cmds.rs`, `group_cmds.rs`), and TypeScript SDK (`@orbis/sdk`).
    - Reconciled `user-profile.svelte` to map canonical `UpdateUserRequest` (`full_name_ar`, `is_active`) and query group memberships accurately.
    - Reconciled `user-sessions.svelte` to use canonical `UserDeviceDto` properties and wired `api.revokeDevice(id)` with confirmation.
    - Reconciled `user-roles.svelte` to remove unsafe `as any` casts and enforce typed role updates.
  - **Academic Structure & Calendar (Phase 5):**
    - Audited and aligned `AcademicPeriodDto` in `calendar_routes.rs` and `calendar_cmds.rs` (`code`, `is_locked`, `is_enrollment_open`, `is_grade_entry_open`, `created_at`, `updated_at`).
    - Aligned `AcademicYearDto` in `calendar_cmds.rs` with `current_period_id`, `notes`, `created_at`, `updated_at`.
    - Aligned `SubjectDto` in `academic_routes.rs` and `academic_cmds.rs` with `description`, `is_elective`, `created_at`, `updated_at`.
    - Aligned `ClassDto` in `academic_routes.rs` and `academic_cmds.rs` with `homeroom_teacher_id`, `current_students`, `room_id`, `color`, `created_at`, `updated_at`.
    - Aligned `RoomDto` in `scheduling_cmds.rs` with `org_unit_id`, `created_at`, `updated_at`.
    - Reconciled `period-form.svelte` to cleanly separate `AcademicYear` vs `AcademicPeriod` creation with full date ISO conversions.
    - Reconciled `subject-form.svelte` to bind `credit_hours` directly and eliminate untyped parameters.
    - Reconciled `room-form.svelte` to sanitize payloads for `createRoom`.
    - Reconciled `class-form.svelte` to support explicit OrgUnit and Period selection.
    - Reconciled `subjects-manager.svelte` and `rooms-manager.svelte` with `formatDate` and typed DTOs.
  - **Student Lifecycle, Admissions, Guardians & Enrollments (Phase 6):**
    - Aligned `RegisterStudentRequest` in `student_routes.rs` with complete demographics, contact, Arabic naming, initial period/class, and study mode fields.
    - Added `UpdateStudentProfileRequest` DTO and endpoint (`PATCH /api/v1/students/:id/profile`) in `student_routes.rs`.
    - Added `TransferClassRequest` and endpoint (`POST /api/v1/enrollments/:id/transfer-class`) in `student_routes.rs`.
    - Added `BulkRegisterStudentsRequest` and bulk register endpoint (`POST /api/v1/students/bulk-register`) in `student_routes.rs`.
    - Added soft delete (`DELETE /api/v1/students/:id`), restore (`POST /api/v1/students/:id/restore`), and remove guardian (`DELETE /api/v1/students/:student_id/guardians/:guardian_id`) endpoints in `student_routes.rs`.
    - Enhanced `StudentService::assign_guardian_full` in `orbis-app` to persist all guardian fields (`relationship_ar`, `is_emergency_contact`, `is_authorized_pickup`, `is_payer`, `payment_share_percent`, `occupation`, `employer`, `work_phone`, `notes`).
    - Aligned `student_cmds.rs` in Tauri with `assign_guardian_full`.
    - Exported updated OpenAPI specification to `packages/sdk` and regenerated TypeScript client.
    - Reconciled `remote-client.ts` and `local-client.ts` with typed contracts.
    - Reconciled `student-details.svelte` to use valid backend status transitions.
  - **Verification:**
    - `apps/api-rust` cargo check: 0 errors
    - `apps/desktop/src-tauri` cargo check: 0 errors
    - `packages/sdk` tsc: 0 errors
    - `apps/desktop` tsc: 0 errors
- **Next Scope in Master Execution Plan:** Phase 7 — Employee Affairs, HR & Teacher Assignments Reconciliation.

