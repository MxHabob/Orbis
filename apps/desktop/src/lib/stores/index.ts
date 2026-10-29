// ── Core Stores (loaded eagerly — used on every page) ─────────────
export { authStore } from "./auth.svelte";
export { systemStore } from "./system.svelte";
export { licenseStore } from "./license.svelte";
export { themeStore } from "./theme.svelte";
export { appearanceStore } from "./appearance.svelte";
export { uiStore } from "./ui.svelte";
export { confirmStore } from "./confirm.svelte";
export { useUnsavedGuard } from "./unsaved-guard.svelte";

// ── Academic Context (unified store — single source of truth)
export { academicContext } from "./academic-context.svelte";

// ── Organization Context (org configuration — locale, timezone, currency, branding)
export { orgContext } from "./org-context.svelte";

// ── AI Context (unified store — single source of truth)
export { aiContext } from "./ai-context.svelte";

// ── Workflow Inbox (pending action counts for Title Bar badge)
export { workflowInbox } from "./workflow-inbox.svelte";

// ── Notification Store
export { notificationStore } from "./notification.svelte";

// ── Effective Context & Permissions
export {
  effectiveContext,
  type ScopeLevel,
  type ScopeFilter,
} from "./effective-context.svelte";

// ── Print Dialog Store
export {
  printState,
  openPrintDialog,
  closePrintDialog,
} from "./print.svelte";

// ── Feature Stores ──────────────────────────────────────────────────
// Import feature stores directly from their feature module:
//   import { studentsStore } from "$lib/features/students/store.svelte";
//   import { examsStore } from "$lib/features/exams/store.svelte";
