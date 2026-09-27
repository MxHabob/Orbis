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

- **Current Status:** `STUDENTS_RECONCILIATION_COMPLETED`
- **Master Reference Document:** `docs/students_backend_desktop_reconciliation_report.md`
- **Accomplished:**
  - Audited backend Clean Architecture domain entities (`Student`, `Person`, `Enrollment`, `StudentGuardian`, `StudentFee`, `Attendance`, `BehaviorRecord`, `DocumentRequirement`).
  - Added and registered missing Tauri IPC commands (`student_remove_guardian`, `student_delete_student`, `student_restore_student`, `assessment_list_student_attendance`, `document_list_by_entity`).
  - Expanded and reconciled API client types (`types.ts`), local client (`local-client.ts`), and remote client (`remote-client.ts`) with 100% parity.
  - Rebuilt Student Directory (`student-directory.svelte`, `student-table.svelte`, `store.svelte.ts`) with canonical `StudentDetailDto` and normalized Arabic search.
  - Reconciled Student Registration form and state controllers (`student-form.svelte`, `form-state.svelte.ts`, `personal-info.svelte.ts`, `financial-info.svelte.ts`, `document-info.svelte.ts`).
  - Rebuilt Student Detail Workspace (`[studentId]/+layout.svelte`, `student-details.svelte`) and all sub-feature tabs (Guardians, Grades, Fees, Attendance, Behavior, Documents).
  - Verified 0 errors across `bun x tsc --noEmit`, `cargo check` in `src-tauri`, and `cargo check --workspace` in `apps/api-rust`.
- **Next Target Module:** `ACADEMICS` (as per the master contract).