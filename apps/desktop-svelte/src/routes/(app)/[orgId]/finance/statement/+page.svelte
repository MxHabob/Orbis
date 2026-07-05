<!--
@component UnifiedFinanceStatementPage
Consolidated Statement of Account (كشف الحساب الموحد)
Offers "All Accounts" (Trial Balance / Ledger List) and "Single Account" (Detailed Transaction Statement).
Uses hierarchical COA Tree sidebar for leaf account filtering and recursive aggregate calculation.
-->
<script lang="ts">
import { onMount } from "svelte";
import { page } from "$app/state";
import { goto } from "$app/navigation";
import { fade, slide } from "svelte/transition";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@orbis/ui-svelte/table";
import { Button } from "@orbis/ui-svelte/button";
import { Input } from "@orbis/ui-svelte/input";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import { Card } from "@orbis/ui-svelte/card";
import { Badge } from "@orbis/ui-svelte/badge";
import { Avatar, AvatarFallback } from "@orbis/ui-svelte/avatar";
import {
  Printer,
  ChevronRight,
  ChevronDown,
  RotateCw,
  LogOut,
  Folder,
  FolderOpen,
  CheckSquare,
  Square,
  FileText,
  ClipboardList,
  Search,
  ArrowUpRight,
  ArrowDownLeft,
  TrendingUp,
  FileSpreadsheet
} from "lucide-svelte";
import { getLocale } from "$lib/paraglide/runtime";
import { formatCurrency, formatDate } from "$lib/core/i18n";
import * as m from "$lib/paraglide/messages";
import {
  createLedgerAccountsQuery,
  createAccountStatementQuery
} from "$lib/features/finance";

let { data }: { data: { orgId: string } } = $props();
const orgId = $derived(data.orgId);

// Core page state
let activeTab = $state<"all" | "single">("all");
let selectedAccountId = $state<string>("");
let selectedCurrency = $state<string>("all");
let movementFilter = $state<string>("all"); // 'all' or 'active' (with movements)
let searchQuery = $state<string>("");

// Date Range filters
let startDate = $state<string>(
  new Date(Date.now() - 30 * 24 * 60 * 60 * 1000).toISOString().split("T")[0]
);
let endDate = $state<string>(
  new Date().toISOString().split("T")[0]
);

// Sidebar tree expanded states
let expandedCategories = $state<Record<string, boolean>>({
  asset: true,
  liability: false,
  expense: false,
  revenue: false,
  equity: false,
});
let expandedAccounts = $state<Record<string, boolean>>({});
let isCoadTreeExpanded = $state(true);

// Synced inputs for comboboxes in Single Account tab
let accountCodeQuery = $state("");
let accountNameQuery = $state("");
let isCodeDropdownOpen = $state(false);
let isNameDropdownOpen = $state(false);

interface AccountStatement {
  account_id: string;
  account_name: string;
  account_code: string;
  currency: string;
  start_date: string;
  end_date: string;
  opening_balance: number;
  closing_balance: number;
  items: Array<{
    id: string;
    date: string;
    reference: string;
    description: string;
    description_ar: string | null;
    debit?: number;
    credit?: number;
    running_balance: number;
  }>;
}

interface LedgerAccountResponse {
  id: string;
  org_id: string;
  code: string;
  name: string;
  name_ar?: string | null;
  item_type: string;
  sub_type?: string | null;
  parent_id?: string | null;
  path: string;
  depth: number;
  currency: string;
  opening_balance?: number | null;
  current_balance?: number | null;
  debit_yer: number;
  credit_yer: number;
  debit_sar: number;
  credit_sar: number;
  debit_usd: number;
  credit_usd: number;
  debit_aed: number;
  credit_aed: number;
}

// Queries
const ledgersQuery = createLedgerAccountsQuery(orgId);
const ledgers = $derived(($ledgersQuery.data || []) as LedgerAccountResponse[]);
const isLoadingLedgers = $derived($ledgersQuery.isLoading);

// Query for detailed statement
const statementQuery = createAccountStatementQuery(
  orgId,
  selectedAccountId,
  startDate,
  endDate
);
const statement = $derived($statementQuery.data as AccountStatement | undefined);
const isLoadingStatement = $derived($statementQuery.isLoading);
const statementError = $derived(
  $statementQuery.error ? ($statementQuery.error as any).message || String($statementQuery.error) : null
);

// Setup onMount
onMount(() => {
  // Query param auto-selection
  const qAccountId = page.url.searchParams.get("accountId");
  if (qAccountId) {
    selectedAccountId = qAccountId;
    activeTab = "single";
  }
});

// Reactively sync combobox inputs when selectedAccountId changes
$effect(() => {
  if (selectedAccountId && ledgers.length > 0) {
    const acc = ledgers.find((l) => l.id === selectedAccountId);
    if (acc) {
      accountCodeQuery = String(acc.code);
      accountNameQuery = acc.name_ar || acc.name;
    }
  } else {
    accountCodeQuery = "";
    accountNameQuery = "";
  }
});

// Combobox search lists
const searchedAccountsByCode = $derived.by(() => {
  const q = accountCodeQuery.toLowerCase().trim();
  if (!q) return ledgers;
  return ledgers.filter((l) => String(l.code).includes(q));
});

const searchedAccountsByName = $derived.by(() => {
  const q = accountNameQuery.toLowerCase().trim();
  if (!q) return ledgers;
  return ledgers.filter(
    (l) =>
      String(l.name_ar || "").toLowerCase().includes(q) ||
      String(l.name || "").toLowerCase().includes(q)
  );
});

// Select account handler
function selectAccount(acc: any) {
  selectedAccountId = acc.id;
  accountCodeQuery = String(acc.code);
  accountNameQuery = acc.name_ar || acc.name;
  isCodeDropdownOpen = false;
  isNameDropdownOpen = false;
}

// Tree and leaf calculation logic
const parentIds = $derived(
  new Set(ledgers.map((l) => l.parent_id).filter(Boolean) as string[])
);

const leafAccounts = $derived(
  ledgers.filter((l) => !parentIds.has(l.id))
);

// Get leaf accounts recursively under any selected node
const selectedLeafAccounts = $derived.by(() => {
  if (!selectedAccountId) return leafAccounts;
  const selectedAcc = ledgers.find((l) => l.id === selectedAccountId);
  if (!selectedAcc) return leafAccounts;

  // If selected account itself is a leaf account
  if (!parentIds.has(selectedAcc.id)) {
    return [selectedAcc];
  }

  // Find all descendant IDs recursively
  function getDescendantIds(parentId: string): string[] {
    const descendants: string[] = [];
    for (const a of ledgers) {
      if (a.parent_id === parentId) {
        descendants.push(a.id);
        descendants.push(...getDescendantIds(a.id));
      }
    }
    return descendants;
  }

  const childIds = getDescendantIds(selectedAcc.id);
  return leafAccounts.filter(
    (l) => childIds.includes(l.id) || l.id === selectedAcc.id
  );
});

