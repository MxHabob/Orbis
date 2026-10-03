// ═══════════════════════════════════════════════════════════════
// @docforge/core — Alignment & Distribution
// Algorithms for aligning and distributing selected elements.
// All calculations in document coordinates (mm).
// ═══════════════════════════════════════════════════════════════

import type { BoundingBox } from "./hit-testing";
import { roundMm } from "./mm-px-transform";

// ── Alignment ────────────────────────────────────────────────

export type AlignDirection =
  | "left"
  | "center"
  | "right"
  | "top"
  | "middle"
  | "bottom";

/** Result of an alignment calculation: node ID → new position */
export interface AlignmentResult {
  nodeId: string;
  newX: number;
  newY: number;
}

/**
 * Calculate new positions to align multiple nodes.
 *
 * @param nodes - Array of { id, bounds } for selected nodes
 * @param direction - Alignment direction
 * @param reference - Optional reference bounds (e.g., page bounds). If omitted, uses the combined bounds of selected nodes.
 * @returns Array of new positions for each node
 */
export function calcAlignment(
  nodes: { id: string; bounds: BoundingBox }[],
  direction: AlignDirection,
  reference?: BoundingBox,
): AlignmentResult[] {
  if (nodes.length === 0) return [];

  // Determine reference bounds
  const ref = reference ?? computeReferenceBounds(nodes);
  const results: AlignmentResult[] = [];

  for (const node of nodes) {
    const { bounds } = node;
    let newX = bounds.x;
    let newY = bounds.y;

    switch (direction) {
      case "left":
        newX = ref.x;
        break;
      case "center":
        newX = ref.x + (ref.width - bounds.width) / 2;
        break;
      case "right":
        newX = ref.x + ref.width - bounds.width;
        break;
      case "top":
        newY = ref.y;
        break;
      case "middle":
        newY = ref.y + (ref.height - bounds.height) / 2;
        break;
      case "bottom":
        newY = ref.y + ref.height - bounds.height;
        break;
    }

    results.push({
      nodeId: node.id,
      newX: roundMm(newX),
      newY: roundMm(newY),
    });
  }

  return results;
}

// ── Distribution ─────────────────────────────────────────────

export type DistributeDirection = "horizontal" | "vertical";

/**
 * Calculate new positions to distribute nodes evenly.
 * Requires at least 3 nodes to be meaningful.
 *
 * @param nodes - Array of { id, bounds } for selected nodes (must be ≥ 3)
 * @param direction - Distribution axis
 * @returns Array of new positions for each node
 */
export function calcDistribution(
  nodes: { id: string; bounds: BoundingBox }[],
  direction: DistributeDirection,
): AlignmentResult[] {
  if (nodes.length < 3)
    return nodes.map((n) => ({
      nodeId: n.id,
      newX: n.bounds.x,
      newY: n.bounds.y,
    }));

  const isHorizontal = direction === "horizontal";

  // Sort by position on the distribution axis
  const sorted = [...nodes].sort((a, b) =>
    isHorizontal ? a.bounds.x - b.bounds.x : a.bounds.y - b.bounds.y,
  );

  const first = sorted[0]!;
  const last = sorted[sorted.length - 1]!;

  if (isHorizontal) {
    const totalSpan = last.bounds.x + last.bounds.width - first.bounds.x;
    const totalNodeWidth = sorted.reduce((sum, n) => sum + n.bounds.width, 0);
    const gap = (totalSpan - totalNodeWidth) / (sorted.length - 1);

    let currentX = first.bounds.x;
    return sorted.map((node) => {
      const result: AlignmentResult = {
        nodeId: node.id,
        newX: roundMm(currentX),
        newY: node.bounds.y,
      };
      currentX += node.bounds.width + gap;
      return result;
    });
  } else {
    const totalSpan = last.bounds.y + last.bounds.height - first.bounds.y;
    const totalNodeHeight = sorted.reduce((sum, n) => sum + n.bounds.height, 0);
    const gap = (totalSpan - totalNodeHeight) / (sorted.length - 1);

    let currentY = first.bounds.y;
    return sorted.map((node) => {
      const result: AlignmentResult = {
        nodeId: node.id,
        newX: node.bounds.x,
        newY: roundMm(currentY),
      };
      currentY += node.bounds.height + gap;
      return result;
    });
  }
}

// ── Smart Snap Lines ─────────────────────────────────────────

