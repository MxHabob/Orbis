// ═══════════════════════════════════════════════════════════════
// @docforge/core — Canvas Utilities Barrel Export
// ═══════════════════════════════════════════════════════════════

export type {
  AlignDirection,
  AlignmentResult,
  DistributeDirection,
  SnapLine,
  SnapResult,
} from "./alignment";
export {
  calcAlignment,
  calcDistribution,
  calcSmartSnap,
} from "./alignment";
export { groupNodes, ungroupNode } from "./grouping";
export type {
  BoundingBox,
  HitTestOptions,
  HitTestResult,
  NodeLayoutInfo,
  ResizeHandle,
} from "./hit-testing";
export {
  computeCombinedBounds,
  detectResizeHandle,
  doBoundsOverlap,
  hitTest,
  hitTestRect,
  isPointInBounds,
} from "./hit-testing";
export type { ViewportTransform } from "./mm-px-transform";
export {
  calcZoomToFit,
  clampZoom,
  createDefaultViewport,
  DEFAULT_CANVAS_DPI,
  docToScreen,
  getPageSizePx,
  MAX_ZOOM,
  MIN_ZOOM,
  MM_PER_INCH,
  mmToPt,
  mmToPx,
  PDF_POINTS_PER_INCH,
  ptToMm,
  pxToMm,
  roundMm,
  screenToDoc,
  snapToGrid,
  ZOOM_PRESETS,
  ZOOM_STEP,
} from "./mm-px-transform";
