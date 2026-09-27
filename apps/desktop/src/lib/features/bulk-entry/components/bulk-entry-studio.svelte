<script lang="ts">
  import { onMount } from "svelte";
  import { OrbisGrid } from "@orbis/svelte-grid";
  import { toast } from "@orbis/ui";
  import { orgContext } from "$lib/stores/org-context.svelte";
  import type {
    BulkProfileId,
    BulkReferenceData,
    BulkRowState,
    BulkPreviewSummary,
    ChunkProgress,
    BulkCommitResult,
  } from "../types";
  import { BULK_PROFILES, getBulkProfile } from "../profiles";
  import { downloadExcelTemplate } from "../engine/template-generator";
  import { pickAndParseFile, parseDroppedFile } from "../engine/file-parser";
  import { validateAllRows } from "../engine/validator";
  import { revalidateCellChange } from "../engine/revalidator";
  import { executeCommitPipeline } from "../engine/commit-pipeline";
  import { exportCorrectedDataset, openExportedFile } from "../engine/exporter";
  import ImpactPreviewModal from "./impact-preview-modal.svelte";
  import CommitProgressModal from "./commit-progress-modal.svelte";
  import ImportResultSummary from "./import-result-summary.svelte";

  let selectedProfileId = $state<BulkProfileId>("students");
  let currentProfile = $derived(getBulkProfile(selectedProfileId));

  let refData = $state<BulkReferenceData | null>(null);
  let isLoadingRefData = $state(false);

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

  // Load reference data on profile switch
  async function loadReferenceData() {
    isLoadingRefData = true;
    try {
      refData = await currentProfile.buildReferenceData(orgContext.baseCurrency|| "");
    } catch (err) {
      console.error("Failed to load reference data:", err);
      toast.error("تعذر تحميل البيانات المرجعية للمؤسسة");
    } finally {
      isLoadingRefData = false;
    }
  }

  $effect(() => {
    if (selectedProfileId) {
      loadReferenceData();
      rawRows = [];
      fileName = null;
      commitResult = null;
    }
  });

  onMount(() => {
    loadReferenceData();
  });

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

  // Handle template download
  async function handleDownloadTemplate() {
    if (!refData) return;
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
        filenamePrefix: `Orbis_${selectedProfileId}_Corrected`,
      });

      if (savedPath) {
        toast.success("تم تصدير ملف البيانات المصححة بنجاح");
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
        <p class="text-xs text-muted-foreground mt-0.5">معاينة وتدقيق وتصحيح ملفات Excel داخل OrbisGrid قبل الاعتماد النهائي</p>
      </div>
    </div>

    <!-- Profile Selector & Template Download -->
    <div class="flex items-center gap-2.5">
      <select
        bind:value={selectedProfileId}
        class="h-9 px-3 rounded-xl bg-muted/60 border border-border text-xs font-bold text-foreground focus:outline-none focus:ring-1 focus:ring-primary"
      >
        {#each Object.entries(BULK_PROFILES) as [id, prof]}
          <option value={id}>{prof.displayName.ar}</option>
        {/each}
      </select>

      <button
        onclick={handleDownloadTemplate}
        disabled={isLoadingRefData}
        class="h-9 px-3.5 rounded-xl bg-muted border border-border text-foreground hover:bg-muted/80 font-bold text-xs flex items-center gap-1.5 transition-colors"
      >
        <span>⬇</span>
        <span>تنزيل قالب Excel المعتمد</span>
      </button>

      <button
        onclick={handlePickFile}
        disabled={isLoadingRefData}
        class="h-9 px-4 rounded-xl bg-primary text-primary-foreground hover:bg-primary/90 font-bold text-xs flex items-center gap-1.5 transition-all shadow-sm"
      >
        <span>📂</span>
        <span>اختيار ملف واستيراده</span>
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
    <!-- Empty State / Drag and Drop Area -->
    <div
      class="flex-1 m-6 rounded-3xl border-2 border-dashed border-border/80 bg-card/40 flex flex-col items-center justify-center p-8 text-center transition-all {isDraggingFile ? 'border-primary bg-primary/5' : ''}"
      ondragover={(e) => { e.preventDefault(); isDraggingFile = true; }}
      ondragleave={() => { isDraggingFile = false; }}
      ondrop={handleDropFile}
      role="region"
      aria-label="منطقة سحب وإفلات الملفات"
    >
      <div class="h-20 w-20 rounded-3xl bg-muted/60 border border-border flex items-center justify-center text-4xl shadow-inner mb-4">
        📊
      </div>
      <h2 class="text-lg font-bold text-foreground">قم بسحب وإفلات ملف Excel هنا أو اضغط للاختيار</h2>
      <p class="text-xs text-muted-foreground max-w-md mt-1.5 leading-relaxed">
        يدعم النظام ملفات <span class="font-bold text-foreground">.xlsx</span> و <span class="font-bold text-foreground">.csv</span>. لن يتم إدخال أي سجل في قاعدة البيانات مباشرة حتى تراجع البيانات وتعتمدها بالكامل.
      </p>

      <div class="mt-6 flex items-center gap-3">
        <button
          onclick={handlePickFile}
          class="px-5 py-2.5 rounded-xl bg-primary text-primary-foreground font-bold text-xs hover:bg-primary/90 shadow-md transition-all flex items-center gap-2"
        >
          <span>📂</span>
          <span>تصفح الملفات على جهازك</span>
        </button>

        <button
          onclick={handleDownloadTemplate}
          class="px-4 py-2.5 rounded-xl bg-card border border-border text-foreground font-bold text-xs hover:bg-muted transition-colors flex items-center gap-1.5"
        >
          <span>⬇</span>
          <span>تحميل نموذج فارغ</span>
        </button>
      </div>
    </div>
  {:else}
    <!-- Active Workspace Bar -->
    <div class="px-6 py-2.5 border-b border-border bg-muted/20 flex items-center justify-between gap-4 flex-wrap text-xs">
      <!-- File Metadata & Filter Pills -->
      <div class="flex items-center gap-2">
        <span class="font-bold text-foreground ml-1">الملف: {fileName || "ملف البيانات"}</span>
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

      <!-- Quick Search & Error Navigation -->
      <div class="flex items-center gap-2">
        <input
          type="text"
          placeholder="بحث سريع في الصفوف..."
          bind:value={searchQuery}
          class="h-8 w-48 px-3 rounded-lg bg-card border border-border text-xs text-foreground focus:outline-none focus:ring-1 focus:ring-primary"
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
    <div class="px-6 py-3.5 border-t border-border bg-card/80 backdrop-blur-md flex items-center justify-between gap-4">
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
          <span>تصدير الملف المصحح</span>
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
