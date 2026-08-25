<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import { goto } from "$app/navigation";
import { commands } from "$lib/core/ipc";
import { getLocale } from "$lib/paraglide/runtime";
import { formatCurrency } from "$lib/core/i18n";
import { Button } from "@orbis/ui-svelte/button";
import { Input } from "@orbis/ui-svelte/input";
import { ComboboxDropdown } from "@orbis/ui-svelte/combobox-dropdown";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import { DataTable } from "$lib/tables";
import type { OrbisColumnDef } from "@orbis/svelte-grid";
import CurrencySelector from "$lib/shared/components/currency-selector.svelte";
import AccountTree from "$lib/features/finance/components/account-tree.svelte";
import { ArrowLeft, ArrowRight } from "@lucide/svelte";

let { data }: { data: { orgId: string } } = $props();

let accounts = $state<any[]>([]);
let currencies = $state<any[]>([]);
let payments = $state<any[]>([]);
let isLoading = $state(true);
let error = $state<string | null>(null);
let activeView = $state<"all" | "single">("all");
let selectedAccountId = $state("");
let selectedCurrency = $state("all");
let selectedIds = $state<Set<string>>(new Set());
let searchQuery = $state("");
let period = $state("all");
let movementFilter = $state<"all" | "active">("all");
let balanceFilter = $state<"all" | "debit" | "credit">("all");
let asOfDate = $state("");

async function loadData() {
  isLoading = true;
  error = null;
  try {
    const [accountList, currencyList, paymentList] = await Promise.all([
      commands.finance.listAccounts(),
      commands.finance.listCurrencies().catch(() => []),
      commands.finance.listPayments(null, null).catch(() => []),
    ]);
    accounts = accountList || [];
    currencies = currencyList || [];
    payments = paymentList || [];

    const leaves = accounts.filter((account) => !accounts.some((candidate) => candidate.parent_id === account.id));
    selectedIds = new Set(leaves.map((account) => account.id));
    if (!selectedAccountId && leaves[0]) selectedAccountId = leaves[0].id;
  } catch (cause) {
    error = cause instanceof Error ? cause.message : String(cause);
  } finally {
    isLoading = false;
  }
}

onMount(() => {
  if (data.orgId) loadData();
});

function buildTree(items: any[]): any[] {
  const nodes = new Map(items.map((account) => [account.id, { ...account, children: [] }]));
  const roots: any[] = [];
  for (const account of items) {
    const node = nodes.get(account.id);
    const parent = account.parent_id ? nodes.get(account.parent_id) : undefined;
    if (parent) parent.children.push(node);
    else roots.push(node);
  }
  return roots;
}

const treeCategories = $derived.by(() => {
  const definitions = [
    ["asset", "الأصول", "Assets"],
    ["liability", "الخصوم", "Liabilities"],
    ["equity", "حقوق الملكية", "Equity"],
    ["revenue", "الإيرادات", "Revenue"],
    ["expense", "المصروفات", "Expenses"],
  ];
  return definitions.map(([id, name_ar, name_en]) => ({
    id,
    name_ar,
    name_en,
    accounts: buildTree(accounts.filter((account) => String(account.type || account.item_type || "asset").toLowerCase() === id)),
  }));
});

const activeCurrencies = $derived(
  currencies
    .map((currency) => ({ code: String(currency.code || "").toUpperCase(), name: currency.name_ar || currency.name || currency.code }))
    .filter((currency) => currency.code),
);

const visibleCurrencies = $derived(
  selectedCurrency === "all" ? activeCurrencies : activeCurrencies.filter((currency) => currency.code === selectedCurrency),
);

const leafAccounts = $derived(accounts.filter((account) => !accounts.some((candidate) => candidate.parent_id === account.id)));
const accountItems = $derived(leafAccounts.map((account) => ({
  id: account.id,
  value: account.id,
  label: `${account.code} - ${account.name_ar || account.name || "-"}`,
})));
const selectedAccountItem = $derived(accountItems.find((item) => item.value === selectedAccountId));

function balanceFor(account: any, code: string) {
  return Number(account.balances?.[code] || account.balances?.[code.toUpperCase()] || 0);
}

