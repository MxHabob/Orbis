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
import { api, type EnrollmentFilterParams } from "$lib/api";
import DragIndicator from "../components/drag-indicator.svelte";

import { localizeHref } from "$lib/paraglide/runtime";
import { formatCurrency, formatDate } from "$lib/core/i18n";
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

let data = $state<{ label: string; invoiced: number; collected: number }[]>([]);
let isLoading = $state(true);
let loadError = $state(false);
let loadSequence = 0;

async function loadData() {
  const sequence = ++loadSequence;
  isLoading = true;
  loadError = false;

  try {
    const enrollmentFilter: EnrollmentFilterParams = {};
    if (selectedClass !== "all") enrollmentFilter.class_id = selectedClass;
    if (selectedTerm !== "all") enrollmentFilter.period_id = selectedTerm;
    const needsEnrollmentFilter =
      selectedClass !== "all" || selectedTerm !== "all";

    const [paymentsList, feesList, enrollments] = await Promise.all([
      api.listPayments(),
      api.listStudentFees(
        selectedTerm === "all" ? undefined : { period_id: selectedTerm },
      ),
      needsEnrollmentFilter ? api.listEnrollments(enrollmentFilter) : [],
    ]);

    if (sequence !== loadSequence) return;

    const enrollmentIds = new Set(
      enrollments.map((enrollment) => enrollment.id),
    );
    const filteredFees = feesList.filter(
      (fee) =>
        selectedClass === "all" ||
        (fee.enrollment_id != null && enrollmentIds.has(fee.enrollment_id)),
    );
    const feeIds = new Set(filteredFees.map((fee) => fee.id));
    const filteredPayments = needsEnrollmentFilter
      ? paymentsList.filter((payment) =>
          payment.fee_id
            ? feeIds.has(payment.fee_id)
            : payment.enrollment_id != null &&
              enrollmentIds.has(payment.enrollment_id),
        )
      : paymentsList;

    const currentMonth = new Date();
    const monthBuckets = Array.from({ length: 6 }, (_, index) => {
      const monthDate = new Date(
        Date.UTC(
          currentMonth.getUTCFullYear(),
          currentMonth.getUTCMonth() - 5 + index,
          1,
        ),
      );
      const key = `${monthDate.getUTCFullYear()}-${String(
        monthDate.getUTCMonth() + 1,
      ).padStart(2, "0")}`;
      return {
        key,
        label: formatDate(monthDate, "date", {
          month: "short",
          timeZone: "UTC",
        }),
        invoiced: 0,
        collected: 0,
      };
    });
    const monthsByKey = new Map(
      monthBuckets.map((month) => [month.key, month]),
    );

    for (const fee of filteredFees) {
      const monthKey = getMonthKey(fee.due_date);
      if (!monthKey) continue;
      const bucket = monthsByKey.get(monthKey);
      if (bucket) bucket.invoiced += toAmount(fee.net_amount);
    }

    for (const payment of filteredPayments) {
      const monthKey = getMonthKey(payment.paid_at);
      if (!monthKey) continue;
      const bucket = monthsByKey.get(monthKey);
      if (bucket) bucket.collected += toAmount(payment.amount);
    }

    data = monthBuckets.map(({ label, invoiced, collected }) => ({
      label,
      invoiced,
      collected,
    }));
  } catch (e) {
    console.error("Error loading cash flow trends:", e);
    if (sequence === loadSequence) {
      loadError = true;
    }
  } finally {
    if (sequence === loadSequence) {
      isLoading = false;
    }
  }
}

$effect(() => {
  void orgId;
  void selectedClass;
  void selectedTerm;
  void period;
  void loadData();
});

const displayData = $derived(data);
const hasRecords = $derived(
  data.some((month) => month.invoiced !== 0 || month.collected !== 0),
);
const maxVal = $derived(
  getNiceMaximum(
    Math.max(
      0,
      ...displayData.map((month) => Math.max(month.invoiced, month.collected)),
    ) * 1.1,
  ),
);

const bars = $derived.by(() => {
  return displayData.map((item) => {
    return {
      label: item.label,
      invPct: Math.max(0, Math.min(100, (item.invoiced / maxVal) * 100)),
      colPct: Math.max(0, Math.min(100, (item.collected / maxVal) * 100)),
      invoiced: item.invoiced,
      collected: item.collected,
    };
  });
});

let hoveredIdx = $state<number | null>(null);
let tooltipPos = $state<{ x: number; y: number } | null>(null);

function handleMouseMove(e: MouseEvent & { currentTarget: HTMLDivElement }) {
  if (!hasRecords || bars.length === 0) return;
  const rect = e.currentTarget.getBoundingClientRect();
  const mouseX = e.clientX - rect.left;
  const mouseY = e.clientY - rect.top;
  const isRtl = document.documentElement.dir === "rtl";
  const plotWidth = Math.max(1, rect.width - 72);
  const plotX = isRtl ? rect.width - mouseX - 8 : mouseX - 64;
  const colWidth = plotWidth / displayData.length;
  const idx = Math.max(
    0,
    Math.min(displayData.length - 1, Math.floor(plotX / colWidth)),
  );

  hoveredIdx = idx;
  tooltipPos = {
    x: Math.min(Math.max(8, mouseX + 12), Math.max(8, rect.width - 228)),
    y: Math.min(Math.max(8, mouseY - 65), Math.max(8, rect.height - 90)),
  };
}

function handleMouseLeave() {
  hoveredIdx = null;
  tooltipPos = null;
}

function formatVal(val: number) {
  if (val >= 1000000)
    return `${(val / 1000000).toFixed(1).replace(/\.0$/, "")}M`;
  if (val >= 1000) return `${(val / 1000).toFixed(1).replace(/\.0$/, "")}k`;
  return `${val}`;
}

