<script lang="ts">
/**
 * SelectionOverlay — Renders selection handles, resize grips,
 * rotation handle, and rubber-band multi-select.
 *
 * Phase B enhancements:
 *   - Rotation handle (circle above element)
 *   - Rubber band selection (drag on empty canvas)
 *   - Selection outline with dashed border
 *   - Dimension label
 */
import type { EditorStore } from "../stores/editor-store.svelte";
import * as m from "$lib/paraglide/messages";
import { mmToPx, pxToMm } from "@docforge/core/canvas";
import { findNode, getPages } from "@docforge/core";
import type { SnapLine, BoundingBox } from "@docforge/core";
import { calcSmartSnap } from "@docforge/core";
import SnapLinesOverlay from "./snap-lines-overlay.svelte";

let {
  store,
  pageSizePx,
}: {
  store: EditorStore;
  pageSizePx: { width: number; height: number };
} = $props();

const vp = $derived(store.viewport);
const HANDLE_SIZE = 8;
const ROTATION_OFFSET = 14; // px above the element

// Compute selection bounds for each selected node
const selectionRects = $derived(
  store.selectedIds
    .map((id) => {
      const node = findNode(store.document, id);
      if (!node) return null;
      const pos = (node as any).props?.position ?? (node as any).position;
      if (!pos) return null;
      const x = mmToPx(pos.x ?? 0, vp.dpi, vp.zoom);
      const y = mmToPx(pos.y ?? 0, vp.dpi, vp.zoom);

      let w: number;
      if (typeof pos.width === "string" && pos.width.endsWith("%")) {
        w = (parseFloat(pos.width) / 100) * pageSizePx.width;
      } else {
        w = mmToPx(Number(pos.width) || 50, vp.dpi, vp.zoom);
      }

      let h: number;
      if (pos.height == null) {
        h = mmToPx(20, vp.dpi, vp.zoom);
      } else if (typeof pos.height === "string" && pos.height.endsWith("%")) {
        h = (parseFloat(pos.height) / 100) * pageSizePx.height;
      } else {
        h = mmToPx(Number(pos.height) || 20, vp.dpi, vp.zoom);
      }

      const rotation = (node as any).rotation ?? 0;

      return { id, x, y, w, h, rotation };
    })
    .filter(Boolean) as {
    id: string;
    x: number;
    y: number;
    w: number;
    h: number;
    rotation: number;
  }[],
);

// ── Drag State ───────────────────────────────────────────────
let isDragging = $state(false);
let dragHandle = $state<string | null>(null);
let dragNodeId = $state<string | null>(null);
let dragStartX = 0;
let dragStartY = 0;
let dragOrigX = 0;
let dragOrigY = 0;
let dragOrigW = 0;
let dragOrigH = 0;
let dragOrigPositions = new Map<string, { x: number; y: number }>();
let dragOtherBounds: BoundingBox[] = [];
let activeSnapLines = $state<SnapLine[]>([]);

// ── Rotation State ───────────────────────────────────────────
let isRotating = $state(false);
let rotateNodeId = $state<string | null>(null);
let rotateCenterX = 0;
let rotateCenterY = 0;
let rotateStartAngle = 0;
let rotateOrigAngle = 0;

// ── Rubber Band State ────────────────────────────────────────
let isRubberBanding = $state(false);
let rubberBandStartX = $state(0);
let rubberBandStartY = $state(0);
let rubberBandEndX = $state(0);
let rubberBandEndY = $state(0);

const rubberBandRect = $derived.by(() => {
  if (!isRubberBanding) return null;
  const x = Math.min(rubberBandStartX, rubberBandEndX);
  const y = Math.min(rubberBandStartY, rubberBandEndY);
  const w = Math.abs(rubberBandEndX - rubberBandStartX);
  const h = Math.abs(rubberBandEndY - rubberBandStartY);
  return { x, y, w, h };
});

// ── Drag Handlers ────────────────────────────────────────────

function startDrag(e: MouseEvent, nodeId: string, handle: string) {
  e.stopPropagation();
  e.preventDefault();
  isDragging = true;
  dragHandle = handle;
  dragNodeId = nodeId;
  store.beginLiveMove(
    handle === "move"
      ? m.shared_ui_move_elements()
      : m.shared_ui_resize_elements(),
  );
  dragStartX = e.clientX;
  dragStartY = e.clientY;

  const rect = selectionRects.find((r) => r.id === nodeId);
  if (rect) {
    dragOrigX = rect.x;
    dragOrigY = rect.y;
    dragOrigW = rect.w;
    dragOrigH = rect.h;
  }

  dragOrigPositions.clear();
  for (const id of store.selectedIds) {
    const node = findNode(store.document, id);
    const position = (node as any)?.props?.position ?? (node as any)?.position;
    if (position) {
      dragOrigPositions.set(id, {
        x: position.x ?? 0,
        y: position.y ?? 0,
      });
    }
  }
  dragOtherBounds = collectOtherNodeBounds(new Set(store.selectedIds));

  window.addEventListener("mousemove", onDragMove);
  window.addEventListener("mouseup", onDragEnd);
}

