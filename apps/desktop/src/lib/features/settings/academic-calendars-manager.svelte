<!--
@component academicCalendarsManager
Allows administrators to manage Academic Years, activate current school year,
close completed years, archive historical years, and view period structure.
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
import { Card, CardContent } from "@orbis/ui/card";
import { Input } from "@orbis/ui/input";
import { Skeleton } from "@orbis/ui/skeleton";
import {
  AcademicCap,
  ArchiveBox,
  Calendar,
  CheckCircle,
  ChevronDown,
  ChevronRight,
  ClockRotateRight,
  Lock,
  Pencil,
  Plus,
  Refresh,
  Search,
} from "@orbis/icons";
import SearchableMultiFilter, {
  type SearchableMultiFilterGroup,
} from "$lib/shared/components/searchable-multi-filter.svelte";
import RelatedActions, {
  type ActionGroup,
} from "$lib/shared/components/related-actions.svelte";
import YearCreateSheet from "./year-create-sheet.svelte";
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

let searchQuery = $state("");
let selectedFilters = $state<string[]>([]);

// Expanded year accordion items
let expandedYears = $state<Record<string, boolean>>({});

// Sheet Controls
let isSheetOpen = $state(false);
let editingYear = $state<academicCalendarDto | null>(null);

// Auto expand active year if not yet set
$effect(() => {
  const active = years.find((y) => y.status === "active");
  if (active && Object.keys(expandedYears).length === 0) {
    expandedYears[active.id] = true;
  }
});

async function handleRefresh() {
  await queryClient.invalidateQueries({ queryKey: queryKeys.calendar.all });
}

// Filter Groups for SearchableMultiFilter
const filterGroups = $derived<SearchableMultiFilterGroup[]>([
  {
    id: "status",
    label: m.common_word_status(),
    options: [
      { value: "status:active", label: m.academic_calendars_filter_active() },
      {
        value: "status:planning",
        label: m.academic_calendars_filter_planning(),
      },
      { value: "status:closed", label: m.academic_calendars_filter_closed() },
      {
        value: "status:archived",
        label: m.academic_calendars_filter_archived(),
      },
    ],
  },
]);

const selectedStatusValues = $derived(
  selectedFilters
    .filter((f) => f.startsWith("status:"))
    .map((f) => f.replace("status:", "")),
);

// Filtered Years
let filteredYears = $derived(
  years.filter((y) => {
    const q = searchQuery.trim().toLowerCase();
    const matchesSearch =
      !q ||
      (y.name && y.name.toLowerCase().includes(q)) ||
      (y.name_ar && y.name_ar.toLowerCase().includes(q)) ||
      (y.notes && y.notes.toLowerCase().includes(q));

    const status = y.status || "planning";
    const matchesStatus =
      selectedStatusValues.length === 0 ||
      selectedStatusValues.includes(status);

    return matchesSearch && matchesStatus;
  }),
);

function getPeriodsForYear(yearId: string) {
  return periods.filter((p) => p.year_id === yearId);
}

function toggleExpand(yearId: string) {
  expandedYears[yearId] = !expandedYears[yearId];
}

function openCreateSheet() {
  editingYear = null;
  isSheetOpen = true;
}

function openEditSheet(year: academicCalendarDto) {
  editingYear = year;
  isSheetOpen = true;
}

async function handleActivate(year: academicCalendarDto) {
  if (year.status === "active") return;

  const confirmed = await confirmStore.prompt({
    title: m.academic_calendars_activate_confirm_title(),
    description: `${m.academic_calendars_activate_confirm_desc()} (${year.name_ar || year.name})`,
    confirmText: m.academic_calendars_activate_btn(),
    variant: "default",
  });

  if (!confirmed) return;

  try {
    await api.activateacademicCalendar(year.id);
    toast.success(m.academic_calendars_activate_success());
    await academicContext.resolveContext(orgId);
    await queryClient.invalidateQueries({ queryKey: queryKeys.calendar.all });
  } catch (e: unknown) {
    toast.error(extractErrorMessage(e));
  }
}

async function handleClose(year: academicCalendarDto) {
  const confirmed = await confirmStore.prompt({
    title: m.academic_calendars_close_confirm_title(),
    description: `${m.academic_calendars_close_confirm_desc()} (${year.name_ar || year.name})`,
    confirmText: m.academic_calendars_close_btn(),
    variant: "destructive",
  });

  if (!confirmed) return;

  try {
    await api.closeacademicCalendar(year.id);
    toast.success(m.academic_calendars_close_success());
    await academicContext.resolveContext(orgId);
    await queryClient.invalidateQueries({ queryKey: queryKeys.calendar.all });
  } catch (e: unknown) {
    toast.error(extractErrorMessage(e));
  }
}

