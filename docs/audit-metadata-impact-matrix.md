# Orbis — Audit Metadata Impact Matrix

> **Version:** 1.0.0
> **Companion:** `audit-metadata-architecture.md`, `audit-metadata-implementation-plan.md`

---

## 1. Full-Screen Impact Analysis

For every backend change, this matrix maps the full impact across all system layers.

### Legend

| Symbol | Meaning |
|---|---|
| ✅ | Change required and specified |
| ❌ | Not affected / No change needed |
| ⚠️ | Review needed during implementation |
| N/A | Not applicable |

---

## 2. Entity-Level Impact Matrix

### 2.1 Student Entity

**Change:** Add `created_by: Option<Uuid>`, `updated_by: Option<Uuid>`, `org_unit_id: Option<Uuid>`

| Layer | Component | Impact | Details |
|---|---|---|---|
| **Database** | `students` table | ✅ | Add 3 nullable UUID columns |
| **Domain** | `student.rs` | ✅ | Add 3 fields to `Student` struct |
| **ORM Model** | `models/student.rs` | ✅ | Add 3 columns to SeaORM model |
| **Repository** | `student_repos.rs` | ✅ | Map new fields in save() and find_*() |
| **Service** | `student_service.rs` | ✅ | Set `created_by`/`updated_by` from ctx |
| **API DTO** | `student_routes.rs` | ✅ | Add to `StudentDto` response |
| **API Request** | `CreateStudentRequest` | ⚠️ | Add `org_unit_id` (optional). Do NOT add `created_by` |
| **SDK Types** | `@orbis/sdk` | ✅ | Regenerate from OpenAPI |
| **Client Types** | `types.ts` | ✅ | Add to `StudentDto` |
| **LocalClient** | `local-client.ts` | ✅ | Map new response fields |
| **RemoteClient** | `remote-client.ts` | ✅ | Map new response fields |
| **Tauri** | `state.rs` | ⚠️ | Verify student commands pass new fields |
| **Grid** | Student list | ✅ | Add optional columns: Created By, Updated By, Branch |
| **Form** | Student create/edit | ⚠️ | Add org_unit selector if branch assignment is required |
| **Detail** | Student detail | ✅ | Show created_by, updated_by in metadata section |
| **Search** | Student search | ⚠️ | Add org_unit_id filter support |
| **Filter** | Student filters | ✅ | Add Branch filter pill |
| **Context Menu** | Student actions | ✅ | Add "View Audit History" item |
| **Reports** | Student report | ✅ | Include Branch column |
| **Export** | Student CSV/Excel | ✅ | Include Created By, Branch |
| **Print** | Student profile | ✅ | Include metadata |
| **Permissions** | — | ❌ | No new permissions needed |
| **Localization** | Labels | ✅ | Paraglide messages for new column headers |
| **RTL** | Layout | ⚠️ | Verify new columns render correctly in RTL |

---

### 2.2 Employee Entity

**Change:** Add `created_by: Option<Uuid>`, `updated_by: Option<Uuid>`, `deleted_at: Option<DateTime>`

| Layer | Component | Impact | Details |
|---|---|---|---|
| **Database** | `employees` table | ✅ | Add 3 columns |
| **Domain** | `employee.rs` | ✅ | Add 3 fields |
| **ORM Model** | `models/employee.rs` | ✅ | Add 3 columns |
| **Repository** | `employee_repo.rs` | ✅ | Map in save/find |
| **Service** | `employee_service.rs` | ✅ | Set from ctx; set deleted_at on soft delete |
| **API DTO** | `employee_routes.rs` | ✅ | Add to response DTO |
| **API Request** | — | ❌ | No change to create/update requests |
| **Client Types** | `types.ts` | ✅ | Add to EmployeeDto |
| **LocalClient** | `local-client.ts` | ✅ | Map response |
| **RemoteClient** | `remote-client.ts` | ✅ | Map response |
| **Grid** | Employee directory | ✅ | Add optional columns |
| **Form** | Employee create/edit | ❌ | No new form inputs |
| **Detail** | EmployeeSheet | ✅ | Show metadata in preview mode |
| **Context Menu** | — | ✅ | Add "View Audit History" |
| **Reports** | Employee report | ✅ | Include Created By |
| **Export** | Employee CSV | ✅ | Include metadata columns |
| **Localization** | — | ✅ | New labels |

---

### 2.3 JournalEntry Entity

**Change:** Add `updated_at`, `updated_by`, `org_unit_id`, `version`

