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

- **Current Status:** `SELECT_COMPONENTS_CONSOLIDATED_AND_SELF_FETCHING`
- **Master Reference Documents:** [orbis_ai_architecture_discovery.md](file:///c:/Users/User/.gemini/antigravity-ide/brain/3e2fb622-6955-4230-8ce1-9809fa99c255/orbis_ai_architecture_discovery.md), [student_backend_desktop_contract_reconciliation.md](file:///d:/projects/Orbis/docs/student_backend_desktop_contract_reconciliation.md)
- **Accomplished:**
  - Built unified, generic [searchable-select.svelte](file:///d:/projects/Orbis/apps/desktop/src/lib/shared/components/searchable-select.svelte) equipped with sticky search bar at the top, rich item snippet rendering, and sticky "Create / Add new..." action button at the bottom.
  - Reconciled Teacher vs Employee domain architecture: teachers are queried authoritatively via `createEmployeesQuery` with `employee_type: "teacher"` and `status: "active"`.
  - Refactored specialized Select components ([teacher-select.svelte](file:///d:/projects/Orbis/apps/desktop/src/lib/features/academics/components/teacher-select.svelte), [class-select.svelte](file:///d:/projects/Orbis/apps/desktop/src/lib/features/academics/components/class-select.svelte), [subject-select.svelte](file:///d:/projects/Orbis/apps/desktop/src/lib/features/academics/components/subject-select.svelte), [period-select.svelte](file:///d:/projects/Orbis/apps/desktop/src/lib/features/academics/components/period-select.svelte)) to encapsulate TanStack Query directly inside the component.
  - Eliminated legacy data-expecting props (`teachers = []`, `classes = []`, `subjects = []`, `periods = []`) from all Select component interfaces and consumer screens ([control-grid.svelte](file:///d:/projects/Orbis/apps/desktop/src/lib/features/academics/components/control-grid.svelte), [class-subject-form.svelte](file:///d:/projects/Orbis/apps/desktop/src/lib/features/academics/components/class-subject-form.svelte), [exam-control.svelte](file:///d:/projects/Orbis/apps/desktop/src/lib/features/exams/exam-control.svelte), [archive-manager.svelte](file:///d:/projects/Orbis/apps/desktop/src/lib/features/settings/archive-manager.svelte), [registration-settings-form.svelte](file:///d:/projects/Orbis/apps/desktop/src/lib/features/settings/components/registration-settings-form.svelte), [add-document-requirement-sheet.svelte](file:///d:/projects/Orbis/apps/desktop/src/lib/features/settings/components/add-document-requirement-sheet.svelte), [profile-section.svelte](file:///d:/projects/Orbis/apps/desktop/src/lib/features/students/sections/profile-section.svelte), [session-create-sheet.svelte](file:///d:/projects/Orbis/apps/desktop/src/lib/features/exams/components/session-create-sheet.svelte)).
  - Cleaned up redundant manual API queries in caller components, avoiding duplicate network/IPC calls and out-of-sync states.
  - Verified 0 errors across `bun x tsc --noEmit` (clean), `cargo check --workspace` in `apps/api-rust` (clean), `cargo check` in `apps/desktop/src-tauri` (clean), and production static build `bun run build` (clean).
- **Next Target Module:** `EXAMS` / `FINANCE` domain workflows and DocForge template integrations.