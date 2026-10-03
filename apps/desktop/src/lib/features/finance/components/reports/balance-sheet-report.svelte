<!--
  @component BalanceSheetReport
  Balance Sheet (الميزانية العمومية) reporting grid.
  Sleek, headerless layout matching FinanceOverview.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import {
  createAccountsQuery,
  createCurrenciesQuery,
} from "$lib/features/finance/queries";
import { extractErrorMessage } from "$lib/api";
import { formatCurrency } from "$lib/core/i18n";
import { OrbisGrid, type OrbisColumnDef } from "@orbis/svelte-grid";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { FinanceSkeleton } from "$lib/shared/components/skeletons";
import ErrorState from "$lib/shared/components/error-state.svelte";
import {
  Download,
  Refresh,
  Search,
  CheckCircle,
  InfoCircle,
} from "@orbis/icons";
import { getLocale } from "$lib/paraglide/runtime";

let { orgId }: { orgId: string } = $props();

let searchQuery = $state("");

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
  };
  for (const type of ["asset", "liability", "equity"]) {
    for (const currency of activeCurrencies) {
      result[type][currency.code] = accountsByType(type).reduce(
        (sum, account) => sum + getAccountBalance(account, currency.code),
        0
      );
    }
  }
  return result;
});

const summaryKpis = $derived.by(() => {
  const code = baseCurrency?.code || "YER";
  const assets = Math.abs(totals.asset[code] || 0);
  const liabilities = Math.abs(totals.liability[code] || 0);
  const equity = Math.abs(totals.equity[code] || 0);
  const liabilitiesAndEquity = liabilities + equity;
  const isBalanced = Math.abs(assets - liabilitiesAndEquity) < 0.01;

  return { code, assets, liabilities, equity, liabilitiesAndEquity, isBalanced };
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
    header: "القسم / التبويب",
    name: "Section",
    width: 170,
    size: 170,
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
    width: 260,
    size: 260,
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

const balanceSheetGroups: [string, string][] = [
  ["الأصول والموجودات", "asset"],
  ["الالتزامات والخصوم", "liability"],
  ["حقوق الملكية ورأس المال", "equity"],
];

const balanceSheetSource = $derived.by(() => {
  const rows: Record<string, unknown>[] = [];
  let index = 1;

  for (const [sectionTitle, type] of balanceSheetGroups) {
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

  // Combined Liabilities + Equity Row
  const leRow: Record<string, unknown> = {
    __idx: "★",
    section: "إجمالي الالتزامات وحقوق الملكية",
    account_code: "—",
    account_name: "مجموع الخصوم + رأس المال",
  };
  for (const curr of activeCurrencies) {
    const l = Math.abs(totals.liability[curr.code] || 0);
    const e = Math.abs(totals.equity[curr.code] || 0);
    leRow[curr.code] = formatCurrency(l + e, curr.code);
  }
  rows.push(leRow);

  return rows;
});

function exportCurrentReport() {
  if (balanceSheetSource.length === 0) return;
  const filename = `balance_sheet_${new Date().toISOString().slice(0, 10)}.csv`;
  const keys = Object.keys(balanceSheetSource[0]);
  const csvContent = [
    keys.join(","),
    ...balanceSheetSource.map((r) => keys.map((k) => `"${String(r[k] ?? "").replaceAll('"', '""')}"`).join(",")),
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

<div class="h-full w-full overflow-hidden bg-background/50 p-6 flex flex-col gap-4 font-sans select-none">
  <!-- Minimal Toolbar Row -->
  <div class="flex flex-wrap items-center justify-between gap-3 shrink-0">
    <!-- Quick Status Badges -->
    <div class="flex items-center gap-2 flex-wrap text-xs">
      <div class="flex items-center gap-1.5 px-2.5 py-1 rounded-lg bg-primary/10 border border-primary/20 text-primary font-medium">
        <span class="text-muted-foreground text-[11px]">الأصول:</span>
        <strong class="font-mono">{formatCurrency(summaryKpis.assets, summaryKpis.code)}</strong>
      </div>

      <div class="flex items-center gap-1.5 px-2.5 py-1 rounded-lg bg-muted border border-border/60 text-foreground font-medium">
        <span class="text-muted-foreground text-[11px]">الخصوم + الملكية:</span>
        <strong class="font-mono">{formatCurrency(summaryKpis.liabilitiesAndEquity, summaryKpis.code)}</strong>
      </div>

      {#if summaryKpis.isBalanced}
        <div class="flex items-center gap-1 text-emerald-600 dark:text-emerald-400 bg-emerald-500/10 px-2.5 py-1 rounded-lg border border-emerald-500/20 font-medium">
          <CheckCircle size={12} />
          <span>الميزانية متطابقة</span>
        </div>
      {:else}
        <div class="flex items-center gap-1 text-amber-600 dark:text-amber-400 bg-amber-500/10 px-2.5 py-1 rounded-lg border border-amber-500/20 font-medium">
          <InfoCircle size={12} />
          <span>يوجد فارق تسوية</span>
        </div>
      {/if}
    </div>

    <!-- Actions & Search -->
    <div class="flex items-center gap-2">
      <div class="relative w-48 sm:w-60">
        <Search size={14} class="absolute start-2.5 top-1/2 -translate-y-1/2 text-muted-foreground pointer-events-none" />
        <Input
          type="search"
          placeholder="بحث في بنود الميزانية..."
          bind:value={searchQuery}
          class="h-8 ps-8 text-xs bg-card border-border/60"
        />
      </div>

      <Button variant="outline" size="sm" onclick={exportCurrentReport} class="h-8 text-xs font-bold gap-1.5">
        <Download size={14} />
        <span>تصدير CSV</span>
      </Button>

      <Button variant="ghost" size="icon-sm" onclick={refetchAll} class="text-muted-foreground h-8 w-8" title="تحديث">
        <Refresh size={14} class={isLoading ? "animate-spin" : ""} />
      </Button>
    </div>
  </div>

  <!-- Grid Content -->
  <div class="min-h-0 flex-1 overflow-hidden">
    {#if isLoading}
      <FinanceSkeleton />
    {:else if queryError}
      <ErrorState
        title={m.finance_report_load_error ? m.finance_report_load_error() : "خطأ في تحميل التقرير"}
        message={extractErrorMessage(queryError)}
        onRetry={refetchAll}
      />
    {:else}
      <div class="h-full min-h-0 border border-border rounded-xl overflow-hidden shadow-xs bg-card">
        <OrbisGrid
          columns={statementColumns}
          source={balanceSheetSource}
          search={searchQuery}
          clientQuery
          {rtl}
          readonly
          emptyTitle="لا توجد بيانات للميزانية العمومية"
          emptyDescription="لم يتم العثور على أصول أو خصوم مسجلة تطابق البحث"
        />
      </div>
    {/if}
  </div>
</div>
