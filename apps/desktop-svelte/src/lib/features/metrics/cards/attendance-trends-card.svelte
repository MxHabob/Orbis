<script lang="ts">
import { Skeleton } from "@orbis/ui-svelte";
import { onMount } from "svelte";
import { commands } from "$lib/core/ipc";
import DragIndicator from "../components/drag-indicator.svelte";
import * as m from "$lib/paraglide/messages";
import { getLocale } from "$lib/paraglide/runtime";

let {
  orgId,
  selectedClass = "all",
  selectedTerm = "all",
  period = "this-year",
  isEditing = false,
}: {
  orgId: string;
  selectedClass: string;
  selectedTerm: string;
  period: string;
  isEditing?: boolean;
} = $props();

let data = $state<any[]>([]);
let overallRate = $state(0);
let isLoading = $state(true);

async function loadData() {
  isLoading = true;
  try {
    // Query analytics for attendance
    const params: any = {};
    if (selectedClass !== "all") params.class_id = selectedClass;
    if (selectedTerm !== "all") params.cycle_id = selectedTerm;

    const analytics = (await commands.attendance.getAnalytics(
      orgId,
      params,
    )) as any;
    overallRate = analytics ? analytics.overall_rate : 0;

    // Query raw attendance to group for the chart trend
    const records = (await commands.attendance.getAll(orgId, params)) as any[];
    if (records && records.length > 0) {
      // Group by date
      const groups: Record<string, { present: number; total: number }> = {};
      for (const r of records) {
        if (!groups[r.date]) {
          groups[r.date] = { present: 0, total: 0 };
        }
        if (r.status === "present" || r.status === "late") {
          groups[r.date].present += 1;
        }
        groups[r.date].total += 1;
      }

      // Sort dates chronologically
      const sortedDates = Object.keys(groups).sort();
      data = sortedDates.map((date) => {
        const g = groups[date]!;
        const rate = g.total > 0 ? (g.present / g.total) * 100 : 100;
        return {
          date: date.substring(5), // mm-dd
          rate: Math.round(rate * 10) / 10,
        };
      });
    } else {
      data = [];
      overallRate = 0;
    }
  } catch (e) {
    console.error("Error loading attendance trends:", e);
    data = [];
    overallRate = 0;
  } finally {
    isLoading = false;
  }
}

$effect(() => {
  if (orgId || selectedClass || selectedTerm || period) {
    loadData();
  }
});

// SVG Dimensions - Shorter height to utilize screen space cleanly
const width = 500;
const height = 150;
const padding = { left: 40, right: 15, top: 15, bottom: 20 };

// Determine display dataset (uses placeholders for grid when empty)
const displayData = $derived(
  data.length > 0 ? data : [
    { date: "09-01", rate: 0 },
    { date: "10-01", rate: 0 },
    { date: "11-01", rate: 0 },
    { date: "12-01", rate: 0 },
    { date: "01-01", rate: 0 },
    { date: "02-01", rate: 0 },
    { date: "03-01", rate: 0 },
  ]
);

// Derive min and max rates with pad for scaling
const minRate = $derived.by(() => {
  if (data.length === 0) return 0;
  const rates = data.map((d) => d.rate);
  return Math.max(0, Math.min(...rates) - 2);
});

const maxRate = $derived.by(() => {
  if (data.length === 0) return 100;
  const rates = data.map((d) => d.rate);
  return Math.min(100, Math.max(...rates) + 2);
});

const chartWidth = $derived(width - padding.left - padding.right);
const chartHeight = $derived(height - padding.top - padding.bottom);
const barWidth = $derived(Math.max(6, chartWidth / displayData.length - 16));

// Compute vertical column bars
const bars = $derived.by(() => {
  return displayData.map((item, idx) => {
    const xStep = chartWidth / displayData.length;
    const x = padding.left + idx * xStep + (xStep - barWidth) / 2;
    const range = maxRate - minRate || 1;
    const pct = (item.rate - minRate) / range;
    const h = pct * chartHeight;
    const y = height - padding.bottom - h;
    return { x, y, width: barWidth, height: h, item };
  });
});

// Interactive Hover State
let hoveredIndex = $state<number | null>(null);
let tooltipPos = $state<{ x: number; y: number } | null>(null);

function handleMouseMove(e: MouseEvent & { currentTarget: SVGSVGElement }) {
  if (data.length === 0 || bars.length === 0) return;
  const rect = e.currentTarget.getBoundingClientRect();
  const containerRect = e.currentTarget.parentElement!.getBoundingClientRect();

  const localX = e.clientX - containerRect.left;
  const localY = e.clientY - containerRect.top;
  tooltipPos = { x: localX, y: localY };

  const mouseSVGX = ((e.clientX - rect.left) / rect.width) * width;

  let closestIdx: number | null = null;
  bars.forEach((bar, idx) => {
    const barLeft = bar.x - 6;
    const barRight = bar.x + bar.width + 6;
    if (mouseSVGX >= barLeft && mouseSVGX <= barRight) {
      closestIdx = idx;
    }
  });

  if (closestIdx === null) {
    let minDiff = Infinity;
    bars.forEach((bar, idx) => {
      const center = bar.x + bar.width / 2;
      const diff = Math.abs(center - mouseSVGX);
      if (diff < minDiff) {
        minDiff = diff;
        closestIdx = idx;
      }
    });
  }
  hoveredIndex = closestIdx;
}

