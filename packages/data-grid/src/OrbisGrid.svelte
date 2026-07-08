<!--
@component OrbisGrid
Enterprise data grid component wrapping RevoGrid with S.C.R.A.P.S. theming.
Designed for spreadsheet-like data entry (grades, attendance, etc.) using Svelte 5 Runes.
-->
<script lang="ts">
import { onMount, tick } from "svelte";
import { toast } from "svelte-sonner";
import { RevoGrid, Editor } from "@revolist/svelte-datagrid";
import { defineCustomElements } from "@revolist/revogrid/loader";

import NumberEditor from "./editors/NumberEditor.svelte";
import TextEditor from "./editors/TextEditor.svelte";

import type { OrbisColumnDef, CellEditEvent, CellValidation } from "./types";
import type { GridActionEvent } from "./renderers/types";
import "./orbis-grid.css";

const isBrowser = typeof window !== "undefined";

// ── Props ────────────────────────────────────────────────────────────────

interface Props {
  /** Column definitions */
  columns: OrbisColumnDef[];
  /** Source data array */
  source: Record<string, unknown>[];
  /** Grid height (CSS value) */
  height?: string;
  /** Enable RTL layout */
  rtl?: boolean;
  /** Make entire grid read-only */
  readonly?: boolean;
  /** RevoGrid theme */
  theme?: "compact" | "material" | "default";
  /** Enable range selection */
  range?: boolean;
  /** Enable clipboard operations */
  clipboard?: boolean;
  /** Enable column auto-size */
  autoSize?: boolean;
  /** Number of extra rows beyond viewport */
  overscan?: number;
  /** Row height in pixels */
  rowSize?: number;
  /** Unique row identifier key */
  rowIdKey?: string;
  /** Cell validation rules */
  validations?: CellValidation[];

  // ── Events ──────────────────────────────────────────────────────────
  /** Fired when a cell value is edited */
  oncelledit?: (event: CellEditEvent) => void;
  /** Fired before cell edit — return false to cancel */
  onbeforeedit?: (event: CellEditEvent) => boolean | void;
  /** Fired when user pastes from clipboard */
  oncellspaste?: (data: string[][], startRow: number, startCol: number) => void;
  /** Fired when a user clicks a [data-grid-action] element in a cell */
  onaction?: (event: GridActionEvent) => void;
}

let {
  columns,
  source,
  height = "500px",
  rtl = false,
  readonly = false,
  theme = "compact",
  range = true,
  clipboard = true,
  autoSize = false,
  overscan = 15,
  rowSize = 42,
  rowIdKey,
  validations = [],
  oncelledit,
  onbeforeedit,
  oncellspaste,
  onaction,
}: Props = $props();

let gridReady = $state(false);
let gridContainer: HTMLDivElement | undefined = $state();
let gridElement: any = $state(null);
let ignoreNextSourceUpdate = $state(false);

// Register custom editors statically using the Editor wrapper
const loadedEditors = {
  number: Editor(NumberEditor),
  text: Editor(TextEditor),
};

onMount(() => {
  if (isBrowser) {
    try {
      defineCustomElements();
      gridReady = true;
    } catch (err) {
      console.error("[OrbisGrid] Failed to initialize custom elements:", err);
    }
  }
});

// ── Map OrbisColumnDef → RevoGrid ColumnRegular ──────────────────────────

