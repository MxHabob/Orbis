<script lang="ts">
import { onMount } from "svelte";
import { commands } from "$lib/services/ipc";
import { Button } from "@orbis/ui-svelte/button";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import { toast } from "svelte-sonner";
import { getLocale } from "$lib/paraglide/runtime";
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
import CycleCreateSheet from "$lib/components/sheets/cycle-create-sheet.svelte";
import * as m from "$lib/paraglide/messages";

let { data }: { data: { orgId: string } } = $props();

// State
let cycles = $state<any[]>([]);
let isLoading = $state(true);
let error = $state<string | null>(null);

// Sheet controls
let isSheetOpen = $state(false);
let editingCycle = $state<any>(null);

// Collapsed states for parent years
let collapsedYears = $state<Record<string, boolean>>({});

async function loadCycles() {
  isLoading = true;
  error = null;
  try {
    const list = await commands.academics.getCycles(data.orgId);
    cycles = list;
  } catch (e) {
    error = e instanceof Error ? e.message : String(e);
    toast.error("Failed to load academic cycles");
  } finally {
    isLoading = false;
  }
}

onMount(() => {
  if (data.orgId) {
    loadCycles();
  }
});

const years = $derived(cycles.filter((c) => c.cycle_type === "year"));
const terms = $derived(cycles.filter((c) => c.cycle_type === "term"));

function getTermsForYear(yearId: string) {
  return terms.filter((t) => String(t.parent_id) === String(yearId));
}

function openCreateSheet() {
  editingCycle = null;
  isSheetOpen = true;
}

function openEditSheet(cycle: any) {
  editingCycle = cycle;
  isSheetOpen = true;
}

async function handleActivate(cycleId: string) {
  try {
    await commands.academics.activateCycle(data.orgId, cycleId);
    toast.success(m.settings_cycles_activate_success());
    loadCycles();
  } catch (e) {
    const errorMsg = e instanceof Error ? e.message : String(e);
    toast.error(errorMsg);
  }
}

async function handleLock(cycleId: string) {
  if (
    !confirm(
      "Are you sure you want to lock this cycle? This prevents further edits to marks/enrollment.",
    )
  )
    return;
  try {
    await commands.academics.lockCycle(cycleId);
    toast.success(m.settings_cycles_lock_success());
    loadCycles();
  } catch (e) {
    const errorMsg = e instanceof Error ? e.message : String(e);
    toast.error(errorMsg);
  }
}

function toggleCollapse(yearId: string) {
  collapsedYears[yearId] = !collapsedYears[yearId];
}
</script>

<svelte:head>
  <title>{m.settings_cycles_title()} | Orbis</title>
</svelte:head>