| Layer | Component | Impact | Details |
|---|---|---|---|
| **Database** | `journal_entries` table | ✅ | Add 4 columns |
| **Domain** | `journal_entry.rs` | ✅ | Add 4 fields |
| **ORM Model** | `models/journal_entry.rs` | ✅ | Add 4 columns |
| **Repository** | `finance_repos.rs` | ✅ | Map in save/find |
| **Service** | `finance_service.rs` | ✅ | Set updated_by/at on post/reverse; version check on update |
| **API DTO** | `finance_routes.rs` | ✅ | Add to `JournalEntryDto` response |
| **API Request** | Update requests | ✅ | Add `expected_version` for optimistic locking |
| **Client Types** | `types.ts` | ✅ | Add to JournalEntryDto |
| **LocalClient** | — | ✅ | Map response |
| **RemoteClient** | — | ✅ | Map response |
| **Grid** | Journal entries list | ✅ | Add optional columns: Updated By, Branch, Version |
| **Form** | Journal entry form | ⚠️ | Include org_unit selector; pass version on submit |
| **Detail** | — | ✅ | Show metadata + version |
| **Context Menu** | — | ✅ | Add "View Audit History" |
| **Reports** | Financial reports | ✅ | Include Branch, Updated By |
| **Export** | — | ✅ | Include metadata |
| **Permissions** | — | ❌ | Existing FinanceRead/Write sufficient |
| **Concurrency** | — | ✅ | Backend rejects if version mismatch |

---

### 2.4 Account Entity

**Change:** Add `created_by`, `updated_by`, `version`

| Layer | Component | Impact | Details |
|---|---|---|---|
| **Database** | `accounts` table | ✅ | Add 3 columns |
| **Domain** | `account.rs` | ✅ | Add 3 fields |
| **ORM Model** | `models/account.rs` | ✅ | Add 3 columns |
| **Repository** | `finance_repos.rs` | ✅ | Map in save/find |
| **Service** | `finance_service.rs` | ✅ | Version check on chart-of-accounts updates |
| **API DTO** | `finance_routes.rs` | ✅ | Add to AccountDto |
| **Client Types** | — | ✅ | Update |
| **Grid** | Chart of accounts | ✅ | Optional columns |
| **Form** | Account edit | ⚠️ | Pass version for concurrency |

---

### 2.5 Grade Entity

**Change:** Add `created_by`, `updated_by`

| Layer | Component | Impact | Details |
|---|---|---|---|
| **Database** | `grades` table | ✅ | Add 2 columns |
| **Domain** | `grade.rs` | ✅ | Add 2 fields |
| **ORM Model** | `models/grade.rs` | ✅ | Add 2 columns |
| **Repository** | `assessment_repos.rs` | ✅ | Map in save/find |
| **Service** | `assessment_service.rs` | ✅ | Set from ctx on record/update |
| **API DTO** | `assessment_routes.rs` | ✅ | Add to GradeDto |
| **Client Types** | — | ✅ | Update |
| **Grid** | Grade entry | ⚠️ | Show who entered grades in teacher view |
| **Reports** | Grade reports | ⚠️ | Include "Entered By" if requested |

---

### 2.6 Enrollment Entity

**Change:** Add `created_by`, `updated_by`

| Layer | Component | Impact | Details |
|---|---|---|---|
| **Database** | `enrollments` table | ✅ | Add 2 columns |
| **Domain** | `enrollment.rs` | ✅ | Add 2 fields |
| **Repository** | `student_repos.rs` | ✅ | Map |
| **Service** | `student_service.rs` | ✅ | Set from ctx |
| **API DTO** | `student_routes.rs` | ✅ | Add to EnrollmentDto |
| **Grid** | Enrollment list | ❌ | Not typically shown in grid |
| **Reports** | Enrollment report | ⚠️ | Include if needed |

---

### 2.7 Group Entity

**Change:** Add `created_by`, `updated_by`

| Layer | Component | Impact | Details |
|---|---|---|---|
| **Database** | `groups` table | ✅ | Add 2 columns |
| **Domain** | `group.rs` | ✅ | Add to `Group` struct |
| **Repository** | `group_repo.rs` | ✅ | Map |
| **Service** | `group_service.rs` | ✅ | Set from ctx |
| **API DTO** | `group_routes.rs` | ✅ | Add to GroupDto |
| **Grid** | Groups list | ✅ | Optional columns |
| **Context Menu** | — | ✅ | Add "View Audit History" |

---

### 2.8 Organization Entity

**Change:** Add `created_by`, `updated_by`, `version`

