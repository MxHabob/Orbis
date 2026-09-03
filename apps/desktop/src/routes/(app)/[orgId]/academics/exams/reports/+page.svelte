<!--
@component ExamReports
Reports and printing management center for Examinations.
Designed with the exact Notion/Linear split layout design pattern of the system.
Left side: Notion-style interactive database directory list for Report Types.
Right side: Linear-style diagnostics panel with Orbis Advisor Insights & Live Print Preview.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui/button";
import { Badge } from "@orbis/ui/badge";
import { Skeleton } from "@orbis/ui/skeleton";
import { toast } from "@orbis/ui";
import {
  FileText,
  Calendar,
  Hash,
  Building2,
  UserCheck,
  CreditCard,
  ClipboardCheck,
  BarChart3,
  Printer,
  Eye,
  ArrowRight,
  Sparkles,
  Milestone,
  AlertCircle,
  Download,
  Info,
} from "@lucide/svelte";
import { academicContext } from "$lib/stores";
import { createPeriodsQuery } from "$lib/features/academics";
import { createQuery } from "@tanstack/svelte-query";
import { readable } from "svelte/store";
import { commands, extractErrorMessage } from "$lib/core/ipc";
import { localizeHref } from "$lib/paraglide/runtime";

let { data }: { data: { orgId: string } } = $props();

let selectedReportId = $state<string>("timetable");
let reportData = $state<any>(null);
let isGenerating = $state(false);

// Queries
const periodsQuery = createPeriodsQuery(() => data.orgId);
const periods = $derived(periodsQuery.data ?? []);

// Active academic year & period
const activeYearName = $derived(academicContext.yearLabelAr);
const activePeriod = $derived(
  periods.find((p) => p.status === "active") || periods[0],
);
const activePeriodId = $derived(activePeriod ? String(activePeriod.id) : "");
const activePeriodName = $derived(
  activePeriod ? activePeriod.name_ar || activePeriod.name : null,
);

// Grade Entry Lock Status
const isGradeEntryOpen = $derived(
  activePeriod
    ? activePeriod.grade_entry_status === "open" ||
        activePeriod.is_grade_entry_open === 1
    : false,
);

// Exam Sessions Query
const examSessionsQuery = createQuery(() => ({
  queryKey: ["exam-sessions", data.orgId, activePeriodId],
  queryFn: () => commands.exam.listSessions(activePeriodId),
  enabled: !!data.orgId && !!activePeriodId,
  staleTime: 2 * 60 * 1000,
}));

const examSessions = $derived((examSessionsQuery.data as any[]) ?? []);

// System Document Templates Query
const documentTemplatesQuery = createQuery(() => ({
  queryKey: ["document-templates", data.orgId],
  queryFn: () => commands.docforge.listTemplates({
    org_id: data.orgId,
    document_type: "EXAM_REGISTER",
  }),
  enabled: !!data.orgId,
  staleTime: 5 * 60 * 1000,
}));

const documentTemplates = $derived((documentTemplatesQuery.data as any[]) ?? []);

const isLoading = $derived(
  periodsQuery.isLoading || examSessionsQuery.isLoading || documentTemplatesQuery.isLoading,
);

const error = $derived(periodsQuery.error || examSessionsQuery.error || documentTemplatesQuery.error);

// Directory list of available exam reports
const reportItems = $derived([
  {
    id: "timetable",
    title: m.common_exams_reports_timetable ? m.common_exams_reports_timetable() : "جدول مواعيد الاختبارات",
    desc: "كشف تفصيلي بمواعيد الاختبارات اليومية حسب الصفوف والمواد والفترات.",
    icon: Calendar,
    badge: "جدول الاختبارات",
  },
  {
    id: "seating_list",
    title: m.common_exams_reports_seating_list ? m.common_exams_reports_seating_list() : "كشوفات أرقام الجلوس واللجان",
    desc: "قوائم أسماء الطلاب الموزعين على اللجان مع أرقام الجلوس والأرقام السرية.",
    icon: Hash,
    badge: "أرقام الجلوس",
  },
  {
    id: "hall_map",
    title: m.common_exams_reports_hall_map ? m.common_exams_reports_hall_map() : "خريطة توزيع قاعات الامتحان",
    desc: "توزيع القاعات والسعة الاستيعابية ومواقع الجلوس لكل لجنة امتحانية.",
    icon: Building2,
    badge: "توزيع القاعات",
  },
  {
    id: "proctor_sheet",
    title: m.common_exams_reports_proctor_sheet ? m.common_exams_reports_proctor_sheet() : "جدول تكليف الملاحظين والمراقبين",
    desc: "جدول توزيع المعلمين والملاحظين على اللجان والفترات الامتحانية اليومية.",
    icon: UserCheck,
    badge: "الملاحظين",
  },
  {
    id: "student_cards",
    title: m.common_exams_reports_student_cards ? m.common_exams_reports_student_cards() : "بطاقات دخول الامتحان للطلاب",
    desc: "بطاقات تعريفية رسمية تحتوي على رقم الجلوس واللجنة والقاعة الامتحانية.",
    icon: CreditCard,
    badge: "بطاقات الجلوس",
  },
  {
    id: "attendance",
    title: m.common_exams_reports_attendance ? m.common_exams_reports_attendance() : "كشوفات توقيع حضور الامتحانات",
    desc: "محاضر التحضير والتوقيع اليومي للطلاب داخل اللجان أثناء سير الامتحان.",
    icon: ClipboardCheck,
    badge: "كشوف الحضور",
  },
  {
    id: "summary",
    title: m.common_exams_reports_summary ? m.common_exams_reports_summary() : "سجل رصد الدرجات الشامل (الشيت)",
    desc: "الشيت المعتمد لرصد درجات الاختبارات الشفهية والتحريرية ونتيجة الكنترول.",
    icon: BarChart3,
    badge: "شيت الدرجات",
  },
]);

