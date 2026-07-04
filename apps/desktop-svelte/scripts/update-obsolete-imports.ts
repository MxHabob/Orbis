/**
 * Migration Script: Update obsolete backward-compatibility import paths across the project
 * to point directly to core/shared/features locations.
 * Run with: bun scripts/update-obsolete-imports.ts
 */
import { readdir, readFile, writeFile } from "node:fs/promises";
import { join, extname } from "node:path";

const SRC_DIR = "src";
const EXTENSIONS = new Set([".svelte", ".ts", ".js"]);

// Define the exact imports we want to clean up
const MAPPINGS: [string | RegExp, string][] = [
  // Debounce
  [/\$lib\/utils\/debounce/g, '$lib/shared/utils/debounce'],

  // IPC / Dedup / Error / Prefetch -> $lib/core/ipc
  [/\$lib\/services\/ipc-dedup/g, '$lib/core/ipc'],
  [/\$lib\/utils\/ipc-error/g, '$lib/core/ipc'],
  [/\$lib\/services\/prefetch/g, '$lib/core/ipc'],
  [/\$lib\/services\/ipc/g, '$lib/core/ipc'],

  // Query Client -> $lib/core/query/client
  [/\$lib\/services\/query-client/g, '$lib/core/query/client'],

  // I18n Utilities -> $lib/core/i18n
  [/\$lib\/i18n-utils/g, '$lib/core/i18n'],

  // Stores -> Features Stores
  [/\$lib\/stores\/academic\.svelte/g, '$lib/features/academics/store.svelte'],
  [/\$lib\/stores\/students\.svelte/g, '$lib/features/students/store.svelte'],
  [/\$lib\/stores\/exams\.svelte/g, '$lib/features/exams/store.svelte']
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
  // Skip the re-export legacy files themselves so they don't modify themselves before deletion
  if (
    filePath.endsWith("src\\lib\\utils\\debounce.ts") ||
    filePath.endsWith("src/lib/utils/debounce.ts") ||
    filePath.endsWith("src\\lib\\utils\\ipc-error.ts") ||
    filePath.endsWith("src/lib/utils/ipc-error.ts") ||
    filePath.endsWith("src\\lib\\i18n-utils.ts") ||
    filePath.endsWith("src/lib/i18n-utils.ts") ||
    filePath.endsWith("src\\lib\\services\\ipc.ts") ||
    filePath.endsWith("src/lib/services/ipc.ts") ||
    filePath.endsWith("src\\lib\\services\\ipc-dedup.ts") ||
    filePath.endsWith("src/lib/services/ipc-dedup.ts") ||
    filePath.endsWith("src\\lib\\services\\query-client.ts") ||
    filePath.endsWith("src/lib/services/query-client.ts") ||
    filePath.endsWith("src\\lib\\services\\prefetch.ts") ||
    filePath.endsWith("src/lib/services/prefetch.ts") ||
    filePath.endsWith("src\\lib\\stores\\academic.svelte.ts") ||
    filePath.endsWith("src/lib/stores/academic.svelte.ts") ||
    filePath.endsWith("src\\lib\\stores\\students.svelte.ts") ||
    filePath.endsWith("src/lib/stores/students.svelte.ts") ||
    filePath.endsWith("src\\lib\\stores\\exams.svelte.ts") ||
    filePath.endsWith("src/lib/stores/exams.svelte.ts")
  ) {
    continue;
  }

  let content = await readFile(filePath, "utf-8");
  let changed = false;

  for (const [regex, newPath] of MAPPINGS) {
    if (content.match(regex)) {
      content = content.replace(regex, newPath);
      changed = true;
    }
  }

  if (changed) {
    await writeFile(filePath, content, "utf-8");
    updatedCount++;
    console.log(`Cleaned imports in: ${filePath}`);
  }
}

console.log(`\nImport updates completed on ${updatedCount} files.`);
