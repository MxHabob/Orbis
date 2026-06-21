<!--
@component FinanceLedgerPage
Account Statement / Trial Balance (كشف حساب تفصيلي)
Groups movements by currency (YER, SAR, USD, AED) and lists account codes and names.
Provides All, Single, and Custom selection mode with interactive Accounts Tree sidebar.
-->
<script lang="ts">
import { onMount } from "svelte";
import { commands } from "$lib/services/ipc";
import { goto } from "$app/navigation";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@orbis/ui-svelte";
import { Button } from "@orbis/ui-svelte/button";
import { Input } from "@orbis/ui-svelte/input";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import { toast } from "svelte-sonner";
import {
  Printer,
  ChevronRight,
  ChevronDown,
  RotateCw,
  LogOut,
  Save,
  Folder,
  FolderOpen,
  CheckSquare,
  Square,
  FileText,
} from "lucide-svelte";
import { getLocale, deLocalizeUrl } from "$lib/paraglide/runtime";
import { formatCurrency, formatDate } from "$lib/i18n-utils";
import { fade, slide } from "svelte/transition";
import * as m from "$lib/paraglide/messages";

let { data }: { data: { orgId: string } } = $props();

// State variables
let ledgers = $state<any[]>([]);
let payments = $state<any[]>([]);
let fees = $state<any[]>([]);
let treasuries = $state<any[]>([]);
let isLoading = $state(true);
let error = $state<string | null>(null);

// Filter states
let activeTab = $state<"all" | "single" | "custom">("all");
let selectedAccountId = $state<string>("");
let selectedCurrency = $state<string>("all");
let periodFilter = $state<string>("all");
let customStartDate = $state<string>(new Date().toISOString().split("T")[0]);
let movementFilter = $state<string>("all"); // 'all' or 'active' (with movements)
let debtorCreditorFilter = $state<string>("all"); // 'all', 'debtor', 'creditor'
let showAllCurrenciesInNotes = $state<boolean>(false);

// Tree state for Custom Tab
let expandedCategories = $state<Record<string, boolean>>({
  asset: true,
  liability: false,
  expense: false,
  revenue: false,
  equity: false,
});
let expandedAccounts = $state<Record<string, boolean>>({});
let selectedTreeAccountIds = $state<Set<string>>(new Set());
let isCoadTreeExpanded = $state(true);

// Sidebar layout state
let isTreeSidebarOpen = $derived(activeTab === "custom");

async function loadData() {
  isLoading = true;
  error = null;
  try {
    const [ledgerList, paymentList, feeList, treasuryList] = await Promise.all([
      commands.finance.getLedgerAccounts(data.orgId),
      commands.finance.getPayments(data.orgId),
      commands.finance.getStudentFees(data.orgId),
      commands.finance.getTreasuries(data.orgId),
    ]);

    ledgers = ledgerList;
    payments = paymentList;
    fees = feeList;
    treasuries = treasuryList;

    if (ledgerList.length > 0) {
      selectedAccountId = ledgerList[0].id;
    }

    // Default select all accounts in tree selection
    const allIds = new Set<string>();
    ledgerList.forEach((acc) => allIds.add(acc.id));
    selectedTreeAccountIds = allIds;
  } catch (e) {
    console.error("Error loading ledger data:", e);
    error = e instanceof Error ? e.message : String(e);
  } finally {
    isLoading = false;
  }
}

onMount(() => {
  if (data.orgId) {
    loadData();

    // Load template preference if saved
    const savedTemplate = localStorage.getItem(`orbis_ledger_template_${data.orgId}`);
    if (savedTemplate) {
      try {
        const filters = JSON.parse(savedTemplate);
        if (filters.selectedCurrency) selectedCurrency = filters.selectedCurrency;
        if (filters.periodFilter) periodFilter = filters.periodFilter;
        if (filters.movementFilter) movementFilter = filters.movementFilter;
        if (filters.debtorCreditorFilter) debtorCreditorFilter = filters.debtorCreditorFilter;
        if (filters.showAllCurrenciesInNotes !== undefined) showAllCurrenciesInNotes = filters.showAllCurrenciesInNotes;
      } catch (err) {
        console.error("Error parsing saved ledger template", err);
      }
    }
  }
});

