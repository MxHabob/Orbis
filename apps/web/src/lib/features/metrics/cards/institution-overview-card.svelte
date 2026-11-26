<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Skeleton } from "@orbis/ui";
import { Card, CardContent } from "@orbis/ui/card";
import { createQuery } from "@tanstack/svelte-query";
import { api, queryKeys } from "$lib/api";
import { formatCurrency, formatNumber } from "$lib/core/i18n";
import { orgContext } from "$lib/stores/org-context.svelte";
import DragIndicator from "../components/drag-indicator.svelte";

let {
  orgId,
  isEditing = false,
}: {
  orgId: string;
  isEditing?: boolean;
} = $props();

const dashboardQuery = createQuery(() => ({
  queryKey: [...queryKeys.org.dashboard(orgId), "metrics-overview"],
  queryFn: () => api.getDashboard(),
  enabled: !!orgId,
  staleTime: 60_000,
}));

const stats = $derived(dashboardQuery.data);
const currency = $derived(orgContext.baseCurrency);

const enrollmentPct = $derived(
  stats ? Math.min(100, Math.max(0, Math.round(stats.enrollment_rate))) : 0,
);
const teacherPct = $derived(
  stats && stats.total_employees > 0
    ? Math.min(100, Math.round((stats.total_teachers / stats.total_employees) * 100))
    : 0,
);
const feeTotal = $derived(
  stats ? stats.total_payments_amount + stats.pending_fees_amount : 0,
);
const collectionPct = $derived(
  stats && feeTotal > 0
    ? Math.min(100, Math.round((stats.total_payments_amount / feeTotal) * 100))
    : 0,
);
</script>

<Card class="h-full border-border/60 bg-card shadow-xs">
  <CardContent class="flex flex-col gap-4 p-5 text-start">
    <div class="flex items-center justify-between gap-2">
      <div class="flex flex-col gap-0.5">
        <span class="text-xs font-medium text-muted-foreground">
          {m.common_nav_dashboard()}
        </span>
        {#if stats?.active_academic_calendar}
          <span class="font-mono text-[11px] text-muted-foreground/80">
            {stats.active_academic_calendar}
          </span>
        {/if}
      </div>
      {#if isEditing}
        <DragIndicator />
      {/if}
    </div>

    {#if dashboardQuery.isLoading}
      <div class="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-4">
        {#each Array(4) as _, i (i)}
          <Skeleton class="h-20 w-full rounded-md" />
        {/each}
      </div>
    {:else if stats}
      <div class="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-4">
        <!-- Enrollment Rate -->
        <div class="flex flex-col justify-between gap-2 rounded-lg border border-border/50 bg-muted/25 p-3.5">
          <div class="flex items-center justify-between gap-2">
            <span class="text-xs text-muted-foreground">{m.common_classes_enrolled_students()}</span>
            <span class="font-mono text-xs font-semibold text-foreground">{enrollmentPct}%</span>
          </div>
          <div class="text-xl font-bold tabular-nums text-foreground">
            {formatNumber(stats.enrolled_students)}
            <span class="text-xs font-normal text-muted-foreground">/ {formatNumber(stats.total_students)}</span>
          </div>
          <div class="h-1.5 w-full overflow-hidden rounded-full bg-muted">
            <div class="h-full rounded-full bg-chart-1 transition-all duration-500" style="width: {enrollmentPct}%"></div>
          </div>
        </div>

        <!-- Teachers vs Staff -->
        <div class="flex flex-col justify-between gap-2 rounded-lg border border-border/50 bg-muted/25 p-3.5">
          <div class="flex items-center justify-between gap-2">
            <span class="text-xs text-muted-foreground">{m.operations_ui_report_category_hr()}</span>
            <span class="font-mono text-xs font-semibold text-foreground">{teacherPct}%</span>
          </div>
          <div class="text-xl font-bold tabular-nums text-foreground">
            {formatNumber(stats.total_teachers)}
            <span class="text-xs font-normal text-muted-foreground">/ {formatNumber(stats.total_employees)}</span>
          </div>
          <div class="h-1.5 w-full overflow-hidden rounded-full bg-muted">
            <div class="h-full rounded-full bg-chart-2 transition-all duration-500" style="width: {teacherPct}%"></div>
          </div>
        </div>

        <!-- Active Sections & Subjects -->
        <div class="flex flex-col justify-between gap-2 rounded-lg border border-border/50 bg-muted/25 p-3.5">
          <span class="text-xs text-muted-foreground">{m.common_card_active_classes()}</span>
          <div class="text-xl font-bold tabular-nums text-foreground">
            {formatNumber(stats.active_sections)}
            <span class="text-xs font-normal text-muted-foreground">· {formatNumber(stats.total_subjects)} {m.common_search_group_subjects()}</span>
          </div>
          <div class="h-1.5 w-full overflow-hidden rounded-full bg-muted">
            <div class="h-full w-full rounded-full bg-chart-3/70"></div>
          </div>
        </div>

        <!-- Fee Collection -->
        <div class="flex flex-col justify-between gap-2 rounded-lg border border-border/50 bg-muted/25 p-3.5">
          <div class="flex items-center justify-between gap-2">
            <span class="text-xs text-muted-foreground">{m.license_billing_view_payments()}</span>
            <span class="font-mono text-xs font-semibold text-foreground">{collectionPct}%</span>
          </div>
          <div class="truncate text-lg font-bold tabular-nums text-foreground">
            {currency
              ? formatCurrency(stats.total_payments_amount, currency)
              : formatNumber(stats.total_payments_amount)}
          </div>
          <div class="h-1.5 w-full overflow-hidden rounded-full bg-muted">
            <div class="h-full rounded-full bg-chart-4 transition-all duration-500" style="width: {collectionPct}%"></div>
          </div>
        </div>
      </div>
    {/if}
  </CardContent>
</Card>
