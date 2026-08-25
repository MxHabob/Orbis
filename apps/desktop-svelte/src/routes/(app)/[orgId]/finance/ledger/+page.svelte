<!--
@component FinanceLedgerPage
Account Statement / Trial Balance (كشف حساب تفصيلي)
Groups movements by currency (YER, SAR, USD, AED) and lists account codes and names.
Provides All, Single, and Custom selection mode with interactive Accounts Tree sidebar.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import { commands } from "$lib/core/ipc";
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
import { DataTable } from "$lib/tables";
import type { OrbisColumnDef } from "@orbis/svelte-grid";
import CurrencySelector from "$lib/shared/components/currency-selector.svelte";
import AccountTree from "$lib/features/finance/components/account-tree.svelte";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogDescription,
} from "@orbis/ui-svelte/dialog";
import {
  Printer,
  RotateCw,
  LogOut,
  FolderOpen,
} from "@lucide/svelte";
import { getLocale } from "$lib/paraglide/runtime";
import { formatCurrency, formatDate } from "$lib/core/i18n";
import { fade } from "svelte/transition";

let { data }: { data: { orgId: string } } = $props();

// State variables
let ledgers = $state<any[]>([]);
let payments = $state<any[]>([]);
let fees = $state<any[]>([]);
let dbCurrencies = $state<any[]>([]);
let isLoading = $state(true);
let error = $state<string | null>(null);
let isAddDialogOpen = $state(false);

function handleSuccess() {
  isAddDialogOpen = false;
  loadData();
}

// Filter states
let activeTab = $state<"all" | "single">("all");
let selectedAccountId = $state<string>("");
let selectedCurrency = $state<string>("all");
let periodFilter = $state<string>("all");
let customStartDate = $state<string>(new Date().toISOString().split("T")[0]);
let searchQuery = $state("");
let movementFilter = $state<string>("all"); // 'all' or 'active' (with movements)
let debtorCreditorFilter = $state<string>("all"); // 'all', 'debtor', 'creditor'

// Tree state
let selectedTreeAccountIds = $state<Set<string>>(new Set());

// Synced inputs for dual-combobox
let accountCodeQuery = $state("");
let accountNameQuery = $state("");
let isCodeDropdownOpen = $state(false);
let isNameDropdownOpen = $state(false);

const parentAccountIds = $derived(
  new Set(ledgers.filter((account) => account.parent_id).map((account) => account.parent_id)),
);
const leafLedgers = $derived(
  ledgers.filter((account) => !parentAccountIds.has(account.id)),
);

const searchedAccountsByCode = $derived.by(() => {
  const q = accountCodeQuery.toLowerCase().trim();
  if (!q) return leafLedgers;
  return leafLedgers.filter((l) => String(l.code).includes(q));
});

const searchedAccountsByName = $derived.by(() => {
  const q = accountNameQuery.toLowerCase().trim();
  if (!q) return leafLedgers;
  return leafLedgers.filter(
    (l) =>
      String(l.name_ar || "")
        .toLowerCase()
        .includes(q) ||
      String(l.name || "")
        .toLowerCase()
        .includes(q),
  );
});

function selectAccount(acc: any) {
  selectedAccountId = acc.id;
  accountCodeQuery = String(acc.code);
  accountNameQuery = acc.name_ar || acc.name;
  isCodeDropdownOpen = false;
  isNameDropdownOpen = false;
}

function resetQueries() {
  const active = ledgers.find((l) => l.id === selectedAccountId);
  if (active) {
    accountCodeQuery = String(active.code);
    accountNameQuery = active.name_ar || active.name;
  } else {
    accountCodeQuery = "";
    accountNameQuery = "";
  }
}

function handleWindowClick(event: MouseEvent) {
  const target = event.target as HTMLElement;
  if (!target.closest(".combobox-container")) {
    isCodeDropdownOpen = false;
    isNameDropdownOpen = false;
    resetQueries();
  }
}

async function loadData() {
  isLoading = true;
  error = null;
  try {
    const [ledgerList, paymentList, feeList, currencyList] = await Promise.all([
      commands.finance.listAccounts(),
      commands.finance.listPayments(null, null),
      commands.finance.listStudentFeesByOrg(),
      commands.finance.listCurrencies().catch(() => []),
    ]);

    ledgers = ledgerList;
    payments = paymentList;
    fees = feeList;
    dbCurrencies = currencyList || [];

    const leafAccounts = ledgerList.filter((account: any) => !ledgerList.some((candidate: any) => candidate.parent_id === account.id));
    if (leafAccounts.length > 0) {
      selectedAccountId = leafAccounts[0].id as string;
      accountCodeQuery = String(leafAccounts[0].code);
      accountNameQuery =
        (leafAccounts[0].name_ar as string) || (leafAccounts[0].name as string);
    }

    // Default select all accounts in tree selection
    const allIds = new Set<string>();
    leafAccounts.forEach((acc: any) => allIds.add(acc.id));
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
  }
});

// Double entry ledger balance calculation
const calculatedLedgerRows = $derived.by(() => {
  return ledgers.map((acc: any) => {
    const balances = acc.balances || {};
    return {
      id: acc.id,
      code: acc.code,
      name: acc.name,
      name_ar: acc.name_ar || acc.name,
      item_type: acc.type || acc.item_type || "asset",
      is_control_account: acc.is_control_account || 0,
      parent_id: acc.parent_id,
      balances,
      notes: acc.sub_type || "",
      debit_yer: Number(balances.YER || 0) > 0 ? Number(balances.YER) : 0,
      credit_yer: Number(balances.YER || 0) < 0 ? Math.abs(Number(balances.YER)) : 0,
      debit_usd: Number(balances.USD || 0) > 0 ? Number(balances.USD) : 0,
      credit_usd: Number(balances.USD || 0) < 0 ? Math.abs(Number(balances.USD)) : 0,
      debit_sar: Number(balances.SAR || 0) > 0 ? Number(balances.SAR) : 0,
      credit_sar: Number(balances.SAR || 0) < 0 ? Math.abs(Number(balances.SAR)) : 0,
      debit_aed: Number(balances.AED || 0) > 0 ? Number(balances.AED) : 0,
      credit_aed: Number(balances.AED || 0) < 0 ? Math.abs(Number(balances.AED)) : 0,
    };
  });
});

// Selected Account Row derived state
const selectedAccountRow = $derived.by(() => {
  return calculatedLedgerRows.find((r) => r.id === selectedAccountId);
});

const selectableLedgerRows = $derived(
  calculatedLedgerRows.filter((row) => !parentAccountIds.has(row.id) && row.is_control_account !== 1),
);