function handleMouseLeave() {
  hoveredIndex = null;
  tooltipPos = null;
}
</script>

<div class="card-tactile h-full relative group min-h-[235px] p-4 flex flex-col justify-between font-sans shadow-sm">
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
        <span class="h-1.5 w-1.5 rounded-full bg-primary {data.length > 0 ? 'animate-pulse' : ''}"></span>
        {m.reports_attendance_trends_title()} {#if data.length > 0}• Average: <span class="text-foreground font-mono">{overallRate}%</span>{/if}
      </div>
    </div>

    <!-- Main Content Area: Full Width SVG Chart Card (No side panel) -->
    <div class="flex-grow relative w-full h-full min-h-[160px] select-none mt-2 flex items-center justify-center" onmouseleave={handleMouseLeave}>
      <svg
        viewBox={`0 0 ${width} ${height}`}
        class="w-full h-full text-primary select-none overflow-visible {data.length > 0 ? 'cursor-crosshair' : ''}"
      >
        <!-- Grid Lines & Y-Axis percentages -->
        {#each [0, 0.25, 0.5, 0.75, 1] as tick}
          {@const y = padding.top + tick * chartHeight}
          <line
            x1={padding.left}
            y1={y}
            x2={width - padding.right}
            y2={y}
            class="stroke-border/40"
            stroke-width="1"
            stroke-dasharray="3,3"
          />
          {@const labelVal = Math.round((1 - tick) * (maxRate - minRate) + minRate)}
          <text
            x={padding.left - 8}
            y={y + 3}
            class="text-[8px] font-bold fill-muted-foreground/80 font-mono"
            text-anchor="end"
          >
            {labelVal}%
          </text>
        {/each}

        <!-- Active Column Slice Highlight background -->
        {#if hoveredIndex !== null && data.length > 0}
          {@const bar = bars[hoveredIndex]}
          <rect
            x={bar.x - 6}
            y={padding.top}
            width={bar.width + 12}
            height={chartHeight}
            class="fill-primary/5"
            rx={2}
          />
        {/if}

        <!-- Bars (Rendered ONLY when there is actual data in the database) -->
        {#if data.length > 0}
          {#each bars as bar, idx}
            <rect
              x={bar.x}
              y={bar.y}
              width={bar.width}
              height={Math.max(2, bar.height)}
              rx={2}
              class="transition-colors duration-200 {hoveredIndex === idx ? 'fill-primary' : 'fill-primary/80'}"
            />
          {/each}
        {/if}

        <!-- X-Axis Labels -->
        {#each bars as bar}
          <text
            x={bar.x + bar.width / 2}
            y={height - 3}
            class="text-[8px] font-bold fill-muted-foreground/80 font-mono"
            text-anchor="middle"
          >
            {bar.item.date}
          </text>
        {/each}
      </svg>

      <!-- Interactive hover overlay tracker -->
      {#if data.length > 0}
        <svg
          class="absolute inset-0 w-full h-full opacity-0 cursor-crosshair"
          viewBox={`0 0 ${width} ${height}`}
          onmousemove={handleMouseMove}
        ></svg>
      {/if}

      <!-- Floating Interactive Tooltip -->
      {#if hoveredIndex !== null && tooltipPos && data.length > 0}
        {@const activeBar = bars[hoveredIndex]}
        <div
          class="absolute z-30 p-2 text-[10px] font-sans border bg-card border-border text-foreground rounded-lg shadow-xl pointer-events-none transition-all duration-100 flex flex-col gap-0.5 min-w-[100px] text-start"
          style="left: {tooltipPos.x + 12}px; top: {tooltipPos.y - 45}px;"
        >
          <span class="text-muted-foreground font-bold border-b border-border/40 pb-0.5 mb-1">{activeBar.item.date}</span>
          <span class="font-bold text-foreground font-mono">{m.reports_attendance_rate_tooltip({ rate: String(activeBar.item.rate) })}</span>
        </div>
      {/if}

      <!-- Backdrop Blur Overlay (No borders, no alert icons, frosted glass blur effect) -->
      {#if data.length === 0}
        <div class="absolute inset-0 flex flex-col items-center justify-center text-center select-none p-4 gap-1.5 backdrop-blur-[3px] bg-background/35 z-10 transition-all duration-300 rounded-xl">
          <span class="font-bold text-xs text-foreground/90">{m.reports_attendance_empty_title()}</span>
          <span class="text-[9px] text-muted-foreground max-w-[280px] leading-relaxed">
            {m.reports_attendance_empty_desc()}
          </span>
        </div>
      {/if}
    </div>

  {/if}
</div>
