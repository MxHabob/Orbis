<!-- General ledger account management page. -->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import { api } from "$lib/api";
import { Button } from "@orbis/ui/button";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui/select";
import { AdjustmentsHorizontal, Grid, Plus } from "@orbis/icons";
import { getLocale } from "$lib/paraglide/runtime";
import { formatCurrency } from "$lib/core/i18n";
import { fade } from "svelte/transition";
import LedgerCreateSheet from "$lib/features/finance/components/ledger-create-sheet.svelte";

import { DataTable } from "$lib/tables";
import { cashAccounts as tDef, withIndex } from "$lib/tables/definitions";

import AccountTree from "$lib/features/finance/components/account-tree.svelte";


let { data }: { data: { orgId: string } } = $props();

// ── State ──────────────────────────────────────────────────────────────
let allAccounts = $state<any[]>([]);
let childAccounts = $state<any[]>([]);
let baseCurrency = $state("YER");
let isLoading = $state(true);
let isAddDialogOpen = $state(false);
let viewMode = $state<"grid" | "tree">("grid");
let searchQuery = $state("");
let activeFilter = $state<UniversalFilter>({
  logic: "AND",
  conditions: [],
  groups: [],
});

function handleSuccess() {
  isAddDialogOpen = false;
  loadData();
}

// ── Data Loading ────────────────────────────────────────────────────────

async function loadData() {
  isLoading = true;
  try {
    const [accounts, currencies] = await Promise.all([
      api.listAccounts(),
      api.listCurrencies(),
    ]);
    allAccounts = accounts;
    childAccounts = accounts.filter(
      (account: any) =>
        !accounts.some((candidate: any) => candidate.parent_id === account.id),
    );
    const base = currencies.find((c: any) => (c as any).is_base === 1 || (c as any).is_base === true);
    if (base) baseCurrency = base.code;
  } catch (e) {
    console.error("Error loading accounts:", e);
  } finally {
    isLoading = false;
  }
}

// Build hierarchical categories for AccountTree
const treeCategories = $derived.by(() => {
  const accountTypes = ["asset", "liability", "equity", "revenue", "expense"];
  const typeLabels: Record<string, string> = {
    asset: getLocale() === "ar" ? "الأصول" : "Assets",
    liability: getLocale() === "ar" ? "الالتزامات" : "Liabilities",
    equity: getLocale() === "ar" ? "حقوق الملكية" : "Equity",
    revenue: getLocale() === "ar" ? "الإيرادات" : "Revenue",
    expense: getLocale() === "ar" ? "المصروفات" : "Expenses",
  };

  function buildTree(parentId: string | null, type: string): any[] {
    return allAccounts
      .filter((a) => {
        const aType = String(a.type || a.account_type || "asset").toLowerCase();
        const matchesParent = parentId ? a.parent_id === parentId : !a.parent_id;
        return matchesParent && (parentId ? true : aType === type);
      })
      .map((node) => ({
        ...node,
        name: node.name_ar || node.name || "—",
        children: buildTree(node.id, type),
      }));
  }

  return accountTypes.map((type) => ({
    id: type,
    name: typeLabels[type] || type,
    accounts: buildTree(null, type),
  }));
});

onMount(() => {
  if (data.orgId) loadData();
});

// ── Helpers ─────────────────────────────────────────────────────────────

function getAccountTypeLabel(account: any) {
  const type = String(
    account.type || account.item_type || account.sub_type || "asset",
  ).toLowerCase();
  const managedTypes: Record<string, string> = {
    partner: getLocale() === "ar" ? "شريك" : "Partner",
    investor: getLocale() === "ar" ? "مستثمر" : "Investor",
    participant: getLocale() === "ar" ? "مشارك" : "Participant",
    shareholder: getLocale() === "ar" ? "مساهم" : "Shareholder",
    member: getLocale() === "ar" ? "عضو" : "Member",
    supplier: getLocale() === "ar" ? "مورد" : "Supplier",
    vendor: getLocale() === "ar" ? "مورد" : "Supplier",
    customer: getLocale() === "ar" ? "عميل" : "Customer",
    employee: getLocale() === "ar" ? "موظف" : "Employee",
    cash: getLocale() === "ar" ? "صندوق" : "Cash",
    bank: getLocale() === "ar" ? "بنك" : "Bank",
  };
  if (managedTypes[type]) return managedTypes[type];
  if (type === "liability") return m.finance_type_liability();
  if (type === "equity") return "Equity";
  if (type === "revenue") return m.finance_type_revenue();
  if (type === "expense") return m.finance_type_expense();
  return m.finance_type_asset();
}

