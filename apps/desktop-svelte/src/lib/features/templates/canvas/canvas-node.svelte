<script lang="ts">
/**
 * CanvasNode — Recursive node renderer for the SVG canvas.
 * Renders each node type (Text, Image, Shape, Frame, Barcode, QR, etc.)
 * as SVG elements at their exact mm positions.
 */
import type { TemplateNode } from "@docforge/core";
import { mmToPx } from "@docforge/core/canvas";
import type { EditorStore } from "../stores/editor-store.svelte";

let {
  node,
  store,
  pageSizePx,
}: {
  node: TemplateNode;
  store: EditorStore;
  pageSizePx: { width: number; height: number };
} = $props();

const vp = $derived(store.viewport);
const isSelected = $derived(store.selectedIds.includes(node.id));
const isVisible = $derived(node.visible !== false);
const opacity = $derived(node.opacity ?? 1);

// Position calculation (mm → px)
const pos = $derived(getNodePosition());
const x = $derived(pos.x);
const y = $derived(pos.y);
const w = $derived(pos.w);
const h = $derived(pos.h);

function getNodePosition() {
  const position = "position" in node ? (node as any).position : null;
  if (!position) {
    return { x: 0, y: 0, w: pageSizePx.width, h: 20 };
  }

  const px = mmToPx(position.x ?? 0, vp.dpi, vp.zoom);
  const py = mmToPx(position.y ?? 0, vp.dpi, vp.zoom);

  let pw: number;
  if (typeof position.width === "string" && position.width.endsWith("%")) {
    pw = (parseFloat(position.width) / 100) * pageSizePx.width;
  } else {
    pw = mmToPx(Number(position.width) || 50, vp.dpi, vp.zoom);
  }

  let ph: number;
  if (position.height == null) {
    ph = mmToPx(20, vp.dpi, vp.zoom); // auto-height default
  } else if (
    typeof position.height === "string" &&
    position.height.endsWith("%")
  ) {
    ph = (parseFloat(position.height) / 100) * pageSizePx.height;
  } else {
    ph = mmToPx(Number(position.height) || 20, vp.dpi, vp.zoom);
  }

  return { x: px, y: py, w: pw, h: ph };
}

// Click handler → select this node
function handleClick(e: MouseEvent) {
  e.stopPropagation();
  store.select(node.id, e.ctrlKey || e.metaKey);
}

// ── Inline Text Editing ──────────────────────────────────────
let isEditing = $state(false);
let editEl: HTMLDivElement | undefined = $state();

function handleDoubleClick(e: MouseEvent) {
  if (node.type !== "Text") return;
  e.stopPropagation();
  e.preventDefault();
  isEditing = true;
  // Focus on next tick
  requestAnimationFrame(() => {
    if (editEl) {
      editEl.focus();
      // Select all text
      const range = document.createRange();
      range.selectNodeContents(editEl);
      const sel = window.getSelection();
      sel?.removeAllRanges();
      sel?.addRange(range);
    }
  });
}

function commitEdit() {
  if (!isEditing || !editEl) return;
  const newContent = editEl.innerText || "";
  if (newContent !== props.content) {
    store.updateSelectedProps({
      props: { ...props, content: newContent },
    } as any);
  }
  isEditing = false;
}

function handleEditKeydown(e: KeyboardEvent) {
  if (e.key === "Escape") {
    e.preventDefault();
    isEditing = false; // Discard changes
  }
  // Stop propagation to prevent canvas shortcuts
  e.stopPropagation();
}

// Props shorthand
const props = $derived("props" in node ? (node as any).props || {} : {});

// Parse text content into segments: literal text and variable expressions
type ContentSegment =
  | { type: "text"; value: string }
  | { type: "variable"; key: string };
function parseContentSegments(content: string): ContentSegment[] {
  const segments: ContentSegment[] = [];
  const regex = /\{\{([^}]+)\}\}/g;
  let lastIndex = 0;
  let match;
  while ((match = regex.exec(content)) !== null) {
    if (match.index > lastIndex) {
      segments.push({
        type: "text",
        value: content.slice(lastIndex, match.index),
      });
    }
    segments.push({ type: "variable", key: match[1]!.trim() });
    lastIndex = regex.lastIndex;
  }
  if (lastIndex < content.length) {
    segments.push({ type: "text", value: content.slice(lastIndex) });
  }
  return segments;
}

const textSegments = $derived(
  node.type === "Text" && props.content
    ? parseContentSegments(props.content)
    : [],
);
</script>