const activeCurrencies = $derived(
  dbCurrencies
    .map((currency) => ({
      code: String(currency.code || "").toUpperCase(),
      name: currency.name_ar || currency.name || currency.code,
    }))
    .filter((currency) => currency.code),
);

const ledgerGridColumns = $derived.by((): OrbisColumnDef[] => {
  const currencyColumns = activeCurrencies.flatMap((currency) => [
    {
      id: `${currency.code}_debit`,
      prop: `${currency.code}_debit`,
      header: `${currency.name} - ${m.common_ledger_col_debit()}`,
      name: `${currency.name} - ${m.common_ledger_col_debit()}`,
      width: 120,
      size: 120,
      readonly: true,
      cellClass: "orbis-cell-number",
    },
    {
      id: `${currency.code}_credit`,
      prop: `${currency.code}_credit`,
      header: `${currency.name} - Credit`,
      name: `${currency.name} - Credit`,
      width: 120,
      size: 120,
      readonly: true,
      cellClass: "orbis-cell-number",
    },
  ]);

  return [
    { id: "__idx", prop: "__idx", header: "#", name: "#", width: 48, size: 48, readonly: true },
    { id: "name_ar", prop: "name_ar", header: m.common_ledger_col_name(), name: m.common_ledger_col_name(), width: 220, size: 220, readonly: true },
    { id: "code", prop: "code", header: m.common_ledger_col_code(), name: m.common_ledger_col_code(), width: 100, size: 100, readonly: true, cellClass: "font-mono" },
    ...currencyColumns,
    { id: "notes", prop: "notes", header: m.common_year_notes(), name: m.common_year_notes(), width: 180, size: 180, readonly: true },
  ];
});

