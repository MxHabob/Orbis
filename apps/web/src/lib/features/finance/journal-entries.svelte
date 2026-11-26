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
import JournalLineDetails from "./components/journal-line-details.svelte";
import VoucherMetadata from "./components/voucher-metadata.svelte";
import { LoadingIndicator } from "@orbis/ui";
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import { api, extractErrorMessage, queryClient, queryKeys } from "$lib/api";
import { formatDateKey } from "$lib/core/i18n/format";
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
import { Plus, Refresh, Search, Download, XMark, Printer } from "@orbis/icons";
import {
  OrbisGrid,
  type GridActionEvent,
  type OrbisColumnDef,
  type SelectionSnapshot,
  selectionColumn,
} from "@orbis/svelte-grid";
import AccountSearchInput from "$lib/features/finance/components/account-search-input.svelte";
import { tick } from "svelte";
import { financeRecordColumn } from "./record-presentation";
import FinanceRecordDialog from "./components/finance-record-dialog.svelte";
import FinanceInlineFormCard from "$lib/features/finance/components/finance-inline-form-card.svelte";
import DateRangeFilter from "$lib/shared/components/date-range-filter.svelte";
import { createDefaultDateRange } from "$lib/shared/components/date-range";
import CurrencySelector from "$lib/shared/components/currency-selector.svelte";
import PrintDialog from "$lib/shared/components/print-dialog.svelte";
import { confirmStore } from "$lib/stores";
import { orgContext } from "$lib/stores/org-context.svelte";
import { accountQueryOptions } from "./queries";

let { orgId }: { orgId: string } = $props();

// ── State ──────────────────────────────────────────────────────────────
let journalEntries = $state<JournalEntryDto[]>([]);
let accounts = $state<AccountDto[]>([]);
let currencies = $state<CurrencyDto[]>([]);
let isLoading = $state(true);
let isSubmitting = $state(false);
let formError = $state("");
let showAddForm = $state(false);

// Form Fields
let referenceNumber = $state("");
let accountingDate = $state(formatDateKey(new Date()));
let amount = $state<number | null>(null);
let selectedCurrency = $state(orgContext.baseCurrency || "");
let debitAccountId = $state("");
let creditAccountId = $state("");
let notes = $state("");

// Filters
let searchQuery = $state("");
let dateRange = $state(createDefaultDateRange());


const journalAccounts = $derived(
  accounts.filter(
    (account) =>
      account.is_active &&
      account.currency.toUpperCase() === selectedCurrency.toUpperCase(),
  ),
);

// Print
let printReportOpen = $state(false);
let journalSelection = $state<SelectionSnapshot>({
  mode: "explicit",
  ids: new Set(),
  range: null,
  active: null,
});

onMount(async () => {
  await loadData();
});

async function loadData(reloadCatalogs = true) {
  if (reloadCatalogs) isLoading = true;
  try {
    const [entriesList, accountsList, currenciesList] = await Promise.all([
      api.listJournalEntries(),
      reloadCatalogs ? queryClient.fetchQuery(accountQueryOptions(orgId)) : Promise.resolve(accounts),
      reloadCatalogs
        ? queryClient.fetchQuery({
            queryKey: queryKeys.finance.currencies(),
            queryFn: () => api.listCurrencies(),
            staleTime: 10 * 60 * 1000,
          })
        : Promise.resolve(currencies),
    ]);

    journalEntries = entriesList;
    if (reloadCatalogs) accounts = accountsList;
    if (reloadCatalogs) currencies = currenciesList;

    if (!currencies.some((currency) => currency.code === selectedCurrency)) {
      selectedCurrency = orgContext.baseCurrency || currencies[0]?.code || "";
    }
  } catch (e) {
    console.error("Failed to load journal entries master data:", e);
    toast.error(m.finance_ui_journal_entries_load_failed());
  } finally {
    if (reloadCatalogs) isLoading = false;
  }
}

