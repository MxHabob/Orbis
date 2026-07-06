<script lang="ts">
import { Skeleton } from "@orbis/ui-svelte";
import { onMount } from "svelte";
import { commands } from "$lib/core/ipc";
import DragIndicator from "../components/drag-indicator.svelte";
import * as m from "$lib/paraglide/messages";
import { getLocale, localizeHref } from "$lib/paraglide/runtime";
import { formatCurrency } from "$lib/core/i18n";
import { ArrowRight } from "lucide-svelte";

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
let totalFees = $state(0);
let totalPaid = $state(0);
let isLoading = $state(true);

async function loadData() {
  isLoading = true;
  try {
    const params: any = {};
    if (selectedClass !== "all") params.class_id = selectedClass;
    if (selectedTerm !== "all") params.cycle_id = selectedTerm;

    const paymentsList = await commands.finance.getPayments(orgId);
    const feesList = await commands.finance.getStudentFees(orgId);

    if (paymentsList && paymentsList.length > 0) {
      const monthsMap: Record<string, { invoiced: number; collected: number }> =
        {};

      for (const p of paymentsList) {
        const dateStr = (p.payment_date || p.created_at || "") as string;
        if (!dateStr) continue;
        const monthKey = dateStr.substring(0, 7);
        if (!monthsMap[monthKey]) {
          monthsMap[monthKey] = { invoiced: 0, collected: 0 };
        }
        monthsMap[monthKey].collected += Number(p.amount || 0);
      }

      if (feesList) {
        for (const f of feesList) {
          const dateStr = (f.due_date || f.created_at || "") as string;
          if (!dateStr) continue;
          const monthKey = dateStr.substring(0, 7);
          if (!monthsMap[monthKey]) {
            monthsMap[monthKey] = { invoiced: 0, collected: 0 };
          }
          monthsMap[monthKey].invoiced += Number(
            f.net_amount || f.original_amount || 0,
          );
        }
      }

      const sortedMonths = Object.keys(monthsMap).sort().slice(-6);
      totalFees = 0;
      totalPaid = 0;

      data = sortedMonths.map((mon) => {
        const vals = monthsMap[mon]!;
        totalFees += vals.invoiced;
        totalPaid += vals.collected;

        const year = Number(mon.split("-")[0]);
        const monthIndex = Number(mon.split("-")[1]) - 1;
        const dateObj = new Date(year, monthIndex, 1);
        const formattedMonth = new Intl.DateTimeFormat(getLocale(), {
          month: "short",
        }).format(dateObj);

        return {
          label: formattedMonth,
          invoiced: vals.invoiced,
          collected: vals.collected,
        };
      });
    } else {
      data = [];
      totalFees = 0;
      totalPaid = 0;
    }
  } catch (e) {
    console.error("Error loading cash flow trends:", e);
    data = [];
    totalFees = 0;
    totalPaid = 0;
  } finally {
    isLoading = false;
  }
}

$effect(() => {
  if (orgId || selectedClass || selectedTerm || period) {
    loadData();
  }
});

// SVG dimensions
const chartWidth = 800;
const chartHeight = 150;
const padding = { left: 65, right: 15, top: 15, bottom: 20 };
const innerWidth = chartWidth - padding.left - padding.right;
const innerHeight = chartHeight - padding.top - padding.bottom;

// Determine display dataset (uses empty months placeholders when empty)
const displayData = $derived(
  data.length > 0
    ? data
    : [
        { label: "Oct", invoiced: 0, collected: 0 },
        { label: "Nov", invoiced: 0, collected: 0 },
        { label: "Dec", invoiced: 0, collected: 0 },
        { label: "Jan", invoiced: 0, collected: 0 },
        { label: "Feb", invoiced: 0, collected: 0 },
        { label: "Mar", invoiced: 0, collected: 0 },
      ],
);

const maxVal = $derived(
  Math.max(1000, ...displayData.map((d) => Math.max(d.invoiced, d.collected))) *
    1.15,
);

const groupWidth = $derived(innerWidth / (displayData.length || 1));
const barWidth = $derived(Math.max(6, (groupWidth - 20) / 2));

