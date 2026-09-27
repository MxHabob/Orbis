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

- **Current Status:** `HR_PAYROLL_AND_LEDGER_ENHANCEMENTS_COMPLETE`
- **Master Reference Document:** `docs/registration_erp_architecture_plan.md`
- **Accomplished:**
  - Completely removed `seed_fee_templates` and default hardcoded fees from both `bootstrap()` and `persist_onboarding()`; fee templates are now only persisted when explicitly provided by the user in the onboarding payload, properly linking to the authoritative COA accounts (`110301` Accounts Receivable, `410301`/`410101` Tuition Revenue).
  - Made `seed_chart_of_accounts` fully idempotent and re-entrant via recursive tree traversal (`seed_account_tree`), preventing duplicate account insertion and resolving existing parent accounts upon partial failures or re-runs.
  - Moved `seed_document_requirements` entirely out of `bootstrap()` into `OnboardingService` (`persist_onboarding`), where it dynamically resolves each class from `data.classes` and attaches exact cumulative previous-grade certificates.
  - Deferred file attachments: Files are held locally in memory during registration and atomically uploaded on form save.
  - Integrated full Payroll & Compensation (`payroll`) into Employee Affairs (`employee-form.svelte`), reactive live calculations for gross/deductions/net salary, allowances, GOSI, and bank routing.
  - Upgraded desktop Ledger form (`ledger-form.svelte`) with complete Chart of Accounts tree selector via TanStack Query (`createAccountsQuery`), authoritative base currency from `orgContext` store, auto-suggested sequential codes with manual override, live Account Nature badges (Debit/Credit), and full entity support (Students, Employees, Vendors, Partners).
  - Fixed Fee Templates `applicable_levels` end-to-end: persisted per-grade targets during onboarding, exposed in `FeeTemplateDto` via Tauri IPC, converted `fee-template-form.svelte` to TanStack Query (`createClassesQuery`) with 100% dynamic level extraction from the database (zero hardcoded dictionaries), and completely eliminated all `$effect` usage in favor of direct deterministic state initialization.
  - Implemented full Fee Templates CRUD: added `update_fee_template` and `delete_fee_template` across `FeeTemplateRepository`, `FinanceRepoImpl`, `FinanceService`, and Tauri IPC commands (`finance_update_fee_template`, `finance_delete_fee_template`); upgraded `fee-templates-manager.svelte` with TanStack Query, `confirmStore` prompt, and an Actions column (`ActionItem[]`) with Edit/Delete capabilities.
  - Redesigned Employee Form (`employee-form.svelte`) to follow the exact Sheet / Accordion design system (matching `discount-template-form.svelte` and `fee-template-form.svelte`) with 4 organized sections (Personal & Identity, Employment & Contract, Live Payroll & Allowances, and System Account & Identity Provisioning with CSPRNG Credential Delivery Modal), zero `$effect` usage, and responsive compact form field grids.
- **Verification:** `cargo check --manifest-path apps/api-rust/Cargo.toml` & `bun x tsc -p apps/desktop/tsconfig.json --noEmit` verified with 0 errors.