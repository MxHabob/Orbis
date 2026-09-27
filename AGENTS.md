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

- **Current Status:** `BULK_ENTRY_STUDIO_IMPLEMENTATION_COMPLETED`
- **Master Reference Document:** `docs/bulk_entry_final_architecture.md`
- **Accomplished:**
  - Implemented the unified, production-grade `BulkEntryStudio` supporting both Direct Visual Grid Entry (manual typing, row additions, clipboard paste) and Excel/CSV File Import with 0 records written on initial load.
  - Upgraded Rust Backend (`StudentService::bulk_register_students`, `EmployeeService::bulk_register_employees`) with multi-stage validation, cumulative class capacity tracking, admission policy enforcement, atomic all-or-nothing execution, deterministic sequence numbering, auto-assigned mandatory tuition fees, and guardian linkages.
  - Implemented Tauri IPC commands (`student_bulk_register`, `employee_bulk_register`) with atomic and dry-run parameters.
  - Refactored frontend data fetching to use TanStack Query (`createBulkReferenceDataQuery`) eliminating manual `$effect` synchronization.
  - Consolidated `/settings/bulk-input`, `/settings/bulk-input/students`, `/settings/bulk-input/employees`, `/settings/bulk-input/grades`, and `/import` routes into the unified studio.
  - Verified 0 errors across `bun x tsc` and `cargo check`.
- **Verification:** Frontend and Rust backend compiled with 0 errors.