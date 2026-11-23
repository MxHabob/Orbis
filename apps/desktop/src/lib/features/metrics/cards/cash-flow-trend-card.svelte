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
import { api, type FeeTemplateDto } from "$lib/api";
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
  feeTemplates,
  isEditing = false,
}: {
  orgId: string;
  dateRange: { fromDate: string; toDate: string };
  branchId?: string;
  selectedCurrency: string;
  feeTemplates: FeeTemplateDto[];
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
    const filter = {
      from_date: dateRange.fromDate,
      to_date: dateRange.toDate,
      currency_code: selectedCurrency || undefined,
      ...(branchId ? { org_unit_id: branchId } : {}),
    };
    const [paymentsList, feesList] = await Promise.all([
      api.listPayments(filter),
      api.listStudentFees(filter),
    ]);

    if (sequence !== loadSequence) return;

    const templatesById = new Map(feeTemplates.map((template) => [template.id, template]));
    const filteredFees = feesList.filter((fee) => {
      return (
        !selectedCurrency ||
        templatesById.get(fee.fee_template_id)?.currency ===
          selectedCurrency
      );
    });
    const filteredPayments = paymentsList.filter(
      (payment) =>
        !selectedCurrency || payment.currency_code === selectedCurrency,
    );

    const startDate = new Date(`${dateRange.fromDate}T00:00:00Z`);
    const endDate = new Date(`${dateRange.toDate}T00:00:00Z`);
    if (
      Number.isNaN(startDate.getTime()) ||
      Number.isNaN(endDate.getTime()) ||
      startDate > endDate
    ) {
      throw new Error("Invalid finance overview date range");
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
                year: monthCount > 18 ? "2-digit" : undefined,
                timeZone: "UTC",
              })
            : String(current.getUTCFullYear()),
        invoiced: 0,
        collected: 0,
      });
    }
    const monthsByKey = new Map(
      chartBuckets.map((month) => [month.key, month]),
    );

    for (const fee of filteredFees) {
      const bucketKey = getBucketKey(fee.due_date, bucketUnit);
      if (!bucketKey) continue;
      const bucket = monthsByKey.get(bucketKey);
      if (bucket) bucket.invoiced += toAmount(fee.net_amount);
    }

    for (const payment of filteredPayments) {
      const bucketKey = getBucketKey(payment.paid_at, bucketUnit);
      if (!bucketKey) continue;
      const bucket = monthsByKey.get(bucketKey);
      if (bucket) bucket.collected += toAmount(payment.amount);
    }

    data = chartBuckets.map(({ label, invoiced, collected }) => ({
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
  void dateRange.fromDate;
  void dateRange.toDate;
  void branchId;
  void selectedCurrency;
  void feeTemplates;
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

function handleMouseMove(e: MouseEvent & { currentTarget: HTMLDivElement }) {
  if (!hasRecords || chartPoints.length === 0) return;
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
          {#each [0, 1, 2, 3, 4] as i (i)}
            {@const tickVal = maxVal - (i / 4) * maxVal}
            {@const y = 22 + i * 43.5}
            <text
              x="62"
              y={y + 3}
              text-anchor="end"
              class="fill-muted-foreground font-mono text-[10px]"
            >{formatVal(tickVal)}</text>
            <line
              x1="74"
              x2="990"
              y1={y}
              y2={y}
              class="stroke-border"
              stroke-dasharray="3 5"
              stroke-width="1"
            />
          {/each}
          {#if chartPoints.length > 1}
            <polyline
              points={invoicedLine}
              fill="none"
              class="stroke-muted-foreground"
              stroke-width="3"
              stroke-linecap="round"
              stroke-linejoin="round"
            />
            <polyline
              points={collectedLine}
              fill="none"
              class="stroke-emerald-600 dark:stroke-emerald-400"
              stroke-width="3"
              stroke-linecap="round"
              stroke-linejoin="round"
            />
          {/if}
          {#each chartPoints as point, idx (point.label)}
            <circle
              cx={point.x}
              cy={point.invoicedY}
              r={hoveredIdx === idx ? 5 : 3}
              class="fill-background stroke-muted-foreground"
              stroke-width="2"
            />
            <circle
              cx={point.x}
              cy={point.collectedY}
              r={hoveredIdx === idx ? 5 : 3}
              class="fill-background stroke-emerald-600 dark:stroke-emerald-400"
              stroke-width="2"
            />
          {/each}
        </svg>
        <div
          class="grid text-center text-[9px] font-mono font-bold text-muted-foreground"
          style={`grid-template-columns: repeat(${chartPoints.length}, minmax(0, 1fr))`}
        >
          {#each chartPoints as point (point.label)}
            <span class="truncate px-0.5">{point.label}</span>
          {/each}
        </div>

        <!-- Floating Tooltip Box -->
        {#if hoveredIdx !== null && tooltipPos && hasRecords}
          {@const activeBar = chartPoints[hoveredIdx]}
          <div 
            class="absolute z-30 w-[220px] max-w-[calc(100%-16px)] p-2.5 text-[10px] font-sans border bg-card border-border text-foreground rounded-lg shadow-xl pointer-events-none flex flex-col gap-1 text-start"
            style="left: {tooltipPos.x}px; top: {tooltipPos.y}px;"
          >
            <span class="text-muted-foreground font-bold border-b border-border/40 pb-0.5 mb-1">{activeBar.label}</span>
            <div class="flex flex-col gap-0.5">
              <div class="flex items-center justify-between gap-4">
                <span class="text-muted-foreground">{m.finance_overview_trend_invoiced()}:</span>
                <span class="font-bold font-mono text-foreground">{formatCurrency(activeBar.invoiced, selectedCurrency)}</span>
              </div>
              <div class="flex items-center justify-between gap-4">
                <span class="text-muted-foreground">{m.finance_overview_trend_collected()}:</span>
                <span class="font-bold font-mono text-foreground">{formatCurrency(activeBar.collected, selectedCurrency)}</span>
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
        <a href={resolve(localizeHref(`/${orgId}/finance/statement`) as Pathname)} class="text-primary hover:underline flex items-center gap-1 group/statement">
          <span>{m.finance_overview_btn_statement()}</span>
          <ArrowRight size={10} class="transition-transform group-hover/statement:translate-x-0.5 rtl:rotate-180" />
        </a>
      </div>
    </CardFooter>
  {/if}
</Card>
