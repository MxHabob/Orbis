<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { onDestroy } from "svelte";
import { api, extractErrorMessage } from "$lib/api";
import { academicContext } from "$lib/stores";
import { l } from "$lib/core/i18n";
import type { ReportDefinitionDto } from "$lib/api/types";
import { Button } from "@orbis/ui/button";
import { Card } from "@orbis/ui/card";
import { Skeleton } from "@orbis/ui/skeleton";
import { Tabs, TabsList, TabsTrigger } from "@orbis/ui/tabs";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui/select";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogDescription,
  DialogFooter,
} from "@orbis/ui/dialog";
import { toast } from "@orbis/ui";
import {
  ChartBar,
  DocumentChartBar,
  DocumentPage,
  Download,
} from "@orbis/icons";
import SearchField from "$lib/shared/components/search-field.svelte";
import { fade } from "svelte/transition";
import ReportDefinitionCard from "./components/report-definition-card.svelte";
import ReportParameterSheet from "./components/report-parameter-sheet.svelte";
import ReportTemplateBindingDialog from "./components/report-template-binding-dialog.svelte";
import MetricsView from "$lib/features/metrics/metrics-view.svelte";
import {
  REPORT_CATEGORIES,
  getFormatFilterLabel,
  matchesReportCategory,
} from "./report-catalog";
import {
  createReportDefinitionsQuery,
  createGenerateReportMutation,
} from "./queries";
import {
  createPeriodsQuery,
  createClassesQuery,
} from "$lib/features/academics/queries";

let { orgId, reportId }: { orgId: string; reportId?: string } = $props();

let activeMainTab = $state<"reports" | "analytics">("reports");
let selectedCategory = $state<string>("all");
let selectedFormatFilter = $state<string>("all");
let searchQuery = $state<string>("");

const defsQuery = createReportDefinitionsQuery(() => orgId);
const periodsQuery = createPeriodsQuery(() => orgId);
const classesQuery = createClassesQuery(() => orgId);
const generateMutation = createGenerateReportMutation();

const reportDefinitions = $derived(
  (defsQuery.data ?? []).filter(
    (report) => !reportId || report.id === reportId || report.code === reportId,
  ),
);
const periods = $derived(periodsQuery.data ?? []);
const classes = $derived(classesQuery.data ?? []);
const isLoading = $derived(
  defsQuery.isLoading || periodsQuery.isLoading || classesQuery.isLoading,
);

let selectedDefinition = $state<ReportDefinitionDto | null>(null);
let isSheetOpen = $state(false);
let selectedFormat = $state<"PDF" | "HTML">("PDF");
let selectedPeriodId = $state<string>("");
let selectedClassId = $state<string>("");
let dateFrom = $state<string>("");
let dateTo = $state<string>("");
let isGenerating = $state(false);
let jobStatus = $state<string>("");
let templateBindingDefinition = $state<ReportDefinitionDto | null>(null);
let isTemplateBindingOpen = $state(false);

let isPreviewOpen = $state(false);
let previewBlobUrl = $state<string>("");
let currentIsPdf = $state<boolean>(false);
let activePollTimer: ReturnType<typeof setInterval> | null = null;

function stopPolling() {
  if (activePollTimer) {
    clearInterval(activePollTimer);
    activePollTimer = null;
  }
}

onDestroy(() => {
  stopPolling();
  if (previewBlobUrl) {
    URL.revokeObjectURL(previewBlobUrl);
  }
});

const filteredDefinitions = $derived(
  reportDefinitions.filter((def) => {
    const matchesCat = matchesReportCategory(def, selectedCategory);
    const matchesFormat =
      selectedFormatFilter === "all" ||
      Boolean(def.supported_formats?.includes(selectedFormatFilter));

    const q = searchQuery.toLowerCase().trim();
    const matchesSearch =
      !q ||
      def.name.toLowerCase().includes(q) ||
      def.name_ar?.toLowerCase().includes(q) ||
      def.code.toLowerCase().includes(q) ||
      def.description?.toLowerCase().includes(q) ||
      def.description_ar?.toLowerCase().includes(q);

    return matchesCat && matchesFormat && matchesSearch;
  }),
);

function openGenerateSheet(def: ReportDefinitionDto) {
  selectedDefinition = def;
  selectedFormat = "PDF";
  isGenerating = false;
  jobStatus = "";
  if (!selectedPeriodId && periods.length > 0) {
    selectedPeriodId = academicContext.effectivePeriodId || periods[0].id;
  }
  isSheetOpen = true;
}

function openTemplateBinding(definition: ReportDefinitionDto) {
  templateBindingDefinition = definition;
  isTemplateBindingOpen = true;
}

