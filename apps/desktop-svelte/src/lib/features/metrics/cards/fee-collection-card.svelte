<script lang="ts">
import { Skeleton } from "@orbis/ui-svelte";
import { onMount } from "svelte";
import { commands } from "$lib/core/ipc";
import DragIndicator from "../components/drag-indicator.svelte";
import * as m from "$lib/paraglide/messages";
import { getLocale } from "$lib/paraglide/runtime";
import { formatCurrency } from "$lib/core/i18n";

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
let totalCollected = $state(0);
let totalPending = $state(0);
let isLoading = $state(true);

async function loadData() {
  isLoading = true;
  try {
    // Query payments
    const payments = await commands.finance.getPayments(orgId);
    // Query student fees to see outstanding fees
    const studentFees = await commands.finance.getStudentFees(orgId);

    if (payments && payments.length > 0) {
      // Parse payments and outstanding student fees by month
      const monthsMap: Record<string, { collected: number; pending: number }> =
        {};

      // Collected payments
      for (const p of payments) {
        const dateStr = (p.payment_date || p.created_at || "") as string;
        if (!dateStr) continue;
        const monthKey = dateStr.substring(0, 7); // yyyy-mm
        if (!monthsMap[monthKey]) {
          monthsMap[monthKey] = { collected: 0, pending: 0 };
        }
        monthsMap[monthKey].collected += Number(p.amount || 0);
      }

      // Outstanding / Pending fees
      if (studentFees) {
        for (const f of studentFees) {
          if (f.status === "unpaid" || f.status === "partially_paid") {
            const dateStr = (f.due_date || f.created_at || "") as string;
            if (!dateStr) continue;
            const monthKey = dateStr.substring(0, 7);
            if (!monthsMap[monthKey]) {
              monthsMap[monthKey] = { collected: 0, pending: 0 };
            }
            const unpaid = Number(f.amount || 0) - Number(f.paid_amount || 0);
            monthsMap[monthKey].pending += Math.max(0, unpaid);
          }
        }
      }

      const sortedMonths = Object.keys(monthsMap).sort().slice(-6); // last 6 months
      totalCollected = 0;
      totalPending = 0;
      data = sortedMonths.map((mon) => {
        const vals = monthsMap[mon]!;
        totalCollected += vals.collected;
        totalPending += vals.pending;
        // Get month short name dynamically based on locale
        const year = Number(mon.split("-")[0]);
        const monthIndex = Number(mon.split("-")[1]) - 1;
        const dateObj = new Date(year, monthIndex, 1);
        const formattedMonth = new Intl.DateTimeFormat(getLocale(), {
          month: "short",
        }).format(dateObj);

        return {
          month: formattedMonth,
          collected: vals.collected,
          pending: vals.pending,
        };
      });
    } else {
      data = [];
      totalCollected = 0;
      totalPending = 0;
    }
  } catch (e) {
    console.error("Error loading finance reports:", e);
    data = [];
    totalCollected = 0;
    totalPending = 0;
  } finally {
    isLoading = false;
  }
}

$effect(() => {
  if (orgId || selectedClass || selectedTerm || period) {
    loadData();
  }
});

// SVG layout dimensions - Shorter height
const width = 500;
const height = 150;
const padding = { left: 45, right: 15, top: 15, bottom: 20 };

// Determine display dataset (uses empty months placeholders when empty)
const displayData = $derived(
  data.length > 0 ? data : [
    { month: "Oct", collected: 0, pending: 0 },
    { month: "Nov", collected: 0, pending: 0 },
    { month: "Dec", collected: 0, pending: 0 },
    { month: "Jan", collected: 0, pending: 0 },
    { month: "Feb", collected: 0, pending: 0 },
    { month: "Mar", collected: 0, pending: 0 },
  ]
);