// Arabic spelling helper
function arabicNumberToWords(num: number): string {
  if (num === 0) return "صفر";

  const integerPart = Math.floor(num);
  const decimalPart = Math.round((num - integerPart) * 100);

  const ones = [
    "",
    "واحد",
    "اثنان",
    "ثلاثة",
    "أربعة",
    "خمسة",
    "ستة",
    "سبعة",
    "ثمانية",
    "تسعة",
    "عشرة",
    "أحد عشر",
    "اثنا عشر",
    "ثلاثة عشر",
    "أربعة عشر",
    "خمسة عشر",
    "ستة عشر",
    "سبعة عشر",
    "ثمانية عشر",
    "تسعة عشر",
  ];
  const tens = [
    "",
    "",
    "عشرون",
    "ثلاثون",
    "أربعون",
    "خمسون",
    "ستون",
    "سبعون",
    "ثمانون",
    "تسعون",
  ];
  const hundreds = [
    "",
    "مائة",
    "مائتان",
    "ثلاثمائة",
    "أربعمائة",
    "خمسمائة",
    "ستمائة",
    "سبعمائة",
    "ثمانمائة",
    "تسعمائة",
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
      billions === 1
        ? "مليار"
        : billions === 2
          ? "ملياران"
          : `${convertGroup(billions)} مليارات`,
    );
  }

  const millions = Math.floor(temp / 1000000);
  temp %= 1000000;
  if (millions > 0) {
    parts.push(
      millions === 1
        ? "مليون"
        : millions === 2
          ? "مليونان"
          : `${convertGroup(millions)} ملايين`,
    );
  }

  const thousands = Math.floor(temp / 1000);
  temp %= 1000;
  if (thousands > 0) {
    parts.push(
      thousands === 1
        ? "ألف"
        : thousands === 2
          ? "ألفان"
          : `${convertGroup(thousands)} آلاف`,
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

// Currency Summary Rows derived state (for Single Account, All Currencies)
const currencySummaryRows = $derived.by(() => {
  const row = selectedAccountRow;
  if (!row) return [];

  const list = [];
  for (const [currency, balanceVal] of Object.entries(row.balances)) {
    const balance = Number(balanceVal);
    if (balance === 0) continue;

    // Mock rates for demonstration since actual exchange rates aren't fetched in this view yet
    const rate =
      currency === "USD"
        ? 530.2
        : currency === "SAR"
          ? 139.8
          : currency === "AED"
            ? 144.1
            : 1.0;

    // If account is Asset (Debit normal), a positive balance means debit.
    // If Liability (Credit normal), a positive balance means credit.
    // In our backend schema, positive usually means debit and negative means credit, or vice versa depending on normal balance flag.
    // Let's assume standard: positive = Debit, negative = Credit.
    const isDebit = balance > 0;
    const absBalance = Math.abs(balance);

    list.push({
      currency: currency,
      debit: isDebit ? absBalance : 0,
      credit: isDebit ? 0 : absBalance,
      rate: rate,
      debit_eq: (isDebit ? absBalance : 0) * rate,
      credit_eq: (isDebit ? 0 : absBalance) * rate,
      notes: `رصيد الحساب بـ ${currency}`,
    });
  }

  return list;
});

const summaryTotals = $derived.by(() => {
  let totDebEq = 0;
  let totCredEq = 0;
  for (const r of currencySummaryRows) {
    totDebEq += r.debit_eq;
    totCredEq += r.credit_eq;
  }
  const balance = totDebEq - totCredEq;
  return {
    debit_eq: totDebEq,
    credit_eq: totCredEq,
    balance: Math.abs(balance),
    balance_type: balance >= 0 ? "debtor" : "creditor", // مدين أو دائن
  };
});

const balanceText = $derived.by(() => {
  const tot = summaryTotals;
  const label = tot.balance_type === "debtor" ? "الرصيد عليكم " : "الرصيد لكم ";
  return label + arabicNumberToWords(tot.balance);
});

// Detailed Transaction Ledger (for Single Account, Specific Currency)
const ledgerTransactions = $derived.by(() => {
  if (
    activeTab !== "single" ||
    !selectedAccountId ||
    selectedCurrency === "all"
  ) {
    return [];
  }

  const acc = ledgers.find((l) => l.id === selectedAccountId);
  if (!acc) return [];

  const list: any[] = [];
  const targetCurr = selectedCurrency.toUpperCase();

  // 1. Add opening balance if any
  const ob = Number(acc.opening_balance || 0);
  if (ob > 0 && String(acc.currency).toUpperCase() === targetCurr) {
    const isDebitType =
      acc.item_type === "asset" || acc.item_type === "expense";
    list.push({
      date: "2026-06-01",
      document: "رصيد سابق",
      number: "-",
      description: "رصيد أول المدة",
      debit: isDebitType ? ob : 0,
      credit: isDebitType ? 0 : ob,
      status: "verified",
    });
  }

  // Find if this account is the Student Tuition Receivables account
  const isAR =
    acc.code === "1301" ||
    String(acc.name_ar).includes("ذمم") ||
    String(acc.name).toLowerCase().includes("receivable");

  // 2. Add Fees (Debit AR, Credit Revenue)
  if (targetCurr === "YER") {
    // Fees are in YER
    if (isAR) {
      // Debit Student Receivables
      for (const fee of fees) {
        list.push({
          date: String(fee.created_at || "2026-06-22").split("T")[0],
          document: "سند قيد بسيط",
          number: fee.id.slice(0, 8),
          description:
            fee.notes ||
            `رسوم دراسية مستحقة - كود القالب ${fee.fee_template_id}`,
          debit: Number(fee.total_amount || 0),
          credit: 0,
          status: "verified",
        });
      }
    } else if (acc.item_type === "revenue") {
      // Credit Revenue Account
      for (const fee of fees) {
        if (fee.fee_template_id === acc.id) {
          list.push({
            date: String(fee.created_at || "2026-06-22").split("T")[0],
            document: "سند قيد بسيط",
            number: fee.id.slice(0, 8),
            description: fee.notes || "إثبات إيرادات رسوم دراسية مستحقة",
            debit: 0,
            credit: Number(fee.total_amount || 0),
            status: "verified",
          });
        }
      }
    }
  }

  // 3. Add Payments (Debit Treasury, Credit AR)
  for (const pay of payments) {
    const payCurr = String(pay.currency || "YER").toUpperCase();
    if (payCurr !== targetCurr) continue;

    const amount = Number(pay.amount || 0);

    if (isAR) {
      // Credit AR
      list.push({
        date: String(pay.paid_at || "2026-06-22").split("T")[0],
        document: "سند قبض",
        number: pay.receipt_number || pay.id.slice(0, 8),
        description:
          pay.notes ||
          `دفعة رسوم مسددة من الطالب - مرجع ${pay.reference || ""}`,
        debit: 0,
        credit: amount,
        status: pay.is_verified === 1 ? "verified" : "pending",
      });
    } else {
      // Check if this is the treasury ledger account
      if (pay.cash_account_id === acc.id) {
        // Debit Treasury
        list.push({
          date: String(pay.paid_at || "2026-06-22").split("T")[0],
          document: "سند قبض",
          number: pay.receipt_number || pay.id.slice(0, 8),
          description:
            pay.notes ||
            `مقبوضات الرسوم الدراسية - دفعة ${pay.payer_name || ""}`,
          debit: amount,
          credit: 0,
          status: pay.is_verified === 1 ? "verified" : "pending",
        });
      }
    }
  }

  // Sort by date
  list.sort((a, b) => a.date.localeCompare(b.date));

  // Compute running balance
  const isDebitNormal =
    acc.item_type === "asset" || acc.item_type === "expense";

  // In computed entries:
  let cumulativeBalance = 0; // represent as Credit - Debit
  const entries = list.map((item) => {
    if (isDebitNormal) {
      cumulativeBalance += item.debit - item.credit;
    } else {
      cumulativeBalance += item.credit - item.debit;
    }

    return {
      ...item,
      balance: Math.abs(cumulativeBalance),
      balance_label: cumulativeBalance >= 0 ? "له" : "عليه",
    };
  });

  return entries;
});

// Detailed transactions totals
const detailedTotals = $derived.by(() => {
  let totDeb = 0;
  let totCred = 0;
  for (const r of ledgerTransactions) {
    totDeb += r.debit;
    totCred += r.credit;
  }

  const acc = ledgers.find((l) => l.id === selectedAccountId);
  const isDebitNormal = acc
    ? acc.item_type === "asset" || acc.item_type === "expense"
    : true;

  let netBalance = 0;
  if (isDebitNormal) {
    netBalance = totDeb - totCred;
  } else {
    netBalance = totCred - totDeb;
  }

  return {
    debit: totDeb,
    credit: totCred,
    balance: Math.abs(netBalance),
    balance_label: netBalance >= 0 ? "له" : "عليه",
  };
});

const totals = $derived({ debY: 0, credY: 0, debS: 0, credS: 0, debU: 0, credU: 0, debA: 0, credA: 0 });
const tableA2Rows = $derived<any[]>([]);
const tableA2Totals = $derived({ debit: 0, credit: 0, balance: 0, balance_label: "له" });

// Categorized tree representation of all accounts (used for Custom Selection sidebar)
const treeCategories = $derived.by(() => {
  const cats: Record<
    string,
    { id: string; name_ar: string; name_en: string; accounts: any[] }
  > = {
    asset: {
      id: "asset",
      name_ar: "الأصول (1)",
      name_en: "Assets",
      accounts: [],
    },
    liability: {
      id: "liability",
      name_ar: "الخصوم (2)",
      name_en: "Liabilities",
      accounts: [],
    },
    expense: {
      id: "expense",
      name_ar: "المصروفات (3)",
      name_en: "Expenses",
      accounts: [],
    },
    revenue: {
      id: "revenue",
      name_ar: "الإيرادات (4)",
      name_en: "Revenues",
      accounts: [],
    },
    equity: {
      id: "equity",
      name_ar: "حقوق الملكية (5)",
      name_en: "Equity",
      accounts: [],
    },
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

// Active filtered rows to display in the main ledger table
const filteredLedgerRows = $derived.by(() => {
  return selectableLedgerRows.filter((row) => {
    const query = searchQuery.toLowerCase().trim();
    if (query && !String(row.name_ar || row.name).toLowerCase().includes(query) && !String(row.code).toLowerCase().includes(query)) {
      return false;
    }

    // 1. Account Filtering by Tab Mode
    if (activeTab === "single") {
      if (row.id !== selectedAccountId) return false;
    } else if (activeTab === "all") {
      if (!selectedTreeAccountIds.has(row.id)) return false;
      if (parentAccountIds.has(row.id)) return false;
    }

    // 2. Currency Filter
    if (selectedCurrency !== "all") {
      const c = selectedCurrency.toUpperCase();
      if (c === "YER" && row.debit_yer === 0 && row.credit_yer === 0)
        return false;
      if (c === "SAR" && row.debit_sar === 0 && row.credit_sar === 0)
        return false;
      if (c === "USD" && row.debit_usd === 0 && row.credit_usd === 0)
        return false;
      if (c === "AED" && row.debit_aed === 0 && row.credit_aed === 0)
        return false;
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

const ledgerGridSource = $derived(
  filteredLedgerRows.map((row, index) => {
    const source: Record<string, unknown> = {
      __idx: index + 1,
      name_ar: row.name_ar,
      code: row.code,
      notes: row.notes || "-",
    };

    for (const currency of activeCurrencies) {
      const values = row as Record<string, unknown>;
      const code = currency.code.toLowerCase();
      const debit = Number(values[`debit_${code}`] || 0);
      const credit = Number(values[`credit_${code}`] || 0);
      source[`${currency.code}_debit`] = debit ? formatCurrency(debit, currency.code) : "-";
      source[`${currency.code}_credit`] = credit ? formatCurrency(credit, currency.code) : "-";
    }

    return source;
  }),
);

function handlePrint() {
  window.print();
}

function exitPage() {
  goto(`/${data.orgId}/finance`);
}
</script>

<svelte:window onclick={handleWindowClick} />

<div class="finance-ledger-page h-full w-full overflow-hidden bg-background text-foreground">
  <!-- Main Filter Controls Box -->
  <div class="p-5 bg-card/15 border-b border-border/80 grid grid-cols-1 lg:grid-cols-4 gap-6 shrink-0 no-print select-none text-start">
    
    <!-- Right Side (Renders on right in RTL): Accounts Tree Selection Box (spans 1 column) -->
    <AccountTree
      categories={treeCategories}
      bind:selectedIds={selectedTreeAccountIds}
    />
    <!-- Left Side (Renders on left in RTL): Filter inputs (spans 3 columns) -->
    <div class="lg:col-span-3 flex flex-col gap-4 justify-between bg-card/5 border border-border/50 rounded-xl p-4 shadow-inner">
      
      <!-- Mode Tabs and Top Status Indicator -->
      <div class="flex items-center justify-between border-b border-border/60 pb-3">
        <!-- Mode Tabs -->
        <div class="flex border border-border bg-muted/50 rounded-lg p-0.5 text-[10px] font-bold">
          <button
            onclick={() => activeTab = "single"}
            class="px-5 py-1.5 rounded-md transition-all duration-150 {activeTab === 'single' ? 'bg-primary text-primary-foreground shadow-sm font-bold' : 'text-muted-foreground hover:text-foreground font-semibold'}"
          >
            {m.common_ledger_tab_one()}
          </button>
          <button
            onclick={() => activeTab = "all"}
            class="px-5 py-1.5 rounded-md transition-all duration-150 {activeTab === 'all' ? 'bg-primary text-primary-foreground shadow-sm font-bold' : 'text-muted-foreground hover:text-foreground font-semibold'}"
          >
            {m.common_ledger_tab_all()}
          </button>
        </div>
        
        <!-- Toggle Status -->
        <span class="text-[10px] text-muted-foreground/80 font-bold bg-muted/40 border border-border/30 px-3 py-1 rounded-md">
          {activeTab === 'single' ? 'حساب واحد مفعل' : 'كافة الحسابات مفعله'}
        </span>
      </div>

      <div class="flex flex-col gap-4 w-full">
        
        <!-- Row 1: Code and Name comboboxes side-by-side -->
        <div class="flex gap-4 w-full">
          <!-- Name combobox selector -->
          <div class="flex-1 flex flex-col gap-1 text-[9px] font-bold text-muted-foreground combobox-container relative font-sans">
            <span>{m.common_ledger_col_name()}</span>
            <div class="relative">
              <input
                type="text"
                placeholder="ابحث بالاسم المكتوب لتصفية الشجرة..."
                bind:value={accountNameQuery}
                onfocus={() => isNameDropdownOpen = true}
                disabled={activeTab !== "single" || isLoading}
                class="w-full bg-background text-foreground border border-border rounded-md px-3 py-1 text-[11px] font-sans outline-none focus:ring-1 focus:ring-ring disabled:opacity-50 text-start cursor-pointer"
              />
              {#if isNameDropdownOpen && activeTab === "single" && !isLoading}
                <div class="absolute z-50 left-0 right-0 mt-1 max-h-48 overflow-y-auto bg-popover border border-border rounded-md shadow-lg flex flex-col p-1 text-[10px]" transition:fade={{ duration: 80 }}>
                  {#if searchedAccountsByName.length === 0}
                    <span class="text-[10px] text-muted-foreground p-2 text-center">{m.finance_statement_no_match()}</span>
                  {:else}
                    {#each searchedAccountsByName as acc (acc.id)}
                      <button
                        type="button"
                        onclick={() => selectAccount(acc)}
                        class="w-full text-start px-2 py-1 rounded hover:bg-accent hover:text-accent-foreground font-sans {selectedAccountId === acc.id ? 'bg-primary/10 text-primary font-bold' : 'text-foreground'}"
                      >
                        {acc.code} - {acc.name_ar || acc.name}
                      </button>
                    {/each}
                  {/if}
                </div>
              {/if}
            </div>
          </div>

          <!-- Code Search Combobox (shrunk) -->
          <div class="flex flex-col gap-1 text-[9px] font-bold text-muted-foreground combobox-container relative w-1/4 max-w-40 min-w-25">
            <span>{m.finance_ledger_account_code()}</span>
            <div class="relative">
              <input
                type="text"
                placeholder="الكود..."
                bind:value={accountCodeQuery}
                onfocus={() => isCodeDropdownOpen = true}
                disabled={activeTab !== "single" || isLoading}
                class="w-full bg-background text-foreground border border-border rounded-md px-2.5 py-1 text-[11px] font-mono outline-none focus:ring-1 focus:ring-ring disabled:opacity-50 text-start cursor-pointer"
              />
              {#if isCodeDropdownOpen && activeTab === "single" && !isLoading}
                <div class="absolute z-50 left-0 right-0 mt-1 max-h-48 overflow-y-auto bg-popover border border-border rounded-md shadow-lg flex flex-col p-1 text-[10px]" transition:fade={{ duration: 80 }}>
                  {#if searchedAccountsByCode.length === 0}
                    <span class="text-[10px] text-muted-foreground p-2 text-center">{m.finance_ledger_no_accounts()}</span>
                  {:else}
                    {#each searchedAccountsByCode as acc (acc.id)}
                      <button
                        type="button"
                        onclick={() => selectAccount(acc)}
                        class="w-full text-start px-2 py-1 rounded hover:bg-accent hover:text-accent-foreground font-mono {selectedAccountId === acc.id ? 'bg-primary/10 text-primary font-bold' : 'text-foreground'}"
                      >
                        {acc.code}
                      </button>
                    {/each}
                  {/if}
                </div>
              {/if}
            </div>
          </div>
        </div>

        <!-- Row 2: Currency, Date, and Period Selectors -->
        <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
          <!-- Currency Selector -->
          <CurrencySelector
            title={m.common_ledger_currency()}
            description=""
            currentCurrency={selectedCurrency}
            includeAll={true}
            allLabel={m.common_ledger_all_currencies()}
            compact={true}
            onchange={(currency) => selectedCurrency = currency}
          />

          <!-- Date picker -->
          <div class="flex flex-col gap-1 text-[9px] font-bold text-muted-foreground">
            <span>{selectedCurrency === "all" ? m.finance_ledger_until_day() : m.finance_ledger_during_day()}</span>
            <Input
              type="date"
              bind:value={customStartDate}
              class="input-sunken py-1 px-2.5 text-[11px] w-full"
            />
          </div>
          
          <!-- Period Selector -->
          <div class="flex flex-col gap-1 text-[9px] font-bold text-muted-foreground">
            <span>{m.common_metrics_period()}</span>
            <select
              bind:value={periodFilter}
              class="bg-background text-foreground border border-border rounded-md px-2.5 py-1 text-[11px] outline-none focus:ring-1 focus:ring-ring font-sans w-full"
            >
              <option value="all">{m.common_ledger_period_all()}</option>
              <option value="day">{m.common_ledger_period()}</option>
              <option value="week">{m.common_ledger_period_week()}</option>
              <option value="month">{m.common_ledger_period_month()}</option>
              <option value="year">{m.common_ledger_period_year()}</option>
            </select>
          </div>
        </div>
      </div>
    </div>
  </div>

  <!-- Unified Toolbar Row under Filter Box -->
  <div class="p-3 border-b border-border bg-card/35 flex justify-between items-center shrink-0 no-print select-none px-5">
    <!-- Exit button on the left -->
    <Button
      variant="ghost"
      class="text-xs h-9 px-5 border border-border flex items-center gap-2 font-bold text-red-500 hover:text-red-600 rounded-md"
      onclick={exitPage}
    >
      <LogOut size={13} class={getLocale() === "ar" ? 'rotate-180' : ''} />
      <span>{m.common_ledger_action_exit()}</span>
    </Button>

    <!-- Toolbar controls and Print actions -->
    <div class="flex items-center gap-3">
      <!-- Add Account / Treasury -->
      <Button
        variant="ghost"
        class="text-xs h-9 px-4 border border-border flex items-center gap-2 font-bold rounded-md text-primary"
        onclick={() => isAddDialogOpen = true}
      >
        <FolderOpen size={13} />
        <span>{m.common_ledger_btn_add_account()}</span>
      </Button>

      <!-- 1. Print Selected -->
      <Button
        variant="ghost"
        class="text-xs h-9 px-4 border border-border flex items-center gap-2 font-bold rounded-md"
        onclick={handlePrint}
      >
        <Printer size={13} />
        <span>{m.common_ledger_action_print_selected()}</span>
      </Button>

      <!-- 2. Print -->
      <Button
        variant="ghost"
        class="text-xs h-9 px-4 border border-border flex items-center gap-2 font-bold rounded-md"
        onclick={handlePrint}
      >
        <Printer size={13} />
        <span>{m.common_ledger_action_print()}</span>
      </Button>

      <!-- 3. View / Show (Main Green Action) -->
      <Button
        variant="lime"
        class="text-xs h-9 px-6 flex items-center gap-2 font-black bg-primary text-primary-foreground hover:bg-primary/90 rounded-md min-w-28 justify-center"
        onclick={loadData}
        disabled={isLoading}
      >
        <RotateCw size={13} class={isLoading ? 'animate-spin' : ''} />
        <span>{m.common_ledger_action_view()}</span>
      </Button>
    </div>
  </div>

  <!-- Content Split Layout (Table on left, Tree Selection Sidebar on right if Custom selection active) -->
  <div class="flex-1 w-full overflow-hidden flex relative">
    
    <!-- Table Display Area -->
    <div class="flex-1 h-full overflow-auto p-5 select-none print-full-width">
      
      <!-- Printable Page Header -->
      <div class="hidden print-only mb-6 text-center select-none" dir={getLocale() === "ar" ? "rtl" : "ltr"}>
        <h1 class="text-lg font-bold border-b border-black pb-2">{m.common_ledger_title()}</h1>
        <div class="flex justify-between text-xs mt-3">
          <span>التاريخ: {formatDate(new Date().toISOString())}</span>
          <span>العملة المحددة: {selectedCurrency === 'all' ? m.common_ledger_all_currencies() : selectedCurrency}</span>
          <span>الفترة: {periodFilter === 'all' ? m.common_ledger_period_all() : periodFilter}</span>
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
          <Button variant="outline" onclick={loadData} class="text-[10px] mt-2">{m.common_retry()}</Button>
        </div>
      {:else}
        {#if activeTab === "all"}
          <DataTable
            title={m.common_ledger_title()}
            subtitle={m.common_ledger_all_currencies()}
            columns={ledgerGridColumns}
            source={ledgerGridSource}
            loading={isLoading}
            searchable={true}
            searchValue={searchQuery}
            onSearch={(query) => searchQuery = query}
            searchPlaceholder={m.finance_fees_search_placeholder()}
            refreshable={true}
            onRefresh={loadData}
            exportable={true}
            rtl={getLocale() === "ar"}
            height="100%"
            rowSize={38}
            theme="compact"
          />
          {#if false}
            <!-- Excel Grid Sheet Ledger -->
            <div class="border border-border rounded-xl bg-card/30 overflow-hidden min-w-225 print-table" in:fade={{ duration: 150 }}>
              <Table class="border-collapse w-full">
                <TableHeader class="bg-card">
                  <!-- Top Currency Header Row -->
                  <TableRow class="hover:bg-transparent border-b border-border/80">
                    <TableHead rowspan={2} class="w-10 border-r border-border/60 text-center font-bold text-foreground">*</TableHead>
                    <TableHead colspan={2} class="border-r border-border/60 text-center font-bold text-foreground bg-primary/5">ريال يمني</TableHead>
                    <TableHead colspan={2} class="border-r border-border/60 text-center font-bold text-foreground bg-emerald-500/5">سعودي</TableHead>
                    <TableHead colspan={2} class="border-r border-border/60 text-center font-bold text-foreground bg-blue-500/5">دولار</TableHead>
                    <TableHead colspan={2} class="border-r border-border/60 text-center font-bold text-foreground bg-purple-500/5">درهم</TableHead>
                    <TableHead rowspan={2} class="border-r border-border/60 text-start font-bold text-foreground">{m.common_ledger_col_name()}</TableHead>
                    <TableHead rowspan={2} class="border-r border-border/60 text-center font-bold text-foreground w-28">{m.common_ledger_col_code()}</TableHead>
                    <TableHead rowspan={2} class="text-start font-bold text-foreground w-64">{m.common_year_notes()}</TableHead>
                  </TableRow>
                  <!-- Sub-columns Debit / Credit -->
                  <TableRow class="hover:bg-transparent border-b border-border/80 text-[9px] font-bold">
                    <TableHead class="text-center border-r border-border/40 w-24 text-red-500">{m.common_ledger_col_debit()}</TableHead>
                    <TableHead class="text-center border-r border-border/60 w-24 text-emerald-500">{m.common_action_edit()}</TableHead>
                    <TableHead class="text-center border-r border-border/40 w-24 text-red-500">{m.common_ledger_col_debit()}</TableHead>
                    <TableHead class="text-center border-r border-border/60 w-24 text-emerald-500">{m.common_action_edit()}</TableHead>
                    <TableHead class="text-center border-r border-border/40 w-24 text-red-500">{m.common_ledger_col_debit()}</TableHead>
                    <TableHead class="text-center border-r border-border/60 w-24 text-emerald-500">{m.common_action_edit()}</TableHead>
                    <TableHead class="text-center border-r border-border/40 w-24 text-red-500">{m.common_ledger_col_debit()}</TableHead>
                    <TableHead class="text-center border-r border-border/60 w-24 text-emerald-500">{m.common_action_edit()}</TableHead>
                  </TableRow>
                </TableHeader>
                <TableBody>
                  {#each filteredLedgerRows as rowItem, index (rowItem.id)}
                    {@const row = rowItem as any}
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
                        {row.notes || "-"}
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
    
                    <TableCell class="border-r border-border/40 text-start text-foreground uppercase tracking-wider text-[9px]">{m.finance_ledger_grand_total()}</TableCell>
                    <TableCell class="border-r border-border/40 text-center text-foreground font-bold">-</TableCell>
                    <TableCell class="text-start text-muted-foreground">-</TableCell>
                  </TableRow>
                </tfoot>
              </Table>
            </div>
          {:else if false}
            <!-- Table A2: Single-currency Ledger Grid for All Accounts -->
            <div class="border border-border rounded-xl bg-card/30 overflow-hidden min-w-225 print-table" in:fade={{ duration: 150 }}>
              <Table class="border-collapse w-full">
                <TableHeader class="bg-card">
                  <TableRow class="hover:bg-transparent border-b border-border/80 text-[10px] font-bold text-foreground">
                    <TableHead class="w-10 border-r border-border/60 text-center font-bold">*</TableHead>
                    <TableHead class="text-center border-r border-border/60 w-28 text-red-500">{m.common_ledger_col_debit()}</TableHead>
                    <TableHead class="text-center border-r border-border/60 w-28 text-emerald-500">{m.common_action_edit()}</TableHead>
                    <TableHead class="text-center border-r border-border/60 w-32 font-bold">{m.finance_ledger_balance()}</TableHead>
                    <TableHead class="text-start border-r border-border/60 font-bold">{m.common_ledger_col_name()}</TableHead>
                    <TableHead class="text-center border-r border-border/60 w-28 font-bold">{m.common_ledger_col_code()}</TableHead>
                    <TableHead class="text-start font-bold w-64">{m.common_year_notes()}</TableHead>
                  </TableRow>
                </TableHeader>
                <TableBody>
                  {#each tableA2Rows as row, index (row.id)}
                    <TableRow class="hover:bg-accent/10 border-b border-border/40 text-[10px] leading-normal font-medium">
                      <TableCell class="border-r border-border/40 text-center text-muted-foreground font-mono">{index + 1}</TableCell>
                      <TableCell class="border-r border-border/40 text-center font-mono {row.debit > 0 ? 'text-foreground font-semibold' : 'text-muted-foreground/30'}">
                        {row.debit > 0 ? formatCurrency(row.debit, "").replace(/YER|USD|SAR|AED/g, '') : "0"}
                      </TableCell>
                      <TableCell class="border-r border-border/60 text-center font-mono {row.credit > 0 ? 'text-foreground font-semibold' : 'text-muted-foreground/30'}">
                        {row.credit > 0 ? formatCurrency(row.credit, "").replace(/YER|USD|SAR|AED/g, '') : "0"}
                      </TableCell>
                      <TableCell class="border-r border-border/60 text-center font-mono font-bold text-foreground">
                        {formatCurrency(row.balance, "").replace(/YER|USD|SAR|AED/g, '')} <span class="text-[9px] text-muted-foreground/80 font-sans">{row.balance_label}</span>
                      </TableCell>
                      <TableCell class="border-r border-border/40 text-start font-semibold text-foreground">{row.name_ar}</TableCell>
                      <TableCell class="border-r border-border/40 text-center font-mono text-foreground font-bold">{row.code}</TableCell>
                      <TableCell class="text-start text-muted-foreground select-text text-[9px] line-clamp-1 py-1 truncate">
                        {row.notes || "-"}
                      </TableCell>
                    </TableRow>
                  {/each}
                </TableBody>
                <tfoot>
                  <TableRow class="bg-card hover:bg-transparent font-bold font-mono text-[10px] border-t-2 border-border/90">
                    <TableCell class="border-r border-border/40 text-center text-foreground">Σ</TableCell>
                    <TableCell class="border-r border-border/40 text-center text-red-500 font-bold">
                      {tableA2Totals.debit > 0 ? formatCurrency(tableA2Totals.debit, "").replace(/YER|USD|SAR|AED/g, '') : "0"}
                    </TableCell>
                    <TableCell class="border-r border-border/60 text-center text-emerald-500 font-bold">
                      {tableA2Totals.credit > 0 ? formatCurrency(tableA2Totals.credit, "").replace(/YER|USD|SAR|AED/g, '') : "0"}
                    </TableCell>
                    <TableCell class="border-r border-border/60 text-center text-foreground font-bold">
                      {formatCurrency(tableA2Totals.balance, "").replace(/YER|USD|SAR|AED/g, '')} <span class="text-[9px] text-muted-foreground font-sans">{tableA2Totals.balance_label}</span>
                    </TableCell>
                    <TableCell class="border-r border-border/40 text-start text-foreground uppercase tracking-wider text-[9px]">{m.finance_ledger_grand_total()}</TableCell>
                    <TableCell class="border-r border-border/40 text-center text-foreground font-bold">-</TableCell>
                    <TableCell class="text-start text-muted-foreground">-</TableCell>
                  </TableRow>
                </tfoot>
              </Table>
            </div>
          {/if}
        {:else}
          <!-- Single Account View Swapped Layout -->
          <div class="border border-border rounded-xl bg-card/30 overflow-hidden min-w-225 print-table" in:fade={{ duration: 150 }}>
            {#if selectedCurrency === "all"}
              <!-- Table B: Currency Summary Grid -->
              <Table class="border-collapse w-full">
                <TableHeader class="bg-card">
                  <TableRow class="hover:bg-transparent border-b border-border/80 text-[10px] font-bold text-foreground">
                    <TableHead class="w-12 text-center font-bold border-r border-border/60">*</TableHead>
                    <TableHead colspan={2} class="text-center font-bold border-r border-border/60">{m.common_ft_col_amount()}</TableHead>
                    <TableHead class="text-center font-bold border-r border-border/60 w-32">{m.finance_ledger_currency()}</TableHead>
                    <TableHead colspan={2} class="text-center font-bold border-r border-border/60">{m.finance_ledger_equivalent_in_yer()}</TableHead>
                    <TableHead class="text-start font-bold">{m.common_calendar_field_description()}</TableHead>
                  </TableRow>
                  <TableRow class="hover:bg-transparent border-b border-border/80 text-[9px] font-bold">
                    <TableHead class="border-r border-border/60 text-center">-</TableHead>
                    <TableHead class="text-center border-r border-border/45 text-red-500 w-24">{m.common_ledger_col_debit()}</TableHead>
                    <TableHead class="text-center border-r border-border/60 text-emerald-500 w-24">{m.common_action_edit()}</TableHead>
                    <TableHead class="border-r border-border/60 text-center">-</TableHead>
                    <TableHead class="text-center border-r border-border/45 text-red-500 w-28">{m.common_ledger_col_debit()}</TableHead>
                    <TableHead class="text-center border-r border-border/60 text-emerald-500 w-28">{m.common_action_edit()}</TableHead>
                    <TableHead class="text-start">-</TableHead>
                  </TableRow>
                </TableHeader>
                <TableBody>
                  {#each currencySummaryRows as row, index}
                    <TableRow class="hover:bg-accent/10 border-b border-border/40 text-[10px] leading-normal font-medium">
                      <TableCell class="border-r border-border/40 text-center text-muted-foreground font-mono">{index + 1}</TableCell>
                      
                      <!-- Original Amount -->
                      <TableCell class="border-r border-border/40 text-center font-mono {row.debit > 0 ? 'text-foreground font-semibold' : 'text-muted-foreground/30'}">
                        {row.debit > 0 ? formatCurrency(row.debit, "").replace(/YER|USD|SAR|AED/g, '') : "0"}
                      </TableCell>
                      <TableCell class="border-r border-border/60 text-center font-mono {row.credit > 0 ? 'text-foreground font-semibold' : 'text-muted-foreground/30'}">
                        {row.credit > 0 ? formatCurrency(row.credit, "").replace(/YER|USD|SAR|AED/g, '') : "0"}
                      </TableCell>
                      
                      <!-- Currency -->
                      <TableCell class="border-r border-border/60 text-center font-semibold text-foreground">{row.currency}</TableCell>
                      
                      <!-- Equivalent Amount YER -->
                      <TableCell class="border-r border-border/40 text-center font-mono {row.debit_eq > 0 ? 'text-foreground font-semibold' : 'text-muted-foreground/30'}">
                        {row.debit_eq > 0 ? formatCurrency(row.debit_eq, "").replace(/YER|USD|SAR|AED/g, '') : "0"}
                      </TableCell>
                      <TableCell class="border-r border-border/60 text-center font-mono {row.credit_eq > 0 ? 'text-foreground font-semibold' : 'text-muted-foreground/30'}">
                        {row.credit_eq > 0 ? formatCurrency(row.credit_eq, "").replace(/YER|USD|SAR|AED/g, '') : "0"}
                      </TableCell>
                      
                      <!-- Notes -->
                      <TableCell class="text-start text-muted-foreground font-medium">{row.notes}</TableCell>
                    </TableRow>
                  {/each}
                </TableBody>
                <tfoot>
                  <!-- Equivalent Totals -->
                  <TableRow class="bg-card hover:bg-transparent font-bold font-mono text-[10px] border-t-2 border-border/90">
                    <TableCell class="border-r border-border/40 text-center text-foreground font-semibold">Σ</TableCell>
                    <TableCell class="border-r border-border/40 text-center text-muted-foreground/40">-</TableCell>
                    <TableCell class="border-r border-border/60 text-center text-muted-foreground/40">-</TableCell>
                    <TableCell class="border-r border-border/60 text-center text-foreground font-bold">{m.common_grades_col_total()}</TableCell>
                    <TableCell class="border-r border-border/40 text-center text-red-500 font-bold">
                      {summaryTotals.debit_eq > 0 ? formatCurrency(summaryTotals.debit_eq, "").replace(/YER|USD|SAR|AED/g, '') : "0"}
                    </TableCell>
                    <TableCell class="border-r border-border/60 text-center text-emerald-500 font-bold">
                      {summaryTotals.credit_eq > 0 ? formatCurrency(summaryTotals.credit_eq, "").replace(/YER|USD|SAR|AED/g, '') : "0"}
                    </TableCell>
                    <TableCell class="text-start text-muted-foreground">-</TableCell>
                  </TableRow>
                  
                  <!-- Net Balance Row -->
                  <TableRow class="bg-card hover:bg-transparent font-bold font-mono text-[10px] border-t border-border/50">
                    <TableCell class="border-r border-border/40 text-center text-foreground font-semibold">♣</TableCell>
                    <TableCell class="border-r border-border/40 text-center text-muted-foreground/40">-</TableCell>
                    <TableCell class="border-r border-border/60 text-center text-muted-foreground/40">-</TableCell>
                    <TableCell class="border-r border-border/60 text-center text-foreground font-bold">{m.finance_ledger_balance()}</TableCell>
                    
                    <!-- Net Debit -->
                    <TableCell class="border-r border-border/40 text-center text-red-500 font-bold">
                      {summaryTotals.balance_type === "debtor" ? formatCurrency(summaryTotals.balance, "").replace(/YER|USD|SAR|AED/g, '') : "0"}
                    </TableCell>
                    <!-- Net Credit -->
                    <TableCell class="border-r border-border/60 text-center text-emerald-500 font-bold">
                      {summaryTotals.balance_type === "creditor" ? formatCurrency(summaryTotals.balance, "").replace(/YER|USD|SAR|AED/g, '') : "0"}
                    </TableCell>
                    
                    <!-- Arabic spelled numbers -->
                    <TableCell class="text-start font-medium text-primary text-[9px] font-sans px-3">
                      {balanceText}
                    </TableCell>
                  </TableRow>
                </tfoot>
              </Table>
            {:else}
              <!-- Table C: Detailed Transaction Ledger -->
              <Table class="border-collapse w-full">
                <TableHeader class="bg-card">
                  <TableRow class="hover:bg-transparent border-b border-border/80 text-[10px] font-bold text-foreground">
                    <TableHead class="w-28 text-center text-red-500 border-r border-border/60">{m.common_ledger_col_debit()}</TableHead>
                    <TableHead class="w-28 text-center text-emerald-500 border-r border-border/60">{m.common_action_edit()}</TableHead>
                    <TableHead class="w-36 text-center text-foreground border-r border-border/60">{m.finance_ledger_balance()}</TableHead>
                    <TableHead class="w-20 text-center text-foreground border-r border-border/60">{m.common_word_status()}</TableHead>
                    <TableHead class="w-28 text-center text-foreground border-r border-border/60">{m.common_overview_date()}</TableHead>
                    <TableHead class="w-32 text-center text-foreground border-r border-border/60">{m.finance_ledger_voucher()}</TableHead>
                    <TableHead class="w-24 text-center text-foreground border-r border-border/60">{m.common_onboarding_schedule_col_num()}</TableHead>
                    <TableHead class="text-start text-foreground">{m.common_calendar_field_description()}</TableHead>
                  </TableRow>
                </TableHeader>
                <TableBody>
                  {#if ledgerTransactions.length === 0}
                    <TableRow>
                      <TableCell colspan={8} class="text-center py-10 text-muted-foreground text-xs font-semibold">{m.finance_ledger_no_entries_found()}</TableCell>
                    </TableRow>
                  {:else}
                    {#each ledgerTransactions as row}
                      <TableRow class="hover:bg-accent/10 border-b border-border/40 text-[10px] leading-normal font-medium">
                        <!-- Debit -->
                        <TableCell class="border-r border-border/40 text-center font-mono text-red-500/90 font-semibold">
                          {row.debit > 0 ? formatCurrency(row.debit, "").replace(/YER|USD|SAR|AED/g, '') : "0"}
                        </TableCell>
                        <!-- Credit -->
                        <TableCell class="border-r border-border/60 text-center font-mono text-emerald-500/90 font-semibold">
                          {row.credit > 0 ? formatCurrency(row.credit, "").replace(/YER|USD|SAR|AED/g, '') : "0"}
                        </TableCell>
                        <!-- Balance -->
                        <TableCell class="border-r border-border/60 text-center font-mono font-bold text-foreground">
                          {formatCurrency(row.balance, "").replace(/YER|USD|SAR|AED/g, '')} <span class="text-[9px] text-muted-foreground/80 font-sans">{row.balance_label}</span>
                        </TableCell>
                        <!-- Status -->
                        <TableCell class="border-r border-border/60 text-center text-emerald-500 text-[9px] font-sans">✓</TableCell>
                        <!-- Date -->
                        <TableCell class="border-r border-border/60 text-center font-mono text-muted-foreground text-[9px]">{row.date}</TableCell>
                        <!-- Document -->
                        <TableCell class="border-r border-border/60 text-center text-foreground font-semibold text-[9px]">{row.document}</TableCell>
                        <!-- Number -->
                        <TableCell class="border-r border-border/60 text-center font-mono text-muted-foreground text-[9px]">{row.number}</TableCell>
                        <!-- Description -->
                        <TableCell class="text-start text-foreground select-text font-medium text-[9px] line-clamp-1 py-1 truncate">{row.description}</TableCell>
                      </TableRow>
                    {/each}
                  {/if}
                </TableBody>
                <tfoot>
                  <TableRow class="bg-card hover:bg-transparent font-bold font-mono text-[10px] border-t-2 border-border/90">
                    <!-- Total Debit -->
                    <TableCell class="border-r border-border/40 text-center text-red-500 font-bold">
                      {detailedTotals.debit > 0 ? formatCurrency(detailedTotals.debit, "").replace(/YER|USD|SAR|AED/g, '') : "0"}
                    </TableCell>
                    <!-- Total Credit -->
                    <TableCell class="border-r border-border/60 text-center text-emerald-500 font-bold">
                      {detailedTotals.credit > 0 ? formatCurrency(detailedTotals.credit, "").replace(/YER|USD|SAR|AED/g, '') : "0"}
                    </TableCell>
                    <!-- Net Balance -->
                    <TableCell class="border-r border-border/60 text-center text-foreground font-bold">
                      {formatCurrency(detailedTotals.balance, "").replace(/YER|USD|SAR|AED/g, '')} <span class="text-[9px] text-muted-foreground font-sans">{detailedTotals.balance_label}</span>
                    </TableCell>
                    <TableCell class="border-r border-border/60 text-center text-muted-foreground/40">-</TableCell>
                    <TableCell class="border-r border-border/60 text-center text-muted-foreground/40">-</TableCell>
                    <TableCell class="border-r border-border/60 text-center text-foreground uppercase tracking-wider text-[9px]">{m.finance_ledger_grand_total()}</TableCell>
                    <TableCell class="border-r border-border/60 text-center text-muted-foreground/40">-</TableCell>
                    <TableCell class="text-start font-sans text-[9px] px-3 font-medium text-primary">
                      {m.finance_ledger_net_detailed_account_ledger_balance()}
                    </TableCell>
                  </TableRow>
                </tfoot>
              </Table>
            {/if}
          </div>
        {/if}
      {/if}
    </div>
  <Dialog bind:open={isAddDialogOpen}>
    <DialogContent class="max-w-xl text-start select-none h-[80vh] flex flex-col p-6 bg-card border-border/80">
      <DialogHeader class="shrink-0 mb-4 text-start">
        <DialogTitle class="text-base font-extrabold text-foreground">
          {m.common_ledger_dialog_add_title()}
        </DialogTitle>
        <DialogDescription class="text-[11px] text-muted-foreground leading-relaxed">
          {m.common_ledger_dialog_add_desc()}
        </DialogDescription>
      </DialogHeader>
      {#if isAddDialogOpen}
        {#await import("$lib/features/finance/components/ledger-form.svelte") then { default: LedgerForm }}
          <LedgerForm
            orgId={data.orgId}
            onSuccess={handleSuccess}
            onCancel={() => isAddDialogOpen = false}
          />
        {/await}
      {/if}
    </DialogContent>
  </Dialog>

</div>
</div>