async function handleExecuteReport() {
  if (!selectedDefinition) return;

  stopPolling();
  isGenerating = true;
  jobStatus = m.operations_ui_report_processing();

  try {
    const params: Record<string, unknown> = {
      period_id: selectedPeriodId || undefined,
      class_id: selectedClassId || undefined,
      date_from: dateFrom || undefined,
      date_to: dateTo || undefined,
      org_id: orgId,
    };

    const jobId = await generateMutation.mutateAsync({
      reportCode: selectedDefinition.code,
      format: selectedFormat,
      parameters: params,
    });

    let attempts = 0;
    activePollTimer = setInterval(async () => {
      attempts++;
      try {
        const status = await api.getReportJobStatus(jobId);
        if (status.status === "completed" || status.status === "ready") {
          stopPolling();
          isGenerating = false;
          jobStatus = m.operations_ui_report_completed();
          toast.success(m.operations_ui_report_generated_success());
          await handlePreviewReport(jobId);
        } else if (status.status === "failed") {
          stopPolling();
          isGenerating = false;
          jobStatus = m.operations_ui_report_failed();
          toast.error(
            status.error_message || m.operations_ui_report_create_error(),
          );
        }
      } catch {
        if (attempts > 15) {
          stopPolling();
          isGenerating = false;
          toast.error(m.operations_ui_report_create_timeout());
        }
      }
    }, 1000);
  } catch (e) {
    isGenerating = false;
    console.error("Failed to execute report:", e);
    toast.error(
      extractErrorMessage(e) || m.operations_ui_report_execution_error(),
    );
  }
}

async function handlePreviewReport(jobId: string) {
  try {
    const bytes = await api.downloadReportFile(jobId);
    const isPdf =
      bytes.length >= 4 &&
      bytes[0] === 0x25 &&
      bytes[1] === 0x50 &&
      bytes[2] === 0x44 &&
      bytes[3] === 0x46;
    currentIsPdf = isPdf;
    const mime = isPdf
      ? "application/pdf"
      : selectedFormat === "PDF" || selectedFormat === "HTML"
        ? "text/html;charset=utf-8"
        : "application/octet-stream";
    const blob = new Blob([new Uint8Array(bytes)], { type: mime });
    if (previewBlobUrl) {
      URL.revokeObjectURL(previewBlobUrl);
    }
    previewBlobUrl = URL.createObjectURL(blob);
    isPreviewOpen = true;
    isSheetOpen = false;
  } catch (e) {
    console.error("Failed to load report preview:", e);
    toast.error(m.operations_ui_report_preview_error());
  }
}

function handleDownloadCurrent() {
  if (!previewBlobUrl || !selectedDefinition) return;
  const link = document.createElement("a");
  link.href = previewBlobUrl;
  const ext = currentIsPdf
    ? "pdf"
    : selectedFormat === "PDF"
      ? "html"
      : selectedFormat.toLowerCase();
  link.download = `${selectedDefinition.code}_${new Date().toISOString().slice(0, 10)}.${ext}`;
  document.body.appendChild(link);
  link.click();
  document.body.removeChild(link);
}
</script>

