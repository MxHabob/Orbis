# Orbis Desktop — AGENTS.md

> **The definitive reference for all developers and AI agents working on `desktop`.**
> Last updated: 2026-07-13.

---

## 1. Architecture Rules

### 1.1 Project Structure

```
apps/desktop/
├── src/
│   ├── app.html                    # HTML template
│   ├── app.d.ts                    # Global type augmentations
│   ├── hooks.server.ts             # Server hooks (paraglide i18n)
│   ├── hooks.ts                    # Client reroute (i18n)
│   ├── lib/
│   │   ├── core/                   # Infrastructure layer (boot, IPC, query, platform, i18n)
│   │   │   ├── bootstrap.ts        # One-time app initialization
│   │   │   ├── ipc/                # Tauri IPC re-exports + utilities (dedup, prefetch, error)
│   │   │   ├── query/              # TanStack Query client configuration
│   │   │   ├── platform/           # Platform detection utilities
│   │   │   └── i18n/               # Date/locale formatting, Hijri calendar
│   │   ├── features/               # Domain features (feature-based architecture)
│   │   │   ├── academics/          # Classes, subjects, periods, schedules, promotions
│   │   │   ├── ai/                 # Orbis Autopilot, Agent sessions
│   │   │   ├── auth/               # Login, account management
│   │   │   ├── bulk-input/         # Bulk grade/student entry grids
│   │   │   ├── dashboard/          # Overview dashboard
│   │   │   ├── exams/              # Exam sessions, scheduling, seating
│   │   │   ├── finance/            # Fees, payments, treasuries, ledger
│   │   │   ├── metrics/            # Analytics cards, charts
│   │   │   ├── onboarding/         # School setup wizard
│   │   │   ├── settings/           # Organization, users, permissions
│   │   │   ├── students/           # Student CRUD, enrollment, guardian forms
│   │   │   ├── templates/          # Document template editor (canvas-based)
│   │   │   └── workflow/           # Approvals, override requests
│   │   ├── shared/                 # Cross-feature shared code
│   │   │   ├── components/         # Reusable UI components
│   │   │   ├── skeletons/          # Loading skeleton components
│   │   │   └── utils/              # Utility functions (search, debounce)
│   │   ├── shell/                  # App chrome: sidebar, title bar, search, AI panel
│   │   ├── stores/                 # Global singleton stores (auth, theme, UI, academic context)
│   │   ├── tables/                 # Table system (DataTable, column defs, cell renderers)
│   │   ├── assets/                 # Static assets (SVG favicon, etc.)
│   │   └── paraglide/              # Generated i18n runtime (DO NOT EDIT)
│   └── routes/                     # SvelteKit file-based routing
│       ├── +layout.svelte          # Root: QueryClientProvider + Toaster + Theme
│       ├── layout.css              # Global CSS (Tailwind + overrides)
│       ├── (app)/                  # App group layout (TitleBar + SearchModal)
│       │   ├── (auth)/             # Unauthenticated routes (login, onboarding)
│       │   └── [orgId]/            # Authenticated routes (AppChrome shell)
│       │       ├── academics/      # Academic management pages
│       │       ├── students/       # Student management pages
│       │       ├── finance/        # Finance pages
│       │       ├── settings/       # Settings pages (18 sub-routes)
│       │       ├── templates/      # Template editor pages
│       │       ├── reports/        # Reports page
│       │       ├── billing/        # Billing page
│       │       └── account/        # User account pages
│       └── agent/                  # AI Agent standalone page
├── src-tauri/                      # Rust backend (Tauri v2)
│   ├── src/
│   │   ├── lib.rs                  # Tauri app builder + all command registrations
│   │   ├── state.rs                # AppState (database connections, services)
│   │   ├── commands/               # Tauri IPC command handlers (by domain)
│   │   └── features/               # Desktop-specific features (deep-link)
│   ├── Cargo.toml                  # Rust dependencies (shared crates from api-rust)
│   └── tauri.conf.json             # Tauri configuration
├── messages/                       # i18n translation files
├── static/                         # Static files
└── package.json                    # Node dependencies
```

### 1.2 Folder Conventions

