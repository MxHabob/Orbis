// ═══════════════════════════════════════════════════════════════
// @docforge/core — Hit Testing
// Determines which element is under the mouse cursor.
// Used by the Canvas editor for selection and interaction.
// ═══════════════════════════════════════════════════════════════

import type { TemplateNode, PageNode } from "../model/nodes";
import { isContainerNode, hasPosition } from "../model/nodes";

// ── Types ────────────────────────────────────────────────────

/** Result of a hit test */
export interface HitTestResult {
  /** The node that was hit */
  node: TemplateNode;
  /** The node's ID */
  nodeId: string;
  /** Ancestor chain from page root → hit node */
  path: string[];
  /** Depth in the tree (0 = page, 1 = direct child, etc.) */
  depth: number;
}

/** Bounding box in document coordinates (mm) */
export interface BoundingBox {
  x: number;
  y: number;
  width: number;
  height: number;
}

/** Computed layout info for a node, provided by the layout resolver */
export interface NodeLayoutInfo {
  nodeId: string;
  bounds: BoundingBox;
  /** Whether the node is visible */
  visible: boolean;
  /** Whether the node is locked */
  locked: boolean;
  /** Z-index for layering */
  zIndex: number;
}

// ── Hit Testing Engine ───────────────────────────────────────

/**
 * Perform a hit test at the given document coordinates (mm).
 * Returns the topmost (highest z-index) visible, unlocked node under the point.
 *
 * @param pointMm - Point in document mm coordinates
 * @param layouts - Pre-computed layout info for all nodes on the current page
 * @param opts - Options for filtering
 * @returns The hit node, or null if nothing is under the point
 */
export function hitTest(
  pointMm: { x: number; y: number },
  layouts: NodeLayoutInfo[],
  opts: HitTestOptions = {},
): HitTestResult | null {
  const {
    includeLockedNodes = false,
    includeHiddenNodes = false,
    hitTolerance = 1.0, // mm tolerance for edge hits
  } = opts;

  // Sort by z-index descending — topmost first
  const sorted = [...layouts].sort((a, b) => b.zIndex - a.zIndex);

  for (const layout of sorted) {
    if (!layout.visible && !includeHiddenNodes) continue;
    if (layout.locked && !includeLockedNodes) continue;

    if (isPointInBounds(pointMm, layout.bounds, hitTolerance)) {
      return {
        node: null as unknown as TemplateNode, // resolved by caller
        nodeId: layout.nodeId,
        path: [], // resolved by caller
        depth: 0,
      };
    }
  }

  return null;
}

export interface HitTestOptions {
  /** Include locked nodes in hit test results */
  includeLockedNodes?: boolean;
  /** Include hidden nodes in hit test results */
  includeHiddenNodes?: boolean;
  /** Tolerance in mm for edge/corner hits (default: 1.0) */
  hitTolerance?: number;
}

/**
 * Check if a point is inside a bounding box (with tolerance).
 */
export function isPointInBounds(
  point: { x: number; y: number },
  bounds: BoundingBox,
  tolerance = 0,
): boolean {
  return (
    point.x >= bounds.x - tolerance &&
    point.x <= bounds.x + bounds.width + tolerance &&
    point.y >= bounds.y - tolerance &&
    point.y <= bounds.y + bounds.height + tolerance
  );
}

/**
 * Check if two bounding boxes overlap (for rubber-band selection).
 */
export function doBoundsOverlap(a: BoundingBox, b: BoundingBox): boolean {
  return !(
    a.x + a.width < b.x ||
    b.x + b.width < a.x ||
    a.y + a.height < b.y ||
    b.y + b.height < a.y
  );
}

/**
 * Get all nodes within a selection rectangle (rubber-band selection).
 * Returns node IDs of all visible, unlocked nodes that overlap the rect.
 */
