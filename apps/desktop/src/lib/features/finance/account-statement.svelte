<!--
  @component AccountStatement
  Minimal, elegant, and high-performance General Ledger Account Statement (كشف الحساب) workspace.
  Adheres strictly to Orbis Clean Architecture and the FinanceOverview design system:
  - Account selection with instant running balance calculation
  - Top KPI summary cards for debit, credit, and net balance
  - Sleek top toolbar with date range, currency, and export
  - High-performance OrbisGrid table for ledger movements
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import { api } from "$lib/api";
import { formatCurrency, formatDate, isAr } from "$lib/core/i18n";
import type { AccountDto, CurrencyDto, JournalEntryDto } from "$lib/api/types";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
import { Card, CardHeader, CardTitle, CardContent } from "@orbis/ui/card";
import { toast } from "@orbis/ui";
import {
  Refresh,
  Search,
  Download,
  BuildingLibrary,
} from "@orbis/icons";
import { OrbisGrid, type OrbisColumnDef } from "@orbis/svelte-grid";
import AccountSearchInput from "$lib/features/finance/components/account-search-input.svelte";
import SearchableSelect from "$lib/shared/components/searchable-select.svelte";
import { fade } from "svelte/transition";

let { orgId }: { orgId: string } = $props();

// ── State ──────────────────────────────────────────────────────────────
let accounts = $state<AccountDto[]>([]);
let currencies = $state<CurrencyDto[]>([]);
let journalEntries = $state<JournalEntryDto[]>([]);
let payments = $state<any[]>([]);
let isLoading = $state(true);

// Filters
let selectedAccountId = $state("");
let selectedCurrency = $state("all");
let startDate = $state("");
let endDate = $state("");
let searchQuery = $state("");

onMount(async () => {
  if (orgId) {
    await loadData();
  }
});

async function loadData() {
  isLoading = true;
  try {
    const [accountsList, currenciesList, entriesList, paymentsList] = await Promise.all([
      api.listAccounts().catch(() => []),
      api.listCurrencies().catch(() => []),
      api.listJournalEntries().catch(() => []),
      api.listPayments(null, null).catch(() => []),
    ]);

    accounts = accountsList || [];
    currencies = currenciesList || [];
    journalEntries = entriesList || [];
    payments = paymentsList || [];

    if (!selectedAccountId && accounts.length > 0) {
      // Pick first child or cash account
      const firstChild = accounts.find((a) => !accounts.some((cand) => cand.parent_id === a.id));
      selectedAccountId = firstChild ? firstChild.id : accounts[0].id;
    }
  } catch (e) {
    console.error("Failed to load account statement data:", e);
    toast.error("فشل تحميل بيانات كشف الحساب");
  } finally {
    isLoading = false;
  }
}

const selectedAccount = $derived(
  accounts.find((a) => a.id === selectedAccountId)
);

