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

- **Current Status:** `DESKTOP_FORMS_AND_FIELDS_SYNCHRONIZED_WITH_CANONICAL_DOMAIN`
- **Master Reference Documents:** [orbis_ai_architecture_discovery.md](file:///c:/Users/User/.gemini/antigravity-ide/brain/3e2fb622-6955-4230-8ce1-9809fa99c255/orbis_ai_architecture_discovery.md), [student_backend_desktop_contract_reconciliation.md](file:///d:/projects/Orbis/docs/student_backend_desktop_contract_reconciliation.md)
- **Accomplished:**
  - Enriched all backend Axum/utoipa response DTOs with complete audit tracking fields (`created_at`, `updated_at`, `created_by`, `updated_by`, `org_unit_id`, `version`) across all server routes.
  - Regenerated OpenAPI 3.1 specification via `export_openapi` and rebuilt `@orbis/sdk`.
  - Enriched Desktop UI tables and grids to render localized audit columns (`created_at`, `updated_at`).
  - Synchronized Desktop forms and state across modules (`Students`, `Employees`, `Finance`, `Academics`):
    - Added and bound `national_id`, `passport_number`, `nationality`, `place_of_birth`, `blood_type`, `medical_notes`, and `special_needs_notes` in `student-details.svelte` and `profile-section.svelte`.
    - Added `is_authorized_pickup`, `employer`, `work_phone` in `family-section.svelte` and `guardian-info.svelte.ts`.
    - Fixed `ledger-form.svelte` mapping to use `account_type` instead of `item_type`.
  - Verified 0 errors across `bun x tsc --noEmit` in `apps/desktop` and `packages/sdk`, `cargo check --workspace` in `apps/api-rust`, and `cargo check` in `apps/desktop/src-tauri`.
- **Next Target Module:** `DOCFORGE` and `EXAMS` / `FINANCE` screen integration tests.