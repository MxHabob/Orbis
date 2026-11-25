<script lang="ts">
import { goto } from "$app/navigation";
import { getLocale, localizeHref } from "$lib/paraglide/runtime";
import { isTauri } from "@tauri-apps/api/core";
import { confirmStore } from "$lib/stores/confirm.svelte";
import { useUnsavedGuard } from "$lib/stores/unsaved-guard.svelte";
import { getActiveImportColumns, isImportColumnRequired } from "../engine/template-columns";
import * as m from "$lib/paraglide/messages";
import { createShowInFolderToastAction } from "$lib/shared/utils/download";
import { orgContext } from "$lib/stores/org-context.svelte";
import {
  ArrowDownTray,
  ArrowUpTray,
  CheckCircle,
  Clipboard,
  FolderOpen,
  Plus,
  Trash,
} from "@orbis/icons";
import {
  OrbisGrid,
  parseClipboard,
  type CellEditEvent,
  type GridRowsChangeEvent,
} from "@orbis/svelte-grid";
import { toast } from "@orbis/ui";
import { Button } from "@orbis/ui/button";
import { tick, untrack } from "svelte";
import { executeCommitPipeline } from "../engine/commit-pipeline";
import { exportCorrectedDataset } from "../engine/exporter";
import { parseDroppedFile, pickAndParseFile } from "../engine/file-parser";
import { revalidateCellChange } from "../engine/revalidator";
import { downloadExcelTemplate } from "../engine/template-generator";
import { validateAllRows } from "../engine/validator";
import {
  getBulkEntryGridColumns,
  getBulkEntryColumnHeader,
  getBulkEntryProfileName,
} from "../localization";
import { BULK_PROFILES, getBulkProfile } from "../profiles";
import { createBulkReferenceDataQuery } from "../queries";
import type {
  BulkCommitResult,
  BulkPreviewSummary,
  BulkProfileId,
  BulkRowState,
} from "../types";
import ImpactPreviewModal from "./impact-preview-modal.svelte";
import ImportResultSummary from "./import-result-summary.svelte";
import { operationStore } from "$lib/stores";

let {
  initialProfileId = "students",
  hideProfileSelector = false,
}: {
  initialProfileId?: BulkProfileId;
  hideProfileSelector?: boolean;
} = $props();

let selectedProfileId = $state<BulkProfileId>(untrack(() => initialProfileId));
let currentProfile = $derived(getBulkProfile(selectedProfileId));

// Declarative TanStack Query for Reference Data (Zero $effect data loading!)
const refDataQuery = createBulkReferenceDataQuery(
  () => selectedProfileId,
  () => orgContext.orgId || "",
  () => orgContext.baseCurrency || "",
);
const refData = $derived(refDataQuery.data ?? null);
const referenceDataError = $derived(
  refDataQuery.error instanceof Error ? refDataQuery.error.message : "",
);
const activeColumnMetadata = $derived(getActiveImportColumns(currentProfile, refData?.registrationFields));
const requiredFields = $derived(activeColumnMetadata.filter((column) =>
  isImportColumnRequired(selectedProfileId, column, refData?.registrationFields)));
const arabic = $derived(getLocale() === "ar");

let fileName = $state<string | null>(null);
let rawRows = $state<BulkRowState[]>([]);
let filterMode = $state<"all" | "errors" | "warnings" | "valid">("all");

let errorCount = $state(0);
let warningCount = $state(0);
let validCount = $state(0);

let isPreviewModalOpen = $state(false);
let previewSummary = $state<BulkPreviewSummary<any> | null>(null);
let commitResult = $state<BulkCommitResult | null>(null);

let isDraggingFile = $state(false);
let isReading = $state(false);
let isSaving = $state(false);
let isDownloading = $state(false);
let isExporting = $state(false);
let isReviewing = $state(false);
let browserFileInput: HTMLInputElement;
const isBusy = $derived(isReading || isSaving || isReviewing);
const ready = $derived(Boolean(refData) && !isBusy);
const guard = useUnsavedGuard({
  title: m.common_unsaved_changes(),
  description: m.bulk_entry_replace_draft(),
  discardText: m.bulk_entry_discard_draft(),
  stayText: m.common_action_cancel(),
});
$effect(() => guard.setDirty(rawRows.length > 0 && !commitResult));

