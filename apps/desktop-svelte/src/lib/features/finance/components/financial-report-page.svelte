<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { createAccountsQuery, createCurrenciesQuery } from "$lib/features/finance";
import { extractErrorMessage } from "$lib/core/ipc";
import { getLocale } from "$lib/paraglide/runtime";
import { formatCurrency } from "$lib/core/i18n";
import { DataTable } from "$lib/tables";
import type { OrbisColumnDef } from "@orbis/svelte-grid";
import { MetricProgress } from "@orbis/ui-svelte";
import { fade } from "svelte/transition";
import { FinanceSkeleton } from "$lib/shared/components/skeletons";
import ErrorState from "$lib/shared/components/error-state.svelte";

let { kind, orgId }: { kind: "overview" | "trial_balance" | "income_statement" | "balance_sheet"; orgId: string } = $props();

const accountsQuery = createAccountsQuery(() => orgId);
const currenciesQuery = createCurrenciesQuery(() => orgId);
const accounts = $derived(accountsQuery.data ?? []);
const currencies = $derived(currenciesQuery.data ?? []);
const isLoading = $derived(accountsQuery.isLoading || currenciesQuery.isLoading);
const queryError = $derived(accountsQuery.error || currenciesQuery.error);

function reloadData() {
  accountsQuery.refetch();
  currenciesQuery.refetch();
}

const activeCurrencies = $derived(currencies.filter((currency) => currency.is_active === 1));
const baseCurrency = $derived(currencies.find((currency) => currency.is_base === 1) || activeCurrencies[0]);
const leafAccounts = $derived(accounts.filter((account) => !accounts.some((candidate) => candidate.parent_id === account.id)));
function balance(account: any, code: string) {
  return Number(account.balances?.[code] || account.balances?.[code.toUpperCase()] || 0);
}

function accountsByType(type: string) {
  return leafAccounts.filter((account) => String(account.item_type || account.type || "").toLowerCase() === type);
}

const totals = $derived.by(() => {
  const result: Record<string, Record<string, number>> = {};
  for (const type of ["asset", "liability", "equity", "revenue", "expense"]) {
    result[type] = {};
    for (const currency of activeCurrencies) {
      result[type][currency.code] = accountsByType(type).reduce((sum, account) => sum + balance(account, currency.code), 0);
    }
  }
  return result;
});

const overviewMetrics = $derived.by(() => {
  const currencyCode = baseCurrency?.code || "YER";
  const revenue = totals.revenue[currencyCode] || 0;
  const expenses = totals.expense[currencyCode] || 0;
  return {
    currencyCode,
    assets: totals.asset[currencyCode] || 0,
    liabilities: totals.liability[currencyCode] || 0,
    equity: totals.equity[currencyCode] || 0,
    netIncome: revenue - expenses,
  };
});

const trialColumns = $derived.by((): OrbisColumnDef[] => [
  { id: "account", prop: "account", name: "الحساب", size: 260, readonly: true },
  ...activeCurrencies.flatMap((currency) => [
    { id: `${currency.code}_debit`, prop: `${currency.code}_debit`, name: `مدين ${currency.code}`, size: 120, readonly: true, cellClass: "orbis-cell-number" },
    { id: `${currency.code}_credit`, prop: `${currency.code}_credit`, name: `دائن ${currency.code}`, size: 120, readonly: true, cellClass: "orbis-cell-number" },
  ]),
]);

const trialSource = $derived(
  leafAccounts.map((account) => {
    const row: Record<string, unknown> = {
      account: `${account.code || "-"} - ${account.name_ar || account.name || "-"}`,
    };
    for (const currency of activeCurrencies) {
      const amount = balance(account, currency.code);
      row[`${currency.code}_debit`] = amount > 0 ? formatCurrency(amount, currency.code) : "-";
      row[`${currency.code}_credit`] = amount < 0 ? formatCurrency(Math.abs(amount), currency.code) : "-";
    }
    return row;
  }),
);

function reportColumns(): OrbisColumnDef[] {
  return [
    { id: "section", prop: "section", name: "القسم", size: 150, readonly: true },
    { id: "account", prop: "account", name: "الحساب", size: 280, readonly: true },
    ...activeCurrencies.map((currency) => ({
      id: currency.code,
      prop: currency.code,
      name: currency.code,
      size: 140,
      readonly: true,
      cellClass: "orbis-cell-number",
    })),
  ];
}

function reportSource(groups: string[][]) {
  const rows: Record<string, unknown>[] = [];
  for (const group of groups) {
    const type = group[1]!;
    for (const account of accountsByType(type)) {
    const row: Record<string, unknown> = {
      section: group[0],
      account: `${account.code || "-"} - ${account.name_ar || account.name || "-"}`,
    };
    for (const currency of activeCurrencies) {
      const amount = balance(account, currency.code);
      row[currency.code] = formatCurrency(amount, currency.code);
    }
    rows.push(row);
    }
  }
  for (const group of groups) {
    const type = group[1]!;
    const totalRow: Record<string, unknown> = { section: group[0], account: "الإجمالي" };
    for (const currency of activeCurrencies) {
      const amount = totals[type][currency.code] || 0;
      totalRow[currency.code] = formatCurrency(amount, currency.code);
    }
    rows.push(totalRow);
  }
  return rows;
}
</script>