async function handleArchive(year: academicCalendarDto) {
  const confirmed = await confirmStore.prompt({
    title: m.academic_calendars_archive_confirm_title(),
    description: `${m.academic_calendars_archive_confirm_desc()} (${year.name_ar || year.name})`,
    confirmText: m.academic_calendars_archive_btn(),
    variant: "destructive",
  });

  if (!confirmed) return;

  try {
    await api.archiveacademicCalendar(year.id);
    toast.success(m.academic_calendars_archive_success());
    await academicContext.resolveContext(orgId);
    await queryClient.invalidateQueries({ queryKey: queryKeys.calendar.all });
  } catch (e: unknown) {
    toast.error(extractErrorMessage(e));
  }
}

let isApplyingTemplate = $state(false);

async function handleApplyTemplate(year: academicCalendarDto) {
  isApplyingTemplate = true;
  try {
    await api.applyCalendarTemplate(year.id, { template_code: "YE-1447-1448" });
    toast.success("Official calendar template applied successfully.");
    await queryClient.invalidateQueries({ queryKey: queryKeys.calendar.all });
  } catch (e: unknown) {
    toast.error(extractErrorMessage(e));
  } finally {
    isApplyingTemplate = false;
  }
}

const relatedActionGroups = $derived<ActionGroup[]>([
  {
    id: "academic_calendar_navigation",
    title: m.academic_calendar_title(),
    icon: Calendar,
    defaultExpanded: true,
    subLinks: [
      {
        id: "periods_manager",
        label: m.settings_nav_cycles(),
        href: `/${orgId}/settings/periods`,
      },
      {
        id: "visual_calendar",
        label: m.academics_nav_calendar(),
        href: `/${orgId}/academics/calendar`,
      },
      {
        id: "academic_policies",
        label: m.ui_copy_academic_policy_grading_73e34776(),
        href: `/${orgId}/settings/academic-policy`,
      },
      {
        id: "promotion_wizard",
        label: m.ui_copy_student_promotion_wizard_e772861a(),
        href: `/${orgId}/academics/promotion`,
      },
    ],
  },
]);
</script>