// ── Ledger Rows & Running Balance Computation ──────────────────────────
const statementRows = $derived.by(() => {
  if (!selectedAccountId) return [];

  const rows: any[] = [];
  const targetAcc = selectedAccount;
  const isCashAccount = targetAcc?.sub_type === "cash" || targetAcc?.sub_type === "bank" || targetAcc?.type === "asset";

  // 1. Process Journal Entries
  for (const entry of journalEntries) {
    const entryDate = entry.accounting_date ? String(entry.accounting_date).slice(0, 10) : "";
    if (startDate && entryDate < startDate) continue;
    if (endDate && entryDate > endDate) continue;

    const curr = entry.currency_code || entry.currency || "YER";
    if (selectedCurrency !== "all" && curr !== selectedCurrency) continue;

    const ref = entry.reference_number || entry.id.slice(0, 8);
    const desc = entry.description_ar || entry.description || "قيد يومية";

    // Direct 2-legged entry
    if (entry.debit_account_id === selectedAccountId) {
      rows.push({
        id: `je-deb-${entry.id}`,
        date: entryDate,
        date_raw: entry.accounting_date || entry.created_at,
        ref,
        desc,
        debit: Number(entry.amount || 0),
        credit: 0,
        currency: curr,
      });
    } else if (entry.credit_account_id === selectedAccountId) {
      rows.push({
        id: `je-crd-${entry.id}`,
        date: entryDate,
        date_raw: entry.accounting_date || entry.created_at,
        ref,
        desc,
        debit: 0,
        credit: Number(entry.amount || 0),
        currency: curr,
      });
    }

    // Multi-line entries
    if (entry.lines && entry.lines.length > 0) {
      for (const line of entry.lines) {
        if (line.account_id === selectedAccountId) {
          const deb = Number(line.debit || 0);
          const crd = Number(line.credit || 0);
          if (deb > 0 || crd > 0) {
            rows.push({
              id: `je-line-${line.id || entry.id}`,
              date: entryDate,
              date_raw: entry.accounting_date || entry.created_at,
              ref,
              desc: line.description || desc,
              debit: deb,
              credit: crd,
              currency: curr,
            });
          }
        }
      }
    }
  }

  // 2. Process Payments if Cash Account
  if (isCashAccount) {
    for (const p of payments) {
      if (String(p.cash_account_id) === selectedAccountId) {
        const pDate = p.payment_date ? String(p.payment_date).slice(0, 10) : (p.created_at ? String(p.created_at).slice(0, 10) : "");
        if (startDate && pDate < startDate) continue;
        if (endDate && pDate > endDate) continue;

        const curr = p.currency || "YER";
        if (selectedCurrency !== "all" && curr !== selectedCurrency) continue;

        rows.push({
          id: `pay-${p.id}`,
          date: pDate,
          date_raw: p.payment_date || p.created_at,
          ref: p.receipt_number || p.id.slice(0, 8),
          desc: p.notes || `سند قبض - ${p.student_name || 'طالب'}`,
          debit: Number(p.amount || 0), // Cash in = Debit Asset
          credit: 0,
          currency: curr,
        });
      }
    }
  }

  // Sort chronologically ascending
  rows.sort((a, b) => new Date(a.date_raw).getTime() - new Date(b.date_raw).getTime());

  // Compute running balance
  let running = 0;
  return rows.map((r, idx) => {
    running += (r.debit - r.credit);
    return {
      ...r,
      __idx: idx + 1,
      date_display: r.date ? formatDate(r.date) : "—",
      debit_formatted: r.debit > 0 ? formatCurrency(r.debit, r.currency) : "—",
      credit_formatted: r.credit > 0 ? formatCurrency(r.credit, r.currency) : "—",
      balance_formatted: formatCurrency(running, r.currency),
      running_balance: running,
    };
  });
});

// ── Summary Totals ─────────────────────────────────────────────────────
const totalDebit = $derived(
  statementRows.reduce((sum, r) => sum + r.debit, 0)
);

const totalCredit = $derived(
  statementRows.reduce((sum, r) => sum + r.credit, 0)
);

const netBalance = $derived(
  totalDebit - totalCredit
);

const columns: OrbisColumnDef[] = [
  { id: "__idx", prop: "__idx", header: "#", name: "#", width: 48, size: 48, readonly: true, pin: "left", align: "center" },
  { id: "date_display", prop: "date_display", header: "التاريخ", name: "التاريخ", width: 110, size: 110, readonly: true, align: "center" },
  { id: "ref", prop: "ref", header: "رقم المرجع / السند", name: "رقم المرجع / السند", width: 130, size: 130, readonly: true, cellClass: "font-mono font-bold" },
  { id: "desc", prop: "desc", header: "البيان / تفاصيل الحركة", name: "البيان / تفاصيل الحركة", width: 280, size: 280, readonly: true },
  { id: "debit_formatted", prop: "debit_formatted", header: "مدين (+)", name: "مدين (+)", width: 140, size: 140, readonly: true, align: "end", cellClass: "font-mono font-bold text-emerald-600 dark:text-emerald-400" },
  { id: "credit_formatted", prop: "credit_formatted", header: "دائن (-)", name: "دائن (-)", width: 140, size: 140, readonly: true, align: "end", cellClass: "font-mono font-bold text-rose-600 dark:text-rose-400" },
  { id: "balance_formatted", prop: "balance_formatted", header: "الرصيد التراكمي", name: "الرصيد التراكمي", width: 150, size: 150, readonly: true, align: "end", cellClass: "font-mono font-extrabold text-foreground" },
  { id: "currency", prop: "currency", header: "العملة", name: "العملة", width: 80, size: 80, readonly: true, align: "center" },
];

