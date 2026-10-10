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
import { LoadingIndicator } from "@orbis/ui";
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import { api } from "$lib/api";
import { formatCurrency, formatDate, isAr } from "$lib/core/i18n";
import type { AccountDto, CurrencyDto, JournalEntryDto, PaymentDto } from "$lib/api/types";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Card } from "@orbis/ui/card";
import { toast } from "@orbis/ui";
import { Refresh, Download } from "@orbis/icons";
import { OrbisGrid, type OrbisColumnDef } from "@orbis/svelte-grid";
import AccountSearchInput from "$lib/features/finance/components/account-search-input.svelte";
import CurrencySelector from "$lib/shared/components/currency-selector.svelte";
import { fade } from "svelte/transition";

let { orgId }: { orgId: string } = $props();

// ── State ──────────────────────────────────────────────────────────────
let accounts = $state<AccountDto[]>([]);
let currencies = $state<CurrencyDto[]>([]);
let journalEntries = $state<JournalEntryDto[]>([]);
let payments = $state<PaymentDto[]>([]);
let isLoading = $state(true);

// Filters
let selectedAccountId = $state("");
let currencyPreference = $state("");
let startDate = $state(getLocalDateInputValue());
let endDate = $state(getLocalDateInputValue());

function getLocalDateInputValue(date = new Date()) {
  return new Date(date.getTime() - date.getTimezoneOffset() * 60_000).toISOString().slice(0, 10);
}

type StatementMovement = {
  id: string;
  date: string;
  date_raw: string;
  ref: string;
  desc: string;
  debit: number;
  credit: number;
  currency: string;
};

type DisplayStatementMovement = StatementMovement & {
  __idx: number;
  date_display: string;
  debit_formatted: string;
  credit_formatted: string;
  balance_formatted: string;
  running_balance: number;
};

onMount(async () => {
  if (orgId) {
    await loadData();
  }
});

async function loadData() {
  isLoading = true;
  try {
    const [accountsList, currenciesList, entriesList, paymentsList] = await Promise.all([
      api.listAccounts(),
      api.listCurrencies(),
      api.listJournalEntries(),
      api.listPayments(),
    ]);

    accounts = accountsList || [];
    currencies = currenciesList || [];
    journalEntries = entriesList || [];
    payments = paymentsList || [];

    if (!selectedAccountId && accounts.length > 0) {
      const parentAccountIds = new Set(
        accounts.flatMap((account) => account.parent_id ? [account.parent_id] : [])
      );
      const postingAccount = accounts.find(
        (account) => account.is_active && !parentAccountIds.has(account.id)
      );
      selectedAccountId = postingAccount?.id || "";
    }
  } catch (e) {
    console.error("Failed to load account statement data:", e);
    toast.error(m.finance_ui_account_statement_load_failed());
  } finally {
    isLoading = false;
  }
}

const selectedAccount = $derived(
  accounts.find((a) => a.id === selectedAccountId)
);

