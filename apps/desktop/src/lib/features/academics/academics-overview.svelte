<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui/button";
import { Skeleton } from "@orbis/ui/skeleton";
import { Badge } from "@orbis/ui/badge";
import {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardAction,
  CardContent,
  CardFooter,
} from "@orbis/ui/card";
import {
  ArrowRight,
  BookOpen,
  Calendar,
  ChartBar,
  Info,
  InfoCircle,
  Sparkles,
  TaskList,
  UserBadgeCheck,
} from "@orbis/icons";
import { localizeHref } from "$lib/paraglide/runtime";

import {
  createSectionsQuery,
  createPeriodsQuery,
  createacademicCalendarsQuery,
} from "$lib/features/academics";
import { createprogressionJobsQuery } from "$lib/features/academics/promotion-queries";
import { academicContext, licenseStore } from "$lib/stores";
import { createExamSessionsQuery } from "$lib/features/exams/queries";
import { api, extractErrorMessage } from "$lib/api";
import { createQuery } from "@tanstack/svelte-query";

let { orgId }: { orgId: string } = $props();

// TanStack Queries
const yearsQuery = createacademicCalendarsQuery(() => orgId);
const sectionsQuery = createSectionsQuery(() => orgId);
const periodsQuery = createPeriodsQuery(() => orgId);
const progressionJobsQuery = createprogressionJobsQuery(() => orgId);
const teachersQuery = createQuery(() => ({
  queryKey: [
    "employees",
    orgId,
    { employee_type: "teacher", status: "active" },
  ],
  queryFn: () =>
    api.listEmployees({ employee_type: "teacher", status: "active" }),
  enabled: !!orgId,
  staleTime: 5 * 60 * 1000,
}));

// Derived Data
const years = $derived(yearsQuery.data ?? academicContext.allYears ?? []);
const sections = $derived(sectionsQuery.data ?? []);
const periods = $derived(periodsQuery.data ?? []);
const progressionJobs = $derived(progressionJobsQuery.data ?? []);
const teachersCount = $derived(teachersQuery.data?.length ?? 0);

// Find active year: check query data first, then academicContext, then fallback to first available
const currentYear = $derived(
  years.find((y) => y.status === "active" || y.is_active) ??
    academicContext.currentYear ??
    years[0] ??
    null,
);

const activeYearName = $derived(
  currentYear?.name_ar ||
    currentYear?.name ||
    academicContext.yearLabelAr ||
    "",
);

// Keep academicContext synced with the resolved active year
$effect(() => {
  if (currentYear && !academicContext.currentYear) {
    academicContext.currentYear = currentYear;
    academicContext.allYears = years;
  }
});

// Ensure global academicContext is resolved on mount if not ready
$effect(() => {
  if (orgId && !academicContext.isInitialized) {
    void academicContext.resolveContext(orgId);
  }
});

// Find active period / term
const activePeriod = $derived(
  periods.find(
    (c) =>
      c.status === "active" ||
      c.status === "active_teaching" ||
      c.status === "grade_submission",
  ) || periods[0],
);

const activePeriodId = $derived(activePeriod ? String(activePeriod.id) : "");
const activePeriodName = $derived(activePeriod ? activePeriod.name : null);

const canViewExams = $derived(licenseStore.isFeatureEnabled("exam_tools"));
const examSessionsQuery = createExamSessionsQuery(() =>
  canViewExams ? activePeriodId : undefined,
);
const examSessions = $derived(examSessionsQuery.data ?? []);

const isLoading = $derived(
  yearsQuery.isLoading ||
    sectionsQuery.isLoading ||
    periodsQuery.isLoading ||
    progressionJobsQuery.isLoading ||
    teachersQuery.isLoading ||
    (canViewExams && examSessionsQuery.isLoading),
);

const error = $derived.by(() => {
  const queryError =
    yearsQuery.error ||
    sectionsQuery.error ||
    periodsQuery.error ||
    progressionJobsQuery.error ||
    teachersQuery.error ||
    (canViewExams ? examSessionsQuery.error : null);
  return queryError ? extractErrorMessage(queryError) : null;
});

