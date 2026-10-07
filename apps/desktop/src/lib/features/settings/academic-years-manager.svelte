<!--
@component AcademicYearsManager
Allows administrators to manage Academic Years, activate current school year,
close completed years, archive historical years, and view period structure.
-->
<script lang="ts">
  import { api, extractErrorMessage } from "$lib/api";
  import * as m from "$lib/paraglide/messages";
  import { academicContext, confirmStore } from "$lib/stores";
  import type { AcademicPeriodDto, AcademicYearDto } from "@orbis/sdk";
  import { LoadingIndicator, toast } from "@orbis/ui";
  import { Badge } from "@orbis/ui/badge";
  import { Button } from "@orbis/ui/button";
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
  import { onMount } from "svelte";
  import { fade } from "svelte/transition";

  let { orgId }: { orgId: string } = $props();

  // State
  let years = $state<AcademicYearDto[]>([]);
  let periods = $state<AcademicPeriodDto[]>([]);
  let isLoading = $state(true);
  let isRefreshing = $state(false);
  let searchQuery = $state("");
  let selectedFilters = $state<string[]>([]);

  // Expanded year accordion items
  let expandedYears = $state<Record<string, boolean>>({});

  // Sheet Controls
  let isSheetOpen = $state(false);
  let editingYear = $state<AcademicYearDto | null>(null);

  async function loadData() {
    isRefreshing = true;
    try {
      const [yearsList, periodsList] = await Promise.all([
        api.listAcademicYears(),
        api.listAcademicPeriods().catch(() => []),
      ]);
      years = yearsList || [];
      periods = periodsList || [];

      // Auto expand active year if not yet set
      const active = years.find((y) => y.status === "active");
      if (active && Object.keys(expandedYears).length === 0) {
        expandedYears[active.id] = true;
      }
    } catch (e: unknown) {
      toast.error(extractErrorMessage(e));
    } finally {
      isLoading = false;
      isRefreshing = false;
    }
  }

  onMount(() => {
    loadData();
  });

  // Filter Groups for SearchableMultiFilter
  const filterGroups = $derived<SearchableMultiFilterGroup[]>([
    {
      id: "status",
      label: m.common_word_status(),
      options: [
        { value: "status:active", label: m.academic_years_filter_active() },
        { value: "status:planning", label: m.academic_years_filter_planning() },
        { value: "status:closed", label: m.academic_years_filter_closed() },
        { value: "status:archived", label: m.academic_years_filter_archived() },
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

  function openEditSheet(year: AcademicYearDto) {
    editingYear = year;
    isSheetOpen = true;
  }

  async function handleActivate(year: AcademicYearDto) {
    if (year.status === "active") return;

    const confirmed = await confirmStore.prompt({
      title: m.academic_years_activate_confirm_title(),
      description: `${m.academic_years_activate_confirm_desc()} (${year.name_ar || year.name})`,
      confirmText: m.academic_years_activate_btn(),
      variant: "default",
    });

    if (!confirmed) return;

    try {
      await api.activateAcademicYear(year.id);
      toast.success(m.academic_years_activate_success());
      await academicContext.resolveContext(orgId);
      await loadData();
    } catch (e: unknown) {
      toast.error(extractErrorMessage(e));
    }
  }

  async function handleClose(year: AcademicYearDto) {
    const confirmed = await confirmStore.prompt({
      title: m.academic_years_close_confirm_title(),
      description: `${m.academic_years_close_confirm_desc()} (${year.name_ar || year.name})`,
      confirmText: m.academic_years_close_btn(),
      variant: "destructive",
    });

    if (!confirmed) return;

    try {
      await api.closeAcademicYear(year.id);
      toast.success(m.academic_years_close_success());
      await academicContext.resolveContext(orgId);
      await loadData();
    } catch (e: unknown) {
      toast.error(extractErrorMessage(e));
    }
  }

  async function handleArchive(year: AcademicYearDto) {
    const confirmed = await confirmStore.prompt({
      title: m.academic_years_archive_confirm_title(),
      description: `${m.academic_years_archive_confirm_desc()} (${year.name_ar || year.name})`,
      confirmText: m.academic_years_archive_btn(),
      variant: "destructive",
    });

    if (!confirmed) return;

    try {
      await api.archiveAcademicYear(year.id);
      toast.success(m.academic_years_archive_success());
      await academicContext.resolveContext(orgId);
      await loadData();
    } catch (e: unknown) {
      toast.error(extractErrorMessage(e));
    }
  }

  function formatDate(d?: string | null): string {
    if (!d) return "-";
    return d.slice(0, 10);
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

<div class="h-full w-full overflow-y-auto p-6 flex flex-col gap-4 scrollbar-hide font-sans">
  <!-- Top Minimal Toolbar -->
  <div class="flex items-center justify-between gap-3 flex-nowrap shrink-0">
    <div class="relative flex-1 max-w-xs">
      <Search
        size={14}
        class="absolute inset-s-2.5 top-1/2 -translate-y-1/2 text-muted-foreground pointer-events-none"
      />
      <Input
        type="text"
        placeholder={m.academic_years_search_placeholder()}
        bind:value={searchQuery}
        class="h-8 ps-8 text-xs bg-card border-border/60"
      />
    </div>

    <div class="flex items-center gap-2">
      <SearchableMultiFilter
        groups={filterGroups}
        bind:selectedValues={selectedFilters}
        placeholder={m.common_filter()}
      />
      <Button
        variant="ghost"
        size="icon-sm"
        onclick={loadData}
        class="text-muted-foreground"
        disabled={isRefreshing}
      >
        {#if isRefreshing}
          <LoadingIndicator size={14} />
        {:else}
          <Refresh size={14} />
        {/if}
      </Button>
      <Button class="text-xs h-8 px-3 font-semibold shrink-0" onclick={openCreateSheet}>
        <Plus size={14} class="me-1.5" />
        {m.academic_years_add_btn()}
      </Button>
    </div>
  </div>

  <!-- Content List / Rows -->
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
          ? m.academic_years_empty_search()
          : m.academic_years_empty_title()}
      </h3>
      <p class="text-[11px] text-muted-foreground max-w-sm mb-4">
        {searchQuery || selectedFilters.length > 0
          ? m.academic_years_empty_search_desc()
          : m.academic_years_empty_desc()}
      </p>
      {#if !searchQuery && selectedFilters.length === 0}
        <Button size="sm" class="text-xs h-8 px-3 font-semibold gap-1.5" onclick={openCreateSheet}>
          <Plus size={13} />
          {m.academic_years_add_btn()}
        </Button>
      {/if}
    </div>
  {:else}
    <div class="flex flex-col gap-2.5" in:fade={{ duration: 150 }}>
      {#each filteredYears as year (year.id)}
        {@const yearPeriods = getPeriodsForYear(year.id)}
        {@const isExpanded = !!expandedYears[year.id]}
        {@const isActive = year.status === "active"}
        {@const isClosed = year.status === "closed"}
        {@const isArchived = year.status === "archived"}
        {@const isPlanning = year.status === "planning" || !year.status}

        <div
          class="rounded-lg border transition-all duration-150 bg-card/40 hover:bg-card/70 overflow-hidden {isActive
            ? 'border-emerald-500/40 ring-1 ring-emerald-500/10'
            : 'border-border/60'}"
        >
          <!-- Compact Main Row -->
          <div class="px-4 py-3 flex items-center justify-between gap-3">
            <!-- Left Info Block -->
            <div class="flex items-center gap-3 min-w-0">
              <button
                type="button"
                class="p-1 rounded hover:bg-muted text-muted-foreground transition-colors cursor-pointer shrink-0"
                onclick={() => toggleExpand(year.id)}
                title={m.academic_years_child_periods_title()}
              >
                {#if isExpanded}
                  <ChevronDown size={15} />
                {:else}
                  <ChevronRight size={15} class="rtl:rotate-180" />
                {/if}
              </button>

              <div class="flex items-center gap-2.5 min-w-0 flex-wrap">
                <span class="text-xs font-bold text-foreground truncate">
                  {year.name_ar || year.name}
                </span>

                {#if year.name && year.name_ar && year.name !== year.name_ar}
                  <span class="text-[11px] text-muted-foreground truncate">
                    ({year.name})
                  </span>
                {/if}

                <!-- Status Badge -->
                {#if isActive}
                  <Badge variant="info" class="text-[9px] px-1.5 py-0 h-4.5 rounded-full font-bold bg-emerald-500/10 text-emerald-600 dark:text-emerald-400 border border-emerald-500/20 gap-1">
                    <span class="w-1 h-1 rounded-full bg-emerald-500 animate-pulse"></span>
                    {m.academic_calendar_active_badge()}
                  </Badge>
                {:else if isPlanning}
                  <Badge variant="secondary" class="text-[9px] px-1.5 py-0 h-4.5 rounded-full font-medium">
                    {m.academic_years_filter_planning()}
                  </Badge>
                {:else if isClosed}
                  <Badge variant="secondary" class="text-[9px] px-1.5 py-0 h-4.5 rounded-full font-medium text-amber-600 dark:text-amber-400 bg-amber-500/10 border-amber-500/20">
                    {m.academic_years_filter_closed()}
                  </Badge>
                {:else if isArchived}
                  <Badge variant="secondary" class="text-[9px] px-1.5 py-0 h-4.5 rounded-full font-medium text-muted-foreground">
                    {m.academic_years_filter_archived()}
                  </Badge>
                {/if}

                <!-- Periods Count -->
                <span class="text-[10px] font-mono px-1.5 py-0.5 rounded bg-muted/60 text-muted-foreground">
                  {m.academic_years_count_tag({ count: yearPeriods.length })}
                </span>

                <!-- Date Range -->
                <span class="text-[10px] text-muted-foreground font-mono">
                  {formatDate(year.start_date)} ↔ {formatDate(year.end_date)}
                </span>
              </div>
            </div>

            <!-- Right Actions Block -->
            <div class="flex items-center gap-1.5 shrink-0">
              {#if !isActive && !isArchived}
                <Button
                  variant="outline"
                  size="sm"
                  class="text-[11px] h-7 px-2.5 font-medium text-emerald-600 dark:text-emerald-400 border-emerald-500/20 hover:bg-emerald-500/10 gap-1"
                  onclick={() => handleActivate(year)}
                >
                  <CheckCircle size={12} />
                  <span>{m.academic_years_activate_btn()}</span>
                </Button>
              {/if}

              {#if isActive}
                <Button
                  variant="outline"
                  size="sm"
                  class="text-[11px] h-7 px-2.5 font-medium text-amber-600 dark:text-amber-400 border-amber-500/20 hover:bg-amber-500/10 gap-1"
                  onclick={() => handleClose(year)}
                >
                  <Lock size={12} />
                  <span>{m.academic_years_close_btn()}</span>
                </Button>
              {/if}

              {#if !isArchived && !isActive}
                <Button
                  variant="ghost"
                  size="icon-sm"
                  class="text-muted-foreground hover:text-foreground h-7 w-7"
                  onclick={() => handleArchive(year)}
                  title={m.academic_years_archive_btn()}
                >
                  <ArchiveBox size={13} />
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
            <div class="border-t border-border/40 bg-muted/10 px-4 py-3">
              <div class="flex items-center justify-between mb-2">
                <span class="text-[11px] font-semibold text-muted-foreground">
                  {m.academic_years_child_periods_title()} ({yearPeriods.length})
                </span>
                <a
                  href={`/${orgId}/settings/periods`}
                  class="text-[11px] text-primary hover:underline font-medium flex items-center gap-1"
                >
                  <span>{m.academic_years_manage_periods_link()}</span>
                  <ChevronRight size={11} class="rtl:rotate-180" />
                </a>
              </div>

              {#if yearPeriods.length === 0}
                <div class="py-2.5 text-center text-[11px] text-muted-foreground/70 border border-dashed border-border/50 rounded-md bg-card/20">
                  <span>{m.academic_years_no_periods_configured()}</span>
                  <a
                    href={`/${orgId}/settings/periods`}
                    class="text-primary hover:underline font-medium ms-1"
                  >
                    {m.academic_years_add_periods_link()}
                  </a>
                </div>
              {:else}
                <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-2">
                  {#each yearPeriods as period (period.id)}
                    {@const isPeriodActive = period.status === "active"}
                    {@const isPeriodLocked = period.is_locked === 1}
                    <div class="p-2 rounded-md border border-border/40 bg-card/70 flex items-center justify-between gap-2 text-xs">
                      <div class="flex flex-col min-w-0">
                        <span class="font-medium text-foreground truncate text-[11px]">
                          {period.name_ar || period.name}
                        </span>
                        <span class="text-[10px] text-muted-foreground font-mono">
                          {formatDate(period.start_date)} ↔ {formatDate(period.end_date)}
                        </span>
                      </div>
                      {#if isPeriodActive}
                        <Badge variant="info" class="text-[9px] px-1 py-0 h-4 rounded-full bg-emerald-500/10 text-emerald-600 dark:text-emerald-400 shrink-0">
                          {m.academic_calendar_active_badge()}
                        </Badge>
                      {:else if isPeriodLocked}
                        <Badge variant="secondary" class="text-[9px] px-1 py-0 h-4 rounded-full shrink-0">
                          {m.academic_calendar_locked_badge()}
                        </Badge>
                      {/if}
                    </div>
                  {/each}
                </div>
              {/if}
            </div>
          {/if}
        </div>
      {/each}
    </div>
  {/if}

  <!-- Related Support Actions -->
  <RelatedActions
    title={m.ui_copy_related_support_bf36e16f()}
    groups={relatedActionGroups}
  />
</div>

<!-- Year Create / Edit Sheet -->
<YearCreateSheet
  bind:open={isSheetOpen}
  {orgId}
  year={editingYear}
  existingYears={years}
  onSuccess={loadData}
/>
