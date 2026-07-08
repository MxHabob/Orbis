<!--
@component BulkGrid
Enterprise-grade data grid for bulk data entry.
Wraps OrbisGrid with proper paste handling, validation overlays, and auto-expand.

Key design: Uses OrbisGrid's native onpaste callback (which fires from RevoGrid's
beforepaste event) to expand rows when needed, and oncelledit for data model sync.
RevoGrid handles the actual cell population internally.
-->
<script lang="ts">
import { OrbisGrid } from "@orbis/svelte-grid";
import type { CellEditEvent, OrbisColumnDef } from "@orbis/svelte-grid";
import { addRowIndices } from "$lib/tables";
import { getLocale } from "$lib/paraglide/runtime";
import { parseTSV, mapTSVToRows, isHeaderRow } from "./paste-handler";

// ── Props ──────────────────────────────────────────────────────────
interface Props {
  /** Column definitions */
  columns: OrbisColumnDef[];
  /** Data property keys in column order (for paste mapping) */
  dataProps: string[];
  /** Grid data rows */
  rows: Record<string, unknown>[];
  /** Header patterns for detecting pasted header rows */
  headerPatterns?: RegExp[];
  /** Minimum empty rows to keep at the bottom */
  minEmptyRows?: number;
  /** Factory function for creating empty rows */
  createEmptyRow: () => Record<string, unknown>;

  onrowschange?: (rows: Record<string, unknown>[]) => void;
}

let {
  columns,
  dataProps,
  rows = $bindable([]),
  headerPatterns = [],
  minEmptyRows = 5,
  createEmptyRow,
  onrowschange,
}: Props = $props();

const isRtl = getLocale() === "ar";

// ── Indexed source for display ─────────────────────────────────────
let indexedSource = $derived(addRowIndices(rows));

// ── Cell Edit Handler ──────────────────────────────────────────────
// Called by OrbisGrid after RevoGrid applies edits (including from paste)
function handleCellEdit(event: CellEditEvent) {
  const { rowIndex, prop, newValue } = event;
  if (rowIndex < 0 || rowIndex >= rows.length) return;

  rows[rowIndex][prop] = newValue;

  // Auto-append rows when editing near the end
  if (rowIndex >= rows.length - 3) {
    appendEmptyRows(minEmptyRows);
  }

  rows = [...rows];
  onrowschange?.(rows);
}

// ── Paste Handler ──────────────────────────────────────────────────
// Called by OrbisGrid's onpaste when data is pasted from clipboard.
// RevoGrid handles the actual cell population — we just need to:
// 1. Ensure enough rows exist for the pasted data
// 2. Sync back the data from RevoGrid's internal model to our rows
function handlePaste(data: string[][], startRow: number, _startCol: number) {
  if (!data || data.length === 0) return;

  // Calculate how many additional rows we need
  const endRow = startRow + data.length;
  const currentLen = rows.length;

  if (endRow > currentLen) {
    // Expand rows to accommodate pasted data
    const newRows = [...rows];
    for (let i = currentLen; i < endRow + minEmptyRows; i++) {
      newRows.push(createEmptyRow());
    }
    rows = newRows;
  }

  // RevoGrid will handle cell population via afteredit events.
  // However, afteredit fires asynchronously, so we also manually
  // sync the pasted data into our model to ensure consistency.
  const newRows = [...rows];
  for (let r = 0; r < data.length; r++) {
    const targetIdx = startRow + r;
    if (targetIdx >= newRows.length) break;

    for (let c = 0; c < data[r].length; c++) {
      // Map column index to property name (skip index column at 0)
      const propIdx = c; // columns[0] is #, data doesn't include it
      if (propIdx < dataProps.length) {
        const val = data[r][c]?.trim() ?? "";
        newRows[targetIdx][dataProps[propIdx]] = val;
      }
    }

    // Reset errors for modified rows
    if (newRows[targetIdx].__errors instanceof Map) {
      (newRows[targetIdx].__errors as Map<string, string>).clear();
    }
  }

  // Ensure empty tail
  ensureEmptyTail(newRows);
  rows = newRows;
  onrowschange?.(rows);
}

