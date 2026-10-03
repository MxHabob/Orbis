<!--
  @component FinancialOverviewReport
  Executive financial dashboard displaying high-level KPIs, solvency structure, and system health.
  Styled in the clean, headerless, elegant FinanceOverview aesthetic.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import {
  createAccountsQuery,
  createCurrenciesQuery,
} from "$lib/features/finance/queries";
import { extractErrorMessage } from "$lib/api";
import { formatCurrency } from "$lib/core/i18n";
import {
  Card,
  CardHeader,
  CardTitle,
  CardContent,
  CardFooter,
} from "@orbis/ui/card";
import { MetricProgress } from "@orbis/ui";
import { fade } from "svelte/transition";
import { FinanceSkeleton } from "$lib/shared/components/skeletons";
import ErrorState from "$lib/shared/components/error-state.svelte";
import {
  BuildingLibrary,
  ChartBar,
  DocumentChartBar,
  Scale,
  ArrowRight,
  Clock,
  CheckCircle,
  InfoCircle,
} from "@orbis/icons";
import { getLocale, localizeHref } from "$lib/paraglide/runtime";

let { orgId }: { orgId: string } = $props();

const accountsQuery = createAccountsQuery(() => orgId);
const currenciesQuery = createCurrenciesQuery(() => orgId);

const accounts = $derived(accountsQuery.data ?? []);
const currencies = $derived(currenciesQuery.data ?? []);
const isLoading = $derived(accountsQuery.isLoading || currenciesQuery.isLoading);
const queryError = $derived(accountsQuery.error || currenciesQuery.error);

function refetchAll() {
  accountsQuery.refetch();
  currenciesQuery.refetch();
}

const activeCurrencies = $derived(
  currencies.length > 0
    ? currencies.filter((c) => c.is_active !== false)
    : [{ code: "YER", name: "ريال يمني", is_base: true, is_active: true }]
);

const baseCurrency = $derived(
  currencies.find((c) => c.is_base) || activeCurrencies[0] || { code: "YER", name: "ريال يمني" }
);

const leafAccounts = $derived(
  accounts.filter(
    (account) => !accounts.some((candidate) => candidate.parent_id === account.id)
  )
);

function getAccountBalance(account: any, currencyCode: string) {
  return Number(
    account.balances?.[currencyCode] ||
    account.balances?.[currencyCode.toUpperCase()] ||
    account.balances?.[currencyCode.toLowerCase()] ||
    0
  );
}

function accountsByType(type: string) {
  return leafAccounts.filter(
    (account) => String(account.account_type || account.sub_type || "asset").toLowerCase() === type
  );
}

const totals = $derived.by(() => {
  const result: Record<string, Record<string, number>> = {
    asset: {},
    liability: {},
    equity: {},
    revenue: {},
    expense: {},
  };
  for (const type of ["asset", "liability", "equity", "revenue", "expense"]) {
    for (const currency of activeCurrencies) {
      result[type][currency.code] = accountsByType(type).reduce(
        (sum, account) => sum + getAccountBalance(account, currency.code),
        0
      );
    }
  }
  return result;
});

const overviewMetrics = $derived.by(() => {
  const currencyCode = baseCurrency?.code || "YER";
  const revenue = Math.abs(totals.revenue[currencyCode] || 0);
  const expenses = Math.abs(totals.expense[currencyCode] || 0);
  const assets = Math.abs(totals.asset[currencyCode] || 0);
  const liabilities = Math.abs(totals.liability[currencyCode] || 0);
  const equity = Math.abs(totals.equity[currencyCode] || 0);
  const netIncome = revenue - expenses;
  const isBalanced = Math.abs(assets - (liabilities + equity)) < 0.01;

  return {
    currencyCode,
    assets,
    liabilities,
    equity,
    revenue,
    expenses,
    netIncome,
    isBalanced,
  };
});
</script>

