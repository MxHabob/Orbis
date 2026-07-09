<!--
@component FinanceLedgerPage
Account Statement / Trial Balance (كشف حساب تفصيلي)
Groups movements by currency (YER, SAR, USD, AED) and lists account codes and names.
Provides All, Single, and Custom selection mode with interactive Accounts Tree sidebar.
-->
<script lang="ts">
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
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogDescription,
} from "@orbis/ui-svelte/dialog";
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
} from "lucide-svelte";
import { getLocale } from "$lib/paraglide/runtime";
import { formatCurrency, formatDate } from "$lib/core/i18n";
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
let movementFilter = $state<string>("all"); // 'all' or 'active' (with movements)
let debtorCreditorFilter = $state<string>("all"); // 'all', 'debtor', 'creditor'


// Tree state
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

// Synced inputs for dual-combobox
let accountCodeQuery = $state("");
let accountNameQuery = $state("");
let isCodeDropdownOpen = $state(false);
let isNameDropdownOpen = $state(false);

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
      selectedAccountId = ledgerList[0].id as string;
      accountCodeQuery = String(ledgerList[0].code);
      accountNameQuery = ledgerList[0].name_ar as string || ledgerList[0].name as string;
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
  }
});

// Double entry ledger balance calculation
const calculatedLedgerRows = $derived.by(() => {
  return ledgers.map((acc) => ({
    id: acc.id,
    code: acc.code,
    name: acc.name,
    name_ar: acc.name_ar || acc.name,
    item_type: acc.item_type || "asset",
    parent_id: acc.parent_id,
    debit_yer: acc.debit_yer || 0.0,
    credit_yer: acc.credit_yer || 0.0,
    debit_usd: acc.debit_usd || 0.0,
    credit_usd: acc.credit_usd || 0.0,
    debit_sar: acc.debit_sar || 0.0,
    credit_sar: acc.credit_sar || 0.0,
    debit_aed: acc.debit_aed || 0.0,
    credit_aed: acc.credit_aed || 0.0,
    notes: acc.sub_type || "",
  }));
});