let revoColumns = $derived.by(() => {
  return columns.map((col) => {
    const revoCol: Record<string, unknown> = {
      prop: col.prop,
      name: col.name,
      size: col.size ?? 120,
      sortable: col.sortable ?? false,
      resizable: col.resizable !== false,
      readonly: readonly || (col.readonly ?? false),
      cellProperties: (params?: any) => {
        const classes: string[] = [];
        if (col.variant) classes.push(`orbis-cell-${col.variant}`);
        if (col.cellClass) classes.push(col.cellClass);
        if (col.align === "center" || col.align === "start" || col.align === "end") {
          classes.push(`text-${col.align}`);
        }

        const row = params?.model || {};
        const rowIndex = params?.rowIndex ?? 0;
        const prop = col.prop;

        let customProps: Record<string, any> = {};
        if (col.cellProperties) {
          customProps = col.cellProperties(row) || {};
        }

        if (customProps.class) {
          classes.push(customProps.class);
        }

        const cellProps: Record<string, any> = {
          ...customProps,
          class: classes.join(" ").trim(),
          "data-row": rowIndex,
          "data-prop": prop,
        };

        const isCustom = col.cellRenderer !== undefined;
        const isSelection = prop === "__selected";

        if (!isCustom && !isSelection) {
          const clickHandler = (e: MouseEvent) => {
            if (!onaction) return;
            onaction({
              actionId: "select",
              row,
              rowIndex,
              prop,
              originalEvent: e,
            });
          };

          const doubleClickHandler = (e: MouseEvent) => {
            if (!onaction) return;
            onaction({
              actionId: "dblclick",
              row,
              rowIndex,
              prop,
              originalEvent: e,
            });
          };

          cellProps.onClick = clickHandler;
          cellProps.onclick = clickHandler;
          cellProps.onDblClick = doubleClickHandler;
          cellProps.ondblclick = doubleClickHandler;
        }

        return cellProps;
      },
    };

    if (col.pin) revoCol.pin = col.pin;
    if (col.minSize) revoCol.minSize = col.minSize;
    if (col.maxSize) revoCol.maxSize = col.maxSize;

    if (col.meta || col.editorConfig) {
      revoCol.meta = {
        ...(col.meta || {}),
        ...(col.editorConfig || {}),
      };
    }

    if (!readonly && !col.readonly) {
      revoCol.editor = col.editor || "text";
    }

    if (col.cellRenderer) {
      revoCol.cellTemplate = (h: any, model: any) => {
        return col.cellRenderer!({
          h,
          value: model.model[model.prop],
          row: model.model,
          prop: model.prop,
          rowIndex: model.rowIndex,
          colIndex: model.colIndex,
          model,
        });
      };
    }

    return revoCol;
  });
});

// ── Native Event Listeners via DOM Delegation ──────────────────────────────
// This guarantees event cancellation (.preventDefault()) functions correctly.

$effect(() => {
  if (gridElement && gridReady) {
    const rawEl = gridElement.getWebComponent?.();
    if (!rawEl) return;

    const handleBeforeEditWrapper = (e: any) => handleBeforeEdit(e);
    const handleAfterEditWrapper = (e: any) => handleAfterEdit(e);
    const handleBeforePasteWrapper = (e: any) => handleBeforePaste(e);

    rawEl.addEventListener("beforeedit", handleBeforeEditWrapper);
    rawEl.addEventListener("afteredit", handleAfterEditWrapper);
    rawEl.addEventListener("beforepaste", handleBeforePasteWrapper);

    return () => {
      rawEl.removeEventListener("beforeedit", handleBeforeEditWrapper);
      rawEl.removeEventListener("afteredit", handleAfterEditWrapper);
      rawEl.removeEventListener("beforepaste", handleBeforePasteWrapper);
    };
  }
});

// ── Event Handlers ───────────────────────────────────────────────────────

async function handleAfterEdit(event: CustomEvent) {
  const detail = event.detail;
  if (!detail || !oncelledit) return;

  const editEvent: CellEditEvent = {
    row: detail.model || detail.row || {},
    rowIndex: detail.rowIndex ?? detail.index ?? 0,
    prop: detail.prop || "",
    oldValue: detail.oldValue ?? detail.val,
    newValue: detail.value ?? detail.val,
  };

  // Prevent reactive source loop resetting grid focus/editor state
  ignoreNextSourceUpdate = true;

  oncelledit(editEvent);

  // Await parent mutation and microtask tick
  await tick();
  ignoreNextSourceUpdate = false;
}

