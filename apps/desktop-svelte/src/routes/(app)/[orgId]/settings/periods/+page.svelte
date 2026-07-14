<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import { commands } from "$lib/core/ipc";
import { extractErrorMessage } from "$lib/core/ipc";
import { Button } from "@orbis/ui-svelte/button";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import { Badge } from "@orbis/ui-svelte/badge";
import { toast } from "svelte-sonner";
import { l } from "$lib/core/i18n";
import { fade } from "svelte/transition";
import {
  Plus,
  Calendar,
  Pencil,
  CheckCircle2,
  Lock as LockIcon,
  ChevronRight,
  ChevronDown,
  Award,
} from "lucide-svelte";
import PeriodCreateSheet from "$lib/features/academics/components/period-create-sheet.svelte";

let { data }: { data: { orgId: string } } = $props();

// State
let periods = $state<any[]>([]);
let isLoading = $state(true);
let error = $state<string | null>(null);

// Sheet controls
let isSheetOpen = $state(false);
let editingPeriod = $state<any>(null);

// Collapsed states for parent years
let collapsedYears = $state<Record<string, boolean>>({});

async function loadPeriods() {
  isLoading = true;
  error = null;
  try {
    const list = await commands.academic.listPeriods();
    periods = list;
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

// Group periods into hierarchical Year -> Term
let years = $derived(
  periods.filter((p) => p.parent_id === null || p.parent_id === undefined),
);

function getTermsForYear(periodId: string) {
  return periods.filter((p) => p.parent_id === periodId);
}

function openCreateSheet() {
  editingPeriod = null;
  isSheetOpen = true;
}

function openEditSheet(period: any) {
  editingPeriod = period;
  isSheetOpen = true;
}

async function handleActivate(periodId: string) {
  try {
    await commands.academic.lockPeriod(periodId);
    toast.success(m.settings_periods_activate_success());
    loadPeriods();
  } catch (e: unknown) {
    toast.error(extractErrorMessage(e));
  }
}

async function handleLock(periodId: string) {
  if (!confirm(m.settings_periods_lock_confirm())) return;
  try {
    await commands.academic.lockPeriod(periodId);
    toast.success(m.settings_periods_lock_success());
    loadPeriods();
  } catch (e: unknown) {
    toast.error(extractErrorMessage(e));
  }
}

function toggleCollapse(periodId: string) {
  collapsedYears[periodId] = !collapsedYears[periodId];
}
</script>

<svelte:head>
  <title>{m.settings_nav_cycles()} | Orbis</title>
</svelte:head>

<div class="h-full w-full overflow-y-auto p-6 flex flex-col gap-6 scrollbar-hide">
  <!-- Header Area -->
  <div class="flex items-center justify-between gap-4 font-sans border-b border-border/40 pb-5">
    <div class="flex items-center gap-2">
      {#if !isLoading}
        <span class="text-[10px] px-2 py-0.5 rounded bg-muted font-mono text-muted-foreground font-bold">
          {m.settings_periods_count({ count: periods.length })}
        </span>
      {/if}
    </div>

    <div class="flex items-center gap-2 shrink-0 flex-nowrap">
      <Button variant="lime" class="text-xs h-9 px-4 font-semibold" onclick={openCreateSheet}>
        <Plus size={14} class="me-1.5" />
        {m.settings_periods_add()}
      </Button>
    </div>
  </div>

    <!-- Main Content -->
    {#if isLoading}
      <div class="flex flex-col gap-4">
        <Skeleton class="h-20 w-full" />
        <Skeleton class="h-20 w-full" />
      </div>
    {:else}
      <div class="flex flex-col divide-y divide-border/60" in:fade={{ duration: 150 }}>
        {#each years as year (year.id)}
          {@const childTerms = getTermsForYear(year.id)}
          {@const isCollapsed = !!collapsedYears[year.id]}
          
          <div class="py-4 flex flex-col gap-2">
            <!-- Year Row/Header -->
            <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
              <!-- Left Info -->
              <div class="flex items-start gap-3">
                <button
                  type="button"
                  class="p-1 hover:bg-accent rounded text-muted-foreground mt-0.5 cursor-pointer"
                  onclick={() => toggleCollapse(year.id)}
                >
                  {#if isCollapsed}
                    <ChevronRight size={16} class="rtl:rotate-180" />
                  {:else}
                    <ChevronDown size={16} />
                  {/if}
                </button>
                
                <div class="flex flex-col text-start gap-1">
                  <div class="flex items-center gap-2 flex-wrap">
                    <h3 class="text-sm font-bold text-foreground">
                      {l(year.name_ar, year.name)}
                    </h3>
                    
                    <!-- Status Badges -->
                    {#if year.status === "active"}
                      <Badge variant="lime" class="text-[9px] px-2 py-0 h-4.5 rounded-full font-bold">
                        {m.common_phase_active()}
                      </Badge>
                    {:else}
                      <Badge variant="secondary" class="text-[9px] px-2 py-0 h-4.5 rounded-full font-bold">
                        {year.is_locked === 1 ? m.common_phase_locked() : m.common_status_disabled()}
                      </Badge>
                    {/if}
                    
                    {#if year.code}
                      <span class="text-[9px] font-mono px-1.5 py-0.5 rounded border border-border/60 text-muted-foreground bg-muted/20 font-bold">
                        {year.code}
                      </span>
                    {/if}
                  </div>
                  
                  <span class="text-[10px] text-muted-foreground font-mono">
                    {year.start_date} ↔ {year.end_date}
                  </span>
                </div>
              </div>

              <!-- Right Actions -->
              <div class="flex items-center gap-2 sm:justify-end">
                {#if year.status !== "active" && year.is_locked !== 1}
                  <Button
                    variant="outline"
                    class="text-[10px] h-7 px-2.5 font-semibold text-emerald-500 hover:text-emerald-600 hover:bg-emerald-500/5 border-emerald-500/20"
                    onclick={() => handleActivate(year.id)}
                  >
                    <CheckCircle2 size={12} class="me-1" />
                    {m.common_word_status()}
                  </Button>
                {/if}

                {#if year.status === "active" && year.is_locked !== 1}
                  <Button
                    variant="outline"
                    class="text-[10px] h-7 px-2.5 font-semibold text-rose-500 hover:text-rose-600 hover:bg-rose-500/5 border-rose-500/20"
                    onclick={() => handleLock(year.id)}
                  >
                    <LockIcon size={12} class="me-1" />
                    {m.common_td_ctx_lock()}
                  </Button>
                {/if}

                <Button
                  variant="outline"
                  class="text-[10px] h-7 px-2.5 font-semibold text-foreground"
                  onclick={() => openEditSheet(year)}
                >
                  <Pencil size={12} class="me-1" />
                  {m.common_action_edit_simple()}
                </Button>
              </div>
            </div>

            <!-- Nested Terms -->
            {#if !isCollapsed}
              <div class="ps-8 pe-4 py-2 flex flex-col gap-2 transition-all duration-200">
                {#if childTerms.length === 0}
                  <div class="text-[10px] text-muted-foreground/60 py-3 text-start select-none">
                    {m.settings_periods_no_terms()}
                  </div>
                {:else}
                  {#each childTerms as term (term.id)}
                    <div class="flex items-center justify-between py-2 border-b border-border/20 last:border-b-0">
                      <div class="flex flex-col text-start gap-1">
                        <div class="flex items-center gap-2">
                          <span class="text-xs font-bold text-foreground">
                            {l(term.name_ar, term.name)}
                          </span>
                          
                          {#if term.status === "active"}
                            <Badge variant="lime" class="text-[8px] px-1.5 py-0 h-4 rounded font-bold">
                              {m.common_phase_active()}
                            </Badge>
                          {:else if term.is_locked === 1}
                            <Badge variant="destructive" class="text-[8px] px-1.5 py-0 h-4 rounded font-bold">
                              {m.common_phase_locked()}
                            </Badge>
                          {/if}

                          <span class="text-[9px] text-muted-foreground flex items-center gap-1">
                            <Award size={10} />
                            {term.grading_system}
                          </span>
                        </div>
                        
                        <span class="text-[9px] text-muted-foreground font-mono leading-none">
                          {term.start_date} ↔ {term.end_date}
                        </span>
                      </div>

                      <div class="flex items-center gap-2">
                        {#if term.status !== "active" && term.is_locked !== 1}
                          <Button
                            variant="ghost"
                            class="text-[9px] h-6 px-2 text-emerald-500 hover:bg-emerald-500/5"
                            onclick={() => handleActivate(term.id)}
                          >
                            {m.common_word_status()}
                          </Button>
                        {/if}
                        {#if term.status === "active" && term.is_locked !== 1}
                          <Button
                            variant="ghost"
                            class="text-[9px] h-6 px-2 text-rose-500 hover:bg-rose-500/5"
                            onclick={() => handleLock(term.id)}
                          >
                            {m.common_td_ctx_lock()}
                          </Button>
                        {/if}
                        
                        <Button
                          variant="ghost"
                          class="text-[9px] h-6 px-2"
                          onclick={() => openEditSheet(term)}
                        >
                          {m.common_action_edit_simple()}
                        </Button>
                      </div>
                    </div>
                  {/each}
                {/if}
              </div>
            {/if}
          </div>
        {/each}
      </div>
    {/if}
</div>

<!-- Sheets -->
<PeriodCreateSheet
  bind:open={isSheetOpen}
  orgId={data.orgId}
  period={editingPeriod}
  onSuccess={loadPeriods}
/>
