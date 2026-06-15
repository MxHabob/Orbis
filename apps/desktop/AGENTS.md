# Orbis Desktop — AI Agent Migration & Cleanup Playbook

> **Scope:** `apps/desktop/` only. This app is **not** Next.js. It is a **Vite + React 19 + React Router 6 + Tauri v2** client that talks to Rust via `@orbis/desktop-client` (IPC). Treat leftover Next.js/dashboard patterns as **debt to remove or migrate**, not as conventions to follow.

---

## 1. Target architecture (2026)

```
Browser/Tauri WebView
  └─ main.tsx → App.tsx
       ├─ QueryClientProvider (@tanstack/react-query)
       ├─ BrowserRouter (react-router-dom)
       ├─ I18nProvider (react-i18next via @/i18n + @/locales/client shim)
       ├─ ThemeProvider
       └─ DesktopShell → AppRouter (lazy routes + lazy layouts)
            ├─ @orbis/desktop-client/commands  (typed IPC)
            ├─ @orbis/desktop-client/platform  (Tauri detection, deep links)
            └─ hooks/api/*                     (preferred data layer)
```

**Hard rules**

| Do | Don't |
|----|--------|
| `react-router-dom` (`Link`, `Navigate`, `useNavigate`, `useLocation`) | `next/link`, `next/navigation`, `useRouter` |
| `React.lazy` + `Suspense` or `@/lib/lazy-component` | `next/dynamic` |
| `@tanstack/react-query` + `hooks/api/*` | `useTRPC`, `@/trpc/*`, tRPC client |
| `@orbis/desktop-client/commands` | Embedded Axum/HTTP server, `@trpc/*` |
| `@/hooks/use-safe-action` for local async UI actions | `next-safe-action`, `@/actions/*` server actions |
| `react-i18next` via `@/locales/client` | `next-international`, `next-intl` |
| Workspace packages (`@orbis/ui`, `@orbis/export`, …) | Vite `resolve.alias` shims for monorepo packages |
| Tailwind v4 via `@/styles/tailwind.css` + `@tailwindcss/vite` | `tailwind.config.ts` v3 presets, `@tailwind` directives in app CSS |

---

## 2. Legacy signals — grep & fix

Run these audits under `apps/desktop/src`:

```bash
# Next.js / RSC leftovers
rg '"use client"' apps/desktop/src
rg "from ['\"]next/" apps/desktop/src
rg "next-safe-action|next/image|next/head|server-only" apps/desktop/src

# Old data layer
rg "useTRPC|@/trpc/|@/ipc/query-adapter" apps/desktop/src

# Server Actions (Next-only)
rg "from ['\"]@/actions/" apps/desktop/src

# Wrong package names (renamed monorepo)
rg "@authen/" apps/desktop/src

# Dashboard-only HTTP assumptions
rg "/api/|metadataBase|generateMetadata|revalidatePath|revalidateTag" apps/desktop/src
```

### Replacement map

| Legacy pattern | Desktop replacement |
|----------------|---------------------|
| `"use client"` at top of file | **Delete** (no-op in Vite) |
| `dynamic(() => import(...))` | `lazy(() => import(...))` + `<Suspense>` or `lazyNamed` / `withSuspense` from `@/lib/lazy-component` |
| `useTRPC()` / `trpc.*.queryOptions` | Hook from `@/hooks/api/*` or temporary `@/ipc/query-adapter` → **migrate to hooks/api** |
| `useAction(serverAction)` | `useSafeAction` from `@/hooks/use-safe-action` + IPC mutation or inline async |
| `@/actions/foo-action` | Rust command via `commands.*` or stub in `@/actions/*` until IPC exists — **prefer deleting stub once IPC wired** |
| `authClient` from Next Auth | `@/hooks/use-auth` / `@/lib/auth-client` (session + `commands.auth.*`) |
| `next/image` | `<img>` or `@orbis/ui` patterns |
| `redirect()` / `notFound()` | `<Navigate to="..." />` or route guards (`AuthGuard`) |
| `@/locales/client` | **Keep** — thin shim over `react-i18next` (already migrated off next-international) |

---

## 3. File classification

For every flagged file, assign **one** bucket:

| Bucket | Action |
|--------|--------|
| **A — Active & correct** | No change; uses IPC/hooks/api/Router |
| **B — Migrate** | Replace legacy imports; keep UI; point data layer to `hooks/api` |
| **C — Legacy feature** | Banking, GoCardless, Enable Banking, tracker billing, team invites, inbox — move under `src/legacy/` and exclude from router/menus unless IPC exists |
| **D — Dead** | No imports, duplicate of dashboard-only code — **delete** |
| **E — Stub** | `@/actions/*`, empty server action — replace with real IPC or remove caller |

**Do not** copy new patterns from `apps/dashboard/` without translating to desktop stack.

---

## 4. Cleanup workflow (agent execution order)

### Phase 0 — Safety

1. `cd apps/desktop && bun run build` — must pass before/after each batch.
2. Never break boot path: `main.tsx` → `App.tsx` → `DesktopShell` → `LoginPage` (`/login`).

### Phase 1 — Boot-critical (eager imports)

Fix files in the **static import graph** of `router.tsx` and layouts:

- Layouts must stay **lazy** (`lazyLayout` in `router.tsx`).
- No `next/dynamic`, no missing `@/actions/*`, no `useTRPC` in login/shell path.

### Phase 2 — Data layer migration (batch by domain)

For each domain (`auth`, `classes`, `students`, `grades`, …):

1. Ensure `hooks/api/use-<domain>.ts` wraps `commands.*` + `queryKeys`.
2. Replace `useTRPC()` calls in components with the domain hook.
3. When domain has zero `query-adapter` references, mark done.

### Phase 3 — Remove Next.js artifacts

- Strip all `"use client"` lines.
- Delete unused `@/actions/**` stubs after IPC migration.
- Remove `query-adapter.ts` when grep shows zero imports.

### Phase 4 — Legacy gating

- Move unsupported features to `src/legacy/**`.
- Remove routes from `router.tsx` and entries from `main-menu.tsx`.
- Add comment header: `/** @legacy — not supported in desktop IPC yet */`

### Phase 5 — Verify

```bash
cd apps/desktop
bun run build
bun run typecheck   # when ts errors are in scope
bun run tauri dev   # login screen renders; F12 console clean on /login
```

---

## 5. IPC & hooks conventions

```typescript
// ✅ Preferred: domain hook
import { useStudents } from "@/hooks/api/use-enrollments";

// ✅ Low-level IPC
import { commands } from "@orbis/desktop-client/commands";

// ⚠️ Temporary only — do not add new usages
import { useTRPC } from "@/ipc/query-adapter";
```

New features **must** add:

1. Rust command in `apps/desktop/src-tauri/src/commands/`
2. Entry in `@orbis/desktop-client` bindings
3. `hooks/api/use-*.ts` with stable `queryKeys`

---

## 6. Styling

- Entry CSS: `@/styles/tailwind.css` (imports `@orbis/ui/globals.css`, `@source` for UI package).
- App overrides: `@/styles/globals.css`.
- Desktop variants: `@orbis/desktop-client/desktop-variants.css` (`desktop:`, `mac:`, …).
- **Never** reintroduce `tailwind.config.ts` v3 or PostCSS `tailwindcss` plugin v3.

---

## 7. PR / commit guidance for agents

- One domain or one cleanup phase per PR.
- Title format: `desktop: migrate <domain> off query-adapter` or `desktop: remove Next.js leftovers in <area>`.
- Include grep output before/after for the pattern being eliminated.
- If deleting >10 files, list them in PR body.

---

## 8. Out of scope (do not refactor unless asked)

- `apps/dashboard/` (Next.js — separate playbook)
- `packages/ui` design tokens / Tailwind v4 theme
- Rust/SQLite schema except when required by a migrated hook
- Git history rewrites

---

## 9. Quick reference — package boundaries

| Package | Role |
|---------|------|
| `@orbis/desktop-client` | IPC commands, events, platform, desktop Tailwind variants |
| `@orbis/ui` | Shared UI components + globals.css (Tailwind v4) |
| `@orbis/export` / `@orbis/import` | CSV/PDF client utilities |
| `@orbis/location` | Countries, currencies, timezones |
| `@orbis/utils` | Shared non-UI helpers |

---

## 10. Definition of done (desktop cleanup)

- [ ] Zero `next/` imports in `apps/desktop/src`
- [ ] Zero `next-safe-action` imports
- [ ] Zero `"use client"` directives
- [ ] Zero `useTRPC` / `@/ipc/query-adapter` (or explicit ticket for remaining domains)
- [ ] Zero `@/actions/*` stubs without tracking issue
- [ ] `bun run build` passes
- [ ] `/login` renders in `tauri dev`
