<!--
@component BulkInputGrid
Bulk input matrix powered directly by @orbis/svelte-grid.
Eliminates duplicate paste event wrappers and leverages OrbisGrid's native
clipboard, range selection, and cell validation lifecycle.
-->
<script lang="ts">
import { OrbisGrid } from "@orbis/svelte-grid";
import type { CellEditEvent, BulkResult } from "@orbis/svelte-grid";
import { getLocale } from "$lib/paraglide/runtime";
import { type BulkInputType, getColumnsForType } from "./config";
import { validateRows, type RowValidationResult } from "./validator";

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

const isRtl = $derived(getLocale() === "ar");

let config = $derived(getColumnsForType(type, labels));

// ── Validation ─────────────────────────────────────────────────────
let validationTimeout: ReturnType<typeof setTimeout> | null = null;

function runValidation() {
  if (validationTimeout) clearTimeout(validationTimeout);
  validationTimeout = setTimeout(() => {
    const results = validateRows(type, rows);
    validationResults = results;
    onvalidationchange?.(results);

    // Apply errors to rows for cell styling
    for (let i = 0; i < rows.length; i++) {
      const result = results[i];
      if (result) {
        rows[i].__errors = result.errors;
      }
    }
  }, 200);
}

// ── Cell Edit Handler ──────────────────────────────────────────────
function handleCellEdit(event: CellEditEvent) {
  const { rowIndex, columnId: prop, newValue } = event;
  if (rowIndex < 0 || rowIndex >= rows.length) return;

  rows[rowIndex][prop] = newValue;

  // Append empty row if editing near the end
  if (rowIndex >= rows.length - 3) {
    const newRows = [...rows];
    for (let i = 0; i < 5; i++) {
      newRows.push(config.emptyRow());
    }
    rows = newRows;
  } else {
    rows = [...rows];
  }

  runValidation();
  onrowschange?.(rows);
}

function handlePasteResult(_result: BulkResult) {
  rows = [...rows];
  runValidation();
  onrowschange?.(rows);
}

// ── Initialize with empty rows ─────────────────────────────────────
$effect(() => {
  const _ = type;
  if (rows.length === 0) {
    const initial: Record<string, unknown>[] = [];
    for (let i = 0; i < 20; i++) {
      initial.push(config.emptyRow());
    }
    rows = initial;
    onrowschange?.(rows);
  }
});
</script>

<div class="bulk-grid-wrapper flex-grow min-h-0 h-full w-full">
  <OrbisGrid
    columns={config.columns}
    bind:source={rows}
    height="100%"
    rtl={isRtl}
    readonly={false}
    clipboard={true}
    range={true}
    createEmptyRow={() => config.emptyRow()}
    oncelledit={handleCellEdit}
    onpasteResult={handlePasteResult}
  />
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
