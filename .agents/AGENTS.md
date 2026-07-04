# Orbis — Agent Rules

> Mandatory for all AI agents. Follow every rule without exception.

---

## 1. Components — Use `@orbis/ui-svelte` Only

Import: `@orbis/ui-svelte/<component>`

- `Button` (variants: `default|lime|outline|ghost|destructive|secondary|link`, sizes: `default|sm|xs|lg|icon|icon-sm|icon-xs|icon-lg`)
- `Input`, `Textarea`, `Label`, `Checkbox`, `Switch`
- `Select`, `SelectTrigger`, `SelectContent`, `SelectItem`, `SelectGroup`, `SelectLabel`, `SelectSeparator`
- `Badge` (variants: `default|lime|destructive|secondary|outline|brand|ghost`)
- `Avatar`, `AvatarImage`, `AvatarFallback`
- `Tabs`, `TabsList`, `TabsTrigger`, `TabsContent`
- `Dialog`, `DialogContent`, `DialogHeader`, `DialogFooter`, `DialogTitle`, `DialogDescription`
- `AlertDialog`, `AlertDialogTrigger`, `AlertDialogContent`, `AlertDialogHeader`, `AlertDialogFooter`, `AlertDialogTitle`, `AlertDialogDescription`, `AlertDialogAction`, `AlertDialogCancel`
- `DropdownMenu`, `DropdownMenuTrigger`, `DropdownMenuContent`, `DropdownMenuItem`, `DropdownMenuSeparator`
- `Card`, `CardHeader`, `CardFooter`, `CardTitle`, `CardDescription`, `CardContent`
- `Table`, `TableBody`, `TableHead`, `TableHeader`, `TableRow`, `TableCell`
- `Skeleton`, `Separator`, `Progress`, `Calendar`
- `Tooltip`, `TooltipTrigger`, `TooltipContent`, `TooltipProvider`
- `Popover`, `PopoverTrigger`, `PopoverContent`
- `Sheet`, `SheetTrigger`, `SheetContent`, `SheetHeader`, `SheetFooter`, `SheetTitle`, `SheetDescription`
- `Breadcrumb`, `BreadcrumbList`, `BreadcrumbItem`, `BreadcrumbLink`, `BreadcrumbPage`, `BreadcrumbSeparator`
- `ScrollArea`, `Scrollbar`
- `ComboboxDropdown`
- `Accordion`, `AccordionContent`, `AccordionItem`, `AccordionTrigger`

**NEVER** use native `<select>`, `<input>`, `<textarea>`, `<button>`, `<label>`, or manual modal/tab `<div>`s when a component exists above.

---

## 2. Styling

**DO:** `bg-background`, `bg-card`, `bg-muted`, `bg-accent`, `bg-popover`, `text-foreground`, `text-muted-foreground`, `text-primary`, `border-border`, `border-border/60`, `bg-primary`, `text-primary-foreground`, `bg-destructive`, `text-destructive`

**NEVER:** hardcoded colors (`bg-[#131316]`, `bg-zinc-900`, `text-gray-400`, `text-[#a3e635]`, `border-[#2a2a3a]`). They break theme switching.

**Direction:** use `text-start` not `text-left`, `ps-4` not `pl-4`, `me-2` not `mr-2`, `rtl:rotate-180` for arrows.

---

## 3. Layout Patterns

**Settings pages:**
```svelte
<div class="h-full w-full overflow-y-auto bg-background/50 scrollbar-hide">
  <div class="max-w-4xl mx-auto p-6 flex flex-col gap-6">
    <!-- header → card sections → action buttons -->
  </div>
</div>
```

**Rules:** `flex-nowrap shrink-0` on button rows. `max-w-4xl mx-auto` for content. `overflow-y-auto scrollbar-hide` for scrollable areas. `grid grid-cols-1 md:grid-cols-2 gap-6` for forms.

**NEVER build:** sidebar action lists, hearts/comments/follow buttons, fake timestamps, duplicate avatars, print/next/prev buttons, multiple save buttons.

---

## 4. i18n — Paraglide-JS

- All user text → `m.key_name()` from `import * as m from "$lib/paraglide/messages"`
- Add keys to BOTH `messages/en.json` AND `messages/ar.json`
- Key format: `section_subsection_element`
- NEVER hardcode strings. NEVER use `getLocale() === "ar" ? "..." : "..."` ternaries.

---

## 5. Error Handling

```typescript
import { extractErrorMessage } from "$lib/utils/ipc-error";
catch (e: unknown) { toast.error(extractErrorMessage(e)); }
```

**NEVER:** `String(e)` → gives `[object Object]`. `e instanceof Error` → never true for Tauri IPC.

---

## 6. IPC / Backend

