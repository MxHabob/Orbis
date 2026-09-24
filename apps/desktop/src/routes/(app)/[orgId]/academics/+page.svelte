<!--
@component AcademicsMainPage
Landing / dashboard overview page for Academics.
Designed with a premium Notion/Linear split layout.
Left side: Notion-style interactive database directory list.
Right side: Linear-style diagnostics panel with Orbis Advisor insights.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui/button";
import { Skeleton } from "@orbis/ui/skeleton";
import { Badge } from "@orbis/ui/badge";
import { AdjustmentsHorizontal, ArrowRight, BookOpen, Calendar, ChartBar, Grid, Info, InfoCircle, Sparkles, TaskList, UserBadgeCheck } from "@orbis/icons";
import { localizeHref } from "$lib/paraglide/runtime";

import {
  createClassesQuery,
  createSubjectsQuery,
  createPeriodsQuery,
} from "$lib/features/academics";
import { createPromotionJobsQuery } from "$lib/features/academics/promotion-queries";
import { academicContext } from "$lib/stores";
import { createExamSessionsQuery } from "$lib/features/exams/queries";
import { extractErrorMessage } from "$lib/api";

let { data }: { data: { orgId: string } } = $props();

// Dynamic Period ID
const activePeriodId = $derived(activePeriod ? String(activePeriod.id) : "");

// TanStack Queries
const classesQuery = createClassesQuery(() => data.orgId);
const subjectsQuery = createSubjectsQuery(() => data.orgId);
const periodsQuery = createPeriodsQuery(() => data.orgId);
const promotionJobsQuery = createPromotionJobsQuery(() => data.orgId);
const examSessionsQuery = createExamSessionsQuery(() => activePeriodId);

// Derived Data
const classes = $derived(classesQuery.data ?? []);
const subjects = $derived(subjectsQuery.data ?? []);
const periods = $derived(periodsQuery.data ?? []);
const currentYear = $derived(academicContext.currentYear);
const promotionJobs = $derived(promotionJobsQuery.data ?? []);
const examSessions = $derived(examSessionsQuery.data ?? []);

const isLoading = $derived(
  classesQuery.isLoading ||
    subjectsQuery.isLoading ||
    periodsQuery.isLoading ||
    promotionJobsQuery.isLoading ||
    examSessionsQuery.isLoading,
);

const error = $derived(
  classesQuery.error ||
    subjectsQuery.error ||
    periodsQuery.error ||
    promotionJobsQuery.error ||
    examSessionsQuery.error,
);

function refetchAll() {
  classesQuery.refetch();
  subjectsQuery.refetch();
  periodsQuery.refetch();
  promotionJobsQuery.refetch();
  examSessionsQuery.refetch();
}

const activeYearName = $derived(academicContext.yearLabelAr);

// Find active period / term
const activePeriod = $derived(
  periods.find((c) => c.status === "active") || periods[0],
);

const activePeriodName = $derived(
  activePeriod ? activePeriod.name || activePeriod.name : null,
);

// Grade entry status (based on active period/term)
const isGradeEntryOpen = $derived(
  activePeriod
    ? activePeriod.grade_entry_status === "open" ||
        activePeriod.is_grade_entry_open === 1
    : false,
);

// Dynamic counters for KPIs
const kpiStats = $derived([
  {
    label: m.common_card_active_classes(),
    value: currentYear?.class_count ?? classes.length,
  },
  {
    label: m.academics_overview_kpi_total_subjects(),
    value: currentYear?.subject_count ?? subjects.length,
  },
  {
    label: m.academics_overview_kpi_total_teachers(),
    value: currentYear?.teacher_count ?? 0,
  },
  {
    label: m.common_classes_enrolled_students(),
    value: currentYear?.student_count ?? 0,
  },
]);

