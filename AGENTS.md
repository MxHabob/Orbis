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
- **Active Phase:** Phase 8 — Scheduling, Timetabling & Lesson Slots Reconciliation (Completed)
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
    - Aligned `AcademicYearDto` in `calendar_cmds.rs` with `current_period_id`, `notes`, `created_at`, `updated_at`).
    - Aligned `SubjectDto` in `academic_routes.rs` and `academic_cmds.rs` with `description`, `is_elective`, `created_at`, `updated_at`).
    - Aligned `ClassDto` in `academic_routes.rs` and `academic_cmds.rs` with `homeroom_teacher_id`, `current_students`, `room_id`, `color`, `created_at`, `updated_at`).
    - Aligned `RoomDto` in `scheduling_cmds.rs` with `org_unit_id`, `created_at`, `updated_at`).
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
  - **Employee Affairs, HR & Teacher Assignments (Phase 7):**
    - Added `delete_employee` (soft delete) and `restore_employee` methods to `EmployeeService` in `orbis-app`.
    - Added `EmployeeDeleted` and `EmployeeRestored` domain events to `DomainEvent` enum and audit trails in `orbis-domain`.
    - Added `BulkRegisterEmployeesRequest`, `BulkRegisterEmployeeItemRequest`, `BulkRegisterEmployeesResponseDto`, and `BulkRegisterEmployeeItemResultDto` DTOs to `employee_routes.rs`.
    - Added `POST /api/v1/employees/bulk-register`, `DELETE /api/v1/employees/:id`, and `POST /api/v1/employees/:id/restore` endpoints to `employee_routes.rs` and registered in `ApiDoc` in `orbis-server`.
    - Added `employee_delete_employee` and `employee_restore_employee` Tauri IPC commands to `employee_cmds.rs` and registered in `src-tauri/src/commands/mod.rs`.
    - Exported updated OpenAPI specification to `packages/sdk` and regenerated typed `@orbis/sdk`.
    - Reconciled `local-client.ts` and `remote-client.ts` to implement `deleteEmployee`, `restoreEmployee`, and `bulkRegisterEmployees` with SDK calls.
    - Reconciled `queries.ts` in desktop with `createDeleteEmployeeMutation`, `createRestoreEmployeeMutation`, and `createBulkRegisterEmployeesMutation`.
    - Reconciled `employee-directory.svelte` (added actions for view dossier, edit details, soft delete with `confirmStore`, and removed invalid route fallbacks) and `employee-form.svelte`.
  - **Scheduling, Timetabling & Lesson Slots (Phase 8):**
    - Added `RoomUpdated`, `RoomDeleted`, `TimeSlotUpdated`, `TimeSlotDeleted`, and `LessonDeleted` domain events and audit trail mappings in `orbis-domain`.
    - Added `delete`, `delete_by_period`, `delete_by_class`, and `save_bulk` methods to `RoomRepository`, `TimeSlotRepository`, and `ScheduledLessonRepository` in `orbis-app` and implemented in `orbis-infra`.
    - Added `update_room`, `delete_room`, `update_time_slot`, `delete_time_slot`, `batch_create_time_slots`, `delete_scheduled_lesson`, and `bulk_schedule_lessons` in `SchedulingService`.
    - Added `UpdateRoomRequest`, `CreateTimeSlotItemRequest`, `BatchCreateTimeSlotsRequest`, `UpdateTimeSlotRequest`, `BulkScheduleLessonsRequest`, and corresponding REST routes (`PATCH/DELETE /api/v1/rooms/:id`, `POST /api/v1/time-slots/batch`, `GET/PATCH/DELETE /api/v1/time-slots/:id`, `POST /api/v1/scheduled-lessons/bulk`, `DELETE /api/v1/scheduled-lessons/:id`) in `orbis-server` registered in `ApiDoc`.
    - Added `scheduling_update_room`, `scheduling_delete_room`, `scheduling_batch_create_time_slots`, `scheduling_get_time_slot`, `scheduling_update_time_slot`, `scheduling_delete_time_slot`, `scheduling_bulk_schedule_lessons`, and `scheduling_delete_scheduled_lesson` Tauri IPC commands in `scheduling_cmds.rs` registered in `commands/mod.rs`.
    - Exported updated OpenAPI specification to `packages/sdk` and regenerated `@orbis/sdk`.
    - Reconciled `local-client.ts` and `remote-client.ts` with all new scheduling methods.
    - Reconciled `time-slot-sheet.svelte` (typed update and delete), `room-form.svelte` (update vs create), `rooms-manager.svelte` (row actions with `confirmStore`), `schedule-workspace.svelte` (real deletion and move handling), and `lesson-form.svelte`.
  - **Assessment, Grading, Conduct & Attendance Tracking (Phase 9):**
    - Added `AssessmentSchemeCreated`, `AssessmentSchemeUpdated`, `AssessmentSchemeDeleted`, `GradeDeleted`, `AttendanceDeleted`, and `BehaviorDeleted` domain events to `DomainEvent` enum and audit trails in `orbis-domain`.
    - Added `delete` to `GradeRepository`, `save_batch` and `delete` to `AttendanceRepository`, `find_by_org` and `delete` to `BehaviorRecordRepository` in `orbis-app` and implemented in `orbis-infra`.
    - Added `BatchGradeItem`, `batch_record_grades`, `delete_grade`, `BatchAttendanceItem`, `batch_mark_attendance`, `delete_attendance`, `list_student_attendance`, `list_behavior_records`, `delete_behavior` in `AssessmentService`.
    - Reconciled REST routes (`POST /api/v1/assessments/grades/batch`, `DELETE /api/v1/assessments/grades/:id`, `POST /api/v1/assessments/attendance/batch`, `DELETE /api/v1/assessments/attendance/:id`, `GET /api/v1/assessments/attendance/students/:student_id`, `GET /api/v1/assessments/behavior`, `DELETE /api/v1/assessments/behavior/:id`, `POST /api/v1/assessments/gpa`) and registered in `ApiDoc` in `orbis-server`.
    - Added all corresponding Tauri IPC commands in `assessment_cmds.rs` and registered in `commands/mod.rs`.
    - Exported updated OpenAPI specification to `packages/sdk` and regenerated `@orbis/sdk`.
    - Reconciled `local-client.ts`, `remote-client.ts`, and `types.ts` with complete dual-transport parity and typed SDK calls (removed fake stubs in `RemoteClient`).
    - Reconciled `attendance-tracker.svelte` (`batchMarkAttendance`), `grade-profile.ts` (`batchRecordGrades`), `student-grades.svelte` (typed DTOs & GPA calculation), `student-attendance.svelte`, `student-behavior.svelte`, and `assessment-scheme-sheet.svelte`.
  - **Verification:**
    - `apps/api-rust` cargo check: 0 errors
    - `apps/desktop/src-tauri` cargo check: 0 errors
    - `packages/sdk` tsc: 0 errors
    - `apps/desktop` tsc: 0 errors
- **Next Scope in Master Execution Plan:** Phase 10 — Examination Operations, Seating & Exam Sessions Reconciliation.