// Filter rows for "All Accounts" list
const filteredLedgerRows = $derived.by(() => {
  let rows = selectedLeafAccounts;

  // Apply search query
  const q = searchQuery.toLowerCase().trim();
  if (q) {
    rows = rows.filter(
      (r) =>
        String(r.code).includes(q) ||
        String(r.name || "").toLowerCase().includes(q) ||
        String(r.name_ar || "").toLowerCase().includes(q)
    );
  }

  // Apply movement filter
  if (movementFilter === "active") {
    rows = rows.filter(
      (r) =>
        (r.debit_yer || 0) > 0 || (r.credit_yer || 0) > 0 ||
        (r.debit_sar || 0) > 0 || (r.credit_sar || 0) > 0 ||
        (r.debit_usd || 0) > 0 || (r.credit_usd || 0) > 0 ||
        (r.debit_aed || 0) > 0 || (r.credit_aed || 0) > 0
    );
  }

  return rows;
});

// Calculate totals for "All Accounts"
const allAccountsTotals = $derived.by(() => {
  let debY = 0, credY = 0;
  let debS = 0, credS = 0;
  let debU = 0, credU = 0;
  let debA = 0, credA = 0;

  for (const r of filteredLedgerRows) {
    debY += r.debit_yer || 0;
    credY += r.credit_yer || 0;
    debS += r.debit_sar || 0;
    credS += r.credit_sar || 0;
    debU += r.debit_usd || 0;
    credU += r.credit_usd || 0;
    debA += r.debit_aed || 0;
    credA += r.credit_aed || 0;
  }

  return { debY, credY, debS, credS, debU, credU, debA, credA };
});

// Detailed transactions statement derived state
const statementTotals = $derived.by(() => {
  if (!statement || !statement.items) return { debit: 0, credit: 0 };
  let debit = 0;
  let credit = 0;
  for (const item of statement.items) {
    if (item.debit) debit += item.debit;
    if (item.credit) credit += item.credit;
  }
  return { debit, credit };
});

// Filter transactions in single account
const filteredItems = $derived.by(() => {
  if (!statement || !statement.items) return [];
  const q = searchQuery.toLowerCase().trim();
  if (!q) return statement.items;
  return statement.items.filter(
    (item: any) =>
      String(item.reference || "").toLowerCase().includes(q) ||
      String(item.description || "").toLowerCase().includes(q) ||
      String(item.description_ar || "").toLowerCase().includes(q)
  );
});

// COA Tree structure builder
const treeAccounts = $derived.by(() => {
  const map: Record<string, any> = {};
  for (const acc of ledgers) {
    map[acc.id] = { ...acc, children: [] };
  }

  const roots: any[] = [];
  for (const acc of ledgers) {
    const node = map[acc.id];
    if (acc.parent_id && map[acc.parent_id]) {
      map[acc.parent_id].children.push(node);
    } else {
      roots.push(node);
    }
  }

  // Sort roots by code (e.g. 1, 2, 3, 4, 5)
  roots.sort((a, b) => String(a.code).localeCompare(String(b.code)));
  return roots;
});

// Tree selection actions
function selectTreeAccount(accId: string) {
  selectedAccountId = accId;
}

function toggleCategoryExpand(catId: string) {
  expandedCategories[catId] = !expandedCategories[catId];
}

function toggleAccountExpand(accId: string) {
  expandedAccounts[accId] = !expandedAccounts[accId];
}

// Arabic Number-to-Words spelling helper
function arabicNumberToWords(num: number): string {
  if (num === 0) return "صفر";

  const integerPart = Math.floor(num);
  const decimalPart = Math.round((num - integerPart) * 100);

  const ones = [
    "", "واحد", "اثنان", "ثلاثة", "أربعة", "خمسة", "ستة", "سبعة", "ثمانية", "تسعة", "عشرة",
    "أحد عشر", "اثنا عشر", "ثلاثة عشر", "أربعة عشر", "خمسة عشر", "ستة عشر", "سبعة عشر", "ثمانية عشر", "تسعة عشر"
  ];
  const tens = [
    "", "", "عشرون", "ثلاثون", "أربعون", "خمسون", "ستون", "سبعون", "ثمانون", "تسعون"
  ];
  const hundreds = [
    "", "مائة", "مائتان", "ثلاثمائة", "أربعمائة", "خمسمائة", "ستمائة", "سبعمائة", "ثمانمائة", "تسعمائة"
  ];

  function convertGroup(n: number): string {
    let parts = [];
    const h = Math.floor(n / 100);
    const remainder = n % 100;

    if (h > 0) parts.push(hundreds[h]);

    if (remainder > 0) {
      if (remainder < 20) {
        parts.push(ones[remainder]);
      } else {
        const o = remainder % 10;
        const t = Math.floor(remainder / 10);
        if (o > 0) {
          parts.push(`${ones[o]} و ${tens[t]}`);
        } else {
          parts.push(tens[t]);
        }
      }
    }
    return parts.join(" و ");
  }

  let parts = [];
  let temp = integerPart;

  const billions = Math.floor(temp / 1000000000);
  temp %= 1000000000;
  if (billions > 0) {
    parts.push(
      billions === 1 ? "مليار" : billions === 2 ? "ملياران" : `${convertGroup(billions)} مليارات`
    );
  }

  const millions = Math.floor(temp / 1000000);
  temp %= 1000000;
  if (millions > 0) {
    parts.push(
      millions === 1 ? "مليون" : millions === 2 ? "مليونان" : `${convertGroup(millions)} ملايين`
    );
  }

  const thousands = Math.floor(temp / 1000);
  temp %= 1000;
  if (thousands > 0) {
    parts.push(
      thousands === 1 ? "ألف" : thousands === 2 ? "ألفان" : `${convertGroup(thousands)} آلاف`
    );
  }

  if (temp > 0) {
    parts.push(convertGroup(temp));
  }

  let arabicString = parts.join(" و ");

  if (decimalPart > 0) {
    arabicString += ` و ${convertGroup(decimalPart)} جزءاً من المائة`;
  }

  return arabicString;
}

// Derived text for running balance spelling
const runningBalanceText = $derived.by(() => {
  if (!statement) return "";
  const net = statement.closing_balance;
  const word = arabicNumberToWords(Math.abs(net));
  const prefix = net >= 0 ? "الرصيد لكم: " : "الرصيد عليكم: ";
  return prefix + word;
});