function accountRows() {
  const query = searchQuery.trim().toLowerCase();
  return accounts
    .filter((account) => !accounts.some((candidate) => candidate.parent_id === account.id))
    .filter((account) => selectedIds.has(account.id))
    .filter((account) => !query || String(account.name_ar || account.name || "").toLowerCase().includes(query) || String(account.code || "").includes(query))
    .filter((account) => activeView === "all" || account.id === selectedAccountId)
    .filter((account) => {
      const balances = visibleCurrencies.map((currency) => balanceFor(account, currency.code));
      const hasMovement = balances.some((balance) => balance !== 0);
      const isDebit = balances.some((balance) => balance > 0);
      const isCredit = balances.some((balance) => balance < 0);
      return (movementFilter === "all" || hasMovement) &&
        (balanceFilter === "all" || (balanceFilter === "debit" ? isDebit : isCredit));
    })
    .map((account, index) => {
      const row: Record<string, unknown> = {
        __idx: index + 1,
        account_name: account.name_ar || account.name || "-",
        account_code: account.code || "-",
        account_type: account.type || account.item_type || "-",
      };
      for (const currency of visibleCurrencies) {
        const balance = balanceFor(account, currency.code);
        row[`${currency.code}_debit`] = balance > 0 ? formatCurrency(balance, currency.code) : "-";
        row[`${currency.code}_credit`] = balance < 0 ? formatCurrency(Math.abs(balance), currency.code) : "-";
      }
      return row;
    });
}

function statementRows() {
  const account = accounts.find((item) => item.id === selectedAccountId);
  if (!account) return [];
  const query = searchQuery.trim().toLowerCase();
  const currency = selectedCurrency === "all" ? String(account.currency || activeCurrencies[0]?.code || "YER").toUpperCase() : selectedCurrency;
  const rows = payments
    .filter((payment) => payment.cash_account_id === selectedAccountId || payment.account_id === selectedAccountId || payment.ledger_account_id === selectedAccountId)
    .filter((payment) => String(payment.currency || currency).toUpperCase() === currency)
    .map((payment) => ({
      date: String(payment.paid_at || payment.created_at || "").split("T")[0] || "-",
      reference: payment.receipt_number || payment.reference || payment.id?.slice(0, 8) || "-",
      description: payment.notes || payment.payer_name || "Payment",
      debit: Number(payment.amount || 0),
      credit: 0,
    }))
    .filter((row) => !query || `${row.date} ${row.reference} ${row.description}`.toLowerCase().includes(query))
    .sort((first, second) => first.date.localeCompare(second.date));

  let running = Number(account.opening_balance || 0);
  return rows.map((row, index) => {
    running += row.debit - row.credit;
    return {
      __idx: index + 1,
      date: row.date,
      reference: row.reference,
      description: row.description,
      debit: row.debit ? formatCurrency(row.debit, currency) : "-",
      credit: row.credit ? formatCurrency(row.credit, currency) : "-",
      balance: formatCurrency(Math.abs(running), currency),
    };
  });
}

const source = $derived(activeView === "single" ? statementRows() : accountRows());
const columns = $derived.by((): OrbisColumnDef[] => {
  if (activeView === "single") {
    return [
      { id: "__idx", prop: "__idx", header: "#", name: "#", width: 48, size: 48, readonly: true },
      { id: "date", prop: "date", header: "Date", name: "Date", width: 115, size: 115, readonly: true },
      { id: "reference", prop: "reference", header: "Reference", name: "Reference", width: 125, size: 125, readonly: true, cellClass: "font-mono" },
      { id: "description", prop: "description", header: "Description", name: "Description", width: 260, size: 260, readonly: true },
      { id: "debit", prop: "debit", header: "Debit", name: "Debit", width: 130, size: 130, readonly: true, cellClass: "orbis-cell-number" },
      { id: "credit", prop: "credit", header: "Credit", name: "Credit", width: 130, size: 130, readonly: true, cellClass: "orbis-cell-number" },
      { id: "balance", prop: "balance", header: "Running balance", name: "Running balance", width: 145, size: 145, readonly: true, cellClass: "orbis-cell-number" },
    ];
  }
  const currencyColumns = visibleCurrencies.flatMap((currency) => [
    { id: `${currency.code}_debit`, prop: `${currency.code}_debit`, header: `${currency.name} - Debit`, name: `${currency.name} - Debit`, width: 125, size: 125, readonly: true, cellClass: "orbis-cell-number" },
    { id: `${currency.code}_credit`, prop: `${currency.code}_credit`, header: `${currency.name} - Credit`, name: `${currency.name} - Credit`, width: 125, size: 125, readonly: true, cellClass: "orbis-cell-number" },
  ]);
  return [
    { id: "__idx", prop: "__idx", header: "#", name: "#", width: 48, size: 48, readonly: true },
    { id: "account_name", prop: "account_name", header: m.common_ledger_col_name(), name: m.common_ledger_col_name(), width: 220, size: 220, readonly: true },
    { id: "account_code", prop: "account_code", header: m.common_ledger_col_code(), name: m.common_ledger_col_code(), width: 105, size: 105, readonly: true, cellClass: "font-mono" },
    ...currencyColumns,
    { id: "account_type", prop: "account_type", header: "Type", name: "Type", width: 110, size: 110, readonly: true },
  ];
});

