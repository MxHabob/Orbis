<script lang="ts">
/**
 * SelectionOverlay — Renders selection handles and resize grips
 * for selected elements on the canvas.
 */
import type { EditorStore } from "../stores/editor-store.svelte";
import { mmToPx, pxToMm } from "@docforge/core";
import { findNode, getPages } from "@docforge/core";
import type { SnapLine, BoundingBox } from "@docforge/core";
import { calcSmartSnap } from "@docforge/core";
import SnapLinesOverlay from "./SnapLinesOverlay.svelte";

let {
  store,
  pageSizePx,
}: {
  store: EditorStore;
  pageSizePx: { width: number; height: number };
} = $props();

const vp = $derived(store.viewport);
const HANDLE_SIZE = 8; // px (screen space, not zoomed)

// Compute selection bounds for each selected node
const selectionRects = $derived(
  store.selectedIds.map((id) => {
    const node = findNode(store.document, id);
    if (!node || !("position" in node) || !(node as any).position) return null;
    const pos = (node as any).position;
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

    return { id, x, y, w, h };
  }).filter(Boolean) as { id: string; x: number; y: number; w: number; h: number }[],
);

// Drag state
let isDragging = $state(false);
let dragHandle = $state<string | null>(null);
let dragNodeId = $state<string | null>(null);
let dragStartX = 0;
let dragStartY = 0;
let dragOrigX = 0;
let dragOrigY = 0;
let dragOrigW = 0;
let dragOrigH = 0;
let activeSnapLines = $state<SnapLine[]>([]);

function startDrag(e: MouseEvent, nodeId: string, handle: string) {
  e.stopPropagation();
  e.preventDefault();
  isDragging = true;
  dragHandle = handle;
  dragNodeId = nodeId;
  dragStartX = e.clientX;
  dragStartY = e.clientY;

  const rect = selectionRects.find((r) => r.id === nodeId);
  if (rect) {
    dragOrigX = rect.x;
    dragOrigY = rect.y;
    dragOrigW = rect.w;
    dragOrigH = rect.h;
  }

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

    // Smart snap: collect other elements on the active page
    if (store.snapEnabled) {
      const otherBounds = collectOtherNodeBounds(dragNodeId!);
      const pageSetup = store.document.meta.pageSetup;
      const pageBounds: BoundingBox = {
        x: pageSetup.margins.left,
        y: pageSetup.margins.top,
        width: getPageW(pageSetup) - pageSetup.margins.left - pageSetup.margins.right,
        height: getPageH(pageSetup) - pageSetup.margins.top - pageSetup.margins.bottom,
      };

      const snapResult = calcSmartSnap(
        { x: rawX, y: rawY, width: rawW, height: rawH },
        otherBounds,
        pageBounds,
        3,
      );
      store.moveSelected(snapResult.x, snapResult.y);
      activeSnapLines = snapResult.snapLines;
    } else {
      store.moveSelected(
        Math.round(rawX * 10) / 10,
        Math.round(rawY * 10) / 10,
      );
      activeSnapLines = [];
    }
  } else if (dragHandle) {
    // Resize
    let newW = dragOrigW;
    let newH = dragOrigH;
    let newX = dragOrigX;
    let newY = dragOrigY;

    if (dragHandle.includes("right")) newW = dragOrigW + dx;
    if (dragHandle.includes("left")) { newW = dragOrigW - dx; newX = dragOrigX + dx; }
    if (dragHandle.includes("bottom")) newH = dragOrigH + dy;
    if (dragHandle.includes("top")) { newH = dragOrigH - dy; newY = dragOrigY + dy; }

    const wMm = Math.max(5, Math.round(pxToMm(newW, vp.dpi, vp.zoom) * 10) / 10);
    const hMm = Math.max(5, Math.round(pxToMm(newH, vp.dpi, vp.zoom) * 10) / 10);

    store.resizeSelected(wMm, hMm);
    if (dragHandle.includes("left") || dragHandle.includes("top")) {
      store.moveSelected(
        Math.round(pxToMm(newX, vp.dpi, vp.zoom) * 10) / 10,
        Math.round(pxToMm(newY, vp.dpi, vp.zoom) * 10) / 10,
      );
    }
  }
}

