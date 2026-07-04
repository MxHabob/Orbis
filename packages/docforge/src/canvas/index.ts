// ═══════════════════════════════════════════════════════════════
// @docforge/core — Canvas Utilities Barrel Export
// ═══════════════════════════════════════════════════════════════

export {
  mmToPx,
  pxToMm,
  mmToPt,
  ptToMm,
  docToScreen,
  screenToDoc,
  getPageSizePx,
  calcZoomToFit,
  clampZoom,
  snapToGrid,
  roundMm,
  createDefaultViewport,
  DEFAULT_CANVAS_DPI,
  PDF_POINTS_PER_INCH,
  MM_PER_INCH,
  MIN_ZOOM,
  MAX_ZOOM,
  ZOOM_STEP,
  ZOOM_PRESETS,
} from "./mm-px-transform";
export type { ViewportTransform } from "./mm-px-transform";

export {
  hitTest,
  hitTestRect,
  isPointInBounds,
  doBoundsOverlap,
  detectResizeHandle,
  computeCombinedBounds,
} from "./hit-testing";
export type {
  HitTestResult,
  HitTestOptions,
  BoundingBox,
  NodeLayoutInfo,
  ResizeHandle,
} from "./hit-testing";

export {
  calcAlignment,
  calcDistribution,
  calcSmartSnap,
} from "./alignment";
export type {
  AlignDirection,
  DistributeDirection,
  AlignmentResult,
  SnapLine,
  SnapResult,
} from "./alignment";

export { groupNodes, ungroupNode } from "./grouping";