| Layer | Purpose | Rule |
|-------|---------|------|
| `core/` | Infrastructure | Zero domain knowledge. Reusable across any app. |
| `features/<domain>/` | Domain features | Self-contained. Owns queries, stores, components, sections. |
| `shared/` | Cross-feature code | Must be used by ≥2 features. No domain logic. |
| `shell/` | App chrome | Layout components only. Sidebar, titlebar, search, AI. |
| `stores/` | Global singletons | Auth, Theme, UI, AcademicContext, WorkflowInbox only. |
| `tables/` | Table system | DataTable core, column definitions, cell renderers. |
| `routes/` | Pages | Thin orchestration. Import from `features/` and `shared/`. |

### 1.3 Naming Conventions

| Element | Convention | Example |
|---------|-----------|---------|
| Svelte components | `kebab-case.svelte` | `student-form.svelte` |
| TypeScript files | `kebab-case.ts` | `form-state.svelte.ts` |
| Svelte stores | `*.svelte.ts` suffix | `auth.svelte.ts` |
| Query files | `queries.ts` | `features/students/queries.ts` |
| Index barrels | `index.ts` | Export public API only |
| Route files | SvelteKit conventions | `+page.svelte`, `+layout.ts` |
| CSS classes | Tailwind utilities | No custom class names unless in `layout.css` |
| IPC commands | `snake_case` | `list_enrollments`, `create_class` |
| Query keys | `kebab-case` arrays | `["enrollments", orgId]` |
| Store classes | `PascalCase` class, `camelCase` export | `class AuthStore`, `export const authStore` |

### 1.4 Dependency Rules

```
routes/ → features/, shared/, shell/, stores/, core/
features/ → shared/, stores/, core/
shell/ → shared/, stores/, core/
shared/ → core/ (ONLY)
stores/ → core/ (ONLY)
core/ → @orbis/sdk-*, @tauri-apps/*, @tanstack/* (ONLY)
```

**Forbidden dependencies:**
- `routes/` → `routes/` (no cross-page imports)
- `features/X` → `features/Y` (no cross-feature imports; use shared/ or stores/)
- `shared/` → `features/` (never depend on domain code)
- `core/` → `features/` (never depend on domain code)
- `core/` → `stores/` (core is store-agnostic)

### 1.5 Module Boundaries

- **Every feature** must export its public API through `index.ts`.
- **No deep imports** from another feature's internals: use `$lib/features/students` not `$lib/features/students/components/guardian-form.svelte`.
- **Exception**: Route pages may import feature-internal components when rendering page-level UI.

---

## 2. Coding Standards

### 2.1 TypeScript Rules

- **Strict mode** is enabled. Never use `// @ts-ignore` or `// @ts-nocheck`.
- **No `any`** types. Use `unknown` and type-narrow, or define proper interfaces.
- **All types** come from `@orbis/sdk-core`. Never duplicate type definitions in the frontend.
- **Use `satisfies`** for compile-time type checking of config objects.
- **Prefer interfaces** over type aliases for object shapes.
- **Use `$derived`** instead of `$:` reactive declarations (Svelte 5 runes).

### 2.2 Svelte 5 Rules

- **Runes mode only**. All components use `$state`, `$derived`, `$effect`, `$props`.
- **No Svelte 4 syntax**. No `export let`, no `$:`, no `$$props`, no `$$restProps`.
- **Use `$props()`** with explicit type annotations:
  ```svelte
  let { orgId, children }: { orgId: string; children: Snippet } = $props();
  ```
- **Use `$derived`** for computed values. Use `$derived.by()` for multi-line computations.
- **Use `$effect`** sparingly. Prefer derived state over side effects.
- **Use `{@render children()}`** instead of `<slot />`.
- **Use `{#snippet name()}...{/snippet}`** for named slots.

### 2.3 Tauri IPC Rules

- **All IPC calls** go through `@orbis/sdk-desktop` commands. Never call `invoke()` directly in app code.
- **Deduplication**: Use `dedup()` for calls that multiple components may trigger simultaneously.
- **Error handling**: Always use `extractErrorMessage(e)` from `@orbis/sdk-core`.
- **Prefetching**: Use `prefetchRouteData()` on sidebar `mouseenter` events.
- **No business logic** in IPC wrappers. The Rust backend is the single source of truth.

### 2.4 Error Handling

