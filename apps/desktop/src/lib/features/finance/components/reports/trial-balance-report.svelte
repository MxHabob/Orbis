<!--
  @component TrialBalanceReport
  Multi-Currency Trial Balance (ميزان المراجعة) reporting grid.
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
let selectedTypeFilter = $state<string>("all");

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
    : [{ code: "YER", name: m.finance_ui_currency_yer(), is_base: true, is_active: true }]
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

const filteredAccounts = $derived(
  leafAccounts.filter((acc) => {
    if (selectedTypeFilter === "all") return true;
    const type = String(acc.account_type || acc.sub_type || "").toLowerCase();
    return type === selectedTypeFilter;
  })
);

// Totals for Trial Balance
const currencyBalances = $derived.by(() => {
  const map: Record<string, { debit: number; credit: number }> = {};
  for (const curr of activeCurrencies) {
    let debit = 0;
    let credit = 0;
    for (const acc of leafAccounts) {
      const amt = getAccountBalance(acc, curr.code);
      if (amt > 0) debit += amt;
      else if (amt < 0) credit += Math.abs(amt);
    }
    map[curr.code] = { debit, credit };
  }
  return map;
});

const isBalanced = $derived.by(() => {
  for (const curr of activeCurrencies) {
    const b = currencyBalances[curr.code];
    if (b && Math.abs(b.debit - b.credit) > 0.01) {
      return false;
    }
  }
  return true;
});

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
    header: m.finance_ui_account_code(),
    name: m.finance_ui_account_code(),
    width: 110,
    size: 110,
    readonly: true,
    cellClass: "font-mono font-bold text-foreground",
  },
  {
    id: "account_name",
    prop: "account_name",
    header: m.common_col_name(),
    name: m.finance_ui_account_name(),
    width: 250,
    size: 250,
    readonly: true,
    cellClass: "font-medium",
  },
  {
    id: "account_type",
    prop: "account_type",
    header: m.common_word_type(),
    name: m.finance_ui_account_category(),
    width: 120,
    size: 120,
    readonly: true,
    align: "center",
  },
  ...activeCurrencies.flatMap((curr) => [
    {
      id: `${curr.code}_debit`,
      prop: `${curr.code}_debit`,
      header: m.finance_ui_trial_debit_column({ currency: curr.name || curr.code, code: curr.code }),
      name: m.finance_ui_trial_debit_column({ currency: curr.name || curr.code, code: curr.code }),
      width: 140,
      size: 140,
      readonly: true,
      align: "end" as const,
      cellClass: "font-mono text-emerald-600 dark:text-emerald-400 font-semibold",
    },
    {
      id: `${curr.code}_credit`,
      prop: `${curr.code}_credit`,
      header: m.finance_ui_trial_credit_column({ currency: curr.name || curr.code, code: curr.code }),
      name: m.finance_ui_trial_credit_column({ currency: curr.name || curr.code, code: curr.code }),
      width: 140,
      size: 140,
      readonly: true,
      align: "end" as const,
      cellClass: "font-mono text-rose-600 dark:text-rose-400 font-semibold",
    },
  ]),
]);

const accountTypeLabels: Record<string, string> = {
  asset: m.finance_ui_account_type_asset(),
  liability: m.finance_ui_account_type_liability(),
  equity: m.finance_ui_account_type_equity(),
  revenue: m.finance_ui_account_type_revenue(),
  expense: m.finance_ui_account_type_expense(),
};

const trialSource = $derived(
  filteredAccounts.map((account, idx) => {
    const rawType = String(account.account_type || account.sub_type || "asset").toLowerCase();
    const typeLabel = accountTypeLabels[rawType] || rawType;
    const row: Record<string, unknown> = {
      __idx: idx + 1,
      account_code: account.code || "—",
      account_name: account.name_ar || account.name || "—",
      account_type: typeLabel,
    };
    for (const curr of activeCurrencies) {
      const amt = getAccountBalance(account, curr.code);
      row[`${curr.code}_debit`] = amt > 0 ? formatCurrency(amt, curr.code) : "—";
      row[`${curr.code}_credit`] = amt < 0 ? formatCurrency(Math.abs(amt), curr.code) : "—";
    }
    return row;
  })
);

