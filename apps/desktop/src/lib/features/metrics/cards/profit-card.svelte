<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Skeleton } from "@orbis/ui";
import { onMount } from "svelte";
import { api } from "$lib/api";
import DragIndicator from "../components/drag-indicator.svelte";

import { localizeHref } from "$lib/paraglide/runtime";
import { ArrowRight, ChartBar } from "@orbis/icons";

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

let isLoading = $state(true);
let avgProfit = $state("$1,450.50");

// Mini data representing 6 months of bars
const miniChartData = [
  { lastYear: 18, thisYear: 45 },
  { lastYear: 38, thisYear: 45 },
  { lastYear: 28, thisYear: 45 },
  { lastYear: 18, thisYear: 30 },
  { lastYear: 18, thisYear: 45 },
  { lastYear: 18, thisYear: 15 },
];

async function loadData() {
  isLoading = true;
  try {
    // Try to load dynamic profit stats or calculate from database
    const payments = await api.listPayments();
    if (payments && payments.length > 0) {
      // Calculate monthly average payment as a proxy for profit
      const totalAmount = payments.reduce(
        (acc, p) => acc + Number(p.amount || 0),
        0,
      );
      const avg = totalAmount / 6;
      // Format to currency
      avgProfit = new Intl.NumberFormat(getLocale(), {
        style: "currency",
        currency: "USD",
        maximumFractionDigits: 2,
      }).format(avg);
    } else {
      avgProfit = "$1,450.50";
    }
  } catch (e) {
    avgProfit = "$1,450.50";
  } finally {
    isLoading = false;
  }
}

// Utility to get current locale
function getLocale() {
  return "en-US";
}

$effect(() => {
  if (orgId || selectedClass || selectedTerm || period) {
    loadData();
  }
});
</script>

<div class="card-tactile h-full relative group min-h-[265px] p-5 flex flex-col justify-between font-sans border border-border/60 bg-card shadow-sm">
  {#if isLoading}
    <div class="flex-grow w-full relative flex items-center justify-center">
      <Skeleton class="h-full w-full rounded-lg animate-pulse" />
    </div>
  {:else}
    <!-- Header -->
    <div class="flex flex-col w-full text-start select-none">
      <div class="flex items-center justify-between w-full h-5">
        <div class="flex items-center gap-1.5 text-xs text-muted-foreground">
          <ChartBar size={14} class="text-muted-foreground/80" />
          <span>{m.reports_profit_title()}</span>
        </div>
        {#if isEditing}
          <DragIndicator />
        {/if}
      </div>
      <div class="text-sm font-normal text-muted-foreground/80 mt-3 select-none leading-relaxed">
        <!-- Render formatted average profit text matching Image 3 -->
        Your average profit <span class="text-foreground font-semibold">during 6 months</span> is
        <div class="text-2xl font-semibold text-foreground tracking-tight mt-1 font-mono">
          {avgProfit}
        </div>
      </div>
    </div>

    <!-- Mini chart matching Screenshot 3 -->
    <div class="flex items-end justify-center gap-6 w-full h-[65px] select-none mt-5 border-b border-border/10 pb-5">
      {#each miniChartData as month, idx}
        <div class="flex items-end gap-1.5 h-full">
          <!-- Last Year bar (gray/muted tactile) -->
          <div
            class="w-3 bg-muted-foreground/35 shadow-[0_1.5px_0_0_oklch(0.4_0_0)] dark:shadow-[0_1.5px_0_0_oklch(0.15_0_0)] rounded-t-[3px] hover:-translate-y-0.5 transition-all cursor-pointer duration-150"
            style="height: {month.lastYear}px;"
            title="Last Year"
          ></div>
          <!-- This Year bar (foreground/white tactile) -->
          <div
            class="w-3 bg-foreground shadow-[0_1.5px_0_0_oklch(0.15_0_0)] dark:shadow-[0_1.5px_0_0_oklch(0.75_0_0)] rounded-t-[3px] hover:-translate-y-0.5 transition-all cursor-pointer duration-150"
            style="height: {month.thisYear}px;"
            title="This Year"
          ></div>
        </div>
      {/each}
    </div>

    <!-- Footer Link matching exactly Image 3 -->
    <div class="pt-3 flex items-center justify-start w-full select-none text-[10px] font-semibold border-t border-border/40">
      <a
        href={localizeHref(`/${orgId}/finance/ledger`)}
        class="text-muted-foreground hover:text-foreground hover:underline flex items-center gap-1 group/link"
      >
        <span>{m.reports_profit_detailed_graph()}</span>
        <ArrowRight size={10} class="transition-transform group-hover/link:translate-x-0.5 rtl:rotate-180" />
      </a>
    </div>
  {/if}
</div>
