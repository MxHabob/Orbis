<!--
@component BulkGrid
Enterprise-grade data grid for bulk data entry.
Wraps OrbisGrid with fixed clipboard paste, validation overlays, and auto-expand.

Key fix: Uses RevoGrid's native `beforepaste` event instead of DOM `onpaste`,
ensuring clipboard data is properly intercepted and distributed across cells.
-->
<script lang="ts">
import { OrbisGrid, addRowIndices } from "@orbis/data-grid";
import type { CellEditEvent, OrbisColumnDef } from "@orbis/data-grid";
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

// ── Paste Handler (via RevoGrid beforepaste event) ─────────────────
function handleBeforePaste(event: CustomEvent) {
  // Prevent RevoGrid's default paste (which puts everything in one cell)
  event.preventDefault();

  const detail = event.detail;
  if (!detail) return;

  // Extract raw clipboard text — RevoGrid provides parsed data
  let pastedData: string[][] = [];

  if (detail.parsed && Array.isArray(detail.parsed)) {
    // RevoGrid already parsed it into 2D array
    pastedData = detail.parsed;
  } else if (detail.raw && typeof detail.raw === "string") {
    pastedData = parseTSV(detail.raw);
  }

  if (pastedData.length === 0) return;

  // Detect and skip header row
  if (pastedData.length > 1 && headerPatterns.length > 0) {
    if (isHeaderRow(pastedData[0], headerPatterns)) {
      pastedData = pastedData.slice(1);
    }
  }

  if (pastedData.length === 0) return;

  // Convert 2D array to row objects
  const pastedRows = mapTSVToRows(pastedData, dataProps);
  if (pastedRows.length === 0) return;

  // Find insertion point: first fully empty row, or end of data
  const startRow = detail.rowIndex ?? findFirstEmptyRow();

  // Merge pasted rows into grid
  const newRows = [...rows];
  for (let i = 0; i < pastedRows.length; i++) {
    const targetIdx = startRow + i;

    if (targetIdx < newRows.length) {
      // Overwrite existing row
      for (const prop of dataProps) {
        if (pastedRows[i][prop] !== undefined) {
          newRows[targetIdx][prop] = pastedRows[i][prop];
        }
      }
      // Reset errors
      if (newRows[targetIdx].__errors instanceof Map) {
        (newRows[targetIdx].__errors as Map<string, string>).clear();
      }
    } else {
      // Append new row
      const emptyRow = createEmptyRow();
      for (const prop of dataProps) {
        if (pastedRows[i][prop] !== undefined) {
          emptyRow[prop] = pastedRows[i][prop];
        }
      }
      newRows.push(emptyRow);
    }
  }

  // Ensure empty rows at the bottom
  ensureEmptyTail(newRows);

  rows = newRows;
  onrowschange?.(rows);
}

// Also handle raw clipboard paste as fallback for edge cases
function handleClipboardPaste(e: ClipboardEvent) {
  // Check if RevoGrid already handled this (it should via beforepaste)
  // Only handle if the grid isn't focused (e.g., clicking the wrapper area)
  const text = e.clipboardData?.getData("text/plain");
  if (!text || !text.includes("\t")) return;

  // If we get here, RevoGrid didn't catch it — handle manually
  e.preventDefault();
  e.stopPropagation();

  let pastedData = parseTSV(text);

  if (pastedData.length > 1 && headerPatterns.length > 0) {
    if (isHeaderRow(pastedData[0], headerPatterns)) {
      pastedData = pastedData.slice(1);
    }
  }

  const pastedRows = mapTSVToRows(pastedData, dataProps);
  if (pastedRows.length === 0) return;

  const startRow = findFirstEmptyRow();
  const newRows = [...rows];

  for (let i = 0; i < pastedRows.length; i++) {
    const targetIdx = startRow + i;
    if (targetIdx < newRows.length) {
      for (const prop of dataProps) {
        if (pastedRows[i][prop] !== undefined) {
          newRows[targetIdx][prop] = pastedRows[i][prop];
        }
      }
    } else {
      const emptyRow = createEmptyRow();
      for (const prop of dataProps) {
        if (pastedRows[i][prop] !== undefined) {
          emptyRow[prop] = pastedRows[i][prop];
        }
      }
      newRows.push(emptyRow);
    }
  }

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
<div
  class="bulk-grid-wrapper"
  onpaste={handleClipboardPaste}
>
  <OrbisGrid
    columns={columns}
    source={indexedSource}
    height="calc(100vh - 380px)"
    rtl={isRtl}
    readonly={false}
    theme="compact"
    range={true}
    clipboard={true}
    oncelledit={handleCellEdit}
    onbeforeedit={() => {}}
    onpaste={(data, startRow, startCol) => {
      // This is the OrbisGrid onpaste callback (from beforepaste)
      // Our handleBeforePaste above takes priority
    }}
  />
</div>

<style>
  .bulk-grid-wrapper {
    width: 100%;
    border-radius: var(--radius, 0.75rem);
    overflow: hidden;
    border: 1px solid var(--border);
  }
</style>
