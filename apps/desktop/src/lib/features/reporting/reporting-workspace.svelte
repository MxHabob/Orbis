<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { api, extractErrorMessage } from "$lib/api";
import { academicContext } from "$lib/stores";
import { l, isAr } from "$lib/core/i18n";
import type { ReportDefinitionDto } from "$lib/api/types";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
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
  Sheet,
  SheetContent,
  SheetHeader,
  SheetTitle,
  SheetDescription,
  SheetFooter,
} from "@orbis/ui/sheet";
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

const categoryOptions = [
  { id: "all", label: "جميع المجالات الأكاديمية والمؤسسية" },
  { id: "academic", label: "الشؤون الأكاديمية والدرجات" },
  { id: "student", label: "شؤون الطلاب والقبول والتسجيل" },
  { id: "attendance", label: "الحضور والمواظبة والانضباط" },
  { id: "exam", label: "الاختبارات والكنترول المدرسي" },
  { id: "finance", label: "المالية والمحاسبة والرسوم" },
  { id: "hr", label: "الموارد البشرية وشؤون الكادر" },
  { id: "administration", label: "الإدارة والتشغيل والسياسات" },
];

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
  jobStatus = "جاري معالجة البيانات وتوليد المستند...";

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
          jobStatus = "تم إكمال التقرير بنجاح";
          toast.success("تم توليد التقرير بنجاح");
          await handlePreviewReport(jobId);
        } else if (status.status === "failed") {
          clearInterval(pollInterval);
          isGenerating = false;
          jobStatus = "فشل توليد التقرير";
          toast.error(status.error_message || "فشلت عملية إنشاء التقرير");
        }
      } catch {
        if (attempts > 15) {
          clearInterval(pollInterval);
          isGenerating = false;
          toast.error("انتهت مهلة انتظار إنشاء التقرير");
        }
      }
    }, 1000);
  } catch (e) {
    isGenerating = false;
    console.error("Failed to execute report:", e);
    toast.error(extractErrorMessage(e) || "حدث خطأ أثناء تنفيذ التقرير");
  }
}

async function handlePreviewReport(jobId: string) {
  try {
    const bytes = await api.downloadReportFile(jobId);
    const mime =
      selectedFormat === "PDF" ? "application/pdf" : "text/html;charset=utf-8";
    const blob = new Blob([bytes], { type: mime });
    if (previewBlobUrl) {
      URL.revokeObjectURL(previewBlobUrl);
    }
    previewBlobUrl = URL.createObjectURL(blob);
    isPreviewOpen = true;
    isSheetOpen = false;
  } catch (e) {
    console.error("Failed to load report preview:", e);
    toast.error("فشل فتح معاينة التقرير");
  }
}

