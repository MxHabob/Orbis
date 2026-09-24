<!-- General ledger account management component. -->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import { api } from "$lib/api";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Grid, Plus, BuildingLibrary, BuildingOffice, Refresh, Download, Search } from "@orbis/icons";
import { getLocale } from "$lib/paraglide/runtime";
import { formatCurrency } from "$lib/core/i18n";
import { fade } from "svelte/transition";
import LedgerCreateSheet from "$lib/features/finance/components/ledger-create-sheet.svelte";
import { OrbisGrid, type OrbisColumnDef } from "@orbis/svelte-grid";
import AccountTree from "$lib/features/finance/components/account-tree.svelte";
import { FilterTrigger, type UniversalFilter } from "@orbis/ui/filter-builder";
import { accountFilterFields } from "$lib/core/filter-configs";

let { orgId }: { orgId: string } = $props();

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
  if (orgId) loadData();
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

const treasurySource = $derived(
  childAccounts.map((account, i) => ({
    ...account,
    __idx: i + 1,
    name: account.name_ar || account.name || "—",
    treasury_type: getAccountTypeLabel(account),
    balance: formatCurrency(
      Number(account.balances?.[account.currency] || 0),
      account.currency || baseCurrency,
    ),
    currency_code: account.currency || baseCurrency,
    status: account.is_active === 1 ? "active" : "inactive",
  })),
);

const treasuryCols: OrbisColumnDef[] = [
  { id: "__idx", prop: "__idx", header: "#", name: "#", width: 48, size: 48, readonly: true, pin: "left", align: "center" },
  { id: "code", prop: "code", header: "Code", name: "Code", width: 110, size: 110, readonly: true, cellClass: "font-mono" },
  { id: "name", prop: "name", header: m.common_col_name(), name: m.common_col_name(), width: 220, size: 220, readonly: true, cellClass: "font-medium" },
  { id: "treasury_type", prop: "treasury_type", header: m.common_word_type(), name: m.common_word_type(), width: 130, size: 130, readonly: true },
  { id: "balance", prop: "balance", header: m.finance_col_balance(), name: m.finance_col_balance(), width: 140, size: 140, readonly: true, align: "end", cellClass: "font-mono font-bold" },
  { id: "currency_code", prop: "currency_code", header: "Currency", name: "Currency", width: 90, size: 90, readonly: true, align: "center" },
  { id: "status", prop: "status", header: "Status", name: "Status", width: 100, size: 100, type: "badge", align: "center", readonly: true },
];

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
  <div class="flex min-h-0 w-full flex-1 flex-col gap-4" in:fade={{ duration: 180 }}>
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
          <BuildingOffice size={13} />
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
        <div class="flex flex-col h-full gap-3">
          <div class="flex items-center justify-between gap-2 shrink-0">
            <div class="relative flex-1 max-w-xs">
              <Search
                size={14}
                class="absolute inset-s-2.5 top-1/2 -translate-y-1/2 text-muted-foreground pointer-events-none"
              />
              <Input
                type="text"
                placeholder="ابحث باسم الحساب أو رقمه..."
                bind:value={searchQuery}
                class="h-8 ps-8 text-xs bg-card border-border/60"
              />
            </div>
            <div class="flex items-center gap-1.5">
              <FilterTrigger
                bind:filter={activeFilter}
                fields={accountFilterFields}
              />
              <Button variant="ghost" size="icon-sm" onclick={loadData} class="text-muted-foreground">
                <Refresh size={14} class={isLoading ? "animate-spin" : ""} />
              </Button>
              <Button variant="ghost" size="icon-sm" onclick={exportAccounts} class="text-muted-foreground">
                <Download size={14} />
              </Button>
            </div>
          </div>
          <div class="flex-1 min-h-0">
            <OrbisGrid
              columns={treasuryCols}
              source={treasurySource}
              loading={isLoading}
              clientQuery
              search={searchQuery}
              filter={activeFilter}
              {rtl}
              readonly
              emptyTitle={m.finance_no_ledgers()}
            />
          </div>
        </div>
      {:else}
        <div class="flex h-full min-h-0 flex-col overflow-y-auto card-tactile bg-card p-4 rounded-xl">
          <AccountTree categories={treeCategories} />
        </div>
      {/if}
    </div>
  </div>
</div>

<LedgerCreateSheet bind:open={isAddDialogOpen} {orgId} onSuccess={handleSuccess} />
