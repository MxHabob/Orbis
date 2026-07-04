/**
 * Migration Script Phase 1b: Copy shared components + skeletons to new locations.
 * Run with: bun scripts/migrate-shared.ts
 */
import { copyFile, mkdir } from "node:fs/promises";
import { join } from "node:path";

const LIB = "src/lib";
const OLD_SHARED = join(LIB, "components/shared");
const OLD_SKELETONS = join(LIB, "components/skeletons");
const NEW_SHARED_COMPONENTS = join(LIB, "shared/components");
const NEW_SHARED_SKELETONS = join(LIB, "shared/skeletons");

await mkdir(NEW_SHARED_COMPONENTS, { recursive: true });
await mkdir(NEW_SHARED_SKELETONS, { recursive: true });

// Shared components — already kebab-case
const sharedFiles = [
  "avatar-upload.svelte",
  "collapsible-summary.svelte",
  "copy-input.svelte",
  "display-name.svelte",
  "horizontal-pagination.svelte",
  "input-color.svelte",
  "logo-upload.svelte",
  "search-field.svelte",
  // PascalCase → kebab-case renames
  ["MultiUpPreview.svelte", "multi-up-preview.svelte"],
  ["PrintConfigPanel.svelte", "print-config-panel.svelte"],
  ["PrintDialog.svelte", "print-dialog.svelte"],
  // Settings-related ones (shared across settings pages)
  "institution-info.svelte",
  "locale-settings.svelte",
  "timezone-settings.svelte",
];

for (const entry of sharedFiles) {
  const [oldName, newName] = Array.isArray(entry) ? entry : [entry, entry];
  try {
    await copyFile(join(OLD_SHARED, oldName), join(NEW_SHARED_COMPONENTS, newName));
    console.log(`Shared: ${oldName} → ${newName}`);
  } catch (e) {
    console.error(`FAILED: ${oldName} — ${e}`);
  }
}

// Skeletons — PascalCase → kebab-case
const skeletonFiles = [
  ["GenericListSkeleton.svelte", "generic-list-skeleton.svelte"],
  ["SettingsFormSkeleton.svelte", "settings-form-skeleton.svelte"],
  ["StudentsTableSkeleton.svelte", "students-table-skeleton.svelte"],
];

for (const [oldName, newName] of skeletonFiles) {
  try {
    await copyFile(join(OLD_SKELETONS, oldName), join(NEW_SHARED_SKELETONS, newName));
    console.log(`Skeleton: ${oldName} → ${newName}`);
  } catch (e) {
    console.error(`FAILED: ${oldName} — ${e}`);
  }
}

console.log("\nDone!");
