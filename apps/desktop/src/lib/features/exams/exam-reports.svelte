<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui/button";
import { Badge } from "@orbis/ui/badge";
import { Skeleton } from "@orbis/ui/skeleton";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui/select";
import { toast } from "@orbis/ui";
import {
  ArrowRight,
  Calendar,
  ChartBar,
  ClipboardCheck,
  DocumentPage,
  Hashtag,
  Home,
  InfoCircle,
  Printer,
  Sparkles,
  UserBadgeCheck,
  Search,
  CheckCircle,
} from "@orbis/icons";
import { academicContext } from "$lib/stores";
import {
  createPeriodsQuery,
  createRoomsQuery,
  createSectionsQuery,
} from "$lib/features/academics";
import { createExamSessionsQuery } from "$lib/features/exams/queries";
import { createReportDefinitionsQuery } from "$lib/features/reporting/queries";
import { createTemplatesQuery } from "$lib/features/templates/queries";
import {
  extractErrorMessage,
  type ExamSessionDto,
  type ReportDefinitionDto,
  type TemplateDto,
} from "$lib/api";
import { localizeHref } from "$lib/paraglide/runtime";
import { openPrintDialog } from "$lib/stores/print.svelte";
import PdfReportViewer from "$lib/shared/components/pdf-report-viewer.svelte";

let { orgId }: { orgId: string } = $props();

// ── Search & Filter State ─────────────────────────────────────────────
let searchQuery = $state("");
let selectedCategory = $state<string>("all");
let selectedCode = $state<string>("");

// ── Parameters State ──────────────────────────────────────────────────
let selectedSessionId = $state<string>("");
let selectedRoomId = $state<string>("");
let selectedSectionId = $state<string>("");
let selectedDate = $state<string>("");
let selectedFormat = $state<"PDF" | "HTML" | "XLSX" | "CSV">("PDF");

// ── Report Viewer Dialog ──────────────────────────────────────────────
let isViewerOpen = $state(false);
let viewerParameters = $state<Record<string, any>>({});

// ── Queries ───────────────────────────────────────────────────────────
const periodsQuery = createPeriodsQuery(() => orgId);
const periods = $derived(periodsQuery.data ?? []);

// Active academic year & period
const activeYearName = $derived(academicContext.yearLabelAr);
const activePeriod = $derived(
  periods.find(
    (p) =>
      p.status === "active" ||
      p.status === "active_teaching" ||
      p.status === "grade_submission",
  ) || periods[0],
);
const activePeriodId = $derived(activePeriod ? String(activePeriod.id) : "");
const activePeriodName = $derived(activePeriod ? activePeriod.name : null);

// Grade Entry Lock Status
const isGradeEntryOpen = $derived(
  activePeriod
    ? activePeriod.status === "grade_submission" ||
        activePeriod.status === "active_teaching" ||
        activePeriod.status === "open"
    : false,
);

// Dynamic Real Exam Reports from Catalog / Database
const examReportsQuery = createReportDefinitionsQuery(
  () => orgId,
  () => ({ domain: "exam" }),
);
const allExamReports = $derived<ReportDefinitionDto[]>(
  examReportsQuery.data ?? [],
);

// Exam Sessions Query
const examSessionsQuery = createExamSessionsQuery(() => activePeriodId);
const examSessions = $derived<ExamSessionDto[]>(examSessionsQuery.data ?? []);

// Rooms & Sections Queries
const roomsQuery = createRoomsQuery(() => orgId);
const rooms = $derived(roomsQuery.data ?? []);

const sectionsQuery = createSectionsQuery(() => orgId);
const sections = $derived(sectionsQuery.data ?? []);

// System Document Templates Query
const documentTemplatesQuery = createTemplatesQuery(() => orgId);
const allTemplates = $derived<TemplateDto[]>(documentTemplatesQuery.data ?? []);
const examTemplates = $derived(
  allTemplates.filter(
    (t) =>
      t.entity_type === "exam" ||
      (t.document_type || "").toLowerCase().includes("exam"),
  ),
);

const isLoading = $derived(
  periodsQuery.isLoading ||
    examReportsQuery.isLoading ||
    examSessionsQuery.isLoading,
);

const error = $derived(
  periodsQuery.error || examReportsQuery.error || examSessionsQuery.error,
);