export function hitTestRect(
  rectMm: BoundingBox,
  layouts: NodeLayoutInfo[],
  opts: HitTestOptions = {},
): string[] {
  const { includeLockedNodes = false, includeHiddenNodes = false } = opts;

  const hits: string[] = [];

  for (const layout of layouts) {
    if (!layout.visible && !includeHiddenNodes) continue;
    if (layout.locked && !includeLockedNodes) continue;

    if (doBoundsOverlap(rectMm, layout.bounds)) {
      hits.push(layout.nodeId);
    }
  }

  return hits;
}

// ── Resize Handle Detection ─────────────────────────────────

/** Which resize handle is under the cursor */
export type ResizeHandle =
  | "top-left"
  | "top-center"
  | "top-right"
  | "middle-left"
  | "middle-right"
  | "bottom-left"
  | "bottom-center"
  | "bottom-right"
  | "rotate"
  | null;

/**
 * Detect which resize handle of a selected node is under the cursor.
 * Handle size is in mm.
 */
export function detectResizeHandle(
  pointMm: { x: number; y: number },
  bounds: BoundingBox,
  handleSizeMm = 2.5,
): ResizeHandle {
  const hs = handleSizeMm;
  const { x, y, width: w, height: h } = bounds;

  // Rotation handle (above top-center)
  const rotateY = y - hs * 3;
  const rotateCx = x + w / 2;
  if (
    isPointInBounds(pointMm, {
      x: rotateCx - hs,
      y: rotateY - hs,
      width: hs * 2,
      height: hs * 2,
    })
  ) {
    return "rotate";
  }

  // Corner handles (checked first — they take priority)
  if (
    isPointInBounds(pointMm, {
      x: x - hs,
      y: y - hs,
      width: hs * 2,
      height: hs * 2,
    })
  )
    return "top-left";
  if (
    isPointInBounds(pointMm, {
      x: x + w - hs,
      y: y - hs,
      width: hs * 2,
      height: hs * 2,
    })
  )
    return "top-right";
  if (
    isPointInBounds(pointMm, {
      x: x - hs,
      y: y + h - hs,
      width: hs * 2,
      height: hs * 2,
    })
  )
    return "bottom-left";
  if (
    isPointInBounds(pointMm, {
      x: x + w - hs,
      y: y + h - hs,
      width: hs * 2,
      height: hs * 2,
    })
  )
    return "bottom-right";

  // Edge handles
  if (
    isPointInBounds(pointMm, {
      x: x + w / 2 - hs,
      y: y - hs,
      width: hs * 2,
      height: hs * 2,
    })
  )
    return "top-center";
  if (
    isPointInBounds(pointMm, {
      x: x + w / 2 - hs,
      y: y + h - hs,
      width: hs * 2,
      height: hs * 2,
    })
  )
    return "bottom-center";
  if (
    isPointInBounds(pointMm, {
      x: x - hs,
      y: y + h / 2 - hs,
      width: hs * 2,
      height: hs * 2,
    })
  )
    return "middle-left";
  if (
    isPointInBounds(pointMm, {
      x: x + w - hs,
      y: y + h / 2 - hs,
      width: hs * 2,
      height: hs * 2,
    })
  )
    return "middle-right";

  return null;
}

// ── Bounding Box Utilities ───────────────────────────────────

/**
 * Compute the combined bounding box of multiple nodes.
 * Used for multi-selection bounds and group sizing.
 */
export function computeCombinedBounds(boxes: BoundingBox[]): BoundingBox {
  if (boxes.length === 0) {
    return { x: 0, y: 0, width: 0, height: 0 };
  }

  let minX = Infinity;
  let minY = Infinity;
  let maxX = -Infinity;
  let maxY = -Infinity;

  for (const box of boxes) {
    minX = Math.min(minX, box.x);
    minY = Math.min(minY, box.y);
    maxX = Math.max(maxX, box.x + box.width);
    maxY = Math.max(maxY, box.y + box.height);
  }

  return {
    x: minX,
    y: minY,
    width: maxX - minX,
    height: maxY - minY,
  };
}