// ── Grid Sources ────────────────────────────────────────────────────────

const availableCurrencies = $derived(
  [
    ...new Set(
      childAccounts.map((account) => account.currency || baseCurrency),
    ),
  ].sort(),
);

import { applyFilter } from "@orbis/ui/filter-builder";
import type { UniversalFilter } from "@orbis/ui/filter-builder";
import { accountFilterFields } from "$lib/tables/definitions/filter-configs";


const filteredAccounts = $derived(
  applyFilter(
    childAccounts.filter((account) => {
      const query = searchQuery.trim().toLowerCase();
      const name = String(account.name_ar || account.name || "").toLowerCase();
      const code = String(account.code || "").toLowerCase();
      return !query || name.includes(query) || code.includes(query);
    }),
    activeFilter,
  ),
);

const treasurySource = $derived(
  withIndex(
    filteredAccounts.map((account) => ({
      ...account,
      name: account.name_ar || account.name || "—",
      treasury_type: getAccountTypeLabel(account),
      balance: formatCurrency(
        Number(account.balances?.[account.currency] || 0),
        account.currency || baseCurrency,
      ),
      currency_code: account.currency || baseCurrency,
      status: account.is_active === 1 ? "active" : "inactive",
    })),
  ),
);

const treasuryCols = $derived(
  tDef.columns({
    idx: "#",
    name: m.common_col_name(),
    type: m.common_word_type(),
    balance: m.finance_col_balance(),
    currency: "Currency",
  }),
);

const rtl = $derived(getLocale() === "ar");

function exportAccounts() {
  const rows = treasurySource.map((account) => [
    account.code,
    account.name,
    account.treasury_type,
    account.balance,
    account.currency_code,
    account.status,
  ]);
  const csv = [
    ["Code", "Name", "Type", "Balance", "Currency", "Status"],
    ...rows,
  ]
    .map((row) =>
      row
        .map((value) => `"${String(value ?? "").replaceAll('"', '""')}"`)
        .join(","),
    )
    .join("\n");
  const url = URL.createObjectURL(
    new Blob(["\ufeff" + csv], { type: "text/csv;charset=utf-8" }),
  );
  const link = document.createElement("a");
  link.href = url;
  link.download = "orbis-accounts.csv";
  link.click();
  URL.revokeObjectURL(url);
}
</script>

<div class="flex h-full min-h-0 w-full flex-col overflow-hidden p-6 font-sans">
  <div class="flex min-h-0 w-full flex-1 flex-col gap-5" in:fade={{ duration: 180 }}>
    <header class="flex shrink-0 items-center justify-between">
      <div class="flex items-center gap-1.5 bg-muted/40 p-0.5 rounded-lg border border-border/40">
        <Button
          variant={viewMode === "grid" ? "default" : "ghost"}
          size="sm"
          class="h-7 text-xs font-semibold px-2.5 gap-1.5"
          onclick={() => viewMode = "grid"}
        >
          <Grid size={13} />
          <span>جدول الحسابات</span>
        </Button>
        <Button
          variant={viewMode === "tree" ? "default" : "ghost"}
          size="sm"
          class="h-7 text-xs font-semibold px-2.5 gap-1.5"
          onclick={() => viewMode = "tree"}
        >
          <Network size={13} />
          <span>شجرة الحسابات</span>
        </Button>
      </div>

      <Button variant="outline" class="h-9 px-3 text-xs font-bold shrink-0" onclick={() => { isAddDialogOpen = true; }}>
        <Plus size={14} class="me-1.5" />
        {m.finance_new_cash_account_button()}
      </Button>
    </header>

    <div class="flex min-h-0 flex-1 flex-col overflow-hidden">
      {#if viewMode === "grid"}
        <DataTable
          columns={treasuryCols}
          source={treasurySource}
          loading={isLoading}
          searchable
          searchPlaceholder="ابحث باسم الحساب أو رقمه..."
          bind:searchValue={searchQuery}
          filterable={true}
          filterFields={accountFilterFields}
          bind:filterValue={activeFilter}
          emptyIcon={Landmark}
          emptyTitle={m.finance_no_ledgers()}
          {rtl}
          refreshable
          onRefresh={loadData}
          exportable
          onExport={exportAccounts}
        />
      {:else}
        <div class="flex h-full min-h-0 flex-col overflow-y-auto card-tactile bg-card p-4 rounded-xl">
          <AccountTree categories={treeCategories} />
        </div>
      {/if}
    </div>
  </div>
</div>

<LedgerCreateSheet bind:open={isAddDialogOpen} orgId={data.orgId} onSuccess={handleSuccess} />
