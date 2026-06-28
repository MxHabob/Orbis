# Orbis — AI Agent Rules

> **هذا الملف إلزامي لأي وكيل ذكاء اصطناعي يعمل على المشروع.**
> This file is mandatory for any AI agent working on this project.

---

## 1. Design System: S.C.R.A.P.S. (`@orbis/ui-svelte`)

Orbis uses a custom **shadcn-svelte** design system called **S.C.R.A.P.S.**, defined in `packages/ui-svelte/`. All pages MUST use these components exclusively. **Never build custom elements that duplicate existing components.**

### 1.1 Available Components

Import components using the `@orbis/ui-svelte/<component>` path:

| Category | Components | Import Path |
|----------|-----------|-------------|
| **Buttons** | `Button` | `@orbis/ui-svelte/button` |
| **Inputs** | `Input` | `@orbis/ui-svelte/input` |
| **Textarea** | `Textarea` | `@orbis/ui-svelte/textarea` |
| **Labels** | `Label` | `@orbis/ui-svelte/label` |
| **Select** | `Select, SelectTrigger, SelectContent, SelectItem, SelectGroup, SelectLabel, SelectSeparator` | `@orbis/ui-svelte/select` |
| **Checkbox** | `Checkbox` | `@orbis/ui-svelte/checkbox` |
| **Switch** | `Switch` | `@orbis/ui-svelte/switch` |
| **Badge** | `Badge` | `@orbis/ui-svelte/badge` |
| **Avatar** | `Avatar, AvatarImage, AvatarFallback` | `@orbis/ui-svelte/avatar` |
| **Tabs** | `Tabs, TabsList, TabsTrigger, TabsContent` | `@orbis/ui-svelte/tabs` |
| **Dialog** | `Dialog, DialogContent, DialogHeader, DialogFooter, DialogTitle, DialogDescription` | `@orbis/ui-svelte/dialog` |
| **AlertDialog** | `AlertDialog, AlertDialogTrigger, AlertDialogContent, AlertDialogHeader, AlertDialogFooter, AlertDialogTitle, AlertDialogDescription, AlertDialogAction, AlertDialogCancel` | `@orbis/ui-svelte/alert-dialog` |
| **Dropdown** | `DropdownMenu, DropdownMenuTrigger, DropdownMenuContent, DropdownMenuItem, DropdownMenuSeparator, ...` | `@orbis/ui-svelte/dropdown-menu` |
| **Card** | `Card, CardHeader, CardFooter, CardTitle, CardDescription, CardContent` | `@orbis/ui-svelte/card` |
| **Table** | `Table, TableBody, TableHead, TableHeader, TableRow, TableCell, ...` | `@orbis/ui-svelte/table` |
| **Skeleton** | `Skeleton` | `@orbis/ui-svelte/skeleton` |
| **Separator** | `Separator` | `@orbis/ui-svelte/separator` |
| **Tooltip** | `Tooltip, TooltipTrigger, TooltipContent, TooltipProvider` | `@orbis/ui-svelte/tooltip` |
| **Popover** | `Popover, PopoverTrigger, PopoverContent` | `@orbis/ui-svelte/popover` |
| **Sheet** | `Sheet, SheetTrigger, SheetContent, SheetHeader, SheetFooter, SheetTitle, SheetDescription` | `@orbis/ui-svelte/sheet` |
| **Breadcrumb** | `Breadcrumb, BreadcrumbList, BreadcrumbItem, BreadcrumbLink, BreadcrumbPage, BreadcrumbSeparator` | `@orbis/ui-svelte/breadcrumb` |
| **Progress** | `Progress` | `@orbis/ui-svelte/progress` |
| **Calendar** | `Calendar` | `@orbis/ui-svelte/calendar` |
| **ScrollArea** | `ScrollArea, Scrollbar` | `@orbis/ui-svelte/scroll-area` |
| **Combobox** | `ComboboxDropdown` | `@orbis/ui-svelte/combobox-dropdown` |
| **Accordion** | `Accordion, AccordionContent, AccordionItem, AccordionTrigger` | `@orbis/ui-svelte/accordion` |

### 1.2 Button Variants

```svelte
<Button variant="default">Primary Action</Button>
<Button variant="lime">Save / Confirm</Button>
<Button variant="outline">Secondary Action</Button>
<Button variant="ghost">Subtle Action</Button>
<Button variant="destructive">Delete / Danger</Button>
<Button variant="secondary">Alternate</Button>
<Button variant="link">Link Style</Button>
```

