<script lang="ts">
import { onMount } from "svelte";
import { commands } from "$lib/services/ipc";
import { Button } from "@orbis/ui-svelte/button";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import { Badge } from "@orbis/ui-svelte/badge";
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
    error = (e as any)?.message || "Failed to load academic cycles";
    toast.error("Failed to load academic cycles");
  } finally {
    isLoading = false;
  }
}

onMount(() => {
  loadCycles();
});

// Group cycles into hierarchical Year -> Term
let years = $derived(cycles.filter((c) => c.parent_id === null || c.parent_id === undefined));

function getTermsForYear(yearId: string) {
  return cycles.filter((c) => c.parent_id === yearId);
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
    await commands.academics.activateCycle(cycleId);
    toast.success(m.settings_cycles_activate_success());
    loadCycles();
  } catch (e) {
    const errorMsg = (e as any)?.message || "Activation failed";
    toast.error(errorMsg);
  }
}

async function handleLock(cycleId: string) {
  if (!confirm(m.settings_cycles_lock_confirm())) return;
  try {
    await commands.academics.lockCycle(cycleId);
    toast.success(m.settings_cycles_lock_success());
    loadCycles();
  } catch (e) {
    const errorMsg = (e as any)?.message || "Locking failed";
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

<div class="h-full w-full overflow-y-auto bg-background scrollbar-hide">
  <div class="max-w-3xl mx-auto p-8 flex flex-col gap-10">
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

      <div class="flex items-center gap-2 shrink-0 flex-nowrap">
        <Button variant="lime" class="text-xs h-9 px-4 font-semibold" onclick={openCreateSheet}>
          <Plus size={14} class={getLocale() === "ar" ? 'ml-1.5' : 'mr-1.5'} />
          {m.settings_cycles_add()}
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
                      <Badge variant="lime" class="text-[9px] px-2 py-0 h-4.5 rounded-full font-bold">
                        {m.settings_cycles_status_active()}
                      </Badge>
                    {:else}
                      <Badge variant="secondary" class="text-[9px] px-2 py-0 h-4.5 rounded-full font-bold">
                        {year.is_locked === 1 ? m.settings_cycles_status_locked() : m.settings_cycles_status_inactive()}
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
              <div class="pl-8 pr-4 py-2 flex flex-col gap-2 transition-all duration-200">
                {#if childTerms.length === 0}
                  <div class="text-[10px] text-muted-foreground/60 py-3 text-start select-none">
                    No academic terms registered for this year.
                  </div>
                {:else}
                  {#each childTerms as term (term.id)}
                    <div class="flex items-center justify-between py-2 border-b border-border/20 last:border-b-0">
                      <div class="flex flex-col text-start gap-1">
                        <div class="flex items-center gap-2">
                          <span class="text-xs font-bold text-foreground">
                            {getLocale() === "ar" && term.name_ar ? term.name_ar : term.name}
                          </span>
                          
                          {#if term.is_current === 1}
                            <Badge variant="lime" class="text-[8px] px-1.5 py-0 h-4 rounded font-bold">
                              Active
                            </Badge>
                          {:else if term.is_locked === 1}
                            <Badge variant="destructive" class="text-[8px] px-1.5 py-0 h-4 rounded font-bold">
                              Locked
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
</div>

<!-- Sheets -->
<CycleCreateSheet
  bind:open={isSheetOpen}
  orgId={data.orgId}
  cycle={editingCycle}
  onSuccess={loadCycles}
/>