function clearForm() {
  referenceNumber = "";
  accountingDate = formatDateKey(new Date());
  amount = null;
  debitAccountId = "";
  creditAccountId = "";
  notes = "";
  formError = "";
}

function handleCurrencyChange(currency: string | undefined) {
  if (!currency || currency === selectedCurrency) return;
  selectedCurrency = currency;
  if (!journalAccounts.some((account) => account.id === debitAccountId))
    debitAccountId = "";
  if (!journalAccounts.some((account) => account.id === creditAccountId))
    creditAccountId = "";
}

const accountsById = $derived(new Map(accounts.map((account) => [account.id, account])));

function getAccountName(id?: string | null): string {
  if (!id) return "—";
  const acc = accountsById.get(id);
  if (!acc) return id.slice(0, 8);
  return acc.name_ar || acc.name || acc.code || "—";
}

async function handleSaveNew() {
  if (isSubmitting) return;
  formError = "";
  if (amount == null || !Number.isFinite(amount) || amount <= 0) {
    formError = m.finance_ui_journal_amount_invalid();
    return;
  }
  if (!debitAccountId) {
    formError = m.finance_ui_debit_account_required();
    return;
  }
  if (!creditAccountId) {
    formError = m.finance_ui_credit_account_required();
    return;
  }
  if (debitAccountId === creditAccountId) {
    formError = m.finance_ui_journal_accounts_must_differ();
    return;
  }
  if (!selectedCurrency) {
    formError = m.finance_ui_currency_select_placeholder();
    return;
  }
  if (
    !journalAccounts.some((account) => account.id === debitAccountId) ||
    !journalAccounts.some((account) => account.id === creditAccountId)
  ) {
    formError = m.finance_ui_journal_accounts_must_differ();
    return;
  }

  isSubmitting = true;
  try {
    const amountValue = String(amount);
    const description =
      notes.trim() ||
      (referenceNumber
        ? m.finance_ui_journal_entry_numbered_description({
            number: referenceNumber,
          })
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
          description:
            notes.trim() || m.finance_ui_journal_from_account_description(),
        },
        {
          account_id: creditAccountId,
          debit_amount: "0",
          credit_amount: amountValue,
          description:
            notes.trim() || m.finance_ui_journal_to_account_description(),
        },
      ],
    };

    await api.createJournalEntry(payload);
    toast.success(m.finance_ui_journal_entry_posted_success());
    clearForm();
    showAddForm = false;
    await loadData(false);
  } catch (err) {
    console.error("Failed to post journal entry:", err);
    formError = extractErrorMessage(err) || m.finance_ui_journal_entry_post_failed();
  } finally {
    isSubmitting = false;
  }
}

async function handleReverse(entry: JournalEntryDto) {
  const confirmed = await confirmStore.prompt({
    title: m.finance_ui_reverse_journal_entry(),
    description: m.finance_ui_reverse_journal_entry_confirm({
      number: entry.reference_number || entry.id.slice(0, 8),
    }),
    confirmText: m.finance_ui_reverse_entry(),
    cancelText: m.finance_ui_cancel(),
    variant: "destructive",
  });

  if (!confirmed) return;

  try {
    await api.reverseJournalEntry(entry.id, {
      reason: m.finance_ui_journal_manual_reverse_reason(),
    });
    toast.success(m.finance_ui_journal_entry_reversed_success());
    await loadData();
  } catch (err: any) {
    console.error("Failed to reverse journal entry:", err);
    toast.error(err?.message || m.finance_ui_journal_entry_reverse_failed());
  }
}

// ── Grid Data ─────────────────────────────────────────────────────────
const gridSource = $derived(
  journalEntries.map((e, idx) => {
    const debitAcc = getAccountName(e.debit_account_id);
    const creditAcc = getAccountName(e.credit_account_id);
    const amt = Number(e.amount);
    const curr = e.currency_code;

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
      status_display:
        e.status === "posted" || e.status === "reconciled"
          ? m.finance_ui_posted_status()
          : e.status === "draft"
            ? m.finance_ui_draft_status()
            : e.status,
    };
  }),
);

