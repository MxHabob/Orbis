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
  Math.max(1000, ...displayData.map((d) => Math.max(d.invoiced, d.collected))) * 1.15
);

const bars = $derived.by(() => {
  return displayData.map((item, idx) => {
    return {
      label: item.label,
      invPct: Math.min(100, (item.invoiced / maxVal) * 100),
      colPct: Math.min(100, (item.collected / maxVal) * 100),
      invoiced: item.invoiced,
      collected: item.collected
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
  const idx = Math.min(displayData.length - 1, Math.max(0, isRtl ? (displayData.length - 1 - rawIdx) : rawIdx));

  hoveredIdx = idx;
  tooltipPos = { x: mouseX, y: mouseY };
}

function handleMouseLeave() {
  hoveredIdx = null;
  tooltipPos = null;
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

    <!-- HTML Chart Area -->
    <div 
      class="flex-grow relative w-full h-[160px] select-none mt-2 flex flex-col justify-between" 
      onmouseleave={handleMouseLeave}
    >
      <!-- Horizontal Grid Lines background -->
      <div class="absolute inset-0 flex flex-col justify-between pb-8 pt-4 pointer-events-none">
        {#each Array(5) as _, i}
          {@const tickVal = maxVal - (i / 4) * maxVal}
          <div class="flex items-center w-full h-0">
            <span class="w-16 text-end text-[8px] font-bold text-muted-foreground/80 font-mono pr-2 select-none">
              {formatCurrency(tickVal, "YER")}
            </span>
            <div class="flex-grow border-t border-dashed border-border/40"></div>
          </div>
        {/each}
      </div>

      <!-- Columns Container -->
      <div 
        class="absolute inset-0 flex items-end justify-between pl-16 pr-2 pb-8 pt-4 cursor-crosshair"
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
                  class="w-2.5 sm:w-3.5 bg-primary shadow-[0_3px_0_0_oklch(0.35_0.1_280)] dark:shadow-[0_3px_0_0_oklch(0.3_0.1_280)] hover:-translate-y-1 transition-all rounded-t-md cursor-pointer duration-150"
                  style="height: {bar.invPct}%"
                  title="Invoiced"
                ></div>
                <!-- Collected Bar (Emerald tactile) -->
                <div 
                  class="w-2.5 sm:w-3.5 bg-emerald-500 shadow-[0_3px_0_0_oklch(0.2_0.15_140)] dark:shadow-[0_3px_0_0_oklch(0.1_0.15_140)] hover:-translate-y-1 transition-all rounded-t-md cursor-pointer duration-150"
                  style="height: {bar.colPct}%"
                  title="Collected"
                ></div>
              </div>
            {/if}

            <!-- X-Axis Label Centered Under Bars -->
            <div class="absolute bottom-[-24px] text-[8px] font-bold text-muted-foreground/90 font-mono select-none">
              {bar.label}
            </div>
          </div>
        {/each}
      </div>

      <!-- Floating Tooltip Box -->
      {#if hoveredIdx !== null && tooltipPos && data.length > 0}
        {@const activeBar = bars[hoveredIdx]}
        <div class="absolute z-20 bg-card border border-border shadow-xl rounded-lg p-2.5 text-start flex flex-col gap-1 text-[10px] min-w-[130px] pointer-events-none transition-all duration-100" style="left: {tooltipPos.x}px; top: {tooltipPos.y - 10}px; transform: translate(-50%, -100%);">
          <span class="font-bold text-foreground pb-1 border-b border-border/40">{activeBar.label}</span>
          <div class="flex items-center justify-between gap-3 mt-1 text-muted-foreground">
            <span class="flex items-center gap-1">
              <span class="h-1.5 w-1.5 rounded-full bg-primary"></span>
              {m.finance_overview_trend_invoiced ? m.finance_overview_trend_invoiced() : "الرسوم المقررة"}
            </span>
            <span class="font-bold text-foreground font-mono">{formatCurrency(activeBar.invoiced)}</span>
          </div>
          <div class="flex items-center justify-between gap-3 text-muted-foreground">
            <span class="flex items-center gap-1">
              <span class="h-1.5 w-1.5 rounded-full bg-emerald-500"></span>
              {m.finance_overview_trend_collected ? m.finance_overview_trend_collected() : "المدفوعات"}
            </span>
            <span class="font-bold text-emerald-500 font-mono">{formatCurrency(activeBar.collected)}</span>
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
