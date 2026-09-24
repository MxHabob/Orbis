# Orbis Desktop — Engineering Rules & Agent Execution Contract

> **Document Version:** 2.0.0-PROD  
> **Target System:** Orbis Enterprise Educational Operating System (Desktop Core)  
> **Master Execution Plan:** `docs/ORBIS_ERP_TRANSFORMATION_EXECUTION_PLAN.md`  

---

## 1. Executive Instructions for AI Coding Agents

Every agent starting a session on Orbis MUST:
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

## 2. Technology Stack & Core Commands

- **Frontend:** SvelteKit 2 with Svelte 5 Runes (`$state`, `$derived`, `$props`, `$effect`, snippets).
- **Backend (Embedded):** Rust Clean Architecture (`orbis-domain`, `orbis-app`, `orbis-infra`, `orbis-server`, `docforge-engine`) via Tauri 2.0 IPC commands.
- **Dual-Transport Parity:** `OrbisApiClient` polymorphic interface implemented identically by `LocalClient` (Tauri IPC / SQLite) and `RemoteClient` (HTTP fetch / `@orbis/sdk` / PostgreSQL).
- **UI & Grid Library:** `@orbis/ui` + `@orbis/svelte-grid` (virtualized DOM/canvas hybrid `OrbisGrid`).
- **Styling:** TailwindCSS 4 with glassmorphic dark/light themes.
- **Internationalization:** Paraglide.js (compile-time type-safe RTL/LTR i18n).

```bash
# Core Verification Commands (Run before declaring any task done)
bun install                                      # Install dependencies
bun run dev                                      # Start SvelteKit dev server
cargo tauri dev                                  # Start full Tauri desktop app
bun x tsc --noEmit --project tsconfig.json       # Desktop TypeScript check
cargo check --manifest-path src-tauri/Cargo.toml # Rust backend check
cargo test --workspace --manifest-path ../api-rust/Cargo.toml # Run backend tests
```

---

## 3. The Cardinal Law: Consolidation Before Addition

$$\text{Existing} \longrightarrow \text{Verify Ownership} \longrightarrow \text{Verify Parity} \longrightarrow \text{Reuse} \longrightarrow \text{Extend} \longrightarrow \text{Refactor} \longrightarrow \text{Merge} \longrightarrow \text{Replace} \longrightarrow \text{Delete Legacy} \longrightarrow \text{Add}$$

- **Search First:** Before creating any new component, store, or service, search the repository for an existing equivalent.
- **Merge Duplicates:** If multiple components perform the same job (e.g. duplicate CRUD toolbars or dialogs), migrate all consumers to the canonical component, then delete the duplicate.
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

## 6. Mandatory Legacy Removal Pass

After finishing work on any section:
1. Grep for all references to superseded components or old sub-routes.
2. Ensure all consumers have been migrated to the canonical primitive.
3. Delete the deprecated files completely.
4. Run `bun x tsc --noEmit` and `cargo check --workspace`.
5. Run full test suite (`cargo test --workspace`).

---
