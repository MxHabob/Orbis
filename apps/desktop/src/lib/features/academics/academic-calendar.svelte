<!--
@component AcademicCalendar
Visual Academic Calendar providing Gantt timeline, term breakdown,
progress tracking, and academic cycle status for the school year.
-->
<script lang="ts">
import { api, extractErrorMessage, queryKeys } from "$lib/api";
import { formatDate } from "$lib/core/i18n";
import * as m from "$lib/paraglide/messages";
import { academicContext, confirmStore } from "$lib/stores";
import {
  createacademicCalendarsQuery,
  createPeriodsQuery,
} from "$lib/features/academics/queries";
import type { AcademicPeriodDto, academicCalendarDto } from "@orbis/sdk";
import { useQueryClient } from "@tanstack/svelte-query";
import { LoadingIndicator, toast } from "@orbis/ui";
import { Badge } from "@orbis/ui/badge";
import { Button } from "@orbis/ui/button";
import { Skeleton } from "@orbis/ui/skeleton";
import {
  AcademicCap,
  Calendar,
  ChartBar,
  ChevronDown,
  ChevronRight,
  ClockRotateRight,
  Pencil,
  Plus,
  Refresh,
} from "@orbis/icons";
import YearCreateSheet from "$lib/features/settings/year-create-sheet.svelte";
import PeriodCreateSheet from "$lib/features/academics/components/period-create-sheet.svelte";
import RelatedActions, {
  type ActionGroup,
} from "$lib/shared/components/related-actions.svelte";
import { fade } from "svelte/transition";

let { orgId }: { orgId: string } = $props();

const queryClient = useQueryClient();

// Queries
const yearsQuery = createacademicCalendarsQuery(() => orgId);
const periodsQuery = createPeriodsQuery(() => orgId);

// Derived state from queries
const years = $derived(yearsQuery.data ?? []);
const periods = $derived(periodsQuery.data ?? []);
const isLoading = $derived(yearsQuery.isLoading || periodsQuery.isLoading);
const isRefreshing = $derived(yearsQuery.isFetching || periodsQuery.isFetching);

let selectedYearId = $state<string>("");

// Select active year by default, or first year
$effect(() => {
  if (
    years.length > 0 &&
    (!selectedYearId || !years.some((y) => y.id === selectedYearId))
  ) {
    const active = years.find((y) => y.status === "active");
    selectedYearId = active ? active.id : years[0]?.id || "";
  }
});

// Sheets
let isYearSheetOpen = $state(false);
let editingYear = $state<academicCalendarDto | null>(null);
let isPeriodSheetOpen = $state(false);
let editingPeriod = $state<any>(null);

// View Mode: 'timeline' | 'cards'
let viewMode = $state<"timeline" | "cards">("timeline");

async function handleRefresh() {
  await queryClient.invalidateQueries({ queryKey: queryKeys.calendar.all });
}

const currentYear = $derived(years.find((y) => y.id === selectedYearId));
const yearPeriods = $derived(
  periods.filter((p) => p.year_id === selectedYearId),
);

const rootPeriods = $derived(
  yearPeriods.filter((p) => !p.parent_id || p.parent_id === ""),
);

function getChildren(parentId: string) {
  return yearPeriods.filter(
    (p) => p.parent_id === parentId && p.id !== parentId,
  );
}

// Progress metrics
const now = new Date();
const yearProgress = $derived.by(() => {
  if (!currentYear?.start_date || !currentYear?.end_date) return null;
  const start = new Date(currentYear.start_date).getTime();
  const end = new Date(currentYear.end_date).getTime();
  const current = now.getTime();

  if (current <= start)
    return {
      percentage: 0,
      status: "upcoming",
      totalDays: Math.ceil((end - start) / (1000 * 60 * 60 * 24)),
      elapsedDays: 0,
      remainingDays: Math.ceil((end - start) / (1000 * 60 * 60 * 24)),
    };
  if (current >= end)
    return {
      percentage: 100,
      status: "completed",
      totalDays: Math.ceil((end - start) / (1000 * 60 * 60 * 24)),
      elapsedDays: Math.ceil((end - start) / (1000 * 60 * 60 * 24)),
      remainingDays: 0,
    };

  const total = end - start;
  const elapsed = current - start;
  const pct = Math.min(100, Math.max(0, Math.round((elapsed / total) * 100)));
  const totalDays = Math.ceil(total / (1000 * 60 * 60 * 24));
  const elapsedDays = Math.ceil(elapsed / (1000 * 60 * 60 * 24));
  const remainingDays = Math.max(0, totalDays - elapsedDays);

  return {
    percentage: pct,
    status: "in_progress",
    totalDays,
    elapsedDays,
    remainingDays,
  };
});

