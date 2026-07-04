/**
 * Migration Script: Update all feature components, queries and stores import paths.
 * Run with: bun scripts/update-feature-imports.ts
 */
import { readdir, readFile, writeFile } from "node:fs/promises";
import { join, extname } from "node:path";

const SRC_DIR = "src";
const EXTENSIONS = new Set([".svelte", ".ts", ".js", ".css"]);

const MAPPINGS: [string, string][] = [
  // 1. Auth Feature
  ['$lib/components/login/LoginForm.svelte', '$lib/features/auth/components/login-form.svelte'],
  ['$lib/components/login/AnnouncementBanner.svelte', '$lib/features/auth/components/announcement-banner.svelte'],
  ['$lib/components/login/BackgroundPattern.svelte', '$lib/features/auth/components/background-pattern.svelte'],
  ['$lib/components/login/EnvironmentSelect.svelte', '$lib/features/auth/components/environment-select.svelte'],

  // 2. Onboarding Feature
  ['$lib/components/onboarding/onboarding-footer.svelte', '$lib/features/onboarding/onboarding-footer.svelte'],
  ['$lib/components/onboarding/onboarding-stepper.svelte', '$lib/features/onboarding/onboarding-stepper.svelte'],
  ['$lib/components/onboarding/onboarding-wizard.svelte', '$lib/features/onboarding/onboarding-wizard.svelte'],
  ['$lib/components/onboarding/onboarding-state.svelte', '$lib/features/onboarding/state.svelte'],
  ['$lib/components/onboarding/onboarding-templates', '$lib/features/onboarding/templates'],
  ['$lib/components/onboarding/onboarding-types', '$lib/features/onboarding/types'],
  ['$lib/components/onboarding/steps/', '$lib/features/onboarding/steps/'],

  // 3. Dashboard Feature
  ['$lib/components/widgets/desktop-banner.svelte', '$lib/features/dashboard/components/desktop-banner.svelte'],
  ['$lib/components/widgets/index.svelte', '$lib/features/dashboard/components/dashboard-widgets.svelte'],
  ['$lib/components/widgets/overview-skeleton.svelte', '$lib/features/dashboard/components/overview-skeleton.svelte'],
  ['$lib/components/widgets/quick-actions.svelte', '$lib/features/dashboard/components/quick-actions.svelte'],
  ['$lib/components/widgets/welcome-section.svelte', '$lib/features/dashboard/components/welcome-section.svelte'],
  ['$lib/components/widgets/widget-cards.svelte', '$lib/features/dashboard/components/widget-cards.svelte'],
  ['$lib/services/queries/dashboard', '$lib/features/dashboard/queries'],

  // 4. Academics Feature
  ['$lib/components/academics/scheduler-ai-panel.svelte', '$lib/features/academics/components/scheduler-ai-panel.svelte'],
  ['$lib/components/forms/class-form.svelte', '$lib/features/academics/components/class-form.svelte'],
  ['$lib/components/forms/cycle-form.svelte', '$lib/features/academics/components/cycle-form.svelte'],
  ['$lib/components/forms/subject-form.svelte', '$lib/features/academics/components/subject-form.svelte'],
  ['$lib/components/forms/lesson-form.svelte', '$lib/features/academics/components/lesson-form.svelte'],
  ['$lib/components/forms/class-subject-form.svelte', '$lib/features/academics/components/class-subject-form.svelte'],
  ['$lib/components/forms/assessment-form.svelte', '$lib/features/academics/components/assessment-form.svelte'],
  ['$lib/components/forms/room-form.svelte', '$lib/features/academics/components/room-form.svelte'],
  ['$lib/components/forms/org-unit-form.svelte', '$lib/features/academics/components/org-unit-form.svelte'],
  ['$lib/components/sheets/class-create-sheet.svelte', '$lib/features/academics/components/class-create-sheet.svelte'],
  ['$lib/components/sheets/cycle-create-sheet.svelte', '$lib/features/academics/components/cycle-create-sheet.svelte'],
  ['$lib/components/sheets/subject-create-sheet.svelte', '$lib/features/academics/components/subject-create-sheet.svelte'],
  ['$lib/components/sheets/lesson-create-sheet.svelte', '$lib/features/academics/components/lesson-create-sheet.svelte'],
  ['$lib/components/sheets/class-subject-sheet.svelte', '$lib/features/academics/components/class-subject-sheet.svelte'],
  ['$lib/components/sheets/assessment-create-sheet.svelte', '$lib/features/academics/components/assessment-create-sheet.svelte'],
  ['$lib/components/sheets/room-create-sheet.svelte', '$lib/features/academics/components/room-create-sheet.svelte'],
  ['$lib/components/sheets/org-unit-sheet.svelte', '$lib/features/academics/components/org-unit-sheet.svelte'],
  ['$lib/services/queries/academic', '$lib/features/academics/queries'],
  ['$lib/stores/academic.svelte', '$lib/features/academics/store.svelte'],

  // 5. Students Feature
  ['$lib/components/students/student-form.svelte', '$lib/features/students/student-form.svelte'],
  ['$lib/components/students/student-inspector.svelte', '$lib/features/students/student-inspector.svelte'],
  ['$lib/components/students/student-sidebar-filters.svelte', '$lib/features/students/student-sidebar-filters.svelte'],
  ['$lib/components/students/student-table.svelte', '$lib/features/students/student-table.svelte'],
  ['$lib/components/students/student-form-state.svelte', '$lib/features/students/form-state.svelte'],
  ['$lib/components/students/sections/DocumentsSection.svelte', '$lib/features/students/sections/documents-section.svelte'],
  ['$lib/components/students/sections/FamilySection.svelte', '$lib/features/students/sections/family-section.svelte'],
  ['$lib/components/students/sections/FinancialsSection.svelte', '$lib/features/students/sections/financials-section.svelte'],
  ['$lib/components/students/sections/LocationSection.svelte', '$lib/features/students/sections/location-section.svelte'],
  ['$lib/components/students/sections/ProfileSection.svelte', '$lib/features/students/sections/profile-section.svelte'],
  ['$lib/components/forms/guardian-form.svelte', '$lib/features/students/components/guardian-form.svelte'],
  ['$lib/components/sheets/guardian-link-sheet.svelte', '$lib/features/students/components/guardian-link-sheet.svelte'],
  ['$lib/services/queries/students', '$lib/features/students/queries'],
  ['$lib/stores/students.svelte', '$lib/features/students/store.svelte'],

  // 6. Exams Feature
  ['$lib/components/exams/readiness-gauge.svelte', '$lib/features/exams/components/readiness-gauge.svelte'],
  ['$lib/components/exams/session-create-sheet.svelte', '$lib/features/exams/components/session-create-sheet.svelte'],
  ['$lib/components/exams/status-badge.svelte', '$lib/features/exams/components/status-badge.svelte'],
  ['$lib/components/exams/subject-add-sheet.svelte', '$lib/features/exams/components/subject-add-sheet.svelte'],
  ['$lib/stores/exams.svelte', '$lib/features/exams/store.svelte'],

  // 7. Finance Feature
  ['$lib/components/forms/fee-template-form.svelte', '$lib/features/finance/components/fee-template-form.svelte'],
  ['$lib/components/forms/fee-assign-form.svelte', '$lib/features/finance/components/fee-assign-form.svelte'],
  ['$lib/components/forms/discount-template-form.svelte', '$lib/features/finance/components/discount-template-form.svelte'],
  ['$lib/components/forms/ledger-form.svelte', '$lib/features/finance/components/ledger-form.svelte'],
  ['$lib/components/forms/payment-record-form.svelte', '$lib/features/finance/components/payment-record-form.svelte'],
  ['$lib/components/forms/treasury-form.svelte', '$lib/features/finance/components/treasury-form.svelte'],
  ['$lib/components/sheets/fee-template-sheet.svelte', '$lib/features/finance/components/fee-template-sheet.svelte'],
  ['$lib/components/sheets/fee-assign-sheet.svelte', '$lib/features/finance/components/fee-assign-sheet.svelte'],
  ['$lib/components/sheets/discount-template-sheet.svelte', '$lib/features/finance/components/discount-template-sheet.svelte'],
  ['$lib/components/sheets/ledger-create-sheet.svelte', '$lib/features/finance/components/ledger-create-sheet.svelte'],
  ['$lib/components/sheets/payment-record-sheet.svelte', '$lib/features/finance/components/payment-record-sheet.svelte'],
  ['$lib/components/sheets/treasury-create-sheet.svelte', '$lib/features/finance/components/treasury-create-sheet.svelte'],
  ['$lib/services/queries/finance', '$lib/features/finance/queries'],

  // 8. Metrics Feature
  ['$lib/components/metrics/cards/attendance-trends-card.svelte', '$lib/features/metrics/cards/attendance-trends-card.svelte'],
  ['$lib/components/metrics/cards/behavior-summary-card.svelte', '$lib/features/metrics/cards/behavior-summary-card.svelte'],
  ['$lib/components/metrics/cards/fee-collection-card.svelte', '$lib/features/metrics/cards/fee-collection-card.svelte'],
  ['$lib/components/metrics/cards/grade-distribution-card.svelte', '$lib/features/metrics/cards/grade-distribution-card.svelte'],
  ['$lib/components/metrics/components/drag-indicator.svelte', '$lib/features/metrics/components/drag-indicator.svelte'],
  ['$lib/components/metrics/components/draggable-chart-card.svelte', '$lib/features/metrics/components/draggable-chart-card.svelte'],
  ['$lib/components/metrics/components/metrics-filter.svelte', '$lib/features/metrics/components/metrics-filter.svelte'],
  ['$lib/components/metrics/components/metrics-grid.svelte', '$lib/features/metrics/components/metrics-grid.svelte'],
  ['$lib/components/metrics/components/metrics-settings.svelte', '$lib/features/metrics/components/metrics-settings.svelte'],
  ['$lib/components/metrics/metrics-view.svelte', '$lib/features/metrics/metrics-view.svelte'],
  ['$lib/components/metrics/utils/chart-types', '$lib/features/metrics/utils/types'],

  // 9. Bulk Input Feature
  ['$lib/components/bulk-input/bulk-input-config', '$lib/features/bulk-input/config'],
  ['$lib/components/bulk-input/bulk-input-validator', '$lib/features/bulk-input/validator'],
  ['$lib/components/bulk-input/BulkGrid.svelte', '$lib/features/bulk-input/bulk-grid.svelte'],
  ['$lib/components/bulk-input/BulkInputGrid.svelte', '$lib/features/bulk-input/bulk-input-grid.svelte'],
  ['$lib/components/bulk-input/BulkInputSummary.svelte', '$lib/features/bulk-input/bulk-input-summary.svelte'],
  ['$lib/components/bulk-input/BulkSummary.svelte', '$lib/features/bulk-input/bulk-summary.svelte'],
  ['$lib/components/bulk-input/paste-handler', '$lib/features/bulk-input/paste-handler'],
  ['$lib/components/bulk-input/students/student-columns', '$lib/features/bulk-input/students/student-columns'],
  ['$lib/components/bulk-input/students/student-validator', '$lib/features/bulk-input/students/student-validator'],
  ['$lib/components/bulk-input/grades/grade-columns', '$lib/features/bulk-input/grades/grade-columns'],
  ['$lib/components/bulk-input/grades/grade-validator', '$lib/features/bulk-input/grades/grade-validator'],

  // 10. Templates Feature
  ['$lib/components/template-designer/canvas/CanvasContextMenu.svelte', '$lib/features/templates/canvas/canvas-context-menu.svelte'],
  ['$lib/components/template-designer/canvas/CanvasNode.svelte', '$lib/features/templates/canvas/canvas-node.svelte'],
  ['$lib/components/template-designer/canvas/CanvasPage.svelte', '$lib/features/templates/canvas/canvas-page.svelte'],
  ['$lib/components/template-designer/canvas/CanvasRoot.svelte', '$lib/features/templates/canvas/canvas-root.svelte'],
  ['$lib/components/template-designer/canvas/GridOverlay.svelte', '$lib/features/templates/canvas/grid-overlay.svelte'],
  ['$lib/components/template-designer/canvas/RulerBar.svelte', '$lib/features/templates/canvas/ruler-bar.svelte'],
  ['$lib/components/template-designer/canvas/SelectionOverlay.svelte', '$lib/features/templates/canvas/selection-overlay.svelte'],
  ['$lib/components/template-designer/canvas/SnapLinesOverlay.svelte', '$lib/features/templates/canvas/snap-lines-overlay.svelte'],
  ['$lib/components/template-designer/layout/ContextToolbar.svelte', '$lib/features/templates/layout/context-toolbar.svelte'],
  ['$lib/components/template-designer/layout/FloatingActionBar.svelte', '$lib/features/templates/layout/floating-action-bar.svelte'],
  ['$lib/components/template-designer/layout/FlyoutPanel.svelte', '$lib/features/templates/layout/flyout-panel.svelte'],
  ['$lib/components/template-designer/layout/SideRail.svelte', '$lib/features/templates/layout/side-rail.svelte'],
  ['$lib/components/template-designer/layout/StatusBar.svelte', '$lib/features/templates/layout/status-bar.svelte'],
  ['$lib/components/template-designer/panels/AssetPanel.svelte', '$lib/features/templates/panels/asset-panel.svelte'],
  ['$lib/components/template-designer/panels/ElementsPanel.svelte', '$lib/features/templates/panels/elements-panel.svelte'],
  ['$lib/components/template-designer/panels/HistoryPanel.svelte', '$lib/features/templates/panels/history-panel.svelte'],
  ['$lib/components/template-designer/panels/InspectorPanel.svelte', '$lib/features/templates/panels/inspector-panel.svelte'],
  ['$lib/components/template-designer/panels/LayersPanel.svelte', '$lib/features/templates/panels/layers-panel.svelte'],
  ['$lib/components/template-designer/panels/PageSetupPanel.svelte', '$lib/features/templates/panels/page-setup-panel.svelte'],
  ['$lib/components/template-designer/panels/PagesPanel.svelte', '$lib/features/templates/panels/pages-panel.svelte'],
  ['$lib/components/template-designer/panels/TextPresetsPanel.svelte', '$lib/features/templates/panels/text-presets-panel.svelte'],
  ['$lib/components/template-designer/panels/ToolbarPanel.svelte', '$lib/features/templates/panels/toolbar-panel.svelte'],
  ['$lib/components/template-designer/panels/VariablesPanel.svelte', '$lib/features/templates/panels/variables-panel.svelte'],
  ['$lib/components/template-designer/shared/ColorPicker.svelte', '$lib/features/templates/shared/color-picker.svelte'],
  ['$lib/components/template-designer/stores/clipboard-manager', '$lib/features/templates/stores/clipboard-manager'],
  ['$lib/components/template-designer/stores/command-system', '$lib/features/templates/stores/command-system'],
  ['$lib/components/template-designer/stores/editor-store.svelte', '$lib/features/templates/stores/editor-store.svelte'],
  
  // Settings components
  ['$lib/components/shared/institution-info.svelte', '$lib/shared/components/institution-info.svelte'],
  ['$lib/components/shared/locale-settings.svelte', '$lib/shared/components/locale-settings.svelte'],
  ['$lib/components/shared/timezone-settings.svelte', '$lib/shared/components/timezone-settings.svelte']
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

  for (const [oldPath, newPath] of MAPPINGS) {
    if (content.includes(oldPath)) {
      content = content.replaceAll(oldPath, newPath);
      changed = true;
    }
  }

  // Also catch relative imports inside migrated templates canvas / layouts / panels pointing to other designer parts
  if (filePath.includes("features/templates")) {
    if (content.includes('from "../stores/editor-store.svelte"')) {
      content = content.replaceAll('from "../stores/editor-store.svelte"', 'from "../stores/editor-store.svelte"'); // no-op but standard
    }
    // Let's replace any PascalCase internal template imports
    const pascalReplacements: [string, string][] = [
      ['../shared/ColorPicker.svelte', '../shared/color-picker.svelte'],
      ['../canvas/GridOverlay.svelte', '../canvas/grid-overlay.svelte'],
      ['../canvas/RulerBar.svelte', '../canvas/ruler-bar.svelte'],
      ['../canvas/SnapLinesOverlay.svelte', '../canvas/snap-lines-overlay.svelte'],
      ['../canvas/SelectionOverlay.svelte', '../canvas/selection-overlay.svelte'],
      ['../canvas/CanvasContextMenu.svelte', '../canvas/canvas-context-menu.svelte'],
      ['../canvas/CanvasNode.svelte', '../canvas/canvas-node.svelte'],
      ['../canvas/CanvasPage.svelte', '../canvas/canvas-page.svelte'],
      ['../canvas/CanvasRoot.svelte', '../canvas/canvas-root.svelte'],
      ['../layout/ContextToolbar.svelte', '../layout/context-toolbar.svelte'],
      ['../layout/FloatingActionBar.svelte', '../layout/floating-action-bar.svelte'],
      ['../layout/FlyoutPanel.svelte', '../layout/flyout-panel.svelte'],
      ['../layout/SideRail.svelte', '../layout/side-rail.svelte'],
      ['../layout/StatusBar.svelte', '../layout/status-bar.svelte'],
      ['../panels/AssetPanel.svelte', '../panels/asset-panel.svelte'],
      ['../panels/ElementsPanel.svelte', '../panels/elements-panel.svelte'],
      ['../panels/HistoryPanel.svelte', '../panels/history-panel.svelte'],
      ['../panels/InspectorPanel.svelte', '../panels/inspector-panel.svelte'],
      ['../panels/LayersPanel.svelte', '../panels/layers-panel.svelte'],
      ['../panels/PageSetupPanel.svelte', '../panels/page-setup-panel.svelte'],
      ['../panels/PagesPanel.svelte', '../panels/pages-panel.svelte'],
      ['../panels/TextPresetsPanel.svelte', '../panels/text-presets-panel.svelte'],
      ['../panels/ToolbarPanel.svelte', '../panels/toolbar-panel.svelte'],
      ['../panels/VariablesPanel.svelte', '../panels/variables-panel.svelte']
    ];
    for (const [oldRel, newRel] of pascalReplacements) {
      if (content.includes(oldRel)) {
        content = content.replaceAll(oldRel, newRel);
        changed = true;
      }
    }
  }

  if (changed) {
    await writeFile(filePath, content, "utf-8");
    updatedCount++;
    console.log(`Updated feature imports in: ${filePath}`);
  }
}

console.log(`\nTotal files updated: ${updatedCount}`);
