<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Skeleton } from "@orbis/ui-svelte";
import { onMount } from "svelte";
import { commands } from "$lib/core/ipc";
import DragIndicator from "../components/drag-indicator.svelte";

import { getLocale } from "$lib/paraglide/runtime";

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

let data = $state<{ month: string; amount: number; lastYearAmount: number }[]>(
  [],
);
let isLoading = $state(true);

async function loadData() {
  isLoading = true;
  try {
    const payments = await commands.finance.listPayments(10000, 0, {
      class_id: selectedClass === "all" ? null : selectedClass,
      period_id: selectedTerm === "all" ? null : selectedTerm,
    });

    // Setup localized short names for 12 months (Jan to Dec)
    const months: { label: string; index: number }[] = [];
    for (let i = 0; i < 12; i++) {
      const dateObj = new Date(new Date().getFullYear(), i, 1);
      const label = new Intl.DateTimeFormat(getLocale(), {
        month: "short",
      }).format(dateObj);
      months.push({ label, index: i });
    }

    const currentYear = new Date().getFullYear();
    const thisYearMap = Array(12).fill(0) as number[];
    const lastYearMap = Array(12).fill(0) as number[];

    for (const payment of payments ?? []) {
      const dateString = String((payment as any).payment_date ?? payment.created_at ?? "");
      const date = new Date(dateString);
      if (!dateString || Number.isNaN(date.getTime())) continue;

      const amount = Number((payment as any).amount ?? 0);
      if (!Number.isFinite(amount)) continue;

      if (date.getFullYear() === currentYear) {
        thisYearMap[date.getMonth()] += amount;
      } else if (date.getFullYear() === currentYear - 1) {
        lastYearMap[date.getMonth()] += amount;
      }
    }

    data = months.map(({ label, index }) => ({
      month: label,
      amount: thisYearMap[index] ?? 0,
      lastYearAmount: lastYearMap[index] ?? 0,
    }));
  } catch (e) {
    console.error("Error loading finance revenue trends:", e);
    data = [];
  } finally {
    isLoading = false;
  }
}

$effect(() => {
  if (orgId || selectedClass || selectedTerm || period) {
    loadData();
  }
});

// Define Y ticks based on max value (e.g. max is around 65k, tick upper bound 80k)
const maxVal = 80000;
const yTicks = [0, 20000, 60000, 80000];

const bars = $derived.by(() => {
  return data.map((item, idx) => {
    return {
      month: item.month,
      lastYearPct: Math.min(100, (item.lastYearAmount / maxVal) * 100),
      thisYearPct: Math.min(100, (item.amount / maxVal) * 100),
      lastYearAmount: item.lastYearAmount,
      amount: item.amount,
    };
  });
});

let hoveredIdx = $state<number | null>(null);
let tooltipPos = $state<{ x: number; y: number } | null>(null);

function handleMouseMove(e: MouseEvent & { currentTarget: HTMLDivElement }) {
  if (data.length === 0 || bars.length === 0) return;
  const rect = e.currentTarget.getBoundingClientRect();
  const mouseX = e.clientX - rect.left;
  const mouseY = e.clientY - rect.top;

  const isRtl = document.documentElement.dir === "rtl";
  const colWidth = rect.width / data.length;
  const rawIdx = Math.floor(mouseX / colWidth);
  const idx = Math.min(
    data.length - 1,
    Math.max(0, isRtl ? data.length - 1 - rawIdx : rawIdx),
  );

  hoveredIdx = idx;
  tooltipPos = { x: mouseX, y: mouseY };
}

function handleMouseLeave() {
  hoveredIdx = null;
  tooltipPos = null;
}

// Compact currency formatter
function formatVal(val: number) {
  if (val === 0) return "0";
  if (val >= 1000) return `€${Math.round(val / 1000)}k`;
  return `€${val}`;
}

function formatTooltipVal(val: number) {
  return `€${val.toLocaleString()}`;
}
</script>

