# Orbis — Students Backend & Desktop Reconciliation Report

> **Document Version:** 1.0.0-PROD  
> **Status:** `STUDENTS_RECONCILIATION_COMPLETE`  
> **Target Subsystem:** Student Lifecycle, Directory, Registration, Detail Workspace & Sub-features  
> **Workspace Root:** `d:/projects/Orbis`  

---

## Executive Summary

The Students vertical across the Orbis educational ERP was fully audited, reconciled, and rebuilt against the authoritative Rust Clean Architecture backend (`orbis-domain`, `orbis-app`, `orbis-infra`, `orbis-server`, and Tauri 2.0 IPC commands).

All legacy field assumptions, outdated DTOs, fake frontend calculations, dead action handlers, and unsourced mutations were removed and replaced with canonical, type-safe backend domain operations.

---

## 1. Backend Contract Inventory & Verification

The authoritative backend domain model separates student lifecycle concerns cleanly:
- **Person (`PersonDto`)**: Source of truth for human identity and demographics (`first_name`, `last_name`, `first_name_ar`, `middle_name_ar`, `last_name_ar`, `gender`, `birth_date`, `place_of_birth`, `nationality`, `blood_type`, `medical_notes`, `phone`, `email`, `address`, `avatar_url`).
- **Student (`StudentDto`)**: Source of truth for educational state (`student_number`, `status`, `standing`, `study_mode`, `admitted_at`, `graduated_at`).
- **StudentDetail (`StudentDetailDto`)**: Canonical composite structure `{ student: StudentDto, person: PersonDto, enrollment?: EnrollmentDto }`.
- **Enrollment (`EnrollmentDto`)**: Connects student to academic period and class (`period_id`, `class_id`, `status`, `is_attendance_exempt`, `is_grade_exempt`).
- **StudentGuardian (`GuardianDto`)**: Links student to parent user with permissions (`is_primary`, `is_emergency_contact`, `is_payer`, `payment_share_percent`).
- **StudentFee (`StudentFeeDto`)**: Authoritative tuition fee ledger containing precalculated `original_amount`, `discount_amount`, `net_amount`, `paid_amount`, and `balance_due`.
- **Attendance (`AttendanceDto`)**: Attendance logs per student per period.
- **BehaviorRecord (`BehaviorRecordDto`)**: Disciplinary and merit records with point weights.
- **Document / Fulfillment (`DocumentDto`, `StudentDocumentFulfillmentDto`)**: Requirement evaluation and concrete file metadata.

---

## 2. API / SDK / Local / Remote Client Parity

### Tauri IPC Commands Added & Registered:
- `student_remove_guardian`: Calls `StudentService::remove_guardian(tenant_id, student_id, guardian_id)`.
- `student_delete_student`: Soft-deletes student record via `StudentService::delete_student`.
- `student_restore_student`: Restores deleted student via `StudentService::restore_student`.
- `assessment_list_student_attendance`: Calls `AttendanceRepository::list` scoped to tenant and student.
- `document_list_by_entity`: Calls `DocumentService::list_documents_by_entity` (e.g. `entity_type="student"`).

### Local Client (`local-client.ts`) & Remote Client (`remote-client.ts`):
Both clients now expose identical, fully typed methods for:
- `listStudents(filter?)`
- `getStudent(id)`
- `registerStudent(req)`
- `updateStudentStatus(req)`
- `updateStudentProfile(req)`
- `deleteStudent(id)`
- `restoreStudent(id)`
- `listGuardians(studentId)`
- `assignGuardian(req)`
- `removeGuardian(studentId, guardianId)`
- `listStudentAttendance(studentId)`
- `listStudentBehaviorRecords(studentId)`
- `recordBehavior(req)`
- `listDocumentsByEntity(entityType, entityId)`
- `evaluateStudentDocumentRequirements(studentId, options)`
- `listStudentFees({ studentId, enrollmentId })`
- `listPayments({ studentId })`

---

## 3. Desktop Subsystem Rebuild & Reconciliation

### A. Student Directory & Table (`student-directory.svelte`, `student-table.svelte`, `store.svelte.ts`)
- Switched to `createStudentsQuery` returning `StudentDetailDto[]`.
- Directory table renders student identity from `person` and lifecycle from `student` and class placement from `enrollment`.
- Dynamic server/client filtering and search support Arabic text normalization (`normalizeArabic`), Indic digits normalization, and multi-field matching without data corruption.
- Zero `as any` casts.

### B. Student Registration & Form State (`student-form.svelte`, `form-state.svelte.ts`, `state/*`)
- Atomic student registration calling `api.registerStudent` with full Arabic and English name parts, demographics, medical notes, study mode, and initial class placement.
- Edit mode uses `api.getStudent` and updates demographics via `api.updateStudentProfile` and status via `api.updateStudentStatus`.
- Live debounced duplicate checking for National ID, phone, and email calling backend `api.checkDuplicate`.

### C. Student Detail Workspace (`[studentId]/+layout.svelte`, `student-details.svelte`)
- Uses `createStudentQuery(studentId)` supplying full `StudentDetailDto`.
- Sidebar presents avatar, academic number, contact details, sex, birth date, status, and historical enrollment records.
- Class transfer action uses `api.transferClass`.
- Withdrawal action uses `api.updateStudentStatus`.
- Profile form updates `Person` entity and `Student` status with immediate query invalidation.

### D. Sub-feature Tabs
1. **Guardians (`student-guardians.svelte`, `guardian-form.svelte`)**:
   - Queries `api.listGuardians`.
   - Real deletion via `api.removeGuardian`.
   - Creation / update via `AssignGuardianRequest`.
2. **Grades (`student-grades.svelte`)**:
   - Calculates GPA directly with student ID and active period.
3. **Fees (`student-fees.svelte`)**:
   - Consumes backend `StudentFeeDto` authoritative calculated balances (`net_amount`, `paid_amount`, `balance_due`).
   - Removed duplicate client-side math.
4. **Attendance (`student-attendance.svelte`)**:
   - Queries `api.listStudentAttendance(studentId)`.
   - Renders statistics (present days, absent days, late cases, attendance rate).
5. **Behavior (`student-behavior.svelte`)**:
   - Queries `api.listStudentBehaviorRecords(studentId)`.
   - Adds incidents via `api.recordBehavior`.
6. **Documents (`student-documents.svelte`)**:
   - Queries `api.listDocumentsByEntity("student", studentId)`.
   - Uploads attachments via `api.uploadDocument`.
   - Deletes documents via `api.deleteDocument`.

---

## 4. Verification Results

| Check | Target | Result |
| :--- | :--- | :--- |
| **TypeScript Typecheck** | `apps/desktop` (`bun x tsc --noEmit`) | **0 Errors** |
| **Desktop Rust Check** | `apps/desktop/src-tauri` (`cargo check`) | **0 Errors** |
| **Backend API Check** | `apps/api-rust` (`cargo check --workspace`) | **0 Errors** |

---

## Final Status

**STUDENTS_RECONCILIATION_COMPLETE**