<div class="flex h-full min-h-0 w-full flex-col overflow-hidden bg-background p-6" dir={getLocale() === "ar" ? "rtl" : "ltr"}>
  <div class="min-h-0 flex-1 overflow-hidden">
    {#if isLoading}
      <FinanceSkeleton />
    {:else if queryError}
      <ErrorState
        title="تعذر تحميل التقرير"
        message={extractErrorMessage(queryError)}
        onRetry={reloadData}
      />
    {:else if kind === "overview"}
      <div class="h-full overflow-y-auto" in:fade={{ duration: 150 }}>
        <div class="flex flex-col gap-6">
          <div class="grid grid-cols-2 gap-3 lg:grid-cols-4">
            <div class="rounded-lg border border-border/70 bg-card p-3">
              <MetricProgress variant="single" value={overviewMetrics.assets} showProgress={false} tone="primary">
                {#snippet title()}إجمالي الأصول{/snippet}
                {#snippet metadata()}{overviewMetrics.currencyCode}{/snippet}
              </MetricProgress>
            </div>
            <div class="rounded-lg border border-border/70 bg-card p-3">
              <MetricProgress variant="single" value={overviewMetrics.liabilities} showProgress={false} tone="warning">
                {#snippet title()}إجمالي الخصوم{/snippet}
                {#snippet metadata()}{overviewMetrics.currencyCode}{/snippet}
              </MetricProgress>
            </div>
            <div class="rounded-lg border border-border/70 bg-card p-3">
              <MetricProgress variant="single" value={overviewMetrics.equity} showProgress={false} tone="success">
                {#snippet title()}حقوق الملكية{/snippet}
                {#snippet metadata()}{overviewMetrics.currencyCode}{/snippet}
              </MetricProgress>
            </div>
            <div class="rounded-lg border border-border/70 bg-card p-3">
              <MetricProgress variant="single" value={overviewMetrics.netIncome} showProgress={false} tone={overviewMetrics.netIncome >= 0 ? "success" : "warning"}>
                {#snippet title()}صافي نتيجة النشاط{/snippet}
                {#snippet metadata()}{overviewMetrics.currencyCode}{/snippet}
              </MetricProgress>
            </div>
          </div>

          <div class="grid grid-cols-1 gap-4 lg:grid-cols-3">
            <div class="border border-border/60 bg-card p-5 text-start shadow-sm lg:col-span-2">
              <MetricProgress
                variant="segmented"
                segments={[
                  { label: "الأصول", value: Math.max(0, overviewMetrics.assets), tone: "primary", displayValue: formatCurrency(overviewMetrics.assets, overviewMetrics.currencyCode) },
                  { label: "الخصوم", value: Math.max(0, overviewMetrics.liabilities), tone: "warning", displayValue: formatCurrency(overviewMetrics.liabilities, overviewMetrics.currencyCode) },
                  { label: "حقوق الملكية", value: Math.max(0, overviewMetrics.equity), tone: "success", displayValue: formatCurrency(overviewMetrics.equity, overviewMetrics.currencyCode) },
                ]}
              >
                {#snippet title()}المركز المالي{/snippet}
                {#snippet metadata()}{overviewMetrics.currencyCode}{/snippet}
              </MetricProgress>
            </div>
            <div class="border border-border/60 bg-card p-5 text-start shadow-sm">
              <h2 class="mb-4 border-b border-border/40 pb-3 text-xs font-bold">حالة البيانات</h2>
              <div class="flex flex-col gap-3 text-xs">
                <div class="flex items-center justify-between"><span class="text-muted-foreground">الحسابات المرحّلة</span><strong class="font-mono">{leafAccounts.length}</strong></div>
                <div class="flex items-center justify-between"><span class="text-muted-foreground">العملات النشطة</span><strong class="font-mono">{activeCurrencies.length}</strong></div>
                <div class="flex items-center justify-between border-t border-border/40 pt-3"><span class="text-muted-foreground">عملة العرض</span><strong class="font-mono">{overviewMetrics.currencyCode}</strong></div>
              </div>
            </div>
          </div>
        </div>
      </div>
    {:else if kind === "trial_balance"}
      <div class="flex h-full min-h-0 flex-col overflow-hidden">
        <div class="min-h-0 flex-1" dir="rtl">
          <DataTable
            columns={trialColumns}
            source={trialSource}
            searchable
            searchPlaceholder="ابحث في الحسابات..."
            emptyTitle="لا توجد حسابات"
            rtl
            exportable
            refreshable
            onRefresh={reloadData}
            height="100%"
            theme="compact"
          />
        </div>
      </div>
    {:else}
      {@const groups = kind === "income_statement" ? [["الإيرادات", "revenue"], ["المصروفات", "expense"]] : [["الأصول", "asset"], ["الخصوم", "liability"], ["حقوق الملكية", "equity"]]}
      <div class="h-full min-h-0" dir="rtl">
        <DataTable
          columns={reportColumns()}
          source={reportSource(groups)}
          searchable
          searchPlaceholder="ابحث في الحسابات..."
          emptyTitle="لا توجد حسابات"
          rtl
          exportable
          refreshable
          onRefresh={reloadData}
          height="100%"
          theme="compact"
        />
      </div>
    {/if}
  </div>
</div>
