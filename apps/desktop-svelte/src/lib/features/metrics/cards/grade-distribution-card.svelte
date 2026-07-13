<script lang="ts">
import { Skeleton } from "@orbis/ui-svelte";
import { onMount } from "svelte";
import { commands } from "$lib/core/ipc";
import DragIndicator from "../components/drag-indicator.svelte";
import * as m from "$lib/paraglide/messages";

import { academicContext } from "$lib/stores";

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

let stats = $state<any>(null);
let isLoading = $state(true);

async function loadData() {
  isLoading = true;
  try {
    const classId = selectedClass !== "all" ? selectedClass : "";
    const cycleId =
      selectedTerm !== "all"
        ? selectedTerm
        : academicContext.effectiveCycleId || "";

    if (!classId || !cycleId) {
      stats = null;
      return;
    }

    const res = await commands.assessment.getGradeAnalytics(
      orgId,
      classId,
      cycleId,
    );
    if (
      res &&
      res.grade_distribution &&
      Object.values(res.grade_distribution).some((v) => Number(v) > 0)
    ) {
      stats = res;
    } else {
      stats = null;
    }
  } catch (e) {
    console.error("Error loading grade analytics:", e);
    stats = null;
  } finally {
    isLoading = false;
  }
}

$effect(() => {
  if (orgId || selectedClass || selectedTerm || period) {
    loadData();
  }
});

// Map distribution to an array of items
const distributionArray = $derived.by(() => {
  if (!stats?.grade_distribution) return [];
  const dist = stats.grade_distribution;
  return [
    { name: "A+", value: dist.a_plus ?? 0 },
    { name: "A", value: dist.a ?? 0 },
    { name: "B+", value: dist.b_plus ?? 0 },
    { name: "B", value: dist.b ?? 0 },
    { name: "C+", value: dist.c_plus ?? 0 },
    { name: "C", value: dist.c ?? 0 },
    { name: "D+", value: dist.d_plus ?? 0 },
    { name: "D", value: dist.d ?? 0 },
    { name: "F", value: dist.f ?? 0 },
  ];
});

// Derive display distribution list (uses empty placeholding bars when empty)
const displayArray = $derived(
  stats && distributionArray.length > 0
    ? distributionArray
    : [
        { name: "A+", value: 0 },
        { name: "A", value: 0 },
        { name: "B+", value: 0 },
        { name: "B", value: 0 },
        { name: "C+", value: 0 },
        { name: "C", value: 0 },
        { name: "D+", value: 0 },
        { name: "D", value: 0 },
        { name: "F", value: 0 },
      ],
);

const maxValue = $derived(Math.max(1, ...displayArray.map((d) => d.value)));

const bars = $derived.by(() => {
  return displayArray.map((item, idx) => {
    return {
      name: item.name,
      value: item.value,
      pct: Math.min(100, (item.value / maxValue) * 100),
    };
  });
});

let hoveredIndex = $state<number | null>(null);
let tooltipPos = $state<{ x: number; y: number } | null>(null);

function handleMouseMove(e: MouseEvent & { currentTarget: HTMLDivElement }) {
  if (!stats || bars.length === 0) return;
  const rect = e.currentTarget.getBoundingClientRect();
  const mouseX = e.clientX - rect.left;
  const mouseY = e.clientY - rect.top;

  const isRtl = document.documentElement.dir === "rtl";
  const colWidth = rect.width / displayArray.length;
  const rawIdx = Math.floor(mouseX / colWidth);
  const idx = Math.min(
    displayArray.length - 1,
    Math.max(0, isRtl ? displayArray.length - 1 - rawIdx : rawIdx),
  );

  hoveredIndex = idx;
  tooltipPos = { x: mouseX, y: mouseY };
}

function handleMouseLeave() {
  hoveredIndex = null;
  tooltipPos = null;
}
</script>