function onDragMove(e: MouseEvent) {
  if (!isDragging || !dragNodeId) return;
  const dx = e.clientX - dragStartX;
  const dy = e.clientY - dragStartY;

  if (dragHandle === "move") {
    const rawX = pxToMm(dragOrigX + dx, vp.dpi, vp.zoom);
    const rawY = pxToMm(dragOrigY + dy, vp.dpi, vp.zoom);
    const rawW = pxToMm(dragOrigW, vp.dpi, vp.zoom);
    const rawH = pxToMm(dragOrigH, vp.dpi, vp.zoom);

    const draggedOrigX = pxToMm(dragOrigX, vp.dpi, vp.zoom);
    const draggedOrigY = pxToMm(dragOrigY, vp.dpi, vp.zoom);

    let finalX = Math.round(rawX * 10) / 10;
    let finalY = Math.round(rawY * 10) / 10;

    if (store.snapEnabled) {
      const pageSetup = store.document.meta.pageSetup;
      const pageBounds: BoundingBox = {
        x: pageSetup.margins.left,
        y: pageSetup.margins.top,
        width:
          getPageW(pageSetup) -
          pageSetup.margins.left -
          pageSetup.margins.right,
        height:
          getPageH(pageSetup) -
          pageSetup.margins.top -
          pageSetup.margins.bottom,
      };

      const snapResult = calcSmartSnap(
        { x: rawX, y: rawY, width: rawW, height: rawH },
        dragOtherBounds,
        pageBounds,
        3,
      );
      finalX = snapResult.x;
      finalY = snapResult.y;
      activeSnapLines = snapResult.snapLines;
    } else {
      activeSnapLines = [];
    }

    const deltaX = finalX - draggedOrigX;
    const deltaY = finalY - draggedOrigY;

    const updates = [];
    for (const id of store.selectedIds) {
      if (id === dragNodeId) {
        updates.push({ id, x: finalX, y: finalY });
      } else {
        const origPos = dragOrigPositions.get(id);
        if (origPos) {
          updates.push({
            id,
            x: Math.round((origPos.x + deltaX) * 10) / 10,
            y: Math.round((origPos.y + deltaY) * 10) / 10,
          });
        }
      }
    }
    store.moveNodes(updates);
  } else if (dragHandle) {
    let newW = dragOrigW;
    let newH = dragOrigH;
    let newX = dragOrigX;
    let newY = dragOrigY;

    if (dragHandle.includes("right")) newW = dragOrigW + dx;
    if (dragHandle.includes("left")) {
      newW = dragOrigW - dx;
      newX = dragOrigX + dx;
    }
    if (dragHandle.includes("bottom")) newH = dragOrigH + dy;
    if (dragHandle.includes("top")) {
      newH = dragOrigH - dy;
      newY = dragOrigY + dy;
    }

    // Shift key: constrain aspect ratio
    if (e.shiftKey && dragOrigW > 0 && dragOrigH > 0) {
      const ratio = dragOrigW / dragOrigH;
      if (Math.abs(dx) > Math.abs(dy)) {
        newH = newW / ratio;
      } else {
        newW = newH * ratio;
      }
    }

    const wMm = Math.max(
      5,
      Math.round(pxToMm(newW, vp.dpi, vp.zoom) * 10) / 10,
    );
    const hMm = Math.max(
      5,
      Math.round(pxToMm(newH, vp.dpi, vp.zoom) * 10) / 10,
    );

    store.resizeSelected(wMm, hMm);
    if (dragHandle.includes("left") || dragHandle.includes("top")) {
      store.moveNodes([
        {
          id: dragNodeId,
          x: Math.round(pxToMm(newX, vp.dpi, vp.zoom) * 10) / 10,
          y: Math.round(pxToMm(newY, vp.dpi, vp.zoom) * 10) / 10,
        },
      ]);
    }
  }
}

function onDragEnd() {
  // Commit the drag as a single history entry
  store.commitLiveMove();
  isDragging = false;
  dragHandle = null;
  dragNodeId = null;
  activeSnapLines = [];
  window.removeEventListener("mousemove", onDragMove);
  window.removeEventListener("mouseup", onDragEnd);
}

