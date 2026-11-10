import type { MultiUpConfig } from "@docforge/core";

// ═══════════════════════════════════════════════════════════════
// Global Print Store (Svelte 5)
// Provides a unified way to trigger the print dialog from anywhere.
// ═══════════════════════════════════════════════════════════════

export type PrintPreferences = Omit<
  MultiUpConfig,
  "templateWidthMm" | "templateHeightMm"
>;

export interface PrintDialogState {
  isOpen: boolean;
  entityType: string;
  documentType?: string;
  entityIds: string[];
  entityLabels?: string[];
  contexts?: Record<string, unknown>[];
  preferences: Partial<PrintPreferences>;
}

const PRINT_PREFERENCES_KEY = "orbis_print_preferences";
const SHEET_SIZES = ["A4", "A3", "A5", "Letter", "Legal", "Custom"] as const;

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null;
}

function isNonNegativeNumber(value: unknown): value is number {
  return typeof value === "number" && Number.isFinite(value) && value >= 0;
}

function readPrintPreferences(value: unknown): Partial<PrintPreferences> {
  if (!isRecord(value)) return {};

  const preferences: Partial<PrintPreferences> = {};
  if (
    typeof value.sheetSize === "string" &&
    SHEET_SIZES.includes(value.sheetSize as (typeof SHEET_SIZES)[number])
  ) {
    preferences.sheetSize = value.sheetSize as PrintPreferences["sheetSize"];
  }
  if (value.sheetOrientation === "portrait" || value.sheetOrientation === "landscape") {
    preferences.sheetOrientation = value.sheetOrientation;
  }
  if (isRecord(value.sheetMargins)) {
    const margins = value.sheetMargins;
    if (
      isNonNegativeNumber(margins.top) &&
      isNonNegativeNumber(margins.bottom) &&
      isNonNegativeNumber(margins.left) &&
      isNonNegativeNumber(margins.right)
    ) {
      preferences.sheetMargins = {
        top: margins.top,
        bottom: margins.bottom,
        left: margins.left,
        right: margins.right,
      };
    }
  }
  if (isNonNegativeNumber(value.gapHorizontalMm)) {
    preferences.gapHorizontalMm = value.gapHorizontalMm;
  }
  if (isNonNegativeNumber(value.gapVerticalMm)) {
    preferences.gapVerticalMm = value.gapVerticalMm;
  }
  if (value.fillOrder === "ltr-ttb" || value.fillOrder === "rtl-ttb") {
    preferences.fillOrder = value.fillOrder;
  }
  if (isNonNegativeNumber(value.cutLineWidthMm)) {
    preferences.cutLineWidthMm = value.cutLineWidthMm;
  }
  if (
    typeof value.cutLineColor === "string" &&
    /^#[\da-f]{6}$/i.test(value.cutLineColor)
  ) {
    preferences.cutLineColor = value.cutLineColor;
  }
  if (typeof value.centerOnSheet === "boolean") {
    preferences.centerOnSheet = value.centerOnSheet;
  }
  if (
    isRecord(value.forcedGrid) &&
    Number.isInteger(value.forcedGrid.columns) &&
    Number(value.forcedGrid.columns) > 0 &&
    Number.isInteger(value.forcedGrid.rows) &&
    Number(value.forcedGrid.rows) > 0
  ) {
    preferences.forcedGrid = {
      columns: Number(value.forcedGrid.columns),
      rows: Number(value.forcedGrid.rows),
    };
  }
  if (
    isNonNegativeNumber(value.customSheetWidthMm) &&
    value.customSheetWidthMm > 0
  ) {
    preferences.customSheetWidthMm = value.customSheetWidthMm;
  }
  if (
    isNonNegativeNumber(value.customSheetHeightMm) &&
    value.customSheetHeightMm > 0
  ) {
    preferences.customSheetHeightMm = value.customSheetHeightMm;
  }

  return preferences;
}

function loadPrintPreferences(): Partial<PrintPreferences> {
  if (typeof localStorage === "undefined") return {};
  try {
    const saved = localStorage.getItem(PRINT_PREFERENCES_KEY);
    return saved ? readPrintPreferences(JSON.parse(saved)) : {};
  } catch (error) {
    console.error("Failed to restore print preferences:", error);
    return {};
  }
}

// Global reactive state
export const printState = $state<PrintDialogState>({
  isOpen: false,
  entityType: "",
  documentType: undefined,
  entityIds: [],
  preferences: loadPrintPreferences(),
});

export function savePrintPreferences(config: MultiUpConfig) {
  const {
    templateWidthMm: _templateWidthMm,
    templateHeightMm: _templateHeightMm,
    ...configPreferences
  } = config;
  const preferences = readPrintPreferences(configPreferences);
  printState.preferences = preferences;

  if (typeof localStorage === "undefined") return;
  try {
    localStorage.setItem(PRINT_PREFERENCES_KEY, JSON.stringify(preferences));
  } catch (error) {
    console.error("Failed to save print preferences:", error);
  }
}

/**
 * Open the print dialog for specific entities.
 * @param entityType The entity type (e.g., 'invoice', 'user', 'enrollment')
 * @param entityIds Array of entity IDs to print
 * @param entityLabels Optional array of labels corresponding to IDs
 */
export function openPrintDialog(
  entityType: string,
  entityIds: string[],
  entityLabels?: string[],
  options?: {
    documentType?: string;
    contexts?: Record<string, unknown>[];
  },
) {
  printState.entityType = entityType;
  printState.documentType = options?.documentType;
  printState.entityIds = entityIds;
  printState.entityLabels = entityLabels;
  printState.contexts = options?.contexts;
  printState.isOpen = true;
}

/**
 * Close the print dialog
 */
export function closePrintDialog() {
  printState.isOpen = false;
}
