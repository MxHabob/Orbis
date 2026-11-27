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
import type { PaymentDto } from "$lib/api";
import { createStudentFeesQuery } from "$lib/features/finance/queries";
import { formatDateKey } from "$lib/core/i18n/format";
import DragIndicator from "../components/drag-indicator.svelte";

import { resolve } from "$app/paths";
import type { Pathname } from "$app/types";
import { localizeHref } from "$lib/paraglide/runtime";
import { formatCurrency, formatDate } from "$lib/core/i18n";
import { ArrowRight } from "@orbis/icons";

let {
  orgId,
  dateRange,
  branchId,
  selectedCurrency,
  payments,
  isEditing = false,
}: {
  orgId: string;
  dateRange: { fromDate: string; toDate: string };
  branchId?: string;
  selectedCurrency: string;
  payments: PaymentDto[];
  isEditing?: boolean;
} = $props();

const feesQuery = createStudentFeesQuery(
  () => orgId,
  () => ({
    from_date: dateRange.fromDate,
    to_date: dateRange.toDate,
    currency_code: selectedCurrency,
    org_unit_id: branchId,
  }),
  () => Boolean(selectedCurrency),
);
const isLoading = $derived(feesQuery.isLoading);
const loadError = $derived(Boolean(feesQuery.error));