const bars = $derived.by(() => {
  return displayData.map((item, idx) => {
    const xInv =
      padding.left + idx * groupWidth + (groupWidth - 2 * barWidth - 4) / 2;
    const xCol = xInv + barWidth + 4;

    const pctInv = item.invoiced / maxVal;
    const hInv = pctInv * innerHeight;
    const yInv = padding.top + innerHeight - hInv;

    const pctCol = item.collected / maxVal;
    const hCol = pctCol * innerHeight;
    const yCol = padding.top + innerHeight - hCol;

    return {
      label: item.label,
      inv: {
        x: xInv,
        y: yInv,
        width: barWidth,
        height: hInv,
        val: item.invoiced,
      },
      col: {
        x: xCol,
        y: yCol,
        width: barWidth,
        height: hCol,
        val: item.collected,
      },
    };
  });
});

// Interactive Tooltip State
let hoveredIdx = $state<number | null>(null);
let tooltipX = $state(0);
let tooltipY = $state(0);

function handleMouseMove(e: MouseEvent & { currentTarget: SVGSVGElement }) {
  if (data.length === 0 || bars.length === 0) return;
  const rect = e.currentTarget.getBoundingClientRect();
  const mouseX = e.clientX - rect.left;
  const svgX = (mouseX / rect.width) * chartWidth;

  const step = innerWidth / (displayData.length - 1);
  const relativeX = svgX - padding.left;
  let idx = Math.round(relativeX / step);
  if (idx < 0) idx = 0;
  if (idx >= displayData.length) idx = displayData.length - 1;

  hoveredIdx = idx;

  const activeBar = bars[idx];
  tooltipX =
    ((activeBar.inv.x + activeBar.col.x + barWidth) / 2 / chartWidth) *
    rect.width;
  const highestY = Math.min(activeBar.inv.y, activeBar.col.y);
  tooltipY = (highestY / chartHeight) * rect.height - 10;
}

function handleMouseLeave() {
  hoveredIdx = null;
}
</script>