// Double entry ledger balance calculation
const calculatedLedgerRows = $derived.by(() => {
  const result: Record<string, any> = {};

  // 1. Initialize with database ledger accounts
  for (const acc of ledgers) {
    let debY = 0, credY = 0;
    let debU = 0, credU = 0;
    let debS = 0, credS = 0;

    // Apply opening balance from seed/config
    const ob = Number(acc.opening_balance || 0);
    if (ob > 0) {
      const type = String(acc.item_type || "").toLowerCase();
      const curr = String(acc.currency || "YER").toUpperCase();
      const isDebitType = type === "asset" || type === "expense";
      
      if (curr === "YER") {
        if (isDebitType) debY = ob; else credY = ob;
      } else if (curr === "USD") {
        if (isDebitType) debU = ob; else credU = ob;
      } else if (curr === "SAR") {
        if (isDebitType) debS = ob; else credS = ob;
      }
    }

    result[acc.id] = {
      id: acc.id,
      code: acc.code,
      name: acc.name,
      name_ar: acc.name_ar || acc.name,
      item_type: acc.item_type || "asset",
      parent_id: acc.parent_id,
      debit_yer: debY,
      credit_yer: credY,
      debit_usd: debU,
      credit_usd: credU,
      debit_sar: debS,
      credit_sar: credS,
      debit_aed: 0,
      credit_aed: 0,
      notes: acc.sub_type || "",
    };
  }

  // 2. Perform math for fees (Debit Tuition Receivables YER, Credit corresponding revenue account YER)
  const tuitionReceivablesAccount = ledgers.find(
    (l) => l.code === "1301" || String(l.name_ar).includes("ذمم") || String(l.name).toLowerCase().includes("receivable")
  );
  const arId = tuitionReceivablesAccount?.id;

  for (const fee of fees) {
    const amount = Number(fee.total_amount || 0);
    const revenueAccId = fee.fee_template_id; // fee_template_id stores the credit ledger ID

    // Debit student receivables
    if (arId && result[arId]) {
      result[arId].debit_yer += amount;
    }
    // Credit revenue classification account
    if (revenueAccId && result[revenueAccId]) {
      result[revenueAccId].credit_yer += amount;
    }
  }

  // 3. Perform math for payments (Debit treasury vault, Credit student receivables)
  for (const pay of payments) {
    const amount = Number(pay.amount || 0);
    const curr = String(pay.currency || "YER").toUpperCase();

    // Debit target treasury's ledger account
    const treasury = treasuries.find((t) => t.id === pay.treasury_id);
    const treasuryLedgerId = treasury?.ledger_account_id;
    if (treasuryLedgerId && result[treasuryLedgerId]) {
      if (curr === "YER") {
        result[treasuryLedgerId].debit_yer += amount;
      } else if (curr === "USD") {
        result[treasuryLedgerId].debit_usd += amount;
      } else if (curr === "SAR") {
        result[treasuryLedgerId].debit_sar += amount;
      }
    }

    // Credit student receivables
    if (arId && result[arId]) {
      if (curr === "YER") {
        result[arId].credit_yer += amount;
      } else if (curr === "USD") {
        result[arId].credit_usd += amount;
      } else if (curr === "SAR") {
        result[arId].credit_sar += amount;
      }
    }
  }

  // 4. Rollup child balances to parent accounts recursively
  const visited = new Set<string>();
  function rollupNode(nodeId: string) {
    if (visited.has(nodeId)) return;
    visited.add(nodeId);

    const children = Object.values(result).filter((acc: any) => acc.parent_id === nodeId);
    for (const child of children) {
      rollupNode(child.id);
      
      result[nodeId].debit_yer += child.debit_yer;
      result[nodeId].credit_yer += child.credit_yer;
      result[nodeId].debit_sar += child.debit_sar;
      result[nodeId].credit_sar += child.credit_sar;
      result[nodeId].debit_usd += child.debit_usd;
      result[nodeId].credit_usd += child.credit_usd;
      result[nodeId].debit_aed += child.debit_aed;
      result[nodeId].credit_aed += child.credit_aed;
    }
  }

  for (const acc of Object.values(result)) {
    if (!acc.parent_id) {
      rollupNode(acc.id);
    }
  }

  // Convert map to list
  return Object.values(result);
});

// Categorized tree representation of all accounts (used for Custom Selection sidebar)
const treeCategories = $derived.by(() => {
  const cats: Record<string, { id: string; name_ar: string; name_en: string; accounts: any[] }> = {
    asset: { id: "asset", name_ar: "الأصول (1)", name_en: "Assets", accounts: [] },
    liability: { id: "liability", name_ar: "الخصوم (2)", name_en: "Liabilities", accounts: [] },
    expense: { id: "expense", name_ar: "المصروفات (3)", name_en: "Expenses", accounts: [] },
    revenue: { id: "revenue", name_ar: "الإيرادات (4)", name_en: "Revenues", accounts: [] },
    equity: { id: "equity", name_ar: "حقوق الملكية (5)", name_en: "Equity", accounts: [] },
  };

  const flatLists: Record<string, any[]> = {
    asset: [],
    liability: [],
    expense: [],
    revenue: [],
    equity: [],
  };

  for (const row of calculatedLedgerRows) {
    const type = String(row.item_type).toLowerCase();
    if (flatLists[type]) {
      flatLists[type].push(row);
    } else {
      flatLists.asset.push(row);
    }
  }

  function buildTree(flatAccounts: any[]) {
    const map: Record<string, any> = {};
    for (const acc of flatAccounts) {
      map[acc.id] = { ...acc, children: [] };
    }

    const roots: any[] = [];
    for (const acc of flatAccounts) {
      const node = map[acc.id];
      if (acc.parent_id && map[acc.parent_id]) {
        map[acc.parent_id].children.push(node);
      } else {
        roots.push(node);
      }
    }
    return roots;
  }

  for (const key of Object.keys(cats)) {
    cats[key].accounts = buildTree(flatLists[key]);
  }

  return Object.values(cats);
});

// Tree selection helpers
function toggleCategorySelect(catId: string, accounts: any[]) {
  const categoryFlatAccounts = calculatedLedgerRows.filter(r => String(r.item_type).toLowerCase() === catId);
  const isAllSelected = categoryFlatAccounts.every((a) => selectedTreeAccountIds.has(a.id));
  const newSet = new Set(selectedTreeAccountIds);
  
  if (isAllSelected) {
    categoryFlatAccounts.forEach((a) => newSet.delete(a.id));
  } else {
    categoryFlatAccounts.forEach((a) => newSet.add(a.id));
  }
  selectedTreeAccountIds = newSet;
}

function toggleAccountSelect(accId: string) {
  const newSet = new Set(selectedTreeAccountIds);
  if (newSet.has(accId)) {
    newSet.delete(accId);
  } else {
    newSet.add(accId);
  }
  selectedTreeAccountIds = newSet;
}