const filteredSource = $derived(
  gridSource.filter((row) => {
    if (
      (dateRange.fromDate &&
        (!row.accounting_date ||
          String(row.accounting_date).slice(0, 10) < dateRange.fromDate)) ||
      (dateRange.toDate &&
        (!row.accounting_date ||
          String(row.accounting_date).slice(0, 10) > dateRange.toDate))
    ) {
      return false;
    }
    return true;
  }),
);
const selectedJournalEntries = $derived(
  filteredSource.filter((entry) =>
    journalSelection.mode === "allMatching"
      ? !journalSelection.ids.has(entry.id)
      : journalSelection.ids.has(entry.id),
  ),
);
let printRows = $state.raw<(typeof gridSource)[number][]>([]);

function openPrint(rows: (typeof gridSource)[number][]) {
  if (!rows.length) return;
  printRows = rows;
  printReportOpen = true;
}

async function printDetail() {
  if (!detailRow) return;
  const row = detailRow;
  detailsOpen = false;
  await tick();
  openPrint([row]);
}

const journalPrintContexts = $derived(
  printRows.map((entry) => ({
    voucher: {
      number: entry.ref_display,
      date: entry.accounting_date,
      primaryName: entry.debit_account_display,
      secondaryName: entry.credit_account_display,
      amount: Number(entry.amount),
      currency: entry.currency_code,
      description: entry.description_ar || entry.description || "",
      status: entry.status,
    },
    $org: {
      name: orgContext.name,
      name_ar: orgContext.nameAr,
      currency: entry.currency_code || orgContext.baseCurrency,
      logo_url: orgContext.logo,
      primary_color: orgContext.primaryColor,
    },
  })),
);

function handleJournalSelection(selection: SelectionSnapshot) {
  journalSelection = selection;
}


const detailsColumn = $derived(financeRecordColumn());
let detailsOpen = $state(false);
let detailRow = $state<(typeof gridSource)[number] | undefined>();

function handleRecordAction(event: GridActionEvent) {
  if (event.actionId !== "dblclick" && event.actionId !== "view") return;
  const row = gridSource.find((item) => item.id === event.row.id);
  if (!row) return;
  detailRow = row;
  detailsOpen = true;
}

const columns: OrbisColumnDef[] = [
  {
    id: "__idx",
    prop: "__idx",
    header: "#",
    name: "#",
    width: 48,
    size: 48,
    readonly: true,
    pin: "left",
    align: "center",
  },
  {
    pin: "colPinStart",
    id: "ref_display",
    prop: "ref_display",
    header: m.finance_ui_voucher_number(),
    name: m.finance_ui_voucher_number(),
    width: 110,
    size: 110,
    readonly: true,
    cellClass: "font-mono font-bold",
  },
  {
    id: "date_display",
    prop: "date_display",
    header: m.finance_ui_date(),
    name: m.finance_ui_date(),
    width: 110,
    size: 110,
    readonly: true,
    align: "center",
  },
  {
    id: "debit_account_display",
    prop: "debit_account_display",
    header: m.finance_ui_debit_account(),
    name: m.finance_ui_debit_account(),
    width: 220,
    size: 220,
    readonly: true,
    cellClass: "font-medium",
  },
  {
    id: "credit_account_display",
    prop: "credit_account_display",
    header: m.finance_ui_credit_account(),
    name: m.finance_ui_credit_account(),
    width: 220,
    size: 220,
    readonly: true,
    cellClass: "font-medium",
  },
  {
    id: "amount_formatted",
    prop: "amount_formatted",
    header: m.finance_ui_amount(),
    name: m.finance_ui_amount(),
    width: 140,
    size: 140,
    readonly: true,
    align: "end",
    cellClass: "font-mono font-bold text-primary",
  },
  {
    id: "currency_display",
    prop: "currency_display",
    header: m.finance_ui_currency(),
    name: m.finance_ui_currency(),
    width: 80,
    size: 80,
    readonly: true,
    align: "center",
  },
  {
    id: "notes_display",
    prop: "notes_display",
    header: m.finance_ui_description_notes(),
    name: m.finance_ui_description_notes(),
    width: 240,
    size: 240,
    readonly: true,
  },
  {
    id: "status_display",
    prop: "status_display",
    header: m.finance_ui_status(),
    name: m.finance_ui_status(),
    width: 90,
    size: 90,
    type: "badge",
    align: "center",
    readonly: true,
  },
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
    [
      m.finance_ui_voucher_number(),
      m.finance_ui_date(),
      m.finance_ui_debit_account(),
      m.finance_ui_credit_account(),
      m.finance_ui_amount(),
      m.finance_ui_currency(),
      m.finance_ui_notes(),
      m.finance_ui_status(),
    ],
    ...rows,
  ]
    .map((row) =>
      row.map((v) => `"${String(v ?? "").replaceAll('"', '""')}"`).join(","),
    )
    .join("\n");
  const url = URL.createObjectURL(
    new Blob(["\ufeff" + csv], { type: "text/csv;charset=utf-8" }),
  );
  const link = document.createElement("a");
  link.href = url;
  link.download = `journal-entries-${orgId}.csv`;
  link.click();
  URL.revokeObjectURL(url);
}
</script>