<div class="card-tactile h-full relative group min-h-[235px] p-4 flex flex-col justify-between font-sans shadow-sm border border-border/60 bg-card">
  {#if isLoading}
    <div class="flex-grow w-full relative flex items-center justify-center">
      <Skeleton class="h-full w-full rounded-lg animate-pulse" />
    </div>
  {:else}
    <!-- Top info bar with DragIndicator if editing, and small Releases-style label on the right -->
    <div class="flex items-center justify-between w-full h-5 select-none text-[9px] font-bold text-muted-foreground/80">
      <div class="flex items-center gap-2">
        {#if isEditing}
          <DragIndicator />
        {/if}
      </div>
      <div class="flex items-center gap-1.5 font-sans font-extrabold uppercase tracking-wider text-[8px] text-muted-foreground/75">
        <span class="h-1.5 w-1.5 rounded-full bg-primary {stats ? 'animate-pulse' : ''}"></span>
        {m.reports_grade_distribution_title()} {#if stats}• Average: <span class="text-foreground font-mono">{stats?.class_average ?? 0}%</span> (Pass: {stats?.pass_rate ?? 0}%){/if}
      </div>
    </div>

    <!-- Main Content Area: Full Width HTML Chart -->
    <div 
      class="flex-grow relative w-full h-[160px] select-none mt-2 flex flex-col justify-between" 
      onmouseleave={handleMouseLeave}
    >
      <!-- Grid Lines and Y-Axis Counts -->
      <div class="absolute inset-0 flex flex-col justify-between pb-8 pt-4 pointer-events-none">
        {#each [0, 0.25, 0.5, 0.75, 1] as tick}
          {@const tickVal = Math.round((1 - tick) * maxValue)}
          <div class="flex items-center w-full h-0">
            <span class="w-8 text-end text-[8px] font-bold text-muted-foreground/80 font-mono pr-2 select-none">
              {tickVal}
            </span>
            <div class="flex-grow border-t border-dashed border-border/40"></div>
          </div>
        {/each}
      </div>

      <!-- Columns Container -->
      <div 
        class="absolute inset-0 flex items-end justify-between pl-8 pr-2 pb-8 pt-4 {stats ? 'cursor-crosshair' : ''}"
        onmousemove={handleMouseMove}
      >
        {#each bars as bar, idx}
          {@const isActive = hoveredIndex === idx}
          <div class="flex-grow flex flex-col items-center justify-end h-full relative group/col">
            <!-- Active slice highlight background -->
            {#if isActive && stats}
              <div class="absolute inset-x-1 top-0 bottom-0 bg-foreground/5 rounded-md pointer-events-none"></div>
            {/if}

            <!-- Value Label overlay above bar -->
            {#if bar.value > 0 && stats}
              <span class="text-[8px] font-extrabold text-foreground mb-1 select-none font-mono z-10">
                {bar.value}
              </span>
            {/if}

            <!-- Bar (tactile primary button style) -->
            {#if stats}
              <div 
                class="w-3 sm:w-4 bg-primary shadow-[0_3px_0_0_oklch(0.35_0.1_280)] dark:shadow-[0_3px_0_0_oklch(0.3_0.1_280)] hover:-translate-y-1 transition-all rounded-t-md cursor-pointer duration-150 z-10"
                style="height: {bar.pct}%"
                title="{bar.name}: {bar.value}"
              ></div>
            {/if}

            <!-- X-Axis Label Centered Under Bars -->
            <div class="absolute bottom-[-24px] text-[8px] font-bold text-muted-foreground/90 font-mono select-none">
              {bar.name}
            </div>
          </div>
        {/each}
      </div>

      <!-- Floating Interactive Tooltip -->
      {#if hoveredIndex !== null && tooltipPos && stats}
        {@const activeBar = bars[hoveredIndex]}
        <div
          class="absolute z-30 p-2 text-[10px] font-sans border bg-card border-border text-foreground rounded-lg shadow-xl pointer-events-none transition-all duration-100 flex flex-col gap-0.5 min-w-[100px] text-start"
          style="left: {tooltipPos.x + 12}px; top: {tooltipPos.y - 50}px;"
        >
          <span class="text-muted-foreground font-bold border-b border-border/40 pb-0.5 mb-1">{m.reports_grade_label({ grade: activeBar.name })}</span>
          <span class="font-bold text-foreground font-mono">{m.reports_students_count({ count: String(activeBar.value) })}</span>
        </div>
      {/if}

      <!-- Backdrop Blur Overlay -->
      {#if !stats}
        <div class="absolute inset-0 flex flex-col items-center justify-center text-center select-none p-4 gap-1.5 backdrop-blur-[3.5px] bg-background/35 z-10 transition-all duration-300 rounded-xl">
          <span class="font-bold text-xs text-foreground/90">{m.reports_grades_empty_title()}</span>
          <span class="text-[10px] text-muted-foreground max-w-[280px] leading-relaxed">
            {m.reports_grades_empty_desc()}
          </span>
        </div>
      {/if}
    </div>
  {/if}
</div>
