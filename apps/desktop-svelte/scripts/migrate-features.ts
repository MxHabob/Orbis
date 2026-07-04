/**
 * Migration Script Phase 2: Create feature modules, copy and rename files to kebab-case.
 * Run with: bun scripts/migrate-features.ts
 */
import { copyFile, mkdir, readdir, stat } from "node:fs/promises";
import { join, dirname } from "node:path";

const LIB = "src/lib";
const FEATURES = join(LIB, "features");

async function ensureDir(dir: string) {
  await mkdir(dir, { recursive: true });
}

async function copyRecursive(src: string, dest: string, renameMap: Record<string, string> = {}) {
  await ensureDir(dest);
  const entries = await readdir(src, { withFileTypes: true });

  for (const entry of entries) {
    const srcPath = join(src, entry.name);
    let destName = renameMap[entry.name] || entry.name;
    // Lowercase svelte components if they are PascalCase
    if (entry.name.endsWith(".svelte") && /^[A-Z]/.test(entry.name)) {
      destName = entry.name.replace(/([a-z0-9])([A-Z])/g, "$1-$2").toLowerCase();
    }
    const destPath = join(dest, destName);

    if (entry.isDirectory()) {
      await copyRecursive(srcPath, destPath, renameMap);
    } else {
      await ensureDir(dirname(destPath));
      await copyFile(srcPath, destPath);
      console.log(`Copied: ${srcPath} -> ${destPath}`);
    }
  }
}

// 1. Auth Feature
await ensureDir(join(FEATURES, "auth/components"));
await copyRecursive(join(LIB, "components/login"), join(FEATURES, "auth/components"));
await Bun.write(join(FEATURES, "auth/index.ts"), `export { default as LoginForm } from "./components/login-form.svelte";\n`);

// 2. Onboarding Feature
await ensureDir(join(FEATURES, "onboarding"));
await copyRecursive(join(LIB, "components/onboarding"), join(FEATURES, "onboarding"), {
  "onboarding-state.svelte.ts": "state.svelte.ts",
  "onboarding-templates.ts": "templates.ts",
  "onboarding-types.ts": "types.ts"
});
await Bun.write(join(FEATURES, "onboarding/index.ts"), `export { default as OnboardingWizard } from "./components/onboarding-wizard.svelte";\nexport { onboardingState } from "./state.svelte";\n`);

// 3. Dashboard Feature
await ensureDir(join(FEATURES, "dashboard/components"));
await copyRecursive(join(LIB, "components/widgets"), join(FEATURES, "dashboard/components"), {
  "index.svelte": "dashboard-widgets.svelte"
});
await copyFile(join(LIB, "services/queries/dashboard.ts"), join(FEATURES, "dashboard/queries.ts"));
await Bun.write(join(FEATURES, "dashboard/index.ts"), `export { default as DashboardWidgets } from "./components/dashboard-widgets.svelte";\nexport * from "./queries";\n`);

// 4. Academics Feature
await ensureDir(join(FEATURES, "academics/components"));
await copyRecursive(join(LIB, "components/academics"), join(FEATURES, "academics/components"));
// Forms & Sheets for Academics
const academicForms = [
  "class-form.svelte", "cycle-form.svelte", "subject-form.svelte",
  "lesson-form.svelte", "class-subject-form.svelte",
  "assessment-form.svelte", "room-form.svelte", "org-unit-form.svelte"
];
const academicSheets = [
  "class-create-sheet.svelte", "cycle-create-sheet.svelte", "subject-create-sheet.svelte",
  "lesson-create-sheet.svelte", "class-subject-sheet.svelte",
  "assessment-create-sheet.svelte", "room-create-sheet.svelte", "org-unit-sheet.svelte"
];
for (const f of academicForms) {
  await copyFile(join(LIB, "components/forms", f), join(FEATURES, "academics/components", f));
}
for (const s of academicSheets) {
  await copyFile(join(LIB, "components/sheets", s), join(FEATURES, "academics/components", s));
}
await copyFile(join(LIB, "services/queries/academic.ts"), join(FEATURES, "academics/queries.ts"));
await copyFile(join(LIB, "stores/academic.svelte.ts"), join(FEATURES, "academics/store.svelte.ts"));
await Bun.write(join(FEATURES, "academics/index.ts"), `export { academicStore } from "./store.svelte";\nexport * from "./queries";\n`);