<div class="h-full w-full overflow-y-auto bg-background/30 scrollbar-hide">
  <div class="flex w-full flex-col gap-6 p-5 md:p-8">
    <!-- Top Controls Toolbar -->
    <div
      class="flex flex-wrap items-center justify-between gap-4"
      in:fade={{ delay: 50, duration: 200 }}
    >
      <div class="flex flex-wrap items-center gap-3 shrink-0">
        <div class="w-full sm:w-72">
          <SearchField
            bind:value={searchQuery}
            placeholder={m.operations_ui_reports_search_placeholder()}
          />
        </div>

        <Select type="single" bind:value={selectedCategory}>
          <SelectTrigger class="h-9 w-auto min-w-[180px] text-xs bg-background border-border/60">
            {REPORT_CATEGORIES.find((c) => c.id === selectedCategory)?.label() ||
              m.operations_ui_report_category_all()}
          </SelectTrigger>
          <SelectContent>
            {#each REPORT_CATEGORIES as cat (cat.id)}
              <SelectItem value={cat.id}>{cat.label()}</SelectItem>
            {/each}
          </SelectContent>
        </Select>

        <Select type="single" bind:value={selectedFormatFilter}>
          <SelectTrigger class="h-9 text-xs bg-background border-border/60">
            {getFormatFilterLabel(selectedFormatFilter)}
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="all">{m.operations_ui_all_export_formats()}</SelectItem>
            <SelectItem value="PDF">{m.operations_ui_pdf_documents()}</SelectItem>
            <SelectItem value="XLSX">{m.operations_ui_xlsx_spreadsheets()}</SelectItem>
            <SelectItem value="CSV">{m.operations_ui_csv_files()}</SelectItem>
            <SelectItem value="HTML">{m.operations_ui_interactive_html()}</SelectItem>
          </SelectContent>
        </Select>
      </div>

      <div class="flex items-center gap-3 shrink-0">
        <Tabs
          value={activeMainTab}
          onValueChange={(v) => (activeMainTab = v as "reports" | "analytics")}
        >
          <TabsList class="h-9 rounded-lg border border-border/50 bg-muted/40 p-0.5">
            <TabsTrigger value="reports" class="gap-1.5 rounded-md px-3 py-1 text-xs font-semibold">
              <DocumentChartBar class="h-3.5 w-3.5" />
              {m.operations_ui_reports_directory()}
              {#if reportDefinitions.length > 0}
                <span class="ms-1 rounded-full bg-primary/15 px-1.5 py-0.2 font-mono text-[10px] font-bold text-primary">
                  {filteredDefinitions.length}
                </span>
              {/if}
            </TabsTrigger>
            <TabsTrigger value="analytics" class="gap-1.5 rounded-md px-3 py-1 text-xs font-semibold">
              <ChartBar class="h-3.5 w-3.5" />
              {m.operations_ui_metrics_analytics()}
            </TabsTrigger>
          </TabsList>
        </Tabs>
      </div>
    </div>

    {#if activeMainTab === "analytics"}
      <div class="w-full" in:fade={{ duration: 150 }}>
        <MetricsView {orgId} />
      </div>
    {:else}
      <div class="space-y-4" in:fade={{ duration: 150 }}>
        {#if isLoading}
          <div class="grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-3" aria-busy="true">
            {#each Array(6) as _, i (i)}
              <Card class="flex h-44 flex-col justify-between p-5">
                <div class="flex flex-col gap-3">
                  <div class="flex items-start justify-between">
                    <Skeleton class="size-9" />
                    <Skeleton class="h-3 w-16" />
                  </div>
                  <div class="flex flex-col gap-1.5">
                    <Skeleton class="h-4 w-2/3" />
                    <Skeleton class="h-3 w-full" />
                  </div>
                </div>
                <div class="flex items-center justify-between border-t border-border/50 pt-3">
                  <Skeleton class="h-7 w-16" />
                  <Skeleton class="h-7 w-24" />
                </div>
              </Card>
            {/each}
          </div>
        {:else if filteredDefinitions.length === 0}
          <Card class="flex flex-col items-center justify-center gap-2 border-dashed p-12 text-center text-muted-foreground">
            <DocumentPage class="h-8 w-8 opacity-40" />
            <div class="text-sm font-semibold text-foreground">
              {m.operations_ui_no_matching_reports()}
            </div>
            <div class="text-xs">{m.operations_ui_try_another_report_filter()}</div>
          </Card>
        {:else}
          <div class="grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-3">
            {#each filteredDefinitions as def (def.id)}
              <ReportDefinitionCard
                definition={def}
                onGenerate={openGenerateSheet}
                onManageTemplate={openTemplateBinding}
              />
            {/each}
          </div>
        {/if}
      </div>
    {/if}
  </div>
</div>

{#if selectedDefinition}
  <ReportParameterSheet
    bind:open={isSheetOpen}
    {selectedDefinition}
    {periods}
    {classes}
    bind:selectedFormat
    bind:selectedPeriodId
    bind:selectedClassId
    bind:dateFrom
    bind:dateTo
    {isGenerating}
    {jobStatus}
    onExecute={handleExecuteReport}
  />
{/if}

<ReportTemplateBindingDialog
  open={isTemplateBindingOpen}
  definition={templateBindingDefinition}
  {orgId}
  onOpenChange={(open) => (isTemplateBindingOpen = open)}
/>

<Dialog bind:open={isPreviewOpen}>
  <DialogContent class="flex h-[85vh] max-w-4xl flex-col p-6 text-start">
    <DialogHeader class="flex flex-row items-center justify-between border-b pb-3">
      <div>
        <DialogTitle class="text-base font-bold">
          {m.operations_ui_report_preview_named({
            name: l(selectedDefinition?.name_ar, selectedDefinition?.name) || "",
          })}
        </DialogTitle>
        <DialogDescription class="text-xs text-muted-foreground">
          {m.operations_ui_document_ready_print_download()}
        </DialogDescription>
      </div>

      <Button
        variant="outline"
        size="sm"
        onclick={handleDownloadCurrent}
        class="h-8 gap-1.5 text-xs font-semibold"
      >
        <Download class="h-3.5 w-3.5" />
        {m.operations_ui_download_file()}
      </Button>
    </DialogHeader>

    <div class="my-2 h-full min-h-0 w-full flex-1 overflow-hidden rounded-md border bg-muted/20">
      {#if previewBlobUrl}
        <iframe
          src={previewBlobUrl}
          title={m.operations_ui_report_preview()}
          class="h-full w-full border-0"
        ></iframe>
      {/if}
    </div>

    <DialogFooter class="flex justify-end border-t pt-2">
      <Button
        variant="ghost"
        size="sm"
        onclick={() => (isPreviewOpen = false)}
        class="h-8 text-xs font-medium"
      >
        {m.common_action_close()}
      </Button>
    </DialogFooter>
  </DialogContent>
</Dialog>
