<!--
@component BulkInputGrid
Wraps OrbisGrid with bulk input logic: dynamic columns, paste handling,
real-time validation, and error highlighting.
-->
<script lang="ts">
import { OrbisGrid } from "@orbis/svelte-grid";
import type { CellEditEvent } from "@orbis/svelte-grid";
import { addRowIndices } from "$lib/tables";
import { getLocale } from "$lib/paraglide/runtime";
import { type BulkInputType, getColumnsForType } from "./bulk-input-config";
import { getDataProps } from "./bulk-input-config";
import {
  validateRows,
  getValidationSummary,
  type RowValidationResult,
} from "./bulk-input-validator";
import { parseExcelPaste, detectHeaderRow } from "./paste-handler";

// ── Props ──────────────────────────────────────────────────────────
interface Props {
  type: BulkInputType;
  labels: Record<string, string>;
  rows: Record<string, unknown>[];
  validationResults: RowValidationResult[];
  onrowschange?: (rows: Record<string, unknown>[]) => void;
  onvalidationchange?: (results: RowValidationResult[]) => void;
}

let {
  type,
  labels,
  rows = $bindable([]),
  validationResults = $bindable([]),
  onrowschange,
  onvalidationchange,
}: Props = $props();

const locale = getLocale();
const isRtl = locale === "ar";

// ── Dynamic Columns ────────────────────────────────────────────────
let config = $derived(getColumnsForType(type, labels));

// Add row indices for display
let indexedSource = $derived(addRowIndices(rows));

// ── Validation ─────────────────────────────────────────────────────
let validationTimeout: ReturnType<typeof setTimeout> | null = null;

function runValidation() {
  if (validationTimeout) clearTimeout(validationTimeout);
  validationTimeout = setTimeout(() => {
    const results = validateRows(type, rows);
    validationResults = results;

    // Apply errors to rows for cell styling
    for (let i = 0; i < rows.length; i++) {
      const result = results[i];
      if (result) {
        rows[i].__errors = result.errors;
      }
    }

    // Trigger reactivity
    rows = [...rows];
    onvalidationchange?.(results);
  }, 300);
}

// ── Cell Edit Handler ──────────────────────────────────────────────
function handleCellEdit(event: CellEditEvent) {
  const { rowIndex, prop, newValue } = event;
  if (rowIndex >= 0 && rowIndex < rows.length) {
    rows[rowIndex][prop] = newValue;

    // Auto-append rows when editing near the end
    if (rowIndex >= rows.length - 3) {
      appendEmptyRows(5);
    }

    runValidation();
    onrowschange?.(rows);
  }
}

// ── Paste Handler ──────────────────────────────────────────────────
function handlePaste(data: string[][], startRow: number, startCol: number) {
  // RevoGrid handles the paste internally, but we also need
  // to handle the raw clipboard event for full-row paste
}

// Listen for raw clipboard paste on the grid container
function handleClipboardPaste(e: ClipboardEvent) {
  const text = e.clipboardData?.getData("text/plain");
  if (!text || !text.includes("\t")) return; // Not tabular data

  e.preventDefault();
  e.stopPropagation();

  let pastedRows = parseExcelPaste(text, type);

  // Detect and skip header row
  if (pastedRows.length > 1 && detectHeaderRow(pastedRows[0], type)) {
    pastedRows = pastedRows.slice(1);
  }

  if (pastedRows.length === 0) return;

  // Find the first empty slot or append
  const dataProps = getDataProps(type);
  let insertAt = rows.findIndex((r) =>
    dataProps.every((p) => !String(r[p] ?? "").trim()),
  );
  if (insertAt === -1) insertAt = rows.length;

  // Merge pasted rows into existing rows array
  const newRows = [...rows];
  for (let i = 0; i < pastedRows.length; i++) {
    const targetIdx = insertAt + i;
    if (targetIdx < newRows.length) {
      // Overwrite existing empty row
      for (const prop of dataProps) {
        newRows[targetIdx][prop] = pastedRows[i][prop];
      }
      newRows[targetIdx].__errors = new Map<string, string>();
    } else {
      // Append new row
      const emptyRow = config.emptyRow();
      for (const prop of dataProps) {
        emptyRow[prop] = pastedRows[i][prop];
      }
      emptyRow.__rowIndex = targetIdx + 1;
      newRows.push(emptyRow);
    }
  }

  // Ensure there are always some empty rows at the bottom
  while (newRows.length < insertAt + pastedRows.length + 5) {
    const emptyRow = config.emptyRow();
    emptyRow.__rowIndex = newRows.length + 1;
    newRows.push(emptyRow);
  }

  rows = newRows;
  onrowschange?.(rows);
  runValidation();
}

// ── Helper: Append Empty Rows ──────────────────────────────────────
function appendEmptyRows(count: number) {
  const newRows = [...rows];
  for (let i = 0; i < count; i++) {
    const emptyRow = config.emptyRow();
    emptyRow.__rowIndex = newRows.length + 1;
    newRows.push(emptyRow);
  }
  rows = newRows;
}

// ── Initialize with empty rows ─────────────────────────────────────
$effect(() => {
  // When type changes, this effect re-runs
  const _ = type;
  if (rows.length === 0) {
    const initial: Record<string, unknown>[] = [];
    for (let i = 0; i < 20; i++) {
      const emptyRow = config.emptyRow();
      emptyRow.__rowIndex = i + 1;
      initial.push(emptyRow);
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
    columns={config.columns}
    source={indexedSource}
    height="calc(100vh - 360px)"
    rtl={isRtl}
    readonly={false}
    theme="compact"
    range={true}
    clipboard={true}
    oncelledit={handleCellEdit}
    onpaste={handlePaste}
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