function toggleCategoryExpand(catId: string) {
  expandedCategories = {
    ...expandedCategories,
    [catId]: !expandedCategories[catId],
  };
}

function toggleAccountExpand(accountId: string) {
  expandedAccounts = {
    ...expandedAccounts,
    [accountId]: !expandedAccounts[accountId],
  };
}

function toggleParentAccountSelect(node: any) {
  const newSet = new Set(selectedTreeAccountIds);
  const descendants: string[] = [];
  function getDescendants(n: any) {
    descendants.push(n.id);
    if (n.children) {
      n.children.forEach(getDescendants);
    }
  }
  getDescendants(node);

  const areAllDescendantsSelected = descendants.every(id => selectedTreeAccountIds.has(id));

  if (areAllDescendantsSelected) {
    descendants.forEach(id => newSet.delete(id));
  } else {
    descendants.forEach(id => newSet.add(id));
  }
  selectedTreeAccountIds = newSet;
}

function getAccountCheckboxState(node: any): "checked" | "partial" | "unchecked" {
  const descendants: string[] = [];
  function getDescendants(n: any) {
    descendants.push(n.id);
    if (n.children) {
      n.children.forEach(getDescendants);
    }
  }
  getDescendants(node);

  const selectedCount = descendants.filter(id => selectedTreeAccountIds.has(id)).length;
  if (selectedCount === descendants.length) {
    return "checked";
  } else if (selectedCount > 0) {
    return "partial";
  } else {
    return "unchecked";
  }
}

// Active filtered rows to display in the main ledger table
const filteredLedgerRows = $derived.by(() => {
  return calculatedLedgerRows.filter((row) => {
    // 1. Account Filtering by Tab Mode
    if (activeTab === "single") {
      if (row.id !== selectedAccountId) return false;
    } else if (activeTab === "custom") {
      if (!selectedTreeAccountIds.has(row.id)) return false;
    }

    // 2. Currency Filter
    if (selectedCurrency !== "all") {
      const c = selectedCurrency.toUpperCase();
      if (c === "YER" && row.debit_yer === 0 && row.credit_yer === 0) return false;
      if (c === "SAR" && row.debit_sar === 0 && row.credit_sar === 0) return false;
      if (c === "USD" && row.debit_usd === 0 && row.credit_usd === 0) return false;
      if (c === "AED" && row.debit_aed === 0 && row.credit_aed === 0) return false;
    }

    // 3. Movement Filter (with movements only)
    if (movementFilter === "active") {
      const hasMovement =
        row.debit_yer !== 0 ||
        row.credit_yer !== 0 ||
        row.debit_sar !== 0 ||
        row.credit_sar !== 0 ||
        row.debit_usd !== 0 ||
        row.credit_usd !== 0 ||
        row.debit_aed !== 0 ||
        row.credit_aed !== 0;
      if (!hasMovement) return false;
    }

    // 4. Debtor / Creditor Filter
    if (debtorCreditorFilter === "debtor") {
      const isDebtor =
        row.debit_yer > row.credit_yer ||
        row.debit_sar > row.credit_sar ||
        row.debit_usd > row.credit_usd ||
        row.debit_aed > row.credit_aed;
      if (!isDebtor) return false;
    } else if (debtorCreditorFilter === "creditor") {
      const isCreditor =
        row.credit_yer > row.debit_yer ||
        row.credit_sar > row.debit_sar ||
        row.credit_usd > row.debit_usd ||
        row.credit_aed > row.debit_aed;
      if (!isCreditor) return false;
    }

    return true;
  });
});

// Sum totals for footer
const totals = $derived.by(() => {
  let debY = 0, credY = 0;
  let debS = 0, credS = 0;
  let debU = 0, credU = 0;
  let debA = 0, credA = 0;

  for (const r of filteredLedgerRows) {
    debY += r.debit_yer;
    credY += r.credit_yer;
    debS += r.debit_sar;
    credS += r.credit_sar;
    debU += r.debit_usd;
    credU += r.credit_usd;
    debA += r.debit_aed;
    credA += r.credit_aed;
  }

  return {
    debY,
    credY,
    debS,
    credS,
    debU,
    credU,
    debA,
    credA,
  };
});

// Actions
function saveTemplate() {
  const filters = {
    selectedCurrency,
    periodFilter,
    movementFilter,
    debtorCreditorFilter,
    showAllCurrenciesInNotes,
  };
  localStorage.setItem(`orbis_ledger_template_${data.orgId}`, JSON.stringify(filters));
  toast.success(getLocale() === "ar" ? "تم حفظ قالب كشف الحساب بنجاح!" : "Ledger statement template saved successfully!");
}

function handlePrint() {
  window.print();
}

function exitPage() {
  goto(`/${data.orgId}/finance`);
}
</script>

