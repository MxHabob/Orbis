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

- **Current Status:** `STUDENTS_RECONCILIATION_COMPLETE`
- **Master Reference Documents:** [student_backend_desktop_contract_reconciliation.md](file:///d:/projects/Orbis/docs/student_backend_desktop_contract_reconciliation.md), [student_backend_desktop_reconciliation_report.md](file:///d:/projects/Orbis/docs/student_backend_desktop_reconciliation_report.md)
- **Accomplished:**
  - Audited backend Clean Architecture database schema, entities, and application use cases for Students (`Person`, `Student`, `StudentGuardian`, `Enrollment`, `StudentFee`, `DocumentRequirement`, `Document`).
  - Completely resolved the Person vs. Student vs. User vs. Guardian relationship hierarchy: Guardians are canonical `Person` entities linked via `student_guardians` without synthetic IAM `User` account requirements.
  - Reconciled schemas in `@orbis/schemas` (`studentRegistrationSchema`, `guardianFormInputSchema`) removing synthetic passwords and aligning with backend `RegisterStudentInput` and `AssignGuardianRequest`.
  - Fully eliminated all `$effect` usages across the Student subsystem (`guardian-form.svelte`, `student-form.svelte`, `student-details.svelte`), strictly using declarative `$derived` runes, Clean Code, and zero reactivity anti-patterns.
  - Eliminated all `any` and legacy fields across `form-state.svelte.ts`, `personal-info.svelte.ts`, `document-info.svelte.ts`, `student-grades.svelte`, `student-attendance.svelte`.
  - Produced comprehensive contract reconciliation document `docs/student_backend_desktop_contract_reconciliation.md`.
  - Verified 0 errors across `bun x tsc --noEmit` (clean), `cargo check --workspace` in `apps/api-rust` (clean), and production static build `bun run build` (clean 38.87s).
- **Next Target Module:** `EXAMS` / `FINANCE` (as per the master contract).