function handleBeforeEdit(event: CustomEvent) {
  const detail = event.detail;
  if (!detail) return;

  const editEvent: CellEditEvent = {
    row: detail.model || {},
    rowIndex: detail.rowIndex ?? 0,
    prop: String(detail.prop || ""),
    oldValue: detail.model ? detail.model[detail.prop] : undefined,
    newValue: detail.val,
  };

  // Run validations
  if (validations && validations.length > 0) {
    const rules = validations.filter((v) => v.prop === editEvent.prop);
    for (const rule of rules) {
      const errorMsg = rule.validate(editEvent.newValue, editEvent.row);
      if (errorMsg) {
        event.preventDefault();
        try {
          toast.error(errorMsg);
        } catch {
          console.warn("[OrbisGrid] Validation error:", errorMsg);
        }
        return;
      }
    }
  }

  if (onbeforeedit) {
    const result = onbeforeedit(editEvent);
    if (result === false) {
      event.preventDefault();
    }
  }
}

async function handleBeforePaste(event: CustomEvent) {
  const detail = event.detail;
  if (!detail) return;

  if (oncellspaste && detail.parsed) {
    oncellspaste(
      detail.parsed,
      detail.rowIndex ?? 0,
      detail.colIndex ?? 0
    );
  }

  // Prevent paste re-assigning source immediately
  ignoreNextSourceUpdate = true;
  await tick();
  ignoreNextSourceUpdate = false;
}

function handleGridClick(e: MouseEvent) {
  if (!onaction) return;

  const target = (e.target as HTMLElement).closest<HTMLElement>(
    "[data-grid-action]",
  );
  if (!target) return;

  const actionId = target.dataset.gridAction!;
  const rowIndex = Number(target.dataset.rowIndex ?? -1);
  const prop = target.dataset.prop ?? "";
  if (rowIndex < 0 || rowIndex >= source.length) return;

  onaction({
    actionId,
    row: source[rowIndex]!,
    rowIndex,
    prop,
    originalEvent: e,
  });
}

function handleGridDblClick(e: MouseEvent) {
  if (!onaction) return;

  const target = (e.target as HTMLElement).closest<HTMLElement>(
    "[data-grid-action]",
  );
  if (!target) return;

  const actionId = target.dataset.gridAction!;
  const rowIndex = Number(target.dataset.rowIndex ?? -1);
  const prop = target.dataset.prop ?? "";
  if (rowIndex < 0 || rowIndex >= source.length) return;

  if (actionId === "select") {
    onaction({
      actionId: "dblclick",
      row: source[rowIndex]!,
      rowIndex,
      prop,
      originalEvent: e,
    });
  }
}

// ── Exported Programmatic API ──────────────────────────────────────────────

export function getGridElement() {
  return gridElement;
}

export function setDataAt(row: number, col: number, val: any) {
  if (gridElement) {
    gridElement.setDataAt?.({ row, col, val });
  }
}

export function updateColumns(cols: any[]) {
  if (gridElement) {
    gridElement.updateColumns?.(cols);
  }
}

export function refresh() {
  if (gridElement) {
    gridElement.refresh?.();
  }
}

export function clearFocus() {
  if (gridElement) {
    gridElement.clearFocus?.();
  }
}

export function scrollToRow(index: number) {
  if (gridElement) {
    gridElement.scrollToRow?.(index);
  }
}

export function scrollToColumnIndex(index: number) {
  if (gridElement) {
    gridElement.scrollToColumnIndex?.(index);
  }
}

export function scrollToColumnProp(prop: string) {
  if (gridElement) {
    gridElement.scrollToColumnProp?.(prop);
  }
}

export async function getColumns() {
  return gridElement ? await gridElement.getColumns?.() : [];
}

export async function getSource() {
  return gridElement ? await gridElement.getSource?.() : [];
}
</script>

