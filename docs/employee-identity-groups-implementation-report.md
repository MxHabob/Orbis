# Enterprise Employee Affairs, User Groups & Unified Identity Provisioning
## Production Architecture & Implementation Report

> **Document Status:** Complete & Verified  
> **System Scope:** Rust Clean Architecture (`orbis-domain`, `orbis-infra`, `orbis-app`, `orbis-server`, `src-tauri`) + SvelteKit 2 / Svelte 5 Runes Frontend + Dual-Transport Parity + Paraglide i18n  
> **Date:** September 2026  

---

## 1. Executive Summary & Core Architectural Invariants

This subsystem establishes the official **Employee Affairs (شؤون الموظفين)**, **User Groups (مجموعات المستخدمين)**, and **Unified Identity Provisioning Architecture** in Orbis, adhering strictly to the fundamental identity rules:

$$\text{Employee} \neq \text{User}, \quad \text{Person} \neq \text{User}, \quad \text{Student} \neq \text{User}$$

### Core Architectural Invariants Implemented:
1. **Actor vs. User Decoupling:**
   - Real human individuals are modeled as `Person` entities (storing national IDs, full bilingual names, demographics, contact info).
   - Domain actors (`Employee`, `Student`, `Guardian`) link to `person_id` and contain actor-specific employment/academic lifecycle metadata.
   - User identity (`User`) is an optional, decoupled credential container attached to an actor.
2. **Reuse Existing Person Records:**
   - Registration and updates search for existing `Person` records by National ID, Email, or Phone to prevent duplicate human profiles across the system.
3. **No Isolated "Create User" Screen:**
   - User account provisioning is natively integrated into actor workflows (Employee creation/editing, Student onboarding, Guardian invite).
4. **CSPRNG In-Memory Temporary Passwords:**
   - Cryptographically secure temporary passwords generated via CSPRNG.
   - Hashed using Argon2 before database persistence.
   - Passwords are **never** logged, never stored plaintext in persistent storage, and presented exclusively via a one-time credential delivery modal (`CredentialDeliveryModal`) or sent via configured external notification channels.
5. **Effective Permission Computation:**
   - `DbPermissionChecker` computes effective permissions across direct roles + all assigned user groups dynamically.
6. **Dual-Transport Parity:**
   - 100% feature and type parity between `LocalClient` (Tauri IPC commands for desktop SQLite) and `RemoteClient` (HTTP REST Axum server for cloud PostgreSQL).
   - Zero `as any` type bypasses.

---

## 2. Architecture & Layer Implementation

```mermaid
graph TD
    subgraph UI Layer [SvelteKit 2 + Svelte 5 Runes]
        ED[EmployeeDirectory]
        EF[EmployeeForm]
        EP[EmployeeProfile 360]
        GM[GroupsManager]
        GD[GroupDetailView + 2D Matrix]
        CDM[CredentialDeliveryModal]
        BAD[BulkAssignDialog]
    end

    subgraph Client Layer [Dual-Transport Parity]
        LC[LocalClient - Tauri IPC]
        RC[RemoteClient - HTTP REST]
        QK[TanStack Query Factories]
    end

    subgraph Backend Core [Rust Clean Architecture]
        subgraph Server & IPC
            AX[Axum REST API Router]
            TC[Tauri Commands]
        end
        subgraph Application Services
            IPS[IdentityProvisioningService]
            ES[EmployeeService]
            GS[GroupService]
            PC[DbPermissionChecker]
        end
        subgraph Repositories & Persistence
            ER[EmployeeRepo - SeaORM]
            GR[GroupRepo - SeaORM]
            PR[PersonRepo - SeaORM]
        end
        subgraph Domain Entities
            DE[Employee & EmployeeDetail]
            DG[Group & GroupDetail]
            DIP[IdentityProvisioning Types]
            DP[IAM Permissions Map]
        end
    end

    UI Layer --> Client Layer
    LC --> TC
    RC --> AX
    TC --> Backend Core
    AX --> Backend Core
```

---

## 3. Rust Backend Components

### 3.1 Domain Layer (`apps/api-rust/crates/orbis-domain/`)
- `src/entities/employee.rs`: Defines `Employee`, `EmployeeDetail`, `EmployeeStatus`, `EmploymentType`, `EmployeeFilter`.
- `src/entities/group.rs`: Defines `Group`, `GroupDetail`, `GroupMemberInfo`, `GroupPermission`, `UserGroupMembership`.
- `src/entities/identity_provisioning.rs`: Defines `ActorType`, `DeliveryChannel`, `AccountProvisioningConfig`, `AccountProvisioningResult`, `ResetPasswordResult`.
- `src/iam/permission.rs`: Extended with granular permissions:
  - `employee:read`, `employee:write`, `employee:create`, `employee:update`, `employee:delete`
  - `group:read`, `group:write`, `group:delete`, `group:assign`

