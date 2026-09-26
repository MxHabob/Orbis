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

```bash
# Core Verification Commands (Run before declaring any task done)
bun install                                      # Install dependencies
bun run dev --cwd apps/desktop                   # Start SvelteKit dev server
cargo tauri dev --manifest-path apps/desktop/src-tauri/Cargo.toml # Start full Tauri desktop app
bun x tsc --noEmit --project apps/desktop/tsconfig.json           # Desktop TypeScript check
cargo check --workspace --manifest-path apps/api-rust/Cargo.toml   # Rust backend check
cargo test --workspace --manifest-path apps/api-rust/Cargo.toml    # Run full test suite
```

---

## 3. The Cardinal Law: Consolidation Before Addition

$$\text{Existing} \longrightarrow \text{Verify Ownership} \longrightarrow \text{Verify Parity} \longrightarrow \text{Reuse} \longrightarrow \text{Extend} \longrightarrow \text{Refactor} \longrightarrow \text{Merge} \longrightarrow \text{Replace} \longrightarrow \text{Delete Legacy} \longrightarrow \text{Add}$$

- **Search First:** Before creating any new component, store, or service, search the repository for an existing equivalent.
- **Merge Duplicates:** If multiple components perform the same job, migrate all consumers to the canonical component, then delete the duplicate.
- **No Parallel Architectures:** Do NOT implement parallel table systems or alternative reporting engines. `OrbisGrid` is the canonical grid foundation.

---

## 4. The 18-Point Section Completion Contract

A section is **NEVER** complete merely because its route renders. A section is complete **ONLY** when:
1. Backend invariants in `orbis-domain` and use cases in `orbis-app` are fully wired.
2. Dual-transport parity (`LocalClient` and `RemoteClient`) is verified.
3. Zero `as any` casts or unchecked type assertions.
4. UI is constructed using the 8 canonical layout primitives.
5. Mode-aware `<FormActionBar>` is integrated with keyboard shortcuts (`Ctrl+N`, `Ctrl+S`, `Esc`, `F7`, `F6`).
6. Explicit skeleton loading states are present.
7. Explicit `<TableEmptyState>` or `<AiEmptyState>` is rendered when data is empty.
8. Explicit error boundaries handle failures gracefully.
9. `unsavedGuardStore` protects dirty forms from navigation or close.
10. Backend permissions (`PermissionCheckerPort`) and client `<PermissionGate>` are enforced.
11. Materialized `OrgUnit` tree scope (`/1/4/12/`) is enforced.
12. Settings follow $\text{Stored} \rightarrow \text{Read} \rightarrow \text{Enforced} \rightarrow \text{Tested}$.
13. User personalization (columns, widths, filters) persists via `usePersonalization`.
14. Documents and receipts render through `docforge-engine` with HarfBuzz RTL Arabic shaping.
15. 100% of labels use Paraglide messages; formatting uses `orgContext.formatDate()` and `orgContext.formatCurrency()`.
16. Tables with 10k+ rows use virtualized `OrbisGrid`.
17. Legacy / superseded code is completely deleted from disk.
18. `cargo check`, `cargo test`, and `bun x tsc --noEmit` pass with zero errors.

---

## 5. Canonical Layout Primitives & Component Registry

| Layout Primitive | Path | Responsibility |
|---|---|---|
| `<FormActionBar>` | `@orbis/ui` | Standard CRUD and transaction lifecycle bar with hotkeys |
| `SimpleFormView` | `$lib/shared/layouts/` | Form + Action Bar + DataTable master catalog shell |
| `AdvancedTransactionView` | `$lib/shared/layouts/` | Header/detail voucher with `Draft` $\rightarrow$ `Posted` $\rightarrow$ `Unposted` state machine |
| `EntityWorkspace360` | `$lib/shared/layouts/` | Split tree + quick action ribbon + multi-tab dossier |
| `ReportViewerShell` | `$lib/shared/layouts/` | Collapsible filter drawer + active chips + DocForge live PDF |
| `BatchEntryGridView` | `$lib/shared/layouts/` | Virtualized matrix entry with keyboard column traversal |
| `CascadingSettingsShell` | `$lib/shared/layouts/` | Hierarchical settings tree with visual inheritance badges |
| `PermissionMatrixView` | `$lib/shared/layouts/` | 2D Screen $\times$ Action permission matrix |
| `MasterDetailCatalog` | `$lib/shared/layouts/` | Side rail list selector + rich detail inspector |
| `OrbisGrid` | `@orbis/svelte-grid` | High-performance virtualized canvas/DOM grid |
| `DataTable` | `$lib/tables/core/` | Search/filter/pagination wrapper around `OrbisGrid` |

---

## 6. CONTINUATION STATE

- **Subsystem:** Employee Affairs, User Groups & IAM Roles
- **State:** 100% DB-driven role architecture without hardcoded role arrays or static fallbacks.
- **OrbisGrid & Sheet Architecture in Employee Affairs:**
  - `employee-directory.svelte`: Fully migrated to high-performance `<OrbisGrid>` with typed column definitions (`__selected`, `__idx`, `full_name`, `employee_number`, `job_dept`, `type_display`, `status`, `account_display`, `hire_date_display`, `actions`), row action dispatcher, filter pills, and multi-selection synchronization.
  - `EmployeeSheet` (`components/employee-sheet.svelte`): Canonical multi-mode slide-over Sheet component supporting `preview` (instant 360 overview + quick actions), `create` (multi-tab structured form with `NationalitySelect`, `GroupSelector`, and unified identity provisioning), and `edit` (targeted updates).
  - Legacy cleanup: Removed superseded `confirm-dialog.svelte`; all status changes and destructive actions use global `confirmStore.prompt(...)`.
- **Verification:** `bun x tsc --noEmit`, `cargo check --workspace`, and `cargo test --workspace` (all 73 tests) pass with 0 errors.
