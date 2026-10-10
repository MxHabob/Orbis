<!--
  @component JournalEntries
  Minimal, elegant, and high-performance Journal Entries workspace.
  Adheres strictly to Orbis Clean Architecture and the FinanceOverview design system:
  - Clean card-based entry form with direct account selection
  - Sleek top toolbar with search, date filter, and export
  - High-performance OrbisGrid table for vouchers
  - Real backend double-entry posting and reversals
-->
<script lang="ts">
import { LoadingIndicator } from "@orbis/ui";
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import { api } from "$lib/api";
import { formatCurrency, formatDate, l, isAr } from "$lib/core/i18n";
import type {
  JournalEntryDto,
  AccountDto,
  CurrencyDto,
  CreateJournalEntryRequest,
} from "$lib/api/types";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
import { toast } from "@orbis/ui";
import {
  Plus,
  Refresh,
  Search,
  Download,
  XMark,
  Printer,
} from "@orbis/icons";
import { OrbisGrid, type OrbisColumnDef } from "@orbis/svelte-grid";
import AccountSearchInput from "$lib/features/finance/components/account-search-input.svelte";
import FinanceInlineFormCard from "$lib/features/finance/components/finance-inline-form-card.svelte";
import CurrencySelector from "$lib/shared/components/currency-selector.svelte";
import PdfReportViewer from "$lib/shared/components/pdf-report-viewer.svelte";
import { confirmStore } from "$lib/stores";
import { orgContext } from "$lib/stores/org-context.svelte";
import { fade, slide } from "svelte/transition";

let { orgId }: { orgId: string } = $props();

// ── State ──────────────────────────────────────────────────────────────
let journalEntries = $state<JournalEntryDto[]>([]);
let accounts = $state<AccountDto[]>([]);
let currencies = $state<CurrencyDto[]>([]);
let isLoading = $state(true);
let isSubmitting = $state(false);
let showAddForm = $state(false);

// Form Fields
let referenceNumber = $state("");
let accountingDate = $state(getLocalDateInputValue());
let amount = $state<number | null>(null);
let selectedCurrency = $state(orgContext.baseCurrency || "");
let debitAccountId = $state("");
let creditAccountId = $state("");
let notes = $state("");

// Filters
let searchQuery = $state("");
let filterDate = $state(getLocalDateInputValue());

function getLocalDateInputValue(date = new Date()) {
  return new Date(date.getTime() - date.getTimezoneOffset() * 60_000).toISOString().slice(0, 10);
}

const journalAccounts = $derived(
  accounts.filter((account) =>
    account.is_active &&
    account.currency.toUpperCase() === selectedCurrency.toUpperCase()
  ),
);

// Print
let printReportOpen = $state(false);
let printParameters = $state<Record<string, any>>({});
let printTitle = $state(m.finance_ui_print_journal_entry());

onMount(async () => {
  await loadData();
});

async function loadData() {
  isLoading = true;
  try {
    const [entriesList, accountsList, currenciesList] = await Promise.all([
      api.listJournalEntries(),
      api.listAccounts(),
      api.listCurrencies(),
    ]);

    journalEntries = entriesList;
    accounts = accountsList;
    currencies = currenciesList;

    if (!currencies.some((currency) => currency.code === selectedCurrency)) {
      selectedCurrency = orgContext.baseCurrency || currencies[0]?.code || "";
    }
  } catch (e) {
    console.error("Failed to load journal entries master data:", e);
    toast.error(m.finance_ui_journal_entries_load_failed());
  } finally {
    isLoading = false;
  }
}

function clearForm() {
  referenceNumber = "";
  accountingDate = getLocalDateInputValue();
  amount = null;
  debitAccountId = "";
  creditAccountId = "";
  notes = "";
}

function handleCurrencyChange(currency: string | undefined) {
  if (!currency || currency === selectedCurrency) return;
  selectedCurrency = currency;
  if (!journalAccounts.some((account) => account.id === debitAccountId)) debitAccountId = "";
  if (!journalAccounts.some((account) => account.id === creditAccountId)) creditAccountId = "";
}

function getAccountName(id?: string | null): string {
  if (!id) return "—";
  const acc = accounts.find((a) => a.id === id);
  if (!acc) return id.slice(0, 8);
  return acc.name_ar || acc.name || acc.code || "—";
}