// ── Ledger Rows & Running Balance Computation ──────────────────────────
const rawStatementRows = $derived.by((): StatementMovement[] => {
  if (!selectedAccountId) return [];

  const rows: StatementMovement[] = [];
  const targetAcc = selectedAccount;
  const isCashAccount = targetAcc?.sub_type === "cash"
    || targetAcc?.sub_type === "bank"
    || targetAcc?.account_type === "asset";
  const journalEntryIds = new Set(journalEntries.map((entry) => entry.id));

  for (const entry of journalEntries) {
    const entryDate = entry.accounting_date ? String(entry.accounting_date).slice(0, 10) : "";
    const curr = entry.currency_code || "YER";
    const ref = entry.reference_number || entry.id.slice(0, 8);
    const desc = isAr()
      ? entry.description_ar || entry.description || m.finance_ui_journal_entry_fallback()
      : entry.description || entry.description_ar || m.finance_ui_journal_entry_fallback();

    if (entry.lines?.length) {
      for (const line of entry.lines) {
        if (line.account_id === selectedAccountId) {
          const deb = Number(line.debit_amount || 0);
          const crd = Number(line.credit_amount || 0);
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
    } else if (entry.debit_account_id === selectedAccountId) {
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
  }

  if (isCashAccount) {
    for (const p of payments) {
      if (
        String(p.account_id) === selectedAccountId
        && !(p.journal_entry_id && journalEntryIds.has(p.journal_entry_id))
      ) {
        const paymentDate = p.paid_at || p.created_at;
        const pDate = paymentDate ? String(paymentDate).slice(0, 10) : "";
        const curr = p.currency_code || "YER";

        rows.push({
          id: `pay-${p.id}`,
          date: pDate,
          date_raw: paymentDate,
          ref: p.receipt_number || p.id.slice(0, 8),
          desc: p.payer_name || m.finance_ui_receipt_student_description({ student: m.finance_ui_student() }),
          debit: Number(p.amount || 0),
          credit: 0,
          currency: curr,
        });
      }
    }
  }

  rows.sort((a, b) => new Date(a.date_raw).getTime() - new Date(b.date_raw).getTime());
  return rows;
});

const movementCurrencies = $derived(
  [...new Set(rawStatementRows.map((row) => row.currency))].sort()
);

const selectedCurrency = $derived.by(() => {
  if (movementCurrencies.includes(currencyPreference)) return currencyPreference;
  if (selectedAccount?.currency && movementCurrencies.includes(selectedAccount.currency)) {
    return selectedAccount.currency;
  }
  return movementCurrencies[0] || "";
});
const movementCurrencyOptions = $derived(
  movementCurrencies.map((code) => currencies.find((currency) => currency.code === code) ?? {
    code,
    name: code,
    symbol: code,
  })
);

function handleAccountSelect() {
  currencyPreference = "";
}

const statementSummary = $derived.by(() => {
  if (!selectedCurrency) {
    return { rows: [], totalDebit: 0, totalCredit: 0, closingBalance: 0 };
  }

  const rows = rawStatementRows.filter((row) => row.currency === selectedCurrency);
  let running = 0;
  let totalDebit = 0;
  let totalCredit = 0;
  const visibleRows: DisplayStatementMovement[] = [];

  for (const row of rows) {
    if (endDate && row.date > endDate) continue;
    running += row.debit - row.credit;
    if (startDate && row.date < startDate) continue;
    totalDebit += row.debit;
    totalCredit += row.credit;
    visibleRows.push({
      ...row,
      __idx: visibleRows.length + 1,
      date_display: row.date ? formatDate(row.date) : "—",
      debit_formatted: row.debit > 0 ? formatCurrency(row.debit, row.currency) : "—",
      credit_formatted: row.credit > 0 ? formatCurrency(row.credit, row.currency) : "—",
      balance_formatted: formatCurrency(running, row.currency),
      running_balance: running,
    });
  }

  return {
    rows: visibleRows,
    totalDebit,
    totalCredit,
    closingBalance: running,
  };
});

const statementRows = $derived(statementSummary.rows);
const totalDebit = $derived(statementSummary.totalDebit);
const totalCredit = $derived(statementSummary.totalCredit);
const netBalance = $derived(statementSummary.closingBalance);

const columns: OrbisColumnDef[] = [
  { id: "__idx", prop: "__idx", header: "#", name: "#", width: 48, size: 48, readonly: true, pin: "left", align: "center" },
  { id: "date_display", prop: "date_display", header: m.finance_ui_date(), name: m.finance_ui_date(), width: 110, size: 110, readonly: true, align: "center" },
  { id: "ref", prop: "ref", header: m.finance_ui_reference_voucher_number(), name: m.finance_ui_reference_voucher_number(), width: 130, size: 130, readonly: true, cellClass: "font-mono font-bold" },
  { id: "desc", prop: "desc", header: m.finance_ui_movement_description(), name: m.finance_ui_movement_description(), width: 280, size: 280, readonly: true },
  { id: "debit_formatted", prop: "debit_formatted", header: m.finance_ui_debit_positive(), name: m.finance_ui_debit_positive(), width: 140, size: 140, readonly: true, align: "end", cellClass: "font-mono font-bold text-emerald-600 dark:text-emerald-400" },
  { id: "credit_formatted", prop: "credit_formatted", header: m.finance_ui_credit_negative(), name: m.finance_ui_credit_negative(), width: 140, size: 140, readonly: true, align: "end", cellClass: "font-mono font-bold text-rose-600 dark:text-rose-400" },
  { id: "balance_formatted", prop: "balance_formatted", header: m.finance_ui_running_balance(), name: m.finance_ui_running_balance(), width: 150, size: 150, readonly: true, align: "end", cellClass: "font-mono font-extrabold text-foreground" },
  { id: "currency", prop: "currency", header: m.finance_ui_currency(), name: m.finance_ui_currency(), width: 80, size: 80, readonly: true, align: "center" },
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
    [m.finance_ui_date(), m.finance_ui_reference(), m.finance_ui_description(), m.finance_ui_debit(), m.finance_ui_credit(), m.finance_ui_balance(), m.finance_ui_currency()],
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
          onselect={handleAccountSelect}
          placeholder={m.finance_ui_account_statement_account_placeholder()}
        />
      </div>

      <div class="flex items-center gap-2">
        <Input
          type="date"
          bind:value={startDate}
          class="h-8 w-36 text-xs bg-card border-border/60 font-mono"
        />
        <span class="text-xs text-muted-foreground">{m.finance_ui_to_date()}</span>
        <Input
          type="date"
          bind:value={endDate}
          class="h-8 w-36 text-xs bg-card border-border/60 font-mono"
        />

        {#if movementCurrencies.length > 0}
          <CurrencySelector
            title={m.finance_ui_currency()}
            description=""
            currentCurrency={selectedCurrency}
            currencyOptions={movementCurrencyOptions}
            compact
            onchange={(currency) => (currencyPreference = currency)}
          />
        {/if}

        <Button variant="ghost" size="icon-sm" onclick={loadData} class="text-muted-foreground">
          {#if isLoading}<LoadingIndicator  size={14} />{:else}<Refresh size={14}   />{/if}
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
          <span>{m.finance_ui_total_debit_movements()}</span>
          <span class="h-2 w-2 rounded-full bg-emerald-500"></span>
        </div>
        <div class="mt-2 text-xl font-mono font-bold text-emerald-600 dark:text-emerald-400">
          {formatCurrency(totalDebit, selectedCurrency || "YER")}
        </div>
      </Card>

      <Card class="p-4 shadow-sm border-border/40 bg-card flex flex-col justify-between">
        <div class="flex items-center justify-between text-muted-foreground text-xs font-semibold">
          <span>{m.finance_ui_total_credit_movements()}</span>
          <span class="h-2 w-2 rounded-full bg-rose-500"></span>
        </div>
        <div class="mt-2 text-xl font-mono font-bold text-rose-600 dark:text-rose-400">
          {formatCurrency(totalCredit, selectedCurrency || "YER")}
        </div>
      </Card>

      <Card class="p-4 shadow-sm border-border/40 bg-card flex flex-col justify-between">
        <div class="flex items-center justify-between text-muted-foreground text-xs font-semibold">
          <span>{m.finance_ui_final_net_balance()}</span>
          <span class="h-2 w-2 rounded-full bg-primary"></span>
        </div>
        <div class="mt-2 text-xl font-mono font-bold text-foreground">
          {formatCurrency(netBalance, selectedCurrency || "YER")}
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
        rtl={isAr()}
        readonly
        emptyTitle={m.finance_ui_no_account_movements()}
      />
    </div>
  </div>
</div>