<div class="h-full w-full overflow-hidden flex flex-col font-sans bg-background text-foreground" dir={getLocale() === "ar" ? "rtl" : "ltr"}>
  
  <!-- Page Header -->
  <div class="p-5 border-b border-border bg-card/30 flex items-center justify-between shrink-0 no-print select-none">
    <div class="text-start">
      <h2 class="text-sm font-bold text-foreground uppercase tracking-wider flex items-center gap-2">
        <FileText size={16} class="text-primary" />
        <span>{m.ledger_title()}</span>
      </h2>
      <p class="text-[10px] text-muted-foreground mt-0.5">
        {m.ledger_subtitle()}
      </p>
    </div>

    <!-- Mode Selector Tabs -->
    <div class="flex border border-border bg-muted/50 rounded-lg p-0.5 text-[10px] font-bold">
      <button
        onclick={() => activeTab = "all"}
        class="px-3 py-1 rounded-md transition-all duration-150 {activeTab === 'all' ? 'bg-card text-foreground shadow-sm' : 'text-muted-foreground hover:text-foreground'}"
      >
        {m.ledger_tab_all()}
      </button>
      <button
        onclick={() => activeTab = "single"}
        class="px-3 py-1 rounded-md transition-all duration-150 {activeTab === 'single' ? 'bg-card text-foreground shadow-sm' : 'text-muted-foreground hover:text-foreground'}"
      >
        {m.ledger_tab_one()}
      </button>
      <button
        onclick={() => activeTab = "custom"}
        class="px-3 py-1 rounded-md transition-all duration-150 {activeTab === 'custom' ? 'bg-card text-foreground shadow-sm' : 'text-muted-foreground hover:text-foreground'}"
      >
        {m.ledger_tab_custom()}
      </button>
    </div>
  </div>

  <!-- Main Filter Controls Box -->
  <div class="p-5 bg-card/15 border-b border-border/80 grid grid-cols-1 lg:grid-cols-4 gap-6 shrink-0 no-print select-none text-start">
    
    <!-- Right Side (Renders on right in RTL): Accounts Tree Selection Box (spans 1 column) -->
    <div 
      class="bg-background border border-border rounded-lg p-3 flex flex-col overflow-hidden shadow-sm lg:col-span-1 self-start"
      style="height: 220px; max-height: 220px;"
    >
      <!-- Tree Header -->
      <div class="flex items-center justify-between border-b border-border pb-1.5 mb-2 shrink-0">
        <h3 class="text-[10px] font-bold text-foreground uppercase tracking-wider flex items-center gap-1.5">
          <FolderOpen size={12} class="text-primary" />
          <span>{m.ledger_tree_title()}</span>
        </h3>
        <span class="text-[8px] px-1 rounded bg-muted font-bold text-muted-foreground border border-border/40">
          {selectedTreeAccountIds.size} / {calculatedLedgerRows.length}
        </span>
      </div>

      <!-- Scrollable Tree content -->
      <div class="flex-1 overflow-y-auto flex flex-col gap-2.5 text-[10px]">
        {#snippet renderNodes(nodes, depth)}
          {#each nodes as node (node.id)}
            {@const isParent = node.children && node.children.length > 0}
            {@const isExpanded = expandedAccounts[node.id] !== false}
            {@const state = getAccountCheckboxState(node)}
            <div class="flex flex-col gap-1 select-none" style="{getLocale() === 'ar' ? 'padding-right' : 'padding-left'}: {depth * 10}px">
              <div class="flex items-center justify-between hover:bg-accent/10 px-1.5 py-0.5 rounded text-[9px] text-muted-foreground hover:text-foreground w-full text-start">
                <button
                  type="button"
                  onclick={() => isParent ? toggleAccountExpand(node.id) : toggleAccountSelect(node.id)}
                  class="truncate flex-1 font-medium text-start flex items-center gap-1 text-[9px] text-foreground"
                >
                  {#if isParent}
                    {#if isExpanded}
                      <ChevronDown size={10} class="text-muted-foreground shrink-0" />
                      <FolderOpen size={10} class="text-primary/80 shrink-0" />
                    {:else}
                      <ChevronRight size={10} class="text-muted-foreground {getLocale() === 'ar' ? 'rotate-180' : ''} shrink-0" />
                      <Folder size={10} class="text-primary/80 shrink-0" />
                    {/if}
                  {:else}
                    <FileText size={10} class="text-muted-foreground/60 shrink-0" />
                  {/if}
                  <span class="truncate">{node.name_ar}</span>
                </button>
                <div class="flex items-center gap-1.5 shrink-0">
                  <span class="font-mono text-[8px] font-bold bg-muted/30 px-1 rounded border border-border/20">{node.code}</span>
                  <button
                    type="button"
                    onclick={() => isParent ? toggleParentAccountSelect(node) : toggleAccountSelect(node.id)}
                    class="p-0.5 text-muted-foreground hover:text-foreground shrink-0"
                  >
                    {#if state === 'checked'}
                      <CheckSquare size={10} class="text-primary" />
                    {:else if state === 'partial'}
                      <div class="h-2.5 w-2.5 bg-primary/20 border border-primary rounded flex items-center justify-center">
                        <span class="h-0.5 w-0.5 bg-primary rounded-sm"></span>
                      </div>
                    {:else}
                      <Square size={10} />
                    {/if}
                  </button>
                </div>
              </div>
              {#if isParent && isExpanded}
                <div class="border-r border-border/40 mr-1.5 my-0.5" transition:slide={{ duration: 150 }}>
                  {@render renderNodes(node.children, depth + 1)}
                </div>
              {/if}
            </div>
          {/each}
        {/snippet}

        <!-- Top level Root Node: الشجرة المحاسبية -->
        <div class="flex flex-col gap-1">
          <!-- Root Row -->
          <div class="flex items-center justify-between hover:bg-accent/15 px-1.5 py-0.5 rounded cursor-pointer font-bold">
            <button
              type="button"
              class="flex items-center gap-1 text-foreground hover:text-primary flex-1 text-start"
              onclick={() => isCoadTreeExpanded = !isCoadTreeExpanded}
            >
              {#if isCoadTreeExpanded}
                <ChevronDown size={12} class="text-muted-foreground" />
                <FolderOpen size={12} class="text-primary" />
              {:else}
                <ChevronRight size={12} class="text-muted-foreground {getLocale() === 'ar' ? 'rotate-180' : ''}" />
                <Folder size={12} class="text-primary" />
              {/if}
              <span>{getLocale() === "ar" ? "الشجرة المحاسبية" : "Chart of Accounts"}</span>
            </button>

            <!-- Chart of Accounts Checkbox -->
            <button
              type="button"
              onclick={() => {
                const newSet = new Set<string>();
                if (selectedTreeAccountIds.size < calculatedLedgerRows.length) {
                  calculatedLedgerRows.forEach((r) => newSet.add(r.id));
                }
                selectedTreeAccountIds = newSet;
              }}
              class="text-muted-foreground hover:text-foreground shrink-0 p-0.5"
            >
              {#if selectedTreeAccountIds.size === calculatedLedgerRows.length}
                <CheckSquare size={12} class="text-primary" />
              {:else if selectedTreeAccountIds.size > 0}
                <div class="h-3 w-3 bg-primary/20 border border-primary rounded flex items-center justify-center">
                  <span class="h-1 w-1 bg-primary rounded-sm"></span>
                </div>
              {:else}
                <Square size={12} />
              {/if}
            </button>
          </div>

          <!-- Categories List (Indented under Chart of Accounts) -->
          {#if isCoadTreeExpanded}
            <div class="border-r border-border/40 mr-1.5 my-0.5 flex flex-col gap-2" transition:slide={{ duration: 150 }}>
              {#each treeCategories as cat (cat.id)}
                {@const catFlatAccounts = calculatedLedgerRows.filter(r => String(r.item_type).toLowerCase() === cat.id)}
                <div class="flex flex-col gap-1 ps-2">
                  <!-- Category Header Row -->
                  <div class="flex items-center justify-between hover:bg-accent/15 px-1.5 py-0.5 rounded cursor-pointer font-bold">
                    <button
                      type="button"
                      class="flex items-center gap-1 text-foreground hover:text-primary flex-1 text-start"
                      onclick={() => toggleCategoryExpand(cat.id)}
                    >
                      {#if expandedCategories[cat.id]}
                        <ChevronDown size={12} class="text-muted-foreground" />
                        <FolderOpen size={12} class="text-primary" />
                      {:else}
                        <ChevronRight size={12} class="text-muted-foreground {getLocale() === 'ar' ? 'rotate-180' : ''}" />
                        <Folder size={12} class="text-primary" />
                      {/if}
                      <span>{getLocale() === "ar" ? cat.name_ar : cat.name_en}</span>
                    </button>

                    <!-- Category Checkbox -->
                    <button
                      type="button"
                      onclick={() => toggleCategorySelect(cat.id, cat.accounts)}
                      class="text-muted-foreground hover:text-foreground shrink-0 p-0.5"
                    >
                      {#if catFlatAccounts.every((a) => selectedTreeAccountIds.has(a.id))}
                        <CheckSquare size={12} class="text-primary" />
                      {:else if catFlatAccounts.some((a) => selectedTreeAccountIds.has(a.id))}
                        <div class="h-3 w-3 bg-primary/20 border border-primary rounded flex items-center justify-center">
                          <span class="h-1 w-1 bg-primary rounded-sm"></span>
                        </div>
                      {:else}
                        <Square size={12} />
                      {/if}
                    </button>
                  </div>

                  <!-- Children Accounts (Visible when Expanded) -->
                  {#if expandedCategories[cat.id]}
                    <div class="border-r border-border/40 mr-2 my-0.5" transition:slide={{ duration: 150 }}>
                      {#if cat.accounts.length === 0}
                        <span class="text-[8px] text-muted-foreground ps-2 py-0.5">لا توجد حسابات مضافة</span>
                      {:else}
                        {@render renderNodes(cat.accounts, 0)}
                      {/if}
                    </div>
                  {/if}
                </div>
              {/each}
            </div>
          {/if}
        </div>
      </div>
    </div>

    <!-- Left Side (Renders on left in RTL): Filter inputs (spans 3 columns) -->
    <div class="lg:col-span-3 flex flex-col gap-4 justify-between">
      <div class="flex flex-wrap gap-4 items-center">
        <!-- 1. Account Dropdown (Enabled only for 'single' mode) -->
        <div class="flex flex-col gap-1 text-[10px] font-bold text-muted-foreground uppercase">
          <span>{m.ledger_tab_one()}</span>
          <select
            bind:value={selectedAccountId}
            disabled={activeTab !== "single" || isLoading}
            class="bg-background text-foreground border border-border rounded-md px-3 py-1.5 text-xs outline-none focus:ring-1 focus:ring-ring min-w-56 disabled:opacity-50"
          >
            {#if isLoading}
              <option>{m.loading()}</option>
            {:else}
              {#each ledgers as l (l.id)}
                <option value={l.id}>{l.code} - {l.name_ar || l.name}</option>
              {/each}
            {/if}
          </select>
        </div>

        <!-- 2. Currency Selector -->
        <div class="flex flex-col gap-1 text-[10px] font-bold text-muted-foreground uppercase">
          <span>{m.ledger_currency()}</span>
          <select
            bind:value={selectedCurrency}
            class="bg-background text-foreground border border-border rounded-md px-3 py-1.5 text-xs outline-none focus:ring-1 focus:ring-ring min-w-32"
          >
            <option value="all">{m.ledger_all_currencies()}</option>
            <option value="YER">YER (ريال يمني)</option>
            <option value="SAR">SAR (سعودي)</option>
            <option value="USD">USD (دولار)</option>
            <option value="AED">AED (درهم)</option>
          </select>
        </div>

        <!-- 3. Period selector & Date picker -->
        <div class="flex flex-col gap-1 text-[10px] font-bold text-muted-foreground uppercase">
          <span>{m.metrics_period()}</span>
          <div class="flex gap-2">
            <select
              bind:value={periodFilter}
              class="bg-background text-foreground border border-border rounded-md px-3 py-1.5 text-xs outline-none focus:ring-1 focus:ring-ring min-w-28"
            >
              <option value="all">{m.ledger_period_all()}</option>
              <option value="day">{m.ledger_period()}</option>
              <option value="week">{m.ledger_period_week()}</option>
              <option value="month">{m.ledger_period_month()}</option>
              <option value="year">{m.ledger_period_year()}</option>
            </select>
            <Input
              type="date"
              bind:value={customStartDate}
              class="input-sunken py-1 px-2.5 text-xs w-36"
            />
          </div>
        </div>

        <!-- 4. Account Type Filters -->
        <div class="flex flex-col gap-1 text-[10px] font-bold text-muted-foreground uppercase">
          <span>حالة الحركة</span>
          <select
            bind:value={movementFilter}
            class="bg-background text-foreground border border-border rounded-md px-3 py-1.5 text-xs outline-none focus:ring-1 focus:ring-ring min-w-32"
          >
            <option value="all">كافة الحسابات</option>
            <option value="active">المتحركة أرصدتهم</option>
          </select>
        </div>

        <!-- 5. Balance Type -->
        <div class="flex flex-col gap-1 text-[10px] font-bold text-muted-foreground uppercase">
          <span>نوع الرصيد</span>
          <select
            bind:value={debtorCreditorFilter}
            class="bg-background text-foreground border border-border rounded-md px-3 py-1.5 text-xs outline-none focus:ring-1 focus:ring-ring min-w-32"
          >
            <option value="all">المدينون والدائنون</option>
            <option value="debtor">المدينون فقط</option>
            <option value="creditor">الدائنون فقط</option>
          </select>
        </div>
      </div>

      <!-- Template Save and Checkbox Row -->
      <div class="flex items-center gap-4 border-t border-border/30 pt-3 select-none">
        <label class="flex items-center gap-2 text-xs font-medium cursor-pointer select-none">
          <input
            type="checkbox"
            bind:checked={showAllCurrenciesInNotes}
            class="rounded border-border bg-background text-primary focus:ring-1 focus:ring-ring cursor-pointer h-4 w-4"
          />
          <span>{m.ledger_show_notes_balance()}</span>
        </label>

        <Button
          variant="ghost"
          class="text-[10px] h-8 px-3 border border-border flex items-center gap-1.5 font-bold"
          onclick={saveTemplate}
        >
          <Save size={12} class="text-primary" />
          <span>{m.ledger_save_template()}</span>
        </Button>
      </div>
    </div>
  </div>

  <!-- Content Split Layout (Table on left, Tree Selection Sidebar on right if Custom selection active) -->
  <div class="flex-1 w-full overflow-hidden flex relative">
    
    <!-- Table Display Area -->
    <div class="flex-1 h-full overflow-auto p-5 select-none print-full-width">
      
      <!-- Printable Page Header -->
      <div class="hidden print-only mb-6 text-center select-none" dir={getLocale() === "ar" ? "rtl" : "ltr"}>
        <h1 class="text-lg font-bold border-b border-black pb-2">{m.ledger_title()}</h1>
        <div class="flex justify-between text-xs mt-3">
          <span>التاريخ: {formatDate(new Date().toISOString())}</span>
          <span>العملة المحددة: {selectedCurrency === 'all' ? m.ledger_all_currencies() : selectedCurrency}</span>
          <span>الفترة: {periodFilter === 'all' ? m.ledger_period_all() : periodFilter}</span>
        </div>
      </div>

      {#if isLoading}
        <div class="flex flex-col gap-3 py-4">
          <Skeleton class="h-10 w-full" />
          <Skeleton class="h-10 w-full" />
          <Skeleton class="h-10 w-full" />
          <Skeleton class="h-10 w-full" />
        </div>
      {:else if error}
        <div class="flex flex-col items-center justify-center py-16 text-destructive border border-destructive/20 bg-destructive/5 rounded-xl gap-2">
          <h3 class="font-semibold text-xs">{m.finance_error_loading()}</h3>
          <p class="text-[10px] text-muted-foreground">{error}</p>
          <Button variant="outline" onclick={loadData} class="text-[10px] mt-2">{m.error_retry()}</Button>
        </div>
      {:else}
        <!-- Excel Grid Sheet Ledger -->
        <div class="border border-border rounded-xl bg-card/30 overflow-hidden min-w-[900px] print-table" in:fade={{ duration: 150 }}>
          <Table class="border-collapse w-full">
            <TableHeader class="bg-card">
              <!-- Top Currency Header Row -->
              <TableRow class="hover:bg-transparent border-b border-border/80">
                <TableHead rowspan="2" class="w-10 border-r border-border/60 text-center font-bold text-foreground">*</TableHead>
                <TableHead colspan="2" class="border-r border-border/60 text-center font-bold text-foreground bg-primary/5">ريال يمني</TableHead>
                <TableHead colspan="2" class="border-r border-border/60 text-center font-bold text-foreground bg-emerald-500/5">سعودي</TableHead>
                <TableHead colspan="2" class="border-r border-border/60 text-center font-bold text-foreground bg-blue-500/5">دولار</TableHead>
                <TableHead colspan="2" class="border-r border-border/60 text-center font-bold text-foreground bg-purple-500/5">درهم</TableHead>
                <TableHead rowspan="2" class="border-r border-border/60 text-start font-bold text-foreground">{m.ledger_col_name()}</TableHead>
                <TableHead rowspan="2" class="border-r border-border/60 text-center font-bold text-foreground w-28">{m.ledger_col_code()}</TableHead>
                <TableHead rowspan="2" class="text-start font-bold text-foreground w-64">{m.ledger_col_notes()}</TableHead>
              </TableRow>
              <!-- Sub-columns Debit / Credit -->
              <TableRow class="hover:bg-transparent border-b border-border/80 text-[9px] font-bold">
                <TableHead class="text-center border-r border-border/40 w-24 text-red-500">{m.ledger_col_debit()}</TableHead>
                <TableHead class="text-center border-r border-border/60 w-24 text-emerald-500">{m.ledger_col_credit()}</TableHead>
                <TableHead class="text-center border-r border-border/40 w-24 text-red-500">{m.ledger_col_debit()}</TableHead>
                <TableHead class="text-center border-r border-border/60 w-24 text-emerald-500">{m.ledger_col_credit()}</TableHead>
                <TableHead class="text-center border-r border-border/40 w-24 text-red-500">{m.ledger_col_debit()}</TableHead>
                <TableHead class="text-center border-r border-border/60 w-24 text-emerald-500">{m.ledger_col_credit()}</TableHead>
                <TableHead class="text-center border-r border-border/40 w-24 text-red-500">{m.ledger_col_debit()}</TableHead>
                <TableHead class="text-center border-r border-border/60 w-24 text-emerald-500">{m.ledger_col_credit()}</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {#each filteredLedgerRows as row, index (row.id)}
                <TableRow class="hover:bg-accent/10 border-b border-border/40 text-[10px] leading-normal font-medium">
                  <TableCell class="border-r border-border/40 text-center text-muted-foreground font-mono">{index + 1}</TableCell>
                  
                  <!-- YER -->
                  <TableCell class="border-r border-border/40 text-center font-mono {row.debit_yer > 0 ? 'text-foreground font-semibold' : 'text-muted-foreground/30'}">
                    {row.debit_yer > 0 ? formatCurrency(row.debit_yer, "").replace(/YER|USD|SAR|AED/g, '') : "0"}
                  </TableCell>
                  <TableCell class="border-r border-border/60 text-center font-mono {row.credit_yer > 0 ? 'text-foreground font-semibold' : 'text-muted-foreground/30'}">
                    {row.credit_yer > 0 ? formatCurrency(row.credit_yer, "").replace(/YER|USD|SAR|AED/g, '') : "0"}
                  </TableCell>
                  
                  <!-- SAR -->
                  <TableCell class="border-r border-border/40 text-center font-mono {row.debit_sar > 0 ? 'text-foreground font-semibold' : 'text-muted-foreground/30'}">
                    {row.debit_sar > 0 ? formatCurrency(row.debit_sar, "").replace(/YER|USD|SAR|AED/g, '') : "0"}
                  </TableCell>
                  <TableCell class="border-r border-border/60 text-center font-mono {row.credit_sar > 0 ? 'text-foreground font-semibold' : 'text-muted-foreground/30'}">
                    {row.credit_sar > 0 ? formatCurrency(row.credit_sar, "").replace(/YER|USD|SAR|AED/g, '') : "0"}
                  </TableCell>

                  <!-- USD -->
                  <TableCell class="border-r border-border/40 text-center font-mono {row.debit_usd > 0 ? 'text-foreground font-semibold' : 'text-muted-foreground/30'}">
                    {row.debit_usd > 0 ? formatCurrency(row.debit_usd, "").replace(/YER|USD|SAR|AED/g, '') : "0"}
                  </TableCell>
                  <TableCell class="border-r border-border/60 text-center font-mono {row.credit_usd > 0 ? 'text-foreground font-semibold' : 'text-muted-foreground/30'}">
                    {row.credit_usd > 0 ? formatCurrency(row.credit_usd, "").replace(/YER|USD|SAR|AED/g, '') : "0"}
                  </TableCell>

                  <!-- AED -->
                  <TableCell class="border-r border-border/40 text-center font-mono {row.debit_aed > 0 ? 'text-foreground font-semibold' : 'text-muted-foreground/30'}">
                    {row.debit_aed > 0 ? formatCurrency(row.debit_aed, "").replace(/YER|USD|SAR|AED/g, '') : "0"}
                  </TableCell>
                  <TableCell class="border-r border-border/60 text-center font-mono {row.credit_aed > 0 ? 'text-foreground font-semibold' : 'text-muted-foreground/30'}">
                    {row.credit_aed > 0 ? formatCurrency(row.credit_aed, "").replace(/YER|USD|SAR|AED/g, '') : "0"}
                  </TableCell>

                  <!-- Name and Code -->
                  <TableCell class="border-r border-border/40 text-start font-semibold text-foreground">{row.name_ar}</TableCell>
                  <TableCell class="border-r border-border/40 text-center font-mono text-foreground font-bold">{row.code}</TableCell>
                  
                  <!-- Notes -->
                  <TableCell class="text-start text-muted-foreground select-text text-[9px] line-clamp-1 py-1 truncate">
                    {showAllCurrenciesInNotes 
                      ? `${row.notes || ''} (YER: ${row.debit_yer - row.credit_yer} | USD: ${row.debit_usd - row.credit_usd})` 
                      : (row.notes || "-")}
                  </TableCell>
                </TableRow>
              {/each}
            </TableBody>

            <!-- Table Footer Totals -->
            <tfoot>
              <TableRow class="bg-card hover:bg-transparent font-bold font-mono text-[10px] border-t-2 border-border/90">
                <TableCell class="border-r border-border/40 text-center text-foreground">Σ</TableCell>
                
                <!-- YER Total -->
                <TableCell class="border-r border-border/40 text-center text-red-500 font-bold">
                  {totals.debY > 0 ? formatCurrency(totals.debY, "").replace(/YER|USD|SAR|AED/g, '') : "0"}
                </TableCell>
                <TableCell class="border-r border-border/60 text-center text-emerald-500 font-bold">
                  {totals.credY > 0 ? formatCurrency(totals.credY, "").replace(/YER|USD|SAR|AED/g, '') : "0"}
                </TableCell>

                <!-- SAR Total -->
                <TableCell class="border-r border-border/40 text-center text-red-500 font-bold">
                  {totals.debS > 0 ? formatCurrency(totals.debS, "").replace(/YER|USD|SAR|AED/g, '') : "0"}
                </TableCell>
                <TableCell class="border-r border-border/60 text-center text-emerald-500 font-bold">
                  {totals.credS > 0 ? formatCurrency(totals.credS, "").replace(/YER|USD|SAR|AED/g, '') : "0"}
                </TableCell>

                <!-- USD Total -->
                <TableCell class="border-r border-border/40 text-center text-red-500 font-bold">
                  {totals.debU > 0 ? formatCurrency(totals.debU, "").replace(/YER|USD|SAR|AED/g, '') : "0"}
                </TableCell>
                <TableCell class="border-r border-border/60 text-center text-emerald-500 font-bold">
                  {totals.credU > 0 ? formatCurrency(totals.credU, "").replace(/YER|USD|SAR|AED/g, '') : "0"}
                </TableCell>

                <!-- AED Total -->
                <TableCell class="border-r border-border/40 text-center text-red-500 font-bold">
                  {totals.debA > 0 ? formatCurrency(totals.debA, "").replace(/YER|USD|SAR|AED/g, '') : "0"}
                </TableCell>
                <TableCell class="border-r border-border/60 text-center text-emerald-500 font-bold">
                  {totals.credA > 0 ? formatCurrency(totals.credA, "").replace(/YER|USD|SAR|AED/g, '') : "0"}
                </TableCell>

                <TableCell class="border-r border-border/40 text-start text-foreground uppercase tracking-wider text-[9px]">{getLocale() === "ar" ? "الإجـمــالي" : "Grand Total"}</TableCell>
                <TableCell class="border-r border-border/40 text-center text-foreground font-bold">-</TableCell>
                <TableCell class="text-start text-muted-foreground">-</TableCell>
              </TableRow>
            </tfoot>
          </Table>
        </div>
      {/if}
    </div>


  </div>

  <!-- Bottom Action Panel Button Row -->
  <div class="p-4 border-t border-border bg-card/25 flex justify-between items-center shrink-0 no-print select-none">
    
    <!-- Left actions (Exit) -->
    <Button
      variant="ghost"
      class="text-xs h-9 px-4 border border-border flex items-center gap-2 bg-gradient-to-b hover:from-muted/20 hover:to-muted/30 font-semibold"
      onclick={exitPage}
    >
      <LogOut size={13} class={getLocale() === "ar" ? 'rotate-180' : ''} />
      <span>{m.ledger_action_exit()}</span>
    </Button>

    <!-- Right actions (View / Prints) -->
    <div class="flex gap-3">
      <!-- 1. Print Selected -->
      <Button
        variant="ghost"
        class="text-xs h-9 px-4 border border-border flex items-center gap-2 bg-gradient-to-b hover:from-muted/20 hover:to-muted/30 font-semibold"
        onclick={handlePrint}
      >
        <Printer size={13} />
        <span>{m.ledger_action_print_selected()}</span>
      </Button>

      <!-- 2. Print -->
      <Button
        variant="ghost"
        class="text-xs h-9 px-4 border border-border flex items-center gap-2 bg-gradient-to-b hover:from-muted/20 hover:to-muted/30 font-semibold"
        onclick={handlePrint}
      >
        <Printer size={13} />
        <span>{m.ledger_action_print()}</span>
      </Button>

      <!-- 3. Show / Refresh (Main Action) -->
      <Button
        variant="lime"
        class="text-xs h-9 px-5 flex items-center gap-2 font-bold bg-gradient-to-b from-brand-lime to-brand-lime/80 shadow-sm"
        onclick={loadData}
        disabled={isLoading}
      >
        <RotateCw size={13} class={isLoading ? 'animate-spin' : ''} />
        <span>{m.ledger_action_view()}</span>
      </Button>
    </div>
  </div>
</div>

<style>
/* Print Friendly Layout Override */
@media print {
  :global(.no-print) {
    display: none !important;
  }
  :global(.print-only) {
    display: block !important;
  }
  :global(body) {
    background: white !important;
    color: black !important;
  }
  .print-full-width {
    width: 100% !important;
    max-width: 100% !important;
    padding: 0 !important;
    margin: 0 !important;
    overflow: visible !important;
  }
  .print-table {
    border: 1px solid #000 !important;
    border-radius: 0 !important;
    width: 100% !important;
    box-shadow: none !important;
    background: white !important;
  }
  :global(.print-table table) {
    border-collapse: collapse !important;
    width: 100% !important;
  }
  :global(.print-table th), :global(.print-table td) {
    border: 1px solid #000 !important;
    padding: 4px 6px !important;
    color: #000 !important;
    background: transparent !important;
  }
  :global(.print-table tfoot td) {
    border-top: 2px solid #000 !important;
    font-weight: bold !important;
  }
}
</style>