function exportCurrentReport() {
  if (trialSource.length === 0) return;
  const filename = `trial_balance_${new Date().toISOString().slice(0, 10)}.csv`;
  const keys = Object.keys(trialSource[0]);
  const csvContent = [
    keys.join(","),
    ...trialSource.map((r) => keys.map((k) => `"${String(r[k] ?? "").replaceAll('"', '""')}"`).join(",")),
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

const filterOptions = [
  { id: "all", label: m.finance_ui_all() },
  { id: "asset", label: m.finance_ui_account_type_asset() },
  { id: "liability", label: m.finance_ui_account_type_liability() },
  { id: "equity", label: m.finance_ui_account_type_equity() },
  { id: "revenue", label: m.finance_ui_account_type_revenue() },
  { id: "expense", label: m.finance_ui_account_type_expense() },
];
</script>

<div class="h-full w-full overflow-hidden bg-background/50 p-6 flex flex-col gap-4 font-sans select-none">
  <!-- Minimal Toolbar Row -->
  <div class="flex flex-wrap items-center justify-between gap-3 shrink-0">
    <div class="flex items-center gap-1.5 flex-wrap">
      {#each filterOptions as opt}
        <button
          type="button"
          class={`px-2.5 py-1 text-xs rounded-lg font-medium transition-all ${
            selectedTypeFilter === opt.id
              ? "bg-primary text-primary-foreground font-bold shadow-xs"
              : "bg-muted/40 text-muted-foreground hover:bg-muted hover:text-foreground border border-border/40"
          }`}
          onclick={() => (selectedTypeFilter = opt.id)}
        >
          {opt.label}
        </button>
      {/each}

      <div class="ms-2 hidden sm:flex items-center">
        {#if isBalanced}
          <span class="inline-flex items-center gap-1 text-[11px] font-medium text-emerald-500 bg-emerald-500/10 px-2 py-0.5 rounded-md border border-emerald-500/20">
            <CheckCircle size={12} />
            <span>{m.finance_ui_balanced()}</span>
          </span>
        {:else}
          <span class="inline-flex items-center gap-1 text-[11px] font-medium text-amber-500 bg-amber-500/10 px-2 py-0.5 rounded-md border border-amber-500/20">
            <InfoCircle size={12} />
            <span>{m.finance_ui_adjustment_difference()}</span>
          </span>
        {/if}
      </div>
    </div>

    <!-- Actions & Search -->
    <div class="flex items-center gap-2">
      <div class="relative w-48 sm:w-60">
        <Search size={14} class="absolute start-2.5 top-1/2 -translate-y-1/2 text-muted-foreground pointer-events-none" />
        <Input
          type="search"
          placeholder={m.finance_ui_search_accounts_placeholder()}
          bind:value={searchQuery}
          class="h-8 ps-8 text-xs bg-card border-border/60"
        />
      </div>

      <Button variant="outline" size="sm" onclick={exportCurrentReport} class="h-8 text-xs font-bold gap-1.5">
        <Download size={14} />
        <span>{m.finance_ui_export_csv()}</span>
      </Button>

      <Button variant="ghost" size="icon-sm" onclick={refetchAll} class="text-muted-foreground h-8 w-8" title={m.finance_ui_refresh()}>
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
        title={m.finance_report_load_error()}
        message={extractErrorMessage(queryError)}
        onRetry={refetchAll}
      />
    {:else}
      <div class="h-full min-h-0 border border-border rounded-xl overflow-hidden shadow-xs bg-card">
        <OrbisGrid
          columns={trialColumns}
          source={trialSource}
          search={searchQuery}
          clientQuery
          {rtl}
          readonly
          emptyTitle={m.finance_ui_no_accounts_registered()}
          emptyDescription={m.finance_ui_no_accounts_match_search()}
        />
      </div>
    {/if}
  </div>
</div>