// ── Rotation Handlers ────────────────────────────────────────

function startRotation(
  e: MouseEvent,
  nodeId: string,
  rect: { x: number; y: number; w: number; h: number },
) {
  e.stopPropagation();
  e.preventDefault();
  isRotating = true;
  store.beginLiveMove(m.shared_ui_update_multiple_elements());
  rotateNodeId = nodeId;

  // Center of the element
  rotateCenterX = rect.x + rect.w / 2;
  rotateCenterY = rect.y + rect.h / 2;

  // Get current angle
  const node = findNode(store.document, nodeId);
  rotateOrigAngle = (node as any)?.rotation ?? 0;

  // Starting mouse angle
  const svgEl = (e.target as SVGElement).closest("svg");
  if (svgEl) {
    const svgRect = svgEl.getBoundingClientRect();
    const gEl = svgEl.querySelector("g.page-transform");
    const gRect = gEl?.getBoundingClientRect();
    const offsetX = gRect ? gRect.left - svgRect.left : 0;
    const offsetY = gRect ? gRect.top - svgRect.top : 0;
    const mouseX = e.clientX - svgRect.left - offsetX;
    const mouseY = e.clientY - svgRect.top - offsetY;
    rotateStartAngle =
      Math.atan2(mouseY - rotateCenterY, mouseX - rotateCenterX) *
      (180 / Math.PI);
  }

  window.addEventListener("mousemove", onRotateMove);
  window.addEventListener("mouseup", onRotateEnd);
}

function onRotateMove(e: MouseEvent) {
  if (!isRotating || !rotateNodeId) return;

  const svgEl = document.querySelector(".selection-overlay")?.closest("svg");
  if (!svgEl) return;
  const svgRect = svgEl.getBoundingClientRect();
  const gEl = svgEl.querySelector("g.page-transform");
  const gRect = gEl?.getBoundingClientRect();
  const offsetX = gRect ? gRect.left - svgRect.left : 0;
  const offsetY = gRect ? gRect.top - svgRect.top : 0;
  const mouseX = e.clientX - svgRect.left - offsetX;
  const mouseY = e.clientY - svgRect.top - offsetY;

  const currentAngle =
    Math.atan2(mouseY - rotateCenterY, mouseX - rotateCenterX) *
    (180 / Math.PI);
  let delta = currentAngle - rotateStartAngle;
  let newAngle = rotateOrigAngle + delta;

  // Snap to 15° increments when shift is held
  if (e.shiftKey) {
    newAngle = Math.round(newAngle / 15) * 15;
  }

  // Normalize to 0-360
  newAngle = ((newAngle % 360) + 360) % 360;

  store.updateSelectedProps({ rotation: Math.round(newAngle * 10) / 10 });
}

function onRotateEnd() {
  store.commitLiveMove();
  isRotating = false;
  rotateNodeId = null;
  window.removeEventListener("mousemove", onRotateMove);
  window.removeEventListener("mouseup", onRotateEnd);
}

// ── Rubber Band Selection ────────────────────────────────────

export function startRubberBand(
  e: MouseEvent,
  canvasX: number,
  canvasY: number,
) {
  isRubberBanding = true;
  rubberBandStartX = canvasX;
  rubberBandStartY = canvasY;
  rubberBandEndX = canvasX;
  rubberBandEndY = canvasY;
  store.clearSelection();

  window.addEventListener("mousemove", onRubberBandMove);
  window.addEventListener("mouseup", onRubberBandEnd);
}

function onRubberBandMove(e: MouseEvent) {
  if (!isRubberBanding) return;
  const svgEl = document.querySelector(".selection-overlay")?.closest("svg");
  if (!svgEl) return;
  const svgRect = svgEl.getBoundingClientRect();
  const gEl = svgEl.querySelector("g.page-transform");
  const gRect = gEl?.getBoundingClientRect();
  const offsetX = gRect ? gRect.left - svgRect.left : 0;
  const offsetY = gRect ? gRect.top - svgRect.top : 0;
  rubberBandEndX = e.clientX - svgRect.left - offsetX;
  rubberBandEndY = e.clientY - svgRect.top - offsetY;
}

