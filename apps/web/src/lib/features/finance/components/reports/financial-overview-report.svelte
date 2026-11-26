<!--
  @component FinancialOverviewReport
  Executive financial dashboard displaying high-level KPIs, solvency structure, and system health.
  Styled in the clean, headerless, elegant FinanceOverview aesthetic.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { createLedgerReport } from "./report-data.svelte";
import { extractErrorMessage } from "$lib/api";
import { formatReportAmount } from "./report-view";
import { balancesMatch } from "./ledger-report";
import {
  Card,
  CardHeader,
  CardTitle,
  CardContent,
  CardFooter,
} from "@orbis/ui/card";
import { MetricProgress } from "@orbis/ui";
import { FinanceSkeleton } from "$lib/shared/components/skeletons";
import ErrorState from "$lib/shared/components/error-state.svelte";
import {
  ArrowRight,
  Clock,
  CheckCircle,
  InfoCircle,
} from "@orbis/icons";
import { localizeHref } from "$lib/paraglide/runtime";

let { orgId }: { orgId: string } = $props();

const report = createLedgerReport(() => orgId);
const { accounts, activeCurrencies, baseCurrency, summaries, isLoading, queryError } = $derived(report);
const { refetchAll } = report;

const overviewMetrics = $derived.by(() => {
  const currencyCode = baseCurrency;
  const summary = summaries.get(currencyCode)!;
  return {
    currencyCode,
    assets: summary.asset,
    liabilities: summary.liability,
    equity: summary.equity + summary.netIncome,
    revenue: summary.revenue,
    expenses: summary.expense,
    netIncome: summary.netIncome,
    isBalanced: activeCurrencies.every(({ code }) => {
      const ledger = summaries.get(code)!;
      return balancesMatch(ledger.asset, ledger.liabilitiesAndEquity, code);
    }),
  };
});

</script>

