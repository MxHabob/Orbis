<!--
@component ExamsOverview
Landing / dashboard overview page for Examination Management.
Designed with the exact Notion/Linear split layout design pattern of the system.
Left side: Notion-style interactive database directory list.
Right side: Linear-style diagnostics panel with Orbis Advisor insights.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { readable } from "svelte/store";
import { Button } from "@orbis/ui/button";
import { Skeleton } from "@orbis/ui/skeleton";
import { Badge } from "@orbis/ui/badge";
import { AdjustmentsHorizontal, ArrowRight, Calendar, DocumentPage, Grid, Home, Info, InfoCircle, Lock, LockClosed, Plus, Sparkles, TaskList, UserBadgeCheck } from "@orbis/icons";
import { localizeHref } from "$lib/paraglide/runtime";
import { createQuery } from "@tanstack/svelte-query";
import { api, extractErrorMessage } from "$lib/api";
import { academicContext } from "$lib/stores";
import { createPeriodsQuery } from "$lib/features/academics";
import SessionCreateSheet from "$lib/features/exams/components/session-create-sheet.svelte";

let { data }: { data: { orgId: string } } = $props();

// Modal trigger state
let isCreateSheetOpen = $state(false);

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

// Grade Entry Status
const isGradeEntryOpen = $derived(
  activePeriod
    ? activePeriod.grade_entry_status === "open" ||
        activePeriod.is_grade_entry_open === 1
    : false,
);

// Exam Sessions Query
const examSessionsQuery = createQuery(() => ({
  queryKey: ["exam-sessions", data.orgId, activePeriodId],
  queryFn: () => api.listExamSessions(activePeriodId),
  enabled: !!data.orgId && !!activePeriodId,
  staleTime: 2 * 60 * 1000,
}));

const examSessions = $derived((examSessionsQuery.data as any[]) ?? []);

const isLoading = $derived(
  periodsQuery.isLoading || examSessionsQuery.isLoading,
);

const error = $derived(periodsQuery.error || examSessionsQuery.error);

function refetchAll() {
  periodsQuery.refetch();
  examSessionsQuery.refetch();
}

// Dynamic KPI Stats
const kpiStats = $derived([
  {
    label: m.academics_exams_center_kpi_sessions(),
    value: examSessions.length,
  },
  {
    label: m.academics_exams_center_kpi_active(),
    value: examSessions.filter(
      (s) =>
        s.status === "active" ||
        s.status === "running" ||
        s.status === "ongoing",
    ).length,
  },
  {
    label: m.academics_exams_center_kpi_grades(),
    value: isGradeEntryOpen
      ? m.academics_exams_center_open()
      : m.academics_exams_center_locked(),
  },
  {
    label: m.academics_exams_center_kpi_control(),
    value:
      examSessions.length > 0
        ? m.academics_exams_center_ready()
        : m.academics_exams_center_disabled(),
  },
]);

// Notion-style Directory Items
const directoryItems = $derived([
  {
    path: `/${data.orgId}/academics/exams/sessions`,
    title: m.common_exams_sessions_title(),
    desc: m.academics_exams_center_sessions_desc(),
    icon: TaskList,
    badge: m.academics_exams_center_sessions_badge({
      count: examSessions.length,
    }),
  },
  {
    path: `/${data.orgId}/academics/exams/control`,
    title: m.academics_exams_center_control_title(),
    desc: m.academics_exams_center_control_desc(),
    icon: UserBadgeCheck,
    badge: m.academics_exams_center_control_badge(),
  },
  {
    path: `/${data.orgId}/academics/exams/sessions`,
    title: m.academics_exams_center_halls_title(),
    desc: m.academics_exams_center_halls_desc(),
    icon: Home,
    badge: m.academics_exams_center_halls_badge(),
  },
  {
    path: `/${data.orgId}/academics/exams/reports`,
    title: m.academics_exams_center_reports_title(),
    desc: m.academics_exams_center_reports_desc(),
    icon: DocumentPage,
    badge: m.academics_exams_center_reports_badge(),
  },
]);

