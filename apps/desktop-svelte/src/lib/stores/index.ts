// ── Core Stores (loaded eagerly — used on every page) ─────────────
export { authStore } from "./auth.svelte";
export { themeStore } from "./theme.svelte";
export { uiStore } from "./ui.svelte";

// ── Academic Context (unified store — single source of truth)
export { academicContext } from "./academic-context.svelte";

// ── AI Context (unified store — single source of truth)
export { aiContext } from "./ai-context.svelte";

// ── Workflow Inbox (pending action counts for Title Bar badge)
export { workflowInbox } from "./workflow-inbox.svelte";

// ── Feature Stores ──────────────────────────────────────────────────
// Import feature stores directly from their feature module:
//   import { studentsStore } from "$lib/features/students/store.svelte";
//   import { examsStore } from "$lib/features/exams/store.svelte";