<div class="flex h-full w-full flex-col overflow-hidden bg-background font-sans text-start">
  <header class="shrink-0 border-b border-border/50 bg-card/40 px-4 py-3 backdrop-blur-sm">
    <div class="flex flex-wrap items-center gap-2">
      <div class="relative min-w-0 flex-1 basis-56 max-w-md">
        <Search
          size={14}
          class="absolute inset-s-2.5 top-1/2 -translate-y-1/2 text-muted-foreground pointer-events-none"
        />
        <Input
          type="text"
          placeholder={m.academic_calendars_search_placeholder()}
          bind:value={searchQuery}
          class="h-8 ps-8 text-xs bg-card border-border/60"
        />
      </div>

      <div class="ms-auto flex items-center gap-2">
        <SearchableMultiFilter
          groups={filterGroups}
          bind:selectedValues={selectedFilters}
          placeholder={m.academic_calendars_filter_placeholder()}
          searchPlaceholder={m.academic_calendars_filter_search()}
          emptyText={m.academic_calendars_no_matching_filters()}
        />
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
          variant="lime"
          class="h-8 shrink-0 px-3 text-xs font-semibold"
          onclick={openCreateSheet}
        >
          <Plus size={14} class="me-1.5" />
          {m.academic_calendars_add_btn()}
        </Button>
      </div>
    </div>
  </header>

  <main class="min-h-0 flex-1 space-y-4 overflow-y-auto p-4 scrollbar-hide sm:p-6">
  <!-- Content List / Year Cards -->
  {#if isLoading}
    <div class="flex flex-col gap-2.5">
      <Skeleton class="h-16 w-full rounded-lg" />
      <Skeleton class="h-16 w-full rounded-lg" />
      <Skeleton class="h-16 w-full rounded-lg" />
    </div>
  {:else if filteredYears.length === 0}
    <div class="flex flex-col items-center justify-center py-16 px-4 rounded-xl border border-dashed border-border/70 text-center">
      <div class="p-2.5 rounded-full bg-muted text-muted-foreground mb-3">
        <Calendar size={22} />
      </div>
      <h3 class="text-xs font-bold text-foreground mb-1">
        {searchQuery || selectedFilters.length > 0
          ? m.academic_calendars_empty_search()
          : m.academic_calendars_empty_title()}
      </h3>
      {#if !searchQuery && selectedFilters.length === 0}
        <Button size="sm" class="h-8 gap-1.5 px-3 text-xs font-semibold" onclick={openCreateSheet}>
          <Plus size={13} />
          {m.academic_calendars_add_btn()}
        </Button>
      {/if}
    </div>
  {:else}
    <div class="mx-auto flex w-full max-w-7xl flex-col gap-3" in:fade={{ duration: 150 }}>
      {#each filteredYears as year (year.id)}
        {@const yearPeriods = getPeriodsForYear(year.id)}
        {@const isExpanded = !!expandedYears[year.id]}
        {@const isActive = year.status === "active"}
        {@const isClosed = year.status === "closed"}
        {@const isArchived = year.status === "archived"}
        {@const isPlanning = year.status === "planning" || !year.status}

        <Card class="overflow-hidden border-border/60 shadow-xs transition-colors hover:border-border">
          <CardContent class="p-0">
          <div class="flex flex-col gap-3 p-4 sm:flex-row sm:items-center sm:justify-between">
            <div class="flex min-w-0 items-start gap-3">
              <button
                type="button"
                class="mt-0.5 flex h-7 w-7 shrink-0 items-center justify-center rounded-md border border-border/50 bg-background text-muted-foreground transition-colors hover:bg-muted"
                onclick={() => toggleExpand(year.id)}
                title={m.academic_calendars_child_periods_title()}
                aria-label={m.academic_calendars_child_periods_title()}
              >
                {#if isExpanded}
                  <ChevronDown size={16} />
                {:else}
                  <ChevronRight size={16} class="rtl:rotate-180" />
                {/if}
              </button>

              <div class="flex min-w-0 flex-1 flex-col gap-2">
                <div class="flex min-w-0 flex-wrap items-center gap-2">
                  <span class="text-sm font-semibold text-foreground">
                    {year.name_ar || year.name}
                  </span>
                  {#if year.name && year.name_ar && year.name !== year.name_ar}
                    <span class="text-xs text-muted-foreground">({year.name})</span>
                  {/if}
                  {#if isActive}
                    <Badge variant="info" class="h-5 gap-1 border-emerald-500/20 bg-emerald-500/10 px-2 text-[10px] font-semibold text-emerald-600 dark:text-emerald-400">
                      <span class="h-1.5 w-1.5 rounded-full bg-emerald-500"></span>
                      {m.academic_calendar_active_badge()}
                    </Badge>
                  {:else if isPlanning}
                    <Badge variant="secondary" class="h-5 px-2 text-[10px] font-medium">
                      {m.academic_calendars_filter_planning()}
                    </Badge>
                  {:else if isClosed}
                    <Badge variant="secondary" class="h-5 border-amber-500/20 bg-amber-500/10 px-2 text-[10px] font-medium text-amber-600 dark:text-amber-400">
                      {m.academic_calendars_filter_closed()}
                    </Badge>
                  {:else if isArchived}
                    <Badge variant="secondary" class="h-5 px-2 text-[10px] font-medium text-muted-foreground">
                      {m.academic_calendars_filter_archived()}
                    </Badge>
                  {/if}
                </div>

                <div class="flex flex-wrap items-center gap-x-4 gap-y-1 text-[11px] text-muted-foreground">
                  <span class="inline-flex items-center gap-1.5 font-mono">
                    <Calendar size={12} />
                    {formatDate(year.start_date)} <span aria-hidden="true">↔</span> {formatDate(year.end_date)}
                  </span>
                  <span class="inline-flex items-center gap-1.5">
                    <ClockRotateRight size={12} />
                    {m.academic_calendars_count_tag({ count: yearPeriods.length })}
                  </span>
                </div>
              </div>
            </div>

            <div class="flex flex-wrap items-center gap-1 border-t border-border/40 pt-2 sm:shrink-0 sm:border-0 sm:pt-0">
              {#if !isActive && !isArchived}
                <Button
                  variant="ghost"
                  size="sm"
                  class="h-7 gap-1 px-2 text-[11px] font-medium text-emerald-600 hover:bg-emerald-500/10 dark:text-emerald-400"
                  onclick={() => handleActivate(year)}
                >
                  <CheckCircle size={12} />
                  <span>{m.academic_calendars_activate_btn()}</span>
                </Button>
              {/if}

              {#if isActive}
                <Button
                  variant="ghost"
                  size="sm"
                  class="h-7 gap-1 px-2 text-[11px] font-medium text-amber-600 hover:bg-amber-500/10 dark:text-amber-400"
                  onclick={() => handleClose(year)}
                >
                  <Lock size={12} />
                  <span>{m.academic_calendars_close_btn()}</span>
                </Button>
              {/if}

              {#if !isArchived && !isActive}
                <Button
                  variant="ghost"
                  size="icon-sm"
                  class="text-muted-foreground hover:text-foreground h-7 w-7"
                  onclick={() => handleArchive(year)}
                  title={m.academic_calendars_archive_btn()}
                >
                  <ArchiveBox size={13} />
                </Button>
              {/if}

              {#if !isArchived}
                <Button
                  variant="ghost"
                  size="sm"
                  class="h-7 gap-1 px-2 text-[11px] font-medium text-indigo-600 hover:bg-indigo-500/10 dark:text-indigo-400"
                  disabled={isApplyingTemplate}
                  onclick={() => handleApplyTemplate(year)}
                  title="Apply Official Yemen Calendar Template"
                >
                  <Calendar size={12} />
                  <span>{isApplyingTemplate ? "Applying…" : "🇾🇪 Apply Template"}</span>
                </Button>
              {/if}

              <Button
                variant="ghost"
                size="icon-sm"
                class="text-muted-foreground hover:text-foreground h-7 w-7"
                onclick={() => openEditSheet(year)}
                title={m.common_action_edit_simple()}
              >
                <Pencil size={13} />
              </Button>
            </div>
          </div>

          <!-- Nested Sub-Periods Breakdown -->
          {#if isExpanded}
            <div class="border-t border-border/50 bg-muted/20 px-4 py-3 sm:ps-14">
              <div class="flex items-center justify-between mb-2">
                <span class="text-[11px] font-semibold text-muted-foreground flex items-center gap-1.5">
                  <ClockRotateRight size={12} />
                  <span>{m.academic_calendars_child_periods_title()}</span>
                  <span class="text-[10px] font-mono">({yearPeriods.length})</span>
                </span>
                <a
                  href={`/${orgId}/settings/periods`}
                  class="text-[11px] text-primary hover:underline font-medium flex items-center gap-1"
                >
                  <span>{m.academic_calendars_manage_periods_link()}</span>
                  <ChevronRight size={11} class="rtl:rotate-180" />
                </a>
              </div>

              {#if yearPeriods.length === 0}
                <div class="py-3 text-center text-[11px] text-muted-foreground/70 border border-dashed border-border/50 rounded-lg bg-card/20">
                  <span>{m.academic_calendars_no_periods_configured()}</span>
                  <a
                    href={`/${orgId}/settings/periods`}
                    class="text-primary hover:underline font-medium ms-1"
                  >
                    {m.academic_calendars_add_periods_link()}
                  </a>
                </div>
              {:else}
                <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-2">
                  {#each yearPeriods as period (period.id)}
                    {@const isPeriodActive = period.status === "active"}
                    {@const isPeriodLocked = period.is_locked === true}
                    <div class="flex items-center justify-between gap-2 rounded-lg border border-border/50 bg-background p-2.5 text-xs">
                      <div class="flex flex-col min-w-0">
                        <span class="font-medium text-foreground truncate text-[11px]">
                          {period.name_ar || period.name}
                        </span>
                        <span class="text-[10px] text-muted-foreground font-mono">
                          {formatDate(period.start_date)} ↔ {formatDate(period.end_date)}
                        </span>
                      </div>
                      {#if isPeriodActive}
                        <Badge variant="info" class="text-[9px] px-1.5 py-0 h-4 rounded-full bg-emerald-500/10 text-emerald-600 dark:text-emerald-400 shrink-0">
                          {m.academic_calendar_active_badge()}
                        </Badge>
                      {:else if isPeriodLocked}
                        <Badge variant="secondary" class="text-[9px] px-1.5 py-0 h-4 rounded-full shrink-0">
                          {m.academic_calendar_locked_badge()}
                        </Badge>
                      {/if}
                    </div>
                  {/each}
                </div>
              {/if}
            </div>
          {/if}
          </CardContent>
        </Card>
      {/each}
    </div>
  {/if}

  <!-- Related Support Actions -->
  <RelatedActions
    title={m.ui_copy_related_support_bf36e16f()}
    groups={relatedActionGroups}
  />
  </main>
</div>

<!-- Year Create / Edit Sheet -->
<YearCreateSheet
  bind:open={isSheetOpen}
  {orgId}
  year={editingYear}
  existingYears={years}
  onSuccess={handleRefresh}
/>
