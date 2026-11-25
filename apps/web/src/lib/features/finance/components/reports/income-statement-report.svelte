<!--
  @component IncomeStatementReport
  Income Statement (قائمة الدخل / بيان الأرباح والخسائر) reporting grid.
  Sleek, headerless layout matching FinanceOverview.
-->
<script lang="ts">
import { LoadingIndicator } from "@orbis/ui";
import * as m from "$lib/paraglide/messages";
import { createLedgerReport } from "./report-data.svelte";
import { extractErrorMessage } from "$lib/api";
import { exportReport, formatReportAmount, searchReportRows, statementRows, statementTotal } from "./report-view";
import { OrbisGrid, type OrbisColumnDef } from "@orbis/svelte-grid";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { FinanceSkeleton } from "$lib/shared/components/skeletons";
import ErrorState from "$lib/shared/components/error-state.svelte";
import { Download, Refresh, Search } from "@orbis/icons";
import { getLocale } from "$lib/paraglide/runtime";

let { orgId }: { orgId: string } = $props();

let searchQuery = $state("");

const report = createLedgerReport(() => orgId);
const { accounts, activeCurrencies, baseCurrency, summaries, isLoading, queryError } = $derived(report);
const { refetchAll } = report;

const summaryKpis = $derived.by(() => {
  const code = baseCurrency;
  const summary = summaries.get(code)!;
  return { code, revenue: summary.revenue, expense: summary.expense, netIncome: summary.netIncome };
});

const statementColumns = $derived.by((): OrbisColumnDef[] => [
  {
    id: "__idx",
    prop: "__idx",
    header: "#",
    name: "#",
    width: 48,
    size: 48,
    readonly: true,
    align: "center",
  },
  {
    id: "section",
    prop: "section",
    header: m.finance_ui_section_tab(),
    name: m.finance_ui_section_tab(),
    width: 170,
    size: 170,
    readonly: true,
    cellClass: "font-bold text-primary",
  },
  {
    id: "account_code",
    prop: "account_code",
    header: m.finance_ui_account_code(),
    name: m.finance_ui_account_code(),
    width: 100,
    size: 100,
    readonly: true,
    cellClass: "font-mono",
  },
  {
    id: "account_name",
    pin: "colPinStart",
    prop: "account_name",
    header: m.finance_ui_account_name(),
    name: m.finance_ui_account_name(),
    width: 260,
    size: 260,
    readonly: true,
    cellClass: "font-medium",
  },
  ...activeCurrencies.map((curr) => ({
    id: curr.code,
    prop: curr.code,
    type: "currency" as const,
    currency: curr.code,
    header: `${curr.name || curr.code} (${curr.code})`,
    name: curr.code,
    width: 150,
    size: 150,
    readonly: true,
    align: "end" as const,
    cellClass: "font-mono font-bold",
  })),
]);

const incomeStatementGroups = [
  [m.finance_ui_revenue_sales(), "revenue"],
  [m.finance_ui_operating_expenses_costs(), "expense"],
] as const;

const incomeStatementSource = $derived.by(() => {
  const rows = statementRows(accounts, incomeStatementGroups, summaries);
  if (!rows.length) return rows;
  rows.push(statementTotal(m.finance_ui_net_income_label(), summaries, (summary) => summary.netIncome));
  return rows;
});

const visibleRows = $derived(searchReportRows(incomeStatementSource, searchQuery));

function exportCurrentReport() {
  exportReport("income_statement", visibleRows, statementColumns);
}

const rtl = $derived(getLocale() === "ar");
</script>

<div class="h-full w-full overflow-hidden bg-background/50 p-6 flex flex-col gap-4 font-sans select-none">
  <!-- Minimal Toolbar Row -->
  <div class="flex flex-wrap items-center justify-between gap-3 shrink-0">
    <!-- Quick Status Badges -->
    <div class="flex items-center gap-2 flex-wrap text-xs">
      <div class="flex items-center gap-1.5 px-2.5 py-1 rounded-lg bg-emerald-500/10 border border-emerald-500/20 text-emerald-600 dark:text-emerald-400 font-medium">
        <span class="text-muted-foreground text-[11px]">{m.finance_ui_revenue_label()}</span>
        <strong class="font-mono">{formatReportAmount(summaryKpis.revenue, summaryKpis.code)}</strong>
      </div>

      <div class="flex items-center gap-1.5 px-2.5 py-1 rounded-lg bg-rose-500/10 border border-rose-500/20 text-rose-600 dark:text-rose-400 font-medium">
        <span class="text-muted-foreground text-[11px]">{m.finance_ui_expenses_label()}</span>
        <strong class="font-mono">{formatReportAmount(summaryKpis.expense, summaryKpis.code)}</strong>
      </div>

      <div class={`flex items-center gap-1.5 px-2.5 py-1 rounded-lg border font-medium ${
        summaryKpis.netIncome >= 0
          ? "bg-emerald-500/10 border-emerald-500/20 text-emerald-600 dark:text-emerald-400"
          : "bg-rose-500/10 border-rose-500/20 text-rose-600 dark:text-rose-400"
      }`}>
        <span class="text-muted-foreground text-[11px]">{m.finance_ui_net_income_label()}</span>
        <strong class="font-mono">{formatReportAmount(summaryKpis.netIncome, summaryKpis.code)}</strong>
      </div>
    </div>

    <!-- Actions & Search -->
    <div class="flex items-center gap-2">
      <div class="relative w-48 sm:w-60">
        <Search size={14} class="absolute start-2.5 top-1/2 -translate-y-1/2 text-muted-foreground pointer-events-none" />
        <Input
          type="search"
          placeholder={m.finance_ui_income_statement_search_placeholder()}
          bind:value={searchQuery}
          class="h-8 ps-8 text-xs bg-card border-border/60"
        />
      </div>

      <Button variant="outline" size="sm" onclick={exportCurrentReport} disabled={Boolean(queryError) || isLoading || !visibleRows.length} class="h-8 text-xs font-bold gap-1.5">
        <Download size={14} />
        <span>{m.finance_ui_export_csv()}</span>
      </Button>

      <Button variant="ghost" size="icon-sm" onclick={refetchAll} class="text-muted-foreground h-8 w-8" title={m.finance_ui_refresh()}>
        {#if isLoading}<LoadingIndicator  size={14} />{:else}<Refresh size={14}   />{/if}
      </Button>
    </div>
  </div>

  <!-- Grid Content -->
  <div class="min-h-0 flex-1 overflow-hidden">
    {#if isLoading}
      <FinanceSkeleton />
    {:else if queryError}
      <ErrorState
        title={m.finance_report_load_error()}
        message={extractErrorMessage(queryError)}
        onRetry={refetchAll}
      />
    {:else}
      <div class="h-full min-h-0 overflow-hidden">
        <OrbisGrid
          columns={statementColumns}
          source={visibleRows}
          showSearch={false}
          clientQuery
          {rtl}
          readonly
          emptyTitle={m.finance_ui_no_income_statement_data()}
          emptyDescription={m.finance_ui_no_income_statement_matches()}
        />
      </div>
    {/if}
  </div>
</div>
