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
- **Active Phase:** Phase 4 — IAM, Users, Roles & Permissions Reconciliation (Completed)
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
  - **Verification:**
    - `apps/api-rust` cargo check: 0 errors
    - `apps/desktop/src-tauri` cargo check: 0 errors
    - `packages/sdk` tsc: 0 errors
    - `apps/desktop` tsc: 0 errors
- **Next Scope in Master Execution Plan:** Phase 5 — Academic Structure & Calendar Reconciliation (Years, Periods, Classes, Subjects, Rooms, TimeSlots).