<div class="h-full w-full overflow-y-auto bg-background/50 scrollbar-hide p-6 flex flex-col gap-6 font-sans">
  {#if isLoading}
    <FinanceSkeleton />
  {:else if queryError}
    <ErrorState
      title={m.finance_report_load_error()}
      message={extractErrorMessage(queryError)}
      onRetry={refetchAll}
    />
  {:else}
    <div class="flex flex-col gap-6">
      <!-- 1. Top 4 KPI Metrics Cards -->
      <div class="grid grid-cols-2 lg:grid-cols-4 gap-4">
        <Card class="p-4 shadow-sm hover:border-primary/40 transition-colors">
          <MetricProgress variant="single" value={overviewMetrics.assets} showProgress={false} tone="primary">
            {#snippet title()}{m.finance_ui_total_assets()}{/snippet}
            {#snippet metadata()}{formatReportAmount(overviewMetrics.assets, overviewMetrics.currencyCode)}{/snippet}
          </MetricProgress>
        </Card>

        <Card class="p-4 shadow-sm hover:border-warning/40 transition-colors">
          <MetricProgress variant="single" value={overviewMetrics.liabilities} showProgress={false} tone="warning">
            {#snippet title()}{m.finance_ui_total_liabilities()}{/snippet}
            {#snippet metadata()}{formatReportAmount(overviewMetrics.liabilities, overviewMetrics.currencyCode)}{/snippet}
          </MetricProgress>
        </Card>

        <Card class="p-4 shadow-sm hover:border-success/40 transition-colors">
          <MetricProgress variant="single" value={overviewMetrics.equity} showProgress={false} tone="success">
            {#snippet title()}{m.finance_ui_equity()}{/snippet}
            {#snippet metadata()}{formatReportAmount(overviewMetrics.equity, overviewMetrics.currencyCode)}{/snippet}
          </MetricProgress>
        </Card>

        <Card class="p-4 shadow-sm hover:border-emerald-500/40 transition-colors">
          <MetricProgress
            variant="single"
            value={overviewMetrics.netIncome}
            showProgress={false}
            tone={overviewMetrics.netIncome >= 0 ? "success" : "warning"}
          >
            {#snippet title()}{m.finance_ui_net_profit_surplus()}{/snippet}
            {#snippet metadata()}{formatReportAmount(overviewMetrics.netIncome, overviewMetrics.currencyCode)}{/snippet}
          </MetricProgress>
        </Card>
      </div>

      <!-- 2. Middle Section: Solvency Breakdown & Accounting Status -->
      <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
        <!-- Structure & Solvency Card -->
        <Card class="flex flex-col justify-between text-start relative overflow-hidden shadow-sm lg:col-span-2 min-h-[260px]">
          <CardHeader class="py-3 px-4">
            <div class="flex items-center justify-between">
              <div class="flex items-center gap-2">
                <span class="h-2 w-2 rounded-full bg-primary"></span>
                <CardTitle class="text-xs font-bold uppercase tracking-wider text-foreground">
                  {m.finance_ui_financial_position_solvency()}
                </CardTitle>
              </div>
              {#if overviewMetrics.isBalanced}
                <span class="inline-flex items-center gap-1 text-[10px] font-medium text-emerald-500 bg-emerald-500/10 px-2 py-0.5 rounded-md border border-emerald-500/20">
                  <CheckCircle size={11} />
                  <span>{m.finance_ui_balanced_feminine()}</span>
                </span>
              {:else}
                <span class="inline-flex items-center gap-1 text-[10px] font-medium text-amber-500 bg-amber-500/10 px-2 py-0.5 rounded-md border border-amber-500/20">
                  <InfoCircle size={11} />
                  <span>{m.finance_ui_adjustment_difference()}</span>
                </span>
              {/if}
            </div>
          </CardHeader>

          <CardContent class="p-5 flex flex-col justify-center flex-grow space-y-4">
            <MetricProgress
              variant="segmented"
              segments={[
                { label: m.finance_ui_assets(), value: Math.max(0, overviewMetrics.assets), tone: "primary", displayValue: formatReportAmount(overviewMetrics.assets, overviewMetrics.currencyCode) },
                { label: m.finance_ui_liabilities(), value: Math.max(0, overviewMetrics.liabilities), tone: "warning", displayValue: formatReportAmount(overviewMetrics.liabilities, overviewMetrics.currencyCode) },
                { label: m.finance_ui_equity(), value: Math.max(0, overviewMetrics.equity), tone: "success", displayValue: formatReportAmount(overviewMetrics.equity, overviewMetrics.currencyCode) },
              ]}
            >
              {#snippet title()}{m.finance_ui_balance_sheet_equation()}{/snippet}
              {#snippet metadata()}{overviewMetrics.currencyCode}{/snippet}
            </MetricProgress>
          </CardContent>

          <CardFooter class="border-t border-border/40 py-2.5 px-4 text-[10px]">
            <a href={localizeHref(`/${orgId}/finance/reports/balance-sheet`)} class="flex items-center gap-1.5 hover:underline text-primary font-bold group/link">
              <span>{m.finance_ui_view_detailed_balance_sheet()}</span>
              <ArrowRight size={10} class="transition-transform group-hover/link:translate-x-0.5 rtl:rotate-180" />
            </a>
          </CardFooter>
        </Card>

        <!-- Accounting Status Card -->
        <Card class="flex flex-col justify-between text-start relative overflow-hidden shadow-sm min-h-[260px]">
          <CardHeader class="py-3 px-4">
            <div class="flex items-center gap-2">
              <Clock size={13} class="text-muted-foreground" />
              <CardTitle class="text-xs font-bold uppercase tracking-wider text-foreground">
                {m.finance_ui_accounting_system_status()}
              </CardTitle>
            </div>
          </CardHeader>

          <CardContent class="p-4 flex flex-col gap-2.5 w-full grow justify-center text-xs">
            <div class="flex items-center justify-between"><span class="text-muted-foreground">{m.finance_ui_chart_of_accounts()}</span><strong class="font-mono">{accounts.length}</strong></div>
            <div class="flex items-center justify-between"><span class="text-muted-foreground">{m.finance_ui_defined_currencies()}</span><strong class="font-mono">{activeCurrencies.length}</strong></div>
            <div class="flex items-center justify-between"><span class="text-muted-foreground">{m.finance_ui_base_currency()}</span><strong class="font-mono text-primary font-bold">{overviewMetrics.currencyCode}</strong></div>
            <div class="flex items-center justify-between border-t border-border/40 pt-2"><span class="text-muted-foreground">{m.finance_ui_total_revenue()}</span><strong class="font-mono text-emerald-600 dark:text-emerald-400">{formatReportAmount(overviewMetrics.revenue, overviewMetrics.currencyCode)}</strong></div>
            <div class="flex items-center justify-between"><span class="text-muted-foreground">{m.finance_ui_total_expenses()}</span><strong class="font-mono text-rose-600 dark:text-rose-400">{formatReportAmount(overviewMetrics.expenses, overviewMetrics.currencyCode)}</strong></div>
          </CardContent>

          <CardFooter class="border-t border-border/40 py-2.5 px-4 text-[10px]">
            <a href={localizeHref(`/${orgId}/finance/accounts`)} class="flex items-center gap-1.5 hover:underline text-primary font-bold group/link">
              <span>{m.finance_ui_manage_chart_of_accounts()}</span>
              <ArrowRight size={10} class="transition-transform group-hover/link:translate-x-0.5 rtl:rotate-180" />
            </a>
          </CardFooter>
        </Card>
      </div>

    </div>
  {/if}
</div>