const data = $derived.by(() => {
  if (!selectedCurrency) return [];
  const startDate = new Date(`${dateRange.fromDate}T00:00:00Z`);
  const endDate = new Date(`${dateRange.toDate}T00:00:00Z`);
  if (
    Number.isNaN(startDate.getTime()) ||
    Number.isNaN(endDate.getTime()) ||
    startDate > endDate
  ) {
    return [];
  }

  const monthCount =
    (endDate.getUTCFullYear() - startDate.getUTCFullYear()) * 12 +
    endDate.getUTCMonth() -
    startDate.getUTCMonth() +
    1;
  const bucketUnit = monthCount > 36 ? "year" : "month";
  const firstBucket = new Date(
    Date.UTC(
      startDate.getUTCFullYear(),
      bucketUnit === "month" ? startDate.getUTCMonth() : 0,
      1,
    ),
  );
  const lastBucket = new Date(
    Date.UTC(
      endDate.getUTCFullYear(),
      bucketUnit === "month" ? endDate.getUTCMonth() : 0,
      1,
    ),
  );
  const chartBuckets: {
    key: string;
    label: string;
    invoiced: number;
    collected: number;
  }[] = [];
  for (
    const current = new Date(firstBucket);
    current <= lastBucket;
    current.setUTCMonth(
      current.getUTCMonth() + (bucketUnit === "month" ? 1 : 12),
    )
  ) {
    const key =
      bucketUnit === "month"
        ? `${current.getUTCFullYear()}-${String(current.getUTCMonth() + 1).padStart(2, "0")}`
        : String(current.getUTCFullYear());
    chartBuckets.push({
      key,
      label:
        bucketUnit === "month"
          ? formatDate(current, "date", {
              month: "short",
              year: monthCount > 12 ? "2-digit" : undefined,
              timeZone: "UTC",
            })
          : String(current.getUTCFullYear()),
      invoiced: 0,
      collected: 0,
    });
  }
  const monthsByKey = new Map(chartBuckets.map((month) => [month.key, month]));

  for (const fee of feesQuery.data ?? []) {
    const bucketKey = getBucketKey(fee.due_date, bucketUnit);
    if (!bucketKey) continue;
    const bucket = monthsByKey.get(bucketKey);
    if (bucket) bucket.invoiced += toAmount(fee.net_amount);
  }

  for (const payment of payments) {
    const paidAt = new Date(payment.paid_at || payment.created_at);
    const bucketKey = Number.isNaN(paidAt.getTime())
      ? undefined
      : formatDateKey(paidAt).slice(0, bucketUnit === "year" ? 4 : 7);
    if (!bucketKey) continue;
    const bucket = monthsByKey.get(bucketKey);
    if (bucket) bucket.collected += toAmount(payment.amount);
  }

  return chartBuckets;
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

const chartPoints = $derived.by(() =>
  displayData.map((item, index) => {
    const x =
      displayData.length <= 1
        ? 530
        : 74 + (index / (displayData.length - 1)) * 912;
    return {
      ...item,
      x,
      invoicedY: 196 - (item.invoiced / maxVal) * 174,
      collectedY: 196 - (item.collected / maxVal) * 174,
    };
  }),
);
const invoicedLine = $derived(
  chartPoints.map((point) => `${point.x},${point.invoicedY}`).join(" "),
);
const collectedLine = $derived(
  chartPoints.map((point) => `${point.x},${point.collectedY}`).join(" "),
);

let hoveredIdx = $state<number | null>(null);
let tooltipPos = $state<{ x: number; y: number } | null>(null);
const hoveredPoint = $derived(
  hoveredIdx === null ? undefined : chartPoints[hoveredIdx],
);
const highlightedPoint = $derived(
  hoveredPoint ?? (chartPoints.length === 1 ? chartPoints[0] : undefined),
);

function handleMouseMove(e: MouseEvent & { currentTarget: HTMLDivElement }) {
  if (!hasRecords || chartPoints.length === 0) return;
  const rect = e.currentTarget.getBoundingClientRect();
  const mouseX = e.clientX - rect.left;
  const mouseY = e.clientY - rect.top;
  const axisWidth = rect.width * 0.074;
  const plotWidth = Math.max(1, rect.width * 0.912);
  const plotX = mouseX - axisWidth;
  const colWidth = plotWidth / Math.max(1, displayData.length - 1);
  const idx = Math.max(
    0,
    Math.min(displayData.length - 1, Math.round(plotX / colWidth)),
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

function getBucketKey(
  value: string | null | undefined,
  unit: "month" | "year",
): string | undefined {
  if (!value) return undefined;
  const date = new Date(value);
  if (Number.isNaN(date.getTime())) return undefined;
  if (unit === "year") return String(date.getUTCFullYear());
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
    <CardHeader class="py-3 px-4">
      <div class="flex items-center gap-2">
        <CardTitle class="text-sm font-semibold tracking-tight text-foreground">
          {m.finance_overview_collection_trend()}
        </CardTitle>
        {#if isEditing}
          <DragIndicator />
        {/if}
      </div>

      <CardAction>
        <div class="flex items-center gap-4 text-[11px] text-muted-foreground font-medium select-none">
          <div class="flex items-center gap-1.5">
            <span class="h-px w-4 border-t border-dashed border-muted-foreground/70"></span>
            <span>{m.finance_overview_trend_invoiced()}</span>
          </div>
          <div class="flex items-center gap-1.5">
            <span class="h-0.5 w-4 rounded-full bg-primary"></span>
            <span>{m.finance_overview_total_receipts()}</span>
          </div>
        </div>
      </CardAction>
    </CardHeader>

    <CardContent class="p-4 pt-2 flex-grow flex flex-col justify-between">
      <div 
        class="flex-grow relative w-full select-none"
        role="img"
        aria-label={m.finance_overview_collection_trend()}
        onmouseleave={handleMouseLeave}
        onmousemove={handleMouseMove}
      >
        <svg
          viewBox="0 0 1000 220"
          preserveAspectRatio="none"
          class="h-[220px] w-full cursor-crosshair overflow-visible"
          aria-hidden="true"
        >
          <defs>
            <linearGradient id="finance-collection-area" x1="0" x2="0" y1="0" y2="1">
              <stop offset="0%" stop-color="var(--primary)" stop-opacity="0.14" />
              <stop offset="100%" stop-color="var(--primary)" stop-opacity="0" />
            </linearGradient>
          </defs>
          {#each [0, 1, 2, 3, 4] as i (i)}
            {@const tickVal = maxVal - (i / 4) * maxVal}
            {@const y = 22 + i * 43.5}
            <text
              x="62"
              y={y + 3}
              text-anchor="end"
              fill="var(--muted-foreground)"
              class="font-mono text-[10px]"
            >{formatVal(tickVal)}</text>
            <line
              x1="74"
              x2="990"
              y1={y}
              y2={y}
              stroke="var(--border)"
              opacity="0.6"
              stroke-dasharray="3 5"
              stroke-width="1"
            />
          {/each}
          {#if chartPoints.length > 1}
            {#if hoveredPoint}
              <line
                x1={hoveredPoint.x}
                x2={hoveredPoint.x}
                y1="22"
                y2="196"
                stroke="var(--border)"
                stroke-width="1"
              />
            {/if}
            <polygon
              points={`${collectedLine} 986,196 74,196`}
              fill="url(#finance-collection-area)"
            />
            <polyline
              points={invoicedLine}
              fill="none"
              stroke="var(--muted-foreground)"
              opacity="0.7"
              stroke-dasharray="5 5"
              stroke-width="1.75"
              stroke-linecap="round"
              stroke-linejoin="round"
            />
            <polyline
              points={collectedLine}
              fill="none"
              stroke="var(--primary)"
              stroke-width="2.5"
              stroke-linecap="round"
              stroke-linejoin="round"
            />
          {/if}
          {#if highlightedPoint && hasRecords}
            {@const point = highlightedPoint}
            <circle
              cx={point.x}
              cy={point.invoicedY}
              r="4"
              fill="var(--background)"
              stroke="var(--muted-foreground)"
              stroke-width="2"
            />
            <circle
              cx={point.x}
              cy={point.collectedY}
              r="4"
              fill="var(--primary)"
              stroke="var(--background)"
              stroke-width="2"
            />
          {/if}
        </svg>
        <div class="relative h-5 text-[9px] font-medium text-muted-foreground">
          {#each chartPoints as point (point.key)}
            <span
              dir="auto"
              class="absolute -translate-x-1/2 whitespace-nowrap"
              style:left={`clamp(1.5rem, ${point.x / 10}%, calc(100% - 1.5rem))`}
            >{point.label}</span>
          {/each}
        </div>

        <!-- Floating Tooltip Box -->
        {#if hoveredPoint && tooltipPos && hasRecords}
          {@const activeBar = hoveredPoint}
          <div 
            class="absolute z-30 w-[220px] max-w-[calc(100%-16px)] rounded-lg border border-border bg-popover p-3 text-start text-xs text-popover-foreground shadow-lg pointer-events-none flex flex-col gap-2"
            style="left: {tooltipPos.x}px; top: {tooltipPos.y}px;"
          >
            <span class="border-b border-border/60 pb-1 font-medium text-muted-foreground">{activeBar.label}</span>
            <div class="flex flex-col gap-1">
              <div class="flex items-center justify-between gap-4">
                <span class="text-muted-foreground">{m.finance_overview_trend_invoiced()}:</span>
                <span class="font-bold font-mono text-foreground">{formatCurrency(activeBar.invoiced, selectedCurrency)}</span>
              </div>
              <div class="flex items-center justify-between gap-4">
                <span class="text-muted-foreground">{m.finance_overview_total_receipts()}:</span>
                <span class="font-bold font-mono text-foreground">{formatCurrency(activeBar.collected, selectedCurrency)}</span>
              </div>
            </div>
          </div>
        {/if}

        <!-- Backdrop Blur Overlay -->
        {#if loadError}
          <div class="absolute inset-0 flex flex-col items-center justify-center text-center p-4 gap-2 bg-background/90 z-20 rounded-xl" role="alert">
            <span class="font-bold text-xs text-foreground/90">{m.finance_overview_error()}</span>
            <button type="button" class="text-[10px] font-semibold text-primary hover:underline" onclick={() => feesQuery.refetch()}>
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
        <a href={resolve(localizeHref(`/${orgId}/finance/statement`) as Pathname)} class="text-primary hover:underline flex items-center gap-1 group/statement">
          <span>{m.finance_overview_btn_statement()}</span>
          <ArrowRight size={10} class="transition-transform group-hover/statement:translate-x-0.5 rtl:rotate-180" />
        </a>
      </div>
    </CardFooter>
  {/if}
</Card>