// Navigation directories configuration mapping to active sub-routes
const directoryItems = $derived([
  {
    path: `/${data.orgId}/academics/classes`,
    title: m.common_search_group_classes(),
    desc: m.academics_overview_card_classes_desc(),
    icon: BookOpen,
    badge: m.academics_overview_card_classes_badge({ count: classes.length }),
  },
  {
    path: `/${data.orgId}/academics/schedule`,
    title: m.common_exams_detail_schedule(),
    desc: m.academics_overview_card_schedule_desc(),
    icon: Calendar,
    badge: m.academics_overview_card_schedule_badge(),
  },
  {
    path: `/${data.orgId}/academics/promotion`,
    title: m.academics_overview_card_promotion_title(),
    desc: m.academics_overview_card_promotion_desc(),
    icon: ChartBar,
    badge: m.common_promo_wizard_title(),
  },
  {
    path: `/${data.orgId}/academics/exams`, // Moved under academics
    title: m.academics_overview_card_exams_title(),
    desc: m.academics_overview_card_exams_desc(),
    icon: TaskList,
    badge: m.academics_overview_card_exams_badge(),
  },
]);

// Diagnostic Advisor Alerts
const advisorInsights = $derived.by(() => {
  const insights = [];

  // 1. Grade Entry Insight
  if (activePeriodName) {
    insights.push({
      id: "grade-entry",
      text: isGradeEntryOpen
        ? m.academics_overview_insight_grades_open({
            period: activePeriodName,
          })
        : m.academics_overview_insight_grades_closed(),
      variant: isGradeEntryOpen ? ("warning" as const) : ("neutral" as const),
      icon: UserBadgeCheck,
    });
  }

  // 2. Schedule Insight
  insights.push({
    id: "schedule",
    text:
      classes.length > 0
        ? m.academics_overview_insight_schedule_ok()
        : m.academics_overview_insight_schedule_warn(),
    variant: classes.length > 0 ? ("success" as const) : ("warning" as const),
    icon: Calendar,
  });

  // 3. Student Promotion Insight
  const activePromoJob = promotionJobs.find(
    (job) =>
      job.status === "dry_run" ||
      job.status === "promoting" ||
      job.status === "pending",
  );
  if (activePromoJob) {
    if (activePromoJob.status === "promoting") {
      insights.push({
        id: "promotion",
        text: m.academics_overview_insight_promo_running(),
        variant: "warning" as const,
        icon: ChartBar,
      });
    } else {
      insights.push({
        id: "promotion",
        text: m.academics_overview_insight_promo_ready({
          promoted: activePromoJob.promoted_count ?? 0,
          retained: activePromoJob.retained_count ?? 0,
        }),
        variant: "success" as const,
        icon: ChartBar,
      });
    }
  } else {
    insights.push({
      id: "promotion",
      text: m.academics_overview_insight_promo_none(),
      variant: "neutral" as const,
      icon: ChartBar,
    });
  }

  // 4. Exams Insight
  const activeExamSession = examSessions.find(
    (s: any) => s.status === "active" || s.status === "running",
  );
  if (activeExamSession) {
    insights.push({
      id: "exams-active",
      text: m.academics_overview_insight_exams_active({
        session: activeExamSession.name || "",
      }),
      variant: "success" as const,
      icon: TaskList,
    });
  } else {
    insights.push({
      id: "exams-none",
      text: m.academics_overview_insight_exams_none(),
      variant: "neutral" as const,
      icon: TaskList,
    });
  }

  return insights;
});
</script>

<div
  class="h-full w-full overflow-y-auto bg-background/30 scrollbar-hide p-6 select-none font-sans flex flex-col gap-6"