function onDragEnd() {
  isDragging = false;
  dragHandle = null;
  dragNodeId = null;
  activeSnapLines = [];
  window.removeEventListener("mousemove", onDragMove);
  window.removeEventListener("mouseup", onDragEnd);
}

// Collect bounding boxes of all sibling elements (excluding the dragged one)
function collectOtherNodeBounds(excludeId: string): BoundingBox[] {
  const page = store.activePage;
  if (!page) return [];
  const bounds: BoundingBox[] = [];
  for (const child of page.children) {
    if (child.id === excludeId) continue;
    if ("position" in child && (child as any).position) {
      const pos = (child as any).position;
      const w = typeof pos.width === "number" ? pos.width : 50;
      const h = typeof pos.height === "number" ? pos.height : 20;
      bounds.push({ x: pos.x ?? 0, y: pos.y ?? 0, width: w, height: h });
    }
  }
  return bounds;
}

function getPageW(setup: any): number {
  const sizes: Record<string, number> = {
    A4: 210, A3: 297, A5: 148, Letter: 216, Legal: 216, ID_CARD: 85.6, BADGE: 86,
  };
  const w = setup.size === "Custom" ? (setup.width ?? 210) : (sizes[setup.size] ?? 210);
  const h = setup.size === "Custom" ? (setup.height ?? 297) : ({ A4: 297, A3: 420, A5: 210, Letter: 279, Legal: 356, ID_CARD: 53.98, BADGE: 54 } as any)[setup.size] ?? 297;
  return setup.orientation === "landscape" ? h : w;
}

function getPageH(setup: any): number {
  const w = getPageW({ ...setup, orientation: "portrait" });
  const sizes: Record<string, number> = { A4: 297, A3: 420, A5: 210, Letter: 279, Legal: 356, ID_CARD: 53.98, BADGE: 54 };
  const h = setup.size === "Custom" ? (setup.height ?? 297) : (sizes[setup.size] ?? 297);
  return setup.orientation === "landscape" ? w : h;
}

// Handle positions for a rect
function getHandles(r: { x: number; y: number; w: number; h: number }) {
  const hs = HANDLE_SIZE / 2;
  return [
    { id: "top-left", cx: r.x - hs, cy: r.y - hs, cursor: "nwse-resize" },
    { id: "top-center", cx: r.x + r.w / 2 - hs, cy: r.y - hs, cursor: "ns-resize" },
    { id: "top-right", cx: r.x + r.w - hs, cy: r.y - hs, cursor: "nesw-resize" },
    { id: "middle-left", cx: r.x - hs, cy: r.y + r.h / 2 - hs, cursor: "ew-resize" },
    { id: "middle-right", cx: r.x + r.w - hs, cy: r.y + r.h / 2 - hs, cursor: "ew-resize" },
    { id: "bottom-left", cx: r.x - hs, cy: r.y + r.h - hs, cursor: "nesw-resize" },
    { id: "bottom-center", cx: r.x + r.w / 2 - hs, cy: r.y + r.h - hs, cursor: "ns-resize" },
    { id: "bottom-right", cx: r.x + r.w - hs, cy: r.y + r.h - hs, cursor: "nwse-resize" },
  ];
}
</script>

<g class="selection-overlay" pointer-events="all">
  {#each selectionRects as rect (rect.id)}
    <!-- Move area (the entire selected element) -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <rect
      x={rect.x}
      y={rect.y}
      width={rect.w}
      height={rect.h}
      fill="transparent"
      class="cursor-move"
      onmousedown={(e) => startDrag(e, rect.id, "move")}
    />

    <!-- Dimension label -->
    <text
      x={rect.x + rect.w / 2}
      y={rect.y - 8}
      text-anchor="middle"
      fill="#3b82f6"
      font-size="9"
      font-family="monospace"
      pointer-events="none"
    >
      {(pxToMm(rect.w, vp.dpi, vp.zoom)).toFixed(1)} × {(pxToMm(rect.h, vp.dpi, vp.zoom)).toFixed(1)}
    </text>

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

  <!-- Snap lines -->
  {#if isDragging && activeSnapLines.length > 0}
    <SnapLinesOverlay snapLines={activeSnapLines} viewport={vp} />
  {/if}
</g>
