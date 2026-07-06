// ═══════════════════════════════════════════════════════════════
// @docforge/core — mm ↔ px Coordinate Transform
// Converts between document millimeters and canvas pixels.
// Ensures 100% WYSIWYG parity between Canvas and PDF output.
//
// All document values are stored in mm.
// Canvas rendering uses px at a configurable DPI × zoom level.
// PDF output uses 1pt = 1/72in (managed by Rust renderer).
// ═══════════════════════════════════════════════════════════════

/** Default screen DPI for canvas rendering */
export const DEFAULT_CANVAS_DPI = 96;

/** PDF points per inch (standard) */
export const PDF_POINTS_PER_INCH = 72;

/** Millimeters per inch */
export const MM_PER_INCH = 25.4;

// ── Conversion Functions ─────────────────────────────────────

/** Convert mm to canvas pixels at a given DPI and zoom level */
export function mmToPx(
  mm: number,
  dpi = DEFAULT_CANVAS_DPI,
  zoom = 1.0,
): number {
  return (mm / MM_PER_INCH) * dpi * zoom;
}

/** Convert canvas pixels to mm at a given DPI and zoom level */
export function pxToMm(
  px: number,
  dpi = DEFAULT_CANVAS_DPI,
  zoom = 1.0,
): number {
  return (px / (dpi * zoom)) * MM_PER_INCH;
}

/** Convert mm to PDF points (1pt = 1/72in) */
export function mmToPt(mm: number): number {
  return (mm / MM_PER_INCH) * PDF_POINTS_PER_INCH;
}

/** Convert PDF points to mm */
export function ptToMm(pt: number): number {
  return (pt / PDF_POINTS_PER_INCH) * MM_PER_INCH;
}

// ── Viewport Transform ───────────────────────────────────────

/**
 * Represents the canvas viewport transform state.
 * All canvas coordinates go through this transform.
 */
export interface ViewportTransform {
  /** Horizontal pan offset in pixels */
  panX: number;
  /** Vertical pan offset in pixels */
  panY: number;
  /** Zoom level (1.0 = 100%) */
  zoom: number;
  /** Canvas DPI */
  dpi: number;
}

/** Create a default viewport transform */
export function createDefaultViewport(
  dpi = DEFAULT_CANVAS_DPI,
): ViewportTransform {
  return {
    panX: 0,
    panY: 0,
    zoom: 1.0,
    dpi,
  };
}

/**
 * Convert document coordinates (mm) to canvas screen coordinates (px).
 * Takes into account zoom, pan offset, and DPI.
 */
export function docToScreen(
  docX: number,
  docY: number,
  viewport: ViewportTransform,
): { x: number; y: number } {
  return {
    x: mmToPx(docX, viewport.dpi, viewport.zoom) + viewport.panX,
    y: mmToPx(docY, viewport.dpi, viewport.zoom) + viewport.panY,
  };
}

/**
 * Convert canvas screen coordinates (px) to document coordinates (mm).
 * Inverse of docToScreen — used for mouse position to document position.
 */
export function screenToDoc(
  screenX: number,
  screenY: number,
  viewport: ViewportTransform,
): { x: number; y: number } {
  return {
    x: pxToMm(screenX - viewport.panX, viewport.dpi, viewport.zoom),
    y: pxToMm(screenY - viewport.panY, viewport.dpi, viewport.zoom),
  };
}

/**
 * Get the size of a document page in canvas pixels.
 */
export function getPageSizePx(
  widthMm: number,
  heightMm: number,
  viewport: ViewportTransform,
): { width: number; height: number } {
  return {
    width: mmToPx(widthMm, viewport.dpi, viewport.zoom),
    height: mmToPx(heightMm, viewport.dpi, viewport.zoom),
  };
}

// ── Zoom Utilities ───────────────────────────────────────────

/** Minimum zoom level (10%) */
export const MIN_ZOOM = 0.1;
/** Maximum zoom level (500%) */
export const MAX_ZOOM = 5.0;
/** Zoom step for keyboard shortcuts */
export const ZOOM_STEP = 0.1;

/** Standard zoom presets */
export const ZOOM_PRESETS = [
  0.25, 0.5, 0.75, 1.0, 1.25, 1.5, 2.0, 3.0, 4.0,
] as const;

/**
 * Calculate zoom-to-fit for a page within a container.
 * Returns the zoom level that makes the page fill the container with padding.
 */
export function calcZoomToFit(
  pageWidthMm: number,
  pageHeightMm: number,
  containerWidthPx: number,
  containerHeightPx: number,
  paddingPx = 40,
  dpi = DEFAULT_CANVAS_DPI,
): number {
  const availWidth = containerWidthPx - paddingPx * 2;
  const availHeight = containerHeightPx - paddingPx * 2;

  const pageWidthPx = mmToPx(pageWidthMm, dpi, 1.0);
  const pageHeightPx = mmToPx(pageHeightMm, dpi, 1.0);

  const zoomX = availWidth / pageWidthPx;
  const zoomY = availHeight / pageHeightPx;

  return Math.min(zoomX, zoomY, MAX_ZOOM);
}

/** Clamp a zoom value to valid range */
export function clampZoom(zoom: number): number {
  return Math.max(MIN_ZOOM, Math.min(MAX_ZOOM, zoom));
}

// ── Snap / Rounding ──────────────────────────────────────────

/**
 * Snap a value in mm to the nearest grid point.
 * @param valueMm - The value to snap
 * @param gridSizeMm - Grid spacing in mm (e.g., 1, 2, 5)
 * @returns The snapped value
 */
export function snapToGrid(valueMm: number, gridSizeMm: number): number {
  if (gridSizeMm <= 0) return valueMm;
  return Math.round(valueMm / gridSizeMm) * gridSizeMm;
}

/**
 * Round mm value to a reasonable precision (0.01mm = 10μm).
 * Avoids floating-point noise in position calculations.
 */
export function roundMm(mm: number): number {
  return Math.round(mm * 100) / 100;
}