// 5. Students Feature
await ensureDir(join(FEATURES, "students/components"));
await copyRecursive(join(LIB, "components/students"), join(FEATURES, "students"), {
  "student-form-state.svelte.ts": "form-state.svelte.ts"
});
await copyFile(join(LIB, "components/forms/guardian-form.svelte"), join(FEATURES, "students/components/guardian-form.svelte"));
await copyFile(join(LIB, "components/sheets/guardian-link-sheet.svelte"), join(FEATURES, "students/components/guardian-link-sheet.svelte"));
await copyFile(join(LIB, "services/queries/students.ts"), join(FEATURES, "students/queries.ts"));
await copyFile(join(LIB, "stores/students.svelte.ts"), join(FEATURES, "students/store.svelte.ts"));
await Bun.write(join(FEATURES, "students/index.ts"), `export { studentsStore } from "./store.svelte";\nexport * from "./queries";\n`);

// 6. Exams Feature
await ensureDir(join(FEATURES, "exams/components"));
await copyRecursive(join(LIB, "components/exams"), join(FEATURES, "exams/components"));
await copyFile(join(LIB, "stores/exams.svelte.ts"), join(FEATURES, "exams/store.svelte.ts"));
await Bun.write(join(FEATURES, "exams/index.ts"), `export { examsStore } from "./store.svelte";\n`);

// 7. Finance Feature
await ensureDir(join(FEATURES, "finance/components"));
const financeForms = [
  "fee-template-form.svelte", "fee-assign-form.svelte", "discount-template-form.svelte",
  "ledger-form.svelte", "payment-record-form.svelte", "treasury-form.svelte"
];
const financeSheets = [
  "fee-template-sheet.svelte", "fee-assign-sheet.svelte", "discount-template-sheet.svelte",
  "ledger-create-sheet.svelte", "payment-record-sheet.svelte", "treasury-create-sheet.svelte"
];
for (const f of financeForms) {
  await copyFile(join(LIB, "components/forms", f), join(FEATURES, "finance/components", f));
}
for (const s of financeSheets) {
  await copyFile(join(LIB, "components/sheets", s), join(FEATURES, "finance/components", s));
}
await copyFile(join(LIB, "services/queries/finance.ts"), join(FEATURES, "finance/queries.ts"));
await Bun.write(join(FEATURES, "finance/index.ts"), `export * from "./queries";\n`);

// 8. Metrics Feature
await copyRecursive(join(LIB, "components/metrics"), join(FEATURES, "metrics"), {
  "chart-types.ts": "types.ts"
});
await Bun.write(join(FEATURES, "metrics/index.ts"), `export { default as MetricsView } from "./metrics-view.svelte";\n`);

// 9. Bulk Input Feature
await copyRecursive(join(LIB, "components/bulk-input"), join(FEATURES, "bulk-input"), {
  "bulk-input-config.ts": "config.ts",
  "bulk-input-validator.ts": "validator.ts"
});
await Bun.write(join(FEATURES, "bulk-input/index.ts"), `export { default as BulkInputGrid } from "./components/bulk-input-grid.svelte";\n`);

// 10. Templates Feature
await copyRecursive(join(LIB, "components/template-designer"), join(FEATURES, "templates"));
await Bun.write(join(FEATURES, "templates/index.ts"), `export { editorStore } from "./stores/editor-store.svelte";\n`);

console.log("\nPhase 2 Feature Migration copies complete!");
