// ── Core Stores (loaded eagerly — used on every page) ─────────────
export { authStore } from "./auth.svelte";
export { themeStore } from "./theme.svelte";
export { uiStore } from "./ui.svelte";

// ── Academic Context (unified store — replaces separate year + academic stores)
export { academicContext, academicYearStore } from "./academic-context.svelte";

// ── Backward Compatibility: re-export old academicStore from new context ──
// The old `academicStore` had selectedCycleId/selectedClassId/selectedSubjectId
// which are now part of `academicContext`. Re-export for gradual migration.
export { academicStore } from "$lib/features/academics/store.svelte";

// ── Feature Stores (lazy-loaded — only when their page is visited) ──
// Import these directly in the page/component that needs them:
//   import { studentsStore } from "$lib/features/students/store.svelte";
//   import { examsStore } from "$lib/features/exams/store.svelte";
//
// Re-exported here for backward compatibility with existing imports:
export { studentsStore } from "$lib/features/students/store.svelte";
export { examsStore } from "$lib/features/exams/store.svelte";
