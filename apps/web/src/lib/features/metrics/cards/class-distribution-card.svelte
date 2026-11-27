<script lang="ts">
import { ChartBar, Users } from "@orbis/icons";
import * as m from "$lib/paraglide/messages";
import { Skeleton } from "@orbis/ui";
import { api } from "$lib/api";
import { academicContext } from "$lib/stores";
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
    const effectivePeriodId =
      selectedTerm !== "all"
        ? selectedTerm
        : academicContext.effectivePeriodId || undefined;
    const [sections, enrollments] = await Promise.all([
      api.listSections(),
      api
        .listEnrollments({
          period_id: effectivePeriodId,
          class_id: selectedClass !== "all" ? selectedClass : undefined,
          page_size: 100,
        })
        .catch(() => []),
    ]);

    const countsBySection = new Map<string, Set<string>>();
    for (const enrollment of enrollments) {
      if (!enrollment.section_id) continue;
      if (
        enrollment.status !== "active" &&
        enrollment.status !== "pending_approval"
      )
        continue;
      if (!countsBySection.has(enrollment.section_id)) {
        countsBySection.set(enrollment.section_id, new Set());
      }
      countsBySection.get(enrollment.section_id)!.add(enrollment.student_id);
    }

    const filteredSections = sections.filter((section) => {
      if (selectedClass !== "all" && section.id !== selectedClass) return false;
      return true;
    });
    if (filteredSections.length > 0) {
      data = filteredSections
        .map((c) => ({
          name:
            (getLocale() === "ar"
              ? c.name_ar || c.name
              : c.name || c.name_ar) || String(c.id),
          count: countsBySection.get(c.id)?.size ?? c.current_students ?? 0,
        }))
        .sort((a, b) => b.count - a.count)
        .slice(0, 9);

      activeClassesCount = filteredSections.length;
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
          <span class="h-2 w-2 rounded-full bg-chart-1 inline-block"></span>
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

          <div class="grow h-2.5 bg-muted/50 border border-border/40 rounded-md overflow-hidden relative">
            <div
              class="h-full rounded-md bg-chart-1 transition-all duration-500"
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