### 3.2 Infrastructure & Database Layer (`apps/api-rust/crates/orbis-infra/`)
- SeaORM Models:
  - `models/employee.rs` (Table `employees`)
  - `models/group.rs` (Table `groups`)
  - `models/user_group_membership.rs` (Table `user_group_memberships`)
  - `models/group_permission.rs` (Table `group_permissions`)
- Database Migration:
  - `m20240104_000001_employee_groups_schema.rs` creating tables, foreign keys, and indexes.
- Repositories:
  - `repositories/employee_repo.rs` (`EmployeeRepoImpl`)
  - `repositories/group_repo.rs` (`GroupRepoImpl`)
  - Enhanced `person_repo.rs` with `find_by_national_id_or_contact` matching.
  - Enhanced `DbPermissionChecker` with recursive group membership + role union permission resolution.

### 3.3 Application Layer (`apps/api-rust/crates/orbis-app/`)
- `services/identity_provisioning_service.rs`: Unified provisioning logic, CSPRNG password generation, Argon2 hashing, identity attachment, password reset.
- `services/employee_service.rs`: Full employee registration, updates, status changes, group associations, statistics computation.
- `services/group_service.rs`: Group CRUD, member additions/removals, bulk member assignments, permission matrix updates.

### 3.4 Server REST Endpoints (`apps/api-rust/crates/orbis-server/`)
- `api/employee_routes.rs`:
  - `GET /api/v1/employees` — List/filter employees
  - `POST /api/v1/employees` — Register employee (with optional provisioning)
  - `GET /api/v1/employees/stats` — Status breakdown stats
  - `GET /api/v1/employees/:id` — Single employee dossier
  - `PUT /api/v1/employees/:id` — Update employee
  - `PATCH /api/v1/employees/:id/status` — Change employment status
  - `POST /api/v1/employees/:id/provision-account` — Provision user account
  - `POST /api/v1/employees/:id/reset-password` — Reset temporary password
  - `PUT /api/v1/employees/:id/groups` — Assign user groups
- `api/group_routes.rs`:
  - `GET /api/v1/groups` — List user groups
  - `POST /api/v1/groups` — Create user group
  - `GET /api/v1/groups/:id` — Single group detail
  - `PUT /api/v1/groups/:id` — Update group
  - `DELETE /api/v1/groups/:id` — Delete group
  - `GET /api/v1/groups/:id/members` — List group members
  - `POST /api/v1/groups/:id/members` — Add group member
  - `DELETE /api/v1/groups/:id/members/:userId` — Remove group member
  - `POST /api/v1/groups/bulk-assign` — Bulk assign users to group
  - `PUT /api/v1/groups/:id/permissions` — Set group permission matrix

### 3.5 Tauri Native IPC Commands (`apps/desktop/src-tauri/`)
- `commands/employee_cmds.rs`: Tauri IPC handlers invoking `EmployeeService` and `IdentityProvisioningService`.
- `commands/group_cmds.rs`: Tauri IPC handlers invoking `GroupService`.
- Handlers registered in `commands/mod.rs` and `lib.rs`.

---

## 4. Frontend Desktop Application (`apps/desktop/`)

### 4.1 Client Layer & Type Safety (`apps/desktop/src/lib/api/`)
- `types.ts`: Defined `EmployeeDetailDto`, `AccountProvisioningResultDto`, `RegisterEmployeeResponseDto`, `PaginatedEmployeesDto`, `GroupDetailDto`, `GroupMemberInfoDto`, and request types.
- `local-client.ts`: Complete Tauri IPC calls for desktop execution.
- `remote-client.ts`: Complete REST HTTP calls for cloud execution.
- `query-keys.ts`: TanStack Query key factories for `employee` and `group`.

### 4.2 Feature Modules (`apps/desktop/src/lib/features/`)
- **`employees/`**:
  - `queries.ts`: TanStack Query functions (`createEmployeesQuery`, `createEmployeeQuery`, `createEmployeeStatsQuery`, `createRegisterEmployeeMutation`, `createUpdateEmployeeMutation`, `createUpdateEmployeeStatusMutation`, `createProvisionEmployeeAccountMutation`, `createAssignEmployeeGroupsMutation`, `createResetEmployeePasswordMutation`).
  - `employee-directory.svelte`: Dual-panel workspace with stats metric bar, status & category filters, debounced search, CSV export, virtualized data table, action dropdowns, floating bottom selection bar.
  - `employee-form.svelte`: Multi-tab form with Svelte 5 runes (`$state`, `$derived`), bilingual personal info, job hierarchy, integrated identity provisioning switch with group picker, hotkey `Ctrl+S` saving.
  - `employee-profile.svelte`: 360-degree Dossier with identity overview, employment contract, system account status, group memberships, and computed effective permissions inspection.
  - `components/credential-delivery-modal.svelte`: Secure one-time temporary credential modal with copy to clipboard and visibility toggle.