function confirmReplaceDraft() {
  if (!rawRows.length || commitResult) return Promise.resolve(true);
  return confirmStore.prompt({
    title: m.common_unsaved_changes(),
    description: m.bulk_entry_replace_draft(),
    confirmText: m.bulk_entry_discard_draft(),
    cancelText: m.common_action_cancel(),
    variant: "destructive",
  });
}

// Switch profile handler (clears active staging rows without $effect)
async function handleProfileChange(newProfileId: BulkProfileId) {
  if (newProfileId === selectedProfileId || isBusy || !await confirmReplaceDraft()) return;
  handleReset();
  if (newProfileId === "grades" && orgContext.orgId) {
    await tick();
    await goto(localizeHref("/" + orgContext.orgId + "/settings/bulk-input/grades"));
    return;
  }
  selectedProfileId = newProfileId;
}

// Filtered rows for OrbisGrid display
const gridSource = $derived.by(() => {
  let filtered = rawRows;

  if (filterMode === "errors") {
    filtered = filtered.filter(
      (r) => r.status === "error" || r.status === "failed",
    );
  } else if (filterMode === "warnings") {
    filtered = filtered.filter((r) => r.status === "warning");
  } else if (filterMode === "valid") {
    filtered = filtered.filter(
      (r) => r.status === "valid" || r.status === "committed",
    );
  }

  return filtered.map((r) => r.data);
});

// Helper to create a single blank row
function createBlankRow(rowNumber: number): BulkRowState {
  const data: Record<string, any> = {
    __idx: rowNumber,
    rowNumber,
    id: `row-${crypto.randomUUID()}`,
  };
  for (const col of activeColumnMetadata) {
    data[col.id] = "";
  }
  return {
    rowNumber,
    data,
    status: "pending",
    errors: new Map(),
    warnings: new Map(),
    info: new Map(),
  };
}

// Add blank rows for direct visual entry
function handleAddBlankRows(count: number = 5) {
  if (!refData) {
    toast.warning(m.common_loading());
    return;
  }
  const startIndex = rawRows.length + 1;
  const newRows: BulkRowState[] = [];
  for (let i = 0; i < count; i++) {
    newRows.push(createBlankRow(startIndex + i));
  }

  const merged = [...rawRows, ...newRows];
  const summary = validateAllRows(merged, currentProfile, refData);
  rawRows = merged;
  errorCount = summary.errorCount;
  warningCount = summary.warningCount;
  validCount = summary.validCount;
}

// Paste tabular data from clipboard
async function handlePasteFromClipboard() {
  if (isBusy) return;
  if (!refData) {
    toast.warning(m.common_loading());
    return;
  }
  isReading = true;
  try {
    const text = await navigator.clipboard.readText();
    if (!text || !text.trim()) {
      toast.warning(m.bulk_entry_toast_clipboard_empty());
      return;
    }

    const matrix = parseClipboard(text);
    if (matrix.length === 0) return;

    const newRows: BulkRowState[] = [];
    const startIndex = rawRows.length + 1;

    for (let r = 0; r < matrix.length; r++) {
      const cells = matrix[r]!;
      const row = createBlankRow(startIndex + r);

      for (
        let c = 0;
        c < Math.min(cells.length, activeColumnMetadata.length);
        c++
      ) {
        const col = activeColumnMetadata[c]!;
        const cellVal = cells[c]?.trim() || "";
        row.data[col.id] = cellVal;
      }
      newRows.push(row);
    }

    const merged = [...rawRows, ...newRows];
    const summary = validateAllRows(merged, currentProfile, refData);
    rawRows = merged;
    errorCount = summary.errorCount;
    warningCount = summary.warningCount;
    validCount = summary.validCount;

    toast.success(m.bulk_entry_toast_pasted({ count: newRows.length }));
  } catch (err: any) {
    toast.error(
      m.bulk_entry_toast_paste_failed({ reason: err.message || String(err) }),
    );
  } finally {
    isReading = false;
  }
}

