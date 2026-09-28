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

- **Current Status:** `FINANCE_UI_COMPLETE_REBUILD_ALIGNMENT_WITH_OVERVIEW_DESIGN`
- **Master Reference Documents:** [orbis_ai_architecture_discovery.md](file:///c:/Users/User/.gemini/antigravity-ide/brain/3e2fb622-6955-4230-8ce1-9809fa99c255/orbis_ai_architecture_discovery.md), [student_backend_desktop_contract_reconciliation.md](file:///d:/projects/Orbis/docs/student_backend_desktop_contract_reconciliation.md)
- **Accomplished:**
  - Rebuilt all Finance module screens (`finance-overview.svelte`, `journal-entries.svelte`, `payments-manager.svelte`, `disbursements-manager.svelte`, `fees-workspace.svelte`, `accounts-manager.svelte`, `account-statement.svelte`, `financial-report-page.svelte`) to strictly follow the clean, minimal, elegant design language established in `FinanceOverview`.
  - Rebuilt `AccountTree` into an ultra-compact C# / .NET WinForms style TreeView with 22px dense rows, authentic `[+]` / `[−]` toggles, connecting guide lines, and direct backend account hierarchy (completely eliminating all synthetic wrapper categories and duplicate nodes).
  - Retained authentic double-entry ledger posting, payment recording, fee installments splitting, refunds, running balances, and multi-currency reporting backed 100% by real Rust backend domain capabilities.
  - Verified 0 errors across `bun x tsc --noEmit` in `apps/desktop`, `cargo check` in `apps/desktop/src-tauri`, and `cargo check --workspace` in `apps/api-rust`.
- **Next Target Module:** `DOCFORGE` and `EXAMS` / `FINANCE` screen integration tests.