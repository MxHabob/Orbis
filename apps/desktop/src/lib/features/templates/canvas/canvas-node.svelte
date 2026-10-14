<script lang="ts">
import { Grid } from "@orbis/icons";
import CanvasNode from "./canvas-node.svelte";

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
  const position =
    "props" in node && node.props && "position" in node.props
      ? (node.props as any).position
      : (node as any).position;
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
  if (isEditing) return;
  store.select(node.id, e.ctrlKey || e.metaKey);
}

// ── Inline Text Editing ──────────────────────────────────────
const isEditing = $derived(store.editingNodeId === node.id);
const nodeProps = $derived("props" in node ? (node as any).props || {} : {});
let editEl: HTMLDivElement | undefined = $state();
let editedContent = $state<string | null>(null);
const editingContent = $derived(editedContent ?? nodeProps.content ?? "");

function focusEditable(element: HTMLDivElement) {
  element.focus();
  const range = document.createRange();
  range.selectNodeContents(element);
  const selection = window.getSelection();
  selection?.removeAllRanges();
  selection?.addRange(range);
}

function handleDoubleClick(e: MouseEvent) {
  if (node.type !== "Text") return;
  e.stopPropagation();
  if (isEditing) return;
  e.preventDefault();
  store.editingNodeId = node.id;
}

function handleEditInput(e: InputEvent) {
  editedContent = (e.currentTarget as HTMLDivElement).textContent || "";
}

function commitEdit() {
  if (!isEditing || !editEl) return;
  const newContent = editingContent;
  if (newContent !== nodeProps.content) {
    store.updateNodeProps(node.id, {
      props: { ...nodeProps, content: newContent },
    });
  }
  editedContent = null;
  store.editingNodeId = null;
}

function handleEditKeydown(e: KeyboardEvent) {
  if (e.key === "Escape") {
    e.preventDefault();
    editedContent = null;
    store.editingNodeId = null;
  }
  // Stop propagation to prevent canvas shortcuts
  e.stopPropagation();
}

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
  node.type === "Text" && nodeProps.content
    ? parseContentSegments(nodeProps.content)
    : [],
);

import { resolveDocumentUrl } from "$lib/features/documents/queries";

// ── Table Layout Derivation ──────────────────────────────────
const tableLayout = $derived.by(() => {
  if (node.type !== "Table") return null;
  const cols = (nodeProps.columns as any[]) || [];
  if (cols.length === 0) return null;

  const style = nodeProps.style || {};
  const isRtl = (nodeProps.direction || "rtl") === "rtl";
  const showHeader = nodeProps.showHeader !== false;
  const showTotals = nodeProps.showTotals === true || cols.some((c: any) => c.total);
  const showGrid = nodeProps.showGrid !== false;
  const showBorder = nodeProps.showBorder !== false;

  let rawWidths: number[] = [];
  let remainingW = w;
  let autoCount = 0;

  for (const col of cols) {
    if (typeof col.width === "string" && col.width.endsWith("%")) {
      const pct = parseFloat(col.width) / 100;
      const cw = pct * w;
      rawWidths.push(cw);
      remainingW -= cw;
    } else if (typeof col.width === "string" && col.width.endsWith("mm")) {
      const cw = mmToPx(parseFloat(col.width), vp.dpi, vp.zoom);
      rawWidths.push(cw);
      remainingW -= cw;
    } else if (typeof col.width === "number") {
      const cw = mmToPx(col.width, vp.dpi, vp.zoom);
      rawWidths.push(cw);
      remainingW -= cw;
    } else {
      rawWidths.push(-1);
      autoCount++;
    }
  }

  const autoWidth = autoCount > 0 ? Math.max(remainingW / autoCount, 20) : 0;
  let widths = rawWidths.map((val) => (val === -1 ? autoWidth : val));
  const sumW = widths.reduce((a, b) => a + b, 0);
  if (sumW > 0 && Math.abs(sumW - w) > 1) {
    const scale = w / sumW;
    widths = widths.map((val) => val * scale);
  }

  const headerH = showHeader ? mmToPx(style.headerFontSize ? style.headerFontSize * 0.9 : 8, vp.dpi, vp.zoom) : 0;
  const rowH = mmToPx(style.rowHeight ? style.rowHeight * 0.35 : 7, vp.dpi, vp.zoom);
  const totalsH = showTotals ? rowH : 0;

  const availableH = h - headerH - totalsH;
  const numRows = Math.max(2, Math.min(7, Math.floor(availableH / Math.max(rowH, 10))));
  const actualRowH = availableH > 0 && numRows > 0 ? availableH / numRows : rowH;

  const colPositions: { x: number; width: number; col: any }[] = [];
  if (isRtl) {
    let currentX = w;
    for (let i = 0; i < cols.length; i++) {
      currentX -= widths[i]!;
      colPositions.push({ x: currentX, width: widths[i]!, col: cols[i] });
    }
  } else {
    let currentX = 0;
    for (let i = 0; i < cols.length; i++) {
      colPositions.push({ x: currentX, width: widths[i]!, col: cols[i] });
      currentX += widths[i]!;
    }
  }

  return {
    cols,
    colPositions,
    showHeader,
    showTotals,
    showGrid,
    showBorder,
    headerH,
    rowH: actualRowH,
    totalsH,
    numRows,
    headerBgColor: style.headerBgColor || "#0f172a",
    headerTextColor: style.headerTextColor || "#ffffff",
    alternateRowBgColor: style.alternateRowBgColor || "#f8fafc",
    borderColor: style.borderColor || "#cbd5e1",
    borderWidth: style.borderWidth ?? 0.75,
    headerFontSize: style.headerFontSize ?? 8,
    fontSize: style.fontSize ?? 7.5,
  };
});