**Sizes:** `default` | `sm` | `xs` | `lg` | `icon` | `icon-sm` | `icon-xs` | `icon-lg`

### 1.3 Badge Variants

```svelte
<Badge variant="default">Default</Badge>
<Badge variant="lime">Active / Success</Badge>
<Badge variant="destructive">Error / Danger</Badge>
<Badge variant="secondary">Neutral</Badge>
<Badge variant="outline">Outlined</Badge>
<Badge variant="brand">Brand</Badge>
<Badge variant="ghost">Ghost</Badge>
```

---

## 2. Styling Rules — STRICTLY ENFORCED

### ✅ DO — Use Semantic Design Tokens

```css
/* Backgrounds */
bg-background    bg-card    bg-muted    bg-muted/30    bg-accent    bg-popover    bg-input/30

/* Text */
text-foreground    text-muted-foreground    text-card-foreground    text-primary

/* Borders */
border-border    border-border/60    border-input

/* Brand */
bg-primary    text-primary    text-primary-foreground
bg-destructive    text-destructive
```

### ❌ NEVER — Use Custom/Hardcoded Colors

```css
/* FORBIDDEN — these break theme switching: */
bg-[#131316]    bg-[#17171c]    bg-[#1a1a2e]    text-[#a3e635]
border-[#2a2a3a]    border-border/20    bg-zinc-900    text-gray-400
```

### ❌ NEVER — Use Native HTML Elements When Components Exist

| ❌ Don't Use | ✅ Use Instead |
|-------------|---------------|
| `<select>` with manual styling | `<Select><SelectTrigger><SelectContent><SelectItem>` |
| `<input type="checkbox">` | `<Checkbox>` |
| `<input type="text">` with manual styling | `<Input>` |
| `<textarea>` with manual styling | `<Textarea>` |
| Manual `<div>` modal overlays | `<Dialog>` or `<AlertDialog>` |
| Manual `<div>` tab switchers | `<Tabs><TabsList><TabsTrigger><TabsContent>` |
| Manual toggle buttons | `<Switch>` |
| `<label>` | `<Label>` |
| `<button>` with custom classes | `<Button variant="...">` |

---

## 3. Page Layout Patterns

### 3.1 Settings Sub-Pages (Standard Pattern)

Settings pages use the shared `+layout.svelte` which provides a sidebar with a `MainViewport` (resizable). The child page receives full height/width:

```svelte
<div class="h-full w-full overflow-y-auto bg-background/50 scrollbar-hide">
  <div class="max-w-4xl mx-auto p-6 flex flex-col gap-6">
    <!-- Page Header -->
    <div class="flex flex-col gap-1 text-start">
      <h1 class="text-xl font-bold tracking-tight text-foreground">{title}</h1>
      <p class="text-xs text-muted-foreground">{description}</p>
    </div>

    <!-- Content Sections -->
    <div class="bg-card border border-border rounded-xl p-6 flex flex-col gap-6">
      <div class="flex flex-col gap-1 text-start border-b border-border/60 pb-4">
        <h2 class="text-sm font-semibold text-foreground">{sectionTitle}</h2>
      </div>
      <!-- Form fields using grid layout -->
      <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
        <!-- Fields with Label + Input pairs -->
      </div>
    </div>

    <!-- Action Buttons -->
    <div class="flex justify-end gap-3 pb-8">
      <Button variant="lime" type="submit">{saveLabel}</Button>
    </div>
  </div>
</div>
```

### 3.2 Form Field Pattern

```svelte
<div class="flex flex-col gap-2 text-start">
  <label for="field_id" class="text-xs font-medium text-muted-foreground">
    {m.field_label()}
  </label>
  <Input id="field_id" type="text" bind:value={fieldValue} />
</div>
```

### 3.3 Header Action Bar (When Needed)

Keep action buttons in a **single non-wrapping row**. Use `flex-nowrap` and `overflow-x-auto` to prevent layout breaking on smaller screens:

```svelte
<div class="flex items-center justify-between gap-4">
  <h1 class="text-lg font-bold text-foreground truncate">{title}</h1>
  <div class="flex items-center gap-2 flex-nowrap shrink-0">
    <Button variant="outline" size="sm">Action</Button>
    <Button variant="lime" size="sm">Save</Button>
  </div>
</div>
```

---