async function handleSaveNew() {
  if (!amount || amount <= 0) {
    toast.error(m.finance_ui_journal_amount_invalid());
    return;
  }
  if (!debitAccountId) {
    toast.error(m.finance_ui_debit_account_required());
    return;
  }
  if (!creditAccountId) {
    toast.error(m.finance_ui_credit_account_required());
    return;
  }
  if (debitAccountId === creditAccountId) {
    toast.error(m.finance_ui_journal_accounts_must_differ());
    return;
  }
  if (!selectedCurrency) {
    toast.error(m.finance_ui_currency_select_placeholder());
    return;
  }
  if (
    !journalAccounts.some((account) => account.id === debitAccountId) ||
    !journalAccounts.some((account) => account.id === creditAccountId)
  ) {
    toast.error(m.finance_ui_journal_accounts_must_differ());
    return;
  }

  isSubmitting = true;
  try {
    const amountValue = String(amount);
    const description = notes.trim() || (referenceNumber
      ? m.finance_ui_journal_entry_numbered_description({ number: referenceNumber })
      : m.finance_ui_journal_entry_default_description());
    const payload: CreateJournalEntryRequest = {
      reference_number: referenceNumber.trim() || undefined,
      accounting_date: accountingDate,
      amount: amountValue,
      currency_code: selectedCurrency,
      debit_account_id: debitAccountId,
      credit_account_id: creditAccountId,
      description,
      description_ar: description,
      lines: [
        {
          account_id: debitAccountId,
          debit_amount: amountValue,
          credit_amount: "0",
          description: notes.trim() || m.finance_ui_journal_from_account_description(),
        },
        {
          account_id: creditAccountId,
          debit_amount: "0",
          credit_amount: amountValue,
          description: notes.trim() || m.finance_ui_journal_to_account_description(),
        },
      ],
    };

    await api.createJournalEntry(payload);
    toast.success(m.finance_ui_journal_entry_posted_success());
    clearForm();
    showAddForm = false;
    await loadData();
  } catch (err: any) {
    console.error("Failed to post journal entry:", err);
    toast.error(err?.message || m.finance_ui_journal_entry_post_failed());
  } finally {
    isSubmitting = false;
  }
}

async function handleReverse(entry: JournalEntryDto) {
  const confirmed = await confirmStore.prompt({
    title: m.finance_ui_reverse_journal_entry(),
    description: m.finance_ui_reverse_journal_entry_confirm({ number: entry.reference_number || entry.id.slice(0, 8) }),
    confirmText: m.finance_ui_reverse_entry(),
    cancelText: m.finance_ui_cancel(),
    variant: "destructive",
  });

  if (!confirmed) return;

  try {
    await api.reverseJournalEntry(entry.id, { reason: m.finance_ui_journal_manual_reverse_reason() });
    toast.success(m.finance_ui_journal_entry_reversed_success());
    await loadData();
  } catch (err: any) {
    console.error("Failed to reverse journal entry:", err);
    toast.error(err?.message || m.finance_ui_journal_entry_reverse_failed());
  }
}

function handlePrint(entry: JournalEntryDto) {
  printParameters = {
    voucher_id: entry.id,
    reference_number: entry.reference_number,
    org_id: orgId,
  };
  printTitle = m.finance_ui_journal_entry_print_title({ number: entry.reference_number || entry.id.slice(0, 8) });
  printReportOpen = true;
}

// ── Grid Data ─────────────────────────────────────────────────────────
const gridSource = $derived(
  journalEntries.map((e, idx) => {
    const debitAcc = e.debit_account_id
      ? getAccountName(e.debit_account_id)
      : (e.lines && e.lines.length > 0 ? getAccountName(e.lines[0].account_id) : "—");

    const creditAcc = e.credit_account_id
      ? getAccountName(e.credit_account_id)
      : (e.lines && e.lines.length > 1 ? getAccountName(e.lines[1].account_id) : "—");

    const amt = Number(e.amount || (e.lines && e.lines.length > 0 ? e.lines[0].debit || e.lines[0].credit : 0));
    const curr = e.currency_code || e.currency || "YER";

    return {
      ...e,
      __idx: idx + 1,
      ref_display: e.reference_number || e.id.slice(0, 8),
      date_display: e.accounting_date ? formatDate(e.accounting_date) : "—",
      amount_formatted: formatCurrency(amt, curr),
      currency_display: curr,
      debit_account_display: debitAcc,
      credit_account_display: creditAcc,
      notes_display: e.description_ar || e.description || "—",
      status_display: e.is_posted ? m.finance_ui_posted_status() : m.finance_ui_draft_status(),
    };
  })
);