function getCellDisplayText(col: any, rowIdx: number, runtimeRows?: any[]): string {
  const field = col.field || col.key || "";
  if (runtimeRows && runtimeRows[rowIdx]) {
    const val = runtimeRows[rowIdx][field];
    return val != null ? String(val) : "";
  }
  const f = field.toLowerCase();
  if (f === "seq" || f === "#") return String(rowIdx + 1);
  if (!field) return "—";
  return `{{${field}}}`;
}

function getTotalDisplayText(col: any): string {
  if (col.total) {
    if (col.total === "label") return col.totalLabel || col.total_label || col.headerAr || "المجموع";
    return `Σ (${col.total})`;
  }
  if (col.totalLabel || col.total_label) return col.totalLabel || col.total_label;
  return "—";
}
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
            use:focusEditable
            contenteditable="true"
            oninput={handleEditInput}
            onblur={commitEdit}
            onkeydown={handleEditKeydown}
            style="
              width: {w}px;
              min-height: {h}px;
              font-size: {mmToPx((nodeProps.fontSize ?? 12) * 0.352778, vp.dpi, vp.zoom)}px;
              font-family: {nodeProps.fontFamily || 'Rubik'}, sans-serif;
              font-weight: {nodeProps.fontWeight === 'bold' ? 700 : nodeProps.fontWeight === 'light' ? 300 : nodeProps.fontWeight || 400};
              font-style: {nodeProps.fontStyle || 'normal'};
              text-decoration: {nodeProps.textDecoration || 'none'};
              color: {nodeProps.color || '#000000'};
              text-align: {nodeProps.textAlign || 'start'};
              direction: {nodeProps.direction === 'auto' ? 'inherit' : nodeProps.direction || 'inherit'};
              line-height: {nodeProps.lineHeight || 1.4};
              letter-spacing: {nodeProps.letterSpacing || 0}px;
              text-transform: {nodeProps.textTransform || 'none'};
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
          >{editingContent}</div>
        {:else}
          <!-- Display mode -->
          <div
            xmlns="http://www.w3.org/1999/xhtml"
            style="
              width: {w}px;
              height: {h}px;
              font-size: {mmToPx((nodeProps.fontSize ?? 12) * 0.352778, vp.dpi, vp.zoom)}px;
              font-family: {nodeProps.fontFamily || 'Rubik'}, sans-serif;
              font-weight: {nodeProps.fontWeight === 'bold' ? 700 : nodeProps.fontWeight === 'light' ? 300 : nodeProps.fontWeight || 400};
              font-style: {nodeProps.fontStyle || 'normal'};
              text-decoration: {nodeProps.textDecoration || 'none'};
              color: {nodeProps.color || '#000000'};
              text-align: {nodeProps.textAlign || 'start'};
              direction: {nodeProps.direction === 'auto' ? 'inherit' : nodeProps.direction || 'inherit'};
              line-height: {nodeProps.lineHeight || 1.4};
              letter-spacing: {nodeProps.letterSpacing || 0}px;
              text-transform: {nodeProps.textTransform || 'none'};
              background: {nodeProps.backgroundColor || 'transparent'};
              display: {nodeProps.maxLines ? '-webkit-box' : 'flex'};
              flex-direction: column;
              justify-content: {nodeProps.verticalAlign === 'center' ? 'center' : nodeProps.verticalAlign === 'bottom' ? 'flex-end' : 'flex-start'};
              overflow: {nodeProps.overflow === 'visible' ? 'visible' : 'hidden'};
              -webkit-line-clamp: {nodeProps.maxLines || 'unset'};
              -webkit-box-orient: vertical;
              word-wrap: break-word;
              padding: 0;
              margin: 0;
            "
          >
            <span style="display: inline;">{#each textSegments as seg, i (i)}{#if seg.type === "variable"}<span style="background: #dbeafe; color: #1d4ed8; border-radius: 3px; padding: 0 3px; font-size: 0.85em; font-weight: 600; white-space: nowrap;">{`{{${seg.key}}}`}</span>{:else}{seg.value}{/if}{/each}{#if textSegments.length === 0}{nodeProps.content || ""}{/if}</span>
          </div>
        {/if}
      </foreignObject>

    {:else if node.type === "Image"}
      <!-- Image Node -->
      {#if nodeProps.src || nodeProps.fallbackSrc}
        {@const currentSrc = (nodeProps.src && !nodeProps.src.startsWith("{{")) ? nodeProps.src : (nodeProps.fallbackSrc || nodeProps.src)}
        {#await resolveDocumentUrl(currentSrc, nodeProps.assetRef) then resolvedSrc}
          <image
            href={resolvedSrc}
            width={w}
            height={h}
            preserveAspectRatio={nodeProps.fit === "cover" ? "xMidYMid slice" : nodeProps.fit === "fill" ? "none" : "xMidYMid meet"}
          />
          {#if nodeProps.borderWidth > 0}
            <rect
              width={w}
              height={h}
              rx={mmToPx(nodeProps.borderRadius ?? 0, vp.dpi, vp.zoom)}
              fill="none"
              stroke={nodeProps.borderColor || "#000000"}
              stroke-width={nodeProps.borderWidth}
            />
          {/if}
        {:catch error}
          {@const _imageError = console.error("Failed to load template image:", error)}
          <rect width={w} height={h} fill="#f1f5f9" stroke="#cbd5e1" stroke-width="1" stroke-dasharray="4,2" rx="2" />
        {/await}
      {:else}
        <!-- Image placeholder -->
        <rect width={w} height={h} fill="#f1f5f9" stroke="#cbd5e1" stroke-width="1" stroke-dasharray="4,2" rx="2" />
        <text x={w/2} y={h/2} text-anchor="middle" dominant-baseline="central" fill="#94a3b8" font-size="10">📷</text>
      {/if}

    {:else if node.type === "Shape"}
      <!-- Shape Node -->
      {@const shapeType = nodeProps.shapeType || nodeProps.shape}
      {@const fill = nodeProps.fill || nodeProps.fillColor || "transparent"}
      {@const stroke = nodeProps.stroke || nodeProps.strokeColor || "#000"}
      {#if shapeType === "circle" || shapeType === "ellipse"}
        <ellipse
          cx={w / 2}
          cy={h / 2}
          rx={w / 2}
          ry={h / 2}
          fill={fill}
          stroke={stroke}
          stroke-width={nodeProps.strokeWidth || 1}
        />
      {:else if shapeType === "line"}
        <line x1="0" y1={h / 2} x2={w} y2={h / 2} stroke={stroke} stroke-width={nodeProps.strokeWidth || 1} />
      {:else}
        <rect
          width={w}
          height={h}
          rx={mmToPx(shapeType === "rounded-rect" ? (nodeProps.borderRadius ?? 4) : (nodeProps.borderRadius ?? 0), vp.dpi, vp.zoom)}
          fill={fill}
          stroke={stroke}
          stroke-width={nodeProps.strokeWidth || 1}
        />
      {/if}

    {:else if node.type === "Separator"}
      <!-- Separator -->
      {#if nodeProps.direction === "vertical"}
        <line x1={w / 2} y1="0" x2={w / 2} y2={h} stroke={nodeProps.color || "#d1d5db"} stroke-width={nodeProps.thickness || 0.5} />
      {:else}
        <line x1="0" y1={h / 2} x2={w} y2={h / 2} stroke={nodeProps.color || "#d1d5db"} stroke-width={nodeProps.thickness || 0.5} />
      {/if}

    {:else if node.type === "Spacer"}
      <!-- Spacer (invisible but selectable) -->
      <rect width={w} height={h} fill="transparent" stroke={isSelected ? "#3b82f6" : "transparent"} stroke-width="1" stroke-dasharray="2,2" />

    {:else if node.type === "Barcode"}
      <!-- Barcode placeholder -->
      <rect width={w} height={h} fill="white" stroke="#e2e8f0" stroke-width="1" rx="2" />
      <g transform="translate(4, 2)">
        {#each Array(12) as _, i (i)}
          <rect x={i * ((w - 8) / 12)} y="0" width={((w - 8) / 12) * 0.6} height={h - 10} fill="#000" />
        {/each}
      </g>
      <text x={w / 2} y={h - 2} text-anchor="middle" fill="#374151" font-size="7" font-family="monospace">
        {nodeProps.data || "BARCODE"}
      </text>

    {:else if node.type === "QRCode"}
      <!-- QR Code placeholder -->
      <rect width={w} height={h} fill="white" stroke="#e2e8f0" stroke-width="1" rx="2" />
      <rect x={w * 0.15} y={h * 0.15} width={w * 0.7} height={h * 0.7} fill="#1a1a2e" rx="2" />
      <rect x={w * 0.2} y={h * 0.2} width={w * 0.15} height={h * 0.15} fill="white" rx="1" />
      <rect x={w * 0.65} y={h * 0.2} width={w * 0.15} height={h * 0.15} fill="white" rx="1" />
      <rect x={w * 0.2} y={h * 0.65} width={w * 0.15} height={h * 0.15} fill="white" rx="1" />

    {:else if node.type === "Table"}
      {#if tableLayout}
        {@const tbl = tableLayout}
        <!-- Table container background -->
        <rect width={w} height={h} fill="#ffffff" />

        <!-- Header Row -->
        {#if tbl.showHeader}
          <rect x="0" y="0" width={w} height={tbl.headerH} fill={tbl.headerBgColor} />
          {#each tbl.colPositions as cp, i (cp.col.id || i)}
            {@const align = cp.col.alignment || cp.col.align || "center"}
            {@const textX = align === "right" ? cp.x + cp.width - 4 : align === "left" ? cp.x + 4 : cp.x + cp.width / 2}
            {@const textAnchor = align === "right" ? "end" : align === "left" ? "start" : "middle"}
            <text
              x={textX}
              y={tbl.headerH / 2}
              text-anchor={textAnchor}
              dominant-baseline="central"
              fill={tbl.headerTextColor}
              font-size={tbl.headerFontSize}
              font-weight="bold"
            >
              {cp.col.headerAr || cp.col.header || cp.col.key || cp.col.field}
            </text>
          {/each}
        {/if}

        <!-- Data Rows -->
        {#each Array(tbl.numRows) as _, ri (ri)}
          {@const rowY = tbl.headerH + ri * tbl.rowH}
          {#if ri % 2 === 1}
            <rect x="0" y={rowY} width={w} height={tbl.rowH} fill={tbl.alternateRowBgColor} />
          {/if}
          {#each tbl.colPositions as cp, i (cp.col.id || i)}
            {@const align = cp.col.alignment || cp.col.align || "center"}
            {@const textX = align === "right" ? cp.x + cp.width - 4 : align === "left" ? cp.x + 4 : cp.x + cp.width / 2}
            {@const textAnchor = align === "right" ? "end" : align === "left" ? "start" : "middle"}
            <text
              x={textX}
              y={rowY + tbl.rowH / 2}
              text-anchor={textAnchor}
              dominant-baseline="central"
              fill="#1e293b"
              font-size={tbl.fontSize}
            >
              {getCellDisplayText(cp.col, ri, nodeProps.rows)}
            </text>
          {/each}
          {#if tbl.showGrid}
            <line x1="0" y1={rowY + tbl.rowH} x2={w} y2={rowY + tbl.rowH} stroke={tbl.borderColor} stroke-width={tbl.borderWidth} />
          {/if}
        {/each}

        <!-- Totals Row -->
        {#if tbl.showTotals}
          {@const totY = tbl.headerH + tbl.numRows * tbl.rowH}
          <rect x="0" y={totY} width={w} height={tbl.totalsH} fill="#f1f5f9" />
          {#each tbl.colPositions as cp, i (cp.col.id || i)}
            {@const align = cp.col.alignment || cp.col.align || "center"}
            {@const textX = align === "right" ? cp.x + cp.width - 4 : align === "left" ? cp.x + 4 : cp.x + cp.width / 2}
            {@const textAnchor = align === "right" ? "end" : align === "left" ? "start" : "middle"}
            <text
              x={textX}
              y={totY + tbl.totalsH / 2}
              text-anchor={textAnchor}
              dominant-baseline="central"
              fill="#0f172a"
              font-size={tbl.fontSize}
              font-weight="bold"
            >
              {getTotalDisplayText(cp.col)}
            </text>
          {/each}
          {#if tbl.showGrid}
            <line x1="0" y1={totY} x2={w} y2={totY} stroke={tbl.borderColor} stroke-width={tbl.borderWidth} />
          {/if}
        {/if}

        <!-- Vertical Grid Dividers -->
        {#if tbl.showGrid}
          {#each tbl.colPositions.slice(1) as cp, i (cp.col.id || i)}
            <line x1={cp.x} y1="0" x2={cp.x} y2={h} stroke={tbl.borderColor} stroke-width={tbl.borderWidth} />
          {/each}
        {/if}

        <!-- Outer Border -->
        {#if tbl.showBorder}
          <rect x="0" y="0" width={w} height={h} fill="none" stroke={tbl.borderColor} stroke-width={tbl.borderWidth} />
        {/if}
      {:else}
        <!-- Fallback empty table placeholder -->
        <rect width={w} height={h} fill="white" stroke="#d1d5db" stroke-width="1" />
        <text x={w / 2} y={h / 2} text-anchor="middle" dominant-baseline="central" fill="#94a3b8" font-size="9">
          جدول بدون أعمدة (انقر لإضافة أعمدة في لوحة الخصائص)
        </text>
      {/if}

    {:else if node.type === "Frame" || node.type === "Grid" || node.type === "Repeater" || node.type === "Conditional" || node.type === "Header" || node.type === "Footer"}
      <!-- Container: render border then children -->
      {#if nodeProps.backgroundColor || nodeProps.borderColor}
        <rect
          width={w}
          height={h}
          rx={mmToPx(nodeProps.borderRadius ?? 0, vp.dpi, vp.zoom)}
          fill={nodeProps.backgroundColor || "transparent"}
          stroke={nodeProps.borderColor || "transparent"}
          stroke-width={nodeProps.borderWidth || 0}
        />
      {:else if isSelected}
        <rect width={w} height={h} fill="transparent" stroke="#3b82f6" stroke-width="1" stroke-dasharray="4,2" opacity="0.4" />
      {/if}

      <!-- Render children recursively -->
      {#if "children" in node && Array.isArray((node as any).children)}
        {#each (node as any).children as child (child.id)}
          <CanvasNode node={child} {store} {pageSizePx} />
        {/each}
      {/if}

    {:else if node.type === "PageBreak"}
      <line x1="0" y1="0" x2={w} y2="0" stroke="#f59e0b" stroke-width="1" stroke-dasharray="6,3" />
      <text x="4" y="10" fill="#b45309" font-size="8">{m.shared_ui_element_page_break()}</text>

    {:else if node.type === "Watermark"}
      <text
        x={w / 2}
        y={h / 2}
        text-anchor="middle"
        dominant-baseline="central"
        fill={nodeProps.color || "#64748b"}
        opacity={nodeProps.opacity ?? 0.15}
        font-size={nodeProps.fontSize || 48}
        transform="rotate({nodeProps.rotation || -35} {w / 2} {h / 2})"
      >{nodeProps.text || "WATERMARK"}</text>

    {:else if node.type === "ComponentRef"}
      <rect width={w} height={h} fill="#eff6ff" stroke="#60a5fa" stroke-width="1" stroke-dasharray="4,2" />
      <text x={w / 2} y={h / 2} text-anchor="middle" dominant-baseline="central" fill="#2563eb" font-size="8">
        Component: {nodeProps.componentId || "未選択"}
      </text>

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