| Layer | Component | Impact | Details |
|---|---|---|---|
| **Database** | `organizations` table | ✅ | Add 3 columns |
| **Domain** | `organization.rs` | ✅ | Add 3 fields; update `Organization::new()` |
| **Repository** | `org_repo.rs` | ✅ | Map |
| **Service** | `organization_service.rs` | ✅ | Set from ctx; version check on update |
| **API DTO** | `org_routes.rs` | ✅ | Add to OrgDto |
| **Settings UI** | Settings page | ⚠️ | Pass version for concurrent settings changes |

---

### 2.9 OrgUnit Entity

**Change:** Add `created_by`, `updated_by`

| Layer | Component | Impact | Details |
|---|---|---|---|
| **Database** | `org_units` table | ✅ | Add 2 columns |
| **Domain** | `org_unit.rs` | ✅ | Add 2 fields |
| **Repository** | `org_unit_repo.rs` | ✅ | Map |
| **Service** | `organization_service.rs` | ✅ | Set from ctx |
| **API DTO** | `org_routes.rs` | ✅ | Add to OrgUnitDto |

---

### 2.10 User Entity

**Change:** Add `created_by`, `updated_by`

| Layer | Component | Impact | Details |
|---|---|---|---|
| **Database** | `users` table | ✅ | Add 2 columns |
| **Domain** | `user.rs` | ✅ | Add 2 fields |
| **Repository** | `user_repo.rs` | ✅ | Map |
| **Service** | `auth_service.rs`, `identity_provisioning_service.rs` | ✅ | Set from ctx |
| **API DTO** | `iam_routes.rs` | ✅ | Add to UserDto |

---

### 2.11 Class Entity

**Change:** Add `org_unit_id: Option<Uuid>`

| Layer | Component | Impact | Details |
|---|---|---|---|
| **Database** | `classes` table | ✅ | Add 1 column |
| **Domain** | `class.rs` | ✅ | Add 1 field |
| **Repository** | `academic_repos.rs` | ✅ | Map; add org_unit filter |
| **Service** | `academic_service.rs` | ✅ | Accept org_unit_id on create |
| **API DTO** | `academic_routes.rs` | ✅ | Add to ClassDto and CreateClassRequest |
| **Grid** | Class list | ✅ | Add Branch column |
| **Form** | Class create | ✅ | Add org_unit selector |
| **Filter** | Class filters | ✅ | Add Branch filter |

---

### 2.12 Payment Entity

**Change:** Add `updated_at`, `org_unit_id`

| Layer | Component | Impact | Details |
|---|---|---|---|
| **Database** | `payments` table | ✅ | Add 2 columns |
| **Domain** | `payment.rs` | ✅ | Add 2 fields |
| **Repository** | `finance_repos.rs` | ✅ | Map |
| **Service** | `finance_service.rs` | ✅ | Set on create |
| **API DTO** | `finance_routes.rs` | ✅ | Add to PaymentDto |
| **Grid** | Payments list | ✅ | Add Branch column |
| **Reports** | Payment receipts | ✅ | Include Branch |

---

### 2.13 Room Entity

**Change:** Add `org_unit_id: Option<Uuid>`

| Layer | Component | Impact | Details |
|---|---|---|---|
| **Database** | `rooms` table | ✅ | Add 1 column |
| **Domain** | `room.rs` | ✅ | Add 1 field |
| **Repository** | `scheduling_repos.rs` | ✅ | Map |
| **Service** | `scheduling_service.rs` | ✅ | Accept org_unit_id on create |
| **API DTO** | `scheduling_routes.rs` | ✅ | Add to RoomDto |

---

### 2.14 StudentFee Entity

**Change:** Add `created_by: Option<Uuid>`

| Layer | Component | Impact | Details |
|---|---|---|---|
| **Database** | `student_fees` table | ✅ | Add 1 column |
| **Domain** | `student_fee.rs` | ✅ | Add 1 field |
| **Repository** | `finance_repos.rs` | ✅ | Map |
| **Service** | `finance_service.rs` | ✅ | Set from ctx |
| **API DTO** | `finance_routes.rs` | ✅ | Add to StudentFeeDto |

---

### 2.15 Attendance Entity

**Change:** Add `marked_by: Option<Uuid>`

| Layer | Component | Impact | Details |
|---|---|---|---|
| **Database** | `attendances` table | ✅ | Add 1 column |
| **Domain** | `attendance.rs` | ✅ | Add 1 field |
| **Repository** | `assessment_repos.rs` | ✅ | Map |
| **Service** | `assessment_service.rs` | ✅ | Set from ctx |
| **API DTO** | `assessment_routes.rs` | ✅ | Add to AttendanceDto |
| **Grid** | Attendance list | ⚠️ | Show who marked attendance |

