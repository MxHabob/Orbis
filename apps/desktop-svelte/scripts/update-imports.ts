/**
 * Migration Script: Update all import paths across the project.
 * Run with: bun scripts/update-imports.ts
 */
import { readdir, readFile, writeFile, stat } from "node:fs/promises";
import { join, extname } from "node:path";

const SRC_DIR = "src";
const EXTENSIONS = new Set([".svelte", ".ts", ".js"]);

const IMPORT_MAPPINGS: [string, string][] = [
  // Layout → Shell
  ['$lib/components/layout/AppChrome.svelte', '$lib/shell/app-chrome.svelte'],
  ['$lib/components/layout/AppSidebar.svelte', '$lib/shell/app-sidebar.svelte'],
  ['$lib/components/layout/AskOrbisSidebar.svelte', '$lib/shell/ask-orbis-sidebar.svelte'],
  ['$lib/components/layout/DetailPageHeader.svelte', '$lib/shell/detail-page-header.svelte'],
  ['$lib/components/layout/MainViewport.svelte', '$lib/shell/main-viewport.svelte'],
  ['$lib/components/layout/OnboardingPopover.svelte', '$lib/shell/onboarding-popover.svelte'],
  ['$lib/components/layout/OrgSidebarSwitcher.svelte', '$lib/shell/org-sidebar-switcher.svelte'],
  ['$lib/components/layout/Resizer.svelte', '$lib/shell/resizer.svelte'],
  ['$lib/components/layout/SashLayer.svelte', '$lib/shell/sash-layer.svelte'],
  ['$lib/components/layout/SearchModal.svelte', '$lib/shell/search-modal.svelte'],
  ['$lib/components/layout/SecondaryMenu.svelte', '$lib/shell/secondary-menu.svelte'],
  ['$lib/components/layout/SidebarNav.svelte', '$lib/shell/sidebar-nav.svelte'],
  ['$lib/components/layout/TitleBar.svelte', '$lib/shell/title-bar.svelte'],
  ['$lib/components/layout/TitleBreadcrumbs.svelte', '$lib/shell/title-breadcrumbs.svelte'],
  ['$lib/components/layout/UpgradeDialog.svelte', '$lib/shell/upgrade-dialog.svelte'],
  ['$lib/components/layout/UserSidebarMenu.svelte', '$lib/shell/user-sidebar-menu.svelte'],
  ['$lib/components/layout/WhatsNewPopover.svelte', '$lib/shell/whats-new-popover.svelte'],
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
    console.log(`Updated: ${filePath}`);
  }
}

console.log(`\nTotal files updated: ${updatedCount}`);