// Clear completely empty rows
function handleClearEmptyRows() {
  if (!refData || rawRows.length === 0) return;

  const cleaned = rawRows.filter((row) => {
    return activeColumnMetadata.some((col) => {
      const val = row.data[col.id];
      return val !== undefined && val !== null && String(val).trim() !== "";
    });
  });

  for (let i = 0; i < cleaned.length; i++) {
    cleaned[i]!.rowNumber = i + 1;
    cleaned[i]!.data.rowNumber = i + 1;
    cleaned[i]!.data.__idx = i + 1;
  }

  const summary = validateAllRows(cleaned, currentProfile, refData);
  rawRows = cleaned;
  errorCount = summary.errorCount;
  warningCount = summary.warningCount;
  validCount = summary.validCount;
}

// Handle template download
async function handleDownloadTemplate() {
  if (isDownloading) return;
  if (!refData) {
    toast.warning(m.common_loading());
    return;
  }
  isDownloading = true;
  try {
    const openFile = await downloadExcelTemplate({
      profile: currentProfile,
      refData,
      orgName: orgContext.displayName,
    });
    if (openFile === undefined) return;
    toast.success(m.bulk_entry_toast_template_downloaded(), {
      action: openFile ? createShowInFolderToastAction(openFile) : undefined,
    });
  } catch (err: any) {
    toast.error(
      m.bulk_entry_toast_template_download_failed({
        reason: err.message || String(err),
      }),
    );
  } finally {
    isDownloading = false;
  }
}

async function loadFile(read: () => ReturnType<typeof pickAndParseFile>) {
  if (!ready) return;
  isReading = true;
  try {
    if (!await confirmReplaceDraft()) return;
    const result = await read();
    if (result) processLoadedRows(result.fileName, result.rows);
  } catch (error) {
    toast.error(m.bulk_entry_toast_file_read_failed({
      reason: error instanceof Error ? error.message : String(error),
    }));
  } finally {
    isReading = false;
  }
}

function handlePickFile() {
  if (!ready) return;
  if (isTauri()) void loadFile(() => pickAndParseFile(currentProfile));
  else browserFileInput?.click();
}

function importFile(file: File | undefined) {
  if (!file || !ready) return;
  if (!/\.(xlsx|xls|csv)$/i.test(file.name)) {
    toast.error(m.bulk_entry_unsupported_file());
    return;
  }
  void loadFile(() => parseDroppedFile(file, currentProfile));
}

function handleDropFile(event: DragEvent) {
  event.preventDefault();
  isDraggingFile = false;
  importFile(event.dataTransfer?.files[0]);
}

function processLoadedRows(name: string, loadedRows: BulkRowState[]) {
  if (!refData) return;
  fileName = name;
  filterMode = "all";
  commitResult = null;

  const summary = validateAllRows(loadedRows, currentProfile, refData);
  rawRows = loadedRows;
  errorCount = summary.errorCount;
  warningCount = summary.warningCount;
  validCount = summary.validCount;

  toast.success(m.bulk_entry_file_loaded({ count: loadedRows.length }));
}

// Live cell edit handler from OrbisGrid
function handleCellEdit(e: CellEditEvent) {
  if (!refData) return;
  const rowId = String(e.row.id ?? "");
  const rowIndex = rawRows.findIndex(
    (row) => rowId && String(row.data.id ?? "") === rowId,
  );
  if (rowIndex < 0) return;
  const columnId = e.columnId;
  const newValue = e.newValue;

  const result = revalidateCellChange(
    rawRows,
    { rowIndex, columnId, newValue },
    currentProfile,
    refData,
  );

  rawRows = result.updatedRows;
  errorCount = result.errorCount;
  warningCount = result.warningCount;
  validCount = result.validCount;
}