// CSV Export
function exportToCsv() {
  let csvContent = "data:text/csv;charset=utf-8,\uFEFF"; // UTF-8 BOM for Excel Arabic support

  if (activeTab === "all") {
    csvContent += "كود الحساب,اسم الحساب,مدين يمني,دائن يمني,مدين سعودي,دائن سعودي,مدين دولار,دائن دولار,مدين درهم,دائن درهم\n";
    for (const r of filteredLedgerRows) {
      csvContent += `="${r.code}","${r.name_ar || r.name}",${r.debit_yer},${r.credit_yer},${r.debit_sar},${r.credit_sar},${r.debit_usd},${r.credit_usd},${r.debit_aed},${r.credit_aed}\n`;
    }
  } else {
    if (!statement) return;
    csvContent += `كشف حساب تفصيلي للحساب: ${statement.account_code} - ${statement.account_name}\n`;
    csvContent += `الفترة من: ${startDate} إلى: ${endDate}\n\n`;
    csvContent += "التاريخ,المرجع,البيان,مدين,دائن,الرصيد التراكمي\n";
    csvContent += `${statement.start_date},OPENING,رصيد أول المدة الافتتاحي,-,-,${statement.opening_balance}\n`;
    for (const item of filteredItems) {
      csvContent += `"${item.date}","${item.reference}","${item.description_ar || item.description}",${item.debit || 0},${item.credit || 0},${item.running_balance}\n`;
    }
  }

  const encodedUri = encodeURI(csvContent);
  const link = document.createElement("a");
  link.setAttribute("href", encodedUri);
  link.setAttribute(
    "download",
    activeTab === "all" ? "ledger_all_accounts.csv" : `statement_${statement?.account_code || "account"}.csv`
  );
  document.body.appendChild(link);
  link.click();
  document.body.removeChild(link);
}

function handlePrint() {
  window.print();
}

function exitPage() {
  goto(`/${orgId}/finance`);
}

function handleWindowClick(event: MouseEvent) {
  const target = event.target as HTMLElement;
  if (!target.closest(".combobox-container")) {
    isCodeDropdownOpen = false;
    isNameDropdownOpen = false;
  }
}
</script>

<svelte:window onclick={handleWindowClick} />