<div class="flex h-full min-h-0 w-full flex-col overflow-hidden p-6 font-sans bg-background/50">
  <div class="flex min-h-0 w-full flex-1 flex-col gap-4">
    
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
        <Button
          variant="outline"
          size="sm"
          class="h-8 px-3 text-xs font-semibold shrink-0 gap-1.5"
          disabled={selectedJournalEntries.length === 0}
          onclick={() => openPrint(selectedJournalEntries)}
        >
          <Printer size={14} />
          <span>{m.finance_ui_print_journal_entry()}{selectedJournalEntries.length ? ` (${selectedJournalEntries.length})` : ""}</span>
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

        <DateRangeFilter bind:value={dateRange} />

        <Button variant="ghost" size="icon-sm" onclick={() => loadData()} disabled={isLoading || isSubmitting} aria-label={m.common_control_refresh()} class="text-muted-foreground">
          {#if isLoading}<LoadingIndicator  size={14} />{:else}<Refresh size={14}   />{/if}
        </Button>

        <Button variant="ghost" size="icon-sm" onclick={exportCsv} class="text-muted-foreground">
          <Download size={14} />
        </Button>
      </div>
    </header>

    <!-- In-Page Creation Card (Matching FinanceOverview Card Style) -->
    {#if showAddForm}
      <div>
        <FinanceInlineFormCard
          title={m.finance_ui_journal_entry_create_heading()}
          busy={isSubmitting}
          errorMessage={formError}
          onsave={handleSaveNew}
          onclose={() => { formError = ""; showAddForm = false; }}
        >
          {#snippet metadata()}
            <VoucherMetadata bind:number={referenceNumber} bind:date={accountingDate} />
          {/snippet}
            <h3 class="text-sm font-semibold">{m.finance_ui_payment_details()}</h3>
            <div class="grid min-w-0 grid-cols-1 gap-4 @min-[30rem]/workspace:grid-cols-2">
              <div class="min-w-0">
                <CurrencySelector
                  compact
                  title={m.finance_ui_currency()}
                  description=""
                  bind:currentCurrency={selectedCurrency}
                  currencyOptions={currencies}
                  onchange={handleCurrencyChange}
                  disabled={isLoading || isSubmitting}
                />
              </div>
              <div class="min-w-0">
                <Label for="journal-amount" class="text-xs font-semibold text-foreground mb-1 block">{m.finance_ui_amount()}</Label>
                <Input id="journal-amount"
                  type="number"
                  step="any"
                  min="0.01"
                  required
                  bind:value={amount}
                  placeholder="0.00"
                  class="h-8 text-xs font-mono font-bold text-primary bg-background border-border/60"
                />
              </div>
            </div>

            <div class="grid min-w-0 grid-cols-1 gap-4 sm:grid-cols-2">
              <div class="min-w-0">
                <Label for="journal-debit-account" class="text-xs font-semibold text-foreground mb-1 block">{m.finance_ui_debit_account()}</Label>
                <AccountSearchInput inlineResults id="journal-debit-account"
                  bind:value={debitAccountId}
                  accounts={journalAccounts}
                  placeholder={m.finance_ui_debit_account_select_placeholder()}
                />
              </div>

              <div class="min-w-0">
                <Label for="journal-credit-account" class="text-xs font-semibold text-foreground mb-1 block">{m.finance_ui_credit_account()}</Label>
                <AccountSearchInput inlineResults id="journal-credit-account"
                  bind:value={creditAccountId}
                  accounts={journalAccounts}
                  placeholder={m.finance_ui_credit_account_select_placeholder()}
                />
              </div>
            </div>

            <div>
              <Label for="journal-description-notes" class="text-xs font-semibold text-foreground mb-1 block">{m.finance_ui_description_notes()}</Label>
              <Input id="journal-description-notes"
                type="text"
                bind:value={notes}
                placeholder={m.finance_ui_journal_notes_placeholder()}
                class="h-8 text-xs bg-background border-border/60"
              />
            </div>

          {#snippet actions()}
            <div class="flex items-center justify-end gap-2 pt-2 border-t border-border/40">
              <Button type="button" variant="ghost" size="sm" class="h-8 text-xs" onclick={clearForm} disabled={isSubmitting}>
                {m.finance_ui_clear_fields()}
              </Button>
              <Button type="submit"
                variant="default"
                size="sm"
                class="h-8 text-xs font-bold px-4"
                disabled={isSubmitting}
              >
                {isSubmitting ? m.finance_ui_saving() : m.finance_ui_journal_entry_save()}
              </Button>
            </div>
          {/snippet}
        </FinanceInlineFormCard>
      </div>
    {/if}

    <!-- Grid Container -->
    <div class="flex-1 min-h-0 overflow-hidden">
      <OrbisGrid
        columns={[selectionColumn(), ...columns, detailsColumn]}
        onaction={handleRecordAction}
        source={filteredSource}
        loading={isLoading}
        clientQuery
        showSearch={false}
        search={searchQuery}
        onselection={handleJournalSelection}
        rtl={isAr()}
        readonly
        emptyTitle={m.finance_ui_no_journal_entries()}
      />
    </div>
  </div>
</div>


<FinanceRecordDialog
  bind:open={detailsOpen}
  title={m.finance_ui_record_details()}
  fields={detailRow ? [
    { label: m.finance_ui_voucher_number(), value: detailRow.ref_display },
    { label: m.finance_ui_date(), value: detailRow.date_display },
    { label: m.finance_ui_debit_account(), value: detailRow.debit_account_display },
    { label: m.finance_ui_credit_account(), value: detailRow.credit_account_display },
    { label: m.finance_ui_amount(), value: detailRow.amount_formatted },
    { label: m.finance_ui_currency(), value: detailRow.currency_display },
    { label: m.finance_ui_receipt_status(), value: detailRow.status_display },
    { label: m.finance_ui_description_notes(), value: detailRow.notes_display },
  ] : []}
  onprint={printDetail}
>
  <JournalLineDetails entry={detailRow} accountName={getAccountName} />
</FinanceRecordDialog>

<PrintDialog
  bind:open={printReportOpen}
  entityType="finance_journal"
  documentType="VOUCHER"
  entityIds={printRows.map((entry) => entry.id)}
  entityLabels={printRows.map((entry) => entry.ref_display)}
  contexts={journalPrintContexts}
/>
