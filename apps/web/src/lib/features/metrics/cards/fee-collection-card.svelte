<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Skeleton } from "@orbis/ui";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
} from "@orbis/ui/select";
import {
  createPaymentsQuery,
  createStudentFeesQuery,
} from "$lib/features/finance/queries";
import { createEnrollmentsQuery } from "$lib/features/students";
import { orgContext } from "$lib/stores/org-context.svelte";
import { formatDateKey } from "$lib/core/i18n/format";
import DragIndicator from "../components/drag-indicator.svelte";

import { formatCurrency, formatDate, formatNumber } from "$lib/core/i18n";

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

const today = $derived(formatDateKey(new Date()));
const currentMonth = $derived(new Date(today.slice(0, 7) + "-01T00:00:00Z"));
const firstMonth = $derived.by(() => {
  const month = new Date(currentMonth);
  if (period === "3-months") month.setUTCMonth(month.getUTCMonth() - 2);
  else if (period === "6-months") month.setUTCMonth(month.getUTCMonth() - 5);
  else month.setUTCMonth(0);
  return month;
});
const needsEnrollmentFilter = $derived(
  selectedClass !== "all" || selectedTerm !== "all",
);
const paymentsQuery = createPaymentsQuery(
  () => orgId,
  () => ({
    from_date:
      period === "all"
        ? undefined
        : `${firstMonth.getUTCFullYear() - 1}-${String(firstMonth.getUTCMonth() + 1).padStart(2, "0")}-01`,
    to_date: today,
  }),
);
const feesQuery = createStudentFeesQuery(
  () => orgId,
  () => (selectedTerm === "all" ? undefined : { period_id: selectedTerm }),
  () => needsEnrollmentFilter,
);
const enrollmentsQuery = createEnrollmentsQuery(
  () => orgId,
  () => ({
    class_id: selectedClass === "all" ? undefined : selectedClass,
    period_id: selectedTerm === "all" ? undefined : selectedTerm,
  }),
);
const isLoading = $derived(
  paymentsQuery.isLoading ||
    (needsEnrollmentFilter &&
      (feesQuery.isLoading || enrollmentsQuery.isLoading)),
);
const filteredPayments = $derived.by(() => {
  const payments = paymentsQuery.data ?? [];
  const fees = feesQuery.data ?? [];
  const enrollments = enrollmentsQuery.data ?? [];
  const enrollmentIds = new Set(enrollments.map((enrollment) => enrollment.id));
  const feeIds = new Set(
    fees
      .filter(
        (fee) =>
          selectedClass === "all" ||
          (fee.enrollment_id != null && enrollmentIds.has(fee.enrollment_id)),
      )
      .map((fee) => fee.id),
  );
  return needsEnrollmentFilter
    ? payments.filter(
        (payment) =>
          (payment.fee_id != null && feeIds.has(payment.fee_id)) ||
          (payment.enrollment_id != null &&
            enrollmentIds.has(payment.enrollment_id)),
      )
    : payments;
});
const currencies = $derived(
  [
    ...new Set(
      filteredPayments.map((payment) => payment.currency_code).filter(Boolean),
    ),
  ].sort(),
);
let selectedCurrency = $state("");
const currency = $derived(
  currencies.includes(selectedCurrency)
    ? selectedCurrency
    : currencies.includes(orgContext.baseCurrency)
      ? orgContext.baseCurrency
      : (currencies[0] ?? orgContext.baseCurrency),
);
const loadError = $derived(
  paymentsQuery.isError ||
    (needsEnrollmentFilter && (feesQuery.isError || enrollmentsQuery.isError)),
);
const data = $derived.by(() => {
  const buckets = new Map<string, number>();
  for (const payment of filteredPayments) {
    if (payment.currency_code !== currency) continue;
    const paidAt = new Date(payment.paid_at || payment.created_at);
    const amount = Number(payment.amount ?? 0);
    if (Number.isNaN(paidAt.getTime()) || !Number.isFinite(amount)) continue;
    const dateKey = formatDateKey(paidAt);
    const key = dateKey.slice(0, period === "all" ? 4 : 7);
    buckets.set(key, (buckets.get(key) ?? 0) + amount);
  }

  if (period === "all") {
    return [...buckets.keys()].sort().map((year) => ({
      month: year,
      amount: buckets.get(year) ?? 0,
      lastYearAmount: buckets.get(String(Number(year) - 1)) ?? 0,
    }));
  }

  const months = [];
  for (
    const date = new Date(firstMonth);
    date <= currentMonth;
    date.setUTCMonth(date.getUTCMonth() + 1)
  ) {
    const month = String(date.getUTCMonth() + 1).padStart(2, "0");
    const year = date.getUTCFullYear();
    months.push({
      month: formatDate(date, "date", { month: "short", timeZone: "UTC" }),
      amount: buckets.get(`${year}-${month}`) ?? 0,
      lastYearAmount: buckets.get(`${year - 1}-${month}`) ?? 0,
    });
  }
  return months;
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

function formatVal(val: number) {
  return formatNumber(val, { notation: "compact" });
}

function formatTooltipVal(val: number) {
  return formatCurrency(val, currency);
}
</script>

<div class="card-tactile h-full relative group min-h-78.75 p-6 flex flex-col justify-between font-sans border border-border/60 bg-card shadow-sm">
  {#if isLoading}
    <div class="grow w-full relative flex items-center justify-center">
      <Skeleton class="h-full w-full rounded-lg animate-pulse" />
    </div>
  {:else if loadError}
    <p role="alert" class="text-sm text-destructive">{m.finance_ui_payments_load_failed()}</p>
  {:else}
    <!-- Top Row Header + Legend -->
    <div class="flex items-start justify-between w-full select-none mb-3">
      <div class="flex flex-col text-start">
        <span class="text-sm font-semibold text-foreground/90">{m.finance_overview_total_receipts()}</span>
        {#if currencies.length > 1}
          <Select type="single" value={currency} onValueChange={(value) => selectedCurrency = value ?? ""}>
            <SelectTrigger class="mt-2 h-7 w-24 text-xs" aria-label={m.finance_ui_currency()}>{currency}</SelectTrigger>
            <SelectContent>
              {#each currencies as code (code)}
                <SelectItem value={code}>{code}</SelectItem>
              {/each}
            </SelectContent>
          </Select>
        {:else if currency}
          <span class="text-xs text-muted-foreground">{currency}</span>
        {/if}
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