function getMonthKey(value: string | null | undefined): string | undefined {
  if (!value) return undefined;
  const date = new Date(value);
  if (Number.isNaN(date.getTime())) return undefined;
  return `${date.getUTCFullYear()}-${String(date.getUTCMonth() + 1).padStart(2, "0")}`;
}

function toAmount(value: string | number | null | undefined): number {
  const amount = Number(value ?? 0);
  return Number.isFinite(amount) ? amount : 0;
}

function getNiceMaximum(value: number): number {
  if (!Number.isFinite(value) || value <= 0) return 1000;
  const magnitude = 10 ** Math.floor(Math.log10(value));
  const normalized = value / magnitude;
  const step =
    normalized <= 1 ? 1 : normalized <= 2 ? 2 : normalized <= 5 ? 5 : 10;
  return step * magnitude;
}
</script>

<Card class="h-full w-full flex flex-col justify-between text-start relative overflow-hidden shadow-sm min-h-[315px] font-sans">
  {#if isLoading}
    <div class="flex-grow w-full p-6 relative flex items-center justify-center">
      <Skeleton class="h-full w-full rounded-lg animate-pulse" />
    </div>
  {:else}
    <!-- Top Row Header + Legend -->
    <CardHeader class="py-3 px-4">
      <div class="flex items-center gap-2">
        <CardTitle class="text-xs font-bold uppercase tracking-wider text-foreground">
          {m.finance_overview_collection_trend()}
        </CardTitle>
        {#if isEditing}
          <DragIndicator />
        {/if}
      </div>

      <CardAction>
        <div class="flex items-center gap-4 text-[10px] text-muted-foreground font-semibold select-none">
          <div class="flex items-center gap-1.5">
            <span class="h-2.5 w-2.5 rounded-sm bg-muted-foreground/30 inline-block"></span>
            <span>{m.finance_overview_trend_invoiced()}</span>
          </div>
          <div class="flex items-center gap-1.5">
            <span class="h-2.5 w-2.5 rounded-sm bg-foreground inline-block"></span>
            <span>{m.finance_overview_trend_collected()}</span>
          </div>
        </div>
      </CardAction>
    </CardHeader>

    <!-- HTML Chart Area -->
    <CardContent class="p-4 pt-2 flex-grow flex flex-col justify-between">
      <div 
        class="flex-grow relative w-full h-[225px] select-none flex flex-col justify-between" 
        role="img"
        aria-label={m.finance_overview_collection_trend()}
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
          role="presentation"
          onmousemove={handleMouseMove}
        >
          {#each bars as bar, idx}
            {@const isActive = hoveredIdx === idx}
            <div class="flex-grow flex flex-col items-center justify-end h-full relative group/col">
              <!-- Active slice highlight background -->
              {#if isActive && hasRecords}
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
        {#if hoveredIdx !== null && tooltipPos && hasRecords}
          {@const activeBar = bars[hoveredIdx]}
          <div 
            class="absolute z-30 w-[220px] max-w-[calc(100%-16px)] p-2.5 text-[10px] font-sans border bg-card border-border text-foreground rounded-lg shadow-xl pointer-events-none flex flex-col gap-1 text-start"
            style="left: {tooltipPos.x}px; top: {tooltipPos.y}px;"
          >
            <span class="text-muted-foreground font-bold border-b border-border/40 pb-0.5 mb-1">{activeBar.label}</span>
            <div class="flex flex-col gap-0.5">
              <div class="flex items-center justify-between gap-4">
                <span class="text-muted-foreground">{m.finance_overview_trend_invoiced()}:</span>
                <span class="font-bold font-mono text-foreground">{formatCurrency(activeBar.invoiced)}</span>
              </div>
              <div class="flex items-center justify-between gap-4">
                <span class="text-muted-foreground">{m.finance_overview_trend_collected()}:</span>
                <span class="font-bold font-mono text-foreground">{formatCurrency(activeBar.collected)}</span>
              </div>
            </div>
          </div>
        {/if}

        <!-- Backdrop Blur Overlay -->
        {#if loadError}
          <div class="absolute inset-0 flex flex-col items-center justify-center text-center p-4 gap-2 bg-background/90 z-20 rounded-xl" role="alert">
            <span class="font-bold text-xs text-foreground/90">{m.finance_overview_error()}</span>
            <button type="button" class="text-[10px] font-semibold text-primary hover:underline" onclick={loadData}>
              {m.common_retry()}
            </button>
          </div>
        {:else if !hasRecords}
          <div class="absolute inset-0 flex flex-col items-center justify-center text-center select-none p-4 gap-1.5 backdrop-blur-[3.5px] bg-background/35 z-10 transition-all duration-300 rounded-xl">
            <span class="font-bold text-xs text-foreground/90">{m.finance_overview_empty_records()}</span>
            <span class="text-[9px] text-muted-foreground max-w-[340px] leading-relaxed">
              {m.finance_overview_empty_records_desc()}
            </span>
          </div>
        {/if}
      </div>
    </CardContent>

    <!-- Footer Links -->
    <CardFooter class="border-t border-border/40 py-2.5 px-4 flex items-center justify-between w-full select-none text-[9px] font-bold">
      <div class="flex items-center gap-4">
        <a href={localizeHref(`/${orgId}/finance/statement`)} class="text-primary hover:underline flex items-center gap-1 group/statement">
          <span>{m.finance_overview_btn_statement()}</span>
          <ArrowRight size={10} class="transition-transform group-hover/statement:translate-x-0.5 rtl:rotate-180" />
        </a>
      </div>
    </CardFooter>
  {/if}
</Card>
