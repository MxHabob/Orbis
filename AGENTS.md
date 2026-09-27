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

- **Current Status:** `BULK_ENTRY_FINAL_ARCHITECTURE_APPROVED`
- **Master Reference Document:** `docs/bulk_entry_final_architecture.md`
- **Accomplished:**
  - Conducted full code-level audit across Rust backend (`orbis-app`, `orbis-domain`, `orbis-infra`, `orbis-server`), Tauri 2.0 plugins (`dialog`, `opener`, `fs`, `window`, `process`), desktop routes, and `OrbisGrid` (`packages/svelte-grid`).
  - Proved that selecting an Excel file writes 0 records to DB: the entire payload is loaded in-memory directly into `OrbisGrid` for visual inspection, inline correction, and pre-commit impact review.
  - Specified the 6-Level Multi-Stage Validation Hierarchy (File, Cell, Row, Cross-Row/Batch, Domain Use Case, Database/Tx) with Live Incremental Revalidation.
  - Designed the dynamic Excel Template Generator with protected reference sheets (Classes, Sections, Levels, Subjects, Fee Templates, User Groups, Enums).
  - Formulated the typed domain profiles for Students (`StudentService::register_student`), Employees (`EmployeeService::register_employee`), and Grades (`AssessmentService::record_grade` + `CalculationEngine`).
  - Established chunked-transaction rollback boundaries, natural-key idempotency, structured append-only audit logging in `audit_events`, and zero new database tables.
  - Documented complete 35 E2E test scenarios, 18-step sequential implementation roadmap, and consolidation matrix.
  - Authored the master architecture blueprint at `docs/bulk_entry_final_architecture.md`.
- **Verification:** Architectural status set to `APPROVED`.