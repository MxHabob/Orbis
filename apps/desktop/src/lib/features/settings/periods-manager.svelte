<!--
@component PeriodsManager
Allows administrators to manage academic periods, terms, semesters, and locking states.
Grouped by Academic Year with real backend deletion and lifecycle integration.
-->
<script lang="ts">
  import { api, extractErrorMessage } from "$lib/api";
  import PeriodCreateSheet from "$lib/features/academics/components/period-create-sheet.svelte";
  import * as m from "$lib/paraglide/messages";
  import { confirmStore } from "$lib/stores";
  import type { AcademicPeriodDto, AcademicYearDto } from "@orbis/sdk";
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
    Trash,
  } from "@orbis/icons";
  import { toast } from "@orbis/ui";
  import { Badge } from "@orbis/ui/badge";
  import { Button } from "@orbis/ui/button";
  import { Skeleton } from "@orbis/ui/skeleton";
  import RelatedActions, {
    type ActionGroup,
  } from "$lib/shared/components/related-actions.svelte";
  import { onMount } from "svelte";
  import { fade } from "svelte/transition";

  let { orgId }: { orgId: string } = $props();

  // State
  let periods = $state<AcademicPeriodDto[]>([]);
  let academicYears = $state<AcademicYearDto[]>([]);
  let selectedYearId = $state<string>("all");
  let isLoading = $state(true);
  let error = $state<string | null>(null);

  // Sheet controls
  let isSheetOpen = $state(false);
  let editingPeriod = $state<any>(null);

  // Collapsed states for parent nodes
  let collapsedNodes = $state<Record<string, boolean>>({});

  async function loadPeriods() {
    isLoading = true;
    error = null;
    try {
      const [periodsList, yearsList] = await Promise.all([
        api.listAcademicPeriods(),
        api.listAcademicYears().catch(() => []),
      ]);
      periods = periodsList || [];
      academicYears = yearsList || [];

      // If active year exists and no filter chosen, default to active year or "all"
      if (selectedYearId === "all" && yearsList && yearsList.length > 0) {
        const active = yearsList.find((y) => y.status === "active");
        if (active) {
          selectedYearId = active.id;
        }
      }
    } catch (e: unknown) {
      error = extractErrorMessage(e);
      toast.error(extractErrorMessage(e));
    } finally {
      isLoading = false;
    }
  }

  onMount(() => {
    loadPeriods();
  });

  // Filtered Periods by Selected Year
  let filteredPeriods = $derived(
    selectedYearId === "all"
      ? periods
      : periods.filter((p) => p.year_id === selectedYearId || !p.year_id),
  );

  // Root periods (Semesters / Terms with no parent_id)
  let rootPeriods = $derived(
    filteredPeriods.filter((p) => !p.parent_id || p.parent_id === ""),
  );

  const selectedYear = $derived(
    academicYears.find((y) => y.id === selectedYearId),
  );

  const relatedActionGroups = $derived<ActionGroup[]>([
    {
      id: "academic_years_nav",
      title: m.academic_calendar_title(),
      icon: Calendar,
      defaultExpanded: true,
      subLinks: [
        {
          id: "academic_years_settings",
          label: m.settings_nav_academic_years(),
          href: `/${orgId}/settings/academic-years`,
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

  function getChildren(parentId: string) {
    return filteredPeriods.filter(
      (p) => p.parent_id === parentId && p.id !== parentId,
    );
  }

  function getYearName(yearId?: string | null): string | null {
    if (!yearId) return null;
    const y = academicYears.find((item) => item.id === yearId);
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
      toast.success(
        m.ui_copy_period_activated_successfully_de714514(),
      );
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
      toast.success(
        m.ui_copy_period_locked_successfully_c75e6ca2(),
      );
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
</script>

<div class="h-full w-full overflow-y-auto p-6 flex flex-col gap-6 scrollbar-hide">
  <!-- Header Area -->
  <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4 font-sans border-b border-border/40 pb-5">
    <div class="flex flex-col gap-1">
      <div class="flex items-center gap-2">
        <h1 class="text-xl font-bold text-foreground">
          {m.settings_nav_cycles()}
        </h1>
        {#if !isLoading}
          <span class="text-[10px] px-2 py-0.5 rounded bg-muted font-mono text-muted-foreground font-bold">
            {m.settings_periods_count({ count: filteredPeriods.length })}
          </span>
        {/if}
      </div>
      <p class="text-xs text-muted-foreground">
        {m.periods_manager_empty_desc() || ""}
      </p>
    </div>

    <div class="flex items-center gap-2 shrink-0 flex-nowrap">
      <a
        href={`/${orgId}/settings/academic-years`}
        class="inline-flex items-center gap-1.5 text-xs h-9 px-3 font-semibold rounded-md border border-border/60 hover:bg-muted/40 transition-colors text-foreground"
      >
        <Calendar size={14} />
        <span>{m.settings_nav_academic_years()}</span>
      </a>
      <Button class="text-xs h-9 px-4 font-semibold" onclick={openCreateSheet}>
        <Plus size={14} class="me-1.5" />
        {m.settings_periods_add()}
      </Button>
    </div>
  </div>

  <!-- Academic Year Filter & Info Banner -->
  <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-3 p-3.5 rounded-xl border border-border/60 bg-card/60">
    <div class="flex items-center gap-2 flex-wrap">
      <span class="text-xs font-semibold text-muted-foreground flex items-center gap-1.5">
        <Calendar size={14} class="text-primary" />
        {m.periods_manager_year_label()}
      </span>

      <!-- Year Filter Buttons -->
      <div class="flex items-center gap-1 flex-wrap">
        <Button
          variant={selectedYearId === "all" ? "secondary" : "ghost"}
          size="sm"
          class="text-xs h-7 px-2.5 rounded-md font-medium"
          onclick={() => (selectedYearId = "all")}
        >
          {m.periods_manager_all_years()}
        </Button>

        {#each academicYears as y (y.id)}
          <Button
            variant={selectedYearId === y.id ? "secondary" : "ghost"}
            size="sm"
            class="text-xs h-7 px-2.5 rounded-md font-medium gap-1.5 {selectedYearId === y.id ? 'border border-primary/30' : ''}"
            onclick={() => (selectedYearId = y.id)}
          >
            <span>{y.name_ar || y.name}</span>
            {#if y.status === "active"}
              <span class="w-1.5 h-1.5 rounded-full bg-emerald-500"></span>
            {/if}
          </Button>
        {/each}
      </div>
    </div>

    {#if selectedYear}
      <div class="text-[11px] font-mono text-muted-foreground flex items-center gap-2 self-end sm:self-auto">
        <span>{selectedYear.start_date?.slice(0, 10)} ↔ {selectedYear.end_date?.slice(0, 10)}</span>
        {#if selectedYear.status === "active"}
          <Badge variant="info" class="text-[9px] px-1.5 py-0 rounded font-bold bg-emerald-500/10 text-emerald-600 dark:text-emerald-400">
            {m.academic_calendar_active_badge()}
          </Badge>
        {/if}
      </div>
    {/if}
  </div>

  {#snippet periodNode(node: any, depth: number)}
    {#if depth <= 10}
      {@const children = getChildren(node.id)}
      {@const isCollapsed = !!collapsedNodes[node.id]}
      {@const isRoot = depth === 0}
      {@const yearName = getYearName(node.year_id)}

      <div
        class="flex flex-col w-full {isRoot
          ? 'py-4 gap-2 border-b border-border/60'
          : 'py-2 border-b border-border/20 last:border-b-0'}"
      >
        <!-- Node Header -->
        <div
          class="flex flex-col sm:flex-row sm:items-center justify-between gap-4"
          style="padding-inline-start: {depth > 0 ? depth * 1.5 : 0}rem;"
        >
          <!-- Left Info -->
          <div class="flex items-start gap-3">
            {#if children.length > 0 || isRoot}
              <button
                type="button"
                class="p-1 hover:bg-accent rounded text-muted-foreground mt-0.5 cursor-pointer {children.length ===
                0
                  ? 'invisible'
                  : ''}"
                onclick={() => toggleCollapse(node.id)}
              >
                {#if isCollapsed}
                  <ChevronRight size={16} class="rtl:rotate-180" />
                {:else}
                  <ChevronDown size={16} />
                {/if}
              </button>
            {:else}
              <div class="w-6 h-6 mt-0.5 shrink-0"></div>
            {/if}

            <div class="flex flex-col text-start gap-1">
              <div class="flex items-center gap-2 flex-wrap">
                <h3
                  class="{isRoot
                    ? 'text-sm'
                    : 'text-xs'} font-bold text-foreground"
                >
                  {node.name_ar || node.name}
                </h3>

                <!-- Status Badges -->
                {#if node.status === "active"}
                  <Badge
                    variant="info"
                    class="text-[9px] px-2 py-0 h-4.5 rounded-full font-bold bg-emerald-500/10 text-emerald-600 dark:text-emerald-400 border border-emerald-500/20"
                  >
                    {m.common_phase_active()}
                  </Badge>
                {:else}
                  <Badge
                    variant="secondary"
                    class="text-[9px] px-2 py-0 h-4.5 rounded-full font-bold"
                  >
                    {node.is_locked === 1
                      ? m.common_phase_locked()
                      : m.common_status_disabled()}
                  </Badge>
                {/if}

                {#if node.code}
                  <span
                    class="text-[9px] font-mono px-1.5 py-0.5 rounded border border-border/60 text-muted-foreground bg-muted/20 font-bold"
                  >
                    {node.code}
                  </span>
                {/if}

                {#if isRoot && selectedYearId === "all" && yearName}
                  <span class="text-[9px] px-1.5 py-0.5 rounded bg-primary/10 text-primary font-medium flex items-center gap-1">
                    <Calendar size={10} />
                    {yearName}
                  </span>
                {/if}

                {#if !isRoot && node.grading_system}
                  <span
                    class="text-[9px] text-muted-foreground flex items-center gap-1"
                  >
                    <AcademicCap size={10} />
                    {node.grading_system}
                  </span>
                {/if}
              </div>

              <span class="text-[10px] text-muted-foreground font-mono">
                {node.start_date?.slice(0, 10)} ↔ {node.end_date?.slice(0, 10)}
              </span>
            </div>
          </div>

          <!-- Right Actions -->
          <div class="flex items-center gap-2 sm:justify-end">
            {#if node.status !== "active" && node.is_locked !== 1}
              <Button
                variant={isRoot ? "outline" : "ghost"}
                class="text-[10px] h-7 px-2.5 font-semibold text-emerald-500 hover:text-emerald-600 hover:bg-emerald-500/5 {isRoot
                  ? 'border-emerald-500/20'
                  : ''}"
                onclick={() => requestActivate(node.id)}
              >
                {#if isRoot}<CheckCircle size={12} class="me-1" />{/if}
                {m.common_word_status()}
              </Button>
            {/if}

            {#if node.status === "active" && node.is_locked !== 1}
              <Button
                variant="outline"
                class="text-[10px] h-7 px-2.5 font-semibold"
                onclick={() => requestLock(node.id)}
              >
                {#if isRoot}<Lock size={12} class="me-1" />{/if}
                {m.common_td_ctx_lock()}
              </Button>
            {/if}

            <Button
              variant="outline"
              class="text-[10px] h-7 px-2.5 font-semibold"
              onclick={() => openEditSheet(node)}
            >
              {#if isRoot}<Pencil size={12} class="me-1" />{/if}
              {m.common_action_edit_simple()}
            </Button>

            <Button
              variant="destructive"
              class="text-[10px] h-7 px-2.5 font-semibold"
              onclick={() => requestDelete(node.id)}
            >
              <Trash size={12} />
            </Button>
          </div>
        </div>

        <!-- Nested Children -->
        {#if !isCollapsed}
          <div class="flex flex-col transition-all duration-200">
            {#if children.length === 0 && isRoot}
              <div
                class="text-[10px] text-muted-foreground/60 py-3 text-start select-none ps-8"
              >
                {m.settings_periods_no_terms()}
              </div>
            {:else if children.length > 0}
              <div class="flex flex-col mt-2">
                {#each children as child (child.id)}
                  {@render periodNode(child, depth + 1)}
                {/each}
              </div>
            {/if}
          </div>
        {/if}
      </div>
    {/if}
  {/snippet}

  <!-- Main Content -->
  {#if isLoading}
    <div class="flex flex-col gap-4">
      <Skeleton class="h-20 w-full" />
      <Skeleton class="h-20 w-full" />
    </div>
  {:else if rootPeriods.length === 0}
    <div class="flex flex-col items-center justify-center py-12 px-4 rounded-xl border border-dashed border-border/70 text-center">
      <div class="p-3 rounded-full bg-primary/10 text-primary mb-3">
        <ClockRotateRight size={24} />
      </div>
      <h3 class="text-sm font-bold text-foreground mb-1">
        {selectedYearId !== "all"
          ? m.periods_manager_empty_year()
          : m.periods_manager_empty_all()}
      </h3>
      <p class="text-xs text-muted-foreground max-w-sm mb-4">
        {m.periods_manager_empty_desc()}
      </p>
      <Button size="sm" class="text-xs font-semibold gap-1.5" onclick={openCreateSheet}>
        <Plus size={14} />
        {m.settings_periods_add()}
      </Button>
    </div>
  {:else}
    <div
      class="flex flex-col divide-y divide-border/60"
      in:fade={{ duration: 150 }}
    >
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
</div>

<!-- Sheets -->
<PeriodCreateSheet
  bind:open={isSheetOpen}
  {orgId}
  period={editingPeriod}
  onSuccess={loadPeriods}
/>