{#if gridReady && isBrowser}
  <div
    bind:this={gridContainer}
    class="orbis-grid-container"
    style="height: {height}; width: 100%;"
    dir={rtl ? "rtl" : "ltr"}
    onclick={handleGridClick}
    ondblclick={handleGridDblClick}
  >
    <RevoGrid
      bind:this={gridElement}
      columns={revoColumns}
      source={ignoreNextSourceUpdate ? null : source}
      theme={theme}
      range={range}
      resize={true}
      useClipboard={clipboard}
      autoSizeColumn={autoSize}
      rowSize={rowSize}
      readonly={readonly}
      rtl={rtl}
      canFocus={true}
      editors={loadedEditors}
      hideAttribution={true}
    />
  </div>
{:else}
  <!-- Loading skeleton using S.C.R.A.P.S. design tokens -->
  <div
    class="orbis-grid-skeleton"
    style="height: {height}; width: 100%;"
  >
    <div class="skeleton-header">
      {#each columns.slice(0, 6) as col}
        <div class="skeleton-header-cell" style="width: {col.size ?? 120}px;">
          <div class="skeleton-bar" style="width: 60%;"></div>
        </div>
      {/each}
    </div>
    {#each Array(8) as _, i}
      <div class="skeleton-row" style="opacity: {1 - i * 0.06}">
        {#each columns.slice(0, 6) as col}
          <div class="skeleton-cell" style="width: {col.size ?? 120}px;">
            <div class="skeleton-bar" style="width: {30 + Math.random() * 50}%;"></div>
          </div>
        {/each}
      </div>
    {/each}
  </div>
{/if}

<style>
  .orbis-grid-container {
    position: relative;
    border-radius: var(--radius, 0.5rem);
    overflow: hidden;
  }

  /* ── Loading Skeleton — S.C.R.A.P.S. themed ───────────────────────────── */
  .orbis-grid-skeleton {
    border: 1px solid var(--border);
    border-radius: var(--radius, 0.5rem);
    overflow: hidden;
    background: var(--card);
    box-shadow:
      0 0 0 1px oklch(0.3 0.05 280 / 0.04),
      0 1px 3px 0 oklch(0.3 0.05 280 / 0.08),
      0 2px 6px -1px oklch(0.3 0.05 280 / 0.05);
  }

  :global(.dark) .orbis-grid-skeleton {
    box-shadow:
      0 0 0 1px oklch(0.52 0.12 280 / 0.06),
      0 1px 3px 0 oklch(0 0 0 / 0.4),
      0 2px 8px -2px oklch(0 0 0 / 0.3);
  }

  .skeleton-header {
    display: flex;
    border-bottom: 1px solid var(--border);
    background: color-mix(in oklch, var(--muted), transparent 70%);
    height: 42px;
    align-items: center;
  }

  .skeleton-header-cell {
    padding: 0 12px;
    flex-shrink: 0;
    display: flex;
    align-items: center;
  }

  .skeleton-row {
    display: flex;
    border-bottom: 1px solid color-mix(in oklch, var(--border), transparent 50%);
    height: 42px;
    align-items: center;
  }

  .skeleton-cell {
    padding: 0 12px;
    flex-shrink: 0;
    display: flex;
    align-items: center;
  }

  .skeleton-bar {
    height: 10px;
    border-radius: 4px;
    background: var(--muted);
    position: relative;
    overflow: hidden;
  }

  .skeleton-bar::after {
    content: "";
    position: absolute;
    inset: 0;
    background: linear-gradient(
      90deg,
      transparent 0%,
      oklch(0.52 0.12 280 / 0.06) 40%,
      oklch(0.52 0.12 280 / 0.12) 50%,
      oklch(0.52 0.12 280 / 0.06) 60%,
      transparent 100%
    );
    background-size: 200% 100%;
    animation: orbis-grid-shimmer 2.5s linear infinite;
  }

  :global(.dark) .skeleton-bar::after {
    background: linear-gradient(
      90deg,
      transparent 0%,
      oklch(0.6 0.12 280 / 0.08) 40%,
      oklch(0.6 0.12 280 / 0.16) 50%,
      oklch(0.6 0.12 280 / 0.08) 60%,
      transparent 100%
    );
  }

  @keyframes orbis-grid-shimmer {
    0% { background-position: 200% 0; }
    100% { background-position: -200% 0; }
  }
</style>
