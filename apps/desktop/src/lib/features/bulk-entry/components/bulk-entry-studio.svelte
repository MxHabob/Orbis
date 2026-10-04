<script lang="ts">
  import { untrack } from "svelte";
  import { OrbisGrid } from "@orbis/svelte-grid";
  import { Button } from "@orbis/ui/button";
  import { Badge } from "@orbis/ui/badge";
  import { toast } from "@orbis/ui";
  import {
    Plus,
    Search,
    Trash,
    Clipboard,
    ArrowUpTray,
    ArrowDownTray,
    CheckCircle,
    Users,
    BookOpen,
    FolderOpen,
  } from "@orbis/icons";
  import * as m from "$lib/paraglide/messages";
  import { orgContext } from "$lib/stores/org-context.svelte";
  import { getBulkEntryGridColumns, getBulkEntryProfileName } from "../localization";
  import type {
    BulkProfileId,
    BulkRowState,
    BulkPreviewSummary,
    ChunkProgress,
    BulkCommitResult,
  } from "../types";
  import { BULK_PROFILES, getBulkProfile } from "../profiles";
  import { createBulkReferenceDataQuery } from "../queries";
  import { downloadExcelTemplate } from "../engine/template-generator";
  import { pickAndParseFile, parseDroppedFile } from "../engine/file-parser";
  import { validateAllRows } from "../engine/validator";
  import { revalidateCellChange } from "../engine/revalidator";
  import { executeCommitPipeline } from "../engine/commit-pipeline";
  import { exportCorrectedDataset, openExportedFile } from "../engine/exporter";
  import ImpactPreviewModal from "./impact-preview-modal.svelte";
  import CommitProgressModal from "./commit-progress-modal.svelte";
  import ImportResultSummary from "./import-result-summary.svelte";

  let {
    initialProfileId = "students",
    hideProfileSelector = false,
  }: {
    initialProfileId?: BulkProfileId;
    hideProfileSelector?: boolean;
  } = $props();

  let selectedProfileId = $state<BulkProfileId>(
    untrack(() => initialProfileId),
  );
  let currentProfile = $derived(getBulkProfile(selectedProfileId));

  // Declarative TanStack Query for Reference Data (Zero $effect data loading!)
  const refDataQuery = createBulkReferenceDataQuery(
    () => selectedProfileId,
    () => orgContext.baseCurrency || "",
  );
  const refData = $derived(refDataQuery.data ?? null);
  const isLoadingRefData = $derived(refDataQuery.isLoading);

  let fileName = $state<string | null>(null);
  let rawRows = $state<BulkRowState[]>([]);
  let filterMode = $state<"all" | "errors" | "warnings" | "valid">("all");
  let searchQuery = $state("");

  let errorCount = $state(0);
  let warningCount = $state(0);
  let validCount = $state(0);

  let isPreviewModalOpen = $state(false);
  let isProgressModalOpen = $state(false);
  let previewSummary = $state<BulkPreviewSummary<any> | null>(null);
  let commitProgress = $state<ChunkProgress | null>(null);
  let commitResult = $state<BulkCommitResult | null>(null);

  let isDraggingFile = $state(false);

  // Switch profile handler (clears active staging rows without $effect)
  function handleProfileChange(newProfileId: BulkProfileId) {
    selectedProfileId = newProfileId;
    rawRows = [];
    fileName = null;
    commitResult = null;
    errorCount = 0;
    warningCount = 0;
    validCount = 0;
  }

  // Filtered rows for OrbisGrid display
  const gridSource = $derived.by(() => {
    let filtered = rawRows;

    if (filterMode === "errors") {
      filtered = filtered.filter((r) => r.status === "error" || r.status === "failed");
    } else if (filterMode === "warnings") {
      filtered = filtered.filter((r) => r.status === "warning");
    } else if (filterMode === "valid") {
      filtered = filtered.filter((r) => r.status === "valid" || r.status === "committed");
    }

    if (searchQuery.trim()) {
      const q = searchQuery.toLowerCase().trim();
      filtered = filtered.filter((r) =>
        Object.values(r.data).some((val) => String(val || "").toLowerCase().includes(q)),
      );
    }

    return filtered.map((r) => r.data);
  });

  // Helper to create a single blank row
  function createBlankRow(rowNumber: number): BulkRowState {
    const data: Record<string, any> = {
      __idx: rowNumber,
      rowNumber,
      id: `row-${Date.now()}-${Math.random().toString(36).slice(2, 7)}`,
    };
    for (const col of currentProfile.columnMetadata) {
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
    if (!refData) {
      toast.warning(m.common_loading());
      return;
    }
    try {
      const text = await navigator.clipboard.readText();
      if (!text || !text.trim()) {
        toast.warning(m.bulk_entry_toast_clipboard_empty());
        return;
      }

      const lines = text.trim().split(/\r?\n/);
      if (lines.length === 0) return;

      const newRows: BulkRowState[] = [];
      const startIndex = rawRows.length + 1;

      for (let r = 0; r < lines.length; r++) {
        const line = lines[r]!;
        const cells = line.includes("\t") ? line.split("\t") : line.split(",");
        const row = createBlankRow(startIndex + r);

        for (let c = 0; c < Math.min(cells.length, currentProfile.columnMetadata.length); c++) {
          const col = currentProfile.columnMetadata[c]!;
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
      toast.error(m.bulk_entry_toast_paste_failed({ reason: err.message || String(err) }));
    }
  }

  // Clear completely empty rows
  function handleClearEmptyRows() {
    if (!refData || rawRows.length === 0) return;

    const cleaned = rawRows.filter((row) => {
      return currentProfile.columnMetadata.some((col) => {
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
    if (!refData) {
      toast.warning(m.common_loading());
      return;
    }
    try {
      await downloadExcelTemplate({
        profile: currentProfile,
        refData,
        orgName: orgContext.displayName,
      });
      toast.success(m.bulk_entry_toast_template_downloaded());
    } catch (err: any) {
      toast.error(m.bulk_entry_toast_template_download_failed({ reason: err.message || String(err) }));
    }
  }

  // Handle native file selection
  async function handlePickFile() {
    try {
      const res = await pickAndParseFile(currentProfile);
      if (res) {
        processLoadedRows(res.fileName, res.rows);
      }
    } catch (err: any) {
      toast.error(m.bulk_entry_toast_file_read_failed({ reason: err.message || String(err) }));
    }
  }

  // Handle dropped file
  async function handleDropFile(e: DragEvent) {
    e.preventDefault();
    isDraggingFile = false;
    const file = e.dataTransfer?.files?.[0];
    if (file) {
      try {
        const res = await parseDroppedFile(file, currentProfile);
        processLoadedRows(res.fileName, res.rows);
      } catch (err: any) {
        toast.error(m.bulk_entry_toast_file_read_failed({ reason: err.message || String(err) }));
      }
    }
  }

  function processLoadedRows(name: string, loadedRows: BulkRowState[]) {
    if (!refData) return;
    fileName = name;
    commitResult = null;

    const summary = validateAllRows(loadedRows, currentProfile, refData);
    rawRows = loadedRows;
    errorCount = summary.errorCount;
    warningCount = summary.warningCount;
    validCount = summary.validCount;

    toast.success(m.bulk_entry_toast_imported({ count: loadedRows.length }));
  }

  // Live cell edit handler from OrbisGrid
  function handleCellEdit(e: any) {
    if (!refData) return;
    const rowIndex = e.rowIndex;
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

  // Jump to next error shortcut (F8)
  function handleNextError() {
    const errIdx = rawRows.findIndex((r) => r.status === "error" || r.status === "failed");
    if (errIdx !== -1) {
      filterMode = "errors";
    } else {
      toast.success(m.bulk_entry_toast_no_errors());
    }
  }

  // Export corrected data
  async function handleExportCorrected() {
    if (rawRows.length === 0) return;
    try {
      const savedPath = await exportCorrectedDataset({
        profile: currentProfile,
        rows: rawRows,
        filenamePrefix: `Orbis_${selectedProfileId}_Data`,
      });

      if (savedPath) {
        toast.success(m.bulk_entry_toast_data_exported());
        await openExportedFile(savedPath);
      }
    } catch (err: any) {
      toast.error(m.bulk_entry_toast_export_failed({ reason: err.message || String(err) }));
    }
  }

  // Review impact before commit
  async function handleReviewImpact() {
    if (!refData || rawRows.length === 0) return;
    try {
      const plainRows = rawRows.map((r) => r.data);
      previewSummary = await currentProfile.generatePreview(plainRows, refData);
      isPreviewModalOpen = true;
    } catch (err: any) {
      toast.error(m.bulk_entry_toast_preview_failed({ reason: err.message || String(err) }));
    }
  }

  // Execute Commit
  async function handleConfirmCommit() {
    if (!refData || rawRows.length === 0) return;
    isPreviewModalOpen = false;
    isProgressModalOpen = true;

    try {
      const result = await executeCommitPipeline({
        rows: rawRows,
        profile: currentProfile,
        refData,
        orgId: orgContext.orgId || "",
        chunkSize: 50,
        onProgress: (prog) => {
          commitProgress = prog;
        },
      });

      commitResult = result;
      isProgressModalOpen = false;

      if (result.failedCount === 0) {
        toast.success(m.bulk_entry_toast_committed({ count: result.committedCount }));
      } else {
        toast.warning(m.bulk_entry_toast_commit_partial({ count: result.failedCount }));
      }
    } catch (err: any) {
      isProgressModalOpen = false;
      toast.error(m.bulk_entry_toast_save_failed({ reason: err.message || String(err) }));
    }
  }

  function handleReset() {
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

<div class="h-full flex flex-col bg-background text-foreground" dir="rtl">
  <!-- Minimal Settings-Style Top Bar -->
  <div class="px-4 py-2.5 border-b border-border flex items-center justify-between gap-3 flex-wrap">
    <!-- Profile Selection / Mode Tabs -->
    <div class="flex items-center gap-1.5 flex-wrap">
      {#if !hideProfileSelector}
        <div class="flex items-center bg-muted/70 p-0.5 rounded-lg border border-border/60">
          {#each Object.entries(BULK_PROFILES) as [id, prof]}
            <button
              type="button"
              onclick={() => handleProfileChange(id as BulkProfileId)}
              class="px-3 py-1 rounded-md text-xs font-semibold transition-all {selectedProfileId === id ? 'bg-background text-foreground shadow-xs font-bold' : 'text-muted-foreground hover:text-foreground'}"
            >
              {getBulkEntryProfileName(id as BulkProfileId)}
            </button>
          {/each}
        </div>
      {/if}

      {#if rawRows.length > 0}
        <Badge variant="outline" class="text-xs px-2 py-0.5">
          {rawRows.length} {m.common_students_count_suffix()}
        </Badge>
      {/if}
    </div>

    <!-- Actions Toolbar -->
    <div class="flex items-center gap-1.5 flex-wrap">
      <Button
        variant="outline"
        size="sm"
        disabled={isLoadingRefData}
        onclick={() => handleAddBlankRows(1)}
        class="h-8 gap-1 text-xs"
      >
        <Plus size={14} />
        <span>{m.bulk_entry_add_row()}</span>
      </Button>

      <Button
        variant="outline"
        size="sm"
        disabled={isLoadingRefData}
        onclick={() => handleAddBlankRows(5)}
        class="h-8 gap-1 text-xs"
      >
        <span>+ 5</span>
      </Button>

      <Button
        variant="outline"
        size="sm"
        disabled={isLoadingRefData}
        onclick={handlePasteFromClipboard}
        class="h-8 gap-1 text-xs"
        title={m.bulk_entry_paste_from_clipboard()}
      >
        <Clipboard size={14} />
        <span>{m.bulk_entry_paste()}</span>
      </Button>

      <Button
        variant="ghost"
        size="sm"
        disabled={isLoadingRefData}
        onclick={handleDownloadTemplate}
        class="h-8 gap-1 text-xs text-muted-foreground hover:text-foreground"
      >
        <ArrowDownTray size={14} />
        <span>{m.bulk_entry_template()}</span>
      </Button>

      <Button
        variant="secondary"
        size="sm"
        disabled={isLoadingRefData}
        onclick={handlePickFile}
        class="h-8 gap-1 text-xs"
      >
        <FolderOpen size={14} />
        <span>{m.bulk_entry_import_file()}</span>
      </Button>
    </div>
  </div>

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
        onexportreport={handleExportCorrected}
      />
    </div>
  {:else if rawRows.length === 0}
    <!-- Minimal Settings-Style Empty Area -->
    <div
      class="flex-1 m-4 rounded-xl border border-dashed border-border/80 bg-muted/10 flex flex-col items-center justify-center p-6 text-center transition-all {isDraggingFile ? 'border-primary bg-primary/5' : ''}"
      ondragover={(e) => { e.preventDefault(); isDraggingFile = true; }}
      ondragleave={() => { isDraggingFile = false; }}
      ondrop={handleDropFile}
      role="region"
      aria-label={m.bulk_entry_file_drop_area()}
    >
      <div class="flex flex-col items-center max-w-sm">
        <div class="h-10 w-10 rounded-xl bg-muted border border-border flex items-center justify-center text-muted-foreground mb-3">
          <FolderOpen size={20} />
        </div>
        <p class="text-xs font-semibold text-foreground">{m.bulk_entry_empty_title()}</p>
        <p class="text-[11px] text-muted-foreground mt-0.5 mb-4">
          {m.bulk_entry_empty_description()}
        </p>

        <div class="flex items-center gap-2 flex-wrap justify-center">
          <Button
            variant="outline"
            size="sm"
            onclick={() => handleAddBlankRows(5)}
            disabled={isLoadingRefData}
            class="h-8 gap-1 text-xs"
          >
            <Plus size={14} />
            <span>{m.bulk_entry_add_five_rows()}</span>
          </Button>

          <Button
            variant="secondary"
            size="sm"
            onclick={handlePickFile}
            disabled={isLoadingRefData}
            class="h-8 gap-1 text-xs"
          >
            <FolderOpen size={14} />
            <span>{m.bulk_entry_choose_file()}</span>
          </Button>

          <Button
            variant="ghost"
            size="sm"
            onclick={handleDownloadTemplate}
            disabled={isLoadingRefData}
            class="h-8 gap-1 text-xs text-muted-foreground"
          >
            <ArrowDownTray size={14} />
            <span>{m.bulk_entry_download_template()}</span>
          </Button>
        </div>
      </div>
    </div>
  {:else}
    <!-- Sub-Filter Bar -->
    <div class="px-4 py-1.5 border-b border-border/80 bg-muted/15 flex items-center justify-between gap-2 flex-wrap text-xs">
      <div class="flex items-center gap-1.5">
        <button
          type="button"
          onclick={() => (filterMode = "all")}
          class="px-2 py-0.5 rounded text-[11px] font-semibold transition-all {filterMode === 'all' ? 'bg-foreground text-background font-bold' : 'text-muted-foreground hover:text-foreground'}"
        >
          {m.bulk_entry_filter_all({ count: rawRows.length })}
        </button>

        {#if errorCount > 0}
          <button
            type="button"
            onclick={() => (filterMode = "errors")}
            class="px-2 py-0.5 rounded text-[11px] font-semibold transition-all {filterMode === 'errors' ? 'bg-destructive text-destructive-foreground' : 'bg-destructive/10 text-destructive'}"
          >
            {m.bulk_entry_filter_errors({ count: errorCount })}
          </button>
        {/if}

        {#if warningCount > 0}
          <button
            type="button"
            onclick={() => (filterMode = "warnings")}
            class="px-2 py-0.5 rounded text-[11px] font-semibold transition-all {filterMode === 'warnings' ? 'bg-amber-500 text-white' : 'bg-amber-500/10 text-amber-600'}"
          >
            {m.bulk_entry_filter_warnings({ count: warningCount })}
          </button>
        {/if}

        <button
          type="button"
          onclick={() => (filterMode = "valid")}
          class="px-2 py-0.5 rounded text-[11px] font-semibold transition-all {filterMode === 'valid' ? 'bg-emerald-600 text-white' : 'bg-emerald-500/10 text-emerald-600'}"
        >
          {m.bulk_entry_filter_valid({ count: validCount })}
        </button>
      </div>

      <div class="flex items-center gap-1.5">
        <div class="relative">
          <input
            type="text"
            placeholder={m.common_search_placeholder()}
            bind:value={searchQuery}
            class="h-7 w-36 px-2.5 text-xs rounded bg-background border border-border text-foreground focus:outline-none focus:ring-1 focus:ring-primary"
          />
        </div>

        {#if errorCount > 0}
          <Button
            variant="ghost"
            size="sm"
            onclick={handleNextError}
            class="h-7 px-2 text-[11px] text-destructive gap-1"
            title={m.bulk_entry_next_error_shortcut()}
          >
            <span>{m.bulk_entry_next_error()}</span>
            <span class="font-mono text-[9px] px-1 rounded bg-muted">F8</span>
          </Button>
        {/if}

        <Button
          variant="ghost"
          size="sm"
          onclick={handleClearEmptyRows}
          class="h-7 px-2 text-[11px] text-muted-foreground hover:text-foreground"
          title={m.bulk_entry_clear_empty_rows()}
        >
          <Trash size={12} />
          <span>{m.bulk_entry_clear_empty()}</span>
        </Button>
      </div>
    </div>

    <!-- Central OrbisGrid Workspace -->
    <div class="flex-1 w-full overflow-hidden bg-background relative">
      <OrbisGrid
        columns={getBulkEntryGridColumns(currentProfile)}
        source={gridSource}
        rtl={true}
        height="100%"
        range={true}
        clipboard={true}
        oncelledit={handleCellEdit}
      />
    </div>

    <!-- Minimal Settings Footer -->
    <div class="px-4 py-2 border-t border-border bg-card/50 flex items-center justify-between gap-3">
      <div class="flex items-center gap-1.5">
        <Button
          variant="ghost"
          size="sm"
          onclick={handleReset}
          class="h-8 text-xs text-muted-foreground hover:text-foreground"
        >
          {m.common_action_clear()}
        </Button>

        <Button
          variant="outline"
          size="sm"
          onclick={handleExportCorrected}
          class="h-8 text-xs gap-1"
        >
          <ArrowUpTray size={13} />
          <span>{m.bulk_entry_export()}</span>
        </Button>
      </div>

      <div class="flex items-center gap-2.5">
        {#if errorCount > 0}
          <span class="text-xs text-destructive font-medium">
            {m.bulk_entry_error_count({ count: errorCount })}
          </span>
        {/if}

        <Button
          variant="primary"
          size="sm"
          disabled={validCount === 0}
          onclick={handleReviewImpact}
          class="h-8 text-xs gap-1.5 shadow-xs"
        >
          <CheckCircle size={14} />
          <span>{m.bulk_entry_commit_and_save({ count: validCount })}</span>
        </Button>
      </div>
    </div>
  {/if}
</div>

<!-- Impact Preview Modal -->
<ImpactPreviewModal
  open={isPreviewModalOpen}
  profileId={selectedProfileId}
  summary={previewSummary}
  onclose={() => (isPreviewModalOpen = false)}
  onconfirm={handleConfirmCommit}
/>

<!-- Commit Progress Modal -->
<CommitProgressModal
  open={isProgressModalOpen}
  progress={commitProgress}
/>