function refetchAll() {
  yearsQuery.refetch();
  sectionsQuery.refetch();
  periodsQuery.refetch();
  progressionJobsQuery.refetch();
  teachersQuery.refetch();
  if (canViewExams) examSessionsQuery.refetch();
  academicContext.resolveContext(orgId);
}

// Grade entry status (based on active period/term)
const isGradeEntryOpen = $derived(
  activePeriod
    ? activePeriod.status === "grade_submission" ||
        activePeriod.status === "active_teaching" ||
        activePeriod.status === "open"
    : false,
);

// Dynamic counters for KPIs
const kpiStats = $derived([
  {
    label: m.common_search_group_classes(),
    value: sections.length,
  },
  {
    label: m.academics_overview_kpi_total_teachers(),
    value: teachersCount,
  },
]);

// Navigation directories configuration mapping to active sub-routes
const directoryItems = $derived([
  {
    path: `/${orgId}/academics/sections`,
    title: m.common_search_group_classes(),
    desc: m.academics_overview_card_classes_desc(),
    icon: BookOpen,
    badge: String(sections.length),
  },
  {
    path: `/${orgId}/academics/schedule`,
    title: m.common_exams_detail_schedule(),
    desc: m.academics_overview_card_schedule_desc(),
    icon: Calendar,
    badge: m.academics_overview_card_schedule_badge(),
  },
  {
    path: `/${orgId}/academics/promotion`,
    title: m.academics_overview_card_promotion_title(),
    desc: m.academics_overview_card_promotion_desc(),
    icon: ChartBar,
    badge: m.common_promo_wizard_title(),
  },
  {
    path: `/${orgId}/academics/exams`,
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
      sections.length > 0
        ? m.academics_overview_insight_schedule_ok()
        : m.academics_overview_insight_schedule_warn(),
    variant: sections.length > 0 ? ("success" as const) : ("warning" as const),
    icon: Calendar,
  });

  // 3. Student Promotion Insight
  const activePromoJob = progressionJobs.find(
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

  // Exam data is optional on this overview and must not fail the free plan.
  if (canViewExams) {
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
  }

  return insights;
});
</script>

<div
  class="h-full w-full overflow-y-auto bg-background/30 scrollbar-hide p-6 select-none font-sans flex flex-col gap-6"