function exportCsv() {
  const rows = statementRows.map((r) => [
    r.date_display,
    r.ref,
    r.desc,
    r.debit,
    r.credit,
    r.running_balance,
    r.currency,
  ]);
  const csv = [
    ["Date", "Reference", "Description", "Debit", "Credit", "Balance", "Currency"],
    ...rows,
  ]
    .map((row) => row.map((v) => `"${String(v ?? "").replaceAll('"', '""')}"`).join(","))
    .join("\n");
  const url = URL.createObjectURL(new Blob(["\ufeff" + csv], { type: "text/csv;charset=utf-8" }));
  const link = document.createElement("a");
  link.href = url;
  link.download = `statement-${selectedAccount?.code || orgId}.csv`;
  link.click();
  URL.revokeObjectURL(url);
}
</script>

<div class="flex h-full min-h-0 w-full flex-col overflow-hidden p-6 font-sans bg-background/50">
  <div class="flex min-h-0 w-full flex-1 flex-col gap-4" in:fade={{ duration: 150 }}>
    
    <!-- Top Filter Bar -->
    <header class="flex shrink-0 flex-wrap items-center justify-between gap-4">
      <div class="flex items-center gap-3 flex-1 min-w-[280px] max-w-md">
        <AccountSearchInput
          bind:value={selectedAccountId}
          {accounts}
          placeholder="اختر الحساب لعرض كشف الحساب..."
        />
      </div>

      <div class="flex items-center gap-2">
        <Input
          type="date"
          bind:value={startDate}
          class="h-8 w-36 text-xs bg-card border-border/60 font-mono"
        />
        <span class="text-xs text-muted-foreground">إلى</span>
        <Input
          type="date"
          bind:value={endDate}
          class="h-8 w-36 text-xs bg-card border-border/60 font-mono"
        />

        <Button variant="ghost" size="icon-sm" onclick={loadData} class="text-muted-foreground">
          <Refresh size={14} class={isLoading ? "animate-spin" : ""} />
        </Button>

        <Button variant="ghost" size="icon-sm" onclick={exportCsv} class="text-muted-foreground">
          <Download size={14} />
        </Button>
      </div>
    </header>

    <!-- Top Summary KPI Cards (Matching FinanceOverview Aesthetic) -->
    <div class="grid grid-cols-1 sm:grid-cols-3 gap-4 shrink-0">
      <Card class="p-4 shadow-sm border-border/40 bg-card flex flex-col justify-between">
        <div class="flex items-center justify-between text-muted-foreground text-xs font-semibold">
          <span>إجمالي الحركات المدينة</span>
          <span class="h-2 w-2 rounded-full bg-emerald-500"></span>
        </div>
        <div class="mt-2 text-xl font-mono font-bold text-emerald-600 dark:text-emerald-400">
          {formatCurrency(totalDebit, selectedAccount?.currency || "YER")}
        </div>
      </Card>

      <Card class="p-4 shadow-sm border-border/40 bg-card flex flex-col justify-between">
        <div class="flex items-center justify-between text-muted-foreground text-xs font-semibold">
          <span>إجمالي الحركات الدائنة</span>
          <span class="h-2 w-2 rounded-full bg-rose-500"></span>
        </div>
        <div class="mt-2 text-xl font-mono font-bold text-rose-600 dark:text-rose-400">
          {formatCurrency(totalCredit, selectedAccount?.currency || "YER")}
        </div>
      </Card>

      <Card class="p-4 shadow-sm border-border/40 bg-card flex flex-col justify-between">
        <div class="flex items-center justify-between text-muted-foreground text-xs font-semibold">
          <span>الرصيد الصافي النهائي</span>
          <span class="h-2 w-2 rounded-full bg-primary"></span>
        </div>
        <div class="mt-2 text-xl font-mono font-bold text-foreground">
          {formatCurrency(netBalance, selectedAccount?.currency || "YER")}
        </div>
      </Card>
    </div>

    <!-- Grid Container -->
    <div class="flex-1 min-h-0 overflow-hidden">
      <OrbisGrid
        columns={columns}
        source={statementRows}
        loading={isLoading}
        clientQuery
        search={searchQuery}
        rtl={isAr()}
        readonly
        emptyTitle="لا توجد حركات مالية مسجلة لهذا الحساب"
      />
    </div>
  </div>
</div>