```typescript
// ✅ Correct: Use toast for user-facing errors, console.warn for non-critical
try {
  const result = await commands.academic.listClasses(orgId);
} catch (e: unknown) {
  toast.error(extractErrorMessage(e));
}

// ✅ Correct: Non-critical — warn and continue
try {
  await commands.platform.getAcademicContext(orgId);
} catch (e: unknown) {
  console.warn("Failed to resolve context:", extractErrorMessage(e));
}
```

- **Never swallow errors silently** (empty catch blocks).
- **Never use `String(e)`** — Tauri errors are plain objects, not Error instances.
- **Use `extractErrorMessage()`** from `@orbis/sdk-core` for all IPC errors.
- **Classify errors**: toast for user-blocking, console.warn for non-critical.

### 2.5 State Management

| Scope | Pattern | Example |
|-------|---------|---------|
| **Server data** | TanStack Query | `createClassesQuery(orgId)` |
| **Global UI state** | Singleton store class | `authStore`, `uiStore`, `themeStore` |
| **Feature UI state** | Feature store class | `studentsStore.searchQuery` |
| **Form state** | Local class instance | `new StudentFormState()` |
| **Component state** | `$state()` rune | `let isOpen = $state(false)` |

**Rules:**
- Server data lives in TanStack Query cache. **Never duplicate** it in `$state`.
- Stores use the **class-based singleton** pattern with runes.
- Feature stores manage **UI-only state** (filters, selection). Not server data.
- `academicContext` is the **single source of truth** for year/period/phase.

### 2.6 Data Fetching

- **Use TanStack Query** for all server data. Create query factories in `features/<domain>/queries.ts`.
- **Query keys**: Structured arrays `["resource", orgId, ...params]`.
- **staleTime**: Configure per-domain (2min for frequent changes, 10min for static data).
- **Mutations**: Use `createMutation` with `onSuccess` invalidation.
- **No manual fetch** in `onMount` or `$effect` for server data.
- **Prefetch** on navigation intent (mouseenter on sidebar links).

### 2.7 Forms

- Complex forms use a dedicated **FormState class** (e.g., `StudentFormState`).
- Validation rules live in the **Backend**. Frontend does only UX validation (required fields, format hints).
- **No duplicate validation** between frontend and backend.
- Use **Zod** schemas when frontend-only validation is needed.

### 2.8 Tables

- Use the `DataTable` component from `$lib/tables/core`.
- Column definitions go in `$lib/tables/definitions/`.
- Table config (sticky columns, row heights, sort maps) in `$lib/tables/configs.ts`.
- Custom cell renderers in `$lib/tables/components/`.
- Tables use **virtualized rendering** via `@orbis/svelte-grid`.

### 2.9 Dialogs

- Lazy-load dialogs with dynamic imports:
  ```svelte
  {#if showDialog}
    {#await import("./my-dialog.svelte") then { default: MyDialog }}
      <MyDialog bind:open={showDialog} />
    {/await}
  {/if}
  ```
- Dialogs should be self-contained. Pass data via props, not stores.

### 2.10 Routing

- All authenticated routes live under `routes/(app)/[orgId]/`.
- Route pages are **thin orchestrators** — they import feature components.
- Use `+layout.ts` for data loading and auth guards.
- Use `localizeHref()` for all navigation links (i18n-safe).
- Prefer `goto()` for programmatic navigation.

### 2.11 Accessibility

- All interactive elements must have `aria-label` or visible label text.
- Support keyboard navigation: `Tab`, `Enter`, `Escape`, arrow keys.
- Color is never the sole indicator of state.
- Respect `prefers-reduced-motion`.

### 2.12 Internationalization (i18n)

- All user-facing strings use Paraglide messages: `import * as m from "$lib/paraglide/messages"`.
- Never hardcode user-visible text.
- Use `localizeHref()` for links. Use `getLocale()` for runtime locale checks.
- Date formatting: use `$lib/core/i18n/format.ts` utilities.
- Support RTL layout via `dir` attribute (handled by `hooks.server.ts`).

---

## 3. Performance Standards

### 3.1 Rendering Rules