<div class="h-full w-full flex overflow-hidden bg-background">
  <!-- ════ Right Sidebar: Accounts Hierarchical Tree Selector (RTL default) ════ -->
  <div class="w-80 border-r border-border bg-card/40 flex flex-col shrink-0 select-none print:hidden">
    <div class="p-4 border-b border-border flex items-center justify-between">
      <h3 class="text-xs font-bold text-foreground">دليل شجرة الحسابات</h3>
      <Button variant="ghost" size="xs" onclick={() => isCoadTreeExpanded = !isCoadTreeExpanded}>
        {isCoadTreeExpanded ? "طي الكل" : "عرض الكل"}
      </Button>
    </div>

    <div class="flex-1 overflow-y-auto p-3 text-xs">
      {#if isLoadingLedgers}
        <div class="flex flex-col gap-2.5">
          {#each Array(8) as _}
            <Skeleton class="h-6 w-full rounded" />
          {/each}
        </div>
      {:else}
        <!-- Recursive snippet for nodes -->
        {#snippet renderNodes(nodes: any[], depth: number)}
          {#each nodes as node (node.id)}
            {@const isParent = parentIds.has(node.id)}
            {@const isExpanded = expandedAccounts[node.id]}
            {@const isSelected = selectedAccountId === node.id}
            
            <div class="flex flex-col">
              <div
                class="flex items-center justify-between hover:bg-muted/40 px-2 py-1.5 rounded-lg gap-2 cursor-pointer transition-colors {isSelected ? 'bg-primary/10 text-primary font-bold border border-primary/20' : 'text-foreground'}"
                style="margin-inline-start: {depth * 10}px"
                onclick={() => selectTreeAccount(node.id)}
                role="button"
                tabindex="0"
                onkeydown={(e) => e.key === 'Enter' && selectTreeAccount(node.id)}
              >
                <div class="flex items-center gap-1.5 min-w-0">
                  {#if isParent}
                    <button
                      type="button"
                      onclick={(e) => {
                        e.stopPropagation();
                        toggleAccountExpand(node.id);
                      }}
                      class="p-0.5 text-muted-foreground hover:text-foreground"
                    >
                      {#if isExpanded}
                        <ChevronDown size={11} />
                      {:else}
                        <ChevronRight size={11} class={getLocale() === "ar" ? "rotate-180" : ""} />
                      {/if}
                    </button>
                    <FolderOpen size={11} class="text-primary/75 shrink-0" />
                  {:else}
                    <span class="w-3.5 shrink-0"></span>
                    <FileText size={11} class="text-muted-foreground shrink-0" />
                  {/if}
                  <span class="truncate font-semibold text-[11px]">{node.name_ar || node.name}</span>
                </div>
                <span class="font-mono text-[9px] text-muted-foreground bg-muted/60 px-1 py-0.5 rounded border border-border/40 shrink-0 select-text">{node.code}</span>
              </div>

              {#if isParent && isExpanded}
                <div class="flex flex-col">
                  {@render renderNodes(node.children, depth + 1)}
                </div>
              {/if}
            </div>
          {/each}
        {/snippet}

        <!-- Tree root -->
        <div class="flex flex-col gap-1.5">
          <div class="flex items-center justify-between hover:bg-muted/40 px-2 py-1.5 rounded-lg font-bold cursor-pointer">
            <button
              type="button"
              class="flex items-center gap-1.5 text-foreground hover:text-primary flex-1 text-start"
              onclick={() => isCoadTreeExpanded = !isCoadTreeExpanded}
            >
              {#if isCoadTreeExpanded}
                <ChevronDown size={12} class="text-muted-foreground" />
                <FolderOpen size={12} class="text-primary" />
              {:else}
                <ChevronRight size={12} class="text-muted-foreground {getLocale() === 'ar' ? 'rotate-180' : ''}" />
                <Folder size={12} class="text-primary" />
              {/if}
              <span>{getLocale() === 'ar' ? 'الدليل المحاسبي العام' : 'Chart of Accounts'}</span>
            </button>
            <button
              type="button"
              onclick={() => { selectedAccountId = ""; }}
              class="text-[9px] font-bold text-muted-foreground hover:text-primary px-1.5 py-0.5 bg-muted/60 rounded border border-border/40 shrink-0"
            >
              إلغاء التحديد
            </button>
          </div>

          {#if isCoadTreeExpanded}
            <div class="flex flex-col gap-1 mt-1 ps-2 border-r border-border/40 mr-1">
              {@render renderNodes(treeAccounts, 0)}
            </div>
          {/if}
        </div>
      {/if}
    </div>
  </div>

  <!-- ════ Main Section: Master View / Details View ════ -->
  <div class="flex-1 flex flex-col overflow-hidden bg-background p-6">
    <!-- Header Title -->
    <div class="flex items-center justify-between pb-4 select-none border-b border-border print:hidden">
      <div class="flex flex-col gap-1 text-start">
        <h1 class="text-lg font-bold tracking-tight text-foreground flex items-center gap-2">
          <ClipboardList class="text-primary animate-pulse" size={18} />
          <span>كشف الحساب الموحد (Statement of Account)</span>
        </h1>
        <p class="text-[11px] text-muted-foreground">
          استعراض ومراجعة كافة الأرصدة والعمليات المالية للحسابات العامة والفرعية والطرفية بدقة عالية
        </p>
      </div>

      <div class="flex items-center gap-2">
        <Button variant="outline" size="sm" onclick={exportToCsv} class="h-8 text-[11px] font-bold flex items-center gap-1.5 cursor-pointer">
          <FileSpreadsheet size={13} />
          <span>تصدير CSV</span>
        </Button>
        <Button variant="outline" size="sm" onclick={handlePrint} class="h-8 text-[11px] font-bold flex items-center gap-1.5 cursor-pointer">
          <Printer size={13} />
          <span>طباعة كشف الحساب</span>
        </Button>
      </div>
    </div>

    <!-- Filter Control Box -->
    <div class="bg-muted/20 border border-border/50 rounded-xl p-4 mt-4 mb-4 flex flex-col gap-4 select-none print:hidden">
      <div class="flex items-center justify-between border-b border-border/40 pb-3">
        <!-- Tabs selector -->
        <div class="flex border border-border bg-muted/40 rounded-lg p-0.5 text-[11px] font-bold">
          <button
            onclick={() => activeTab = "all"}
            class="px-5 py-1.5 rounded-md transition-all duration-150 {activeTab === 'all' ? 'bg-primary text-primary-foreground shadow font-bold' : 'text-muted-foreground hover:text-foreground font-semibold'}"
          >
            كافة الحسابات (All Leaf Accounts)
          </button>
          <button
            onclick={() => activeTab = "single"}
            class="px-5 py-1.5 rounded-md transition-all duration-150 {activeTab === 'single' ? 'bg-primary text-primary-foreground shadow font-bold' : 'text-muted-foreground hover:text-foreground font-semibold'}"
          >
            حساب واحد (Single Account Statement)
          </button>
        </div>

        <span class="text-[10px] text-muted-foreground/80 font-bold bg-muted/50 border border-border/40 px-3 py-1 rounded-md">
          {activeTab === 'all' ? 'عرض كشف الميزان التفصيلي' : 'كشف حساب تفصيلي جاري'}
        </span>
      </div>

      <!-- Filters depending on Tab selection -->
      <div class="flex flex-col gap-3">
        {#if activeTab === "single"}
          <!-- Combobox row for Single account select -->
          <div class="flex flex-wrap gap-4 items-center">
            <!-- Account name search combobox -->
            <div class="flex-1 min-w-[200px] flex flex-col gap-1 text-[10px] font-bold text-muted-foreground combobox-container relative">
              <span>ابحث باسم الحساب الدفتري</span>
              <div class="relative">
                <input
                  type="text"
                  placeholder="اكتب الاسم للبحث وتحديد الحساب..."
                  bind:value={accountNameQuery}
                  onfocus={() => isNameDropdownOpen = true}
                  disabled={isLoadingLedgers}
                  class="w-full bg-background text-foreground border border-border rounded-md px-3 py-1.5 text-xs outline-none focus:ring-1 focus:ring-ring disabled:opacity-50 text-start cursor-pointer"
                />
                {#if isNameDropdownOpen && !isLoadingLedgers}
                  <div class="absolute z-50 left-0 right-0 mt-1 max-h-48 overflow-y-auto bg-popover border border-border rounded-md shadow-lg flex flex-col p-1 text-[10px]" transition:fade={{ duration: 80 }}>
                    {#if searchedAccountsByName.length === 0}
                      <span class="text-[10px] text-muted-foreground p-2 text-center">لا توجد حسابات مطابقة</span>
                    {:else}
                      {#each searchedAccountsByName as acc (acc.id)}
                        <button
                          type="button"
                          onclick={() => selectAccount(acc)}
                          class="w-full text-start px-2 py-1.5 rounded hover:bg-accent hover:text-accent-foreground {selectedAccountId === acc.id ? 'bg-primary/10 text-primary font-bold' : 'text-foreground'}"
                        >
                          {acc.code} - {acc.name_ar || acc.name}
                        </button>
                      {/each}
                    {/if}
                  </div>
                {/if}
              </div>
            </div>

            <!-- Account code search combobox -->
            <div class="w-32 flex flex-col gap-1 text-[10px] font-bold text-muted-foreground combobox-container relative">
              <span>كود الحساب</span>
              <div class="relative">
                <input
                  type="text"
                  placeholder="كود..."
                  bind:value={accountCodeQuery}
                  onfocus={() => isCodeDropdownOpen = true}
                  disabled={isLoadingLedgers}
                  class="w-full bg-background text-foreground border border-border rounded-md px-3 py-1.5 text-xs font-mono outline-none focus:ring-1 focus:ring-ring disabled:opacity-50 text-start cursor-pointer"
                />
                {#if isCodeDropdownOpen && !isLoadingLedgers}
                  <div class="absolute z-50 left-0 right-0 mt-1 max-h-48 overflow-y-auto bg-popover border border-border rounded-md shadow-lg flex flex-col p-1 text-[10px]" transition:fade={{ duration: 80 }}>
                    {#if searchedAccountsByCode.length === 0}
                      <span class="text-[10px] text-muted-foreground p-2 text-center">لا يوجد</span>
                    {:else}
                      {#each searchedAccountsByCode as acc (acc.id)}
                        <button
                          type="button"
                          onclick={() => selectAccount(acc)}
                          class="w-full text-start px-2 py-1.5 rounded hover:bg-accent hover:text-accent-foreground font-mono {selectedAccountId === acc.id ? 'bg-primary/10 text-primary font-bold' : 'text-foreground'}"
                        >
                          {acc.code}
                        </button>
                      {/each}
                    {/if}
                  </div>
                {/if}
              </div>
            </div>

            <!-- Date Selectors -->
            <div class="flex items-center gap-2">
              <div class="flex flex-col gap-1 text-[10px] font-bold text-muted-foreground">
                <span>تاريخ البداية</span>
                <Input type="date" bind:value={startDate} class="h-8 text-xs w-[130px]" />
              </div>
              <div class="flex flex-col gap-1 text-[10px] font-bold text-muted-foreground">
                <span>تاريخ النهاية</span>
                <Input type="date" bind:value={endDate} class="h-8 text-xs w-[130px]" />
              </div>
            </div>
          </div>
        {:else}
          <!-- All accounts filter row -->
          <div class="flex flex-wrap gap-4 items-center">
            <!-- Currency selector filter -->
            <div class="w-48 flex flex-col gap-1 text-[10px] font-bold text-muted-foreground">
              <span>العملة المفعلة</span>
              <select
                bind:value={selectedCurrency}
                class="bg-background text-foreground border border-border rounded-md px-3 h-8.5 text-xs outline-none focus:ring-1 focus:ring-ring cursor-pointer"
              >
                <option value="all">كافة العملات (جدول كامل)</option>
                <option value="YER">YER (ريال يمني)</option>
                <option value="SAR">SAR (سعودي)</option>
                <option value="USD">USD (دولار)</option>
                <option value="AED">AED (درهم)</option>
              </select>
            </div>

            <!-- Movement status filter -->
            <div class="w-48 flex flex-col gap-1 text-[10px] font-bold text-muted-foreground">
              <span>عرض الحسابات</span>
              <select
                bind:value={movementFilter}
                class="bg-background text-foreground border border-border rounded-md px-3 h-8.5 text-xs outline-none focus:ring-1 focus:ring-ring cursor-pointer"
              >
                <option value="all">جميع الحسابات</option>
                <option value="active">الحسابات ذات الحركات المالية فقط</option>
              </select>
            </div>

            <!-- Live instant search -->
            <div class="flex-grow flex flex-col gap-1 text-[10px] font-bold text-muted-foreground min-w-[200px]">
              <span>البحث السريع</span>
              <div class="relative">
                <Search class="absolute left-2.5 top-2.5 h-3.5 w-3.5 text-muted-foreground" />
                <Input
                  type="text"
                  placeholder="ابحث برقم الحساب أو الاسم..."
                  bind:value={searchQuery}
                  class="pl-8.5 h-8.5 text-xs"
                />
              </div>
            </div>
          </div>
        {/if}
      </div>
    </div>

    <!-- Print-only Title Info -->
    <div class="hidden print:flex flex-col gap-1.5 text-center pb-6 border-b border-border/80 select-none">
      <h2 class="text-xl font-bold">كشف الحساب المالي الموحد</h2>
      <p class="text-xs font-semibold text-muted-foreground">
        {#if activeTab === "all"}
          كشف ميزان المراجعة والأستاذ لكافة الحسابات الطرفية
        {:else if statement}
          الحساب: {statement.account_code} - {statement.account_name} | الفترة: {formatDate(statement.start_date)} إلى {formatDate(statement.end_date)}
        {/if}
      </p>
    </div>

    <!-- ════ Main Content Area ════ -->
    <div class="flex-1 overflow-y-auto py-2">
      {#if activeTab === "all"}
        <!-- ════ TAB: ALL ACCOUNTS ════ -->
        {#if isLoadingLedgers}
          <div class="flex flex-col gap-3">
            {#each Array(6) as _}
              <Skeleton class="h-10 w-full rounded" />
            {/each}
          </div>
        {:else if filteredLedgerRows.length === 0}
          <div class="border border-dashed border-border rounded-2xl p-16 flex flex-col items-center justify-center text-center gap-2 select-none my-8">
            <ClipboardList size={30} class="text-muted-foreground/60 mb-2" />
            <h3 class="text-sm font-bold text-foreground">لا توجد حسابات مطابقة للفلاتر</h3>
            <p class="text-xs text-muted-foreground max-w-xs leading-relaxed">
              لم نجد أي حساب طرفي مسجل أسفل العقدة المحددة يطابق الكلمة المدخلة في البحث.
            </p>
          </div>
        {:else}
          <!-- Check if Selected Currency is ALL or specific currency -->
          {#if selectedCurrency === "all"}
            <!-- Multi-Currency Trial Balance Excel Grid Layout -->
            <div class="border border-border/60 rounded-xl overflow-hidden bg-card/10 shadow-sm print:border-none print:shadow-none min-w-[850px] overflow-x-auto">
              <Table class="border-collapse w-full">
                <TableHeader class="bg-muted/40 select-none print:bg-transparent">
                  <TableRow class="hover:bg-transparent border-b border-border/80 text-[10px] font-bold text-foreground">
                    <TableHead class="w-10 border-l border-border/40 text-center font-bold font-mono">#</TableHead>
                    <TableHead colspan={2} class="border-l border-border/40 text-center bg-primary/5 text-primary">ريال يمني (YER)</TableHead>
                    <TableHead colspan={2} class="border-l border-border/40 text-center bg-emerald-500/5 text-emerald-600">سعودي (SAR)</TableHead>
                    <TableHead colspan={2} class="border-l border-border/40 text-center bg-blue-500/5 text-blue-600">دولار (USD)</TableHead>
                    <TableHead colspan={2} class="border-l border-border/40 text-center bg-purple-500/5 text-purple-600">درهم (AED)</TableHead>
                    <TableHead class="text-start border-l border-border/40">اسم الحساب الطرفي</TableHead>
                    <TableHead class="text-center border-l border-border/40 w-24">الكود</TableHead>
                    <TableHead class="text-center w-16">إجراء</TableHead>
                  </TableRow>
                  <TableRow class="hover:bg-transparent border-b border-border/80 text-[9px] font-bold">
                    <TableHead class="border-l border-border/40"></TableHead>
                    <TableHead class="text-center border-l border-border/20 text-red-500 w-20">مدين</TableHead>
                    <TableHead class="text-center border-l border-border/40 text-emerald-500 w-20">دائن</TableHead>
                    <TableHead class="text-center border-l border-border/20 text-red-500 w-20">مدين</TableHead>
                    <TableHead class="text-center border-l border-border/40 text-emerald-500 w-20">دائن</TableHead>
                    <TableHead class="text-center border-l border-border/20 text-red-500 w-20">مدين</TableHead>
                    <TableHead class="text-center border-l border-border/40 text-emerald-500 w-20">دائن</TableHead>
                    <TableHead class="text-center border-l border-border/20 text-red-500 w-20">مدين</TableHead>
                    <TableHead class="text-center border-l border-border/40 text-emerald-500 w-20">دائن</TableHead>
                    <TableHead class="border-l border-border/40"></TableHead>
                    <TableHead class="border-l border-border/40"></TableHead>
                    <TableHead></TableHead>
                  </TableRow>
                </TableHeader>
                <TableBody>
                  {#each filteredLedgerRows as row, idx (row.id)}
                    <TableRow class="hover:bg-muted/20 border-b border-border/30 text-[11px] leading-normal font-semibold">
                      <TableCell class="border-l border-border/40 text-center text-muted-foreground font-mono select-none">{idx + 1}</TableCell>
                      
                      <!-- YER -->
                      <TableCell class="border-l border-border/20 text-center font-mono {row.debit_yer > 0 ? 'text-foreground font-extrabold' : 'text-muted-foreground/30'}">
                        {row.debit_yer > 0 ? formatCurrency(row.debit_yer, "").replace(/YER|SAR|USD|AED/g, "") : "0"}
                      </TableCell>
                      <TableCell class="border-l border-border/40 text-center font-mono {row.credit_yer > 0 ? 'text-foreground font-extrabold' : 'text-muted-foreground/30'}">
                        {row.credit_yer > 0 ? formatCurrency(row.credit_yer, "").replace(/YER|SAR|USD|AED/g, "") : "0"}
                      </TableCell>

                      <!-- SAR -->
                      <TableCell class="border-l border-border/20 text-center font-mono {row.debit_sar > 0 ? 'text-foreground font-extrabold' : 'text-muted-foreground/30'}">
                        {row.debit_sar > 0 ? formatCurrency(row.debit_sar, "").replace(/YER|SAR|USD|AED/g, "") : "0"}
                      </TableCell>
                      <TableCell class="border-l border-border/40 text-center font-mono {row.credit_sar > 0 ? 'text-foreground font-extrabold' : 'text-muted-foreground/30'}">
                        {row.credit_sar > 0 ? formatCurrency(row.credit_sar, "").replace(/YER|SAR|USD|AED/g, "") : "0"}
                      </TableCell>

                      <!-- USD -->
                      <TableCell class="border-l border-border/20 text-center font-mono {row.debit_usd > 0 ? 'text-foreground font-extrabold' : 'text-muted-foreground/30'}">
                        {row.debit_usd > 0 ? formatCurrency(row.debit_usd, "").replace(/YER|SAR|USD|AED/g, "") : "0"}
                      </TableCell>
                      <TableCell class="border-l border-border/40 text-center font-mono {row.credit_usd > 0 ? 'text-foreground font-extrabold' : 'text-muted-foreground/30'}">
                        {row.credit_usd > 0 ? formatCurrency(row.credit_usd, "").replace(/YER|SAR|USD|AED/g, "") : "0"}
                      </TableCell>

                      <!-- AED -->
                      <TableCell class="border-l border-border/20 text-center font-mono {row.debit_aed > 0 ? 'text-foreground font-extrabold' : 'text-muted-foreground/30'}">
                        {row.debit_aed > 0 ? formatCurrency(row.debit_aed, "").replace(/YER|SAR|USD|AED/g, "") : "0"}
                      </TableCell>
                      <TableCell class="border-l border-border/40 text-center font-mono {row.credit_aed > 0 ? 'text-foreground font-extrabold' : 'text-muted-foreground/30'}">
                        {row.credit_aed > 0 ? formatCurrency(row.credit_aed, "").replace(/YER|SAR|USD|AED/g, "") : "0"}
                      </TableCell>

                      <!-- Meta info -->
                      <TableCell class="border-l border-border/40 text-start font-bold text-foreground truncate max-w-[180px]">{row.name_ar || row.name}</TableCell>
                      <TableCell class="border-l border-border/40 text-center font-mono text-muted-foreground font-bold">{row.code}</TableCell>
                      
                      <!-- Drill down button -->
                      <TableCell class="text-center print:hidden">
                        <Button
                          variant="ghost"
                          size="xs"
                          class="h-6 w-12 text-[10px] text-primary hover:underline hover:bg-primary/5 cursor-pointer font-bold shrink-0 p-0"
                          onclick={() => {
                            selectedAccountId = row.id;
                            activeTab = "single";
                          }}
                        >
                          تفاصيل
                        </Button>
                      </TableCell>
                    </TableRow>
                  {/each}
                </TableBody>

                <!-- Grid Totals Footer -->
                <tfoot>
                  <TableRow class="bg-muted/40 hover:bg-transparent font-bold font-mono text-[11px] border-t-2 border-border/80">
                    <TableCell class="border-l border-border/40 text-center text-foreground">∑</TableCell>
                    
                    <!-- YER -->
                    <TableCell class="border-l border-border/20 text-center text-red-500 font-extrabold">
                      {allAccountsTotals.debY > 0 ? formatCurrency(allAccountsTotals.debY, "").replace(/YER|SAR|USD|AED/g, "") : "0"}
                    </TableCell>
                    <TableCell class="border-l border-border/40 text-center text-emerald-500 font-extrabold">
                      {allAccountsTotals.credY > 0 ? formatCurrency(allAccountsTotals.credY, "").replace(/YER|SAR|USD|AED/g, "") : "0"}
                    </TableCell>

                    <!-- SAR -->
                    <TableCell class="border-l border-border/20 text-center text-red-500 font-extrabold">
                      {allAccountsTotals.debS > 0 ? formatCurrency(allAccountsTotals.debS, "").replace(/YER|SAR|USD|AED/g, "") : "0"}
                    </TableCell>
                    <TableCell class="border-l border-border/40 text-center text-emerald-500 font-extrabold">
                      {allAccountsTotals.credS > 0 ? formatCurrency(allAccountsTotals.credS, "").replace(/YER|SAR|USD|AED/g, "") : "0"}
                    </TableCell>

                    <!-- USD -->
                    <TableCell class="border-l border-border/20 text-center text-red-500 font-extrabold">
                      {allAccountsTotals.debU > 0 ? formatCurrency(allAccountsTotals.debU, "").replace(/YER|SAR|USD|AED/g, "") : "0"}
                    </TableCell>
                    <TableCell class="border-l border-border/40 text-center text-emerald-500 font-extrabold">
                      {allAccountsTotals.credU > 0 ? formatCurrency(allAccountsTotals.credU, "").replace(/YER|SAR|USD|AED/g, "") : "0"}
                    </TableCell>

                    <!-- AED -->
                    <TableCell class="border-l border-border/20 text-center text-red-500 font-extrabold">
                      {allAccountsTotals.debA > 0 ? formatCurrency(allAccountsTotals.debA, "").replace(/YER|SAR|USD|AED/g, "") : "0"}
                    </TableCell>
                    <TableCell class="border-l border-border/40 text-center text-emerald-500 font-extrabold">
                      {allAccountsTotals.credA > 0 ? formatCurrency(allAccountsTotals.credA, "").replace(/YER|SAR|USD|AED/g, "") : "0"}
                    </TableCell>

                    <TableCell class="border-l border-border/40 text-start text-foreground uppercase tracking-wider text-[10px] font-bold">الإجمالي العام</TableCell>
                    <TableCell class="border-l border-border/40 text-center font-bold font-mono">-</TableCell>
                    <TableCell class="text-center print:hidden">-</TableCell>
                  </TableRow>
                </tfoot>
              </Table>
            </div>
          {:else}
            <!-- Specific Single Currency Selected: simpler layout -->
            <div class="border border-border/60 rounded-xl overflow-hidden bg-card/10 shadow-sm print:border-none print:shadow-none w-full">
              <Table class="border-collapse w-full">
                <TableHeader class="bg-muted/40 select-none print:bg-transparent">
                  <TableRow class="hover:bg-transparent border-b border-border/80 text-[10px] font-bold text-foreground">
                    <TableHead class="w-10 border-l border-border/40 text-center font-bold font-mono">#</TableHead>
                    <TableHead class="text-center border-l border-border/40 w-32">الرصيد الافتتاحي</TableHead>
                    <TableHead class="text-center border-l border-border/40 text-red-500 w-32">الحركة (مدين)</TableHead>
                    <TableHead class="text-center border-l border-border/40 text-emerald-500 w-32">الحركة (دائن)</TableHead>
                    <TableHead class="text-center border-l border-border/40 w-32">الرصيد الحالي</TableHead>
                    <TableHead class="text-start border-l border-border/40">اسم الحساب</TableHead>
                    <TableHead class="text-center border-l border-border/40 w-32">الكود</TableHead>
                    <TableHead class="text-center w-20">إجراء</TableHead>
                  </TableRow>
                </TableHeader>
                <TableBody>
                  {#each filteredLedgerRows as row, idx (row.id)}
                    {@const isDebitType = row.item_type === "asset" || row.item_type === "expense"}
                    {@const ob = row.currency === selectedCurrency ? (row.opening_balance || 0) : 0}
                    {@const deb = selectedCurrency === "YER" ? row.debit_yer : selectedCurrency === "SAR" ? row.debit_sar : selectedCurrency === "USD" ? row.debit_usd : row.debit_aed}
                    {@const cred = selectedCurrency === "YER" ? row.credit_yer : selectedCurrency === "SAR" ? row.credit_sar : selectedCurrency === "USD" ? row.credit_usd : row.credit_aed}
                    {@const bal = ob + (isDebitType ? (deb - cred) : (cred - deb))}
                    
                    <TableRow class="hover:bg-muted/20 border-b border-border/30 text-[11px] font-semibold">
                      <TableCell class="border-l border-border/40 text-center text-muted-foreground font-mono select-none">{idx + 1}</TableCell>
                      <TableCell class="border-l border-border/40 text-center font-mono">{formatCurrency(ob, selectedCurrency)}</TableCell>
                      <TableCell class="border-l border-border/40 text-center font-mono text-red-500">{deb > 0 ? formatCurrency(deb, selectedCurrency) : "0"}</TableCell>
                      <TableCell class="border-l border-border/40 text-center font-mono text-emerald-500">{cred > 0 ? formatCurrency(cred, selectedCurrency) : "0"}</TableCell>
                      <TableCell class="border-l border-border/40 text-center font-mono font-extrabold">{formatCurrency(bal, selectedCurrency)}</TableCell>
                      <TableCell class="border-l border-border/40 text-start font-bold text-foreground">{row.name_ar || row.name}</TableCell>
                      <TableCell class="border-l border-border/40 text-center font-mono text-muted-foreground font-bold">{row.code}</TableCell>
                      <TableCell class="text-center print:hidden">
                        <Button
                          variant="ghost"
                          size="xs"
                          class="h-6 w-12 text-[10px] text-primary hover:underline hover:bg-primary/5 cursor-pointer font-bold shrink-0 p-0"
                          onclick={() => {
                            selectedAccountId = row.id;
                            activeTab = "single";
                          }}
                        >
                          تفاصيل
                        </Button>
                      </TableCell>
                    </TableRow>
                  {/each}
                </TableBody>
              </Table>
            </div>
          {/if}
        {/if}
      {:else}
        <!-- ════ TAB: SINGLE ACCOUNT ════ -->
        {#if isLoadingStatement}
          <div class="grid grid-cols-1 md:grid-cols-4 gap-4 mb-6">
            {#each Array(4) as _}
              <Skeleton class="h-20 rounded-xl" />
            {/each}
          </div>
          <div class="flex flex-col gap-3">
            {#each Array(6) as _}
              <Skeleton class="h-10 w-full rounded" />
            {/each}
          </div>
        {:else if statementError}
          <div in:fade={{ duration: 150 }} class="border border-destructive/20 bg-destructive/10 text-destructive rounded-xl p-4 flex items-center gap-3 text-sm text-start print:hidden">
            <div class="font-bold">خطأ في تحميل كشف الحساب:</div>
            <div>{statementError}</div>
          </div>
        {:else if !statement}
          <div class="border border-dashed border-border/80 rounded-2xl p-16 flex flex-col items-center justify-center text-center gap-3 select-none my-8">
            <div class="w-12 h-12 rounded-full bg-primary/10 flex items-center justify-center text-primary mb-1 shrink-0">
              <ClipboardList size={22} />
            </div>
            <h3 class="text-sm font-bold text-foreground">الرجاء اختيار حساب طرفي لعرض كشف الحركة</h3>
            <p class="text-[11px] text-muted-foreground max-w-xs leading-relaxed">
              اختر أي حساب من الشجرة الدفترية الجانبية أو اكتب في قائمة البحث للوصول المباشر إلى مستندات وقيود الحساب وحركته التراكمية.
            </p>
          </div>
        {:else}
          <!-- Single Account Summary Cards -->
          <div in:slide={{ duration: 200 }} class="grid grid-cols-2 lg:grid-cols-4 gap-4 mb-6 select-none">
            <!-- Card 1: Opening Balance -->
            <div class="border border-border/60 bg-card/25 p-4 rounded-2xl text-start flex flex-col justify-between hover:border-primary/30 transition-all duration-200">
              <span class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">الرصيد الافتتاحي</span>
              <div class="flex items-baseline gap-1 mt-2">
                <span class="text-lg font-extrabold font-mono text-foreground">
                  {formatCurrency(statement.opening_balance, statement.currency)}
                </span>
              </div>
              <span class="text-[9px] text-muted-foreground mt-1">قبل تاريخ {formatDate(statement.start_date)}</span>
            </div>

            <!-- Card 2: Total Debits -->
            <div class="border border-border/60 bg-card/25 p-4 rounded-2xl text-start flex flex-col justify-between hover:border-emerald-500/30 transition-all duration-200">
              <div class="flex items-center justify-between w-full">
                <span class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">إجمالي الحركات (مدين)</span>
                <div class="w-5 h-5 rounded-full bg-emerald-500/10 flex items-center justify-center text-emerald-500 shrink-0">
                  <ArrowDownLeft size={10} />
                </div>
              </div>
              <div class="flex items-baseline gap-1 mt-2">
                <span class="text-lg font-extrabold font-mono text-emerald-500">
                  {formatCurrency(statementTotals.debit, statement.currency)}
                </span>
              </div>
              <span class="text-[9px] text-muted-foreground mt-1">مجموع المبالغ المقيدة كمدين</span>
            </div>

            <!-- Card 3: Total Credits -->
            <div class="border border-border/60 bg-card/25 p-4 rounded-2xl text-start flex flex-col justify-between hover:border-rose-500/30 transition-all duration-200">
              <div class="flex items-center justify-between w-full">
                <span class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">إجمالي الحركات (دائن)</span>
                <div class="w-5 h-5 rounded-full bg-rose-500/10 flex items-center justify-center text-rose-500 shrink-0">
                  <ArrowUpRight size={10} />
                </div>
              </div>
              <div class="flex items-baseline gap-1 mt-2">
                <span class="text-lg font-extrabold font-mono text-rose-500">
                  {formatCurrency(statementTotals.credit, statement.currency)}
                </span>
              </div>
              <span class="text-[9px] text-muted-foreground mt-1">مجموع المبالغ المقيدة كدائن</span>
            </div>

            <!-- Card 4: Closing Balance -->
            <div class="border-2 border-primary bg-primary/5 p-4 rounded-2xl text-start flex flex-col justify-between shadow-sm relative overflow-hidden">
              <div class="flex items-center justify-between w-full">
                <span class="text-[10px] font-bold text-primary uppercase tracking-wider">الرصيد الختامي</span>
                <TrendingUp size={12} class="text-primary shrink-0" />
              </div>
              <div class="flex items-baseline gap-1 mt-2">
                <span class="text-xl font-extrabold font-mono text-primary">
                  {formatCurrency(statement.closing_balance, statement.currency)}
                </span>
              </div>
              <span class="text-[9px] text-primary/80 mt-1">صافي الرصيد الجاري الفعلي</span>
            </div>
          </div>

          <!-- Spelling helper bar for accounting audit -->
          <div class="mb-4 bg-muted/30 border border-border/40 p-2.5 rounded-lg text-xs font-bold text-foreground text-start select-text flex items-center gap-2">
            <span class="text-primary">ملاحظة الرصيد الفصيح:</span>
            <span>{runningBalanceText}</span>
          </div>

          <!-- Statement Transactions Table -->
          {#if filteredItems.length === 0}
            <div class="border border-dashed border-border/80 rounded-2xl p-12 flex flex-col items-center justify-center text-center gap-2 select-none">
              <h4 class="text-xs font-bold text-muted-foreground">لا توجد قيود مسجلة للحساب</h4>
              <p class="text-[10px] text-muted-foreground/80">
                لم تسجل أي حركة ترحيل على هذا الحساب خلال الفترة المحددة.
              </p>
            </div>
          {:else}
            <div in:fade={{ duration: 150 }} class="border border-border/60 rounded-xl overflow-hidden bg-card/10 shadow-sm print:border-none print:shadow-none w-full">
              <Table class="border-collapse w-full">
                <TableHeader class="bg-muted/40 select-none print:bg-transparent">
                  <TableRow class="hover:bg-transparent border-b border-border/80 text-[11px] font-bold text-foreground">
                    <TableHead class="text-start">التاريخ</TableHead>
                    <TableHead class="text-start">المرجع</TableHead>
                    <TableHead class="text-start">البيان / الحركة التفصيلية</TableHead>
                    <TableHead class="text-end text-emerald-500 w-28">مدين (+)</TableHead>
                    <TableHead class="text-end text-rose-500 w-28">دائن (-)</TableHead>
                    <TableHead class="text-end w-32">الرصيد الجاري</TableHead>
                  </TableRow>
                </TableHeader>
                <TableBody>
                  <!-- Opening balance row inside statement table -->
                  <TableRow class="bg-muted/10 font-bold hover:bg-muted/10 transition-colors text-[11px]">
                    <TableCell class="text-start text-muted-foreground font-mono">{formatDate(statement.start_date)}</TableCell>
                    <TableCell class="text-start text-muted-foreground font-mono">OPENING</TableCell>
                    <TableCell class="text-start text-muted-foreground font-bold">رصيد أول المدة قبل الفترة المحددة</TableCell>
                    <TableCell class="text-end font-mono">-</TableCell>
                    <TableCell class="text-end font-mono">-</TableCell>
                    <TableCell class="text-end font-bold font-mono text-foreground">
                      {formatCurrency(statement.opening_balance, statement.currency)}
                    </TableCell>
                  </TableRow>

                  {#each filteredItems as item (item.id)}
                    <TableRow class="hover:bg-muted/20 border-b border-border/30 text-[11px] font-semibold">
                      <TableCell class="text-start text-muted-foreground font-mono whitespace-nowrap">{formatDate(item.date)}</TableCell>
                      <TableCell class="text-start font-mono text-foreground font-bold whitespace-nowrap">{item.reference}</TableCell>
                      <TableCell class="text-start text-foreground max-w-xs md:max-w-md break-words font-medium">
                        {item.description_ar || item.description}
                      </TableCell>
                      <TableCell class="text-end font-bold font-mono text-emerald-500 whitespace-nowrap">
                        {item.debit ? formatCurrency(item.debit, statement.currency) : "-"}
                      </TableCell>
                      <TableCell class="text-end font-bold font-mono text-rose-500 whitespace-nowrap">
                        {item.credit ? formatCurrency(item.credit, statement.currency) : "-"}
                      </TableCell>
                      <TableCell class="text-end font-bold font-mono text-foreground whitespace-nowrap font-extrabold">
                        {formatCurrency(item.running_balance, statement.currency)}
                      </TableCell>
                    </TableRow>
                  {/each}
                </TableBody>
              </Table>
            </div>
          {/if}
        {/if}
      {/if}
    </div>
  </div>
</div>

<style>
@media print {
  :global(body) {
    background-color: white !important;
    color: black !important;
  }
  :global(main) {
    background-color: white !important;
  }
  .print\:hidden {
    display: none !important;
  }
  .print\:flex {
    display: flex !important;
  }
}
</style>