- **`groups/`**:
  - `queries.ts`: TanStack Query functions for user groups.
  - `groups-manager.svelte`: Group catalog grid cards with system badges, member counts, permission counts, create/edit/delete dialogs.
  - `group-detail-view.svelte`: Dual-tab inspector with members list, removal actions, and full 2D IAM Permission Matrix across 10 functional domains.
  - `components/group-selector.svelte`: Reusable searchable multi-select component for user groups with instant badge visualization and toggle controls.
  - `bulk-assign-dialog.svelte`: Searchable multi-user selection dialog for batch assignment.
- **`shared/components/`**:
  - `user-avatar.svelte`: Rewritten to use canonical `@orbis/ui/avatar` (`Avatar`, `AvatarImage`, `AvatarFallback`) with initials and deterministic color generation.
  - `role-select.svelte`: Reusable dynamic role dropdown querying real backend roles from `api.listRoles()`.
  - `nationality-select.svelte`: Dedicated country/nationality selector powered by `@orbis/location/countries-intl` with search, flag emojis, and bilingual Arabic/English demonyms.
- **`Modular Standalone Sheets`**:
  - `provision-account-sheet.svelte`: Standalone slide-over Sheet for domain user account provisioning with automated integration delivery.
  - `manage-employee-groups-sheet.svelte`: Standalone slide-over Sheet for assigning user groups to employees.
  - `manage-user-groups-sheet.svelte`: Standalone slide-over Sheet for assigning user groups to system users from Settings.

### 4.3 Route Integration & Legacy Cleanup
- `routes/(app)/[orgId]/employees/+page.svelte` (Employee Directory)
- `routes/(app)/[orgId]/employees/new/+page.svelte` (New Employee Registration with `NationalitySelect`)
- `routes/(app)/[orgId]/employees/[id]/+page.svelte` (Employee 360 Dossier with `ProvisionAccountSheet` and `ManageEmployeeGroupsSheet`)
- `routes/(app)/[orgId]/employees/[id]/edit/+page.svelte` (Edit Employee)
- `routes/(app)/[orgId]/settings/groups/+page.svelte` (User Groups Manager)
- `routes/(app)/[orgId]/settings/groups/[id]/+page.svelte` (Group Detail & Permission Matrix)
- `routes/(app)/[orgId]/settings/users/+page.svelte` (Updated Users Directory with domain provisioning guidance and dynamic `RoleSelect`)
- `routes/(app)/[orgId]/settings/users/[id]/+page.svelte` (User Profile with `RoleSelect` and `ManageUserGroupsSheet`)
- Completely deleted legacy standalone user creation pages and sheets (`settings/users/new` and `user-form.svelte`).

### 4.4 Localization (i18n)
- 100% bilingual support in `apps/desktop/messages/ar.json` and `apps/desktop/messages/en.json`.

---

## 5. Verification Matrix & Test Results

| Test / Check Suite | Scope | Result | Details |
|---|---|---|---|
| `cargo check --workspace` | Rust Backend Workspace | **PASSED (0 errors)** | All crates compiled cleanly |
| `cargo check (src-tauri)` | Native Tauri Wrapper | **PASSED (0 errors)** | All IPC handlers bound |
| `cargo test --workspace` | Full Backend Test Suite | **PASSED (73/73 passed)** | Zero test regressions |
| `bun x tsc --noEmit` | TypeScript & Svelte Types | **PASSED (0 errors)** | Zero `as any` bypasses |
| `svelte-kit sync` | SvelteKit Routes & Paraglide | **PASSED (0 errors)** | All route types generated |

---

## 6. Security & Architectural Invariants Verification

- [x] Auto-seeding of system default groups moved to the authoritative Bootstrap / Onboarding adapter (`bootstrap_adapter.rs`).
- [x] Notification delivery dynamically routes via active tenant integrations (Email/WhatsApp/SMS) and policies, falling back safely to in-memory one-time presentation without fixed manual dropdowns.
- [x] All hardcoded role arrays replaced with dynamic `<RoleSelect>` querying backend roles.
- [x] `UserAvatar` rewritten to standard `@orbis/ui/avatar` primitives.
- [x] Lightweight operations (managing groups, provisioning) converted to side `<Sheet>` drawers.
- [x] Standalone "Create User" legacy views and forms completely deleted from disk.
- [x] Passwords generated via CSPRNG and hashed using Argon2; never persisted in plaintext or logged.
- [x] Dual-transport parity verified across all desktop and server operations.