<div class="card-tactile h-full relative group min-h-78.75 p-6 flex flex-col justify-between font-sans border border-border/60 bg-card shadow-sm">
  {#if isLoading}
    <div class="grow w-full relative flex items-center justify-center">
      <Skeleton class="h-full w-full rounded-lg animate-pulse" />
    </div>
  {:else}
    <!-- Top Row Header + Legend -->
    <div class="flex items-start justify-between w-full select-none mb-3">
      <div class="flex flex-col text-start">
        <span class="text-sm font-semibold text-foreground/90">{m.reports_revenue_trend_title()}</span>
      </div>
      <div class="flex items-center gap-2">
        {#if isEditing}
          <DragIndicator />
        {/if}
      </div>
      
      <!-- Legends: Last year (gray block) and This year (black/white block) -->
      <div class="flex items-center gap-4 text-[10px] text-muted-foreground font-semibold">
        <div class="flex items-center gap-1.5">
          <span class="h-2.5 w-2.5 rounded-sm bg-muted-foreground/30 inline-block"></span>
          <span>{m.reports_revenue_trend_last_year()}</span>
        </div>
        <div class="flex items-center gap-1.5">
          <span class="h-2.5 w-2.5 rounded-sm bg-foreground inline-block"></span>
          <span>{m.common_year_current()}</span>
        </div>
      </div>
    </div>

    <!-- Main Chart Area -->
    <div 
      role="presentation"
      class="grow relative w-full h-56.25 select-none mt-2 flex flex-col justify-between"
      onmouseleave={handleMouseLeave}
    >
      <!-- Horizontal Grid Lines background -->
      <div class="absolute inset-0 flex flex-col justify-between pb-8 pt-4 pointer-events-none">
        {#each yTicks.slice().reverse() as tickVal}
          <div class="flex items-center w-full h-0">
            <span class="w-10 text-end text-[9px] font-bold text-muted-foreground/80 font-mono pe-2 select-none">
              {formatVal(tickVal)}
            </span>
            <div class="grow border-t border-dashed border-border/40"></div>
          </div>
        {/each}
      </div>

      <!-- Columns Container -->
      <div 
        role="presentation"
        class="absolute inset-0 flex items-end justify-between ps-10 pe-2 pb-8 pt-4 cursor-crosshair"
        onmousemove={handleMouseMove}
      >
        {#each bars as bar, idx}
          {@const isActive = hoveredIdx === idx}
          <div class="grow flex flex-col items-center justify-end h-full relative group/col">
            <!-- Active slice highlight background -->
            {#if isActive}
              <div class="absolute inset-x-1 top-0 bottom-0 bg-foreground/5 rounded-md pointer-events-none"></div>
            {/if}

            <!-- The Double Bars -->
            <div class="flex items-end gap-1 md:gap-1.5 h-full pb-1 z-10">
              <!-- Last Year Bar -->
              <div 
                class="w-2.5 sm:w-3.5 bar-tactile-last"
                style="height: {bar.lastYearPct}%"
                title="Last Year"
              ></div>
              <!-- This Year Bar -->
              <div 
                class="w-2.5 sm:w-3.5 bar-tactile-this"
                style="height: {bar.thisYearPct}%"
                title="This Year"
              ></div>
            </div>

            <!-- X-Axis Label Centered Under Bars -->
            <div class="absolute -bottom-6 text-[9px] font-bold text-muted-foreground/90 font-mono select-none">
              {bar.month}
            </div>
          </div>
        {/each}
      </div>

      <!-- Tooltip -->
      {#if hoveredIdx !== null && tooltipPos}
        {@const activeBar = bars[hoveredIdx]}
        <div
          class="absolute z-30 p-2.5 text-[10px] font-sans border bg-card border-border text-foreground rounded-lg shadow-xl pointer-events-none flex flex-col gap-1 min-w-32.5 text-start"
          style="left: {tooltipPos.x + 15}px; top: {tooltipPos.y - 65}px;"
        >
          <span class="text-muted-foreground font-bold border-b border-border/40 pb-0.5 mb-1">{activeBar.month}</span>
          <div class="flex flex-col gap-0.5">
            <div class="flex items-center justify-between gap-4">
              <span class="text-muted-foreground">{m.common_year_current()}:</span>
              <span class="font-bold font-mono text-foreground">{formatTooltipVal(activeBar.amount)}</span>
            </div>
            <div class="flex items-center justify-between gap-4">
              <span class="text-muted-foreground">{m.reports_revenue_trend_last_year()}:</span>
              <span class="font-bold font-mono text-foreground">{formatTooltipVal(activeBar.lastYearAmount)}</span>
            </div>
          </div>
        </div>
      {/if}
    </div>
  {/if}
</div>