function handleGridRowsChange(event: GridRowsChangeEvent) {
  if (!refData || event.action === "edit" || event.changedRows.length === 0)
    return;

  const changedIds = new Set(
    event.changedRows.map((row) => String(row.id ?? "")),
  );
  const updatedRows =
    event.action === "delete-rows"
      ? rawRows.filter((row) => !changedIds.has(String(row.data.id ?? "")))
      : rawRows.slice();
  if (event.action === "delete-rows") {
    const summary = validateAllRows(updatedRows, currentProfile, refData);
    rawRows = updatedRows;
    errorCount = summary.errorCount;
    warningCount = summary.warningCount;
    validCount = summary.validCount;
    return;
  }

  const rowIndexes: Record<string, number> = {};
  updatedRows.forEach((row, index) => {
    rowIndexes[String(row.data.id ?? "")] = index;
  });

  for (const changedData of event.changedRows) {
    const id = String(changedData.id ?? "");
    const existingIndex = rowIndexes[id];
    if (existingIndex !== undefined) {
      const existing = updatedRows[existingIndex]!;
      existing.data = { ...existing.data, ...changedData };
      continue;
    }

    const rowNumber = updatedRows.length + 1;
    const data = { ...changedData, __idx: rowNumber, rowNumber };
    const row: BulkRowState = {
      rowNumber,
      data,
      errors: new Map(),
      warnings: new Map(),
      info: new Map(),
      status: "pending",
    };
    rowIndexes[id] = updatedRows.length;
    updatedRows.push(row);
  }

  const summary = validateAllRows(updatedRows, currentProfile, refData);
  rawRows = updatedRows;
  errorCount = summary.errorCount;
  warningCount = summary.warningCount;
  validCount = summary.validCount;
}

// Jump to next error shortcut (F8)
function handleNextError() {
  const errIdx = rawRows.findIndex(
    (r) => r.status === "error" || r.status === "failed",
  );
  if (errIdx !== -1) {
    filterMode = "errors";
  } else {
    toast.success(m.bulk_entry_toast_no_errors());
  }
}

// Export corrected data
async function handleExportCorrected(format: "xlsx" | "csv" = "xlsx") {
  if (rawRows.length === 0 || isExporting || isBusy) return;
  isExporting = true;
  try {
    const openFile = await exportCorrectedDataset({
      profile: currentProfile,
      rows: rawRows,
      filenamePrefix: `Orbis-${selectedProfileId}-review`,
      format,
      registrationFields: refData?.registrationFields,
    });

    if (openFile === undefined) return;
    toast.success(m.bulk_entry_toast_data_exported(), {
      action: openFile ? createShowInFolderToastAction(openFile) : undefined,
    });
  } catch (err: any) {
    toast.error(
      m.bulk_entry_toast_export_failed({ reason: err.message || String(err) }),
    );
  } finally {
    isExporting = false;
  }
}

// Review impact before commit
async function handleReviewImpact() {
  if (!ready || !refData || rawRows.length === 0) return;
  isReviewing = true;
  try {
    const plainRows = rawRows.map((r) => r.data);
    previewSummary = await currentProfile.generatePreview(plainRows, refData);
    isPreviewModalOpen = true;
  } catch (err: any) {
    toast.error(
      m.bulk_entry_toast_preview_failed({ reason: err.message || String(err) }),
    );
  } finally {
    isReviewing = false;
  }
}