function calculateTimelinePosition(
  startStr?: string | null,
  endStr?: string | null,
) {
  if (
    !startStr ||
    !endStr ||
    !currentYear?.start_date ||
    !currentYear?.end_date
  ) {
    return { left: 0, width: 100 };
  }
  const yearStart = new Date(currentYear.start_date).getTime();
  const yearEnd = new Date(currentYear.end_date).getTime();
  const periodStart = Math.max(yearStart, new Date(startStr).getTime());
  const periodEnd = Math.min(yearEnd, new Date(endStr).getTime());

  const totalDuration = Math.max(1, yearEnd - yearStart);
  const leftPct = Math.max(
    0,
    Math.min(100, ((periodStart - yearStart) / totalDuration) * 100),
  );
  const widthPct = Math.max(
    2,
    Math.min(100 - leftPct, ((periodEnd - periodStart) / totalDuration) * 100),
  );

  return { left: leftPct, width: widthPct };
}

function openCreatePeriod() {
  editingPeriod = null;
  isPeriodSheetOpen = true;
}

function openEditPeriod(period: any) {
  editingPeriod = period;
  isPeriodSheetOpen = true;
}

function openEditYear(year: academicCalendarDto) {
  editingYear = year;
  isYearSheetOpen = true;
}

const relatedActionGroups = $derived<ActionGroup[]>([
  {
    id: "calendar_academics_links",
    title: m.academics_layout_panel_title(),
    icon: AcademicCap,
    defaultExpanded: true,
    subLinks: [
      {
        id: "academics_schedule",
        label: m.common_exams_detail_schedule(),
        href: `/${orgId}/academics/schedule`,
      },
      {
        id: "academics_exams",
        label: m.common_nav_exams(),
        href: `/${orgId}/academics/exams`,
      },
      {
        id: "academics_sections",
        label: m.common_search_group_classes(),
        href: `/${orgId}/academics/sections`,
      },
      {
        id: "settings_academic_calendars",
        label: m.settings_nav_academic_calendars(),
        href: `/${orgId}/settings/academic-calendars`,
      },
      {
        id: "settings_periods",
        label: m.settings_nav_cycles(),
        href: `/${orgId}/settings/periods`,
      },
    ],
  },
]);
</script>