## 4. Localization (i18n) — Paraglide-JS

### 4.1 Translation Keys

- All user-visible text MUST use Paraglide compiled messages: `m.key_name()`
- Translation files: `apps/desktop-svelte/messages/en.json` and `ar.json`
- Import: `import * as m from "$lib/paraglide/messages"`
- Locale check: `import { getLocale } from "$lib/paraglide/runtime"`

### 4.2 Rules

1. **NEVER hardcode English or Arabic strings in templates.** Use `m.key()` functions.
2. If a key doesn't exist, **add it to BOTH `en.json` AND `ar.json`** before using `m.key()`.
3. **NEVER use `getLocale() === "ar" ? "..." : "..."` inline ternaries** — always create proper translation keys.
4. Key naming convention: `section_subsection_element` (e.g., `settings_user_detail_label_email`)
5. Compile messages after adding keys so `m.<key>()` functions become available.

### 4.3 Text Direction (RTL/LTR)

- The settings layout already sets `dir={getLocale() === "ar" ? "rtl" : "ltr"}` on the container.
- Use direction-neutral classes: `text-start` (not `text-left`), `ps-4` (not `pl-4`), `me-2` (not `mr-2`).
- For directional icons (arrows), use RTL-aware classes: `rtl:rotate-180`.

---

## 5. Error Handling

### 5.1 Tauri IPC Errors

Tauri `invoke()` throws **plain objects** matching the `CommandError` interface `{ message: string; code?: string }`. They are NOT `Error` instances.

```typescript
// ✅ CORRECT — handles both Error instances and Tauri CommandError objects:
catch (e: unknown) {
  const msg = (e as any)?.message || "Unknown error";
  toast.error(msg);
}

// ❌ WRONG — String(e) on a plain object gives "[object Object]":
catch (e) {
  toast.error(String(e));  // ← Shows "[object Object]"
}

// ❌ WRONG — Tauri errors are NOT Error instances:
catch (e) {
  if (e instanceof Error) { ... }  // ← Never true for Tauri IPC errors
}
```

---

## 6. Design Philosophy

### 6.1 Visual Principles

- **Elegant simplicity** — Pages must look premium and professional. Avoid clutter.
- **Clean spacing** — Use consistent gaps (`gap-6` for sections, `gap-2` for field groups).
- **Subtle hierarchy** — Use `text-foreground` for primary text, `text-muted-foreground` for secondary.
- **Soft borders** — Use `border-border` with `rounded-xl` on cards.
- **No visual noise** — Remove unnecessary dividers, shadows, or decorative elements.

### 6.2 Layout Principles

- **Never let buttons wrap to a new line.** Use `flex-nowrap shrink-0` on button containers.
- **Max-width content areas** (`max-w-4xl mx-auto`) for readability.
- **Scrollable content** — Use `overflow-y-auto scrollbar-hide` on main content areas.
- **Responsive grids** — Use `grid grid-cols-1 md:grid-cols-2` for form layouts.

### 6.3 What NOT to Build

- ❌ Custom sidebar action lists (Assigned To, Attachments, Tags, Share, Follow buttons) on detail pages — these are project tracker patterns, not settings pages.
- ❌ Hearts / Comments / Follow buttons on settings pages.
- ❌ Fake "auditing metadata" timestamps (e.g., "You edited this 21 min ago").
- ❌ Duplicate avatars (one big + one small) on the same sidebar.
- ❌ Print / Next / Previous navigation buttons unless explicitly requested.
- ❌ Multiple save buttons across different tabs — one primary save action is enough.

---

## 7. IPC / Backend

- Desktop app uses **Tauri v2** with Rust backend.
- All API calls go through `import { commands } from "$lib/services/ipc"`.
- Types: `import type { User } from "$lib/services/ipc"`.
- Commands mirror `#[tauri::command]` functions in `src-tauri/`.

---

## 8. Tech Stack Summary

| Layer | Technology |
|-------|-----------|
| Framework | SvelteKit 5 (Svelte 5 with Runes) |
| Desktop | Tauri v2 |
| Styling | Tailwind CSS v4 |
| Components | shadcn-svelte (`@orbis/ui-svelte`) |
| i18n | Paraglide-JS |
| Icons | lucide-svelte |
| Toasts | svelte-sonner |
| State | Svelte 5 Runes (`$state`, `$derived`, `$effect`) |
| Fonts | Rubik (via @fontsource-variable) |
