<!--
@component PeriodsManager
Allows administrators to manage academic periods, terms, semesters, and locking states.
-->
<script lang="ts">
  import { api, extractErrorMessage } from "$lib/api";
  import { l } from "$lib/core/i18n";
  import PeriodCreateSheet from "$lib/features/academics/components/period-create-sheet.svelte";
  import * as m from "$lib/paraglide/messages";
  import { confirmStore } from "$lib/stores";
  import { CheckCircle, ChevronDown, ChevronRight, Lock, Pencil, Plus, Trash, AcademicCap } from "@orbis/icons";
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
  let periods = $state<any[]>([]);
  let academicYears = $state<any[]>([]);
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

  // Root periods (Semesters / Terms with no parent_id)
  let rootPeriods = $derived(
    periods.filter((p) => !p.parent_id || p.parent_id === ""),
  );

  const relatedActionGroups = $derived<ActionGroup[]>([
    {
      id: "academic_promotion_policies",
      title: l("السياسات والعمليات المرتبطة", "Academic Policies & Promotion"),
      icon: AcademicCap,
      defaultExpanded: true,
      subLinks: [
        {
          id: "promotion_wizard",
          label: l("معالج ترحيل الطلاب وترفيع الفصول", "Student Promotion Wizard"),
          href: `/${orgId}/academics/promotion`,
        },
        {
          id: "academic_policy",
          label: l("السياسات واللوائح الأكاديمية", "Academic Policy & Grading"),
          href: `/${orgId}/settings/academic-policy`,
        },
        {
          id: "registration_rules",
          label: l("قواعد وشروط التسجيل والقبول", "Registration & Enrollment Rules"),
          href: `/${orgId}/settings/registration`,
        },
      ],
    },
  ]);

  function getChildren(parentId: string) {
    return periods.filter((p) => p.parent_id === parentId && p.id !== parentId);
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
      title: l("تنشيط الفترة الدراسية؟", "Activate Period?"),
      description: l(
        "سيتم فتح إدخال الدرجات والتسجيل للفترة المختارة.",
        "Grade entry and enrollment will be enabled for this period.",
      ),
      confirmText: l("تنشيط", "Activate"),
      variant: "default",
    });

    if (!confirmed) return;

    try {
      await api.unlockAcademicPeriod(periodId);
      toast.success(
        l("تم تنشيط الفترة بنجاح", "Period activated successfully"),
      );
      loadPeriods();
    } catch (e: unknown) {
      toast.error(extractErrorMessage(e));
    }
  }

  async function requestLock(periodId: string) {
    const confirmed = await confirmStore.prompt({
      title: l("إغلاق وقفل الفترة الدراسية؟", "Lock / Close Period?"),
      description: l(
        "سيتم قفل إدخال الدرجات وتجميد العمليات الأكاديمية لهذه الفترة.",
        "Grade entry, attendance, and enrollments will be locked for this period.",
      ),
      confirmText: l("قفل الفترة", "Lock Period"),
      variant: "destructive",
    });

    if (!confirmed) return;

    try {
      await api.lockAcademicPeriod(periodId);
      toast.success(
        l("تم قفل الفترة الدراسية بنجاح", "Period locked successfully"),
      );
      loadPeriods();
    } catch (e: unknown) {
      toast.error(extractErrorMessage(e));
    }
  }

  async function requestDelete(periodId: string) {
    const confirmed = await confirmStore.prompt({
      title: m.common_action_delete() + "؟",
      description: m.common_onboarding_delete_building_confirm(),
      confirmText: m.common_action_delete(),
      variant: "destructive",
    });

    if (!confirmed) return;

    try {
      toast.success(m.common_action_delete());
      loadPeriods();
    } catch (e: unknown) {
      toast.error(extractErrorMessage(e));
    }
  }

  function toggleCollapse(periodId: string) {
    collapsedNodes[periodId] = !collapsedNodes[periodId];
  }
</script>

<div
  class="h-full w-full overflow-y-auto p-6 flex flex-col gap-6 scrollbar-hide"
>
  <!-- Header Area -->
  <div
    class="flex items-center justify-between gap-4 font-sans border-b border-border/40 pb-5"
  >
    <div class="flex items-center gap-2">
      {#if !isLoading}
        <span
          class="text-[10px] px-2 py-0.5 rounded bg-muted font-mono text-muted-foreground font-bold"
        >
          {m.settings_periods_count({ count: periods.length })}
        </span>
      {/if}
    </div>

    <div class="flex items-center gap-2 shrink-0 flex-nowrap">
      <Button class="text-xs h-9 px-4 font-semibold" onclick={openCreateSheet}>
        <Plus size={14} class="me-1.5" />
        {m.settings_periods_add()}
      </Button>
    </div>
  </div>

  {#snippet periodNode(node: any, depth: number)}
    {#if depth <= 10}
      {@const children = getChildren(node.id)}
      {@const isCollapsed = !!collapsedNodes[node.id]}
      {@const isRoot = depth === 0}

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
                  {l(node.name_ar, node.name)}
                </h3>

                <!-- Status Badges -->
                {#if node.status === "active"}
                  <Badge
                    variant="info"
                    class="text-[9px] px-2 py-0 h-4.5 rounded-full font-bold"
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
                {node.start_date} ↔ {node.end_date}
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
    title={l("العمليات والإجراءات المرتبطة", "Related support")}
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