const maxVal = $derived(
  Math.max(1000, ...displayData.map((d) => Math.max(d.collected, d.pending))),
);
const chartWidth = $derived(width - padding.left - padding.right);
const chartHeight = $derived(height - padding.top - padding.bottom);
const groupWidth = $derived(chartWidth / (displayData.length || 1));
const barWidth = $derived(Math.max(4, (groupWidth - 16) / 2));

const bars = $derived.by(() => {
  return displayData.map((item, idx) => {
    const xCol =
      padding.left + idx * groupWidth + (groupWidth - 2 * barWidth - 4) / 2;
    const xPend = xCol + barWidth + 4;

    const pctCol = item.collected / maxVal;
    const hCol = pctCol * chartHeight;
    const yCol = height - padding.bottom - hCol;

    const pctPend = item.pending / maxVal;
    const hPend = pctPend * chartHeight;
    const yPend = height - padding.bottom - hPend;

    return {
      month: item.month,
      col: {
        x: xCol,
        y: yCol,
        width: barWidth,
        height: hCol,
        val: item.collected,
      },
      pend: {
        x: xPend,
        y: yPend,
        width: barWidth,
        height: hPend,
        val: item.pending,
      },
    };
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
    const colLeft = bar.col.x - 6;
    const pendRight = bar.pend.x + bar.pend.width + 6;
    if (mouseSVGX >= colLeft && mouseSVGX <= pendRight) {
      closestIdx = idx;
    }
  });

  if (closestIdx === null) {
    let minDiff = Infinity;
    bars.forEach((bar, idx) => {
      const center = (bar.col.x + bar.pend.x + bar.col.width) / 2;
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

// Formatter helper
function formatVal(val: number) {
  if (val >= 1000000) {
    return `${Math.round(val / 100000) / 10}M`;
  }
  if (val >= 1000) {
    return `${Math.round(val / 100) / 10}k`;
  }
  return String(val);
}

// Compact currency formatter for side panel
function formatCompact(val: number) {
  if (val >= 1000000) {
    return `${(val / 1000000).toFixed(1)}M YER`;
  }
  if (val >= 1000) {
    return `${(val / 1000).toFixed(0)}k YER`;
  }
  return `${val} YER`;
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
        <span class="h-1.5 w-1.5 rounded-full bg-emerald-500 {data.length > 0 ? 'animate-pulse' : ''}"></span>
        {m.reports_fee_collection_title()}
      </div>
    </div>

    <!-- Main Content Area: Side boxes + SVG Canvas or Blurred Empty State -->
    <div class="flex items-stretch gap-4 mt-2 flex-grow min-h-[160px] relative">
      
      <!-- Left Column: Stacked Sentry-style Metrics Boxes -->
      <div class="flex flex-col gap-2 shrink-0 justify-center select-none pe-2 border-e border-border/40 min-w-[75px]">
        <!-- Box 1: Collected -->
        <div class="flex flex-col gap-0.5 items-start bg-muted/20 border border-border/50 rounded-lg px-2 py-1.5 text-start w-full">
          <span class="text-[8px] font-extrabold text-emerald-500 uppercase tracking-wide">Collected</span>
          <span class="text-[10px] font-black text-foreground font-mono truncate max-w-[70px]">{formatCompact(totalCollected)}</span>
        </div>
        
        <!-- Box 2: Pending -->
        <div class="flex flex-col gap-0.5 items-start bg-muted/20 border border-border/50 rounded-lg px-2 py-1.5 text-start w-full">
          <span class="text-[8px] font-extrabold text-muted-foreground/70 uppercase tracking-wide">Pending</span>
          <span class="text-[10px] font-black text-foreground truncate max-w-[70px] font-mono">{formatCompact(totalPending)}</span>
        </div>
      </div>

      <!-- Right Column: Bar Chart covering the size of the card -->
      <div class="flex-grow relative h-full select-none flex items-center justify-center" onmouseleave={handleMouseLeave}>
        <svg
          viewBox={`0 0 ${width} ${height}`}
          class="w-full h-full select-none overflow-visible {data.length > 0 ? 'cursor-crosshair' : ''}"
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
            <!-- Left axis currency labels -->
            {@const labelVal = Math.round((1 - tick) * maxVal)}
            <text
              x={padding.left - 6}
              y={y + 3}
              class="text-[8px] font-bold fill-muted-foreground/80 font-mono"
              text-anchor="end"
            >
              {formatVal(labelVal)}
            </text>
          {/each}

          <!-- Active Column Slice Highlight background -->
          {#if hoveredIndex !== null && data.length > 0}
            {@const bar = bars[hoveredIndex]}
            <rect
              x={bar.col.x - 6}
              y={padding.top}
              width={bar.pend.x + bar.pend.width - bar.col.x + 12}
              height={chartHeight}
              class="fill-primary/5"
              rx={2}
            />
          {/if}

          <!-- Bars with rounded corners rx=2 (Rendered ONLY when data is available) -->
          {#if data.length > 0}
            {#each bars as bar, idx}
              <!-- Collected Bar (brand lime/emerald color) -->
              <rect
                x={bar.col.x}
                y={bar.col.y}
                width={bar.col.width}
                height={Math.max(1.5, bar.col.height)}
                rx={2}
                class="transition-colors duration-200 {hoveredIndex === idx ? 'fill-emerald-500' : 'fill-emerald-500/80'}"
              />

              <!-- Pending Bar (muted dark-gray/border color) -->
              <rect
                x={bar.pend.x}
                y={bar.pend.y}
                width={bar.pend.width}
                height={Math.max(1.5, bar.pend.height)}
                rx={2}
                class="transition-colors duration-200 {hoveredIndex === idx ? 'fill-muted-foreground' : 'fill-muted-foreground/70'}"
              />
            {/each}
          {/if}

          <!-- X-Axis Labels -->
          {#each bars as bar}
            <text
              x={bar.col.x + bar.col.width + 2}
              y={height - 3}
              class="text-[8px] font-bold fill-muted-foreground/80 font-mono"
              text-anchor="middle"
            >
              {bar.month}
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
            class="absolute z-30 p-2 text-[10px] font-sans border bg-card border-border text-foreground rounded-lg shadow-xl pointer-events-none transition-all duration-100 flex flex-col gap-1 min-w-[120px] text-start"
            style="left: {tooltipPos.x + 12}px; top: {tooltipPos.y - 70}px;"
          >
            <span class="text-muted-foreground font-bold border-b border-border/40 pb-0.5 mb-1">{activeBar.month}</span>
            <div class="flex flex-col gap-0.5">
              <div class="flex items-center justify-between gap-4">
                <span class="text-emerald-500 font-bold">{m.reports_fee_collected_label()}</span>
                <span class="font-bold text-foreground font-mono">{formatCurrency(activeBar.col.val)}</span>
              </div>
              <div class="flex items-center justify-between gap-4">
                <span class="text-muted-foreground font-bold">{m.reports_fee_pending_label()}</span>
                <span class="font-bold text-foreground font-mono">{formatCurrency(activeBar.pend.val)}</span>
              </div>
            </div>
          </div>
        {/if}
      </div>

      <!-- Backdrop Blur Overlay (Floats on top of BOTH columns when empty) -->
      {#if data.length === 0}
        <div class="absolute inset-0 flex flex-col items-center justify-center text-center select-none p-4 gap-1.5 backdrop-blur-[3.5px] bg-background/35 z-10 transition-all duration-300 rounded-xl">
          <span class="font-bold text-xs text-foreground/90">{m.reports_fees_empty_title()}</span>
          <span class="text-[10px] text-muted-foreground max-w-[280px] leading-relaxed">
            {m.reports_fees_empty_desc()}
          </span>
        </div>
      {/if}
    </div>

  {/if}
</div>
