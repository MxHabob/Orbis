<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Skeleton } from "@orbis/ui-svelte";
import { onMount } from "svelte";
import { commands } from "$lib/core/ipc";
import DragIndicator from "../components/drag-indicator.svelte";

import { getLocale } from "$lib/paraglide/runtime";

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

let data = $state<any[]>([]);
let overallRate = $state(0);
let isLoading = $state(true);

async function loadData() {
  isLoading = true;
  try {
    const periodId = selectedTerm !== "all" ? selectedTerm : "";
    const classId = selectedClass !== "all" ? selectedClass : undefined;
    const effectivePeriodId =
      periodId || academicContext.effectivePeriodId || "";

    if (!effectivePeriodId) {
      data = [];
      overallRate = 0;
      return;
    }

    const records = await commands.assessment.listAttendance(effectivePeriodId,
      classId,
    );

    if (records && records.length > 0) {
      const groups: Record<string, { present: number; total: number }> = {};
      let totalPresent = 0;

      for (const r of records) {
        const dateStr = r.date ? String(r.date).slice(0, 10) : "";
        if (!dateStr) continue;

        if (!groups[dateStr]) {
          groups[dateStr] = { present: 0, total: 0 };
        }

        const isPresent =
          r.status === "present" ||
          r.status === "late" ||
          String(r.status).toLowerCase().includes("present") ||
          String(r.status).includes(m.metrics_attendance_present());
        if (isPresent) {
          groups[dateStr].present += 1;
          totalPresent += 1;
        }
        groups[dateStr].total += 1;
      }

      overallRate =
        records.length > 0 ? (totalPresent / records.length) * 100 : 0;

      const sortedDates = Object.keys(groups).sort();
      data = sortedDates.map((date) => {
        const g = groups[date]!;
        const rate = g.total > 0 ? (g.present / g.total) * 100 : 100;
        const dateObj = new Date(date);
        const label = isNaN(dateObj.getTime())
          ? date.substring(5)
          : new Intl.DateTimeFormat(getLocale(), { month: "short" }).format(
              dateObj,
            );
        return {
          date: label,
          rawDate: date,
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

// SVG Chart dimensions
const width = 500;
const height = 150;
const padding = { left: 35, right: 15, top: 15, bottom: 20 };
const chartWidth = width - padding.left - padding.right;
const chartHeight = height - padding.top - padding.bottom;
const targetY = padding.top + chartHeight - 0.95 * chartHeight;

// Generate path command for SVG line chart
const linePath = $derived.by(() => {
  if (data.length === 0) return "";
  const points = data.map((d, idx) => {
    const x = padding.left + (idx / (data.length - 1 || 1)) * chartWidth;
    const y = padding.top + chartHeight - (d.rate / 100) * chartHeight;
    return `${x},${y}`;
  });
  return `M ${points.join(" L ")}`;
});

const dataPoints = $derived.by(() => {
  if (data.length === 0) return [];
  return data.map((d, idx) => {
    const x = padding.left + (idx / (data.length - 1 || 1)) * chartWidth;
    const y = padding.top + chartHeight - (d.rate / 100) * chartHeight;
    return { x, y, item: d };
  });
});

let hoveredPointIdx = $state<number | null>(null);
let tooltipPos = $state<{ x: number; y: number } | null>(null);

function handleMouseMove(e: MouseEvent & { currentTarget: SVGSVGElement }) {
  if (data.length === 0) return;
  const rect = e.currentTarget.getBoundingClientRect();
  const mouseX = e.clientX - rect.left;
  const mouseY = e.clientY - rect.top;
  const svgX = (mouseX / rect.width) * width;

  // Find closest point
  let closestIdx = 0;
  let minDistance = Infinity;

  dataPoints.forEach((pt, idx) => {
    const dist = Math.abs(pt.x - svgX);
    if (dist < minDistance) {
      minDistance = dist;
      closestIdx = idx;
    }
  });

  hoveredPointIdx = closestIdx;
  tooltipPos = { x: mouseX, y: mouseY };
}

function handleMouseLeave() {
  hoveredPointIdx = null;
  tooltipPos = null;
}
</script>

<div class="card-tactile h-full relative group min-h-[265px] p-5 flex flex-col justify-between font-sans border border-border/60 bg-card shadow-sm">
  {#if isLoading}
    <div class="grow w-full relative flex items-center justify-center">
      <Skeleton class="h-full w-full rounded-lg animate-pulse" />
    </div>
  {:else}
    <!-- Top Row layout matching exactly screenshots -->
    <div class="flex flex-col w-full text-start select-none">
      <div class="flex items-center justify-between w-full h-5">
        <span class="text-xs text-muted-foreground">{m.common_card_attendance_rate()}</span>
        {#if isEditing}
          <DragIndicator />
        {/if}
      </div>
      <div class="text-4xl font-light tracking-tight mt-1 text-foreground">
        {Math.round(overallRate)}%
      </div>
      
      <!-- Subtitle legend matching exactly: Daily Average (circle dot) and Target Rate (dashed line) -->
      <div class="flex items-center gap-4 text-[10px] text-muted-foreground mt-2 font-medium">
        <div class="flex items-center gap-1.5">
          <span class="h-2 w-2 rounded-full bg-foreground inline-block"></span>
          <span>{m.reports_attendance_rate_daily_avg()}</span>
        </div>
        <div class="flex items-center gap-1.5">
          <span class="font-mono text-muted-foreground/60 select-none">--</span>
          <span>{m.reports_attendance_rate_target_rate()}</span>
        </div>
      </div>
    </div>

    <!-- Main Chart Area -->
    <div class="grow relative w-full h-full min-h-[165px] select-none mt-4 flex items-center justify-center">
      {#if data.length === 0}
        <!-- Dotted rectangle outline placeholder matching screenshot 4 left -->
        <div class="w-full h-full border border-dashed border-border/60 rounded-xl flex items-center justify-center bg-muted/5 transition-colors">
          <div class="flex flex-col items-center gap-1 px-4 text-center">
            <span class="text-xs font-semibold text-foreground/80">{m.reports_attendance_empty_title()}</span>
            <span class="text-[9px] text-muted-foreground max-w-[260px] leading-relaxed">
              {m.reports_attendance_empty_desc()}
            </span>
          </div>
        </div>
      {:else}
        <!-- Interactive line chart -->
        <svg
          role="img"
          viewBox={`0 0 ${width} ${height}`}
          preserveAspectRatio="none"
          class="w-full h-full text-foreground select-none overflow-visible cursor-crosshair"
          onmousemove={handleMouseMove}
          onmouseleave={handleMouseLeave}
        >
          <!-- Grid Lines -->
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
            {@const labelVal = Math.round((1 - tick) * 100)}
            <text
              x={padding.left - 8}
              y={y + 3}
              class="text-[8px] font-bold fill-muted-foreground/80 font-mono"
              text-anchor="end"
            >
              {labelVal}%
            </text>
          {/each}

          <!-- Horizontal Target Rate line (95%) -->
          <line
            x1={padding.left}
            y1={targetY}
            x2={width - padding.right}
            y2={targetY}
            class="stroke-muted-foreground/60"
            stroke-width="1"
            stroke-dasharray="4,4"
          />

          <!-- Trend Line -->
          <path
            d={linePath}
            fill="none"
            class="stroke-foreground"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
          />

          <!-- Data Points -->
          {#each dataPoints as pt, idx}
            <circle
              cx={pt.x}
              cy={pt.y}
              r={hoveredPointIdx === idx ? 5 : 3}
              class="fill-background stroke-foreground transition-all duration-100"
              stroke-width="1.5"
            />
          {/each}

          <!-- X-Axis Labels -->
          {#each dataPoints as pt}
            <text
              x={pt.x}
              y={height - 2}
              class="text-[8px] font-bold fill-muted-foreground/80 font-mono"
              text-anchor="middle"
            >
              {pt.item.date}
            </text>
          {/each}
        </svg>

        <!-- Floating Tooltip -->
        {#if hoveredPointIdx !== null && tooltipPos}
          {@const activePt = dataPoints[hoveredPointIdx]}
          <div
            class="absolute z-30 p-2 text-[10px] font-sans border bg-card border-border text-foreground rounded-lg shadow-xl pointer-events-none transition-all duration-700 flex flex-col gap-0.5 min-w-[100px] text-start"
            style="left: {tooltipPos.x + 15}px; top: {tooltipPos.y - 25}px;"
          >
            <span class="text-muted-foreground font-bold border-b border-border/40 pb-0.5 mb-1">{activePt.item.rawDate}</span>
            <span class="font-bold text-foreground font-mono">{m.reports_attendance_rate_tooltip({ rate: String(activePt.item.rate) })}</span>
          </div>
        {/if}
      {/if}
    </div>
  {/if}
</div>