// Advisor Diagnostic Insights
const advisorInsights = $derived.by(() => {
  const insights = [];

  // 1. Grade Entry Lock Insight
  insights.push({
    id: "grade-lock",
    text: isGradeEntryOpen
      ? m.academics_exams_center_grade_open_insight()
      : m.academics_exams_center_grade_locked_insight(),
    variant: isGradeEntryOpen ? ("warning" as const) : ("success" as const),
    icon: isGradeEntryOpen ? Lock : LockClosed,
  });

  // 2. Exam Sessions Insight
  if (examSessions.length > 0) {
    insights.push({
      id: "sessions-active",
      text: m.academics_exams_center_sessions_active_insight({
        count: examSessions.length,
      }),
      variant: "success" as const,
      icon: TaskList,
    });
  } else {
    insights.push({
      id: "sessions-none",
      text: m.academics_exams_center_sessions_empty_insight(),
      variant: "neutral" as const,
      icon: TaskList,
    });
  }

  // 3. Control & Seating Insight
  insights.push({
    id: "seating-ready",
    text: m.academics_exams_center_seating_ready_insight(),
    variant: "success" as const,
    icon: UserBadgeCheck,
  });

  return insights;
});
</script>

<div class="h-full w-full overflow-y-auto bg-background/30 scrollbar-hide p-6 select-none font-sans flex flex-col gap-6">
  
  <!-- Sleek Header with Active Year & Term Indicators -->
  <div class="flex items-center justify-between border-b border-border/40 pb-4">
    <div class="flex items-center gap-3 text-start">
      <div class="h-8.5 w-8.5 rounded-lg bg-primary/10 text-primary flex items-center justify-center shrink-0 font-bold text-xs">
        <TaskList size={18} />
      </div>
      <div class="flex flex-col">
        <h2 class="text-xs font-bold text-foreground tracking-wide uppercase">
          {m.academics_exams_center_title()}
        </h2>
        <span class="text-[9.5px] text-muted-foreground/90 mt-0.5">
          {m.academics_exams_center_description()}
        </span>
      </div>
    </div>

    <!-- Active Academic Year & Term Pills -->
    <div class="flex items-center gap-2">
      {#if activeYearName}
        <Badge variant="outline" class="text-[9px] px-2 py-0.5 font-bold flex items-center gap-1.5 border-border/60 bg-card/60">
          <Calendar size={10} class="text-primary" />
          <span>{activeYearName}</span>
        </Badge>
      {:else}
        <Badge variant="outline" class="text-[9px] px-2 py-0.5 text-muted-foreground border-dashed">
          {m.academics_exams_center_no_active_year()}
        </Badge>
      {/if}
      
      {#if activePeriodName}
        <Badge variant="outline" class="text-[9px] px-2 py-0.5 font-bold flex items-center gap-1.5 border-border/60 bg-card/60">
          <span class="h-1.5 w-1.5 rounded-full {isGradeEntryOpen ? 'bg-amber-500 animate-pulse' : 'bg-emerald-500'}"></span>
          <span>{activePeriodName}</span>
        </Badge>
      {/if}

      <Button 
        variant="default" 
        size="sm" 
        class="text-[10px] font-bold px-3 h-7.5 rounded-md gap-1.5 cursor-pointer ms-2"
        onclick={() => isCreateSheetOpen = true}
      >
        <Plus size={13} />
        <span>{m.academics_exams_center_add_session()}</span>
      </Button>
    </div>
  </div>

  {#if isLoading}
    <!-- Split Grid Loading Skeleton -->
    <div class="grid grid-cols-1 lg:grid-cols-12 gap-8 items-start w-full">
      <!-- Left Directory List Skeleton -->
      <div class="lg:col-span-7 xl:col-span-8 flex flex-col gap-3">
        <Skeleton class="h-4 w-32 mb-2 rounded" />
        {#each Array(4) as _}
          <div class="flex items-center justify-between p-3.5 border-b border-border/30">
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
      <div class="lg:col-span-5 xl:col-span-4 flex flex-col gap-5 border border-border/50 bg-card/20 rounded-xl p-5">
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
          {#each Array(3) as _}
            <div class="flex items-start gap-2.5">
              <Skeleton class="h-4 w-4 rounded-full shrink-0" />
              <Skeleton class="h-3 w-full rounded" />
            </div>
          {/each}
        </div>
      </div>
    </div>
  {:else if error}
    <div class="flex flex-col items-center justify-center py-16 text-destructive border border-destructive/20 bg-destructive/5 rounded-xl gap-2">
      <InfoCircle size={24} />
      <h3 class="font-semibold text-xs">{m.academics_exams_center_error_title()}</h3>
      <p class="text-[10px] text-muted-foreground">{extractErrorMessage(error)}</p>
      <Button variant="outline" onclick={refetchAll} class="text-[10px] mt-2 h-8">{m.academics_exams_center_retry()}</Button>
    </div>
  {:else}
    <!-- Main Content Split Grid -->
    <div class="grid grid-cols-1 lg:grid-cols-12 gap-8 items-start w-full">
      
      <!-- 1. LEFT COLUMN: Directory Index List (Notion Database style) -->
      <div class="lg:col-span-7 xl:col-span-8 flex flex-col">
        <h3 class="text-[10px] font-bold text-muted-foreground/80 tracking-wider uppercase mb-3 text-start">
          {m.academics_exams_center_directory_title()}
        </h3>
        
        <div class="flex flex-col">
          {#each directoryItems as item (item.path + item.title)}
            <a 
              href={localizeHref(item.path)} 
              class="group flex items-center justify-between py-2.5 px-3 border-b border-border/30 hover:bg-muted/40 rounded-lg transition-colors duration-150 select-none cursor-pointer"
            >
              <div class="flex items-center gap-3.5 min-w-0 text-start">
                <!-- Sleek Monolithic icon box -->
                <div class="h-8 w-8 rounded-lg bg-muted/65 text-muted-foreground flex items-center justify-center shrink-0 group-hover:bg-primary/10 group-hover:text-primary transition-colors duration-150">
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
            </a>
          {/each}
        </div>
      </div>

      <!-- 2. RIGHT COLUMN: Academic Health & Diagnostics (Linear Sidebar style) -->
      <div class="lg:col-span-5 xl:col-span-4 flex flex-col gap-5 border border-border/50 bg-card/10 rounded-xl p-5 select-none text-start">
        
        <!-- Sidebar Section: KPI Summary -->
        <div>
          <h4 class="text-[9.5px] font-bold text-muted-foreground/80 tracking-wider uppercase border-b border-border/30 pb-2 mb-3">
            {m.academics_exams_center_kpi_control()}
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
            <span>{m.academics_exams_center_advisor()}</span>
          </div>
          
          <div class="flex flex-col gap-2.5">
            {#each advisorInsights as insight (insight.id)}
              <div class="flex items-start gap-2.5 p-2 rounded-lg bg-card/30 border border-border/20 text-[10px] leading-relaxed text-muted-foreground">
                
                <!-- Status icon or color dot -->
                <div class="shrink-0 mt-0.5">
                  {#if insight.variant === 'warning'}
                    <span class="relative flex h-2 w-2">
                      <span class="animate-ping absolute inline-flex h-full w-full rounded-full bg-amber-400 opacity-75"></span>
                      <span class="relative inline-flex rounded-full h-2 w-2 bg-amber-500"></span>
                    </span>
                  {:else if insight.variant === 'success'}
                    <span class="inline-flex rounded-full h-2 w-2 bg-emerald-500"></span>
                  {:else}
                    <span class="inline-flex rounded-full h-2 w-2 bg-muted-foreground/40"></span>
                  {/if}
                </div>

                <div class="flex flex-col min-w-0">
                  <span class="text-foreground/90 font-medium">{insight.text}</span>
                </div>
              </div>
            {/each}
          </div>
        </div>

        <!-- System Information Footer -->
        <div class="pt-2 border-t border-border/30 flex items-center gap-2 text-[9px] text-muted-foreground/75">
          <Info size={11} />
          <span>{m.academics_exams_center_system_status()}</span>
        </div>

      </div>

    </div>
  {/if}
</div>

<!-- Exam Session Creation Side Sheet -->
<SessionCreateSheet
  bind:open={isCreateSheetOpen}
  orgId={data.orgId}
  onSuccess={refetchAll}
/>