- Tauri v2 with Rust backend
- `import { commands } from "$lib/services/ipc"` — all IPC calls
- `import type { User } from "$lib/services/ipc"` — all types

---

## 7. Tech Stack

SvelteKit 5 (Runes) · Tauri v2 · Tailwind CSS v4 · shadcn-svelte (`@orbis/ui-svelte`) · Paraglide-JS · lucide-svelte · svelte-sonner · TanStack Query (`@tanstack/svelte-query`) · Rubik font

---

## 8. Performance — STRICTLY ENFORCED

### 8.1 Data Fetching → TanStack Query

All IPC data MUST use TanStack Query. Query hooks live in `src/lib/services/queries/`:

| File | Domain |
|------|--------|
| `queries/academic.ts` | Cycles, classes, subjects |
| `queries/students.ts` | Enrollments |
| `queries/finance.ts` | Fees, treasuries, payments |
| `queries/dashboard.ts` | Dashboard stats |

```typescript
// ✅ DO:
import { createQuery } from "@tanstack/svelte-query";
export function createClassesQuery(orgId: string) {
  return createQuery({
    queryKey: ["classes", orgId],
    queryFn: () => commands.classes.getAll(orgId),
    enabled: !!orgId,
    staleTime: 5 * 60 * 1000,
  });
}
// ❌ NEVER: raw commands.* in onMount without caching
```

QueryClient config in `src/lib/services/query-client.ts`: staleTime 5min, gcTime 30min, refetchOnWindowFocus false, networkMode always.

### 8.2 Stores → UI State Only

- **UI Stores** (`$lib/stores/`): `authStore`, `uiStore`, `themeStore`, `academicStore` — toggles, filters, selections
- **Data Queries** (`$lib/services/queries/`): server data with caching

NEVER store fetched server data in `$state` lists inside stores.

### 8.3 IPC Deduplication

```typescript
import { dedup } from "$lib/services/ipc-dedup";
await dedup(`cycles:${orgId}`, () => commands.academics.getCycles(orgId));
```

### 8.4 Lazy Loading

Components >5KB that are conditionally visible → `{#await import(...)}`:

```svelte
{#if visible}
  {#await import("./HeavyComponent.svelte") then { default: C }}
    <C />
  {/await}
{/if}
```

Currently lazy: `SearchModal` (15KB), `AskOrbisSidebar` (8.6KB), `UpgradeDialog` (7.5KB).

### 8.5 Skeletons

Every data page → layout-matching skeleton during loading. Available in `src/lib/components/skeletons/`:
- `StudentsTableSkeleton` — data tables
- `SettingsFormSkeleton` — settings forms
- `GenericListSkeleton` — list pages (accepts `rows` prop)

NEVER use spinners. NEVER show blank space.

### 8.6 Debounced Search

```typescript
import { debounce } from "$lib/utils/debounce";
let localSearch = $state("");
const updateSearch = debounce((q: string) => { store.searchQuery = q; }, 150);
// <Input bind:value={localSearch} oninput={() => updateSearch(localSearch)} />
```

NEVER `bind:value={store.searchQuery}` directly on large datasets.

### 8.7 Prefetching

```typescript
import { prefetchRouteData } from "$lib/services/prefetch";
// On sidebar mouseenter:
onmouseenter={() => prefetchRouteData("students", orgId)}
```

### 8.8 Layout Stats → onMount Only

```typescript
// ✅ DO:
onMount(() => { if (data.orgId) loadSidebarStats(); });
// ❌ NEVER: $effect tracking page.url.pathname → refetches on every sub-route
```

### 8.9 Boot → Non-Blocking

```typescript
authStore.checkSetupStatus(); // fire-and-forget, no await
```

### 8.10 Imports

- Static import for `commands`, `extractErrorMessage` (lightweight, always needed)
- NEVER `await import("$lib/services/ipc")` — unnecessary overhead

### 8.11 DOM Reads → Cache

```typescript
let _cached: number | null = null;
function getValue(): number {
  _cached ??= parseFloat(getComputedStyle(document.documentElement).fontSize);
  return _cached;
}
```

### 8.12 Anti-Pattern Summary

| ❌ NEVER | ✅ DO |
|----------|------|
| `await` IPC in layout `load()` for non-critical data | Fire-and-forget |
| `$effect` + `page.url.pathname` for data refetch | `onMount` once |
| `e instanceof Error` / `String(e)` | `extractErrorMessage(e)` |
| `bind:value={store.searchQuery}` on large data | `debounce()` |
| Import heavy components unconditionally | `{#await import(...)}` |
| `getComputedStyle()` per call | Cache with `??=` |
| Multiple identical concurrent IPC calls | `dedup()` |
| Raw `commands.*` without cache | TanStack Query |
| Spinners / blank loading | Skeleton components |