// Also handle raw clipboard paste as fallback when grid isn't focused
function handleClipboardPaste(e: ClipboardEvent) {
  const text = e.clipboardData?.getData("text/plain");
  if (!text) return;

  // Take complete control over the paste operation
  e.preventDefault();
  e.stopPropagation();

  let pastedData = parseTSV(text);
  if (pastedData.length === 0) return;

  // Detect and skip header row
  if (pastedData.length > 1 && headerPatterns.length > 0) {
    if (isHeaderRow(pastedData[0], headerPatterns)) {
      pastedData = pastedData.slice(1);
    }
  }

  // Convert 2D TSV to standard row objects
  const pastedRows = mapTSVToRows(pastedData, dataProps);
  if (pastedRows.length === 0) return;

  // ── Find active cell coordinates ──
  const activeEl = document.activeElement as HTMLElement;
  const targetEl = e.target as HTMLElement;

  // Find closest cell container
  const cell = (activeEl?.closest(".rgCell") ||
    targetEl?.closest(".rgCell") ||
    // RevoGrid overlays selection elements outside cells, check if target is near selection
    document.querySelector("revo-grid .rgCell.focus")) as HTMLElement;

  let startRow = findFirstEmptyRow();
  let startColProp = dataProps[0];

  if (cell) {
    const r = cell.getAttribute("data-row");
    const p = cell.getAttribute("data-prop");
    if (r !== null) startRow = Number(r);
    if (p !== null) startColProp = p;
  }

  const startColIdx = dataProps.indexOf(startColProp);
  const newRows = [...rows];

  for (let r = 0; r < pastedRows.length; r++) {
    const targetRowIdx = startRow + r;

    // Ensure row exists in grid
    if (targetRowIdx >= newRows.length) {
      newRows.push(createEmptyRow());
    }

    // Write pasted columns matching the starting column offset
    const maxCols = Math.min(dataProps.length - startColIdx, dataProps.length);
    for (let c = 0; c < maxCols; c++) {
      const sourceColProp = dataProps[c]; // raw column property name
      const targetColProp = dataProps[startColIdx + c]; // offset destination property

      if (pastedRows[r][sourceColProp] !== undefined && targetColProp) {
        newRows[targetRowIdx][targetColProp] = pastedRows[r][sourceColProp];
      }
    }

    // Reset validation errors for updated row
    if (newRows[targetRowIdx].__errors instanceof Map) {
      (newRows[targetRowIdx].__errors as Map<string, string>).clear();
    }
  }

  // Maintain minimum tail of empty rows
  ensureEmptyTail(newRows);

  rows = newRows;
  onrowschange?.(rows);
}

// ── Helpers ────────────────────────────────────────────────────────
function findFirstEmptyRow(): number {
  const idx = rows.findIndex((r) =>
    dataProps.every((p) => !String(r[p] ?? "").trim()),
  );
  return idx === -1 ? rows.length : idx;
}

function ensureEmptyTail(arr: Record<string, unknown>[]) {
  let emptyCount = 0;
  for (let i = arr.length - 1; i >= 0; i--) {
    if (dataProps.every((p) => !String(arr[i][p] ?? "").trim())) {
      emptyCount++;
    } else {
      break;
    }
  }
  while (emptyCount < minEmptyRows) {
    arr.push(createEmptyRow());
    emptyCount++;
  }
}

function appendEmptyRows(count: number) {
  const newRows = [...rows];
  for (let i = 0; i < count; i++) {
    newRows.push(createEmptyRow());
  }
  rows = newRows;
}

// ── Initialize with empty rows ─────────────────────────────────────
$effect(() => {
  if (rows.length === 0) {
    const initial: Record<string, unknown>[] = [];
    for (let i = 0; i < 20; i++) {
      initial.push(createEmptyRow());
    }
    rows = initial;
    onrowschange?.(rows);
  }
});
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="flex flex-col gap-1 w-full">
  <div
    class="bulk-grid-wrapper"
    onpaste={handleClipboardPaste}
  >
    <OrbisGrid
      {columns}
      source={indexedSource}
      height="calc(100vh - 180px)"
      rtl={isRtl}
      readonly={false}
      theme="compact"
      range={true}
      clipboard={true}
      oncelledit={handleCellEdit}
      oncellspaste={handlePaste}
    />
  </div>
</div>

<style>
  .bulk-grid-wrapper {
    width: 100%;
    border-radius: var(--radius, 0.75rem);
    overflow: hidden;
    border: 1px solid var(--border);
  }
</style>
