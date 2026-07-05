<script lang="ts">
import { Skeleton } from "@orbis/ui-svelte";
import { onMount } from "svelte";
import { commands } from "$lib/core/ipc";
import DragIndicator from "../components/drag-indicator.svelte";
import * as m from "$lib/paraglide/messages";

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

let stats = $state<any>(null);
let isLoading = $state(true);

async function loadData() {
  isLoading = true;
  try {
    const params: any = {};
    if (selectedClass !== "all") params.class_id = selectedClass;
    if (selectedTerm !== "all") params.cycle_id = selectedTerm;

    const res = await commands.grades.getAnalytics(orgId, params);
    if (res && res.grade_distribution && Object.values(res.grade_distribution).some(v => Number(v) > 0)) {
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
  stats && distributionArray.length > 0 ? distributionArray : [
    { name: "A+", value: 0 },
    { name: "A", value: 0 },
    { name: "B+", value: 0 },
    { name: "B", value: 0 },
    { name: "C+", value: 0 },
    { name: "C", value: 0 },
    { name: "D+", value: 0 },
    { name: "D", value: 0 },
    { name: "F", value: 0 }
  ]
);

// SVG calculation - Shorter height
const width = 500;
const height = 150;
const padding = { left: 30, right: 10, top: 15, bottom: 20 };

const maxValue = $derived(
  Math.max(1, ...displayArray.map((d) => d.value)),
);
const chartWidth = $derived(width - padding.left - padding.right);
const chartHeight = $derived(height - padding.top - padding.bottom);
const barWidth = $derived(
  Math.max(5, chartWidth / displayArray.length - 12),
);

const bars = $derived.by(() => {
  return displayArray.map((item, idx) => {
    const xStep = chartWidth / displayArray.length;
    const x = padding.left + idx * xStep + (xStep - barWidth) / 2;
    const pct = item.value / maxValue;
    const h = pct * chartHeight;
    const y = height - padding.bottom - h;
    return { x, y, width: barWidth, height: h, item };
  });
});

// Interactive Hover State
let hoveredIndex = $state<number | null>(null);
let tooltipPos = $state<{ x: number; y: number } | null>(null);

function handleMouseMove(e: MouseEvent & { currentTarget: SVGSVGElement }) {
  if (!stats || bars.length === 0) return;
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
        <span class="h-1.5 w-1.5 rounded-full bg-primary {stats ? 'animate-pulse' : ''}"></span>
        {m.reports_grade_distribution_title()} {#if stats}• Average: <span class="text-foreground font-mono">{stats?.class_average ?? 0}%</span> (Pass: {stats?.pass_rate ?? 0}%){/if}
      </div>
    </div>

    <!-- Main Content Area: Full Width SVG Chart (No side panel) -->
    <div class="flex-grow relative w-full h-full min-h-[160px] select-none mt-2 flex items-center justify-center" onmouseleave={handleMouseLeave}>
      <svg
        viewBox={`0 0 ${width} ${height}`}
        class="w-full h-full select-none overflow-visible {stats ? 'cursor-crosshair' : ''}"
      >
        <!-- Grid Lines and Y-Axis Counts -->
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
          {@const labelVal = Math.round((1 - tick) * maxValue)}
          <text
            x={padding.left - 8}
            y={y + 3}
            class="text-[8px] font-bold fill-muted-foreground/80 font-mono"
            text-anchor="end"
          >
            {labelVal}
          </text>
        {/each}

        <!-- Active Column Slice Highlight background -->
        {#if hoveredIndex !== null && stats}
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

        <!-- Bars with rounded top corners rx=2 (Rendered ONLY when stats is not null) -->
        {#if stats}
          {#each bars as bar, idx}
            <rect
              x={bar.x}
              y={bar.y}
              width={bar.width}
              height={Math.max(2, bar.height)}
              rx={2}
              class="transition-colors duration-200 {hoveredIndex === idx ? 'fill-primary' : 'fill-primary/80'}"
            />

            <!-- Value Label overlay above bar -->
            {#if bar.item.value > 0}
              <text
                x={bar.x + bar.width / 2}
                y={bar.y - 6}
                class="text-[8px] font-extrabold fill-foreground font-mono"
                text-anchor="middle"
              >
                {bar.item.value}
              </text>
            {/if}
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
            {bar.item.name}
          </text>
        {/each}
      </svg>

      <!-- Interactive hover overlay tracker -->
      {#if stats}
        <svg
          class="absolute inset-0 w-full h-full opacity-0 cursor-crosshair"
          viewBox={`0 0 ${width} ${height}`}
          onmousemove={handleMouseMove}
        ></svg>
      {/if}

      <!-- Floating Interactive Tooltip -->
      {#if hoveredIndex !== null && tooltipPos && stats}
        {@const activeBar = bars[hoveredIndex]}
        <div
          class="absolute z-30 p-2 text-[10px] font-sans border bg-card border-border text-foreground rounded-lg shadow-xl pointer-events-none transition-all duration-100 flex flex-col gap-0.5 min-w-[100px] text-start"
          style="left: {tooltipPos.x + 12}px; top: {tooltipPos.y - 50}px;"
        >
          <span class="text-muted-foreground font-bold border-b border-border/40 pb-0.5 mb-1">{m.reports_grade_label({ grade: activeBar.item.name })}</span>
          <span class="font-bold text-foreground font-mono">{m.reports_students_count({ count: String(activeBar.item.value) })}</span>
        </div>
      {/if}

      <!-- Backdrop Blur Overlay (No borders, no alert icons, frosted glass blur effect) -->
      {#if !stats}
        <div class="absolute inset-0 flex flex-col items-center justify-center text-center select-none p-4 gap-1.5 backdrop-blur-[3.5px] bg-background/35 z-10 transition-all duration-300 rounded-xl">
          <span class="font-bold text-xs text-foreground/90">لا توجد درجات أو تقديرات متوفرة</span>
          <span class="text-[10px] text-muted-foreground max-w-[280px] leading-relaxed">
            لا توجد درجات اختبارات مسجلة للطلبة في هذا الفصل. للبدء بمشاهدة التوزيع التحليلي، قم بإضافة درجات المواد من سجل الأكاديميات.
          </span>
        </div>
      {/if}
    </div>

  {/if}
</div>