---

### 2.16 BehaviorRecord Entity

**Change:** Add `recorded_by: Option<Uuid>`

| Layer | Component | Impact | Details |
|---|---|---|---|
| **Database** | `behavior_records` table | ✅ | Add 1 column |
| **Domain** | `behavior_record.rs` | ✅ | Add 1 field |
| **Repository** | `assessment_repos.rs` | ✅ | Map |
| **Service** | `assessment_service.rs` | ✅ | Set from ctx |
| **API DTO** | `assessment_routes.rs` | ✅ | Add to BehaviorRecordDto |

---

## 3. Cross-Cutting Impact: Audit Trail

**Change:** New `audit_events` table + service + API + UI

| Layer | Component | Impact | Details |
|---|---|---|---|
| **Database** | New table | ✅ | `audit_events` with indexes |
| **Domain** | New entity | ✅ | `AuditEvent` struct |
| **ORM Model** | New model | ✅ | `audit_event.rs` |
| **Repository** | New port + impl | ✅ | `AuditEventRepository` |
| **Service** | New service | ✅ | `AuditService` with `log()` |
| **EventBus** | Modified | ✅ | Write audit event on DomainEvent publish |
| **API** | New routes | ✅ | `GET /audit/events`, `GET /audit/events/entity/:type/:id` |
| **Permissions** | Existing | ❌ | `AuditRead` already exists |
| **Client Types** | New types | ✅ | `AuditEventDto`, `AuditEventFilter` |
| **LocalClient** | New methods | ✅ | `listAuditEvents()`, `getEntityAudit()` |
| **RemoteClient** | New methods | ✅ | Same interface |
| **Tauri** | New commands | ✅ | `list_audit_events`, `get_entity_audit` |
| **Desktop UI** | New component | ✅ | `<AuditTimeline>` component |
| **Context Menu** | All entity grids | ✅ | "View Audit History" item |
| **Localization** | — | ✅ | Labels for audit actions, columns, filters |

---

## 4. Cross-Cutting Impact: RequestContext Enhancement

**Change:** Add `source: ActionSource`, `org_unit_path: Option<String>`

| Layer | Component | Impact | Details |
|---|---|---|---|
| **Domain** | `context.rs` | ✅ | Add 2 fields + ActionSource enum |
| **Extractors** | `extractors.rs` | ✅ | Set `source: Api` and resolve `org_unit_path` |
| **Tauri State** | `state.rs` | ✅ | Set `source: TauriIpc` |
| **Autopilot** | `autopilot_service.rs` | ✅ | Set `source: Autopilot` |
| **Bootstrap** | `bootstrap_service.rs` | ✅ | Set `source: System` |
| **Integration** | `integration_service.rs` | ✅ | Set `source: Integration(name)` |

---

## 5. Summary: Total File Changes

### Backend (Rust)

| Category | Files Modified | Files Created |
|---|---|---|
| Domain entities | 16 | 1 (audit.rs) |
| Application context | 1 | 0 |
| Application services | 10 | 1 (audit_service.rs) |
| Application ports | 1 | 0 |
| Infrastructure models | 16 | 1 (audit_event.rs) |
| Infrastructure repos | 12 | 1 (audit_event_repo.rs) |
| Infrastructure event_bus | 1 | 0 |
| Infrastructure migration | 0 | 1 |
| Server API routes | 8 | 1 (audit_routes.rs) |
| Server state | 1 | 0 |
| Server extractors | 1 | 0 |
| Domain events | 1 | 0 |
| **Total** | **~68** | **~6** |

### Frontend (TypeScript/Svelte)

| Category | Files Modified | Files Created |
|---|---|---|
| API types | 1 | 0 |
| LocalClient | 1 | 0 |
| RemoteClient | 1 | 0 |
| Grid columns (per feature) | ~6 | 0 |
| Detail panels | ~5 | 0 |
| Context menus | ~5 | 0 |
| Shared components | 0 | 1 (AuditTimeline) |
| Shared components | 0 | 1 (ActorBadge) |
| Localization | 1 | 0 |
| **Total** | **~20** | **~2** |

### Tauri (Rust)

| Category | Files Modified | Files Created |
|---|---|---|
| State | 1 | 0 |
| Commands | 0 | 1 (audit commands) |
| **Total** | **1** | **1** |
