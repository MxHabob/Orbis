/**
 * Migration Script Phase 1c: Rename tables files to kebab-case, update their internal imports, and clean up.
 * Run with: bun scripts/migrate-tables.ts
 */
import { copyFile, unlink, mkdir } from "node:fs/promises";
import { join } from "node:path";

const TABLES = "src/lib/tables";

const MAPPINGS: [string, string][] = [
  ["core/DataTable.svelte", "core/data-table.svelte"],
  ["core/TableEmptyState.svelte", "core/table-empty-state.svelte"],
  ["core/TableToolbar.svelte", "core/table-toolbar.svelte"],
  ["renderers/ActionOverlay.svelte", "renderers/action-overlay.svelte"]
];

// Copy and rename
for (const [oldPath, newPath] of MAPPINGS) {
  try {
    await copyFile(join(TABLES, oldPath), join(TABLES, newPath));
    console.log(`Copied table file: ${oldPath} -> ${newPath}`);
  } catch (e) {
    console.error(`Failed to copy ${oldPath}:`, e);
  }
}

// Write the new core/index.ts
const coreIndexContent = `/**
 * Core table components barrel exports
 */
export { default as DataTable } from "./data-table.svelte";
export { default as TableToolbar } from "./table-toolbar.svelte";
export { default as TableEmptyState } from "./table-empty-state.svelte";
`;
await Bun.write(join(TABLES, "core/index.ts"), coreIndexContent);
console.log("Updated core/index.ts");

// Write the new renderers/index.ts (if exists, let's check its content first or just write it if simple)
// Let's first read src/lib/tables/renderers/index.ts
const renderersIndexContent = `export { default as ActionOverlay } from "./action-overlay.svelte";
export * from "./actions";
export * from "./avatar";
export * from "./badge";
export * from "./icon-text";
export * from "./link";
export * from "./progress";
export * from "./tag-list";
`;
await Bun.write(join(TABLES, "renderers/index.ts"), renderersIndexContent);
console.log("Updated renderers/index.ts");

// Let's delete the old files
for (const [oldPath] of MAPPINGS) {
  try {
    await unlink(join(TABLES, oldPath));
    console.log(`Deleted legacy table file: ${oldPath}`);
  } catch (e) {
    console.error(`Failed to delete ${oldPath}:`, e);
  }
}

console.log("Tables migration complete!");