function onRubberBandEnd() {
  if (isRubberBanding && rubberBandRect) {
    // Find all nodes within the rubber band
    const rb = rubberBandRect;
    const page = store.activePage;
    if (page && rb.w > 4 && rb.h > 4) {
      const matchIds: string[] = [];
      for (const child of page.children) {
        if (!("position" in child) || !(child as any).position) continue;
        const pos = (child as any).position;
        const nx = mmToPx(pos.x ?? 0, vp.dpi, vp.zoom);
        const ny = mmToPx(pos.y ?? 0, vp.dpi, vp.zoom);
        const nw =
          typeof pos.width === "number"
            ? mmToPx(pos.width, vp.dpi, vp.zoom)
            : 50;
        const nh =
          typeof pos.height === "number"
            ? mmToPx(pos.height, vp.dpi, vp.zoom)
            : 20;

        // Check if element intersects with rubber band
        if (
          nx < rb.x + rb.w &&
          nx + nw > rb.x &&
          ny < rb.y + rb.h &&
          ny + nh > rb.y
        ) {
          matchIds.push(child.id);
        }
      }
      if (matchIds.length > 0) {
        for (const id of matchIds) {
          store.select(id, true); // additive selection
        }
      }
    }
  }

  isRubberBanding = false;
  window.removeEventListener("mousemove", onRubberBandMove);
  window.removeEventListener("mouseup", onRubberBandEnd);
}

// ── Helpers ──────────────────────────────────────────────────

function collectOtherNodeBounds(excludeIds: Set<string>): BoundingBox[] {
  const page = store.activePage;
  if (!page) return [];
  const bounds: BoundingBox[] = [];
  const pageSetup = store.document.meta.pageSetup;
  const pageWidth = getPageW(pageSetup);
  const pageHeight = getPageH(pageSetup);
  for (const child of page.children) {
    if (excludeIds.has(child.id)) continue;
    const pos =
      ("props" in child
        ? (
            child.props as {
              position?: {
                x?: number;
                y?: number;
                width?: number | string;
                height?: number | string;
              };
            }
          ).position
        : undefined) ??
      (
        child as {
          position?: {
            x?: number;
            y?: number;
            width?: number | string;
            height?: number | string;
          };
        }
      ).position;
    if (!pos) continue;

    const width =
      typeof pos.width === "string" && pos.width.endsWith("%")
        ? (Number.parseFloat(pos.width) / 100) * pageWidth
        : Number(pos.width) || 50;
    const height =
      typeof pos.height === "string" && pos.height.endsWith("%")
        ? (Number.parseFloat(pos.height) / 100) * pageHeight
        : Number(pos.height) || 20;
    bounds.push({ x: pos.x ?? 0, y: pos.y ?? 0, width, height });
  }
  return bounds;
}

function getPageW(setup: any): number {
  const sizes: Record<string, number> = {
    A4: 210,
    A3: 297,
    A5: 148,
    Letter: 216,
    Legal: 216,
    ID_CARD: 85.6,
    BADGE: 86,
  };
  const w =
    setup.size === "Custom" ? (setup.width ?? 210) : (sizes[setup.size] ?? 210);
  const h =
    setup.size === "Custom"
      ? (setup.height ?? 297)
      : ((
          {
            A4: 297,
            A3: 420,
            A5: 210,
            Letter: 279,
            Legal: 356,
            ID_CARD: 53.98,
            BADGE: 54,
          } as any
        )[setup.size] ?? 297);
  return setup.orientation === "landscape" ? h : w;
}

function getPageH(setup: any): number {
  const w = getPageW({ ...setup, orientation: "portrait" });
  const sizes: Record<string, number> = {
    A4: 297,
    A3: 420,
    A5: 210,
    Letter: 279,
    Legal: 356,
    ID_CARD: 53.98,
    BADGE: 54,
  };
  const h =
    setup.size === "Custom"
      ? (setup.height ?? 297)
      : (sizes[setup.size] ?? 297);
  return setup.orientation === "landscape" ? w : h;
}

// Handle positions for a rect
function getHandles(r: { x: number; y: number; w: number; h: number }) {
  const hs = HANDLE_SIZE / 2;
  return [
    { id: "top-left", cx: r.x - hs, cy: r.y - hs, cursor: "nwse-resize" },
    {
      id: "top-center",
      cx: r.x + r.w / 2 - hs,
      cy: r.y - hs,
      cursor: "ns-resize",
    },
    {
      id: "top-right",
      cx: r.x + r.w - hs,
      cy: r.y - hs,
      cursor: "nesw-resize",
    },
    {
      id: "middle-left",
      cx: r.x - hs,
      cy: r.y + r.h / 2 - hs,
      cursor: "ew-resize",
    },
    {
      id: "middle-right",
      cx: r.x + r.w - hs,
      cy: r.y + r.h / 2 - hs,
      cursor: "ew-resize",
    },
    {
      id: "bottom-left",
      cx: r.x - hs,
      cy: r.y + r.h - hs,
      cursor: "nesw-resize",
    },
    {
      id: "bottom-center",
      cx: r.x + r.w / 2 - hs,
      cy: r.y + r.h - hs,
      cursor: "ns-resize",
    },
    {
      id: "bottom-right",
      cx: r.x + r.w - hs,
      cy: r.y + r.h - hs,
      cursor: "nwse-resize",
    },
  ];
}
</script>

