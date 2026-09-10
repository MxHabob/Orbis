// ═══════════════════════════════════════════════════════════════
// @docforge/core — Standard Page Size Definitions
// All dimensions in millimeters
// ═══════════════════════════════════════════════════════════════

import type { Orientation, PageSize } from "../model/document";

export interface PageDimensions {
  width: number;
  height: number;
}

/**
 * Standard page sizes in portrait orientation (width × height in mm).
 * For landscape, width and height are swapped.
 */
export const PAGE_SIZES: Record<PageSize, PageDimensions> = {
  A3: { width: 297, height: 420 },
  A4: { width: 210, height: 297 },
  A5: { width: 148, height: 210 },
  Letter: { width: 216, height: 279 },
  Legal: { width: 216, height: 356 },
  ID_CARD: { width: 53.98, height: 85.6 },
  BADGE: { width: 54, height: 86 },
  Custom: { width: 210, height: 297 }, // default to A4
};

/**
 * Get the effective page dimensions accounting for orientation.
 * For landscape, width and height are swapped.
 */
export function getPageDimensions(
  size: PageSize,
  orientation: Orientation,
  customWidth?: number,
  customHeight?: number,
): PageDimensions {
  let dims: PageDimensions;

  if (size === "Custom" && customWidth && customHeight) {
    dims = { width: customWidth, height: customHeight };
  } else {
    dims = { ...PAGE_SIZES[size] };
  }

  if (orientation === "landscape") {
    return { width: dims.height, height: dims.width };
  }

  return dims;
}

/**
 * Calculate the printable area within margins.
 */
export function getPrintableArea(
  pageWidth: number,
  pageHeight: number,
  margins: { top: number; bottom: number; left: number; right: number },
): { x: number; y: number; width: number; height: number } {
  return {
    x: margins.left,
    y: margins.top,
    width: pageWidth - margins.left - margins.right,
    height: pageHeight - margins.top - margins.bottom,
  };
}