const filteredSource = $derived(
  gridSource.filter((row) => {
    if (filterDate && (!row.accounting_date || !String(row.accounting_date).startsWith(filterDate))) {
      return false;
    }
    return true;
  })
);

const columns: OrbisColumnDef[] = [
  { id: "__idx", prop: "__idx", header: "#", name: "#", width: 48, size: 48, readonly: true, pin: "left", align: "center" },
  { id: "ref_display", prop: "ref_display", header: m.finance_ui_voucher_number(), name: m.finance_ui_voucher_number(), width: 110, size: 110, readonly: true, cellClass: "font-mono font-bold" },
  { id: "date_display", prop: "date_display", header: m.finance_ui_date(), name: m.finance_ui_date(), width: 110, size: 110, readonly: true, align: "center" },
  { id: "debit_account_display", prop: "debit_account_display", header: m.finance_ui_debit_account(), name: m.finance_ui_debit_account(), width: 220, size: 220, readonly: true, cellClass: "font-medium" },
  { id: "credit_account_display", prop: "credit_account_display", header: m.finance_ui_credit_account(), name: m.finance_ui_credit_account(), width: 220, size: 220, readonly: true, cellClass: "font-medium" },
  { id: "amount_formatted", prop: "amount_formatted", header: m.finance_ui_amount(), name: m.finance_ui_amount(), width: 140, size: 140, readonly: true, align: "end", cellClass: "font-mono font-bold text-primary" },
  { id: "currency_display", prop: "currency_display", header: m.finance_ui_currency(), name: m.finance_ui_currency(), width: 80, size: 80, readonly: true, align: "center" },
  { id: "notes_display", prop: "notes_display", header: m.finance_ui_description_notes(), name: m.finance_ui_description_notes(), width: 240, size: 240, readonly: true },
  { id: "status_display", prop: "status_display", header: m.finance_ui_status(), name: m.finance_ui_status(), width: 90, size: 90, type: "badge", align: "center", readonly: true },
];

function exportCsv() {
  const rows = filteredSource.map((e) => [
    e.ref_display,
    e.date_display,
    e.debit_account_display,
    e.credit_account_display,
    e.amount_formatted,
    e.currency_display,
    e.notes_display,
    e.status_display,
  ]);
  const csv = [
    [m.finance_ui_voucher_number(), m.finance_ui_date(), m.finance_ui_debit_account(), m.finance_ui_credit_account(), m.finance_ui_amount(), m.finance_ui_currency(), m.finance_ui_notes(), m.finance_ui_status()],
    ...rows,
  ]
    .map((row) => row.map((v) => `"${String(v ?? "").replaceAll('"', '""')}"`).join(","))
    .join("\n");
  const url = URL.createObjectURL(new Blob(["\ufeff" + csv], { type: "text/csv;charset=utf-8" }));
  const link = document.createElement("a");
  link.href = url;
  link.download = `journal-entries-${orgId}.csv`;
  link.click();
  URL.revokeObjectURL(url);
}
</script>

