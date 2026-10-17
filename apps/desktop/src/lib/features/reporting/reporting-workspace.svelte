<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { api, extractErrorMessage } from "$lib/api";
import { academicContext } from "$lib/stores";
import { isAr } from "$lib/core/i18n";
import type { ReportDefinitionDto } from "$lib/api/types";
import { Button } from "@orbis/ui/button";
import { Badge } from "@orbis/ui/badge";
import { Card } from "@orbis/ui/card";
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
  AdjustmentsHorizontal,
  Banknotes,
  BookOpen,
  Calendar,
  ChartBar,
  DocumentChartBar,
  DocumentPage,
  Download,
  Eye,
  Play,
  Refresh,
  Users,
} from "@orbis/icons";
import SearchField from "$lib/shared/components/search-field.svelte";
import { fade } from "svelte/transition";
import ReportParameterSheet from "./components/report-parameter-sheet.svelte";

import MetricsView from "$lib/features/metrics/metrics-view.svelte";
import { createReportDefinitionsQuery, createGenerateReportMutation } from "./queries";
import { createPeriodsQuery, createClassesQuery } from "$lib/features/academics/queries";

let { orgId }: { orgId: string } = $props();

let activeMainTab = $state<"reports" | "analytics">("reports");
let selectedCategory = $state<string>("all");
let selectedFormatFilter = $state<string>("all");
let searchQuery = $state<string>("");

// TanStack Query layer for deterministic caching and lifecycle
const defsQuery = createReportDefinitionsQuery(() => orgId);
const periodsQuery = createPeriodsQuery(() => orgId);
const classesQuery = createClassesQuery(() => orgId);
const generateMutation = createGenerateReportMutation();

const reportDefinitions = $derived(defsQuery.data ?? []);
const periods = $derived(periodsQuery.data ?? []);
const classes = $derived(classesQuery.data ?? []);
const isLoading = $derived(defsQuery.isLoading || periodsQuery.isLoading || classesQuery.isLoading);

// Report Generation Execution State
let selectedDefinition = $state<ReportDefinitionDto | null>(null);
let isSheetOpen = $state(false);
let selectedFormat = $state<"PDF" | "HTML">("PDF");
let selectedPeriodId = $state<string>("");
let selectedClassId = $state<string>("");
let dateFrom = $state<string>("");
let dateTo = $state<string>("");
let isGenerating = $state(false);
let jobStatus = $state<string>("");
let currentJobId = $state<string>("");

// Preview State
let isPreviewOpen = $state(false);
let previewBlobUrl = $state<string>("");
let currentIsPdf = $state<boolean>(false);

const categoryOptions = [
  { id: "all", label: m.operations_ui_report_category_all() },
  { id: "academic", label: m.operations_ui_report_category_academic() },
  { id: "student", label: m.operations_ui_report_category_student() },
  { id: "attendance", label: m.operations_ui_report_category_attendance() },
  { id: "exam", label: m.operations_ui_report_category_exams() },
  { id: "finance", label: m.operations_ui_report_category_finance() },
  { id: "hr", label: m.operations_ui_report_category_hr() },
  { id: "administration", label: m.operations_ui_report_category_administration() },
];

function getFormatFilterLabel(format: string): string {
  switch (format) {
    case "all": return m.operations_ui_all_export_formats();
    case "PDF": return m.operations_ui_pdf_documents();
    case "XLSX": return m.operations_ui_xlsx_spreadsheets();
    case "CSV": return m.operations_ui_csv_files();
    case "HTML": return m.operations_ui_interactive_html();
    default: return m.operations_ui_export_format();
  }
}

function getDomainIcon(domain?: string, category?: string) {
  const d = (domain || "").toLowerCase();
  const c = (category || "").toLowerCase();
  if (d === "finance" || c.includes("billing") || c.includes("accounting") || c.includes("cash")) return Banknotes;
  if (d === "academic" || c.includes("grades") || c.includes("transcript") || c.includes("curriculum")) return BookOpen;
  if (d === "students" || d === "student" || c.includes("admission") || c.includes("guardians")) return Users;
  if (d === "attendance" || c.includes("daily") || c.includes("monthly") || c.includes("absence")) return Calendar;
  if (d.includes("exam") || c.includes("seating") || c.includes("control")) return DocumentPage;
  if (d.includes("employee") || d.includes("hr") || c.includes("staff") || c.includes("payroll")) return Users;
  return AdjustmentsHorizontal;
}

