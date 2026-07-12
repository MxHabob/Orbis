<script lang="ts">
/**
 * CanvasRoot — The main SVG viewport for the template designer.
 * Renders the document page at real mm dimensions with zoom/pan support.
 * This is the WYSIWYG design surface.
 */
import { onMount } from "svelte";
import type { EditorStore } from "../stores/editor-store.svelte";
import { mmToPx, pxToMm, screenToDoc } from "@docforge/core/canvas";
import CanvasPage from "./canvas-page.svelte";
import SelectionOverlay from "./selection-overlay.svelte";
import GridOverlay from "./grid-overlay.svelte";
import RulerBar from "./ruler-bar.svelte";

// ── Props ────────────────────────────────────────────────────

let {
  store,
}: {
  store: EditorStore;
} = $props();

// ── Local State ──────────────────────────────────────────────

let containerEl: HTMLDivElement | undefined = $state();
let svgEl: SVGSVGElement | undefined = $state();
let containerWidth = $state(800);
let containerHeight = $state(600);
let isPanning = $state(false);
let panStartX = 0;
let panStartY = 0;
let panStartPanX = 0;
let panStartPanY = 0;
let selectionOverlay: SelectionOverlay | undefined = $state();

// ── Derived ──────────────────────────────────────────────────

const vp = $derived(store.viewport);
const pageSetup = $derived(store.document.meta.pageSetup);

// Page dimensions in mm
const pageWidthMm = $derived(getPageDimensionMm("width"));
const pageHeightMm = $derived(getPageDimensionMm("height"));

// Page dimensions in px at current zoom
const pageWidthPx = $derived(mmToPx(pageWidthMm, vp.dpi, vp.zoom));
const pageHeightPx = $derived(mmToPx(pageHeightMm, vp.dpi, vp.zoom));

// Canvas center offset for page centering
const offsetX = $derived(
  Math.max(0, (containerWidth - pageWidthPx) / 2) + vp.panX,
);
const offsetY = $derived(
  Math.max(0, (containerHeight - pageHeightPx) / 2) + vp.panY,
);

// ── Lifeperiod ────────────────────────────────────────────────

onMount(() => {
  if (containerEl) {
    const obs = new ResizeObserver((entries) => {
      for (const entry of entries) {
        containerWidth = entry.contentRect.width;
        containerHeight = entry.contentRect.height;
      }
    });
    obs.observe(containerEl);

    // Initial zoom to fit
    setTimeout(() => {
      store.zoomToFit(containerWidth, containerHeight);
    }, 50);

    return () => obs.disconnect();
  }
});

// Attach wheel listener imperatively with {passive: false}
// so that preventDefault() works (Svelte's onwheel is passive by default)
$effect(() => {
  const svg = svgEl;
  if (!svg) return;
  svg.addEventListener("wheel", handleWheel, { passive: false });
  return () => svg.removeEventListener("wheel", handleWheel);
});

// ── Mouse Position ───────────────────────────────────────────

let cursorDocMm = $state({ x: 0, y: 0 });

function handleMouseMove(e: MouseEvent) {
  if (!svgEl) return;
  const rect = svgEl.getBoundingClientRect();
  const screenX = e.clientX - rect.left - offsetX;
  const screenY = e.clientY - rect.top - offsetY;
  cursorDocMm = screenToDoc(screenX, screenY, { ...vp, panX: 0, panY: 0 });

  if (isPanning) {
    const dx = e.clientX - panStartX;
    const dy = e.clientY - panStartY;
    store.setPan(panStartPanX + dx, panStartPanY + dy);
  }
}

// ── Zoom (Ctrl+Wheel) ────────────────────────────────────────

function handleWheel(e: WheelEvent) {
  if (e.ctrlKey || e.metaKey) {
    e.preventDefault();
    const zoomFactor = e.deltaY > 0 ? 0.95 : 1.05;
    const newZoom = vp.zoom * zoomFactor;
    store.setZoom(newZoom);
  }
}

// ── Pan (Middle mouse or Space+drag) ─────────────────────────

function handleMouseDown(e: MouseEvent) {
  // Middle mouse button or space+click for pan
  if (e.button === 1 || (e.button === 0 && store.activeTool === "pan")) {
    e.preventDefault();
    isPanning = true;
    panStartX = e.clientX;
    panStartY = e.clientY;
    panStartPanX = vp.panX;
    panStartPanY = vp.panY;
    return;
  }

  // Left click on empty canvas area → rubber band selection or clear
  if (
    e.button === 0 &&
    (e.target === svgEl ||
      (e.target as Element)?.classList?.contains("canvas-bg"))
  ) {
    // Start rubber band selection
    if (selectionOverlay && svgEl) {
      const rect = svgEl.getBoundingClientRect();
      const canvasX = e.clientX - rect.left - offsetX;
      const canvasY = e.clientY - rect.top - offsetY;
      selectionOverlay.startRubberBand(e, canvasX, canvasY);
    } else {
      store.clearSelection();
    }
  }
}

function handleMouseUp() {
  isPanning = false;
}

// ── Helpers ──────────────────────────────────────────────────

function getPageDimensionMm(dim: "width" | "height"): number {
  const sizes: Record<string, { w: number; h: number }> = {
    A4: { w: 210, h: 297 },
    A3: { w: 297, h: 420 },
    A5: { w: 148, h: 210 },
    Letter: { w: 216, h: 279 },
    Legal: { w: 216, h: 356 },
    ID_CARD: { w: 85.6, h: 53.98 },
    BADGE: { w: 86, h: 54 },
  };
  const size = sizes[pageSetup.size] ?? {
    w: pageSetup.width ?? 210,
    h: pageSetup.height ?? 297,
  };
  const isLandscape = pageSetup.orientation === "landscape";
  if (dim === "width") return isLandscape ? size.h : size.w;
  return isLandscape ? size.w : size.h;
}

