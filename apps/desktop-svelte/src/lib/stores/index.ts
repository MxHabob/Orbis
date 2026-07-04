// ── Core Stores (loaded eagerly — used on every page) ─────────────
export { authStore } from "./auth.svelte";
export { themeStore } from "./theme.svelte";
export { uiStore } from "./ui.svelte";
export { academicStore } from "./academic.svelte";

// ── Feature Stores (lazy-loaded — only when their page is visited) ──
// Import these directly in the page/component that needs them:
//   import { studentsStore } from "$lib/stores/students.svelte";
//   import { examsStore } from "$lib/stores/exams.svelte";
//
// Re-exported here for backward compatibility with existing imports:
export { studentsStore } from "./students.svelte";
export { examsStore } from "./exams.svelte";