const filteredDefinitions = $derived(
  reportDefinitions.filter((def) => {
    const domain = (def.domain || "").toLowerCase();
    const category = (def.category || "").toLowerCase();
    const selected = selectedCategory.toLowerCase();

    const matchesCat =
      selected === "all" ||
      domain === selected ||
      category === selected ||
      (selected === "student" && (domain === "students" || category === "students" || category === "admissions")) ||
      (selected === "exam" && (domain === "exams" || domain === "examinations" || category === "examinations" || category === "seating" || category === "control")) ||
      (selected === "academic" && (category === "grades" || category === "transcripts" || category === "scheduling" || category === "curriculum")) ||
      (selected === "finance" && (category === "accounting" || category === "billing" || category === "cash" || category === "discounts" || category === "tax")) ||
      (selected === "attendance" && (category === "daily" || category === "monthly" || category === "alerts" || category === "behavior")) ||
      (selected === "hr" && (domain === "employees" || category === "employees" || category === "staff" || category === "payroll" || category === "workload" || category === "contracts")) ||
      (selected === "administration" && (domain === "system" || category === "facilities" || category === "security" || category === "autopilot"));

    const matchesFormat =
      selectedFormatFilter === "all" ||
      (def.supported_formats && def.supported_formats.includes(selectedFormatFilter));

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
  currentJobId = "";
  if (!selectedPeriodId && periods.length > 0) {
    selectedPeriodId = academicContext.effectivePeriodId || periods[0].id;
  }
  isSheetOpen = true;
}

async function handleExecuteReport() {
  if (!selectedDefinition) return;

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
    currentJobId = jobId;

    // Poll job status
    let attempts = 0;
    const pollInterval = setInterval(async () => {
      attempts++;
      try {
        const status = await api.getReportJobStatus(jobId);
        if (status.status === "completed" || status.status === "ready") {
          clearInterval(pollInterval);
          isGenerating = false;
          jobStatus = m.operations_ui_report_completed();
          toast.success(m.operations_ui_report_generated_success());
          await handlePreviewReport(jobId);
        } else if (status.status === "failed") {
          clearInterval(pollInterval);
          isGenerating = false;
          jobStatus = m.operations_ui_report_failed();
          toast.error(status.error_message || m.operations_ui_report_create_error());
        }
      } catch {
        if (attempts > 15) {
          clearInterval(pollInterval);
          isGenerating = false;
          toast.error(m.operations_ui_report_create_timeout());
        }
      }
    }, 1000);
  } catch (e) {
    isGenerating = false;
    console.error("Failed to execute report:", e);
    toast.error(extractErrorMessage(e) || m.operations_ui_report_execution_error());
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
    const blob = new Blob([bytes], { type: mime });
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
  <div class="w-full p-6 md:p-8 flex flex-col gap-6">
    
    <!-- Sleek Top Controls Toolbar (matching Settings and Templates pages) -->
    <div class="flex flex-wrap items-center justify-between gap-4" in:fade={{ delay: 50, duration: 200 }}>
      
      <!-- Filters & Search Controls -->
      <div class="flex flex-wrap items-center gap-3 shrink-0">
        <!-- Search Field -->
        <div class="w-full sm:w-72">
          <SearchField
            bind:value={searchQuery}
            placeholder={m.operations_ui_reports_search_placeholder()}
          />
        </div>

        <!-- Category / Domain Filter -->
        <Select type="single" bind:value={selectedCategory}>
          <SelectTrigger class="h-9 text-xs bg-background/40 border-border/40 hover:bg-muted/40 transition-colors w-auto min-w-[200px]">
            {categoryOptions.find((c) => c.id === selectedCategory)?.label || m.operations_ui_report_category_all()}
          </SelectTrigger>
          <SelectContent>
            {#each categoryOptions as cat}
              <SelectItem value={cat.id}>{cat.label}</SelectItem>
            {/each}
          </SelectContent>
        </Select>

        <!-- Format Filter -->
        <Select type="single" bind:value={selectedFormatFilter}>
          <SelectTrigger class="h-9 text-xs bg-background/40 border-border/40 hover:bg-muted/40 transition-colors">
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

      <!-- Mode Switcher Tabs (Reports vs Analytics) -->
      <div class="flex items-center gap-3 shrink-0">
        <Tabs value={activeMainTab} onValueChange={(v) => (activeMainTab = v as "reports" | "analytics")}>
          <TabsList class="h-9 bg-muted/40 p-0.5 rounded-lg border border-border/40">
            <TabsTrigger value="reports" class="text-xs font-semibold gap-1.5 px-3 py-1 rounded-md">
              <DocumentChartBar class="w-3.5 h-3.5" />
              {m.operations_ui_reports_directory()}
              {#if reportDefinitions.length > 0}
                <span class="ms-1 px-1.5 py-0.2 rounded-full text-[10px] bg-primary/15 text-primary font-mono font-bold">
                  {filteredDefinitions.length}
                </span>
              {/if}
            </TabsTrigger>
            <TabsTrigger value="analytics" class="text-xs font-semibold gap-1.5 px-3 py-1 rounded-md">
              <ChartBar class="w-3.5 h-3.5" />
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
      <!-- Reports Cards Grid -->
      <div class="space-y-4" in:fade={{ duration: 150 }}>
        {#if isLoading}
          <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
            {#each Array(6) as _}
              <div class="h-44 rounded-xl bg-card/50 border border-border/40 animate-pulse"></div>
            {/each}
          </div>
        {:else if filteredDefinitions.length === 0}
          <Card class="border border-dashed border-border/60 bg-transparent p-12 text-center flex flex-col items-center justify-center gap-2 text-muted-foreground rounded-xl">
            <DocumentPage class="w-8 h-8 opacity-40" />
            <div class="text-sm font-bold text-foreground">{m.operations_ui_no_matching_reports()}</div>
            <div class="text-xs">{m.operations_ui_try_another_report_filter()}</div>
          </Card>
        {:else}
          <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
            {#each filteredDefinitions as def}
              {@const IconComponent = getDomainIcon(def.domain, def.category)}
              <div class="group relative rounded-xl border border-border/40 bg-card/60 backdrop-blur-xs p-5 shadow-2xs hover:shadow-md hover:border-primary/40 transition-all duration-200 flex flex-col justify-between gap-4">
                
                <div class="space-y-3">
                  <!-- Header Row: Icon + Code Badge -->
                  <div class="flex items-start justify-between gap-2">
                    <div class="w-8 h-8 rounded-lg bg-primary/10 text-primary border border-primary/20 flex items-center justify-center shrink-0 group-hover:bg-primary group-hover:text-primary-foreground transition-all">
                      <IconComponent class="w-4 h-4" />
                    </div>
                    <Badge variant="outline" class="text-[9px] font-mono uppercase bg-muted/30 border-border/40 text-muted-foreground">
                      {def.code}
                    </Badge>
                  </div>

                  <!-- Titles -->
                  <div>
                    <h3 class="text-sm font-semibold text-foreground group-hover:text-primary transition-colors leading-snug">
                      {def.name_ar || def.name}
                    </h3>
                    <div class="text-[10px] text-muted-foreground/80 font-mono mt-0.5 line-clamp-1">
                      {def.name}
                    </div>
                  </div>

                  <!-- Description -->
                  <p class="text-xs text-muted-foreground leading-relaxed line-clamp-2">
                    {def.description_ar || def.description || m.operations_ui_report_default_description()}
                  </p>
                </div>

                <!-- Footer: Supported Formats & Action -->
                <div class="pt-3 border-t border-border/40 flex items-center justify-between gap-2">
                  <div class="flex items-center gap-1">
                    {#each def.supported_formats || ["PDF"] as fmt}
                      <span class="px-1.5 py-0.5 rounded text-[9px] font-mono font-bold bg-muted/50 text-muted-foreground border border-border/40">
                        {fmt}
                      </span>
                    {/each}
                  </div>

                  <Button
                    variant="outline"
                    size="sm"
                    onclick={() => openGenerateSheet(def)}
                    class="h-8 px-3 text-xs font-semibold gap-1.5 group-hover:bg-primary group-hover:text-primary-foreground group-hover:border-primary transition-all cursor-pointer"
                  >
                    <Play class="w-3 h-3" />
                    {m.operations_ui_generate_report()}
                  </Button>
                </div>
              </div>
            {/each}
          </div>
        {/if}
      </div>
    {/if}
  </div>
</div>

<!-- Parameter Drawer Sheet -->
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

<!-- PDF / Document Preview Dialog -->
<Dialog bind:open={isPreviewOpen}>
  <DialogContent class="max-w-4xl h-[85vh] flex flex-col p-6 text-start">
    <DialogHeader class="flex flex-row items-center justify-between border-b pb-3">
      <div>
        <DialogTitle class="text-base font-bold">
          {m.operations_ui_report_preview_named({ name: selectedDefinition?.name_ar || selectedDefinition?.name || "" })}
        </DialogTitle>
        <DialogDescription class="text-xs text-muted-foreground">
          {m.operations_ui_document_ready_print_download()}
        </DialogDescription>
      </div>

      <div class="flex items-center gap-2">
        <Button
          variant="outline"
          size="sm"
          onclick={handleDownloadCurrent}
          class="h-8 text-xs font-semibold gap-1.5"
        >
          <Download class="w-3.5 h-3.5" />
          {m.operations_ui_download_file()}
        </Button>
      </div>
    </DialogHeader>

    <div class="flex-1 w-full h-full min-h-0 bg-muted/20 rounded-xl overflow-hidden my-2 border">
      {#if previewBlobUrl}
        <iframe
          src={previewBlobUrl}
          title={m.operations_ui_report_preview()}
          class="w-full h-full border-0"
        ></iframe>
      {/if}
    </div>

    <DialogFooter class="pt-2 border-t flex justify-end">
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
