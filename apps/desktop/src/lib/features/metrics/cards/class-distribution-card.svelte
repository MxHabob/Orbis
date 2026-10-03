<script lang="ts">
import { ChartBar, Users } from "@orbis/icons";
import * as m from "$lib/paraglide/messages";
import { Skeleton } from "@orbis/ui";
import { onMount } from "svelte";
import { api } from "$lib/api";
import { getLocale } from "$lib/paraglide/runtime";
import DragIndicator from "../components/drag-indicator.svelte";

let {
  orgId,
  selectedClass = "all",
  selectedTerm = "all",
  period = "this-year",
  isEditing = false,
}: {
  orgId: string;
  selectedClass?: string;
  selectedTerm?: string;
  period?: string;
  isEditing?: boolean;
} = $props();

let data = $state<{ name: string; count: number }[]>([]);
let activeClassesCount = $state(0);
let isLoading = $state(true);

async function loadData() {
  isLoading = true;
  try {
    const params: any = {};
    if (selectedClass !== "all") params.class_id = selectedClass;
    if (selectedTerm !== "all") params.cycle_id = selectedTerm;

    const classes = await api.listClasses(params);
    if (classes && classes.length > 0) {
      data = classes
        .map((c: any) => ({
          name:
            getLocale() === "ar" ? c.name_ar || c.name : c.name || c.name_ar,
          count: c.current_students || 0,
        }))
        .sort((a: any, b: any) => b.count - a.count)
        .slice(0, 9); // top 9 classes

      activeClassesCount = classes.length;
    } else {
      data = [];
      activeClassesCount = 0;
    }
  } catch (e) {
    console.error("Error loading class distribution:", e);
    data = [];
    activeClassesCount = 0;
  } finally {
    isLoading = false;
  }
}

$effect(() => {
  if (orgId || selectedClass || selectedTerm || period) {
    loadData();
  }
});

const maxVal = $derived(Math.max(...data.map((d) => d.count), 1));
</script>

<div class="card-tactile h-full relative group min-h-[265px] p-5 flex flex-col justify-between font-sans border border-border/60 bg-card shadow-sm">
  {#if isLoading}
    <div class="grow w-full relative flex items-center justify-center">
      <Skeleton class="h-full w-full rounded-lg animate-pulse" />
    </div>
  {:else}
    <!-- Header -->
    <div class="flex flex-col w-full text-start select-none">
      <div class="flex items-center justify-between w-full h-5">
        <span class="text-xs text-muted-foreground">{m.reports_class_dist_title()}</span>
        {#if isEditing}
          <DragIndicator />
        {/if}
      </div>
      <div class="text-4xl font-light tracking-tight mt-1 text-foreground">
        {m.reports_class_dist_active_classes({ count: String(activeClassesCount) })}
      </div>
      
      <!-- Subtitle legend matching exactly: Student Population -->
      <div class="flex items-center gap-4 text-[10px] text-muted-foreground mt-2 font-medium">
        <div class="flex items-center gap-1.5">
          <span class="h-2 w-2 rounded-full bg-foreground inline-block"></span>
          <span>{m.common_class_overview_students_count()}</span>
        </div>
      </div>
    </div>

    <!-- Chart List Area: Horizontal Progress Bars -->
    <div class="grow flex flex-col gap-2.5 mt-4 justify-center w-full min-h-[140px] relative">
      {#each data as item}
        {@const pct = (item.count / maxVal) * 100}
        <div class="flex items-center gap-3 w-full text-[10px] hover:bg-foreground/5 p-1 rounded-md transition-colors duration-150">
          <!-- Class Name -->
          <span class="w-[85px] shrink-0 text-start text-muted-foreground truncate select-none font-medium">
            {item.name}
          </span>

          <!-- Sunken Progress Bar Track -->
          <div class="grow h-2.5 bg-muted/40 border border-border/40 rounded-md overflow-hidden relative shadow-[inset_0_1px_2.5px_rgba(0,0,0,0.08)] dark:shadow-[inset_0_1px_3px_rgba(0,0,0,0.35)]">
            <div
              class="h-full rounded-md bg-linear-to-r from-muted-foreground/20 to-foreground rtl:bg-gradient-to-l rtl:from-muted-foreground/20 rtl:to-foreground transition-all duration-500 shadow-[0_1.5px_0_0_rgba(0,0,0,0.15)] border-b border-black/10"
              style="width: {pct}%"
            ></div>
          </div>
          
          <!-- Optional Count display on end -->
          <span class="w-[30px] text-end font-mono text-muted-foreground/80 select-none font-bold">
            {item.count}
          </span>
        </div>
      {/each}
    </div>
  {/if}
</div>