// Selected Account Row derived state
const selectedAccountRow = $derived.by(() => {
  return calculatedLedgerRows.find((r) => r.id === selectedAccountId);
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

  // YER Row
  if (row.debit_yer > 0 || row.credit_yer > 0) {
    list.push({
      currency: "ريال يمني",
      debit: row.debit_yer,
      credit: row.credit_yer,
      rate: 1.0,
      debit_eq: row.debit_yer,
      credit_eq: row.credit_yer,
      notes: "رصيد الحساب بالريال اليمني",
    });
  }

  // SAR Row
  if (row.debit_sar > 0 || row.credit_sar > 0) {
    const rate = 139.8;
    list.push({
      currency: "سعودي",
      debit: row.debit_sar,
      credit: row.credit_sar,
      rate: rate,
      debit_eq: row.debit_sar * rate,
      credit_eq: row.credit_sar * rate,
      notes: `رصيد الحساب بالسعودي (سعر الصرف ${rate})`,
    });
  }

  // USD Row
  if (row.debit_usd > 0 || row.credit_usd > 0) {
    const rate = 530.2;
    list.push({
      currency: "دولار",
      debit: row.debit_usd,
      credit: row.credit_usd,
      rate: rate,
      debit_eq: row.debit_usd * rate,
      credit_eq: row.credit_usd * rate,
      notes: `رصيد الحساب بالدولار (سعر الصرف ${rate})`,
    });
  }

  // AED Row
  if (row.debit_aed > 0 || row.credit_aed > 0) {
    const rate = 144.1;
    list.push({
      currency: "درهم",
      debit: row.debit_aed,
      credit: row.credit_aed,
      rate: rate,
      debit_eq: row.debit_aed * rate,
      credit_eq: row.credit_aed * rate,
      notes: `رصيد الحساب بالدرهم (سعر الصرف ${rate})`,
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
      const treasury = treasuries.find((t) => t.ledger_account_id === acc.id);
      if (treasury && pay.treasury_id === treasury.id) {
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
  let runningBalance = 0;
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

// Tree selection helpers
function toggleCategorySelect(catId: string, accounts: any[]) {
  const categoryFlatAccounts = calculatedLedgerRows.filter(
    (r) => String(r.item_type).toLowerCase() === catId,
  );
  const isAllSelected = categoryFlatAccounts.every((a) =>
    selectedTreeAccountIds.has(a.id),
  );
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

  const areAllDescendantsSelected = descendants.every((id) =>
    selectedTreeAccountIds.has(id),
  );

  if (areAllDescendantsSelected) {
    descendants.forEach((id) => newSet.delete(id));
  } else {
    descendants.forEach((id) => newSet.add(id));
  }
  selectedTreeAccountIds = newSet;
}

function getAccountCheckboxState(
  node: any,
): "checked" | "partial" | "unchecked" {
  const descendants: string[] = [];
  function getDescendants(n: any) {
    descendants.push(n.id);
    if (n.children) {
      n.children.forEach(getDescendants);
    }
  }
  getDescendants(node);

  const selectedCount = descendants.filter((id) =>
    selectedTreeAccountIds.has(id),
  ).length;
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
    } else if (activeTab === "all") {
      if (!selectedTreeAccountIds.has(row.id)) return false;
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

// Sum totals for footer
const totals = $derived.by(() => {
  let debY = 0,
    credY = 0;
  let debS = 0,
    credS = 0;
  let debU = 0,
    credU = 0;
  let debA = 0,
    credA = 0;

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

// Derived rows for Table A2 (All Accounts, Specific Currency)
const tableA2Rows = $derived.by(() => {
  if (activeTab !== "all" || selectedCurrency === "all") return [];
  const curr = selectedCurrency.toUpperCase();
  return filteredLedgerRows.map((row) => {
    let deb = 0,
      cred = 0;
    if (curr === "YER") {
      deb = row.debit_yer;
      cred = row.credit_yer;
    } else if (curr === "SAR") {
      deb = row.debit_sar;
      cred = row.credit_sar;
    } else if (curr === "USD") {
      deb = row.debit_usd;
      cred = row.credit_usd;
    } else if (curr === "AED") {
      deb = row.debit_aed;
      cred = row.credit_aed;
    }

    const isDebitNormal =
      row.item_type === "asset" || row.item_type === "expense";
    let bal = isDebitNormal ? deb - cred : cred - deb;
    return {
      ...row,
      debit: deb,
      credit: cred,
      balance: Math.abs(bal),
      balance_label: bal >= 0 ? "له" : "عليه",
    };
  });
});

const tableA2Totals = $derived.by(() => {
  let deb = 0,
    cred = 0;
  for (const r of tableA2Rows) {
    deb += r.debit;
    cred += r.credit;
  }
  const diff = deb - cred;
  return {
    debit: deb,
    credit: cred,
    balance: Math.abs(diff),
    balance_label: diff >= 0 ? "له" : "عليه",
  };
});

function handlePrint() {
  window.print();
}

function exitPage() {
  goto(`/${data.orgId}/finance`);
}
</script>

<svelte:window onclick={handleWindowClick} />



  <!-- Main Filter Controls Box -->
  <div class="p-5 bg-card/15 border-b border-border/80 grid grid-cols-1 lg:grid-cols-4 gap-6 shrink-0 no-print select-none text-start">
    
    <!-- Right Side (Renders on right in RTL): Accounts Tree Selection Box (spans 1 column) -->
    <div 
      class="bg-background border border-border rounded-lg p-3 flex flex-col overflow-hidden shadow-sm lg:col-span-1 self-start font-sans"
      style="height: 220px; max-height: 220px;"
    >
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
                <ChevronRight size={12} class="text-muted-foreground {getLocale() === "ar" ? "rotate-180" : ""}" />
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
                        <ChevronRight size={12} class="text-muted-foreground {getLocale() === "ar" ? "rotate-180" : ""}" />
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
    <div class="lg:col-span-3 flex flex-col gap-4 justify-between bg-card/5 border border-border/50 rounded-xl p-4 shadow-inner">
      
      <!-- Mode Tabs and Top Status Indicator -->
      <div class="flex items-center justify-between border-b border-border/60 pb-3">
        <!-- Mode Tabs -->
        <div class="flex border border-border bg-muted/50 rounded-lg p-0.5 text-[10px] font-bold">
          <button
            onclick={() => activeTab = "single"}
            class="px-5 py-1.5 rounded-md transition-all duration-150 {activeTab === 'single' ? 'bg-primary text-primary-foreground shadow-sm font-bold' : 'text-muted-foreground hover:text-foreground font-semibold'}"
          >
            {m.ledger_tab_one()}
          </button>
          <button
            onclick={() => activeTab = "all"}
            class="px-5 py-1.5 rounded-md transition-all duration-150 {activeTab === 'all' ? 'bg-primary text-primary-foreground shadow-sm font-bold' : 'text-muted-foreground hover:text-foreground font-semibold'}"
          >
            {m.ledger_tab_all()}
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
            <span>اسم الحساب</span>
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
                    <span class="text-[10px] text-muted-foreground p-2 text-center">لا توجد حسابات مطابقة</span>
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
          <div class="flex flex-col gap-1 text-[9px] font-bold text-muted-foreground combobox-container relative w-1/4 max-w-[160px] min-w-[100px]">
            <span>كود الحساب</span>
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
                    <span class="text-[10px] text-muted-foreground p-2 text-center">لا توجد حسابات</span>
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
          <div class="flex flex-col gap-1 text-[9px] font-bold text-muted-foreground">
            <span>{m.ledger_currency()}</span>
            <select
              bind:value={selectedCurrency}
              class="bg-background text-foreground border border-border rounded-md px-2.5 py-1 text-[11px] outline-none focus:ring-1 focus:ring-ring font-sans w-full"
            >
              <option value="all">&lt;&lt;كافة العملات&gt;&gt;</option>
              <option value="YER">YER (ريال يمني)</option>
              <option value="SAR">SAR (سعودي)</option>
              <option value="USD">USD (دولار)</option>
              <option value="AED">AED (درهم)</option>
            </select>
          </div>

          <!-- Date picker -->
          <div class="flex flex-col gap-1 text-[9px] font-bold text-muted-foreground">
            <span>{selectedCurrency === "all" ? "حتى يوم" : "خلال يوم"}</span>
            <Input
              type="date"
              bind:value={customStartDate}
              class="input-sunken py-1 px-2.5 text-[11px] w-full"
            />
          </div>
          
          <!-- Period Selector -->
          <div class="flex flex-col gap-1 text-[9px] font-bold text-muted-foreground">
            <span>{m.metrics_period()}</span>
            <select
              bind:value={periodFilter}
              class="bg-background text-foreground border border-border rounded-md px-2.5 py-1 text-[11px] outline-none focus:ring-1 focus:ring-ring font-sans w-full"
            >
              <option value="all">{m.ledger_period_all()}</option>
              <option value="day">{m.ledger_period()}</option>
              <option value="week">{m.ledger_period_week()}</option>
              <option value="month">{m.ledger_period_month()}</option>
              <option value="year">{m.ledger_period_year()}</option>
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
      class="text-xs h-9 px-5 border border-border flex items-center gap-2 bg-gradient-to-b hover:from-muted/20 hover:to-muted/30 font-bold text-red-500 hover:text-red-600 rounded-md shadow-sm"
      onclick={exitPage}
    >
      <LogOut size={13} class={getLocale() === "ar" ? 'rotate-180' : ''} />
      <span>{m.ledger_action_exit()}</span>
    </Button>

    <!-- Toolbar controls and Print actions -->
    <div class="flex items-center gap-3">
      <!-- Add Account / Treasury -->
      <Button
        variant="ghost"
        class="text-xs h-9 px-4 border border-border flex items-center gap-2 bg-gradient-to-b hover:from-muted/20 hover:to-muted/30 font-bold rounded-md shadow-sm text-primary"
        onclick={() => isAddDialogOpen = true}
      >
        <FolderOpen size={13} />
        <span>{m.ledger_btn_add_account()}</span>
      </Button>

      <!-- 1. Print Selected -->
      <Button
        variant="ghost"
        class="text-xs h-9 px-4 border border-border flex items-center gap-2 bg-gradient-to-b hover:from-muted/20 hover:to-muted/30 font-bold rounded-md shadow-sm"
        onclick={handlePrint}
      >
        <Printer size={13} />
        <span>{m.ledger_action_print_selected()}</span>
      </Button>

      <!-- 2. Print -->
      <Button
        variant="ghost"
        class="text-xs h-9 px-4 border border-border flex items-center gap-2 bg-gradient-to-b hover:from-muted/20 hover:to-muted/30 font-bold rounded-md shadow-sm"
        onclick={handlePrint}
      >
        <Printer size={13} />
        <span>{m.ledger_action_print()}</span>
      </Button>

      <!-- 3. View / Show (Main Green Action) -->
      <Button
        variant="lime"
        class="text-xs h-9 px-6 flex items-center gap-2 font-black bg-gradient-to-b from-emerald-500 to-emerald-600 hover:from-emerald-600 hover:to-emerald-700 text-white rounded-md shadow-md shadow-emerald-500/10 min-w-28 justify-center"
        onclick={loadData}
        disabled={isLoading}
      >
        <RotateCw size={13} class={isLoading ? 'animate-spin' : ''} />
        <span>{m.ledger_action_view()}</span>
      </Button>
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
        {#if activeTab === "all"}
          {#if selectedCurrency === "all"}
            <!-- Excel Grid Sheet Ledger -->
            <div class="border border-border rounded-xl bg-card/30 overflow-hidden min-w-[900px] print-table" in:fade={{ duration: 150 }}>
              <Table class="border-collapse w-full">
                <TableHeader class="bg-card">
                  <!-- Top Currency Header Row -->
                  <TableRow class="hover:bg-transparent border-b border-border/80">
                    <TableHead rowspan={2} class="w-10 border-r border-border/60 text-center font-bold text-foreground">*</TableHead>
                    <TableHead colspan={2} class="border-r border-border/60 text-center font-bold text-foreground bg-primary/5">ريال يمني</TableHead>
                    <TableHead colspan={2} class="border-r border-border/60 text-center font-bold text-foreground bg-emerald-500/5">سعودي</TableHead>
                    <TableHead colspan={2} class="border-r border-border/60 text-center font-bold text-foreground bg-blue-500/5">دولار</TableHead>
                    <TableHead colspan={2} class="border-r border-border/60 text-center font-bold text-foreground bg-purple-500/5">درهم</TableHead>
                    <TableHead rowspan={2} class="border-r border-border/60 text-start font-bold text-foreground">{m.ledger_col_name()}</TableHead>
                    <TableHead rowspan={2} class="border-r border-border/60 text-center font-bold text-foreground w-28">{m.ledger_col_code()}</TableHead>
                    <TableHead rowspan={2} class="text-start font-bold text-foreground w-64">{m.ledger_col_notes()}</TableHead>
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
    
                    <TableCell class="border-r border-border/40 text-start text-foreground uppercase tracking-wider text-[9px]">{getLocale() === "ar" ? "الإجـمــالي" : "Grand Total"}</TableCell>
                    <TableCell class="border-r border-border/40 text-center text-foreground font-bold">-</TableCell>
                    <TableCell class="text-start text-muted-foreground">-</TableCell>
                  </TableRow>
                </tfoot>
              </Table>
            </div>
          {:else}
            <!-- Table A2: Single-currency Ledger Grid for All Accounts -->
            <div class="border border-border rounded-xl bg-card/30 overflow-hidden min-w-[900px] print-table" in:fade={{ duration: 150 }}>
              <Table class="border-collapse w-full">
                <TableHeader class="bg-card">
                  <TableRow class="hover:bg-transparent border-b border-border/80 text-[10px] font-bold text-foreground">
                    <TableHead class="w-10 border-r border-border/60 text-center font-bold">*</TableHead>
                    <TableHead class="text-center border-r border-border/60 w-28 text-red-500">{m.ledger_col_debit()}</TableHead>
                    <TableHead class="text-center border-r border-border/60 w-28 text-emerald-500">{m.ledger_col_credit()}</TableHead>
                    <TableHead class="text-center border-r border-border/60 w-32 font-bold">{getLocale() === "ar" ? "الرصيد" : "Balance"}</TableHead>
                    <TableHead class="text-start border-r border-border/60 font-bold">{m.ledger_col_name()}</TableHead>
                    <TableHead class="text-center border-r border-border/60 w-28 font-bold">{m.ledger_col_code()}</TableHead>
                    <TableHead class="text-start font-bold w-64">{m.ledger_col_notes()}</TableHead>
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
                    <TableCell class="border-r border-border/40 text-start text-foreground uppercase tracking-wider text-[9px]">{getLocale() === "ar" ? "الإجـمــالي" : "Grand Total"}</TableCell>
                    <TableCell class="border-r border-border/40 text-center text-foreground font-bold">-</TableCell>
                    <TableCell class="text-start text-muted-foreground">-</TableCell>
                  </TableRow>
                </tfoot>
              </Table>
            </div>
          {/if}
        {:else}
          <!-- Single Account View Swapped Layout -->
          <div class="border border-border rounded-xl bg-card/30 overflow-hidden min-w-[900px] print-table" in:fade={{ duration: 150 }}>
            {#if selectedCurrency === "all"}
              <!-- Table B: Currency Summary Grid -->
              <Table class="border-collapse w-full">
                <TableHeader class="bg-card">
                  <TableRow class="hover:bg-transparent border-b border-border/80 text-[10px] font-bold text-foreground">
                    <TableHead class="w-12 text-center font-bold border-r border-border/60">*</TableHead>
                    <TableHead colspan={2} class="text-center font-bold border-r border-border/60">{getLocale() === "ar" ? "المبلـغ" : "Amount"}</TableHead>
                    <TableHead class="text-center font-bold border-r border-border/60 w-32">{getLocale() === "ar" ? "الالعملة" : "Currency"}</TableHead>
                    <TableHead colspan={2} class="text-center font-bold border-r border-border/60">{getLocale() === "ar" ? "المقابل بالريال اليمني" : "Equivalent in YER"}</TableHead>
                    <TableHead class="text-start font-bold">{getLocale() === "ar" ? "البيان" : "Description"}</TableHead>
                  </TableRow>
                  <TableRow class="hover:bg-transparent border-b border-border/80 text-[9px] font-bold">
                    <TableHead class="border-r border-border/60 text-center">-</TableHead>
                    <TableHead class="text-center border-r border-border/45 text-red-500 w-24">{m.ledger_col_debit()}</TableHead>
                    <TableHead class="text-center border-r border-border/60 text-emerald-500 w-24">{m.ledger_col_credit()}</TableHead>
                    <TableHead class="border-r border-border/60 text-center">-</TableHead>
                    <TableHead class="text-center border-r border-border/45 text-red-500 w-28">{m.ledger_col_debit()}</TableHead>
                    <TableHead class="text-center border-r border-border/60 text-emerald-500 w-28">{m.ledger_col_credit()}</TableHead>
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
                    <TableCell class="border-r border-border/60 text-center text-foreground font-bold">{getLocale() === "ar" ? "الإجـمــالي" : "Total"}</TableCell>
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
                    <TableCell class="border-r border-border/60 text-center text-foreground font-bold">{getLocale() === "ar" ? "الرصيـد" : "Balance"}</TableCell>
                    
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
                    <TableHead class="w-28 text-center text-red-500 border-r border-border/60">{m.ledger_col_debit()}</TableHead>
                    <TableHead class="w-28 text-center text-emerald-500 border-r border-border/60">{m.ledger_col_credit()}</TableHead>
                    <TableHead class="w-36 text-center text-foreground border-r border-border/60">{getLocale() === "ar" ? "الرصيد" : "Balance"}</TableHead>
                    <TableHead class="w-20 text-center text-foreground border-r border-border/60">{getLocale() === "ar" ? "حالته" : "Status"}</TableHead>
                    <TableHead class="w-28 text-center text-foreground border-r border-border/60">{getLocale() === "ar" ? "التاريخ" : "Date"}</TableHead>
                    <TableHead class="w-32 text-center text-foreground border-r border-border/60">{getLocale() === "ar" ? "المستند" : "Voucher"}</TableHead>
                    <TableHead class="w-24 text-center text-foreground border-r border-border/60">{getLocale() === "ar" ? "رقمه" : "No."}</TableHead>
                    <TableHead class="text-start text-foreground">{getLocale() === "ar" ? "البيان" : "Description"}</TableHead>
                  </TableRow>
                </TableHeader>
                <TableBody>
                  {#if ledgerTransactions.length === 0}
                    <TableRow>
                      <TableCell colspan={8} class="text-center py-10 text-muted-foreground text-xs font-semibold">لا توجد قيود أو حركات مطابقة لهذا الحساب بالفترة المحددة</TableCell>
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
                    <TableCell class="border-r border-border/60 text-center text-foreground uppercase tracking-wider text-[9px]">{getLocale() === "ar" ? "الإجـمــالي" : "Grand Total"}</TableCell>
                    <TableCell class="border-r border-border/60 text-center text-muted-foreground/40">-</TableCell>
                    <TableCell class="text-start text-muted-foreground font-sans text-[9px] px-3 font-medium text-primary">
                      {getLocale() === "ar" ? "صافي رصيد كشف الحساب التفصيلي الموضح" : "Net detailed account ledger balance summary"}
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
          {m.ledger_dialog_add_title()}
        </DialogTitle>
        <DialogDescription class="text-[11px] text-muted-foreground leading-relaxed">
          {m.ledger_dialog_add_desc()}
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