- **Zero unnecessary re-renders**. Use `$derived` instead of `$effect` for computed values.
- **Key lists** properly with `{#each items as item (item.id)}`.
- **Avoid `$effect`** for data transformation — use `$derived` or `$derived.by`.
- **No inline object/array creation** in component props (creates new references each render).

### 3.2 Virtualization

- Any list with >50 items must use **virtualized rendering** (`@orbis/svelte-grid`).
- Tables always use virtualization by default.
- Student lists, grade matrices, attendance grids — all virtualized.

### 3.3 Lazy Loading

- **Dialogs and modals**: Always lazy-load with `{#await import(...)}`.
- **Heavy features**: AI sidebar, search modal, upgrade dialog — already lazy-loaded.
- **Route-level code splitting**: SvelteKit handles this automatically per page.
- **Heavy components** (>10KB): Wrap in conditional `{#if}` with dynamic import.

### 3.4 IPC Usage

- **Minimize IPC calls**: Use `dedup()` for simultaneous identical requests.
- **Batch operations**: Prefer bulk APIs (`submitBulkGrades`, `recordBulkAttendance`).
- **Prefetch**: Warm TanStack Query cache before navigation.
- **staleTime**: Set appropriate values to avoid unnecessary refetches.
- **No waterfall**: Launch independent queries in parallel (TanStack handles this).

### 3.5 Memoization

- Use TanStack Query cache as the primary memoization layer.
- For expensive computations, use `$derived` (automatically memoized by Svelte 5).
- For search/filter, use the feature store's filter methods.

### 3.6 Background Tasks

- Use `fire-and-forget` pattern for non-blocking operations (e.g., `bootstrapApp()`).
- Long operations: Show progress via `$state` updates from Tauri events.
- Solver jobs: Poll via TanStack Query with `refetchInterval`.

### 3.7 Bundle Optimization

- Vendor chunks are split in `vite.config.ts`: icons, tauri, query, utils.
- `chunkSizeWarningLimit: 350` — no chunk should exceed 350KB.
- Monitor bundle size after adding new dependencies.
- Prefer tree-shakeable imports: `lucide-svelte/icons/GraduationCap` over full import.

---

## 4. UI/UX Standards

### 4.1 Loading States

- **Skeleton loading only**. Show skeleton components that match the final layout.
- **No spinners** except for inline action feedback (e.g., submit button).
- Skeleton components live in `$lib/shared/skeletons/`.
- Use `{#if $query.isLoading}` → Skeleton, `{:else}` → Content.

### 4.2 Desktop Design

- Full viewport utilization — no wasted whitespace.
- Resizable panels with `Resizer` component (widths persisted to localStorage).
- Focus mode: sidebar collapses with `Ctrl+/` toggle.
- Compact information density appropriate for desktop (not mobile).

### 4.3 Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Ctrl+/` or `⌘+/` | Toggle Orbis AI sidebar |
| `Ctrl+K` or `⌘+K` | Open search modal |
| `Escape` | Close modal/dialog |

### 4.4 Focus Management

- Dialogs trap focus within their boundaries.
- Search modal auto-focuses input on open.
- After closing a dialog, return focus to the trigger element.

### 4.5 Window Management

- Title bar is rendered only in desktop mode (`isDesktopApp()`).
- Window controls (minimize, maximize, close) are native Tauri controls.
- Breadcrumb navigation in title bar for context awareness.
- Academic period/phase badge in title bar.

### 4.6 Large Dataset Handling

- Virtualized tables for >50 rows.
- Paginated loading for reports and audit logs.
- Search with debounce (300ms default).
- Incremental loading for grade matrices.

---

## 5. Quality Standards

### 5.1 Linting & Formatting

- **Biome** for formatting and linting (`biome.json` at repo root).
- Run `bun run check` to validate TypeScript + Svelte.
- No warnings allowed in CI/CD pipeline.

### 5.2 File Size Limits

| File Type | Max Lines | Action if exceeded |
|-----------|-----------|-------------------|
| Svelte component | 400 | Split into sub-components |
| TypeScript module | 500 | Extract into separate files |
| Store class | 200 | Decompose into focused stores |
| Query file | 200 | Split by sub-domain |

### 5.3 Component Complexity

- **Max 3 levels** of conditional nesting in template.
- **Max 5 props** for simple components. Use an options/config object for more.
- **Max 10 imports** per component. If exceeded, create a barrel `index.ts`.
- **No god components**. If a component does multiple things, split it.