>
  <!-- Sleek Header with Active Year Indicator -->
  <div class="flex items-center justify-end border-b border-border/40 pb-4">
    <!-- Active Academic Year Pills -->
    <div class="flex items-center gap-2">
      {#if activeYearName}
        <Badge
          variant="outline"
          class="text-[9px] px-2 py-0.5 font-bold flex items-center gap-1.5 border-border/60 bg-card/60"
        >
          <Calendar size={10} class="text-primary" />
          <span>{activeYearName}</span>
        </Badge>
      {:else}
        <Badge
          variant="outline"
          class="text-[9px] px-2 py-0.5 text-muted-foreground border-dashed"
        >
          {m.academics_overview_no_active_year()}
        </Badge>
      {/if}

      {#if activePeriodName}
        <Badge
          variant="outline"
          class="text-[9px] px-2 py-0.5 font-bold flex items-center gap-1.5 border-border/60 bg-card/60"
        >
          <span
            class="h-1.5 w-1.5 rounded-full {isGradeEntryOpen
              ? 'bg-emerald-500 animate-pulse'
              : 'bg-rose-500'}"
          ></span>
          <span>{activePeriodName}</span>
        </Badge>
      {/if}
    </div>
  </div>

  {#if isLoading}
    <!-- Split Grid Loading Skeleton -->
    <div class="grid grid-cols-1 lg:grid-cols-12 gap-8 items-start w-full">
      <!-- Left Directory List Skeleton -->
      <div class="lg:col-span-7 xl:col-span-8 flex flex-col gap-3">
        <Skeleton class="h-4 w-32 mb-2 rounded" />
        {#each Array(7) as _}
          <div
            class="flex items-center justify-between p-3.5 border-b border-border/30"
          >
            <div class="flex items-center gap-3 w-full">
              <Skeleton class="h-8.5 w-8.5 rounded-lg" />
              <div class="flex flex-col gap-1.5 w-2/3">
                <Skeleton class="h-3 w-28 rounded" />
                <Skeleton class="h-2.5 w-full rounded" />
              </div>
            </div>
            <Skeleton class="h-4 w-12 rounded" />
          </div>
        {/each}
      </div>

      <!-- Right Diagnostics Sidebar Skeleton -->
      <div
        class="lg:col-span-5 xl:col-span-4 flex flex-col gap-5 border border-border/50 bg-card/20 rounded-xl p-5"
      >
        <Skeleton class="h-3 w-40 mb-1 rounded" />
        <div class="grid grid-cols-2 gap-4 pb-4 border-b border-border/40">
          {#each Array(4) as _}
            <div class="flex flex-col gap-1">
              <Skeleton class="h-2.5 w-16 rounded" />
              <Skeleton class="h-4 w-8 rounded" />
            </div>
          {/each}
        </div>
        <Skeleton class="h-3 w-32 mb-1 rounded" />
        <div class="flex flex-col gap-3">
          {#each Array(4) as _}
            <div class="flex items-start gap-2.5">
              <Skeleton class="h-4 w-4 rounded-full shrink-0" />
              <Skeleton class="h-3 w-full rounded" />
            </div>
          {/each}
        </div>
      </div>
    </div>
  {:else if error}
    <div
      class="flex flex-col items-center justify-center py-16 text-destructive border border-destructive/20 bg-destructive/5 rounded-xl gap-2"
    >
      <InfoCircle size={24} />
      <h3 class="font-semibold text-xs">{m.common_error_load_overview()}</h3>
      <p class="text-[10px] text-muted-foreground">
        {extractErrorMessage(error)}
      </p>
      <Button
        variant="outline"
        onclick={refetchAll}
        class="text-[10px] mt-2 h-8">{m.common_retry()}</Button
      >
    </div>
  {:else}
    <!-- Main Content Split Grid -->
    <div class="grid grid-cols-1 lg:grid-cols-12 gap-8 items-start w-full">
      <!-- 1. LEFT COLUMN: Directory Index List (Notion Database style) -->
      <div class="lg:col-span-7 xl:col-span-8 flex flex-col">
        <h3
          class="text-[10px] font-bold text-muted-foreground/80 tracking-wider uppercase mb-3 text-start"
        >
          {m.academics_overview_sections_title()}
        </h3>

        <div class="flex flex-col">
          {#each directoryItems as item (item.path)}
            <a
              href={localizeHref(item.path)}
              class="group flex items-center justify-between py-2.5 px-3 border-b border-border/30 hover:bg-muted/40 rounded-lg transition-colors duration-150 select-none cursor-pointer"
            >
              <div class="flex items-center gap-3.5 min-w-0 text-start">
                <!-- Sleek Monolithic icon box -->
                <div
                  class="h-8 w-8 rounded-lg bg-muted/65 text-muted-foreground flex items-center justify-center shrink-0 group-hover:bg-primary/10 group-hover:text-primary transition-colors duration-150"
                >
                  <item.icon size={16} />
                </div>
                <div class="flex flex-col min-w-0">
                  <span
                    class="text-xs font-bold text-foreground group-hover:text-primary transition-colors duration-150"
                  >
                    {item.title}
                  </span>
                  <span
                    class="text-[9.5px] text-muted-foreground/90 leading-normal truncate mt-0.5 max-w-sm sm:max-w-md md:max-w-lg"
                  >
                    {item.desc}
                  </span>
                </div>
              </div>

              <div class="flex items-center gap-2 shrink-0">
                {#if item.badge}
                  <span
                    class="text-[8.5px] font-mono font-medium text-muted-foreground/80 bg-muted/50 border border-border/40 px-2 py-0.5 rounded select-none"
                  >
                    {item.badge}
                  </span>
                {/if}
                <ArrowRight
                  size={12}
                  class="text-muted-foreground/40 opacity-0 group-hover:opacity-100 transition-opacity duration-150 rtl:rotate-180"
                />
              </div>
            </a>
          {/each}
        </div>
      </div>

      <!-- 2. RIGHT COLUMN: Academic Health & Diagnostics (Linear Sidebar style) -->
      <div
        class="lg:col-span-5 xl:col-span-4 flex flex-col gap-5 border border-border/50 bg-card/10 rounded-xl p-5 select-none text-start"
      >
        <!-- Sidebar Section: KPI Summary -->
        <div>
          <h4
            class="text-[9.5px] font-bold text-muted-foreground/80 tracking-wider uppercase border-b border-border/30 pb-2 mb-3"
          >
            {m.academics_overview_sidebar_title()}
          </h4>

          <div class="grid grid-cols-2 gap-4 pb-4 border-b border-border/30">
            {#each kpiStats as stat}
              <div class="flex flex-col text-start">
                <span class="text-[9.5px] text-muted-foreground/80"
                  >{stat.label}</span
                >
                <span
                  class="text-sm font-bold text-foreground font-mono mt-0.5 leading-none"
                  >{stat.value}</span
                >
              </div>
            {/each}
          </div>
        </div>

        <!-- Sidebar Section: Orbis AI Advisor System Diagnostics -->
        <div class="flex flex-col gap-3">
          <div
            class="flex items-center gap-2 text-foreground font-semibold text-[11px] pb-1"
          >
            <Sparkles size={13} class="text-primary animate-pulse" />
            <span>{m.academics_overview_advisor_title()}</span>
          </div>

          <div class="flex flex-col gap-2.5">
            {#each advisorInsights as insight (insight.id)}
              <div
                class="flex items-start gap-2.5 p-2 rounded-lg bg-card/30 border border-border/20 text-[10px] leading-relaxed text-muted-foreground"
              >
                <!-- Status icon or color dot -->
                <div class="shrink-0 mt-0.5">
                  {#if insight.variant === "warning"}
                    <span class="relative flex h-2 w-2">
                      <span
                        class="animate-ping absolute inline-flex h-full w-full rounded-full bg-amber-400 opacity-75"
                      ></span>
                      <span
                        class="relative inline-flex rounded-full h-2 w-2 bg-amber-500"
                      ></span>
                    </span>
                  {:else if insight.variant === "success"}
                    <span
                      class="inline-flex rounded-full h-2 w-2 bg-emerald-500"
                    ></span>
                  {:else}
                    <span
                      class="inline-flex rounded-full h-2 w-2 bg-muted-foreground/40"
                    ></span>
                  {/if}
                </div>

                <div class="flex flex-col min-w-0">
                  <span class="text-foreground/90 font-medium"
                    >{insight.text}</span
                  >
                </div>
              </div>
            {/each}
          </div>
        </div>

        <!-- System Information Footer -->
        <div
          class="pt-2 border-t border-border/30 flex items-center gap-2 text-[9px] text-muted-foreground/75"
        >
          <Info size={11} />
          <div class="flex items-center gap-1.5">
            <span>{activeYearName || m.common_nav_academics()}{#if activePeriodName} • {activePeriodName}{/if}</span>
          </div>
        </div>
      </div>
    </div>
  {/if}
</div>