// Execute Commit
async function handleConfirmCommit() {
  if (!refData || rawRows.length === 0 || isSaving) return;
  isSaving = true;
  isPreviewModalOpen = false;
  const operation = operationStore.start(
    getBulkEntryProfileName(currentProfile.id),
    validCount,
  );

  try {
    const result = await executeCommitPipeline({
      rows: rawRows,
      profile: currentProfile,
      refData,
      orgId: orgContext.orgId || "",
      chunkSize:
        currentProfile.id === "students"
          ? Math.max(1, validCount)
          : currentProfile.id === "employees"
            ? 500
            : currentProfile.id === "grades"
              ? 500
              : 100,
      onProgress: (progress) =>
        operationStore.updateProgress(operation.id, progress),
    });

    operationStore.finish(operation.id, result);
    commitResult = result;

    if (result.failedCount === 0 && result.warnings.length === 0) {
      toast.success(
        m.bulk_entry_toast_committed({ count: result.committedCount }),
      );
    } else {
      toast.warning(
        m.bulk_entry_toast_commit_partial({
          count: result.failedCount + result.warnings.length,
        }),
      );
    }
  } catch (err: any) {
    operationStore.fail(operation.id);
    toast.error(
      m.bulk_entry_toast_save_failed({ reason: err.message || String(err) }),
    );
  } finally {
    isSaving = false;
  }
}

function handleReset() {
  filterMode = "all";
  rawRows = [];
  fileName = null;
  commitResult = null;
  errorCount = 0;
  warningCount = 0;
  validCount = 0;
}
</script>

<svelte:window
  onkeydown={(e) => {
    if (e.key === "F8") {
      e.preventDefault();
      handleNextError();
    }
  }}
/>