### 5.4 Import Rules

- **Absolute imports** only: `$lib/features/students`, never `../../features/students`.
- **Barrel exports**: Import from `index.ts`, not from internal files (cross-feature).
- **No circular imports**. If A imports B and B imports A, extract shared code to C.
- **Type-only imports**: Use `import type { X }` for types to enable tree-shaking.

### 5.5 Dead Code Prevention

- No commented-out code in committed files.
- No unused imports (enforced by Biome).
- No unused variables or parameters.
- No empty files or placeholder modules.
- Feature flags for incomplete features, not dead code.

---

## 6. AI Development Rules

### 6.1 AI-Readable Code

- Every file starts with a doc comment explaining its purpose.
- Every exported function has a JSDoc comment.
- Complex logic has inline comments explaining "why", not "what".
- File-level `@component` comment for Svelte components.

### 6.2 Naming Standards for AI

- Function names are verb-noun: `createEnrollmentsQuery`, `submitBulkGrades`.
- Query factories: `create{Resource}Query()`.
- Mutation hooks: `use{Action}{Resource}()`.
- Store methods: `verb()` — `reset()`, `filterList()`, `toggleAskOrbis()`.

### 6.3 Module Discoverability

- Every feature has an `index.ts` barrel with explicit exports.
- Queries are in `queries.ts`. Stores are in `store.svelte.ts`.
- Types are imported from `@orbis/sdk-core`, not redefined locally.
- Config is in `configs.ts` or `config.ts` within the module.

### 6.4 Self-Describing APIs

- SDK commands mirror Rust function names: `list_classes` → `commands.academic.listClasses`.
- Query keys match resource names: `["classes", orgId]`.
- Store properties match UI state: `searchQuery`, `statusFilter`, `selectedId`.

### 6.5 Orbis Autopilot Integration

- All pages support smart context via `academicContext` store.
- AI sidebar (`AskOrbisSidebar`) is available app-wide via `Ctrl+/`.
- Pages expose relevant context for AI suggestions.
- Autopilot tasks are managed through `features/ai/queries.ts`.

---

## 7. SDK Integration Rules

### 7.1 Single Source of Truth

The Rust backend (`api-rust`) is the **single source of truth** for:
- All business logic and domain rules.
- Data validation (Zod is for UX hints only).
- Permission checks.
- Data transformations and calculations.

### 7.2 SDK Package Usage

| Package | Purpose | Import Pattern |
|---------|---------|----------------|
| `@orbis/sdk-core` | Types, errors, permissions, events | `import type { Class } from "@orbis/sdk-core"` |
| `@orbis/sdk-desktop` | Tauri IPC command implementations | `import { commands } from "$lib/core/ipc"` |
| `@orbis/ui-svelte` | UI component library | `import { Button } from "@orbis/ui-svelte/button"` |
| `@orbis/svelte-grid` | Virtualized grid/table | `import { DataGrid } from "@orbis/svelte-grid"` |
| `@orbis/utils` | Shared utilities | `import { formatDate } from "@orbis/utils"` |

### 7.3 Type Safety Chain

```
Rust Entity → sea-orm Model → Tauri Command → @orbis/sdk-core Type → UI Component
```

Every type in the chain must match. If the backend adds a field, it propagates through:
1. `api-rust` entity → 2. `sdk-core` type definition → 3. Frontend component.

---

## 8. Anti-Patterns to Avoid

| Anti-Pattern | Correct Pattern |
|-------------|----------------|
| `any` type | Define proper interface or use `unknown` |
| Storing server data in `$state` | Use TanStack Query |
| `$effect` for data derivation | Use `$derived` or `$derived.by` |
| Business logic in frontend | Move to Rust backend |
| Duplicate validation | Single validation in backend |
| Cross-feature imports | Use shared/ or stores/ |
| Manual `invoke()` calls | Use `commands` from SDK |
| `String(e)` for errors | Use `extractErrorMessage(e)` |
| Spinner for page loading | Skeleton loading |
| Large monolithic components | Split at 400 lines |
| localStorage for server data | Use TanStack Query cache |
| Inline styles | Tailwind utilities |
