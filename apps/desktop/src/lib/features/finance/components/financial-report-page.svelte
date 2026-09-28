<!--
  @component FinancialReportPage
  Authoritative Financial Reporting Suite:
  - Financial Overview & KPI position
  - Multi-Currency Trial Balance (ميزان المراجعة)
  - Income Statement (قائمة الدخل / الأرباح والخسائر)
  - Balance Sheet (الميزانية العمومية)
  Backed 100% by real Orbis accounts, double-entry ledgers, and currency rates.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import {
  createAccountsQuery,
  createCurrenciesQuery,
} from "$lib/features/finance";
import { extractErrorMessage } from "$lib/api";
import { formatCurrency } from "$lib/core/i18n";
import { OrbisGrid, type OrbisColumnDef } from "@orbis/svelte-grid";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { MetricProgress } from "@orbis/ui";
import { fade } from "svelte/transition";
import { FinanceSkeleton } from "$lib/shared/components/skeletons";
import ErrorState from "$lib/shared/components/error-state.svelte";
import {
  ChartBar,
  DocumentChartBar,
  Download,
  Refresh,
  Search,
  BuildingLibrary,
} from "@orbis/icons";
    import { getLocale } from "$lib/paraglide/runtime";

let {
  kind = "overview",
  orgId,
}: {
  kind?: "overview" | "trial_balance" | "income_statement" | "balance_sheet";
  orgId: string;
} = $props();

let activeReport = $state<"overview" | "trial_balance" | "income_statement" | "balance_sheet">(kind);
let searchQuery = $state("");

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

  return {
    currencyCode,
    assets,
    liabilities,
    equity,
    revenue,
    expenses,
    netIncome,
  };
});

// ── Trial Balance Grid ──────────────────────────────────────────────
const trialColumns = $derived.by((): OrbisColumnDef[] => [
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
    id: "account_code",
    prop: "account_code",
    header: "الرمز",
    name: "Code",
    width: 100,
    size: 100,
    readonly: true,
    cellClass: "font-mono font-bold",
  },
  {
    id: "account_name",
    prop: "account_name",
    header: m.common_col_name ? m.common_col_name() : "اسم الحساب",
    name: "Account Name",
    width: 240,
    size: 240,
    readonly: true,
    cellClass: "font-medium",
  },
  {
    id: "account_type",
    prop: "account_type",
    header: m.common_word_type ? m.common_word_type() : "النوع",
    name: "Type",
    width: 110,
    size: 110,
    readonly: true,
    align: "center",
  },
  ...activeCurrencies.flatMap((curr) => [
    {
      id: `${curr.code}_debit`,
      prop: `${curr.code}_debit`,
      header: `${curr.code} - مدين`,
      name: `${curr.code} Debit`,
      width: 130,
      size: 130,
      readonly: true,
      align: "end" as const,
      cellClass: "font-mono text-emerald-600 font-bold",
    },
    {
      id: `${curr.code}_credit`,
      prop: `${curr.code}_credit`,
      header: `${curr.code} - دائن`,
      name: `${curr.code} Credit`,
      width: 130,
      size: 130,
      readonly: true,
      align: "end" as const,
      cellClass: "font-mono text-rose-600 font-bold",
    },
  ]),
]);

const trialSource = $derived(
  leafAccounts.map((account, idx) => {
    const row: Record<string, unknown> = {
      __idx: idx + 1,
      account_code: account.code || "—",
      account_name: account.name_ar || account.name || "—",
      account_type: account.account_type || account.sub_type || "asset",
    };
    for (const curr of activeCurrencies) {
      const amt = getAccountBalance(account, curr.code);
      row[`${curr.code}_debit`] = amt > 0 ? formatCurrency(amt, curr.code) : "—";
      row[`${curr.code}_credit`] = amt < 0 ? formatCurrency(Math.abs(amt), curr.code) : "—";
    }
    return row;
  })
);

