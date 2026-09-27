<script lang="ts">
  import { OrbisGrid } from "@orbis/svelte-grid";
  import { toast } from "@orbis/ui";
  import * as m from "$lib/paraglide/messages";
  import { orgContext } from "$lib/stores/org-context.svelte";
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

  let selectedProfileId = $state<BulkProfileId>(initialProfileId);
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
      rowNumber,
      id: `row-${Date.now()}-${Math.random().toString(36).slice(2, 7)}`,
    };
    for (const field of currentProfile.fields) {
      data[field.key] = field.defaultValue ?? "";
    }
    return {
      rowNumber,
      data,
      status: "pending",
      errors: {},
      warnings: {},
    };
  }

  // Add blank rows for direct visual entry
  function handleAddBlankRows(count: number = 5) {
    if (!refData) {
      toast.warning(m.common_loading?.() || "جاري تحميل البيانات المرجعية...");
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

    toast.success(`تمت إضافة ${count} صفوف جديدة في الجدول`);
  }

  // Paste tabular data from clipboard
  async function handlePasteFromClipboard() {
    if (!refData) {
      toast.warning(m.common_loading?.() || "جاري تحميل البيانات المرجعية...");
      return;
    }
    try {
      const text = await navigator.clipboard.readText();
      if (!text || !text.trim()) {
        toast.warning("الحافظة فارغة أو لا تحتوي على نص قابل للصق");
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

        for (let c = 0; c < Math.min(cells.length, currentProfile.fields.length); c++) {
          const field = currentProfile.fields[c]!;
          const cellVal = cells[c]?.trim() || "";
          row.data[field.key] = cellVal;
        }
        newRows.push(row);
      }

      const merged = [...rawRows, ...newRows];
      const summary = validateAllRows(merged, currentProfile, refData);
      rawRows = merged;
      errorCount = summary.errorCount;
      warningCount = summary.warningCount;
      validCount = summary.validCount;

      toast.success(`تم لصق ${newRows.length} سجل بنجاح من الحافظة`);
    } catch (err: any) {
      toast.error(`تعذر اللصق من الحافظة: ${err.message || String(err)}`);
    }
  }

  // Clear completely empty rows
  function handleClearEmptyRows() {
    if (!refData || rawRows.length === 0) return;

    const cleaned = rawRows.filter((row) => {
      return currentProfile.fields.some((f) => {
        const val = row.data[f.key];
        return val !== undefined && val !== null && String(val).trim() !== "";
      });
    });

    for (let i = 0; i < cleaned.length; i++) {
      cleaned[i]!.rowNumber = i + 1;
      cleaned[i]!.data.rowNumber = i + 1;
    }

    const summary = validateAllRows(cleaned, currentProfile, refData);
    rawRows = cleaned;
    errorCount = summary.errorCount;
    warningCount = summary.warningCount;
    validCount = summary.validCount;

    toast.info(`تم تنظيف وحذف الصفوف الفارغة`);
  }

  // Handle template download
  async function handleDownloadTemplate() {
    if (!refData) {
      toast.warning(m.common_loading?.() || "جاري تحميل البيانات المرجعية...");
      return;
    }
    try {
      await downloadExcelTemplate({
        profile: currentProfile,
        refData,
        orgName: orgContext.displayName,
      });
      toast.success("تم تنزيل قالب Excel المعتمد بنجاح");
    } catch (err: any) {
      toast.error(`تعذر تنزيل القالب: ${err.message || String(err)}`);
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
      toast.error(`تعذر قراءة الملف: ${err.message || String(err)}`);
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
        toast.error(`تعذر قراءة الملف المسحوب: ${err.message || String(err)}`);
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

    toast.success(`تم استيراد ${loadedRows.length} سجل بنجاح إلى الجدول للمراجعة`);
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
      toast.info(`الانتقال للخطأ في الصف رقم ${errIdx + 1}`);
    } else {
      toast.success("لا توجد أخطاء متبقية في البيانات");
    }
  }

  // Export corrected data
  async function handleExportCorrected() {
    if (rawRows.length === 0) return;
    try {
      const savedPath = await exportCorrectedDataset({
        profile: currentProfile,
        rows: rawRows,
        filenamePrefix: `Orbis_${selectedProfileId}_Dataset`,
      });

      if (savedPath) {
        toast.success("تم تصدير ملف البيانات بنجاح");
        await openExportedFile(savedPath);
      }
    } catch (err: any) {
      toast.error(`تعذر تصدير الملف: ${err.message || String(err)}`);
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
      toast.error(`تعذر إنشاء معاينة الأثر: ${err.message || String(err)}`);
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
        toast.success(`تم اعتماد وقيد كافة السجلات بنجاح (${result.committedCount} سجل)`);
      } else {
        toast.warning(`اكتمل الاعتماد مع وجود ${result.failedCount} صفوف متعثرة`);
      }
    } catch (err: any) {
      isProgressModalOpen = false;
      toast.error(`حدث خطأ أثناء تنفيذ الحفظ: ${err.message || String(err)}`);
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

<div class="h-full flex flex-col bg-background text-foreground select-none" dir="rtl">
  <!-- Top Header / Profile Selector Bar -->
  <div class="px-6 py-4 border-b border-border bg-card/60 backdrop-blur-md flex items-center justify-between gap-4 flex-wrap">
    <div class="flex items-center gap-3">
      <div class="h-10 w-10 rounded-xl bg-primary/10 border border-primary/20 flex items-center justify-center text-primary text-lg">
        📥
      </div>
      <div>
        <h1 class="text-base font-bold text-foreground">استوديو الإدخال والاستيراد الجماعي</h1>
        <p class="text-xs text-muted-foreground mt-0.5">
          إدخال مباشر في الجدول أو استيراد ملفات Excel/CSV مع التحقق الفوري والاعتماد التلقائي
        </p>
      </div>
    </div>

    <!-- Profile Selector & Quick Actions -->
    <div class="flex items-center gap-2.5 flex-wrap">
      {#if !hideProfileSelector}
        <select
          value={selectedProfileId}
          onchange={(e) => handleProfileChange((e.target as HTMLSelectElement).value as BulkProfileId)}
          class="h-9 px-3 rounded-xl bg-muted/60 border border-border text-xs font-bold text-foreground focus:outline-none focus:ring-1 focus:ring-primary"
        >
          {#each Object.entries(BULK_PROFILES) as [id, prof]}
            <option value={id}>{prof.displayName.ar}</option>
          {/each}
        </select>
      {/if}

      <button
        onclick={() => handleAddBlankRows(5)}
        disabled={isLoadingRefData}
        class="h-9 px-3.5 rounded-xl bg-primary/10 border border-primary/20 text-primary hover:bg-primary/20 font-bold text-xs flex items-center gap-1.5 transition-colors disabled:opacity-50"
      >
        <span>➕</span>
        <span>إضافة 5 صفوف فارغة</span>
      </button>

      <button
        onclick={handlePasteFromClipboard}
        disabled={isLoadingRefData}
        class="h-9 px-3.5 rounded-xl bg-muted border border-border text-foreground hover:bg-muted/80 font-bold text-xs flex items-center gap-1.5 transition-colors disabled:opacity-50"
        title="لصق بيانات جدولية من Excel أو الحافظة"
      >
        <span>📋</span>
        <span>لصق من الحافظة</span>
      </button>

      <button
        onclick={handleDownloadTemplate}
        disabled={isLoadingRefData}
        class="h-9 px-3.5 rounded-xl bg-muted border border-border text-foreground hover:bg-muted/80 font-bold text-xs flex items-center gap-1.5 transition-colors disabled:opacity-50"
      >
        <span>⬇</span>
        <span>تنزيل نموذج Excel</span>
      </button>

      <button
        onclick={handlePickFile}
        disabled={isLoadingRefData}
        class="h-9 px-4 rounded-xl bg-primary text-primary-foreground hover:bg-primary/90 font-bold text-xs flex items-center gap-1.5 transition-all shadow-sm disabled:opacity-50"
      >
        <span>📂</span>
        <span>استيراد ملف</span>
      </button>
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
    <!-- Unified Dual-Action Starting Area (Direct Entry vs File Upload) -->
    <div class="flex-1 p-6 flex flex-col items-center justify-center max-w-5xl mx-auto w-full">
      <div class="text-center mb-8">
        <h2 class="text-xl font-bold text-foreground">اختر طريقة الإدخال الجماعي للبيانات</h2>
        <p class="text-xs text-muted-foreground mt-1 max-w-lg">
          يمكنك البدء مباشرة بالكتابة في الجدول التفاعلي، أو لصق البيانات من الحافظة، أو استيراد ملف Excel جاهز.
        </p>
      </div>

      <div class="grid grid-cols-1 md:grid-cols-2 gap-6 w-full">
        <!-- Direct Rapid Grid Entry Card -->
        <div class="rounded-3xl border border-border bg-card p-6 flex flex-col justify-between shadow-sm hover:border-primary/50 transition-all group">
          <div>
            <div class="h-12 w-12 rounded-2xl bg-primary/10 border border-primary/20 flex items-center justify-center text-2xl text-primary mb-4">
              ⌨️
            </div>
            <h3 class="text-base font-bold text-foreground">الإدخال المباشر في الجدول</h3>
            <p class="text-xs text-muted-foreground mt-1.5 leading-relaxed">
              افتح شبكة OrbisGrid التفاعلية وابدأ الكتابة فوراً أو الصق بيانات من جدول خارجي، مع تدقيق فوري للأخطاء وانتقال سهل بين الحقول عبر لوحة المفاتيح.
            </p>
          </div>

          <div class="mt-6 flex items-center gap-2.5 flex-wrap">
            <button
              onclick={() => handleAddBlankRows(10)}
              disabled={isLoadingRefData}
              class="px-4 py-2.5 rounded-xl bg-primary text-primary-foreground font-bold text-xs hover:bg-primary/90 shadow-sm transition-all flex items-center gap-1.5 disabled:opacity-50"
            >
              <span>➕</span>
              <span>بدء جدول بـ 10 صفوف</span>
            </button>

            <button
              onclick={handlePasteFromClipboard}
              disabled={isLoadingRefData}
              class="px-3.5 py-2.5 rounded-xl bg-muted border border-border text-foreground font-bold text-xs hover:bg-muted/80 transition-colors flex items-center gap-1.5 disabled:opacity-50"
            >
              <span>📋</span>
              <span>لصق من الحافظة</span>
            </button>
          </div>
        </div>

        <!-- Excel / CSV File Import Card -->
        <div
          class="rounded-3xl border-2 border-dashed border-border/80 bg-card/50 p-6 flex flex-col justify-between transition-all {isDraggingFile ? 'border-primary bg-primary/5' : 'hover:border-primary/40'}"
          ondragover={(e) => { e.preventDefault(); isDraggingFile = true; }}
          ondragleave={() => { isDraggingFile = false; }}
          ondrop={handleDropFile}
          role="region"
          aria-label="منطقة رفع الملفات"
        >
          <div>
            <div class="h-12 w-12 rounded-2xl bg-muted border border-border flex items-center justify-center text-2xl mb-4">
              📊
            </div>
            <h3 class="text-base font-bold text-foreground">استيراد ملف Excel أو CSV</h3>
            <p class="text-xs text-muted-foreground mt-1.5 leading-relaxed">
              قم بسحب وإفلات ملف Excel المعد مسبقاً أو تصفح الملفات على جهازك لمعاينة وفحص كافة السجلات في الجدول قبل الاعتماد.
            </p>
          </div>

          <div class="mt-6 flex items-center gap-2.5 flex-wrap">
            <button
              onclick={handlePickFile}
              disabled={isLoadingRefData}
              class="px-4 py-2.5 rounded-xl bg-card border border-border text-foreground font-bold text-xs hover:bg-muted shadow-sm transition-colors flex items-center gap-1.5 disabled:opacity-50"
            >
              <span>📂</span>
              <span>اختيار ملف</span>
            </button>

            <button
              onclick={handleDownloadTemplate}
              disabled={isLoadingRefData}
              class="px-3.5 py-2.5 rounded-xl bg-muted border border-border text-foreground font-bold text-xs hover:bg-muted/80 transition-colors flex items-center gap-1.5 disabled:opacity-50"
            >
              <span>⬇</span>
              <span>تنزيل النموذج</span>
            </button>
          </div>
        </div>
      </div>
    </div>
  {:else}
    <!-- Active Workspace Bar -->
    <div class="px-6 py-2.5 border-b border-border bg-muted/20 flex items-center justify-between gap-4 flex-wrap text-xs">
      <!-- File Metadata & Filter Pills -->
      <div class="flex items-center gap-2 flex-wrap">
        <span class="font-bold text-foreground ml-1">{fileName ? `الملف: ${fileName}` : "جدول الإدخال المباشر"}</span>
        <span class="text-muted-foreground">({rawRows.length} صف)</span>

        <div class="h-4 w-[1px] bg-border mx-1"></div>

        <!-- Filter Pills -->
        <button
          onclick={() => (filterMode = "all")}
          class="px-2.5 py-1 rounded-lg font-bold transition-all {filterMode === 'all' ? 'bg-foreground text-background' : 'bg-muted text-muted-foreground hover:text-foreground'}"
        >
          الكل ({rawRows.length})
        </button>

        <button
          onclick={() => (filterMode = "errors")}
          class="px-2.5 py-1 rounded-lg font-bold transition-all flex items-center gap-1 {filterMode === 'errors' ? 'bg-destructive text-destructive-foreground' : 'bg-destructive/10 text-destructive hover:bg-destructive/20'}"
        >
          <span>🔴 أخطاء ({errorCount})</span>
        </button>

        <button
          onclick={() => (filterMode = "warnings")}
          class="px-2.5 py-1 rounded-lg font-bold transition-all flex items-center gap-1 {filterMode === 'warnings' ? 'bg-amber-500 text-white' : 'bg-amber-500/10 text-amber-700 dark:text-amber-400 hover:bg-amber-500/20'}"
        >
          <span>🟡 تنبيهات ({warningCount})</span>
        </button>

        <button
          onclick={() => (filterMode = "valid")}
          class="px-2.5 py-1 rounded-lg font-bold transition-all flex items-center gap-1 {filterMode === 'valid' ? 'bg-emerald-600 text-white' : 'bg-emerald-500/10 text-emerald-700 dark:text-emerald-400 hover:bg-emerald-500/20'}"
        >
          <span>🟢 صالحة ({validCount})</span>
        </button>
      </div>

      <!-- Quick Actions, Search & Error Navigation -->
      <div class="flex items-center gap-2 flex-wrap">
        <button
          onclick={() => handleAddBlankRows(1)}
          class="h-8 px-2.5 rounded-lg bg-primary/10 text-primary border border-primary/20 font-bold text-xs hover:bg-primary/20 transition-colors"
        >
          + صف
        </button>

        <button
          onclick={() => handleAddBlankRows(5)}
          class="h-8 px-2.5 rounded-lg bg-primary/10 text-primary border border-primary/20 font-bold text-xs hover:bg-primary/20 transition-colors"
        >
          + 5 صفوف
        </button>

        <button
          onclick={handlePasteFromClipboard}
          class="h-8 px-2.5 rounded-lg bg-muted border border-border text-foreground hover:bg-muted/80 font-bold text-xs"
          title="لصق بيانات إضافية من الحافظة"
        >
          📋 لصق
        </button>

        <button
          onclick={handleClearEmptyRows}
          class="h-8 px-2.5 rounded-lg bg-muted border border-border text-foreground hover:bg-muted/80 font-bold text-xs"
          title="حذف الصفوف الفارغة بالكامل"
        >
          🧹 مسح الفارغ
        </button>

        <input
          type="text"
          placeholder="بحث سريع في الصفوف..."
          bind:value={searchQuery}
          class="h-8 w-44 px-3 rounded-lg bg-card border border-border text-xs text-foreground focus:outline-none focus:ring-1 focus:ring-primary"
        />

        <button
          onclick={handleNextError}
          class="h-8 px-3 rounded-lg bg-muted border border-border text-foreground hover:bg-muted/80 font-bold text-xs flex items-center gap-1"
          title="الانتقال للخطأ التالي (F8)"
        >
          <span>الخطأ التالي</span>
          <span class="px-1 py-0.2 rounded bg-background border border-border text-[10px] font-mono">F8</span>
        </button>
      </div>
    </div>

    <!-- Central OrbisGrid Workspace -->
    <div class="flex-1 w-full overflow-hidden bg-card relative">
      <OrbisGrid
        columns={currentProfile.gridColumns}
        source={gridSource}
        rtl={true}
        height="100%"
        range={true}
        clipboard={true}
        oncelledit={handleCellEdit}
      />
    </div>

    <!-- Action Footer -->
    <div class="px-6 py-3.5 border-t border-border bg-card/80 backdrop-blur-md flex items-center justify-between gap-4 flex-wrap">
      <div class="flex items-center gap-2">
        <button
          onclick={handleReset}
          class="px-3.5 py-2 rounded-xl border border-border text-xs font-bold text-muted-foreground hover:text-foreground hover:bg-muted transition-colors"
        >
          إفراغ الجدول
        </button>

        <button
          onclick={handleExportCorrected}
          class="px-3.5 py-2 rounded-xl bg-muted border border-border text-xs font-bold text-foreground hover:bg-muted/80 transition-colors flex items-center gap-1.5"
        >
          <span>📤</span>
          <span>تصدير البيانات</span>
        </button>
      </div>

      <div class="flex items-center gap-3">
        {#if errorCount > 0}
          <span class="text-xs font-bold text-destructive">
            ⚠ يوجد {errorCount} خطأ يجب تصحيحه قبل الاعتماد
          </span>
        {/if}

        <button
          onclick={handleReviewImpact}
          disabled={validCount === 0}
          class="px-6 py-2.5 rounded-xl bg-primary text-primary-foreground font-bold text-xs hover:bg-primary/90 disabled:opacity-50 disabled:pointer-events-none transition-all shadow-md flex items-center gap-2"
        >
          <span>معاينة الأثر واعتماد الحفظ ({validCount})</span>
          <span>◄</span>
        </button>
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