<div class="h-full w-full overflow-y-auto bg-background/50 scrollbar-hide p-6 flex flex-col gap-6 font-sans">
  {#if isLoading}
    <FinanceSkeleton />
  {:else if queryError}
    <ErrorState
      title={m.finance_report_load_error ? m.finance_report_load_error() : "خطأ في تحميل التقرير"}
      message={extractErrorMessage(queryError)}
      onRetry={refetchAll}
    />
  {:else}
    <div class="flex flex-col gap-6" in:fade={{ duration: 150 }}>
      <!-- 1. Top 4 KPI Metrics Cards -->
      <div class="grid grid-cols-2 lg:grid-cols-4 gap-4">
        <Card class="p-4 shadow-sm hover:border-primary/40 transition-colors">
          <MetricProgress variant="single" value={overviewMetrics.assets} showProgress={false} tone="primary">
            {#snippet title()}إجمالي الأصول{/snippet}
            {#snippet metadata()}{formatCurrency(overviewMetrics.assets, overviewMetrics.currencyCode)}{/snippet}
          </MetricProgress>
        </Card>

        <Card class="p-4 shadow-sm hover:border-warning/40 transition-colors">
          <MetricProgress variant="single" value={overviewMetrics.liabilities} showProgress={false} tone="warning">
            {#snippet title()}إجمالي الالتزامات{/snippet}
            {#snippet metadata()}{formatCurrency(overviewMetrics.liabilities, overviewMetrics.currencyCode)}{/snippet}
          </MetricProgress>
        </Card>

        <Card class="p-4 shadow-sm hover:border-success/40 transition-colors">
          <MetricProgress variant="single" value={overviewMetrics.equity} showProgress={false} tone="success">
            {#snippet title()}حقوق الملكية{/snippet}
            {#snippet metadata()}{formatCurrency(overviewMetrics.equity, overviewMetrics.currencyCode)}{/snippet}
          </MetricProgress>
        </Card>

        <Card class="p-4 shadow-sm hover:border-emerald-500/40 transition-colors">
          <MetricProgress
            variant="single"
            value={Math.abs(overviewMetrics.netIncome)}
            showProgress={false}
            tone={overviewMetrics.netIncome >= 0 ? "success" : "warning"}
          >
            {#snippet title()}صافي الأرباح / الفائض{/snippet}
            {#snippet metadata()}{formatCurrency(overviewMetrics.netIncome, overviewMetrics.currencyCode)}{/snippet}
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
                  هيكل المركز المالي والملاءة
                </CardTitle>
              </div>
              {#if overviewMetrics.isBalanced}
                <span class="inline-flex items-center gap-1 text-[10px] font-medium text-emerald-500 bg-emerald-500/10 px-2 py-0.5 rounded-md border border-emerald-500/20">
                  <CheckCircle size={11} />
                  <span>متوازنة</span>
                </span>
              {:else}
                <span class="inline-flex items-center gap-1 text-[10px] font-medium text-amber-500 bg-amber-500/10 px-2 py-0.5 rounded-md border border-amber-500/20">
                  <InfoCircle size={11} />
                  <span>فارق تسوية</span>
                </span>
              {/if}
            </div>
          </CardHeader>

          <CardContent class="p-5 flex flex-col justify-center flex-grow space-y-4">
            <MetricProgress
              variant="segmented"
              segments={[
                { label: "الأصول", value: Math.max(0, overviewMetrics.assets), tone: "primary", displayValue: formatCurrency(overviewMetrics.assets, overviewMetrics.currencyCode) },
                { label: "الالتزامات", value: Math.max(0, overviewMetrics.liabilities), tone: "warning", displayValue: formatCurrency(overviewMetrics.liabilities, overviewMetrics.currencyCode) },
                { label: "حقوق الملكية", value: Math.max(0, overviewMetrics.equity), tone: "success", displayValue: formatCurrency(overviewMetrics.equity, overviewMetrics.currencyCode) },
              ]}
            >
              {#snippet title()}توازن الميزانية (الأصول = الالتزامات + حقوق الملكية){/snippet}
              {#snippet metadata()}{overviewMetrics.currencyCode}{/snippet}
            </MetricProgress>
          </CardContent>

          <CardFooter class="border-t border-border/40 py-2.5 px-4 text-[10px]">
            <a href={localizeHref(`/${orgId}/finance/reports/balance-sheet`)} class="flex items-center gap-1.5 hover:underline text-primary font-bold group/link">
              <span>عرض الميزانية العمومية المفصلة</span>
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
                حالة النظام المحاسبي
              </CardTitle>
            </div>
          </CardHeader>

          <CardContent class="p-4 flex flex-col gap-2.5 w-full grow justify-center text-xs">
            <div class="flex items-center justify-between"><span class="text-muted-foreground">حسابات الدليل</span><strong class="font-mono">{leafAccounts.length}</strong></div>
            <div class="flex items-center justify-between"><span class="text-muted-foreground">العملات المعرفة</span><strong class="font-mono">{activeCurrencies.length}</strong></div>
            <div class="flex items-center justify-between"><span class="text-muted-foreground">العملة الأساسية</span><strong class="font-mono text-primary font-bold">{overviewMetrics.currencyCode}</strong></div>
            <div class="flex items-center justify-between border-t border-border/40 pt-2"><span class="text-muted-foreground">إجمالي الإيرادات</span><strong class="font-mono text-emerald-600 dark:text-emerald-400">{formatCurrency(overviewMetrics.revenue, overviewMetrics.currencyCode)}</strong></div>
            <div class="flex items-center justify-between"><span class="text-muted-foreground">إجمالي المصروفات</span><strong class="font-mono text-rose-600 dark:text-rose-400">{formatCurrency(overviewMetrics.expenses, overviewMetrics.currencyCode)}</strong></div>
          </CardContent>

          <CardFooter class="border-t border-border/40 py-2.5 px-4 text-[10px]">
            <a href={localizeHref(`/${orgId}/finance/accounts`)} class="flex items-center gap-1.5 hover:underline text-primary font-bold group/link">
              <span>إدارة دليل الحسابات</span>
              <ArrowRight size={10} class="transition-transform group-hover/link:translate-x-0.5 rtl:rotate-180" />
            </a>
          </CardFooter>
        </Card>
      </div>

      <!-- 3. Bottom Section: 3 Quick Report Cards -->
      <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
        <Card class="flex flex-col justify-between text-start relative overflow-hidden shadow-sm hover:border-primary/40 transition-colors">
          <CardHeader class="py-3 px-4">
            <div class="flex items-center gap-2 text-primary">
              <Scale size={14} />
              <CardTitle class="text-xs font-bold uppercase tracking-wider text-foreground">ميزان المراجعة</CardTitle>
            </div>
          </CardHeader>
          <CardContent class="p-4 pt-1 flex-grow">
            <p class="text-xs text-muted-foreground leading-relaxed">
              أرصدة جميع الحسابات المدينة والدائنة بكافة العملات للتحقق من التوازن المحاسبي.
            </p>
          </CardContent>
          <CardFooter class="border-t border-border/40 py-2.5 px-4 text-[10px]">
            <a href={localizeHref(`/${orgId}/finance/reports/trial-balance`)} class="flex items-center gap-1.5 hover:underline text-primary font-bold group/link">
              <span>فتح ميزان المراجعة</span>
              <ArrowRight size={10} class="transition-transform group-hover/link:translate-x-0.5 rtl:rotate-180" />
            </a>
          </CardFooter>
        </Card>

        <Card class="flex flex-col justify-between text-start relative overflow-hidden shadow-sm hover:border-primary/40 transition-colors">
          <CardHeader class="py-3 px-4">
            <div class="flex items-center gap-2 text-emerald-600 dark:text-emerald-400">
              <DocumentChartBar size={14} />
              <CardTitle class="text-xs font-bold uppercase tracking-wider text-foreground">قائمة الدخل</CardTitle>
            </div>
          </CardHeader>
          <CardContent class="p-4 pt-1 flex-grow">
            <p class="text-xs text-muted-foreground leading-relaxed">
              بيان الأرباح والخسائر وإجمالي الإيرادات مقابل المصروفات وهوامش الربح.
            </p>
          </CardContent>
          <CardFooter class="border-t border-border/40 py-2.5 px-4 text-[10px]">
            <a href={localizeHref(`/${orgId}/finance/reports/income-statement`)} class="flex items-center gap-1.5 hover:underline text-primary font-bold group/link">
              <span>فتح قائمة الدخل</span>
              <ArrowRight size={10} class="transition-transform group-hover/link:translate-x-0.5 rtl:rotate-180" />
            </a>
          </CardFooter>
        </Card>

        <Card class="flex flex-col justify-between text-start relative overflow-hidden shadow-sm hover:border-primary/40 transition-colors">
          <CardHeader class="py-3 px-4">
            <div class="flex items-center gap-2 text-sky-600 dark:text-sky-400">
              <BuildingLibrary size={14} />
              <CardTitle class="text-xs font-bold uppercase tracking-wider text-foreground">الميزانية العمومية</CardTitle>
            </div>
          </CardHeader>
          <CardContent class="p-4 pt-1 flex-grow">
            <p class="text-xs text-muted-foreground leading-relaxed">
              تفصيل الأصول والموجودات والالتزامات وحقوق الملكية لبيان الوضع المالي.
            </p>
          </CardContent>
          <CardFooter class="border-t border-border/40 py-2.5 px-4 text-[10px]">
            <a href={localizeHref(`/${orgId}/finance/reports/balance-sheet`)} class="flex items-center gap-1.5 hover:underline text-primary font-bold group/link">
              <span>فتح الميزانية العمومية</span>
              <ArrowRight size={10} class="transition-transform group-hover/link:translate-x-0.5 rtl:rotate-180" />
            </a>
          </CardFooter>
        </Card>
      </div>
    </div>
  {/if}
</div>
