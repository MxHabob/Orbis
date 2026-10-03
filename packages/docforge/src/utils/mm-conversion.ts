// ═══════════════════════════════════════════════════════════════
// @docforge/core — Unit Conversion Utilities
// Precise mm ↔ px ↔ pt conversion for print-quality layout
// ═══════════════════════════════════════════════════════════════

const MM_PER_INCH = 25.4;
const PT_PER_INCH = 72;
const DEFAULT_DPI = 96;

/** Convert millimeters to pixels at a given DPI (default 96) */
export function mmToPx(mm: number, dpi = DEFAULT_DPI): number {
  return (mm / MM_PER_INCH) * dpi;
}

/** Convert pixels to millimeters at a given DPI (default 96) */
export function pxToMm(px: number, dpi = DEFAULT_DPI): number {
  return (px / dpi) * MM_PER_INCH;
}

/** Convert points to millimeters */
export function ptToMm(pt: number): number {
  return (pt / PT_PER_INCH) * MM_PER_INCH;
}

/** Convert millimeters to points */
export function mmToPt(mm: number): number {
  return (mm / MM_PER_INCH) * PT_PER_INCH;
}

/** Convert points to pixels at a given DPI */
export function ptToPx(pt: number, dpi = DEFAULT_DPI): number {
  return (pt / PT_PER_INCH) * dpi;
}

/** Convert pixels to points at a given DPI */
export function pxToPt(px: number, dpi = DEFAULT_DPI): number {
  return (px / dpi) * PT_PER_INCH;
}