{#if isVisible}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <g
    data-node-id={node.id}
    data-node-type={node.type}
    opacity={opacity}
    transform="translate({x}, {y}){node.rotation ? ` rotate(${node.rotation} ${w/2} ${h/2})` : ''}"
    onclick={handleClick}
    ondblclick={handleDoubleClick}
    class="cursor-pointer"
    style="outline: none;"
  >
    {#if node.type === "Text"}
      <!-- Text Node -->
      <foreignObject width={w} height={h} class="overflow-visible">
        {#if isEditing}
          <!-- Inline edit mode -->
          <!-- svelte-ignore a11y_no_static_element_interactions -->
          <div
            xmlns="http://www.w3.org/1999/xhtml"
            bind:this={editEl}
            contenteditable="true"
            onblur={commitEdit}
            onkeydown={handleEditKeydown}
            style="
              width: {w}px;
              min-height: {h}px;
              font-size: {mmToPx((props.fontSize ?? 12) * 0.352778, vp.dpi, vp.zoom)}px;
              font-family: {props.fontFamily || 'Rubik'}, sans-serif;
              font-weight: {props.fontWeight || '400'};
              font-style: {props.fontStyle || 'normal'};
              text-decoration: {props.textDecoration || 'none'};
              color: {props.color || '#000000'};
              text-align: {props.textAlign || 'start'};
              direction: {props.direction || 'rtl'};
              line-height: {props.lineHeight || 1.4};
              overflow: visible;
              word-wrap: break-word;
              padding: 2px;
              margin: 0;
              outline: 2px solid #3b82f6;
              outline-offset: 1px;
              border-radius: 2px;
              background: rgba(255,255,255,0.95);
              cursor: text;
            "
          >{props.content || ""}</div>
        {:else}
          <!-- Display mode -->
          <div
            xmlns="http://www.w3.org/1999/xhtml"
            style="
              width: {w}px;
              height: {h}px;
              font-size: {mmToPx((props.fontSize ?? 12) * 0.352778, vp.dpi, vp.zoom)}px;
              font-family: {props.fontFamily || 'Rubik'}, sans-serif;
              font-weight: {props.fontWeight || '400'};
              font-style: {props.fontStyle || 'normal'};
              text-decoration: {props.textDecoration || 'none'};
              color: {props.color || '#000000'};
              text-align: {props.textAlign || 'start'};
              direction: {props.direction || 'rtl'};
              line-height: {props.lineHeight || 1.4};
              display: flex;
              align-items: {props.verticalAlign === 'center' ? 'center' : props.verticalAlign === 'bottom' ? 'flex-end' : 'flex-start'};
              overflow: hidden;
              word-wrap: break-word;
              padding: 0;
              margin: 0;
            "
          >
            <span style="display: inline;">{#each textSegments as seg}{#if seg.type === "variable"}<span style="background: #dbeafe; color: #1d4ed8; border-radius: 3px; padding: 0 3px; font-size: 0.85em; font-weight: 600; white-space: nowrap;">{`{{${seg.key}}}`}</span>{:else}{seg.value}{/if}{/each}{#if textSegments.length === 0}{props.content || ""}{/if}</span>
          </div>
        {/if}
      </foreignObject>

    {:else if node.type === "Image"}
      <!-- Image Node -->
      {#if props.src}
        <image
          href={props.src}
          width={w}
          height={h}
          preserveAspectRatio={props.fit === "cover" ? "xMidYMid slice" : props.fit === "fill" ? "none" : "xMidYMid meet"}
        />
      {:else}
        <!-- Image placeholder -->
        <rect width={w} height={h} fill="#f1f5f9" stroke="#cbd5e1" stroke-width="1" stroke-dasharray="4,2" rx="2" />
        <text x={w/2} y={h/2} text-anchor="middle" dominant-baseline="central" fill="#94a3b8" font-size="10">📷</text>
      {/if}

    {:else if node.type === "Shape"}
      <!-- Shape Node -->
      {#if props.shape === "circle" || props.shape === "ellipse"}
        <ellipse
          cx={w / 2}
          cy={h / 2}
          rx={w / 2}
          ry={h / 2}
          fill={props.fillColor || "transparent"}
          stroke={props.strokeColor || "#000"}
          stroke-width={props.strokeWidth || 1}
        />
      {:else}
        <rect
          width={w}
          height={h}
          rx={mmToPx(props.borderRadius ?? 0, vp.dpi, vp.zoom)}
          fill={props.fillColor || "transparent"}
          stroke={props.strokeColor || "#000"}
          stroke-width={props.strokeWidth || 1}
        />
      {/if}

    {:else if node.type === "Separator"}
      <!-- Separator -->
      {#if props.direction === "vertical"}
        <line x1={w / 2} y1="0" x2={w / 2} y2={h} stroke={props.color || "#d1d5db"} stroke-width={props.thickness || 0.5} />
      {:else}
        <line x1="0" y1={h / 2} x2={w} y2={h / 2} stroke={props.color || "#d1d5db"} stroke-width={props.thickness || 0.5} />
      {/if}

    {:else if node.type === "Spacer"}
      <!-- Spacer (invisible but selectable) -->
      <rect width={w} height={h} fill="transparent" stroke={isSelected ? "#3b82f6" : "transparent"} stroke-width="1" stroke-dasharray="2,2" />

    {:else if node.type === "Barcode"}
      <!-- Barcode placeholder -->
      <rect width={w} height={h} fill="white" stroke="#e2e8f0" stroke-width="1" rx="2" />
      <g transform="translate(4, 2)">
        {#each Array(12) as _, i}
          <rect x={i * ((w - 8) / 12)} y="0" width={((w - 8) / 12) * 0.6} height={h - 10} fill="#000" />
        {/each}
      </g>
      <text x={w / 2} y={h - 2} text-anchor="middle" fill="#374151" font-size="7" font-family="monospace">
        {props.data || "BARCODE"}
      </text>

    {:else if node.type === "QRCode"}
      <!-- QR Code placeholder -->
      <rect width={w} height={h} fill="white" stroke="#e2e8f0" stroke-width="1" rx="2" />
      <rect x={w * 0.15} y={h * 0.15} width={w * 0.7} height={h * 0.7} fill="#1a1a2e" rx="2" />
      <rect x={w * 0.2} y={h * 0.2} width={w * 0.15} height={h * 0.15} fill="white" rx="1" />
      <rect x={w * 0.65} y={h * 0.2} width={w * 0.15} height={h * 0.15} fill="white" rx="1" />
      <rect x={w * 0.2} y={h * 0.65} width={w * 0.15} height={h * 0.15} fill="white" rx="1" />

    {:else if node.type === "Table"}
      <!-- Table -->
      <rect width={w} height={h} fill="white" stroke="#d1d5db" stroke-width="1" />
      {#if props.columns}
        {@const cols = props.columns as any[]}
        {@const colW = w / Math.max(cols.length, 1)}
        {@const headerH = Math.min(h * 0.3, mmToPx(8, vp.dpi, vp.zoom))}
        <!-- Header row -->
        <rect width={w} height={headerH} fill="#f1f5f9" stroke="#d1d5db" stroke-width="0.5" />
        {#each cols as col, ci}
          <text x={ci * colW + colW / 2} y={headerH / 2} text-anchor="middle" dominant-baseline="central" fill="#374151" font-size="8">
            {col.headerAr || col.header || col.key}
          </text>
          {#if ci > 0}
            <line x1={ci * colW} y1="0" x2={ci * colW} y2={h} stroke="#e5e7eb" stroke-width="0.5" />
          {/if}
        {/each}
      {/if}

    {:else if node.type === "Frame" || node.type === "Grid"}
      <!-- Container: render border then children -->
      {#if props.backgroundColor || props.borderColor}
        <rect
          width={w}
          height={h}
          rx={mmToPx(props.borderRadius ?? 0, vp.dpi, vp.zoom)}
          fill={props.backgroundColor || "transparent"}
          stroke={props.borderColor || "transparent"}
          stroke-width={props.borderWidth || 0}
        />
      {:else if isSelected}
        <rect width={w} height={h} fill="transparent" stroke="#3b82f6" stroke-width="1" stroke-dasharray="4,2" opacity="0.4" />
      {/if}

      <!-- Render children recursively -->
      {#if "children" in node && Array.isArray((node as any).children)}
        {#each (node as any).children as child (child.id)}
          <svelte:self node={child} {store} {pageSizePx} />
        {/each}
      {/if}

    {:else}
      <!-- Unknown node type fallback -->
      <rect width={w} height={h} fill="#fef3c7" stroke="#f59e0b" stroke-width="1" stroke-dasharray="3,2" rx="2" />
      <text x={w / 2} y={h / 2} text-anchor="middle" dominant-baseline="central" fill="#92400e" font-size="9">
        {node.type}
      </text>
    {/if}

    <!-- Selection highlight border -->
    {#if isSelected}
      <rect
        width={w}
        height={h}
        fill="none"
        stroke="#3b82f6"
        stroke-width="2"
        rx="1"
        pointer-events="none"
      />
    {/if}
  </g>
{/if}