/** A snap guide line shown during dragging */
export interface SnapLine {
  /** Axis of the snap line */
  axis: "horizontal" | "vertical";
  /** Position in mm on the perpendicular axis */
  position: number;
  /** Start point on the parallel axis (for rendering) */
  start: number;
  /** End point on the parallel axis (for rendering) */
  end: number;
  /** Type of snap source */
  type: "edge" | "center" | "guide" | "grid";
}

/** Snap result after snapping a moving element */
export interface SnapResult {
  /** Snapped X position in mm */
  x: number;
  /** Snapped Y position in mm */
  y: number;
  /** Active snap lines to render */
  snapLines: SnapLine[];
}

/**
 * Calculate snap positions for a moving element against other elements.
 *
 * @param movingBounds - The bounds of the element being moved
 * @param otherNodes - Bounds of other (stationary) nodes
 * @param pageBounds - Page content area bounds
 * @param snapThresholdMm - Distance threshold for snapping (default: 3mm)
 * @returns Snapped position and active snap lines
 */
export function calcSmartSnap(
  movingBounds: BoundingBox,
  otherNodes: BoundingBox[],
  pageBounds: BoundingBox,
  snapThresholdMm = 3,
): SnapResult {
  const snapLines: SnapLine[] = [];
  let snappedX = movingBounds.x;
  let snappedY = movingBounds.y;

  // Collect snap targets: edges and centers of other nodes + page bounds
  const vTargets: number[] = []; // vertical lines (x positions)
  const hTargets: number[] = []; // horizontal lines (y positions)

  // Page edges
  vTargets.push(
    pageBounds.x,
    pageBounds.x + pageBounds.width,
    pageBounds.x + pageBounds.width / 2,
  );
  hTargets.push(
    pageBounds.y,
    pageBounds.y + pageBounds.height,
    pageBounds.y + pageBounds.height / 2,
  );

  // Other nodes
  for (const b of otherNodes) {
    vTargets.push(b.x, b.x + b.width, b.x + b.width / 2);
    hTargets.push(b.y, b.y + b.height, b.y + b.height / 2);
  }

  // Moving element reference points
  const movingLeft = movingBounds.x;
  const movingRight = movingBounds.x + movingBounds.width;
  const movingCenterX = movingBounds.x + movingBounds.width / 2;
  const movingTop = movingBounds.y;
  const movingBottom = movingBounds.y + movingBounds.height;
  const movingCenterY = movingBounds.y + movingBounds.height / 2;

  // Find closest vertical snap
  let bestVDist = snapThresholdMm;
  for (const target of vTargets) {
    for (const ref of [movingLeft, movingRight, movingCenterX]) {
      const dist = Math.abs(ref - target);
      if (dist < bestVDist) {
        bestVDist = dist;
        snappedX = movingBounds.x + (target - ref);
        // Clear previous V snap lines and add new one
        const vLines = snapLines.filter((l) => l.axis === "horizontal");
        snapLines.length = 0;
        snapLines.push(...vLines);
        snapLines.push({
          axis: "vertical",
          position: target,
          start: Math.min(movingTop, pageBounds.y),
          end: Math.max(movingBottom, pageBounds.y + pageBounds.height),
          type: "edge",
        });
      }
    }
  }

  // Find closest horizontal snap
  let bestHDist = snapThresholdMm;
  for (const target of hTargets) {
    for (const ref of [movingTop, movingBottom, movingCenterY]) {
      const dist = Math.abs(ref - target);
      if (dist < bestHDist) {
        bestHDist = dist;
        snappedY = movingBounds.y + (target - ref);
        snapLines.push({
          axis: "horizontal",
          position: target,
          start: Math.min(movingLeft, pageBounds.x),
          end: Math.max(movingRight, pageBounds.x + pageBounds.width),
          type: "edge",
        });
      }
    }
  }

  return {
    x: roundMm(snappedX),
    y: roundMm(snappedY),
    snapLines,
  };
}

// ── Helpers ──────────────────────────────────────────────────

function computeReferenceBounds(nodes: { bounds: BoundingBox }[]): BoundingBox {
  let minX = Infinity,
    minY = Infinity;
  let maxX = -Infinity,
    maxY = -Infinity;

  for (const { bounds } of nodes) {
    minX = Math.min(minX, bounds.x);
    minY = Math.min(minY, bounds.y);
    maxX = Math.max(maxX, bounds.x + bounds.width);
    maxY = Math.max(maxY, bounds.y + bounds.height);
  }

  return { x: minX, y: minY, width: maxX - minX, height: maxY - minY };
}
