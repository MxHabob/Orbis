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

- **Current Status:** `USER_PROVISIONING_AND_INTEGRATION_CHANNELS_CONSOLIDATED`
- **Master Reference Documents:** [orbis_ai_architecture_discovery.md](file:///c:/Users/User/.gemini/antigravity-ide/brain/3e2fb622-6955-4230-8ce1-9809fa99c255/orbis_ai_architecture_discovery.md), [student_backend_desktop_contract_reconciliation.md](file:///d:/projects/Orbis/docs/student_backend_desktop_contract_reconciliation.md)
- **Accomplished:**
  - Built enterprise-grade User Account Provisioning & Identity Management workflow in [users-directory.svelte](file:///d:/projects/Orbis/apps/desktop/src/lib/features/settings/users-directory.svelte) and [create-user-sheet.svelte](file:///d:/projects/Orbis/apps/desktop/src/lib/features/settings/components/create-user-sheet.svelte).
  - Adopted the canonical Orbis Sheet structure matching [discount-template-sheet.svelte](file:///d:/projects/Orbis/apps/desktop/src/lib/features/finance/components/discount-template-sheet.svelte) and [discount-template-form.svelte](file:///d:/projects/Orbis/apps/desktop/src/lib/features/finance/components/discount-template-form.svelte), with `Sheet.Root`, `Sheet.Header`, `Accordion` multi-section groupings, and standard `FormField` components.
  - Guaranteed Zero Person Duplication across all personas: directly reuses `hr_employees.person_id`, `students.person_id`, and `student_guardians.guardian_id` (`person_id`) without creating unnecessary duplicate `Person` records.
  - Dynamically wired credential delivery channels (Email, SMS, WhatsApp) to actual active workspace integrations via `api.listIntegrations()`, replacing static hardcoded channels with dynamic gateway capability checks and status badges.
  - Seamlessly integrated with [credential-delivery-modal.svelte](file:///d:/projects/Orbis/apps/desktop/src/lib/shared/components/credential-delivery-modal.svelte) for instant temporary credential display, copy, and print slips.
  - Verified 0 errors across `bun x tsc --noEmit` (clean), `cargo check --workspace` in `apps/api-rust` (clean), `cargo check` in `apps/desktop/src-tauri` (clean), and production static build `bun run build` (clean in 27s).
- **Next Target Module:** `EXAMS` / `FINANCE` domain workflows and DocForge template integrations.