// ── Financial Statements (Income Statement & Balance Sheet) ────────
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
    header: "التبويب / القائمة",
    name: "Section",
    width: 160,
    size: 160,
    readonly: true,
    cellClass: "font-bold text-primary",
  },
  {
    id: "account_code",
    prop: "account_code",
    header: "الرمز",
    name: "Code",
    width: 100,
    size: 100,
    readonly: true,
    cellClass: "font-mono",
  },
  {
    id: "account_name",
    prop: "account_name",
    header: "اسم الحساب",
    name: "Account Name",
    width: 250,
    size: 250,
    readonly: true,
    cellClass: "font-medium",
  },
  ...activeCurrencies.map((curr) => ({
    id: curr.code,
    prop: curr.code,
    header: `${curr.name || curr.code} (${curr.code})`,
    name: curr.code,
    width: 150,
    size: 150,
    readonly: true,
    align: "end" as const,
    cellClass: "font-mono font-bold",
  })),
]);

function buildStatementSource(groups: [string, string][]) {
  const rows: Record<string, unknown>[] = [];
  let index = 1;

  for (const [sectionTitle, type] of groups) {
    const list = accountsByType(type);
    for (const account of list) {
      const row: Record<string, unknown> = {
        __idx: index++,
        section: sectionTitle,
        account_code: account.code || "—",
        account_name: account.name_ar || account.name || "—",
      };
      for (const curr of activeCurrencies) {
        const amt = getAccountBalance(account, curr.code);
        row[curr.code] = formatCurrency(Math.abs(amt), curr.code);
      }
      rows.push(row);
    }

    // Subtotal Row
    const subtotalRow: Record<string, unknown> = {
      __idx: "∑",
      section: `إجمالي ${sectionTitle}`,
      account_code: "—",
      account_name: `مجموع ${sectionTitle}`,
    };
    for (const curr of activeCurrencies) {
      const totalAmt = totals[type]?.[curr.code] || 0;
      subtotalRow[curr.code] = formatCurrency(Math.abs(totalAmt), curr.code);
    }
    rows.push(subtotalRow);
  }

  return rows;
}

const incomeStatementGroups: [string, string][] = [
  ["الإيرادات والمبيعات", "revenue"],
  ["المصروفات والتكاليف التشغيلية", "expense"],
];

const balanceSheetGroups: [string, string][] = [
  ["الأصول والموجودات", "asset"],
  ["الالتزامات والخصوم", "liability"],
  ["حقوق الملكية ورأس المال", "equity"],
];

const incomeStatementSource = $derived(buildStatementSource(incomeStatementGroups));
const balanceSheetSource = $derived(buildStatementSource(balanceSheetGroups));

function exportCurrentReport() {
  let exportRows: any[] = [];
  let filename = `financial_report_${activeReport}_${new Date().toISOString().slice(0, 10)}.csv`;

  if (activeReport === "trial_balance") {
    exportRows = trialSource;
  } else if (activeReport === "income_statement") {
    exportRows = incomeStatementSource;
  } else if (activeReport === "balance_sheet") {
    exportRows = balanceSheetSource;
  }

  if (exportRows.length === 0) return;

  const keys = Object.keys(exportRows[0]);
  const csvContent = [
    keys.join(","),
    ...exportRows.map((r) => keys.map((k) => `"${String(r[k] ?? "").replaceAll('"', '""')}"`).join(",")),
  ].join("\n");

  const blob = new Blob(["\ufeff" + csvContent], { type: "text/csv;charset=utf-8;" });
  const url = URL.createObjectURL(blob);
  const a = document.createElement("a");
  a.href = url;
  a.download = filename;
  a.click();
  URL.revokeObjectURL(url);
}

const rtl = $derived(getLocale() === "ar");
</script>