function handleDownloadCurrent() {
  if (!previewBlobUrl || !selectedDefinition) return;
  const link = document.createElement("a");
  link.href = previewBlobUrl;
  link.download = `${selectedDefinition.code}_${new Date().toISOString().slice(0, 10)}.${selectedFormat.toLowerCase()}`;
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
            placeholder="بحث في التقارير والكشوفات..."
          />
        </div>

        <!-- Category / Domain Filter -->
        <Select type="single" bind:value={selectedCategory}>
          <SelectTrigger class="h-9 text-xs bg-background/40 border-border/40 hover:bg-muted/40 transition-colors w-auto min-w-[200px]">
            {categoryOptions.find((c) => c.id === selectedCategory)?.label || "جميع المجالات"}
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
            {({ all: "كافة الصيغ (PDF, XLSX)", PDF: "مستندات PDF", XLSX: "جداول XLSX", CSV: "ملفات CSV", HTML: "تفاعلي HTML" } as Record<string, string>)[selectedFormatFilter] || "صيغة التصدير"}
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="all">كافة الصيغ (PDF, XLSX)</SelectItem>
            <SelectItem value="PDF">مستندات PDF</SelectItem>
            <SelectItem value="XLSX">جداول XLSX</SelectItem>
            <SelectItem value="CSV">ملفات CSV</SelectItem>
            <SelectItem value="HTML">تفاعلي HTML</SelectItem>
          </SelectContent>
        </Select>
      </div>

      <!-- Mode Switcher Tabs (Reports vs Analytics) -->
      <div class="flex items-center gap-3 shrink-0">
        <Tabs value={activeMainTab} onValueChange={(v) => (activeMainTab = v as "reports" | "analytics")}>
          <TabsList class="h-9 bg-muted/40 p-0.5 rounded-lg border border-border/40">
            <TabsTrigger value="reports" class="text-xs font-semibold gap-1.5 px-3 py-1 rounded-md">
              <DocumentChartBar class="w-3.5 h-3.5" />
              دليل التقارير
              {#if reportDefinitions.length > 0}
                <span class="ms-1 px-1.5 py-0.2 rounded-full text-[10px] bg-primary/15 text-primary font-mono font-bold">
                  {filteredDefinitions.length}
                </span>
              {/if}
            </TabsTrigger>
            <TabsTrigger value="analytics" class="text-xs font-semibold gap-1.5 px-3 py-1 rounded-md">
              <ChartBar class="w-3.5 h-3.5" />
              المؤشرات والتحليلات
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
            <div class="text-sm font-bold text-foreground">لا توجد تقارير مطابقة</div>
            <div class="text-xs">جرب اختيار مجال آخر أو تغيير كلمة البحث</div>
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
                    {def.description_ar || def.description || "تقرير جاهز للتوليد والطباعة بصيغة PDF وجداول بيانات."}
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
                    توليد التقرير
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
  <Sheet bind:open={isSheetOpen}>
    <SheetContent side="right" class="w-full sm:max-w-md flex flex-col justify-between p-6 text-start">
      <div class="space-y-6 overflow-y-auto pr-1">
        <SheetHeader class="space-y-1">
          <div class="flex items-center gap-2">
            <div class="w-8 h-8 rounded-lg bg-primary/10 flex items-center justify-center text-primary">
              <DocumentChartBar class="w-4 h-4" />
            </div>
            <div>
              <SheetTitle class="text-base font-bold text-foreground">
                توليد {selectedDefinition.name_ar || selectedDefinition.name}
              </SheetTitle>
              <SheetDescription class="text-xs text-muted-foreground">
                حدد معايير الفلترة وصيغة التصدير المطلوبة
              </SheetDescription>
            </div>
          </div>
        </SheetHeader>

        <!-- Form Fields -->
        <div class="space-y-4">
          <!-- Academic Period -->
          <div class="space-y-1.5">
            <Label class="text-xs font-semibold">الفصل الدراسي</Label>
            <Select value={selectedPeriodId} onValueChange={(v) => (selectedPeriodId = v)}>
              <SelectTrigger class="h-9 text-xs">
                <span>
                  {periods.find((p) => p.id === selectedPeriodId)
                    ? l(periods.find((p) => p.id === selectedPeriodId)?.name_ar, periods.find((p) => p.id === selectedPeriodId)?.name)
                    : "اختر الفصل الأكاديمي..."}
                </span>
              </SelectTrigger>
              <SelectContent>
                {#each periods as p}
                  <SelectItem value={p.id}>
                    {l(p.name_ar, p.name) || p.name}
                  </SelectItem>
                {/each}
              </SelectContent>
            </Select>
          </div>

          <!-- Class Selection (Optional) -->
          <div class="space-y-1.5">
            <Label class="text-xs font-semibold">تصفية حسب الفصل (اختياري)</Label>
            <Select value={selectedClassId} onValueChange={(v) => (selectedClassId = v)}>
              <SelectTrigger class="h-9 text-xs">
                <span>
                  {classes.find((c) => String(c.id) === selectedClassId)
                    ? l(classes.find((c) => String(c.id) === selectedClassId)?.name_ar, classes.find((c) => String(c.id) === selectedClassId)?.name)
                    : "جميع الفصول والشعب"}
                </span>
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="">جميع الفصول والشعب</SelectItem>
                {#each classes as c}
                  <SelectItem value={String(c.id)}>
                    {l(c.name_ar, c.name) || c.name}
                  </SelectItem>
                {/each}
              </SelectContent>
            </Select>
          </div>

          <!-- Date Range (Optional) -->
          <div class="grid grid-cols-2 gap-3">
            <div class="space-y-1.5">
              <Label class="text-xs font-semibold">من تاريخ</Label>
              <Input type="date" bind:value={dateFrom} class="h-9 text-xs" />
            </div>
            <div class="space-y-1.5">
              <Label class="text-xs font-semibold">إلى تاريخ</Label>
              <Input type="date" bind:value={dateTo} class="h-9 text-xs" />
            </div>
          </div>

          <!-- Output Format -->
          <div class="space-y-1.5">
            <Label class="text-xs font-semibold">صيغة الملف</Label>
            <div class="grid grid-cols-2 gap-2">
              <button
                type="button"
                onclick={() => (selectedFormat = "PDF")}
                class="p-2.5 rounded-xl border text-xs font-bold transition-all flex items-center justify-center gap-2 cursor-pointer {selectedFormat === 'PDF' ? 'border-primary bg-primary/10 text-primary' : 'border-border bg-card text-muted-foreground hover:text-foreground'}"
              >
                <DocumentPage class="w-4 h-4" />
                مستند PDF جاهز للطباعة
              </button>
              <button
                type="button"
                onclick={() => (selectedFormat = "HTML")}
                class="p-2.5 rounded-xl border text-xs font-bold transition-all flex items-center justify-center gap-2 cursor-pointer {selectedFormat === 'HTML' ? 'border-primary bg-primary/10 text-primary' : 'border-border bg-card text-muted-foreground hover:text-foreground'}"
              >
                <Eye class="w-4 h-4" />
                عرض تفاعلي HTML
              </button>
            </div>
          </div>

          {#if isGenerating}
            <div class="p-4 rounded-xl bg-primary/5 border border-primary/20 flex items-center gap-3">
              <Refresh class="w-5 h-5 text-primary animate-spin" />
              <div class="text-xs text-primary font-semibold">{jobStatus}</div>
            </div>
          {/if}
        </div>
      </div>

      <SheetFooter class="pt-4 border-t border-border mt-4 flex items-center justify-end gap-2">
        <Button
          variant="outline"
          size="sm"
          onclick={() => (isSheetOpen = false)}
          disabled={isGenerating}
          class="h-9 text-xs font-semibold"
        >
          إلغاء
        </Button>
        <Button
          variant="default"
          size="sm"
          onclick={handleExecuteReport}
          disabled={isGenerating}
          class="h-9 text-xs font-bold gap-1.5 shadow-sm cursor-pointer"
        >
          <Play class="w-3.5 h-3.5" />
          {isGenerating ? "جاري الإنشاء..." : "بدء التوليد والمعاينة"}
        </Button>
      </SheetFooter>
    </SheetContent>
  </Sheet>
{/if}

<!-- PDF / Document Preview Dialog -->
<Dialog bind:open={isPreviewOpen}>
  <DialogContent class="max-w-4xl h-[85vh] flex flex-col p-6 text-start">
    <DialogHeader class="flex flex-row items-center justify-between border-b pb-3">
      <div>
        <DialogTitle class="text-base font-bold">
          معاينة التقرير: {selectedDefinition?.name_ar || selectedDefinition?.name}
        </DialogTitle>
        <DialogDescription class="text-xs text-muted-foreground">
          المستند جاهز للطباعة والتنزيل
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
          تنزيل الملف
        </Button>
      </div>
    </DialogHeader>

    <div class="flex-1 w-full h-full min-h-0 bg-muted/20 rounded-xl overflow-hidden my-2 border">
      {#if previewBlobUrl}
        <iframe
          src={previewBlobUrl}
          title="معاينة التقرير"
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
        إغلاق
      </Button>
    </DialogFooter>
  </DialogContent>
</Dialog>