<g class="selection-overlay" pointer-events="all">
  {#each selectionRects as rect (rect.id)}
    <!-- Selection outline (dashed border) -->
    <rect
      x={rect.x}
      y={rect.y}
      width={rect.w}
      height={rect.h}
      fill="none"
      stroke="#3b82f6"
      stroke-width="1"
      stroke-dasharray="4,3"
      pointer-events="none"
    />

    <!-- Move area (the entire selected element) -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    {#if store.editingNodeId !== rect.id}
      <rect
        x={rect.x}
        y={rect.y}
        width={rect.w}
        height={rect.h}
        fill="transparent"
        class="cursor-move"
        onmousedown={(e) => startDrag(e, rect.id, "move")}
        ondblclick={(e) => {
          const node = findNode(store.document, rect.id);
          if (node?.type === "Text" && store.editingNodeId !== rect.id) {
            e.preventDefault();
            store.editingNodeId = rect.id;
          }
        }}
      />
    {/if}

    <!-- Dimension label (above rotation handle) -->
    <text
      x={rect.x + rect.w / 2}
      y={rect.y - ROTATION_OFFSET - 6}
      text-anchor="middle"
      fill="#3b82f6"
      font-size="9"
      font-family="monospace"
      pointer-events="none"
    >
      {(pxToMm(rect.w, vp.dpi, vp.zoom)).toFixed(1)} × {(pxToMm(rect.h, vp.dpi, vp.zoom)).toFixed(1)}
    </text>

    <!-- Rotation handle -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <g class="rotation-handle-group" pointer-events="all">
      <!-- Connection line -->
      <line
        x1={rect.x + rect.w / 2}
        y1={rect.y}
        x2={rect.x + rect.w / 2}
        y2={rect.y - ROTATION_OFFSET}
        stroke="#3b82f6"
        stroke-width="1"
        stroke-dasharray="2,2"
        pointer-events="none"
      />
      <!-- Rotation circle -->
      <circle
        cx={rect.x + rect.w / 2}
        cy={rect.y - ROTATION_OFFSET}
        r="5"
        fill="white"
        stroke="#3b82f6"
        stroke-width="1.5"
        style="cursor: url('data:image/svg+xml;utf8,<svg xmlns=%22http://www.w3.org/2000/svg%22 width=%2224%22 height=%2224%22 viewBox=%220 0 24 24%22><path d=%22M21 12a9 9 0 1 1-9-9%22 fill=%22none%22 stroke=%22%233b82f6%22 stroke-width=%222%22/><path d=%22M12 3l3 3-3 3%22 fill=%22none%22 stroke=%22%233b82f6%22 stroke-width=%222%22/></svg>') 12 12, crosshair;"
        onmousedown={(e) => startRotation(e, rect.id, rect)}
      />
      {#if rect.rotation}
        <text
          x={rect.x + rect.w / 2 + 12}
          y={rect.y - ROTATION_OFFSET + 3}
          fill="#3b82f6"
          font-size="8"
          font-family="monospace"
          pointer-events="none"
        >
          {rect.rotation}°
        </text>
      {/if}
    </g>

    <!-- Resize handles -->
    {#each getHandles(rect) as handle (handle.id)}
      <!-- svelte-ignore a11y_no_static_element_interactions -->
      <rect
        x={handle.cx}
        y={handle.cy}
        width={HANDLE_SIZE}
        height={HANDLE_SIZE}
        rx="1.5"
        fill="white"
        stroke="#3b82f6"
        stroke-width="1.5"
        class="cursor-{handle.cursor}"
        onmousedown={(e) => startDrag(e, rect.id, handle.id)}
      />
    {/each}
  {/each}

  <!-- Rubber band selection rectangle -->
  {#if isRubberBanding && rubberBandRect}
    <rect
      x={rubberBandRect.x}
      y={rubberBandRect.y}
      width={rubberBandRect.w}
      height={rubberBandRect.h}
      fill="rgba(59, 130, 246, 0.08)"
      stroke="#3b82f6"
      stroke-width="1"
      stroke-dasharray="4,2"
      pointer-events="none"
    />
  {/if}

  <!-- Snap lines -->
  {#if isDragging && activeSnapLines.length > 0}
    <SnapLinesOverlay snapLines={activeSnapLines} viewport={vp} />
  {/if}
</g>