<div class="flex h-full min-h-0 w-full flex-col overflow-hidden p-6 font-sans bg-background/50">
  <div class="flex min-h-0 w-full flex-1 flex-col gap-4" in:fade={{ duration: 150 }}>
    
    <!-- Top Action Bar -->
    <header class="flex shrink-0 items-center justify-between gap-4">
      <div class="flex items-center gap-2">
        <Button
          variant={showAddForm ? "secondary" : "default"}
          size="sm"
          class="h-8 px-3 text-xs font-bold shrink-0 gap-1.5"
          onclick={() => { showAddForm = !showAddForm; }}
        >
          {#if showAddForm}
            <XMark size={14} />
            <span>{m.finance_ui_cancel()}</span>
          {:else}
            <Plus size={14} />
            <span>{m.finance_ui_new_journal_entry()}</span>
          {/if}
        </Button>
      </div>

      <div class="flex items-center gap-2">
        <div class="relative w-48 sm:w-64">
          <Search size={14} class="absolute inset-s-2.5 top-1/2 -translate-y-1/2 text-muted-foreground pointer-events-none" />
          <Input
            type="text"
            placeholder={m.finance_ui_journal_search_placeholder()}
            bind:value={searchQuery}
            class="h-8 ps-8 text-xs bg-card border-border/60"
          />
        </div>

        <Input
          type="date"
          bind:value={filterDate}
          class="h-8 w-36 text-xs bg-card border-border/60 font-mono"
        />

        <Button variant="ghost" size="icon-sm" onclick={loadData} class="text-muted-foreground">
          {#if isLoading}<LoadingIndicator  size={14} />{:else}<Refresh size={14}   />{/if}
        </Button>

        <Button variant="ghost" size="icon-sm" onclick={exportCsv} class="text-muted-foreground">
          <Download size={14} />
        </Button>
      </div>
    </header>

    <!-- In-Page Creation Card (Matching FinanceOverview Card Style) -->
    {#if showAddForm}
      <div transition:slide={{ duration: 200 }}>
        <FinanceInlineFormCard
          title={m.finance_ui_journal_entry_create_heading()}
          onclose={() => (showAddForm = false)}
        >
            <div class="grid min-w-0 grid-cols-1 gap-4 sm:grid-cols-2 xl:grid-cols-4">
              <div class="min-w-0">
                <Label class="text-xs font-semibold text-foreground mb-1 block">{m.finance_ui_voucher_number_optional()}</Label>
                <Input
                  type="text"
                  bind:value={referenceNumber}
                  placeholder={m.finance_ui_automatic()}
                  class="h-8 text-xs font-mono bg-background border-border/60"
                />
              </div>

              <div class="min-w-0">
                <Label class="text-xs font-semibold text-foreground mb-1 block">{m.finance_ui_date()}</Label>
                <Input
                  type="date"
                  bind:value={accountingDate}
                  class="h-8 text-xs font-mono bg-background border-border/60"
                />
              </div>

              <div class="min-w-0">
                <Label class="text-xs font-semibold text-foreground mb-1 block">{m.finance_ui_amount()}</Label>
                <Input
                  type="number"
                  bind:value={amount}
                  placeholder="0.00"
                  class="h-8 text-xs font-mono font-bold text-primary bg-background border-border/60"
                />
              </div>

              <div class="min-w-0">
                <CurrencySelector
                  compact
                  title={m.finance_ui_currency()}
                  description=""
                  bind:currentCurrency={selectedCurrency}
                  currencyOptions={currencies}
                  onchange={handleCurrencyChange}
                  disabled={isLoading || isSubmitting || currencies.length < 2}
                />
              </div>
            </div>

            <div class="grid min-w-0 grid-cols-1 gap-4 sm:grid-cols-2">
              <div class="min-w-0">
                <Label class="text-xs font-semibold text-foreground mb-1 block">{m.finance_ui_debit_account()}</Label>
                <AccountSearchInput
                  bind:value={debitAccountId}
                  accounts={journalAccounts}
                  placeholder={m.finance_ui_debit_account_select_placeholder()}
                />
              </div>

              <div class="min-w-0">
                <Label class="text-xs font-semibold text-foreground mb-1 block">{m.finance_ui_credit_account()}</Label>
                <AccountSearchInput
                  bind:value={creditAccountId}
                  accounts={journalAccounts}
                  placeholder={m.finance_ui_credit_account_select_placeholder()}
                />
              </div>
            </div>

            <div>
              <Label class="text-xs font-semibold text-foreground mb-1 block">{m.finance_ui_description_notes()}</Label>
              <Input
                type="text"
                bind:value={notes}
                placeholder={m.finance_ui_journal_notes_placeholder()}
                class="h-8 text-xs bg-background border-border/60"
              />
            </div>

            <div class="flex items-center justify-end gap-2 pt-2 border-t border-border/40">
              <Button variant="ghost" size="sm" class="h-8 text-xs" onclick={clearForm}>
                {m.finance_ui_clear_fields()}
              </Button>
              <Button
                variant="default"
                size="sm"
                class="h-8 text-xs font-bold px-4"
                disabled={isSubmitting}
                onclick={handleSaveNew}
              >
                {isSubmitting ? m.finance_ui_saving() : m.finance_ui_journal_entry_save()}
              </Button>
            </div>
        </FinanceInlineFormCard>
      </div>
    {/if}

    <!-- Grid Container -->
    <div class="flex-1 min-h-0 overflow-hidden">
      <OrbisGrid
        columns={columns}
        source={filteredSource}
        loading={isLoading}
        clientQuery
        search={searchQuery}
        rtl={isAr()}
        readonly
        emptyTitle={m.finance_ui_no_journal_entries()}
      />
    </div>
  </div>
</div>

<PdfReportViewer
  bind:open={printReportOpen}
  reportTemplateId="journal-voucher"
  parameters={printParameters}
  title={printTitle}
/>