// ── Drag & Drop ──────────────────────────────────────────────

let isDragOver = $state(false);

function handleDragOver(e: DragEvent) {
  e.preventDefault();
  if (e.dataTransfer) e.dataTransfer.dropEffect = "copy";
  isDragOver = true;
}

function handleDragLeave() {
  isDragOver = false;
}

function handleDrop(e: DragEvent) {
  e.preventDefault();
  isDragOver = false;
  if (!svgEl || !e.dataTransfer) return;

  // Calculate drop position in document mm
  const rect = svgEl.getBoundingClientRect();
  const screenX = e.clientX - rect.left - offsetX;
  const screenY = e.clientY - rect.top - offsetY;
  const docPos = screenToDoc(screenX, screenY, { ...vp, panX: 0, panY: 0 });

  // Check for element type drop (from toolbar)
  const elementType = e.dataTransfer.getData("application/x-docforge-element");
  if (elementType) {
    store.addElement(elementType, undefined, docPos.x, docPos.y);
    return;
  }

  // Check for variable drop
  const varName = e.dataTransfer.getData("application/x-docforge-variable");
  if (varName) {
    const selectedNode = store.selectedNode;
    if (selectedNode && selectedNode.type === "Text") {
      const currentContent = (selectedNode as any).props?.content || "";
      store.updateSelectedProps({
        props: {
          ...(selectedNode as any).props,
          content: currentContent + `{{${varName}}}`,
        },
      } as any);
    } else {
      // Create a new text node with the variable
      store.addElement(
        "Text",
        {
          content: `{{${varName}}}`,
          fontSize: 12,
          fontFamily: "Rubik",
          direction: "rtl",
        },
        docPos.x,
        docPos.y,
      );
    }
    return;
  }

  // Check for asset drop
  const assetRef = e.dataTransfer.getData("application/x-docforge-asset");
  if (assetRef) {
    const asset = store.document.assets[assetRef];
    store.addElement(
      "Image",
      {
        assetRef,
        src: asset?.url || "",
        width: 40,
        height: 30,
        fit: "contain",
      },
      docPos.x,
      docPos.y,
    );
    return;
  }
}
</script>

<div
  bind:this={containerEl}
  class="relative flex-grow overflow-hidden bg-muted/20 select-none"
  role="application"
  aria-label="Template Canvas"
>
  <!-- Rulers -->
  {#if store.rulersVisible}
    <RulerBar
      axis="horizontal"
      pageSizeMm={pageWidthMm}
      viewport={vp}
      {offsetX}
      {offsetY}
      cursorMm={cursorDocMm.x}
    />
    <RulerBar
      axis="vertical"
      pageSizeMm={pageHeightMm}
      viewport={vp}
      {offsetX}
      {offsetY}
      cursorMm={cursorDocMm.y}
    />
  {/if}

  <!-- SVG Canvas -->
  <svg
    bind:this={svgEl}
    width={containerWidth}
    height={containerHeight}
    class="absolute inset-0 {isPanning ? 'cursor-grabbing' : ''}"
    style="top: {store.rulersVisible ? '24px' : '0'}; left: {store.rulersVisible ? '24px' : '0'};"
    onmousemove={handleMouseMove}
    onmousedown={handleMouseDown}
    onmouseup={handleMouseUp}
    onmouseleave={handleMouseUp}
    ondragover={handleDragOver}
    ondragleave={handleDragLeave}
    ondrop={handleDrop}
  >
    <!-- Background pattern -->
    <defs>
      <pattern id="canvas-dots" width="20" height="20" patternUnits="userSpaceOnUse">
        <circle cx="10" cy="10" r="0.5" fill="var(--color-muted-foreground)" opacity="0.15" />
      </pattern>
    </defs>
    <rect width="100%" height="100%" fill="url(#canvas-dots)" class="canvas-bg" />

    <!-- Page Group (translated for centering + pan) -->
    <g transform="translate({offsetX}, {offsetY})" class="page-transform">
      <!-- Page shadow -->
      <rect
        x="3"
        y="3"
        width={pageWidthPx}
        height={pageHeightPx}
        rx="2"
        fill="black"
        opacity="0.08"
      />

      <!-- Page background -->
      <rect
        width={pageWidthPx}
        height={pageHeightPx}
        rx="1"
        fill="white"
        stroke="var(--color-border)"
        stroke-width="1"
      />

      <!-- Grid overlay -->
      {#if store.gridVisible}
        <GridOverlay
          widthPx={pageWidthPx}
          heightPx={pageHeightPx}
          gridSizeMm={store.gridSize}
          viewport={vp}
        />
      {/if}

      <!-- Page content (nodes) -->
      {#if store.activePage}
        <CanvasPage
          page={store.activePage}
          {store}
          pageSizePx={{ width: pageWidthPx, height: pageHeightPx }}
        />
      {/if}

      <!-- Selection overlay (handles, guides) -->
      <SelectionOverlay bind:this={selectionOverlay} {store} pageSizePx={{ width: pageWidthPx, height: pageHeightPx }} />

      <!-- Drag-over indicator -->
      {#if isDragOver}
        <rect
          x="0" y="0"
          width={pageWidthPx}
          height={pageHeightPx}
          fill="none"
          stroke="#3b82f6"
          stroke-width="3"
          stroke-dasharray="8,4"
          rx="4"
          pointer-events="none"
          opacity="0.5"
        />
      {/if}
    </g>
  </svg>
</div>