<div class="card-tactile h-full w-full p-4 bg-card/60 border border-border flex flex-col justify-between text-start relative overflow-hidden shadow-sm min-h-[235px] font-sans">
  {#if isLoading}
    <div class="flex-grow w-full relative flex items-center justify-center">
      <Skeleton class="h-full w-full rounded-lg animate-pulse" />
    </div>
  {:else}
    <!-- Top info bar with small Releases-style label on the right -->
    <div class="flex items-center justify-between w-full h-5 select-none text-[9px] font-bold text-muted-foreground/80">
      <div class="flex items-center gap-2">
        {#if isEditing}
          <DragIndicator />
        {/if}
      </div>
      <div class="flex items-center gap-1.5 font-sans font-extrabold uppercase tracking-wider text-[8px] text-muted-foreground/75">
        <span class="h-1.5 w-1.5 rounded-full bg-primary {data.length > 0 ? 'animate-pulse' : ''}"></span>
        {m.finance_overview_collection_trend ? m.finance_overview_collection_trend() : "Cash Flow Trend"}
      </div>
    </div>

    <!-- SVG Container -->
    <div class="relative w-full h-[160px] select-none mt-2 flex items-center justify-center" onmouseleave={handleMouseLeave}>
      <svg viewBox={`0 0 ${chartWidth} ${chartHeight}`} class="w-full h-full overflow-visible {data.length > 0 ? 'cursor-crosshair' : ''}">
        <!-- Grid lines -->
        {#each Array(5) as _, i}
          {@const y = padding.top + (i / 4) * innerHeight}
          <line x1={padding.left} y1={y} x2={chartWidth - padding.right} y2={y} class="stroke-border/40" stroke-width="1" stroke-dasharray="3 3" />
          <text x={padding.left - 10} y={y + 3.5} class="text-[8px] font-bold fill-muted-foreground/80 font-mono text-end" text-anchor="end">
            {formatCurrency(maxVal - (i / 4) * maxVal, "YER")}
          </text>
        {/each}

        <!-- X Axis Monthly Labels -->
        {#each bars as bar}
          <text x={bar.inv.x + bar.inv.width + 2} y={chartHeight - 3} class="text-[8px] font-bold fill-muted-foreground/90 font-mono text-center" text-anchor="middle">
            {bar.label}
          </text>
        {/each}

        <!-- Active Column Slice Highlight background -->
        {#if hoveredIdx !== null && data.length > 0}
          {@const bar = bars[hoveredIdx]}
          <rect
            x={bar.inv.x - 6}
            y={padding.top}
            width={bar.col.x + bar.col.width - bar.inv.x + 12}
            height={innerHeight}
            class="fill-primary/5"
            rx={2}
          />
        {/if}

        <!-- Bars (Rendered ONLY when there is actual data in the database) -->
        {#if data.length > 0}
          {#each bars as bar, idx}
            <!-- Invoiced Bar (Primary) -->
            <rect
              x={bar.inv.x}
              y={bar.inv.y}
              width={bar.inv.width}
              height={Math.max(1.5, bar.inv.height)}
              rx={2}
              class="transition-colors duration-200 {hoveredIdx === idx ? 'fill-primary' : 'fill-primary/80'}"
            />

            <!-- Collected Bar (Emerald) -->
            <rect
              x={bar.col.x}
              y={bar.col.y}
              width={bar.col.width}
              height={Math.max(1.5, bar.col.height)}
              rx={2}
              class="transition-colors duration-200 {hoveredIdx === idx ? 'fill-emerald-500' : 'fill-emerald-500/80'}"
            />
          {/each}
        {/if}
      </svg>

      <!-- Mouse Tracker Box Overlay -->
      {#if data.length > 0}
        <svg class="absolute inset-0 w-full h-full opacity-0 cursor-crosshair" viewBox={`0 0 ${chartWidth} ${chartHeight}`} onmousemove={handleMouseMove}></svg>
      {/if}

      <!-- Floating Tooltip Box -->
      {#if hoveredIdx !== null && data.length > 0}
        {@const item = displayData[hoveredIdx]}
        <div class="absolute z-20 bg-card border border-border shadow-xl rounded-lg p-2.5 text-start flex flex-col gap-1 text-[10px] min-w-[130px] pointer-events-none transition-all duration-100" style="left: {tooltipX}px; top: {tooltipY}px; transform: translate(-50%, -100%);">
          <span class="font-bold text-foreground pb-1 border-b border-border/40">{item.label}</span>
          <div class="flex items-center justify-between gap-3 mt-1 text-muted-foreground">
            <span class="flex items-center gap-1">
              <span class="h-1.5 w-1.5 rounded-full bg-primary"></span>
              {m.finance_overview_trend_invoiced ? m.finance_overview_trend_invoiced() : "الرسوم المقررة"}
            </span>
            <span class="font-bold text-foreground font-mono">{formatCurrency(item.invoiced)}</span>
          </div>
          <div class="flex items-center justify-between gap-3 text-muted-foreground">
            <span class="flex items-center gap-1">
              <span class="h-1.5 w-1.5 rounded-full bg-emerald-500"></span>
              {m.finance_overview_trend_collected ? m.finance_overview_trend_collected() : "المدفوعات"}
            </span>
            <span class="font-bold text-emerald-500 font-mono">{formatCurrency(item.collected)}</span>
          </div>
        </div>
      {/if}

      <!-- Backdrop Blur Overlay (No borders, no alert icons, frosted glass blur effect) -->
      {#if data.length === 0}
        <div class="absolute inset-0 flex flex-col items-center justify-center text-center select-none p-4 gap-1.5 backdrop-blur-[3.5px] bg-background/35 z-10 transition-all duration-300 rounded-xl">
          <span class="font-bold text-xs text-foreground/90">{m.finance_overview_empty_records()}</span>
          <span class="text-[9px] text-muted-foreground max-w-[340px] leading-relaxed">
            {m.finance_overview_empty_records_desc()}
          </span>
        </div>
      {/if}
    </div>

    <!-- Elegant Footer Links -->
    <div class="border-t border-border/40 pt-2 flex items-center justify-between w-full select-none text-[9px] font-bold">
      <div class="flex items-center gap-4">
        <a href={localizeHref(`/${orgId}/finance/statement`)} class="text-primary hover:underline flex items-center gap-1 group/statement">
          <span>{m.finance_overview_btn_statement ? m.finance_overview_btn_statement() : "كشف الحساب التفصيلي"}</span>
          <ArrowRight size={10} class="transition-transform group-hover/statement:translate-x-0.5 rtl:rotate-180" />
        </a>
      </div>
    </div>
  {/if}
</div>