<div class="h-full w-full overflow-y-auto p-6 flex flex-col gap-4 scrollbar-hide font-sans">
  <!-- Top Bar -->
  <div class="flex items-center justify-between gap-3 flex-wrap shrink-0">
    <!-- Year Switcher Buttons -->
    <div class="flex items-center gap-1.5 flex-wrap">
      {#each years as y (y.id)}
        <Button
          variant={selectedYearId === y.id ? "secondary" : "ghost"}
          size="sm"
          class="text-xs h-8 px-3 font-semibold rounded-md gap-1.5 {selectedYearId === y.id ? 'bg-primary/10 text-primary border border-primary/20' : 'text-muted-foreground'}"
          onclick={() => (selectedYearId = y.id)}
        >
          <span>{y.name_ar || y.name}</span>
          {#if y.status === "active"}
            <span class="w-1.5 h-1.5 rounded-full bg-emerald-500"></span>
          {/if}
        </Button>
      {/each}
    </div>

    <!-- Actions & View Switcher -->
    <div class="flex items-center gap-2">
      <div class="flex items-center bg-muted/60 p-0.5 rounded-lg border border-border/50">
        <button
          type="button"
          class="px-2.5 py-1 text-xs font-semibold rounded-md transition-colors cursor-pointer {viewMode === 'timeline' ? 'bg-card text-foreground shadow-xs' : 'text-muted-foreground hover:text-foreground'}"
          onclick={() => (viewMode = "timeline")}
        >
          {m.academic_calendar_timeline_view()}
        </button>
        <button
          type="button"
          class="px-2.5 py-1 text-xs font-semibold rounded-md transition-colors cursor-pointer {viewMode === 'cards' ? 'bg-card text-foreground shadow-xs' : 'text-muted-foreground hover:text-foreground'}"
          onclick={() => (viewMode = "cards")}
        >
          {m.academic_calendar_cards_view()}
        </button>
      </div>

      <Button
        variant="ghost"
        size="icon-sm"
        onclick={handleRefresh}
        class="text-muted-foreground"
        disabled={isRefreshing}
      >
        {#if isRefreshing}
          <LoadingIndicator size={14} />
        {:else}
          <Refresh size={14} />
        {/if}
      </Button>

      <Button
        variant="outline"
        size="sm"
        class="text-xs h-8 px-3 font-semibold gap-1.5"
        onclick={openCreatePeriod}
      >
        <Plus size={13} />
        {m.academic_calendar_add_period_btn()}
      </Button>
    </div>
  </div>

  {#if isLoading}
    <div class="flex flex-col gap-3">
      <Skeleton class="h-20 w-full rounded-lg" />
      <Skeleton class="h-64 w-full rounded-lg" />
    </div>
  {:else if !currentYear}
    <div class="flex flex-col items-center justify-center py-16 px-4 rounded-xl border border-dashed border-border/70 text-center">
      <div class="p-2.5 rounded-full bg-muted text-muted-foreground mb-3">
        <Calendar size={24} />
      </div>
      <h3 class="text-xs font-bold text-foreground mb-1">
        {m.academic_calendar_no_year_title()}
      </h3>
      <p class="text-[11px] text-muted-foreground max-w-sm mb-4">
        {m.academic_calendar_no_year_desc()}
      </p>
      <Button
        size="sm"
        class="text-xs h-8 px-3 font-semibold gap-1.5"
        onclick={() => {
          editingYear = null;
          isYearSheetOpen = true;
        }}
      >
        <Plus size={13} />
        {m.academic_calendar_create_year_btn()}
      </Button>
    </div>
  {:else}
    <!-- Compact Year Overview & Progress Bar -->
    <div class="p-4 rounded-lg border border-border/60 bg-card/40 flex flex-col gap-3" in:fade={{ duration: 150 }}>
      <div class="flex items-center justify-between gap-3">
        <div class="flex items-center gap-2.5">
          <span class="text-xs font-bold text-foreground">
            {currentYear.name_ar || currentYear.name}
          </span>
          {#if currentYear.name && currentYear.name_ar && currentYear.name !== currentYear.name_ar}
            <span class="text-[11px] text-muted-foreground">
              ({currentYear.name})
            </span>
          {/if}
          {#if currentYear.status === "active"}
            <Badge variant="info" class="text-[9px] px-1.5 py-0 h-4.5 rounded-full font-bold bg-emerald-500/10 text-emerald-600 dark:text-emerald-400 border border-emerald-500/20 gap-1">
              <span class="w-1 h-1 rounded-full bg-emerald-500 animate-pulse"></span>
              {m.academic_calendar_active_year_badge()}
            </Badge>
          {/if}
          <span class="text-[10px] text-muted-foreground font-mono">
            {formatDate(currentYear.start_date)} ↔ {formatDate(currentYear.end_date)}
          </span>
        </div>

        <Button
          variant="ghost"
          size="icon-sm"
          class="text-muted-foreground hover:text-foreground h-7 w-7"
          onclick={() => openEditYear(currentYear)}
          title={m.common_action_edit_simple()}
        >
          <Pencil size={13} />
        </Button>
      </div>

      {#if yearProgress}
        <div class="flex flex-col gap-1.5 pt-2 border-t border-border/30">
          <div class="flex items-center justify-between text-[11px] text-muted-foreground">
            <span class="font-medium">
              {m.academic_calendar_progress_label()} <strong class="text-foreground font-mono">{yearProgress.percentage}%</strong>
            </span>
            <div class="flex items-center gap-2 font-mono text-[10px]">
              <span>{m.academic_calendar_elapsed()} {m.academic_calendar_days({ count: yearProgress.elapsedDays })}</span>
              <span>•</span>
              <span>{m.academic_calendar_remaining()} {m.academic_calendar_days({ count: yearProgress.remainingDays })}</span>
              <span>•</span>
              <span>{m.academic_calendar_total()} {m.academic_calendar_days({ count: yearProgress.totalDays })}</span>
            </div>
          </div>
          <div class="w-full h-1.5 rounded-full bg-muted/80 overflow-hidden">
            <div
              class="h-full bg-primary transition-all duration-300 rounded-full"
              style="width: {yearProgress.percentage}%"
            ></div>
          </div>
        </div>
      {/if}
    </div>

    <!-- Timeline Gantt Visualizer View -->
    {#if viewMode === "timeline"}
      <div class="p-4 rounded-lg border border-border/60 bg-card/40 flex flex-col gap-4" in:fade={{ duration: 150 }}>
        <div class="flex items-center justify-between">
          <span class="text-xs font-bold text-foreground">
            {m.academic_calendar_timeline_chart_title()}
          </span>
          <span class="text-[11px] text-muted-foreground">
            {m.academic_calendar_root_terms_count({ count: rootPeriods.length })}
          </span>
        </div>

        {#if rootPeriods.length === 0}
          <div class="py-10 text-center text-xs text-muted-foreground border border-dashed border-border/50 rounded-lg bg-card/20 flex flex-col items-center gap-2">
            <ClockRotateRight size={20} class="text-muted-foreground/60" />
            <p class="text-[11px]">{m.academic_calendar_no_terms_defined()}</p>
            <Button size="sm" class="text-xs h-7 px-2.5 font-medium gap-1" onclick={openCreatePeriod}>
              <Plus size={12} />
              {m.academic_calendar_add_first_term()}
            </Button>
          </div>
        {:else}
          <div class="flex flex-col gap-3 overflow-x-auto pb-1">
            <!-- Timeline Ruler Header -->
            <div class="relative w-full h-7 bg-muted/30 rounded-md flex items-center justify-between px-3 text-[10px] font-mono text-muted-foreground border border-border/30 select-none">
              <span>{formatDate(currentYear.start_date)}</span>
              <span class="font-sans font-medium text-foreground text-[11px]">{m.academic_calendar_full_year_timeline()}</span>
              <span>{formatDate(currentYear.end_date)}</span>
            </div>

            <!-- Root Periods Timeline Bars -->
            <div class="flex flex-col gap-2.5">
              {#each rootPeriods as root (root.id)}
                {@const children = getChildren(root.id)}
                {@const rootPos = calculateTimelinePosition(root.start_date, root.end_date)}
                {@const isRootActive = root.status === "active"}
                {@const isRootLocked = root.is_locked === 1}

                <div class="flex flex-col gap-1 p-2.5 rounded-lg border border-border/40 bg-background/40 hover:bg-background/70 transition-colors">
                  <!-- Term Title Bar -->
                  <div class="flex items-center justify-between gap-2 text-xs">
                    <div class="flex items-center gap-2 min-w-0">
                      <span class="font-semibold text-foreground text-xs truncate">
                        {root.name_ar || root.name}
                      </span>
                      {#if isRootActive}
                        <Badge variant="info" class="text-[9px] px-1 py-0 rounded-full font-bold bg-emerald-500/10 text-emerald-600 dark:text-emerald-400">
                          {m.academic_calendar_active_badge()}
                        </Badge>
                      {:else if isRootLocked}
                        <Badge variant="secondary" class="text-[9px] px-1 py-0 rounded-full">
                          {m.academic_calendar_locked_badge()}
                        </Badge>
                      {/if}
                    </div>

                    <div class="flex items-center gap-1.5 font-mono text-[10px] text-muted-foreground shrink-0">
                      <span>{formatDate(root.start_date)} ↔ {formatDate(root.end_date)}</span>
                      <button
                        type="button"
                        class="p-0.5 hover:bg-accent rounded text-muted-foreground hover:text-foreground cursor-pointer"
                        onclick={() => openEditPeriod(root)}
                        title={m.common_action_edit_simple()}
                      >
                        <Pencil size={11} />
                      </button>
                    </div>
                  </div>

                  <!-- Visual Timeline Gantt Bar for Root -->
                  <div class="relative w-full h-5 rounded bg-muted/40 overflow-hidden">
                    <div
                      class="absolute top-0 bottom-0 rounded transition-all flex items-center px-1.5 text-[9px] font-bold truncate {isRootActive ? 'bg-emerald-500/30 text-emerald-700 dark:text-emerald-300 border border-emerald-500/40' : 'bg-primary/20 text-primary border border-primary/30'}"
                      style="left: {rootPos.left}%; width: {rootPos.width}%;"
                    >
                      <span class="truncate">{root.name_ar || root.name}</span>
                    </div>
                  </div>

                  <!-- Sub-periods -->
                  {#if children.length > 0}
                    <div class="flex flex-col gap-1 mt-0.5 ps-3 border-s border-primary/20">
                      <div class="relative w-full h-4 rounded bg-muted/20 overflow-hidden">
                        {#each children as child (child.id)}
                          {@const childPos = calculateTimelinePosition(child.start_date, child.end_date)}
                          <div
                            class="absolute top-0 bottom-0 rounded transition-all flex items-center px-1 text-[8px] font-medium truncate bg-secondary/80 text-secondary-foreground border border-border/50 hover:z-10 cursor-pointer"
                            style="left: {childPos.left}%; width: {childPos.width}%;"
                            title="{child.name_ar || child.name} ({formatDate(child.start_date)} - {formatDate(child.end_date)})"
                            onclick={() => openEditPeriod(child)}
                          >
                            <span class="truncate">{child.name_ar || child.name}</span>
                          </div>
                        {/each}
                      </div>
                    </div>
                  {/if}
                </div>
              {/each}
            </div>
          </div>
        {/if}
      </div>
    {:else}
      <!-- Cards / Grid View -->
      <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3" in:fade={{ duration: 150 }}>
        {#each rootPeriods as root (root.id)}
          {@const children = getChildren(root.id)}
          {@const isRootActive = root.status === "active"}
          {@const isRootLocked = root.is_locked === 1}

          <div class="p-3.5 rounded-lg border border-border/60 bg-card/40 flex flex-col justify-between gap-3">
            <div class="flex flex-col gap-1.5">
              <div class="flex items-center justify-between gap-2">
                <span class="text-xs font-bold text-foreground truncate">
                  {root.name_ar || root.name}
                </span>
                {#if isRootActive}
                  <Badge variant="info" class="text-[9px] px-1.5 py-0 rounded-full font-bold bg-emerald-500/10 text-emerald-600 dark:text-emerald-400">
                    {m.academic_calendar_active_badge()}
                  </Badge>
                {:else if isRootLocked}
                  <Badge variant="secondary" class="text-[9px] px-1.5 py-0 rounded-full font-medium">
                    {m.academic_calendar_locked_badge()}
                  </Badge>
                {/if}
              </div>

              <span class="text-[10px] text-muted-foreground font-mono">
                {formatDate(root.start_date)} ↔ {formatDate(root.end_date)}
              </span>

              <!-- Sub-Periods List -->
              <div class="flex flex-col gap-1 mt-1 pt-1.5 border-t border-border/40">
                <span class="text-[10px] font-medium text-muted-foreground">
                  {m.academic_calendar_subperiods_title()} ({children.length})
                </span>
                {#if children.length === 0}
                  <span class="text-[10px] text-muted-foreground/60">
                    {m.academic_calendar_no_subperiods()}
                  </span>
                {:else}
                  {#each children as child (child.id)}
                    <div class="flex items-center justify-between text-[10px] p-1 rounded bg-muted/30 font-mono text-muted-foreground">
                      <span class="font-sans font-medium text-foreground truncate">
                        {child.name_ar || child.name}
                      </span>
                      <span>{formatDate(child.start_date)}</span>
                    </div>
                  {/each}
                {/if}
              </div>
            </div>

            <div class="flex items-center justify-end gap-1.5 pt-1.5 border-t border-border/30">
              <Button
                variant="ghost"
                size="icon-sm"
                class="text-muted-foreground hover:text-foreground h-6 w-6"
                onclick={() => openEditPeriod(root)}
                title={m.common_action_edit_simple()}
              >
                <Pencil size={12} />
              </Button>
            </div>
          </div>
        {/each}
      </div>
    {/if}
  {/if}

  <!-- Related Support Actions -->
  <RelatedActions
    title={m.ui_copy_related_support_bf36e16f()}
    groups={relatedActionGroups}
  />
</div>

<!-- Sheets -->
<YearCreateSheet
  bind:open={isYearSheetOpen}
  {orgId}
  year={editingYear}
  existingYears={years}
  onSuccess={handleRefresh}
/>

<PeriodCreateSheet
  bind:open={isPeriodSheetOpen}
  {orgId}
  period={editingPeriod}
  onSuccess={handleRefresh}
/>