// Categories
const categoryTabs = [
  { id: "all", label: () => m.exam_reports_category_all() },
  { id: "seating", label: () => m.exam_reports_category_seating() },
  { id: "halls", label: () => m.exam_reports_category_halls() },
  { id: "proctors", label: () => m.exam_reports_category_proctors() },
  { id: "control", label: () => m.exam_reports_category_control() },
];

function getCategoryIcon(cat?: string) {
  const c = (cat || "").toLowerCase();
  if (c === "seating" || c.includes("seat")) return Hashtag;
  if (c === "halls" || c.includes("hall") || c.includes("room")) return Home;
  if (c === "proctors" || c.includes("proctor") || c.includes("staff"))
    return UserBadgeCheck;
  if (c === "control" || c.includes("result") || c.includes("secret"))
    return ClipboardCheck;
  return DocumentPage;
}

// Filtered Reports
const filteredReports = $derived.by(() => {
  return allExamReports.filter((report) => {
    const q = searchQuery.toLowerCase().trim();
    const matchesSearch =
      !q ||
      report.name.toLowerCase().includes(q) ||
      (report.name_ar && report.name_ar.toLowerCase().includes(q)) ||
      (report.description && report.description.toLowerCase().includes(q)) ||
      (report.description_ar &&
        report.description_ar.toLowerCase().includes(q)) ||
      report.code.toLowerCase().includes(q);

    const matchesCat =
      selectedCategory === "all" ||
      report.category.toLowerCase() === selectedCategory.toLowerCase();

    return matchesSearch && matchesCat;
  });
});

// Reactively select the first report if none or invalid
$effect(() => {
  if (filteredReports.length > 0) {
    if (
      !selectedCode ||
      !filteredReports.some((r) => r.code === selectedCode)
    ) {
      selectedCode = filteredReports[0].code;
    }
  }
});

const selectedReport = $derived<ReportDefinitionDto | null>(
  allExamReports.find((r) => r.code === selectedCode) ??
    (filteredReports[0] || null),
);

// Dynamic Real KPI Stats
const kpiStats = $derived([
  {
    label: m.exam_reports_total_reports(),
    value: allExamReports.length,
  },
  {
    label: m.operations_ui_available_exam_sessions(),
    value: examSessions.length,
  },
  {
    label: m.exam_reports_smart_templates(),
    value: examTemplates.length,
  },
  {
    label: m.exam_reports_available_halls(),
    value: rooms.length,
  },
]);

// Real Diagnostic Insights
const advisorInsights = $derived.by(() => {
  const insights = [];

  if (allExamReports.length > 0) {
    insights.push({
      id: "reports-catalog",
      text: m.exam_reports_catalog_ready({
        count: String(allExamReports.length),
      }),
      variant: "success" as const,
    });
  }

  if (examSessions.length > 0) {
    insights.push({
      id: "sessions-ok",
      text: m.exam_reports_sessions_registered({
        count: String(examSessions.length),
      }),
      variant: "success" as const,
    });
  } else {
    insights.push({
      id: "sessions-empty",
      text: m.exam_reports_no_sessions_registered(),
      variant: "warning" as const,
    });
  }

  insights.push({
    id: "lock-state",
    text: isGradeEntryOpen
      ? m.operations_ui_grade_entry_open_warning()
      : m.operations_ui_grade_entry_closed_notice(),
    variant: isGradeEntryOpen ? ("warning" as const) : ("success" as const),
  });

  return insights;
});

function handleSelectReport(code: string) {
  selectedCode = code;
  const rep = allExamReports.find((r) => r.code === code);
  if (rep) {
    toast.info(
      m.operations_ui_report_selected({ title: rep.name_ar || rep.name }),
    );
  }
}

function buildParameters(): Record<string, any> {
  const params: Record<string, any> = {
    org_id: orgId,
    period_id: activePeriodId || undefined,
  };
  if (selectedSessionId) params.session_id = selectedSessionId;
  if (selectedRoomId) params.room_id = selectedRoomId;
  if (selectedSectionId) params.section_id = selectedSectionId;
  if (selectedDate) params.date = selectedDate;
  return params;
}

function handlePreviewAndPrint() {
  if (!selectedReport) return;
  viewerParameters = buildParameters();
  isViewerOpen = true;
}