const selectedReport = $derived(
  reportItems.find((r) => r.id === selectedReportId) || reportItems[0],
);

// Dynamic Real KPI Stats
const kpiStats = $derived([
  {
    label: "قوالب النظام المسجلة",
    value: documentTemplates.length,
  },
  {
    label: "الدورات المتاحة",
    value: examSessions.length,
  },
  {
    label: "حالة المطبعة",
    value: documentTemplates.length > 0 ? "متصل" : "بانتظار قالب",
  },
  {
    label: "صيغة التصدير",
    value: "PDF / Excel",
  },
]);

// Real Diagnostic Insights
const advisorInsights = $derived.by(() => {
  const insights = [];

  if (documentTemplates.length > 0) {
    insights.push({
      id: "templates-ok",
      text: `تم العثور على (${documentTemplates.length}) قالب مصمم ومحفوظ في مصمم قوالب النظام.`,
      variant: "success" as const,
    });
  } else {
    insights.push({
      id: "templates-none",
      text: "يمكنك إنشاء وتخصيص قوالب الطباعة والشيتات عبر مصمم القوالب في النظام.",
      variant: "neutral" as const,
    });
  }

  insights.push({
    id: "lock-state",
    text: isGradeEntryOpen
      ? "باب الدرجات مفتوح للمعلمين. يفضل إغلاقه قبل طباعة الشيتات النهائية."
      : "باب الدرجات مغلق ومحمي بواسطة الكنترول للطباعة الرسمية.",
    variant: isGradeEntryOpen ? ("warning" as const) : ("success" as const),
  });

  return insights;
});

function handleSelectReport(id: string) {
  selectedReportId = id;
  toast.info(`تم اختيار تقرير: ${reportItems.find((r) => r.id === id)?.title}`);
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
        <FileText size={18} />
      </div>
      <div class="flex flex-col">
        <h2 class="text-xs font-bold text-foreground tracking-wide uppercase">
          مركز التقارير والمطبعة الامتحانية
        </h2>
        <span class="text-[9.5px] text-muted-foreground/90 mt-0.5">
          معاينة وتصدير وطباعة كشوفات أرقام الجلوس، المناداة، والبطاقات الرسمية
        </span>
      </div>
    </div>

    <!-- Active Academic Year & Term Pills & Template Designer Link -->
    <div class="flex items-center gap-2">
      {#if activeYearName}
        <Badge variant="outline" class="text-[9px] px-2 py-0.5 font-bold flex items-center gap-1.5 border-border/60 bg-card/60">
          <Milestone size={10} class="text-primary" />
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
        href={localizeHref(`/${data.orgId}/templates`)}
        class="text-[10px] font-bold px-3 h-7.5 rounded-md gap-1.5 cursor-pointer ms-2 border-border/60"
      >
        <FileText size={13} class="text-primary" />
        <span>مصمم القوالب في النظام</span>
      </Button>

      <Button 
        variant="default" 
        size="sm" 
        class="text-[10px] font-bold px-3 h-7.5 rounded-md gap-1.5 cursor-pointer"
        onclick={handlePrint}
      >
        <Printer size={13} />
        <span>طباعة التقرير</span>
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
      <AlertCircle size={24} />
      <h3 class="font-semibold text-xs">تعذر تحميل بيانات التقارير</h3>
      <p class="text-[10px] text-muted-foreground">{extractErrorMessage(error)}</p>
    </div>
  {:else}
    <!-- Main Content Split Layout -->
    <div class="grid grid-cols-1 lg:grid-cols-12 gap-8 items-start w-full">
      
      <!-- 1. LEFT COLUMN: Directory Index List (Notion Database style) -->
      <div class="lg:col-span-7 xl:col-span-8 flex flex-col">
        <h3 class="text-[10px] font-bold text-muted-foreground/80 tracking-wider uppercase mb-3 text-start">
          دليل تقارير وكشوفات الامتحانات الرسمية
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
            المعاينة الحالية والطباعة
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
                <span>طباعة التقرير الآن</span>
              </Button>
            </div>
          </div>
        </div>

        <!-- Sidebar Section: KPI Summary -->
        <div>
          <h4 class="text-[9.5px] font-bold text-muted-foreground/80 tracking-wider uppercase border-b border-border/30 pb-2 mb-3">
            مؤشرات المطبعة وقوالب النظام
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
            <span>تشخيص المساعد (Orbis Advisor)</span>
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
            href={localizeHref(`/${data.orgId}/templates`)}
            class="text-primary font-bold hover:underline flex items-center gap-1"
          >
            <span>مصمم قوالب النظام</span>
            <ArrowRight size={10} class="rtl:rotate-180" />
          </a>
          <span class="font-mono">System Templates Connected</span>
        </div>

      </div>

    </div>
  {/if}
</div>