function selectAccount(account: any) {
  selectedAccountId = account.id;
  activeView = "single";
}

function exitPage() {
  goto(`/${data.orgId}/finance`);
}
</script>

<div class="flex h-full min-h-0 w-full flex-col overflow-hidden bg-background text-foreground">
  <div class="flex min-h-0 flex-1 flex-col gap-4 overflow-hidden p-6 lg:flex-row">
    <aside class="flex w-full shrink-0 flex-col gap-3 lg:w-64">
      <AccountTree categories={treeCategories} bind:selectedIds={selectedIds} />
      <div class="flex flex-col gap-3 border-t border-border/70 pt-3">
        <div class="flex flex-col gap-1"><CurrencySelector title={m.common_ledger_currency()} description="" compact includeAll allLabel={m.common_ledger_all_currencies()} currentCurrency={selectedCurrency} onchange={(currency) => selectedCurrency = currency} /></div>
        <div class="grid grid-cols-2 gap-2">
          <label class="flex flex-col gap-1 text-[9px] font-bold text-muted-foreground">Period
            <select bind:value={period} class="h-8 rounded-md border border-border bg-card px-2 text-xs text-foreground"><option value="all">All periods</option><option value="day">Today</option><option value="month">This month</option><option value="year">This year</option></select>
          </label>
          <label class="flex flex-col gap-1 text-[9px] font-bold text-muted-foreground">As of
            <Input type="date" bind:value={asOfDate} class="h-8 px-2 text-xs" />
          </label>
        </div>
        <label class="flex items-center justify-between gap-2 text-[10px] text-muted-foreground">Show active accounts
          <input type="checkbox" checked={movementFilter === "active"} onchange={(event) => movementFilter = (event.currentTarget as HTMLInputElement).checked ? "active" : "all"} class="accent-primary" />
        </label>
        <select bind:value={balanceFilter} class="h-8 rounded-md border border-border bg-card px-2 text-xs text-foreground"><option value="all">All balances</option><option value="debit">Debit balances</option><option value="credit">Credit balances</option></select>
      </div>
      <Button variant="ghost" class="justify-start text-xs text-muted-foreground" onclick={exitPage}>{#if getLocale() === "ar"}<ArrowRight size={14} class="me-2" />{:else}<ArrowLeft size={14} class="me-2" />{/if}Back to finance</Button>
    </aside>

    <main class="flex min-h-0 min-w-0 flex-1 flex-col overflow-hidden">
      <div class="mb-3 flex shrink-0 items-center justify-between gap-3">
        <div class="flex items-center gap-1 rounded-md border border-border bg-card p-1">
          <button type="button" class="rounded px-3 py-1.5 text-xs font-semibold transition-colors {activeView === 'all' ? 'bg-primary text-primary-foreground' : 'text-muted-foreground hover:text-foreground'}" onclick={() => activeView = "all"}>All accounts</button>
          <button type="button" class="rounded px-3 py-1.5 text-xs font-semibold transition-colors {activeView === 'single' ? 'bg-primary text-primary-foreground' : 'text-muted-foreground hover:text-foreground'}" onclick={() => activeView = "single"}>Single account</button>
        </div>
        {#if activeView === "single"}
          <ComboboxDropdown
            class="w-64"
            placeholder="Search account..."
            searchPlaceholder="Search by account name or code..."
            items={accountItems}
            selectedItem={selectedAccountItem}
            onSelect={(item) => selectAccount(accounts.find((account) => account.id === item.value))}
          />
        {/if}
      </div>

      {#if error}
        <div class="flex flex-1 flex-col items-center justify-center gap-3 text-center"><p class="text-sm font-semibold text-destructive">{m.finance_error_loading()}</p><p class="max-w-md text-xs text-muted-foreground">{error}</p><Button variant="outline" size="sm" onclick={loadData}>{m.common_retry()}</Button></div>
      {:else if isLoading}
        <div class="flex flex-col gap-2">{#each Array(7) as _}<Skeleton class="h-9 w-full" />{/each}</div>
      {:else}
        <DataTable columns={columns} source={source} searchable searchValue={searchQuery} onSearch={(query) => searchQuery = query} searchPlaceholder="Search account by name or code..." refreshable onRefresh={loadData} exportable rtl={getLocale() === "ar"} height="100%" rowSize={38} theme="compact" />
      {/if}
    </main>
  </div>
</div>
