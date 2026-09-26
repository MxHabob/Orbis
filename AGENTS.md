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

- **Current Status:** `ARCHIVE_AND_SETTINGS_GRID_REFACTOR_COMPLETE`
- **Master Reference Document:** `docs/registration_erp_architecture_plan.md`
- **Accomplished:**
  - Rebuilt [`ArchiveManager`](file:///d:/projects/Orbis/apps/desktop/src/lib/features/settings/archive-manager.svelte) from the ground up: integrated student-specific autocomplete document filtering, integrated pre-built [`ClassSelect`](file:///d:/projects/Orbis/apps/desktop/src/lib/features/academics/components/class-select.svelte) and [`PeriodSelect`](file:///d:/projects/Orbis/apps/desktop/src/lib/features/academics/components/period-select.svelte) from Academics, eliminated all `$effect` usage in favor of TanStack queries and declarative derived states.
  - Refactored [`RegistrationSettingsForm`](file:///d:/projects/Orbis/apps/desktop/src/lib/features/settings/components/registration-settings-form.svelte) with [`OrbisGrid`](file:///d:/projects/Orbis/packages/svelte-grid/src/OrbisGrid.svelte) for both Document Requirements Studio and Form Fields Matrix with interactive actions.
  - Modernized [`AuditLogViewer`](file:///d:/projects/Orbis/apps/desktop/src/lib/features/settings/audit-log-viewer.svelte) to use TanStack Query (`createQuery`), eliminated manual `loadData` and mock data, bound directly to authoritative backend fields, and hardened [`formatDate`](file:///d:/projects/Orbis/apps/desktop/src/lib/core/i18n/format.ts).
- **Verification:** `cargo check --manifest-path apps/api-rust/Cargo.toml` & `bun x tsc -p apps/desktop/tsconfig.json --noEmit` verified with 0 errors.