<div class="h-full w-full overflow-y-auto p-6 flex flex-col gap-6 scrollbar-hide">
  <!-- Header Area -->
  <div class="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4 font-sans border-b border-border/40 pb-5">
    <div class="flex flex-col gap-1 text-start">
      <h1 class="text-xl font-bold tracking-tight text-foreground flex items-center gap-2">
        <span>{m.settings_cycles_title()}</span>
        {#if !isLoading}
          <span class="text-[10px] px-1.5 py-0.5 rounded bg-muted font-mono text-muted-foreground font-bold">
            {cycles.length}
          </span>
        {/if}
      </h1>
      <p class="text-xs text-muted-foreground">{m.settings_cycles_desc()}</p>
    </div>

    <Button variant="lime" class="text-xs h-9 px-4 font-semibold" onclick={openCreateSheet}>
      <Plus size={14} class={getLocale() === "ar" ? 'ml-1.5' : 'mr-1.5'} />
      {m.settings_cycles_add()}
    </Button>
  </div>

  <!-- Main Content -->
  {#if isLoading}
    <div class="flex flex-col gap-4">
      <Skeleton class="h-28 w-full" />
      <Skeleton class="h-28 w-full" />
    </div>
  {:else if error}
    <div class="flex flex-col items-center justify-center p-8 border border-destructive/20 bg-destructive/5 rounded-xl text-center gap-3">
      <p class="text-xs text-destructive font-semibold">Failed to load academic cycles</p>
      <p class="text-xs text-muted-foreground">{error}</p>
      <Button variant="outline" class="text-xs" onclick={loadCycles}>{m.error_retry()}</Button>
    </div>
  {:else if years.length === 0}
    <div class="flex flex-col items-center justify-center py-16 text-muted-foreground border border-border border-dashed rounded-xl gap-4 bg-card/10">
      <Calendar size={48} class="text-muted-foreground/30" />
      <div class="flex flex-col gap-1 text-center">
        <p class="text-xs font-semibold">{m.settings_cycles_empty()}</p>
        <p class="text-[10px] text-muted-foreground/80">{m.settings_cycles_empty_desc()}</p>
      </div>
    </div>
  {:else}
    <div class="flex flex-col gap-4" in:fade={{ duration: 150 }}>
      {#each years as year (year.id)}
        {@const childTerms = getTermsForYear(year.id)}
        {@const isCollapsed = !!collapsedYears[year.id]}
        
        <div class="border border-border rounded-xl bg-card/25 overflow-hidden flex flex-col">
          <!-- Year Row/Header -->
          <div class="flex flex-col sm:flex-row sm:items-center justify-between p-5 gap-4 bg-accent/5">
            <!-- Left Info -->
            <div class="flex items-start gap-3">
              <button
                type="button"
                class="p-1 hover:bg-accent rounded text-muted-foreground mt-0.5 cursor-pointer"
                onclick={() => toggleCollapse(year.id)}
              >
                {#if isCollapsed}
                  <ChevronRight size={16} class={getLocale() === 'ar' ? 'rotate-180' : ''} />
                {:else}
                  <ChevronDown size={16} />
                {/if}
              </button>
              
              <div class="flex flex-col text-start gap-1">
                <div class="flex items-center gap-2 flex-wrap">
                  <h3 class="text-sm font-bold text-foreground">
                    {getLocale() === "ar" && year.name_ar ? year.name_ar : year.name}
                  </h3>
                  
                  <!-- Status Badges -->
                  {#if year.is_current === 1}
                    <span class="text-[9px] px-2 py-0.5 rounded-full font-bold bg-emerald-500/10 text-emerald-500 border border-emerald-500/20">
                      {m.settings_cycles_status_active()}
                    </span>
                  {:else if year.is_locked === 1}
                    <span class="text-[9px] px-2 py-0.5 rounded-full font-bold bg-rose-500/10 text-rose-500 border border-rose-500/20">
                      {m.settings_cycles_status_locked()}
                    </span>
                  {:else}
                    <span class="text-[9px] px-2 py-0.5 rounded-full font-bold bg-muted text-muted-foreground border border-border/40">
                      {m.settings_cycles_status_inactive()}
                    </span>
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
              <!-- Activate -->
              {#if year.is_current !== 1 && year.is_locked !== 1}
                <Button
                  variant="outline"
                  class="text-[10px] h-7 px-2.5 font-semibold text-emerald-500 hover:text-emerald-600 hover:bg-emerald-500/5 border-emerald-500/20"
                  onclick={() => handleActivate(year.id)}
                >
                  <CheckCircle2 size={12} class={getLocale() === 'ar' ? 'ml-1' : 'mr-1'} />
                  {m.settings_cycles_activate()}
                </Button>
              {/if}

              <!-- Lock -->
              {#if year.is_current === 1 && year.is_locked !== 1}
                <Button
                  variant="outline"
                  class="text-[10px] h-7 px-2.5 font-semibold text-rose-500 hover:text-rose-600 hover:bg-rose-500/5 border-rose-500/20"
                  onclick={() => handleLock(year.id)}
                >
                  <LockIcon size={12} class={getLocale() === 'ar' ? 'ml-1' : 'mr-1'} />
                  {m.settings_cycles_lock()}
                </Button>
              {/if}

              <!-- Edit -->
              <Button
                variant="outline"
                class="text-[10px] h-7 px-2.5 font-semibold text-foreground"
                onclick={() => openEditSheet(year)}
              >
                <Pencil size={12} class={getLocale() === 'ar' ? 'ml-1' : 'mr-1'} />
                Edit
              </Button>
            </div>
          </div>

          <!-- Nested Terms (shown if not collapsed and there are child terms) -->
          {#if !isCollapsed}
            <div class="border-t border-border/40 pl-6 pr-6 py-2 flex flex-col gap-2 bg-background/30 transition-all duration-200">
              {#if childTerms.length === 0}
                <div class="text-[10px] text-muted-foreground/60 py-3 text-start select-none">
                  No academic terms registered for this year.
                </div>
              {:else}
                {#each childTerms as term (term.id)}
                  <div class="flex items-center justify-between p-3 rounded-lg border border-border/40 bg-card/40 hover:bg-accent/5 my-1">
                    <div class="flex flex-col text-start gap-1">
                      <div class="flex items-center gap-2">
                        <span class="text-xs font-bold text-foreground">
                          {getLocale() === "ar" && term.name_ar ? term.name_ar : term.name}
                        </span>
                        
                        {#if term.is_current === 1}
                          <span class="text-[8px] px-1.5 py-0.2 rounded bg-emerald-500/10 text-emerald-500 font-bold">
                            Active
                          </span>
                        {:else if term.is_locked === 1}
                          <span class="text-[8px] px-1.5 py-0.2 rounded bg-rose-500/10 text-rose-500 font-bold">
                            Locked
                          </span>
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
                      {#if term.is_current !== 1 && term.is_locked !== 1}
                        <Button
                          variant="ghost"
                          class="text-[9px] h-6 px-2 text-emerald-500 hover:bg-emerald-500/5"
                          onclick={() => handleActivate(term.id)}
                        >
                          Activate
                        </Button>
                      {/if}
                      {#if term.is_current === 1 && term.is_locked !== 1}
                        <Button
                          variant="ghost"
                          class="text-[9px] h-6 px-2 text-rose-500 hover:bg-rose-500/5"
                          onclick={() => handleLock(term.id)}
                        >
                          Lock
                        </Button>
                      {/if}
                      
                      <Button
                        variant="ghost"
                        class="text-[9px] h-6 px-2"
                        onclick={() => openEditSheet(term)}
                      >
                        Edit
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
<CycleCreateSheet
  bind:open={isSheetOpen}
  orgId={data.orgId}
  cycle={editingCycle}
  onSuccess={loadCycles}
/>
