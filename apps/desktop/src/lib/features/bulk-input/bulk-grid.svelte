<!--
@component BulkGrid
High-performance data grid for bulk entry powered directly by @orbis/svelte-grid.
Eliminates duplicate wrapper handlers and relies on OrbisGrid's native clipboard,
keyboard navigation, and cell editing capabilities.
-->
<script lang="ts">
import { OrbisGrid } from "@orbis/svelte-grid";
import type { CellEditEvent, OrbisColumnDef, BulkResult } from "@orbis/svelte-grid";
import { getLocale } from "$lib/paraglide/runtime";

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
  dataProps: _dataProps,
  rows = $bindable([]),
  headerPatterns: _headerPatterns = [],
  minEmptyRows = 5,
  createEmptyRow,
  onrowschange,
}: Props = $props();

const isRtl = $derived(getLocale() === "ar");

// ── Cell Edit Handler ──────────────────────────────────────────────
function handleCellEdit(event: CellEditEvent) {
  const { rowIndex, columnId: prop, newValue } = event;
  if (rowIndex < 0 || rowIndex >= rows.length) return;

  rows[rowIndex][prop] = newValue;

  // Auto-append empty rows when editing near the end
  if (rowIndex >= rows.length - 3) {
    const newRows = [...rows];
    for (let i = 0; i < minEmptyRows; i++) {
      newRows.push(createEmptyRow());
    }
    rows = newRows;
  } else {
    rows = [...rows];
  }

  onrowschange?.(rows);
}

function handlePasteResult(_result: BulkResult) {
  rows = [...rows];
  onrowschange?.(rows);
}

// ── Initialize with empty rows if empty ───────────────────────────
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

<div class="flex flex-col gap-1 w-full max-w-full min-w-0 overflow-hidden flex-grow min-h-0 h-full">
  <div class="bulk-grid-wrapper flex-grow min-h-0 h-full">
    <OrbisGrid
      {columns}
      bind:source={rows}
      height="100%"
      rtl={isRtl}
      readonly={false}
      clipboard={true}
      range={true}
      rowIdKey="id"
      {createEmptyRow}
      oncelledit={handleCellEdit}
      onpasteResult={handlePasteResult}
    />
  </div>
</div>

<style>
  .bulk-grid-wrapper {
    width: 100%;
    max-width: 100%;
    min-width: 0;
    height: 100%;
    display: flex;
    flex-direction: column;
    border-radius: var(--radius, 0.75rem);
    overflow: hidden;
    border: 1px solid var(--border);
  }
</style>
