<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui/button";
import { Badge } from "@orbis/ui/badge";
import { Skeleton } from "@orbis/ui/skeleton";
import { toast } from "@orbis/ui";
import { ArrowRight, Calendar, ChartBar, ClipboardCheck, Document, DocumentPage, Hashtag, Home, InfoCircle, Printer, Sparkles, UserBadgeCheck } from "@orbis/icons";
import { academicContext } from "$lib/stores";
import { createPeriodsQuery } from "$lib/features/academics";
import { createExamSessionsQuery } from "$lib/features/exams/queries";
import { createTemplatesQuery } from "$lib/features/templates/queries";
import { extractErrorMessage, type ExamSessionDto, type TemplateDto } from "$lib/api";
import { localizeHref } from "$lib/paraglide/runtime";

let { orgId }: { orgId: string } = $props();

let selectedReportId = $state<string>("timetable");

// Queries
const periodsQuery = createPeriodsQuery(() => orgId);
const periods = $derived(periodsQuery.data ?? []);

// Active academic year & period
const activeYearName = $derived(academicContext.yearLabelAr);
const activePeriod = $derived(
  periods.find((p) => p.status === "active" || p.status === "active_teaching" || p.status === "grade_submission") || periods[0],
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

// Exam Sessions Query
const examSessionsQuery = createExamSessionsQuery(() => activePeriodId);
const examSessions = $derived<ExamSessionDto[]>(examSessionsQuery.data ?? []);

// System Document Templates Query
const documentTemplatesQuery = createTemplatesQuery(
  () => orgId,
  () => ({ documentType: "EXAM_REGISTER" }),
);
const documentTemplates = $derived<TemplateDto[]>(
  documentTemplatesQuery.data ?? [],
);

const isLoading = $derived(
  periodsQuery.isLoading ||
    examSessionsQuery.isLoading ||
    documentTemplatesQuery.isLoading,
);

const error = $derived(
  periodsQuery.error || examSessionsQuery.error || documentTemplatesQuery.error,
);

// Directory list of available exam reports
const reportItems = $derived([
  {
    id: "timetable",
    title: m.common_exams_reports_timetable(),
    desc: m.operations_ui_report_timetable_description(),
    icon: Calendar,
    badge: m.operations_ui_report_timetable_badge(),
  },
  {
    id: "seating_list",
    title: m.common_exams_reports_seating_list(),
    desc: m.operations_ui_report_seating_list_description(),
    icon: Hashtag,
    badge: m.operations_ui_report_seating_list_badge(),
  },
  {
    id: "hall_map",
    title: m.common_exams_reports_hall_map(),
    desc: m.operations_ui_report_hall_map_description(),
    icon: Home,
    badge: m.operations_ui_report_hall_map_badge(),
  },
  {
    id: "proctor_sheet",
    title: m.common_exams_reports_proctor_sheet(),
    desc: m.operations_ui_report_proctor_sheet_description(),
    icon: UserBadgeCheck,
    badge: m.operations_ui_report_proctor_sheet_badge(),
  },
  {
    id: "student_cards",
    title: m.common_exams_reports_student_cards(),
    desc: m.operations_ui_report_student_cards_description(),
    icon: Document,
    badge: m.operations_ui_report_student_cards_badge(),
  },
  {
    id: "attendance",
    title: m.common_exams_reports_attendance(),
    desc: m.operations_ui_report_attendance_description(),
    icon: ClipboardCheck,
    badge: m.operations_ui_report_attendance_badge(),
  },
  {
    id: "summary",
    title: m.common_exams_reports_summary(),
    desc: m.operations_ui_report_summary_description(),
    icon: ChartBar,
    badge: m.operations_ui_report_summary_badge(),
  },
]);

const selectedReport = $derived(
  reportItems.find((r) => r.id === selectedReportId) || reportItems[0],
);

// Dynamic Real KPI Stats
const kpiStats = $derived([
  {
    label: m.operations_ui_registered_system_templates(),
    value: documentTemplates.length,
  },
  {
    label: m.operations_ui_available_exam_sessions(),
    value: examSessions.length,
  },
  {
    label: m.operations_ui_print_center_status(),
    value: documentTemplates.length > 0 ? m.operations_ui_connected() : m.operations_ui_waiting_for_template(),
  },
  {
    label: m.operations_ui_export_format(),
    value: "PDF / Excel",
  },
]);

// Real Diagnostic Insights
const advisorInsights = $derived.by(() => {
  const insights = [];

  if (documentTemplates.length > 0) {
    insights.push({
      id: "templates-ok",
      text: m.operations_ui_templates_found({ count: documentTemplates.length }),
      variant: "success" as const,
    });
  } else {
    insights.push({
      id: "templates-none",
      text: m.operations_ui_templates_not_found_tip(),
      variant: "neutral" as const,
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

function handleSelectReport(id: string) {
  selectedReportId = id;
  toast.info(m.operations_ui_report_selected({ title: reportItems.find((r) => r.id === id)?.title ?? "" }));
}

function handlePrint() {
  window.print();
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
        onclick={handlePrint}
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
        <Skeleton class="h-4 w-32 mb-2 rounded" />
        {#each Array(5) as _}
          <Skeleton class="h-12 w-full rounded-lg" />
        {/each}
      </div>
      <div class="lg:col-span-5 xl:col-span-4 flex flex-col gap-5 border border-border/50 bg-card/20 rounded-xl p-5">
        <Skeleton class="h-3 w-40 mb-1 rounded" />
        <Skeleton class="h-32 w-full rounded" />
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
    <div class="grid grid-cols-1 lg:grid-cols-12 gap-8 items-start w-full">
      
      <!-- 1. LEFT COLUMN: Directory Index List (Notion Database style) -->
      <div class="lg:col-span-7 xl:col-span-8 flex flex-col">
        <h3 class="text-[10px] font-bold text-muted-foreground/80 tracking-wider uppercase mb-3 text-start">
          {m.operations_ui_exam_reports_guide_title()}
        </h3>
        
        <div class="flex flex-col">
          {#each reportItems as item (item.id)}
            <!-- svelte-ignore a11y_click_events_have_key_events -->
            <!-- svelte-ignore a11y_no_static_element_interactions -->
            <div 
              onclick={() => handleSelectReport(item.id)}
              class="group flex items-center justify-between py-2.5 px-3 border-b border-border/30 rounded-lg transition-colors duration-150 select-none cursor-pointer
                {selectedReportId === item.id ? 'bg-primary/10 border-primary/40' : 'hover:bg-muted/40'}"
            >
              <div class="flex items-center gap-3.5 min-w-0 text-start">
                <!-- Sleek Monolithic icon box -->
                <div class="h-8 w-8 rounded-lg flex items-center justify-center shrink-0 transition-colors duration-150
                  {selectedReportId === item.id ? 'bg-primary text-primary-foreground' : 'bg-muted/65 text-muted-foreground group-hover:bg-primary/10 group-hover:text-primary'}"
                >
                  <item.icon size={16} />
                </div>
                <div class="flex flex-col min-w-0">
                  <span class="text-xs font-bold text-foreground group-hover:text-primary transition-colors duration-150">
                    {item.title}
                  </span>
                  <span class="text-[9.5px] text-muted-foreground/90 leading-normal truncate mt-0.5 max-w-sm sm:max-w-md md:max-w-lg">
                    {item.desc}
                  </span>
                </div>
              </div>
              
              <div class="flex items-center gap-2 shrink-0">
                {#if item.badge}
                  <span class="text-[8.5px] font-mono font-medium text-muted-foreground/80 bg-muted/50 border border-border/40 px-2 py-0.5 rounded select-none">
                    {item.badge}
                  </span>
                {/if}
                <ArrowRight 
                  size={12} 
                  class="text-muted-foreground/40 opacity-0 group-hover:opacity-100 transition-opacity duration-150 rtl:rotate-180" 
                />
              </div>
            </div>
          {/each}
        </div>
      </div>

      <!-- 2. RIGHT COLUMN: Report Preview & Diagnostics (Linear Sidebar style) -->
      <div class="lg:col-span-5 xl:col-span-4 flex flex-col gap-5 border border-border/50 bg-card/10 rounded-xl p-5 select-none text-start">
        
        <!-- Sidebar Section: Selected Report Details & Action -->
        <div>
          <h4 class="text-[9.5px] font-bold text-muted-foreground/80 tracking-wider uppercase border-b border-border/30 pb-2 mb-3">
            {m.operations_ui_current_preview_and_print()}
          </h4>
          
          <div class="flex flex-col gap-3 bg-card/40 border border-border/40 p-4 rounded-xl text-start">
            <div class="flex items-center gap-3">
              <div class="h-8.5 w-8.5 rounded-lg bg-primary/10 text-primary flex items-center justify-center shrink-0">
                <selectedReport.icon size={18} />
              </div>
              <div class="flex flex-col min-w-0">
                <span class="text-xs font-bold text-foreground truncate">{selectedReport.title}</span>
                <span class="text-[9px] font-mono text-muted-foreground">{selectedReport.badge}</span>
              </div>
            </div>

            <p class="text-[10px] text-muted-foreground leading-relaxed">
              {selectedReport.desc}
            </p>

            <div class="pt-2 border-t border-border/30 flex items-center justify-between gap-2">
              <Button 
                variant="default" 
                size="sm" 
                class="w-full text-xs font-bold justify-center gap-2 cursor-pointer h-8.5"
                onclick={handlePrint}
              >
                <Printer size={14} />
                <span>{m.operations_ui_print_report_now()}</span>
              </Button>
            </div>
          </div>
        </div>

        <!-- Sidebar Section: KPI Summary -->
        <div>
          <h4 class="text-[9.5px] font-bold text-muted-foreground/80 tracking-wider uppercase border-b border-border/30 pb-2 mb-3">
            {m.operations_ui_print_center_indicators()}
          </h4>
          
          <div class="grid grid-cols-2 gap-4 pb-4 border-b border-border/30">
            {#each kpiStats as stat}
              <div class="flex flex-col text-start">
                <span class="text-[9.5px] text-muted-foreground/80">{stat.label}</span>
                <span class="text-sm font-bold text-foreground font-mono mt-0.5 leading-none">{stat.value}</span>
              </div>
            {/each}
          </div>
        </div>

        <!-- Sidebar Section: Orbis AI Advisor System Diagnostics -->
        <div class="flex flex-col gap-3">
          <div class="flex items-center gap-2 text-foreground font-semibold text-[11px] pb-1">
            <Sparkles size={13} class="text-primary animate-pulse" />
            <span>{m.operations_ui_advisor_diagnostics()}</span>
          </div>
          
          <div class="flex flex-col gap-2.5">
            {#each advisorInsights as insight (insight.id)}
              <div class="flex items-start gap-2.5 p-2 rounded-lg bg-card/30 border border-border/20 text-[10px] leading-relaxed text-muted-foreground">
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
        <div class="pt-2 border-t border-border/30 flex items-center justify-between text-[9.5px] text-muted-foreground">
          <a 
            href={localizeHref(`/${orgId}/templates`)}
            class="text-primary font-bold hover:underline flex items-center gap-1"
          >
            <span>{m.operations_ui_system_template_designer()}</span>
            <ArrowRight size={10} class="rtl:rotate-180" />
          </a>
          <span class="font-mono">System Templates Connected</span>
        </div>

      </div>

    </div>
  {/if}
</div>
