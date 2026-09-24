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

## 6. CONTINUATION STATE (Agent Session Hand-off)

```yaml
CURRENT_SECTION: "Phase 1: Canonical Architecture & UI Foundation"
CURRENT_PHASE: "PHASE_1"
OVERALL_STATUS: "PLANNING_COMPLETE_READY_FOR_PHASE_1"

COMPLETED:
  - "Full competitor reverse-engineering across 12 modules (docs/competitor_*.md)"
  - "Complete 104-screen mathematical inventory and archetype taxonomy"
  - "Current Orbis UI flexibility audit (docs/orbis_ui_flexibility_audit.md)"
  - "Target user personalization architecture specification (docs/orbis_personalization_target_architecture.md)"
  - "Master 21-column capability comparison matrix (docs/orbis_competitor_gap_matrix.md)"
  - "Target ERP UI architecture & 8 layout primitives design (docs/orbis_target_erp_ui_architecture.md)"
  - "Canonical transformation execution plan (docs/ORBIS_ERP_TRANSFORMATION_EXECUTION_PLAN.md)"

IN_PROGRESS:
  - "Preparation for Phase 1 implementation (Core Layout Primitives & FormActionBar)"

REMAINING_PHASES:
  - "PHASE_1: Core Layout Primitives, FormActionBar, usePersonalization Rune"
  - "PHASE_2: Settings Subsystem as Golden Reference (CascadingSettingsShell)"
  - "PHASE_3: IAM, Users & Permission Matrix (PermissionMatrixView)"
  - "PHASE_4: Academic Structure & Calendar (MasterDetailCatalog)"
  - "PHASE_5: Student Lifecycle & 360 Dossier (EntityWorkspace360)"
  - "PHASE_6: Batch Entry Grid for Attendance & Grades (BatchEntryGridView)"
  - "PHASE_7: Examination Operations & Seating Badges (DocForge batch_print)"
  - "PHASE_8: Financial General Ledger, Vouchers & Fees (AdvancedTransactionView)"
  - "PHASE_9: Universal Reporting & Statements (ReportViewerShell)"
  - "PHASE_10: Desktop Shell, Command Palette & Keyboard Productivity"
  - "PHASE_11: Final Legacy Purge & System Hardening"

KNOWN_BLOCKERS: []

CANONICAL_COMPONENTS_READY:
  - "OrbisGrid (@orbis/svelte-grid)"
  - "DataTable ($lib/tables/core/data-table.svelte)"
  - "MainViewport ($lib/shell/main-viewport.svelte)"
  - "SidebarNav ($lib/shell/sidebar-nav.svelte)"

NEXT_EXACT_TASK:
  id: "TASK-P1-01"
  title: "Implement Core UI Primitives & <FormActionBar> Foundation"
  target_files:
    - "packages/ui/src/components/form-action-bar.svelte"
    - "apps/desktop/src/lib/core/personalization/use-personalization.svelte.ts"
    - "apps/desktop/src/lib/shared/layouts/SimpleFormView.svelte"
    - "apps/desktop/src/lib/shared/layouts/CascadingSettingsShell.svelte"
  description: "Construct the universal <FormActionBar> with keyboard shortcuts and mode states, implement the usePersonalization rune for 0ms cold-start layout persistence, and build SimpleFormView and CascadingSettingsShell layout primitives."
  verification_required:
    - "bun x tsc --noEmit --project apps/desktop/tsconfig.json passes with 0 errors"
    - "Unit test verifying usePersonalization reactive state persistence"
    - "Zero modifications to production database or backend code"
```
