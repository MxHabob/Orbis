<!--
@component PeriodsManager
Allows administrators to manage configured academic periods and locking states.
Redesigned with system-wide minimalist toolbar, SearchableMultiFilter, and hierarchical trees.
-->
<script lang="ts">
  import { api, extractErrorMessage, queryKeys } from "$lib/api";
  import { formatDate } from "$lib/core/i18n";
  import { createPeriodsQuery, createacademicCalendarsQuery } from "$lib/features/academics/queries";
  import PeriodCreateSheet from "$lib/features/academics/components/period-create-sheet.svelte";
  import * as m from "$lib/paraglide/messages";
  import { confirmStore } from "$lib/stores";
  import type { AcademicPeriodDto } from "@orbis/sdk";
  import { useQueryClient } from "@tanstack/svelte-query";
  import {
    AcademicCap,
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
    Trash,
  } from "@orbis/icons";
  import { LoadingIndicator, toast } from "@orbis/ui";
  import { Badge } from "@orbis/ui/badge";
  import { Button } from "@orbis/ui/button";
  import { Input } from "@orbis/ui/input";
  import { Skeleton } from "@orbis/ui/skeleton";
  import SearchableMultiFilter, {
    type SearchableMultiFilterGroup,
  } from "$lib/shared/components/searchable-multi-filter.svelte";
  import RelatedActions, {
    type ActionGroup,
  } from "$lib/shared/components/related-actions.svelte";
  import { fade } from "svelte/transition";

  let { orgId }: { orgId: string } = $props();

  const queryClient = useQueryClient();

  // Queries
  const periodsQuery = createPeriodsQuery(() => orgId);
  const yearsQuery = createacademicCalendarsQuery(() => orgId);

  // Derived state from queries
  const periods = $derived(periodsQuery.data ?? []);
  const academicCalendars = $derived(yearsQuery.data ?? []);
  const isLoading = $derived(periodsQuery.isLoading || yearsQuery.isLoading);
  const isRefreshing = $derived(periodsQuery.isFetching || yearsQuery.isFetching);

  let searchQuery = $state("");
  let selectedFilters = $state<string[]>([]);

  // Sheet controls
  let isSheetOpen = $state(false);
  let editingPeriod = $state<any>(null);

  // Collapsed states for parent nodes
  let collapsedNodes = $state<Record<string, boolean>>({});

  async function handleRefresh() {
    await queryClient.invalidateQueries({ queryKey: queryKeys.calendar.all });
  }

  // Filter Groups for SearchableMultiFilter
  const filterGroups = $derived<SearchableMultiFilterGroup[]>([
    {
      id: "years",
      label: m.settings_nav_academic_calendars(),
      options: academicCalendars.map((y) => ({
        value: `year:${y.id}`,
        label: `${y.name_ar || y.name}${y.status === "active" ? " ★" : ""}`,
      })),
    },
    {
      id: "status",
      label: m.common_word_status(),
      options: [
        { value: "status:active", label: m.academic_calendars_filter_active() },
        { value: "status:locked", label: m.academic_calendar_locked_badge() },
      ],
    },
    {
      id: "type",
      label: m.common_filter(),
      options: [
        ...new Set(
          periods
            .map((period) => period.period_type)
            .filter((periodType): periodType is string => Boolean(periodType)),
        ),
      ].map((periodType) => ({
        value: `type:${periodType}`,
        label: periodType,
      })),
    },
  ]);

  const selectedYearFilters = $derived(
    selectedFilters
      .filter((f) => f.startsWith("year:"))
      .map((f) => f.replace("year:", "")),
  );

  const selectedStatusFilters = $derived(
    selectedFilters
      .filter((f) => f.startsWith("status:"))
      .map((f) => f.replace("status:", "")),
  );

  const selectedTypeFilters = $derived(
    selectedFilters
      .filter((f) => f.startsWith("type:"))
      .map((f) => f.replace("type:", "")),
  );

  // Filtered Periods
  let filteredPeriods = $derived(
    periods.filter((p) => {
      const q = searchQuery.trim().toLowerCase();
      const matchesSearch =
        !q ||
        (p.name && p.name.toLowerCase().includes(q)) ||
        (p.name_ar && p.name_ar.toLowerCase().includes(q)) ||
        (p.code && p.code.toLowerCase().includes(q));

      const matchesYear =
        selectedYearFilters.length === 0 ||
        (p.year_id && selectedYearFilters.includes(p.year_id));

      const matchesStatus =
        selectedStatusFilters.length === 0 ||
        (selectedStatusFilters.includes("active") && p.status === "active") ||
        (selectedStatusFilters.includes("locked") && p.is_locked === 1);

      const matchesType =
        selectedTypeFilters.length === 0 ||
        (p.period_type && selectedTypeFilters.includes(p.period_type));

      return matchesSearch && matchesYear && matchesStatus && matchesType;
    }),
  );

  // Root periods (Semesters / Terms with no parent_id)
  let rootPeriods = $derived(
    filteredPeriods.filter((p) => !p.parent_id || p.parent_id === ""),
  );

  function getChildren(parentId: string) {
    return filteredPeriods.filter(
      (p) => p.parent_id === parentId && p.id !== parentId,
    );
  }

  function getYearName(yearId?: string | null): string | null {
    if (!yearId) return null;
    const y = academicCalendars.find((item) => item.id === yearId);
    return y ? (y.name_ar || y.name) : null;
  }

  function openCreateSheet() {
    editingPeriod = null;
    isSheetOpen = true;
  }

  function openEditSheet(period: any) {
    editingPeriod = period;
    isSheetOpen = true;
  }

  async function requestActivate(periodId: string) {
    const confirmed = await confirmStore.prompt({
      title: m.ui_copy_activate_period_03ace2fa(),
      description: m.settings_ui_period_open_confirm(),
      confirmText: m.ui_copy_activate_c2b12d71(),
      variant: "default",
    });

    if (!confirmed) return;

    try {
      await api.unlockAcademicPeriod(periodId);
      toast.success(m.ui_copy_period_activated_successfully_de714514());
      await loadPeriods();
    } catch (e: unknown) {
      toast.error(extractErrorMessage(e));
    }
  }

  async function requestLock(periodId: string) {
    const confirmed = await confirmStore.prompt({
      title: m.ui_copy_lock_close_period_57387762(),
      description: m.settings_ui_period_close_confirm(),
      confirmText: m.ui_copy_lock_period_54879736(),
      variant: "destructive",
    });

    if (!confirmed) return;

    try {
      await api.lockAcademicPeriod(periodId);
      toast.success(m.ui_copy_period_locked_successfully_c75e6ca2());
      await loadPeriods();
    } catch (e: unknown) {
      toast.error(extractErrorMessage(e));
    }
  }

  async function requestDelete(periodId: string) {
    const confirmed = await confirmStore.prompt({
      title: m.periods_manager_delete_confirm_title(),
      description: m.periods_manager_delete_confirm_desc(),
      confirmText: m.common_action_delete(),
      variant: "destructive",
    });

    if (!confirmed) return;

    try {
      await api.deleteAcademicPeriod(periodId);
      toast.success(m.periods_manager_delete_success());
      await loadPeriods();
    } catch (e: unknown) {
      toast.error(extractErrorMessage(e));
    }
  }

  function toggleCollapse(periodId: string) {
    collapsedNodes[periodId] = !collapsedNodes[periodId];
  }


  const relatedActionGroups = $derived<ActionGroup[]>([
    {
      id: "academic_calendars_nav",
      title: m.academic_calendar_title(),
      icon: Calendar,
      defaultExpanded: true,
      subLinks: [
        {
          id: "academic_calendars_settings",
          label: m.settings_nav_academic_calendars(),
          href: `/${orgId}/settings/academic-calendars`,
        },
        {
          id: "visual_calendar",
          label: m.academics_nav_calendar(),
          href: `/${orgId}/academics/calendar`,
        },
      ],
    },
    {
      id: "academic_promotion_policies",
      title: m.ui_copy_academic_policies_promotion_8c34613a(),
      icon: AcademicCap,
      defaultExpanded: true,
      subLinks: [
        {
          id: "promotion_wizard",
          label: m.ui_copy_student_promotion_wizard_e772861a(),
          href: `/${orgId}/academics/promotion`,
        },
        {
          id: "academic_policy",
          label: m.ui_copy_academic_policy_grading_73e34776(),
          href: `/${orgId}/settings/academic-policy`,
        },
        {
          id: "registration_rules",
          label: m.ui_copy_registration_enrollment_rules_34f1e148(),
          href: `/${orgId}/settings/registration`,
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
          {m.settings_periods_add()}
        </Button>
      </div>
    </div>
  </header>

  <main class="min-h-0 flex-1 space-y-4 overflow-y-auto p-4 scrollbar-hide sm:p-6">
  {#snippet periodNode(node: any, depth: number)}
    {#if depth <= 10}
      {@const children = getChildren(node.id)}
      {@const isCollapsed = !!collapsedNodes[node.id]}
      {@const isRoot = depth === 0}
      {@const yearName = getYearName(node.year_id)}

      <div
        class="flex flex-col w-full {isRoot
          ? 'py-3 border-b border-border/60 first:pt-0'
          : 'py-2 border-b border-border/20 last:border-b-0'}"
      >
        <!-- Node Row -->
        <div
          class="flex items-center justify-between gap-3"
          style="padding-inline-start: {depth > 0 ? depth * 1.5 : 0}rem;"
        >
          <!-- Left Info -->
          <div class="flex items-center gap-2.5 min-w-0">
            {#if children.length > 0}
              <button
                type="button"
                class="p-1 rounded hover:bg-muted text-muted-foreground transition-colors cursor-pointer shrink-0"
                onclick={() => toggleCollapse(node.id)}
              >
                {#if isCollapsed}
                  <ChevronRight size={15} class="rtl:rotate-180" />
                {:else}
                  <ChevronDown size={15} />
                {/if}
              </button>
            {:else}
              <div class="w-6 shrink-0"></div>
            {/if}

            <div class="flex items-center gap-2 min-w-0 flex-wrap">
              <span class="font-semibold text-foreground text-xs truncate">
                {node.name_ar || node.name}
              </span>

              <!-- Status Badges -->
              {#if node.status === "active"}
                <Badge
                  variant="info"
                  class="text-[9px] px-1.5 py-0 h-4 rounded-full font-bold bg-emerald-500/10 text-emerald-600 dark:text-emerald-400 border border-emerald-500/20"
                >
                  {m.common_phase_active()}
                </Badge>
              {:else}
                <Badge
                  variant="secondary"
                  class="text-[9px] px-1.5 py-0 h-4 rounded-full font-medium"
                >
                  {node.is_locked === 1
                    ? m.common_phase_locked()
                    : m.common_status_disabled()}
                </Badge>
              {/if}

              {#if node.code}
                <span class="text-[9px] font-mono px-1.5 py-0.5 rounded border border-border/50 text-muted-foreground bg-muted/30">
                  {node.code}
                </span>
              {/if}

              {#if yearName}
                <span class="text-[9px] px-1.5 py-0.5 rounded bg-primary/10 text-primary font-medium flex items-center gap-1">
                  <Calendar size={10} />
                  {yearName}
                </span>
              {/if}

              <span class="text-[10px] text-muted-foreground font-mono">
                {formatDate(node.start_date)} ↔ {formatDate(node.end_date)}
              </span>
            </div>
          </div>

          <!-- Right Actions -->
          <div class="flex items-center gap-1 shrink-0">
            {#if node.status !== "active" && node.is_locked !== 1}
              <Button
                variant="ghost"
                size="sm"
                class="text-[11px] h-7 px-2 font-medium text-emerald-600 hover:text-emerald-700 hover:bg-emerald-500/10 gap-1"
                onclick={() => requestActivate(node.id)}
              >
                <CheckCircle size={12} />
                <span class="hidden sm:inline">{m.ui_copy_activate_c2b12d71()}</span>
              </Button>
            {/if}

            {#if node.status === "active" && node.is_locked !== 1}
              <Button
                variant="ghost"
                size="sm"
                class="text-[11px] h-7 px-2 font-medium text-amber-600 hover:text-amber-700 hover:bg-amber-500/10 gap-1"
                onclick={() => requestLock(node.id)}
              >
                <Lock size={12} />
                <span class="hidden sm:inline">{m.common_td_ctx_lock()}</span>
              </Button>
            {/if}

            <Button
              variant="ghost"
              size="icon-sm"
              class="text-muted-foreground hover:text-foreground h-7 w-7"
              onclick={() => openEditSheet(node)}
              title={m.common_action_edit_simple()}
            >
              <Pencil size={12} />
            </Button>

            <Button
              variant="ghost"
              size="icon-sm"
              class="text-muted-foreground hover:text-destructive h-7 w-7"
              onclick={() => requestDelete(node.id)}
              title={m.common_action_delete()}
            >
              <Trash size={12} />
            </Button>
          </div>
        </div>

        <!-- Nested Children -->
        {#if !isCollapsed && children.length > 0}
          <div class="flex flex-col mt-1 ps-3 border-s border-border/30 ms-3">
            {#each children as child (child.id)}
              {@render periodNode(child, depth + 1)}
            {/each}
          </div>
        {/if}
      </div>
    {/if}
  {/snippet}

  <!-- Content List / Tree -->
  {#if isLoading}
    <div class="flex flex-col gap-3">
      <Skeleton class="h-14 w-full rounded-lg" />
      <Skeleton class="h-14 w-full rounded-lg" />
      <Skeleton class="h-14 w-full rounded-lg" />
    </div>
  {:else if rootPeriods.length === 0}
    <div class="flex flex-col items-center justify-center py-16 px-4 rounded-xl border border-dashed border-border/70 text-center">
      <div class="p-2.5 rounded-full bg-muted text-muted-foreground mb-3">
        <ClockRotateRight size={22} />
      </div>
      <h3 class="text-xs font-bold text-foreground mb-1">
        {searchQuery || selectedFilters.length > 0
          ? m.academic_calendars_empty_search()
          : m.periods_manager_empty_all()}
      </h3>
      <Button size="sm" variant="lime" class="text-xs h-8 px-3 font-semibold gap-1.5" onclick={openCreateSheet}>
        <Plus size={13} />
        {m.settings_periods_add()}
      </Button>
    </div>
  {:else}
    <div class="flex flex-col divide-y divide-border/40 rounded-xl border border-border/60 bg-card/50 p-3 sm:p-4" in:fade={{ duration: 150 }}>
      {#each rootPeriods as root (root.id)}
        {@render periodNode(root, 0)}
      {/each}
    </div>
  {/if}

  <!-- Contextual Related Actions (Full Width Footer) -->
  <RelatedActions
    title={m.ui_copy_related_support_bf36e16f()}
    groups={relatedActionGroups}
  />
  </main>
</div>

<!-- Sheets -->
<PeriodCreateSheet
  bind:open={isSheetOpen}
  {orgId}
  period={editingPeriod}
  onSuccess={handleRefresh}
/>
