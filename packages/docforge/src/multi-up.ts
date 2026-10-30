// ═══════════════════════════════════════════════════════════════
// @docforge/core — Multi-Up Layout Types & Calculator
//
// TypeScript mirror of docforge-engine/layout/multi_up.rs
// Used for instant frontend preview without IPC round-trip.
// The Rust engine is the source of truth for PDF rendering.
// ═══════════════════════════════════════════════════════════════

import type { Orientation } from "./model/document";

// ── Types ────────────────────────────────────────────────────

export type SheetSize = "A4" | "A3" | "A5" | "Letter" | "Legal" | "Custom";
export type FillOrder = "ltr-ttb" | "rtl-ttb";

export interface MultiUpConfig {
  sheetSize: SheetSize;
  sheetOrientation: Orientation;
  sheetMargins: {
    top: number;
    bottom: number;
    left: number;
    right: number;
  };
  /** Template width in mm (from TemplateDocument.meta.page_setup) */
  templateWidthMm: number;
  /** Template height in mm */
  templateHeightMm: number;
  /** Horizontal gap between templates in mm */
  gapHorizontalMm: number;
  /** Vertical gap between templates in mm */
  gapVerticalMm: number;
  /** Fill order: ltr-ttb for LTR, rtl-ttb for RTL (Arabic) */
  fillOrder: FillOrder;
  /** Cut line width in mm (0 = disabled). Default: 0.5 */
  cutLineWidthMm: number;
  /** Cut line color (hex). Default: #cccccc */
  cutLineColor: string;
  /** Manually forced grid dimensions. undefined = auto-compute */
  forcedGrid?: { columns: number; rows: number };
  /** Whether to center templates within available area */
  centerOnSheet: boolean;
  /** Custom sheet dimensions (only used when sheetSize = 'Custom') */
  customSheetWidthMm?: number;
  customSheetHeightMm?: number;
}

export interface MultiUpLayout {
  /** Number of columns */
  columns: number;
  /** Number of rows */
  rows: number;
  /** Total items per physical sheet */
  itemsPerSheet: number;
  /** Positioned slots (in mm for CSS preview) */
  slots: SlotPosition[];
  /** Physical sheet width in mm */
  sheetWidthMm: number;
  /** Physical sheet height in mm */
  sheetHeightMm: number;
  /** Template width in mm */
  templateWidthMm: number;
  /** Template height in mm */
  templateHeightMm: number;
}

export interface SlotPosition {
  /** X position in mm */
  xMm: number;
  /** Y position in mm */
  yMm: number;
  /** Slot width in mm */
  widthMm: number;
  /** Slot height in mm */
  heightMm: number;
  /** Column index (0-based) */
  col: number;
  /** Row index (0-based) */
  row: number;
  /** Sequential index (0-based, follows fill_order) */
  index: number;
}

// ── Sheet Size Dimensions ───────────────────────────────────

const SHEET_DIMENSIONS: Record<SheetSize, { width: number; height: number }> = {
  A4: { width: 210, height: 297 },
  A3: { width: 297, height: 420 },
  A5: { width: 148, height: 210 },
  Letter: { width: 216, height: 279 },
  Legal: { width: 216, height: 356 },
  Custom: { width: 210, height: 297 }, // fallback
};

// ── Default Config ──────────────────────────────────────────

export function createDefaultMultiUpConfig(
  templateWidthMm: number,
  templateHeightMm: number,
): MultiUpConfig {
  return {
    sheetSize: "A4",
    sheetOrientation: "portrait",
    sheetMargins: { top: 10, bottom: 10, left: 10, right: 10 },
    templateWidthMm,
    templateHeightMm,
    gapHorizontalMm: 3,
    gapVerticalMm: 3,
    fillOrder: "rtl-ttb", // Arabic-first default
    cutLineWidthMm: 0.5,
    cutLineColor: "#cccccc",
    centerOnSheet: true,
  };
}

// ── Core Algorithm (Mirror of Rust) ─────────────────────────

/**
 * Compute optimal multi-up layout.
 * This is a TS mirror of docforge-engine's MultiUpConfig::compute_layout().
 * Used for instant frontend preview without Tauri IPC round-trip.
 */
