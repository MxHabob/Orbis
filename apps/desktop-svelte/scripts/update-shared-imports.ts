/**
 * Migration Script: Update all shared/skeleton import paths across the project.
 * Run with: bun scripts/update-shared-imports.ts
 */
import { readdir, readFile, writeFile } from "node:fs/promises";
import { join, extname } from "node:path";

const SRC_DIR = "src";
const EXTENSIONS = new Set([".svelte", ".ts", ".js"]);

const IMPORT_MAPPINGS: [string, string][] = [
  // Skeletons
  ['$lib/components/skeletons/GenericListSkeleton.svelte', '$lib/shared/skeletons/generic-list-skeleton.svelte'],
  ['$lib/components/skeletons/SettingsFormSkeleton.svelte', '$lib/shared/skeletons/settings-form-skeleton.svelte'],
  ['$lib/components/skeletons/StudentsTableSkeleton.svelte', '$lib/shared/skeletons/students-table-skeleton.svelte'],
  ['$lib/components/skeletons/', '$lib/shared/skeletons/'],

  // Shared Components
  ['$lib/components/shared/MultiUpPreview.svelte', '$lib/shared/components/multi-up-preview.svelte'],
  ['$lib/components/shared/PrintConfigPanel.svelte', '$lib/shared/components/print-config-panel.svelte'],
  ['$lib/components/shared/PrintDialog.svelte', '$lib/shared/components/print-dialog.svelte'],
  ['$lib/components/shared/avatar-upload.svelte', '$lib/shared/components/avatar-upload.svelte'],
  ['$lib/components/shared/collapsible-summary.svelte', '$lib/shared/components/collapsible-summary.svelte'],
  ['$lib/components/shared/copy-input.svelte', '$lib/shared/components/copy-input.svelte'],
  ['$lib/components/shared/display-name.svelte', '$lib/shared/components/display-name.svelte'],
  ['$lib/components/shared/horizontal-pagination.svelte', '$lib/shared/components/horizontal-pagination.svelte'],
  ['$lib/components/shared/input-color.svelte', '$lib/shared/components/input-color.svelte'],
  ['$lib/components/shared/institution-info.svelte', '$lib/shared/components/institution-info.svelte'],
  ['$lib/components/shared/locale-settings.svelte', '$lib/shared/components/locale-settings.svelte'],
  ['$lib/components/shared/logo-upload.svelte', '$lib/shared/components/logo-upload.svelte'],
  ['$lib/components/shared/search-field.svelte', '$lib/shared/components/search-field.svelte'],
  ['$lib/components/shared/timezone-settings.svelte', '$lib/shared/components/timezone-settings.svelte'],
  ['$lib/components/shared/', '$lib/shared/components/']
];

async function* walk(dir: string): AsyncGenerator<string> {
  const entries = await readdir(dir, { withFileTypes: true });
  for (const entry of entries) {
    const fullPath = join(dir, entry.name);
    if (entry.name === "node_modules" || entry.name === ".svelte-kit" || entry.name === "scripts") continue;
    if (entry.isDirectory()) {
      yield* walk(fullPath);
    } else if (EXTENSIONS.has(extname(entry.name))) {
      yield fullPath;
    }
  }
}

let updatedCount = 0;

for await (const filePath of walk(SRC_DIR)) {
  let content = await readFile(filePath, "utf-8");
  let changed = false;

  for (const [oldPath, newPath] of IMPORT_MAPPINGS) {
    if (content.includes(oldPath)) {
      content = content.replaceAll(oldPath, newPath);
      changed = true;
    }
  }

  if (changed) {
    await writeFile(filePath, content, "utf-8");
    updatedCount++;
    console.log(`Updated shared imports in: ${filePath}`);
  }
}

console.log(`\nTotal files updated: ${updatedCount}`);
