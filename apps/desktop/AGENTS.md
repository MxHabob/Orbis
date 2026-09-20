# Orbis Desktop — Agent Instructions

This is a **SvelteKit + Tauri 2.0** desktop application (NOT Expo/React Native). It builds a native desktop app with a Svelte 5 frontend and a Rust backend embedded via Tauri.

## Technology Stack

- **Frontend**: SvelteKit 2 with Svelte 5 (runes, snippets, `$state`, `$derived`)
- **Backend (embedded)**: Rust via Tauri 2.0 IPC commands
- **UI Library**: `@orbis/ui` (internal, shadcn-svelte based)
- **State Management**: Svelte 5 runes (`$state`, `$derived`) + TanStack Query
- **Styling**: TailwindCSS 4
- **Internationalization**: Paraglide.js (compile-time i18n)
- **API Client**: Dual-transport `OrbisApiClient` — `LocalClient` (Tauri IPC) and `RemoteClient` (HTTP via `@orbis/sdk`)

## Commands

This project uses `bun` as the package manager (`bun.lock` is present).

```bash
bun install                              # install dependencies
bun run dev                              # start SvelteKit dev server (frontend only)
cargo tauri dev                          # start full Tauri desktop app (frontend + Rust)
bun x tsc --noEmit                       # typecheck
bun x svelte-check --tsconfig tsconfig.json  # svelte-specific checks
cargo check --manifest-path src-tauri/Cargo.toml  # check Rust backend
```

Run typecheck before declaring any task done.

## Architecture

- **Routes**: `src/routes/` — SvelteKit file-based routing. `(app)/[orgId]/` is the authenticated layout.
- **Stores**: `src/lib/stores/` — Svelte 5 rune-based stores (`$state`, `$derived`). Key stores:
  - `auth.svelte.ts` — authentication state
  - `effective-context.svelte.ts` — role/permission/scope context
  - `org-context.svelte.ts` — organization configuration (locale, timezone, currency)
  - `academic-context.svelte.ts` — academic year/period state
  - `system.svelte.ts` — connection and readiness state
- **API Layer**: `src/lib/api/` — `types.ts` defines the `OrbisApiClient` interface; `local-client.ts` (Tauri IPC) and `remote-client.ts` (HTTP SDK) implement it.
- **Features**: `src/lib/features/` — domain feature modules (students, onboarding, templates, etc.)
- **Shell**: `src/lib/shell/` — application chrome (sidebar, titlebar, search, etc.)
- **Tauri Commands**: `src-tauri/src/commands/` — Rust IPC command handlers

## Rules

- **Backend is authoritative**: Never fabricate or hardcode data. Desktop consumes backend state.
- **No `as any` casts**: Use properly typed access. If the SDK type is missing fields, fix the server DTO and regenerate.
- **Use `OrbisApiClient`**: All data access goes through `$lib/api/client.ts`. Never call `invoke()` directly from components.
- **Svelte 5 only**: Use `$state`, `$derived`, `$effect`, `$props`. No legacy `$:` reactive statements.
- **Organization context**: The `orgContext` store is the source of truth for locale, timezone, and currency. Use `formatDate()`, `formatCurrency()`, `formatNumber()` from `$lib/core/i18n/format.ts` — never use raw `Intl` constructors in components.
- **Permissions**: Use `<PermissionGate>` component or `effectiveContext.hasPermission()`. Never hardcode role checks.
- Read the Svelte 5 skill (`svelte-code-writer`) before editing any `.svelte` file.
- Read the Tauri skill (`tauri`) before editing any Rust IPC command.