export function computeMultiUpLayout(config: MultiUpConfig): MultiUpLayout {
  // Step 1: Resolve sheet dimensions in mm
  const dims =
    config.sheetSize === "Custom"
      ? {
          width: config.customSheetWidthMm ?? 210,
          height: config.customSheetHeightMm ?? 297,
        }
      : SHEET_DIMENSIONS[config.sheetSize];

  const [sheetWidthMm, sheetHeightMm] =
    config.sheetOrientation === "landscape"
      ? [dims.height, dims.width]
      : [dims.width, dims.height];

  // Step 2: Printable area in mm
  const printableW =
    sheetWidthMm - config.sheetMargins.left - config.sheetMargins.right;
  const printableH =
    sheetHeightMm - config.sheetMargins.top - config.sheetMargins.bottom;

  // Step 3: Compute grid dimensions
  let columns: number;
  let rows: number;

  if (config.forcedGrid) {
    columns = config.forcedGrid.columns;
    rows = config.forcedGrid.rows;
  } else {
    columns = Math.max(
      1,
      computeMaxItems(
        printableW,
        config.templateWidthMm,
        config.gapHorizontalMm,
      ),
    );
    rows = Math.max(
      1,
      computeMaxItems(
        printableH,
        config.templateHeightMm,
        config.gapVerticalMm,
      ),
    );
  }

  const itemsPerSheet = columns * rows;

  // Step 4: Compute slot positions in mm
  const contentBlockW =
    config.templateWidthMm * columns + config.gapHorizontalMm * (columns - 1);
  const contentBlockH =
    config.templateHeightMm * rows + config.gapVerticalMm * (rows - 1);

  let offsetX: number;
  let offsetY: number;

  if (config.centerOnSheet) {
    offsetX = config.sheetMargins.left + (printableW - contentBlockW) / 2;
    offsetY = config.sheetMargins.top + (printableH - contentBlockH) / 2;
  } else {
    offsetX = config.sheetMargins.left;
    offsetY = config.sheetMargins.top;
  }

  // Build slot positions
  const slots: SlotPosition[] = [];

  for (let rowIdx = 0; rowIdx < rows; rowIdx++) {
    for (let colIdx = 0; colIdx < columns; colIdx++) {
      const physicalCol =
        config.fillOrder === "rtl-ttb" ? columns - 1 - colIdx : colIdx;

      const x =
        offsetX +
        physicalCol * (config.templateWidthMm + config.gapHorizontalMm);
      const y =
        offsetY + rowIdx * (config.templateHeightMm + config.gapVerticalMm);

      const sequentialIndex = rowIdx * columns + colIdx;

      slots.push({
        xMm: x,
        yMm: y,
        widthMm: config.templateWidthMm,
        heightMm: config.templateHeightMm,
        col: physicalCol,
        row: rowIdx,
        index: sequentialIndex,
      });
    }
  }

  return {
    columns,
    rows,
    itemsPerSheet,
    slots,
    sheetWidthMm,
    sheetHeightMm,
    templateWidthMm: config.templateWidthMm,
    templateHeightMm: config.templateHeightMm,
  };
}

/**
 * Compute maximum number of items that fit in `available` space,
 * given item `size` and `gap` between items.
 *
 * Formula: n = floor((available + gap) / (size + gap))
 */
function computeMaxItems(available: number, size: number, gap: number): number {
  if (size <= 0 || available <= 0) return 0;
  return Math.floor((available + gap) / (size + gap));
}

// ── Helpers ─────────────────────────────────────────────────

/**
 * Compute total number of physical sheets needed.
 */
export function computeTotalSheets(
  totalRecords: number,
  itemsPerSheet: number,
): number {
  if (itemsPerSheet <= 0) return 0;
  return Math.ceil(totalRecords / itemsPerSheet);
}

/**
 * Suggest possible multi-up layouts for a given sheet+template combination.
 * Returns layouts sorted by itemsPerSheet descending.
 */
export function suggestLayouts(
  templateWidthMm: number,
  templateHeightMm: number,
  sheetSize: SheetSize = "A4",
): MultiUpLayout[] {
  const orientations: Orientation[] = ["portrait", "landscape"];
  const gaps = [2, 3, 5];
  const results: MultiUpLayout[] = [];
  const seen = new Set<number>();

  for (const orientation of orientations) {
    for (const gap of gaps) {
      const config = createDefaultMultiUpConfig(
        templateWidthMm,
        templateHeightMm,
      );
      config.sheetSize = sheetSize;
      config.sheetOrientation = orientation;
      config.gapHorizontalMm = gap;
      config.gapVerticalMm = gap;

      const layout = computeMultiUpLayout(config);

      if (layout.itemsPerSheet > 0 && !seen.has(layout.itemsPerSheet)) {
        seen.add(layout.itemsPerSheet);
        results.push(layout);
      }
    }
  }

  // Sort by items per sheet descending
  results.sort((a, b) => b.itemsPerSheet - a.itemsPerSheet);
  return results;
}

/**
 * Get human-readable label for a layout.
 * e.g., "2×4 = 8 per sheet (Portrait)"
 */
export function layoutLabel(layout: MultiUpLayout): string {
  const orientation =
    layout.sheetWidthMm < layout.sheetHeightMm ? "Portrait" : "Landscape";
  return `${layout.columns}×${layout.rows} = ${layout.itemsPerSheet} per sheet (${orientation})`;
}

/**
 * Get Arabic label for a layout.
 * e.g., "2×4 = 8 مستند/صفحة (طولي)"
 */
export function layoutLabelAr(layout: MultiUpLayout): string {
  const orientation =
    layout.sheetWidthMm < layout.sheetHeightMm ? "طولي" : "عرضي";
  return `${layout.columns}×${layout.rows} = ${layout.itemsPerSheet} مستند/صفحة (${orientation})`;
}