function handlePrintTemplate() {
  if (!selectedReport) return;
  const session = examSessions.find((s) => String(s.id) === selectedSessionId);
  const room = rooms.find((r) => String(r.id) === selectedRoomId);
  const section = sections.find((sec) => String(sec.id) === selectedSectionId);

  const examContext = {
    report: {
      code: selectedReport.code,
      title: selectedReport.name_ar || selectedReport.name,
      description: selectedReport.description_ar || selectedReport.description,
      category: selectedReport.category,
      domain: selectedReport.domain,
    },
    exam: {
      sessionId: selectedSessionId || null,
      sessionName:
        session?.title_ar || session?.title || activePeriodName || "",
      hallId: selectedRoomId || null,
      hallName: room?.name_ar || room?.name || "",
      classId: selectedSectionId || null,
      className: section?.name_ar || section?.name || "",
      examDate: selectedDate || "",
    },
    AcademicCalendar: {
      name: activeYearName || "",
      periodName: activePeriodName || "",
    },
  };

  openPrintDialog(
    "exam",
    [selectedReport.code],
    [selectedReport.name_ar || selectedReport.name],
    {
      documentType: selectedReport.code,
      contexts: [examContext],
    },
  );
}
</script>

<div class="h-full w-full overflow-y-auto bg-background/30 scrollbar-hide p-6 select-none font-sans flex flex-col gap-6">
  
  <!-- Sleek Header with Active Year & Term Indicators -->
  <div class="flex items-center justify-between border-b border-border/40 pb-4">
    <div class="flex items-center gap-3 text-start">
      <div class="h-8.5 w-8.5 rounded-lg bg-amber-500/10 text-amber-500 flex items-center justify-center shrink-0 font-bold text-xs">
        <DocumentPage size={18} />
      </div>
      <div class="flex flex-col">
        <h2 class="text-xs font-bold text-foreground tracking-wide uppercase">
          {m.operations_ui_exam_reports_center_title()}
        </h2>
        <span class="text-[9.5px] text-muted-foreground/90 mt-0.5">
          {m.operations_ui_exam_reports_center_description()}
        </span>
      </div>
    </div>

    <!-- Active Academic Year & Term Pills & Template Designer Link -->
    <div class="flex items-center gap-2">
      {#if activeYearName}
        <Badge variant="outline" class="text-[9px] px-2 py-0.5 font-bold flex items-center gap-1.5 border-border/60 bg-card/60">
          <Calendar size={10} class="text-primary" />
          <span>{activeYearName}</span>
        </Badge>
      {/if}
      
      {#if activePeriodName}
        <Badge variant="outline" class="text-[9px] px-2 py-0.5 font-bold flex items-center gap-1.5 border-border/60 bg-card/60">
          <span class="h-1.5 w-1.5 rounded-full {isGradeEntryOpen ? 'bg-amber-500 animate-pulse' : 'bg-emerald-500'}"></span>
          <span>{activePeriodName}</span>
        </Badge>
      {/if}

      <Button 
        variant="outline" 
        size="sm" 
        href={localizeHref(`/${orgId}/templates`)}
        class="text-[10px] font-bold px-3 h-7.5 rounded-md gap-1.5 cursor-pointer ms-2 border-border/60"
      >
        <DocumentPage size={13} class="text-primary" />
        <span>{m.operations_ui_system_template_designer()}</span>
      </Button>

      <Button 
        variant="default" 
        size="sm" 
        class="text-[10px] font-bold px-3 h-7.5 rounded-md gap-1.5 cursor-pointer"
        onclick={handlePreviewAndPrint}
        disabled={!selectedReport}
      >
        <Printer size={13} />
        <span>{m.operations_ui_print_report()}</span>
      </Button>
    </div>
  </div>

  {#if isLoading}
    <!-- Loading Skeleton -->
    <div class="grid grid-cols-1 lg:grid-cols-12 gap-8 items-start w-full">
      <div class="lg:col-span-7 xl:col-span-8 flex flex-col gap-3">
        <Skeleton class="h-8 w-full mb-2 rounded-lg" />
        {#each Array(6) as _}
          <Skeleton class="h-14 w-full rounded-lg" />
        {/each}
      </div>
      <div class="lg:col-span-5 xl:col-span-4 flex flex-col gap-5 border border-border/50 bg-card/20 rounded-xl p-5">
        <Skeleton class="h-4 w-40 mb-1 rounded" />
        <Skeleton class="h-48 w-full rounded-lg" />
      </div>
    </div>
  {:else if error}
    <div class="flex flex-col items-center justify-center py-16 text-destructive border border-destructive/20 bg-destructive/5 rounded-xl gap-2">
      <InfoCircle size={24} />
      <h3 class="font-semibold text-xs">{m.operations_ui_report_data_load_error()}</h3>
      <p class="text-[10px] text-muted-foreground">{extractErrorMessage(error)}</p>
    </div>
  {:else}
    <!-- Main Content Split Grid -->
    <div class="grid grid-cols-1 lg:grid-cols-12 gap-6 items-start w-full">
      
      <!-- 1. LEFT COLUMN: Directory Index List with Search & Category Filters -->
      <div class="lg:col-span-7 xl:col-span-8 flex flex-col gap-3">
        
        <!-- Search bar -->
        <div class="relative">
          <Search size={14} class="absolute start-3 top-1/2 -translate-y-1/2 text-muted-foreground/70" />
          <Input
            type="text"
            bind:value={searchQuery}
            placeholder={m.exam_reports_search_placeholder()}
            class="ps-9 h-9 text-xs bg-card/60 border-border/60 rounded-lg"
          />
        </div>

        <!-- Category filter pills -->
        <div class="flex items-center gap-1.5 overflow-x-auto pb-1 scrollbar-hide">
          {#each categoryTabs as tab}
            <button
              type="button"
              onclick={() => (selectedCategory = tab.id)}
              class="px-2.5 py-1 rounded-md text-[10px] font-semibold transition-colors duration-150 whitespace-nowrap cursor-pointer border {selectedCategory === tab.id ? 'bg-primary text-primary-foreground border-primary' : 'bg-muted/40 text-muted-foreground border-border/40 hover:bg-muted/70 hover:text-foreground'}"
            >
              {tab.label()}
            </button>
          {/each}
        </div>

        <div class="flex items-center justify-between text-[10px] text-muted-foreground font-semibold px-1 mt-1">
          <span>{m.exam_reports_available_count({ count: String(filteredReports.length) })}</span>
          <span class="font-mono text-[9px]">Domain: Exam & Control</span>
        </div>
        
        <!-- Reports list -->
        <div class="flex flex-col gap-1 max-h-[calc(100vh-280px)] overflow-y-auto pe-1">
          {#if filteredReports.length === 0}
            <div class="flex flex-col items-center justify-center py-12 text-muted-foreground border border-dashed border-border/50 rounded-xl gap-2">
              <DocumentPage size={20} class="opacity-40" />
              <span class="text-xs">{m.exam_reports_no_matches()}</span>
            </div>
          {:else}
            {#each filteredReports as report (report.code)}
              {@const IconComp = getCategoryIcon(report.category)}
              <!-- svelte-ignore a11y_click_events_have_key_events -->
              <!-- svelte-ignore a11y_no_static_element_interactions -->
              <div 
                onclick={() => handleSelectReport(report.code)}
                class="group flex items-center justify-between py-2 px-3 border border-border/30 rounded-lg transition-all duration-150 select-none cursor-pointer
                  {selectedCode === report.code ? 'bg-primary/10 border-primary/50 shadow-xs' : 'hover:bg-muted/40 bg-card/30'}"
              >
                <div class="flex items-center gap-3 min-w-0 text-start">
                  <div class="h-8 w-8 rounded-lg flex items-center justify-center shrink-0 transition-colors duration-150
                    {selectedCode === report.code ? 'bg-primary text-primary-foreground' : 'bg-muted/60 text-muted-foreground group-hover:bg-primary/10 group-hover:text-primary'}"
                  >
                    <IconComp size={15} />
                  </div>
                  <div class="flex flex-col min-w-0">
                    <span class="text-xs font-bold text-foreground group-hover:text-primary transition-colors duration-150 truncate">
                      {report.name_ar || report.name}
                    </span>
                    <span class="text-[9.5px] text-muted-foreground/80 leading-normal truncate mt-0.5 max-w-sm sm:max-w-md md:max-w-lg">
                      {report.description_ar || report.description || report.code}
                    </span>
                  </div>
                </div>
                
                <div class="flex items-center gap-2 shrink-0">
                  <span class="text-[8.5px] font-mono text-muted-foreground/80 bg-muted/50 border border-border/40 px-2 py-0.5 rounded select-none">
                    {report.category}
                  </span>
                  <ArrowRight 
                    size={12} 
                    class="text-muted-foreground/40 opacity-0 group-hover:opacity-100 transition-opacity duration-150 rtl:rotate-180" 
                  />
                </div>
              </div>
            {/each}
          {/if}
        </div>
      </div>

      <!-- 2. RIGHT COLUMN: Parameter Inputs, Selected Report Actions, and Diagnostics -->
      <div class="lg:col-span-5 xl:col-span-4 flex flex-col gap-4 border border-border/50 bg-card/20 rounded-xl p-4 select-none text-start">
        
        {#if selectedReport}
          <!-- Selected Report Details & Parameters Card -->
          <div class="flex flex-col gap-3 bg-card/60 border border-border/50 p-4 rounded-xl">
            <div class="flex items-center gap-3">
              <div class="h-9 w-9 rounded-lg bg-primary/10 text-primary flex items-center justify-center shrink-0">
                {#key selectedReport.code}
                  {@const IconComp = getCategoryIcon(selectedReport.category)}
                  <IconComp size={18} />
                {/key}
              </div>
              <div class="flex flex-col min-w-0">
                <span class="text-xs font-bold text-foreground truncate">{selectedReport.name_ar || selectedReport.name}</span>
                <span class="text-[9px] font-mono text-muted-foreground">{selectedReport.code}</span>
              </div>
            </div>

            <p class="text-[10px] text-muted-foreground leading-relaxed">
              {selectedReport.description_ar || selectedReport.description || m.exam_reports_default_description()}
            </p>

            <!-- Parameter Configuration Controls -->
            <div class="pt-2 border-t border-border/40 flex flex-col gap-2.5">
              <span class="text-[9.5px] font-bold text-foreground tracking-wide">{m.exam_reports_options_criteria()}</span>

              <!-- Session Selector -->
              {#if examSessions.length > 0}
                <div class="space-y-1">
                  <Label class="text-[9.5px] text-muted-foreground font-semibold">{m.exam_reports_session_label()}</Label>
                  <Select type="single" value={selectedSessionId} onValueChange={(v) => (selectedSessionId = v ?? "")}>
                    <SelectTrigger class="h-8 text-xs bg-background/50">
                      <span>{examSessions.find((s) => String(s.id) === selectedSessionId)?.title_ar || examSessions.find((s) => String(s.id) === selectedSessionId)?.title || m.exam_reports_all_sessions()}</span>
                    </SelectTrigger>
                    <SelectContent>
                      <SelectItem value="" class="text-xs">{m.exam_reports_all_sessions()}</SelectItem>
                      {#each examSessions as sess (sess.id)}
                        <SelectItem value={String(sess.id)} class="text-xs">{sess.title_ar || sess.title}</SelectItem>
                      {/each}
                    </SelectContent>
                  </Select>
                </div>
              {/if}

              <!-- Room / Hall Selector -->
              {#if rooms.length > 0}
                <div class="space-y-1">
                  <Label class="text-[9.5px] text-muted-foreground font-semibold">{m.exam_reports_room_committee()}</Label>
                  <Select type="single" value={selectedRoomId} onValueChange={(v) => (selectedRoomId = v ?? "")}>
                    <SelectTrigger class="h-8 text-xs bg-background/50">
                      <span>{rooms.find((r) => String(r.id) === selectedRoomId)?.name_ar || rooms.find((r) => String(r.id) === selectedRoomId)?.name || m.exam_reports_all_rooms_committees()}</span>
                    </SelectTrigger>
                    <SelectContent>
                      <SelectItem value="" class="text-xs">{m.exam_reports_all_rooms_committees()}</SelectItem>
                      {#each rooms as rm (rm.id)}
                        <SelectItem value={String(rm.id)} class="text-xs">{rm.name_ar || rm.name}</SelectItem>
                      {/each}
                    </SelectContent>
                  </Select>
                </div>
              {/if}

              <!-- Section / Class Selector -->
              {#if sections.length > 0}
                <div class="space-y-1">
                  <Label class="text-[9.5px] text-muted-foreground font-semibold">{m.exam_reports_class_section()}</Label>
                  <Select type="single" value={selectedSectionId} onValueChange={(v) => (selectedSectionId = v ?? "")}>
                    <SelectTrigger class="h-8 text-xs bg-background/50">
                      <span>{sections.find((s) => String(s.id) === selectedSectionId)?.name_ar || sections.find((s) => String(s.id) === selectedSectionId)?.name || m.exam_reports_all_classes_sections()}</span>
                    </SelectTrigger>
                    <SelectContent>
                      <SelectItem value="" class="text-xs">{m.exam_reports_all_classes_sections()}</SelectItem>
                      {#each sections as sec (sec.id)}
                        <SelectItem value={String(sec.id)} class="text-xs">{sec.name_ar || sec.name}</SelectItem>
                      {/each}
                    </SelectContent>
                  </Select>
                </div>
              {/if}

              <!-- Date Filter -->
              <div class="space-y-1">
                <Label class="text-[9.5px] text-muted-foreground font-semibold">{m.exam_reports_session_date()}</Label>
                <Input
                  type="date"
                  bind:value={selectedDate}
                  class="h-8 text-xs bg-background/50 font-mono"
                />
              </div>
            </div>

            <!-- Action Buttons -->
            <div class="pt-3 border-t border-border/40 flex flex-col gap-2">
              <Button 
                variant="default" 
                size="sm" 
                class="w-full text-xs font-bold justify-center gap-2 cursor-pointer h-8.5 shadow-xs"
                onclick={handlePreviewAndPrint}
              >
                <Printer size={14} />
                <span>{m.exam_reports_preview_print_pdf()}</span>
              </Button>

              <Button 
                variant="outline" 
                size="sm" 
                class="w-full text-xs font-semibold justify-center gap-2 cursor-pointer h-8 border-border/60"
                onclick={handlePrintTemplate}
              >
                <DocumentPage size={13} class="text-primary" />
                <span>{m.exam_reports_smart_template_print()}</span>
              </Button>
            </div>
          </div>
        {/if}

        <!-- Sidebar Section: KPI Summary -->
        <div>
          <h4 class="text-[9.5px] font-bold text-muted-foreground/80 tracking-wider uppercase border-b border-border/30 pb-2 mb-2.5">
            {m.operations_ui_print_center_indicators()}
          </h4>
          
          <div class="grid grid-cols-2 gap-3 pb-3 border-b border-border/30">
            {#each kpiStats as stat}
              <div class="flex flex-col text-start">
                <span class="text-[9px] text-muted-foreground/80">{stat.label}</span>
                <span class="text-sm font-bold text-foreground font-mono mt-0.5 leading-none">{stat.value}</span>
              </div>
            {/each}
          </div>
        </div>

        <!-- Sidebar Section: Advisor System Diagnostics -->
        <div class="flex flex-col gap-2.5">
          <div class="flex items-center gap-2 text-foreground font-semibold text-[10.5px]">
            <Sparkles size={13} class="text-primary animate-pulse" />
            <span>{m.operations_ui_advisor_diagnostics()}</span>
          </div>
          
          <div class="flex flex-col gap-2">
            {#each advisorInsights as insight (insight.id)}
              <div class="flex items-start gap-2 p-2 rounded-lg bg-card/30 border border-border/20 text-[9.5px] leading-relaxed text-muted-foreground">
                <div class="shrink-0 mt-0.5">
                  {#if insight.variant === 'warning'}
                    <span class="relative flex h-2 w-2">
                      <span class="animate-ping absolute inline-flex h-full w-full rounded-full bg-amber-400 opacity-75"></span>
                      <span class="relative inline-flex rounded-full h-2 w-2 bg-amber-500"></span>
                    </span>
                  {:else}
                    <span class="inline-flex rounded-full h-2 w-2 bg-emerald-500"></span>
                  {/if}
                </div>

                <div class="flex flex-col min-w-0">
                  <span class="text-foreground/90 font-medium">{insight.text}</span>
                </div>
              </div>
            {/each}
          </div>
        </div>

        <!-- Link to System Templates -->
        <div class="pt-2 border-t border-border/30 flex items-center justify-between text-[9px] text-muted-foreground">
          <a 
            href={localizeHref(`/${orgId}/templates`)}
            class="text-primary font-bold hover:underline flex items-center gap-1"
          >
            <span>{m.operations_ui_system_template_designer()}</span>
            <ArrowRight size={10} class="rtl:rotate-180" />
          </a>
          <span class="font-mono">Templates Connected</span>
        </div>

      </div>

    </div>
  {/if}
</div>

<!-- Dynamic PDF Report Viewer Modal -->
{#if selectedReport}
  <PdfReportViewer
    bind:open={isViewerOpen}
    templateId={selectedReport.code}
    parameters={viewerParameters}
    reportTitle={selectedReport.name_ar || selectedReport.name}
  />
{/if}