>
  <!-- Sleek Header with Active Year Indicator -->
  <div class="flex items-center justify-between border-b border-border/40 pb-4">
    <div class="flex flex-col text-start">
      <h2 class="text-base font-bold text-foreground">
        {m.common_nav_academics()}
      </h2>
      <p class="text-xs text-muted-foreground">
        {m.shared_ui_academic_overview_subtitle()}
      </p>
    </div>

    <!-- Active Academic Year Pills -->
    <div class="flex items-center gap-2">
      {#if activeYearName}
        <Badge
          variant="outline"
          class="text-[9px] px-2 py-0.5 font-bold flex items-center gap-1.5 border-border/60 bg-card"
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
          class="text-[9px] px-2 py-0.5 font-bold flex items-center gap-1.5 border-border/60 bg-card"
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
    <div class="grid grid-cols-1 lg:grid-cols-12 gap-6 items-start w-full">
      <!-- Left Directory List Skeleton -->
      <div class="lg:col-span-7 xl:col-span-8 flex flex-col gap-3">
        <Skeleton class="h-4 w-32 mb-2 rounded" />
        {#each Array(5) as _}
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
      <div class="lg:col-span-5 xl:col-span-4">
        <Skeleton class="h-72 w-full rounded-xl" />
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
        class="text-[10px] mt-2 h-8"
      >
        {m.common_retry()}
      </Button>
    </div>
  {:else}
    <!-- Main Content Split Grid -->
    <div class="grid grid-cols-1 lg:grid-cols-12 gap-6 items-start w-full">
      <!-- 1. LEFT COLUMN: Directory Index List Card -->
      <Card class="lg:col-span-7 xl:col-span-8 flex flex-col overflow-hidden">
        <CardHeader class="py-3 px-4">
          <CardTitle class="text-xs font-bold uppercase tracking-wider text-foreground">
            {m.academics_overview_sections_title()}
          </CardTitle>
          <CardDescription class="text-[10px]">
            {m.shared_ui_academic_operations_description()}
          </CardDescription>
        </CardHeader>

        <CardContent class="p-0 divide-y divide-border/40">
          {#each directoryItems as item (item.path)}
            <a
              href={localizeHref(item.path)}
              class="group flex items-center justify-between p-3.5 hover:bg-muted/40 transition-colors duration-150 select-none cursor-pointer"
            >
              <div class="flex items-center gap-3.5 min-w-0 text-start">
                <div
                  class="h-8.5 w-8.5 rounded-lg bg-primary/10 text-primary flex items-center justify-center shrink-0 group-hover:bg-primary group-hover:text-primary-foreground transition-colors duration-150"
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
                    class="text-[10px] text-muted-foreground leading-normal truncate mt-0.5 max-w-sm sm:max-w-md md:max-w-lg"
                  >
                    {item.desc}
                  </span>
                </div>
              </div>

              <div class="flex items-center gap-2.5 shrink-0">
                {#if item.badge}
                  <span
                    class="text-[9px] font-mono font-medium text-muted-foreground bg-muted/60 border border-border/40 px-2 py-0.5 rounded select-none"
                  >
                    {item.badge}
                  </span>
                {/if}
                <ArrowRight
                  size={13}
                  class="text-muted-foreground/40 opacity-0 group-hover:opacity-100 transition-opacity duration-150 rtl:rotate-180"
                />
              </div>
            </a>
          {/each}
        </CardContent>
      </Card>

      <!-- 2. RIGHT COLUMN: Academic Health & Diagnostics Card -->
      <Card class="lg:col-span-5 xl:col-span-4 flex flex-col select-none text-start overflow-hidden">
        <CardHeader class="py-3 px-4">
          <CardTitle class="text-xs font-bold uppercase tracking-wider text-foreground">
            {m.academics_overview_sidebar_title()}
          </CardTitle>
          <CardDescription class="text-[10px]">
            {m.shared_ui_academic_ai_insights_description()}
          </CardDescription>
        </CardHeader>

        <CardContent class="p-4 space-y-4">
          <!-- Sidebar Section: KPI Summary -->
          <div class="grid grid-cols-2 gap-3 pb-3 border-b border-border/40">
            {#each kpiStats as stat}
              <div class="flex flex-col p-2 rounded-lg bg-card border border-border/40 text-start">
                <span class="text-[9px] text-muted-foreground font-semibold">
                  {stat.label}
                </span>
                <span class="text-base font-bold text-foreground font-mono mt-0.5 leading-none">
                  {stat.value}
                </span>
              </div>
            {/each}
          </div>

          <!-- Sidebar Section: Orbis AI Advisor System Diagnostics -->
          <div class="flex flex-col gap-2.5">
            <div class="flex items-center gap-1.5 text-foreground font-bold text-[11px]">
              <Sparkles size={13} class="text-primary animate-pulse" />
              <span>{m.academics_overview_advisor_title()}</span>
            </div>

            <div class="flex flex-col gap-2">
              {#each advisorInsights as insight (insight.id)}
                <div
                  class="flex items-start gap-2.5 p-2 rounded-lg bg-muted/30 border border-border/30 text-[10px] leading-relaxed text-muted-foreground"
                >
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
                    <span class="text-foreground/90 font-medium">
                      {insight.text}
                    </span>
                  </div>
                </div>
              {/each}
            </div>
          </div>
        </CardContent>

        <!-- System Information Footer -->
        <CardFooter class="border-t border-border/40 py-2.5 px-4 flex items-center gap-2 text-[9px] text-muted-foreground">
          <Info size={11} />
          <div class="flex items-center gap-1.5">
            <span>{activeYearName || m.common_nav_academics()}{#if activePeriodName} • {activePeriodName}{/if}</span>
          </div>
        </CardFooter>
      </Card>
    </div>
  {/if}
</div>
