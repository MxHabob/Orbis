import fs from 'fs';
import path from 'path';

const svelteIconsIndex = fs.readFileSync('packages/icons/src/svelte/index.ts', 'utf8');
const validIconNames = new Set<string>();
for (const match of svelteIconsIndex.matchAll(/export\s*\{\s*default\s*as\s*([A-Za-z0-9_]+)\s*\}/g)) {
  validIconNames.add(match[1]);
}

function getFiles(dir: string, exts = ['.svelte', '.tsx']): string[] {
  let results: string[] = [];
  if (!fs.existsSync(dir)) return results;
  for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
    const full = path.join(dir, entry.name);
    if (entry.isDirectory()) {
      if (entry.name !== 'node_modules' && entry.name !== '.svelte-kit') {
        results.push(...getFiles(full, exts));
      }
    } else if (exts.some((ext) => entry.name.endsWith(ext))) {
      results.push(full);
    }
  }
  return results;
}

const files = [
  ...getFiles('apps/desktop/src'),
  ...getFiles('packages/ui/src'),
  ...getFiles('apps/mobile/src'),
];

// Check 1: Any aliased icon imports
let aliasedCount = 0;
for (const file of files) {
  const content = fs.readFileSync(file, 'utf8');
  const matches = content.matchAll(/import\s*\{([^}]+)\}\s*from\s*['"](@orbis\/icons(?:\/[a-z]+)?)['"];?/g);
  for (const match of matches) {
    if (match[1].includes(' as ')) {
      console.log(`Aliased @orbis/icons import in: ${file} -> ${match[0]}`);
      aliasedCount++;
    }
  }
}

// Check 2: Any invalid icon imports
let invalidImports = 0;
for (const file of files) {
  const content = fs.readFileSync(file, 'utf8');
  const matches = content.matchAll(/import\s*\{([^}]+)\}\s*from\s*['"](@orbis\/icons(?:\/[a-z]+)?)['"];?/g);
  for (const match of matches) {
    const items = match[1].split(',').map((s) => s.trim()).filter(Boolean);
    for (const item of items) {
      if (!validIconNames.has(item)) {
        console.log(`Invalid icon imported in ${file}: ${item}`);
        invalidImports++;
      }
    }
  }
}

console.log(`Summary:`);
console.log(`- Aliased imports: ${aliasedCount}`);
console.log(`- Invalid imports: ${invalidImports}`);