<div class="flex h-full min-h-0 w-full flex-col overflow-hidden bg-background p-6 font-sans text-start select-none">
  <!-- Report Switcher Tabs & Toolbar -->
  <div class="flex flex-wrap items-center justify-between gap-4 pb-4 border-b border-border/80 shrink-0">
    <div class="flex items-center gap-1.5 bg-muted/40 p-1 rounded-xl border border-border/60">
      <Button
        variant={activeReport === "overview" ? "default" : "ghost"}
        size="sm"
        onclick={() => (activeReport = "overview")}
        class="h-8 text-xs font-bold gap-1.5 px-3"
      >
        <ChartBar size={14} />
        <span>المركز المالي</span>
      </Button>
      <Button
        variant={activeReport === "trial_balance" ? "default" : "ghost"}
        size="sm"
        onclick={() => (activeReport = "trial_balance")}
        class="h-8 text-xs font-bold gap-1.5 px-3"
      >
        <BuildingLibrary size={14} />
        <span>ميزان المراجعة</span>
      </Button>
      <Button
        variant={activeReport === "income_statement" ? "default" : "ghost"}
        size="sm"
        onclick={() => (activeReport = "income_statement")}
        class="h-8 text-xs font-bold gap-1.5 px-3"
      >
        <DocumentChartBar size={14} />
        <span>قائمة الدخل</span>
      </Button>
      <Button
        variant={activeReport === "balance_sheet" ? "default" : "ghost"}
        size="sm"
        onclick={() => (activeReport = "balance_sheet")}
        class="h-8 text-xs font-bold gap-1.5 px-3"
      >
        <DocumentChartBar size={14} />
        <span>الميزانية العمومية</span>
      </Button>
    </div>

    <!-- Actions -->
    <div class="flex items-center gap-2">
      {#if activeReport !== "overview"}
        <div class="relative w-48 sm:w-60">
          <Search size={14} class="absolute start-2.5 top-1/2 -translate-y-1/2 text-muted-foreground pointer-events-none" />
          <Input
            type="search"
            placeholder="بحث في الحسابات..."
            bind:value={searchQuery}
            class="h-8 ps-8 text-xs bg-card border-border/60"
          />
        </div>
        <Button variant="outline" size="sm" onclick={exportCurrentReport} class="h-8 text-xs font-bold gap-1.5">
          <Download size={14} />
          <span>تصدير CSV</span>
        </Button>
      {/if}
      <Button variant="ghost" size="icon-sm" onclick={reloadData} class="text-muted-foreground h-8 w-8">
        <Refresh size={14} class={isLoading ? "animate-spin" : ""} />
      </Button>
    </div>
  </div>

  <!-- Main Report Body -->
  <div class="min-h-0 flex-1 pt-4 overflow-hidden">
    {#if isLoading}
      <FinanceSkeleton />
    {:else if queryError}
      <ErrorState
        title={m.finance_report_load_error ? m.finance_report_load_error() : "خطأ في تحميل التقرير"}
        message={extractErrorMessage(queryError)}
        onRetry={reloadData}
      />
    {:else if activeReport === "overview"}
      <!-- 1. Executive Financial Overview Dashboard -->
      <div class="h-full overflow-y-auto space-y-6" in:fade={{ duration: 150 }}>
        <!-- Top Metrics Cards -->
        <div class="grid grid-cols-2 lg:grid-cols-4 gap-4">
          <div class="rounded-xl border border-border bg-card p-4 shadow-xs">
            <MetricProgress variant="single" value={overviewMetrics.assets} showProgress={false} tone="primary">
              {#snippet title()}إجمالي الأصول (Assets){/snippet}
              {#snippet metadata()}{formatCurrency(overviewMetrics.assets, overviewMetrics.currencyCode)}{/snippet}
            </MetricProgress>
          </div>
          <div class="rounded-xl border border-border bg-card p-4 shadow-xs">
            <MetricProgress variant="single" value={overviewMetrics.liabilities} showProgress={false} tone="warning">
              {#snippet title()}إجمالي الالتزامات (Liabilities){/snippet}
              {#snippet metadata()}{formatCurrency(overviewMetrics.liabilities, overviewMetrics.currencyCode)}{/snippet}
            </MetricProgress>
          </div>
          <div class="rounded-xl border border-border bg-card p-4 shadow-xs">
            <MetricProgress variant="single" value={overviewMetrics.equity} showProgress={false} tone="success">
              {#snippet title()}حقوق الملكية (Equity){/snippet}
              {#snippet metadata()}{formatCurrency(overviewMetrics.equity, overviewMetrics.currencyCode)}{/snippet}
            </MetricProgress>
          </div>
          <div class="rounded-xl border border-border bg-card p-4 shadow-xs">
            <MetricProgress variant="single" value={overviewMetrics.netIncome} showProgress={false} tone={overviewMetrics.netIncome >= 0 ? "success" : "warning"}>
              {#snippet title()}صافي الأرباح / الفائض (Net Income){/snippet}
              {#snippet metadata()}{formatCurrency(overviewMetrics.netIncome, overviewMetrics.currencyCode)}{/snippet}
            </MetricProgress>
          </div>
        </div>

        <!-- Position Breakdown & Status Summary -->
        <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
          <div class="border border-border bg-card p-5 rounded-xl shadow-xs lg:col-span-2 space-y-4">
            <h3 class="text-xs font-bold text-foreground uppercase tracking-wider">هيكل المركز المالي والملاءة</h3>
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
          </div>

          <div class="border border-border bg-card p-5 rounded-xl shadow-xs space-y-3">
            <h3 class="text-xs font-bold text-foreground border-b border-border/60 pb-2">حالة النظام المحاسبي</h3>
            <div class="flex flex-col gap-2.5 text-xs">
              <div class="flex items-center justify-between"><span class="text-muted-foreground">حسابات دليل الحسابات</span><strong class="font-mono">{leafAccounts.length}</strong></div>
              <div class="flex items-center justify-between"><span class="text-muted-foreground">العملات المعرفة</span><strong class="font-mono">{activeCurrencies.length}</strong></div>
              <div class="flex items-center justify-between"><span class="text-muted-foreground">العملة الأساسية للنظام</span><strong class="font-mono text-primary font-bold">{overviewMetrics.currencyCode}</strong></div>
              <div class="flex items-center justify-between border-t border-border/40 pt-2.5"><span class="text-muted-foreground">إجمالي الإيرادات</span><strong class="font-mono text-emerald-600">{formatCurrency(overviewMetrics.revenue, overviewMetrics.currencyCode)}</strong></div>
              <div class="flex items-center justify-between"><span class="text-muted-foreground">إجمالي المصروفات</span><strong class="font-mono text-rose-600">{formatCurrency(overviewMetrics.expenses, overviewMetrics.currencyCode)}</strong></div>
            </div>
          </div>
        </div>
      </div>

    {:else if activeReport === "trial_balance"}
      <!-- 2. Trial Balance (ميزان المراجعة) OrbisGrid -->
      <div class="h-full min-h-0">
        <OrbisGrid
          columns={trialColumns}
          source={trialSource}
          search={searchQuery}
          clientQuery
          {rtl}
          readonly
          emptyTitle="لا توجد حسابات مسجلة"
          emptyDescription="لم يتم العثور على حسابات لميزان المراجعة"
        />
      </div>

    {:else if activeReport === "income_statement"}
      <!-- 3. Income Statement (قائمة الدخل) OrbisGrid -->
      <div class="h-full min-h-0">
        <OrbisGrid
          columns={statementColumns}
          source={incomeStatementSource}
          search={searchQuery}
          clientQuery
          {rtl}
          readonly
          emptyTitle="لا توجد بيانات لقائمة الدخل"
          emptyDescription="لم يتم العثور على إيرادات أو مصروفات مسجلة"
        />
      </div>

    {:else if activeReport === "balance_sheet"}
      <!-- 4. Balance Sheet (الميزانية العمومية) OrbisGrid -->
      <div class="h-full min-h-0">
        <OrbisGrid
          columns={statementColumns}
          source={balanceSheetSource}
          search={searchQuery}
          clientQuery
          {rtl}
          readonly
          emptyTitle="لا توجد بيانات للميزانية العمومية"
          emptyDescription="لم يتم العثور على أصول أو التزامات مسجلة"
        />
      </div>
    {/if}
  </div>
</div>
