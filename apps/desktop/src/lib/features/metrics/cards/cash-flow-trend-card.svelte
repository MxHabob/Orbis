<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Skeleton } from "@orbis/ui";
import {
  Card,
  CardHeader,
  CardTitle,
  CardAction,
  CardContent,
  CardFooter,
} from "@orbis/ui/card";
import { onMount } from "svelte";
import { api } from "$lib/api";
import DragIndicator from "../components/drag-indicator.svelte";

import { getLocale, localizeHref } from "$lib/paraglide/runtime";
import { formatCurrency } from "$lib/core/i18n";
import { ArrowRight } from "@orbis/icons";

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
    if (selectedTerm !== "all") params.period_id = selectedTerm;

    const paymentsList = await api.listPayments();
    const feesList = await api.listStudentFees();

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

const bars = $derived.by(() => {
  return displayData.map((item, idx) => {
    return {
      label: item.label,
      invPct: Math.min(100, (item.invoiced / maxVal) * 100),
      colPct: Math.min(100, (item.collected / maxVal) * 100),
      invoiced: item.invoiced,
      collected: item.collected,
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
  const colWidth = rect.width / displayData.length;
  const rawIdx = Math.floor(mouseX / colWidth);
  const idx = Math.min(
    displayData.length - 1,
    Math.max(0, isRtl ? displayData.length - 1 - rawIdx : rawIdx),
  );

  hoveredIdx = idx;
  tooltipPos = { x: mouseX, y: mouseY };
}

function handleMouseLeave() {
  hoveredIdx = null;
  tooltipPos = null;
}

function formatVal(val: number) {
  if (val >= 1000000)
    return `${(val / 1000000).toFixed(1).replace(/\.0$/, "")}M`;
  if (val >= 1000) return `${Math.round(val / 1000)}k`;
  return `${val}`;
}
</script>

<div class="card-tactile h-full w-full p-6 bg-card border border-border/60 flex flex-col justify-between text-start relative overflow-hidden shadow-sm min-h-[315px] font-sans">
  {#if isLoading}
    <div class="flex-grow w-full relative flex items-center justify-center">
      <Skeleton class="h-full w-full rounded-lg animate-pulse" />
    </div>
  {:else}
    <!-- Top Row Header + Legend -->
    <div class="flex items-start justify-between w-full select-none mb-3">
      <div class="flex flex-col text-start">
        <span class="text-sm font-semibold text-foreground/90">
          {m.finance_overview_collection_trend ? m.finance_overview_collection_trend() : "Cash Flow Trend"}
        </span>
      </div>
      <div class="flex items-center gap-2">
        {#if isEditing}
          <DragIndicator />
        {/if}
      </div>
      
      <!-- Legends: Invoiced (Primary color) and Collected (Emerald color) -->
      <div class="flex items-center gap-4 text-[10px] text-muted-foreground font-semibold">
        <div class="flex items-center gap-1.5">
          <span class="h-2.5 w-2.5 rounded-sm bg-muted-foreground/30 inline-block"></span>
          <span>{m.finance_overview_trend_invoiced ? m.finance_overview_trend_invoiced() : m.finance_overview_trend_invoiced()}</span>
        </div>
        <div class="flex items-center gap-1.5">
          <span class="h-2.5 w-2.5 rounded-sm bg-foreground inline-block"></span>
          <span>{m.finance_overview_trend_collected ? m.finance_overview_trend_collected() : m.finance_overview_trend_collected()}</span>
        </div>
      </div>
    </div>

    <!-- HTML Chart Area -->
    <div 
      class="flex-grow relative w-full h-[225px] select-none mt-2 flex flex-col justify-between" 
      role="application"
      onmouseleave={handleMouseLeave}
    >
      <!-- Horizontal Grid Lines background -->
      <div class="absolute inset-0 flex flex-col justify-between pb-8 pt-4 pointer-events-none">
        {#each Array(5) as _, i}
          {@const tickVal = maxVal - (i / 4) * maxVal}
          <div class="flex items-center w-full h-0">
            <span class="w-16 text-end text-[9px] font-bold text-muted-foreground/80 font-mono pe-2 select-none">
              {formatVal(tickVal)}
            </span>
            <div class="flex-grow border-t border-dashed border-border/40"></div>
          </div>
        {/each}
      </div>

      <!-- Columns Container -->
      <div 
        class="absolute inset-0 flex items-end justify-between ps-16 pe-2 pb-8 pt-4 cursor-crosshair"
        role="application"
        onmousemove={handleMouseMove}
      >
        {#each bars as bar, idx}
          {@const isActive = hoveredIdx === idx}
          <div class="flex-grow flex flex-col items-center justify-end h-full relative group/col">
            <!-- Active slice highlight background -->
            {#if isActive && data.length > 0}
              <div class="absolute inset-x-1 top-0 bottom-0 bg-foreground/5 rounded-md pointer-events-none"></div>
            {/if}

            <!-- The Double Bars -->
            {#if data.length > 0}
              <div class="flex items-end gap-1 md:gap-1.5 h-full pb-1 z-10">
                <!-- Invoiced Bar (Primary color tactile) -->
                <div 
                  class="w-2.5 sm:w-3.5 bar-tactile-last cursor-pointer"
                  style="height: {bar.invPct}%"
                  title="Invoiced"
                ></div>
                <!-- Collected Bar (Emerald tactile) -->
                <div 
                  class="w-2.5 sm:w-3.5 bar-tactile-this cursor-pointer"
                  style="height: {bar.colPct}%"
                  title="Collected"
                ></div>
              </div>
            {/if}

            <!-- X-Axis Label Centered Under Bars -->
            <div class="absolute bottom-[-24px] text-[9px] font-bold text-muted-foreground/90 font-mono select-none">
              {bar.label}
            </div>
          </div>
        {/each}
      </div>

      <!-- Floating Tooltip Box -->
      {#if hoveredIdx !== null && tooltipPos && data.length > 0}
        {@const activeBar = bars[hoveredIdx]}
        <div 
          class="absolute z-30 p-2.5 text-[10px] font-sans border bg-card border-border text-foreground rounded-lg shadow-xl pointer-events-none flex flex-col gap-1 min-w-[130px] text-start"
          style="left: {tooltipPos.x + 15}px; top: {tooltipPos.y - 65}px;"
        >
          <span class="text-muted-foreground font-bold border-b border-border/40 pb-0.5 mb-1">{activeBar.label}</span>
          <div class="flex flex-col gap-0.5">
            <div class="flex items-center justify-between gap-4">
              <span class="text-muted-foreground">{m.finance_overview_trend_invoiced ? m.finance_overview_trend_invoiced() : m.finance_overview_trend_invoiced()}:</span>
              <span class="font-bold font-mono text-foreground">{formatCurrency(activeBar.invoiced)}</span>
            </div>
            <div class="flex items-center justify-between gap-4">
              <span class="text-muted-foreground">{m.finance_overview_trend_collected ? m.finance_overview_trend_collected() : m.finance_overview_trend_collected()}:</span>
              <span class="font-bold font-mono text-foreground">{formatCurrency(activeBar.collected)}</span>
            </div>
          </div>
        </div>
      {/if}

      <!-- Backdrop Blur Overlay -->
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
    <div class="border-t border-border/40 pt-2 flex items-center justify-between w-full select-none text-[9px] font-bold mt-2">
      <div class="flex items-center gap-4">
        <a href={localizeHref(`/${orgId}/finance/statement`)} class="text-primary hover:underline flex items-center gap-1 group/statement">
          <span>{m.finance_overview_btn_statement ? m.finance_overview_btn_statement() : m.finance_overview_btn_statement()}</span>
          <ArrowRight size={10} class="transition-transform group-hover/statement:translate-x-0.5 rtl:rotate-180" />
        </a>
      </div>
    </div>
  {/if}
</div>