<div class="flex h-full min-h-0 flex-col bg-background p-4 text-foreground sm:p-6">
  <div class="flex min-h-0 flex-1 flex-col overflow-hidden">
    <header class="shrink-0 border-b border-border pb-4">
      <h1 class="text-xl font-semibold tracking-tight">{m.ui_page_bulk_entry()}</h1>
      {#if !hideProfileSelector}
        <nav class="mt-3 flex flex-wrap gap-2" aria-label={m.bulk_entry_workbook_template_type()}>
          {#each Object.keys(BULK_PROFILES) as id (id)}
            <Button variant={selectedProfileId === id ? "secondary" : "ghost"}
              size="sm" aria-pressed={selectedProfileId === id} disabled={isBusy}
              onclick={() => handleProfileChange(id as BulkProfileId)}>
              {getBulkEntryProfileName(id as BulkProfileId)}
            </Button>
          {/each}
        </nav>
      {/if}
    </header>
    <input type="file" hidden accept=".xlsx,.xls,.csv" bind:this={browserFileInput}
      onchange={(event) => {
        const input = event.currentTarget;
        const file = input.files?.[0];
        input.value = "";
        importFile(file);
      }} />

    {#if referenceDataError}
      <div class="mt-3 rounded-lg border border-destructive/30 bg-destructive/5 p-3 flex items-center justify-between gap-3" role="alert">
        <span class="text-xs text-destructive">{referenceDataError}</span>
        <Button variant="outline" size="sm" onclick={() => refDataQuery.refetch()}>
          {m.common_retry()}
        </Button>
      </div>
    {/if}

    {#if commitResult}
      <!-- Completion View -->
      <div class="flex-1 overflow-y-auto p-6 flex items-center justify-center">
        <ImportResultSummary
          result={commitResult}
          onreset={handleReset}
          onretryfailed={() => {
            commitResult = null;
            filterMode = "errors";
          }}
          onexportreport={() => handleExportCorrected()}
        />
      </div>
    {:else if rawRows.length === 0}
      <div class="min-h-0 flex-1 overflow-y-auto py-5">
        {#if selectedProfileId === "grades" && orgContext.orgId}
          <div class="mb-4 flex flex-wrap items-center justify-between gap-3 rounded-lg border border-border bg-muted/40 p-3">
            <p class="text-sm">{m.bulk_entry_grade_sheet_hint()}</p>
            <Button variant="outline" size="sm" href={localizeHref(`/${orgContext.orgId}/academics/grades`)}>{m.academics_nav_grade_entry()}</Button>
          </div>
        {/if}
        <div class="grid gap-4 lg:grid-cols-2">
          <section class="rounded-xl border border-border bg-card p-5">
            <ArrowDownTray size={22} class="mb-3 text-primary" />
            <h2 class="text-base font-semibold">{m.bulk_entry_download_template()}</h2>
            <p class="mt-1 text-sm leading-relaxed text-muted-foreground">{m.bulk_entry_template_help()}</p>
            {#if refData && requiredFields.length}
              <details class="my-4 rounded-lg border border-border px-3 py-2 text-sm">
                <summary class="cursor-pointer font-medium">{m.bulk_entry_required_fields({ count: requiredFields.length })}</summary>
                <ul class="mt-2 grid list-inside list-disc gap-1 text-muted-foreground sm:grid-cols-2">
                  {#each requiredFields as column (column.id)}
                    <li>{getBulkEntryColumnHeader(selectedProfileId, column.id, arabic ? column.headerAr : column.headerEn, false)}</li>
                  {/each}
                </ul>
              </details>
            {/if}
            <Button class="mt-3 gap-2" variant="outline" disabled={!ready || isDownloading} onclick={handleDownloadTemplate}>
              <ArrowDownTray size={15} /> {m.bulk_entry_download_template()}
            </Button>
          </section>
          <section
            class="rounded-xl border border-dashed bg-card p-5 transition-colors {isDraggingFile ? 'border-primary bg-primary/5' : 'border-border'}"
            ondragover={(event) => { event.preventDefault(); isDraggingFile = ready; }}
            ondragleave={() => (isDraggingFile = false)}
            ondrop={handleDropFile}
            aria-label={m.bulk_entry_file_drop_area()}
          >
            <FolderOpen size={22} class="mb-3 text-primary" />
            <h2 class="text-base font-semibold">{m.bulk_entry_import_file()}</h2>
            <p class="mt-1 text-sm leading-relaxed text-muted-foreground">{m.bulk_entry_import_help()}</p>
            <Button class="mt-4 gap-2" disabled={!ready} onclick={handlePickFile}>
              <FolderOpen size={15} /> {isReading ? m.common_loading() : m.bulk_entry_choose_file()}
            </Button>
            <div class="mt-4 flex flex-wrap gap-2 border-t border-border pt-3">
              <Button variant="ghost" size="sm" disabled={!ready} onclick={() => handleAddBlankRows(1)}>
                <Plus size={15} /> {m.bulk_entry_add_row()}
              </Button>
              <Button variant="ghost" size="sm" disabled={!ready} onclick={handlePasteFromClipboard}>
                <Clipboard size={15} /> {m.bulk_entry_paste_from_clipboard()}
              </Button>
            </div>
          </section>
        </div>
      </div>
    {:else}
      <div class="flex shrink-0 flex-wrap items-center justify-between gap-2 py-3">
        <div class="min-w-0" role="status">
          <h2 class="font-semibold">{m.bulk_entry_review_rows({ count: rawRows.length })}</h2>
          {#if fileName}<p class="truncate text-xs text-muted-foreground" title={fileName} dir="auto">{fileName}</p>{/if}
        </div>
        <div class="flex flex-wrap gap-1">
          <Button variant="ghost" size="sm" disabled={!ready} onclick={handlePickFile}>{m.bulk_entry_choose_file()}</Button>
          <Button variant="ghost" size="sm" disabled={!ready} onclick={() => handleAddBlankRows(1)}><Plus size={14} /> {m.bulk_entry_add_row()}</Button>
          <Button variant="ghost" size="sm" disabled={!ready} onclick={handlePasteFromClipboard}><Clipboard size={14} /> {m.bulk_entry_paste()}</Button>
        </div>
      </div>
      <div class="flex-1 min-h-0 min-w-0 w-full overflow-hidden bg-background relative">
        <OrbisGrid
          columns={getBulkEntryGridColumns(currentProfile, refData?.registrationFields).map((column) => ({
            ...column,
            ...(!column.readonly ? { type: "text" as const, options: undefined } : {}),
            sortable: false,
            sort: false,
          }))}
          source={gridSource}
          rtl={arabic}
          readonly={isBusy}
          height="100%"
          clientQuery
          range={true}
          clipboard={true}
          showAddRow={true}
          createEmptyRow={() => createBlankRow(rawRows.length + 1).data}
          oncelledit={handleCellEdit}
          onrowschange={handleGridRowsChange}
        />
      </div>

      <!-- Minimal Settings Footer -->
      <div class="flex flex-wrap items-center justify-between gap-3 border-t border-border/70 pt-3">
        <div class="flex min-w-0 flex-wrap items-center gap-1.5">
          <button
            type="button"
            onclick={() => (filterMode = "all")}
            class="whitespace-nowrap rounded-md px-2 py-1 text-[11px] font-semibold transition-all {filterMode === 'all' ? 'bg-foreground text-background font-bold' : 'text-muted-foreground hover:text-foreground'}"
          >
            {m.bulk_entry_filter_all({ count: rawRows.length })}
          </button>

          {#if errorCount > 0}
            <button
              type="button"
              onclick={() => (filterMode = "errors")}
              class="whitespace-nowrap rounded-md px-2 py-1 text-[11px] font-semibold transition-all {filterMode === 'errors' ? 'bg-destructive text-destructive-foreground' : 'bg-destructive/10 text-destructive'}"
            >
              {m.bulk_entry_filter_errors({ count: errorCount })}
            </button>
          {/if}

          {#if warningCount > 0}
            <button
              type="button"
              onclick={() => (filterMode = "warnings")}
              class="whitespace-nowrap rounded-md px-2 py-1 text-[11px] font-semibold transition-all {filterMode === 'warnings' ? 'bg-amber-500 text-white' : 'bg-amber-500/10 text-amber-600'}"
            >
              {m.bulk_entry_filter_warnings({ count: warningCount })}
            </button>
          {/if}

          <button
            type="button"
            onclick={() => (filterMode = "valid")}
            class="whitespace-nowrap rounded-md px-2 py-1 text-[11px] font-semibold transition-all {filterMode === 'valid' ? 'bg-emerald-600 text-white' : 'bg-emerald-500/10 text-emerald-600'}"
          >
            {m.bulk_entry_filter_valid({ count: Math.max(0, validCount - warningCount) })}
          </button>

          {#if errorCount > 0}
            <Button
              variant="ghost"
              size="sm"
              onclick={handleNextError}
              class="h-7 gap-1 px-2 text-[11px] text-destructive"
              title={m.bulk_entry_next_error_shortcut()}
            >
              <span>{m.bulk_entry_next_error()}</span>
              <span class="rounded bg-muted px-1 font-mono text-[9px]">F8</span>
            </Button>
          {/if}
          <Button
            variant="ghost"
            size="sm"
            disabled={isBusy}
            onclick={handleClearEmptyRows}
            class="h-7 px-2 text-[11px] text-muted-foreground hover:text-foreground"
            title={m.bulk_entry_clear_empty_rows()}
          >
            <Trash size={12} />
            <span>{m.bulk_entry_clear_empty()}</span>
          </Button>

          <Button
            variant="outline"
            size="sm"
            disabled={isBusy || isExporting}
            onclick={() => handleExportCorrected()}
            class="h-8 gap-1 text-xs"
          >
            <ArrowUpTray size={13} />
            <span>{m.bulk_entry_export_review()}</span>
          </Button>
        </div>

        <div class="flex items-center gap-2.5">
          {#if errorCount > 0}
            <span class="text-xs font-medium text-destructive">
              {m.bulk_entry_error_count({ count: errorCount })}
            </span>
          {/if}

          <Button
            variant="default"
            size="sm"
            disabled={!ready || validCount === 0}
            onclick={handleReviewImpact}
            class="h-8 gap-1.5 text-xs shadow-xs"
          >
            <CheckCircle size={14} />
            <span>{m.bulk_entry_preview_title()}</span>
          </Button>
        </div>
      </div>
    {/if}
  </div>
</div>

<!-- Impact Preview Modal -->
<ImpactPreviewModal
  open={isPreviewModalOpen}
  summary={previewSummary}
  onclose={() => (isPreviewModalOpen = false)}
  onconfirm={handleConfirmCommit}
/>
