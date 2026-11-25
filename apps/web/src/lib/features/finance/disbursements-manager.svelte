<!--
  @component DisbursementsManager
  Minimal, elegant, and high-performance Disbursement Vouchers (سندات الصرف) workspace.
  Adheres strictly to Orbis Clean Architecture and the FinanceOverview design system:
  - Clean card-based disbursement voucher entry
  - Direct selection of treasury (credit) and expense/beneficiary (debit) accounts
  - Sleek top toolbar with search, method filter, and export
  - High-performance OrbisGrid table
  - Real backend double-entry recording and printable vouchers
-->
<script lang="ts">
import { LoadingIndicator } from "@orbis/ui";
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import { api, extractErrorMessage, queryClient, queryKeys } from "$lib/api";
import type { AccountDto, CurrencyDto, JournalEntryDto } from "$lib/api/types";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
} from "@orbis/ui/select";
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
import { formatDateKey } from "$lib/core/i18n/format";
import { formatCurrency, formatDate, isAr } from "$lib/core/i18n";
import { fade, slide } from "svelte/transition";
import { orgContext } from "$lib/stores/org-context.svelte";
import { accountQueryOptions, isCashOrBankAccount } from "./queries";

let { orgId }: { orgId: string } = $props();

type DisbursementLine = {
  id: number;
  sequence: number;
  beneficiaryAccountId: string;
  amount: number | null;
  notes: string;
};

let nextDisbursementLineId = 1;
function createDisbursementLine(): DisbursementLine {
  const id = nextDisbursementLineId++;
  return {
    id,
    sequence: id,
    beneficiaryAccountId: "",
    amount: null,
    notes: "",
  };
}

// ── State ──────────────────────────────────────────────────────────────
let disbursements = $state<JournalEntryDto[]>([]);
let accounts = $state<AccountDto[]>([]);
let treasuries = $state<AccountDto[]>([]);
let currencies = $state<CurrencyDto[]>([]);
let isLoading = $state(true);
let isSubmitting = $state(false);
let formError = $state("");
let showAddForm = $state(false);

// Form Fields
let referenceNumber = $state("");
let voucherDate = $state(formatDateKey(new Date()));
let treasuryAccountId = $state(""); // Credit: Cash/Bank
let selectedCurrency = $state(orgContext.baseCurrency || "");
let paymentMethod = $state<"Cash" | "Bank" | "Check">("Cash");
let disbursementLines = $state<DisbursementLine[]>([createDisbursementLine()]);

const availableCurrencies = $derived([
  ...currencies,
  ...treasuries
    .map((account) => account.currency)
    .filter((currency) => Boolean(currency))
    .filter(
      (currency) =>
        !currencies.some(
          (option) => option.code.toUpperCase() === currency.toUpperCase(),
        ),
    )
    .map(
      (currency) =>
        ({
          code: currency,
          name: currency,
          name_ar: null,
          symbol: currency,
          symbol_position: "after",
          decimal_places: 2,
          exchange_rate: "1",
        }) as CurrencyDto,
    ),
]);
const treasuryAccountsForCurrency = $derived(
  treasuries.filter(
    (account) =>
      String(account.currency).toUpperCase() ===
      String(selectedCurrency).toUpperCase(),
  ),
);
const disbursementAccounts = $derived(
  accounts.filter(
    (account) =>
      account.is_active &&
      !isCashOrBankAccount(account) &&
      account.currency.toUpperCase() === selectedCurrency.toUpperCase(),
  ),
);
const calculatedTotal = $derived(
  disbursementLines.reduce(
    (total, line) => total + (Number(line.amount) || 0),
    0,
  ),
);

// Filters
let searchQuery = $state("");
let methodFilter = $state("all");
let dateRange = $state(createDefaultDateRange());

// Print
let printDialogOpen = $state(false);
let disbursementSelection = $state<SelectionSnapshot>({
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

    if (reloadCatalogs) accounts = accountsList;
    if (reloadCatalogs) treasuries = accountsList.filter(isCashOrBankAccount);
    if (reloadCatalogs) currencies = currenciesList;

    if (reloadCatalogs && treasuries.length > 0) {
      const selectedTreasury = treasuries.find(
        (account) => String(account.id) === String(treasuryAccountId),
      );
      const matchingTreasury =
        selectedTreasury ||
        treasuries.find(
          (account) =>
            String(account.currency).toUpperCase() ===
            String(selectedCurrency).toUpperCase(),
        ) ||
        treasuries[0];
      handleTreasuryChange(matchingTreasury.id);
    }

    disbursements = entriesList.filter((entry) => entry.source_type === "disbursement_voucher");
  } catch (e) {
    console.error("Failed to load disbursements data:", e);
    toast.error(m.finance_ui_disbursements_load_failed());
  } finally {
    if (reloadCatalogs) isLoading = false;
  }
}

function clearForm() {
  referenceNumber = "";
  voucherDate = formatDateKey(new Date());
  paymentMethod = "Cash";
  disbursementLines = [createDisbursementLine()];
}

function handleTreasuryChange(accountId: string) {
  treasuryAccountId = accountId;
  const currency = treasuries.find(
    (account) => account.id === accountId,
  )?.currency;
  if (currency) selectedCurrency = currency.toUpperCase();
  for (const line of disbursementLines) {
    const account = accounts.find(
      (candidate) => candidate.id === line.beneficiaryAccountId,
    );
    if (
      account &&
      currency &&
      account.currency.toUpperCase() !== currency.toUpperCase()
    ) {
      line.beneficiaryAccountId = "";
    }
  }
}

function handleCurrencyChange(currencyCode: string | undefined) {
  if (!currencyCode || currencyCode === selectedCurrency) return;
  selectedCurrency = currencyCode.toUpperCase();
  const treasury = treasuries.find(
    (account) => account.currency.toUpperCase() === currencyCode.toUpperCase(),
  );
  if (treasury) handleTreasuryChange(treasury.id);
  else {
    treasuryAccountId = "";
    for (const line of disbursementLines) line.beneficiaryAccountId = "";
  }
}

function addDisbursementLine() {
  const line = createDisbursementLine();
  disbursementLines = [...disbursementLines, line];
  return line.id;
}

function addDisbursementLineAndFocus() {
  const lineId = addDisbursementLine();
  requestAnimationFrame(() => {
    const accountInput = document.getElementById(
      `disbursement-line-account-${lineId}`,
    );
    if (accountInput instanceof HTMLInputElement) accountInput.focus();
  });
}

function handleQuickAddDisbursementLine(event: KeyboardEvent) {
  if (event.key !== "Enter" || !event.shiftKey || isSubmitting) return;
  event.preventDefault();
  event.stopPropagation();
  addDisbursementLineAndFocus();
}

function removeDisbursementLine(lineId: number) {
  if (disbursementLines.length === 1) {
    disbursementLines = [createDisbursementLine()];
    return;
  }
  disbursementLines = disbursementLines.filter((line) => line.id !== lineId);
}

const accountsById = $derived(new Map(accounts.map((account) => [account.id, account])));

function getAccountName(id?: string | null): string {
  if (!id) return "—";
  const acc = accountsById.get(id);
  if (!acc) return id.slice(0, 8);
  return acc.name_ar || acc.name || acc.code || "—";
}

async function handleSaveDisbursement() {
  if (isSubmitting) return;
  formError = "";
  if (disbursementLines.some((line) => line.amount == null || !Number.isFinite(line.amount) || line.amount <= 0)) {
    formError = m.finance_ui_disbursement_amount_invalid();
    return;
  }
  if (!treasuryAccountId) {
    formError = m.finance_ui_disbursement_treasury_required();
    return;
  }
  if (disbursementLines.some((line) => !line.beneficiaryAccountId)) {
    formError = m.finance_ui_disbursement_beneficiary_required();
    return;
  }
  if (
    disbursementLines.some(
      (line) =>
        !disbursementAccounts.some(
          (account) => account.id === line.beneficiaryAccountId,
        ),
    )
  ) {
    formError = m.finance_ui_disbursement_beneficiary_required();
    return;
  }
  if (
    disbursementLines.some(
      (line) => line.beneficiaryAccountId === treasuryAccountId,
    )
  ) {
    formError = m.finance_ui_disbursement_accounts_must_differ();
    return;
  }

  isSubmitting = true;
  let savedCount = 0;
  let failure: unknown;
  const batchSize = disbursementLines.length;
  try {
    for (const line of [...disbursementLines]) {
      const baseReference = referenceNumber.trim();
      const lineReference = baseReference
        ? batchSize > 1 || line.sequence > 1
          ? `${baseReference}-${String(line.sequence).padStart(2, "0")}`
          : baseReference
        : undefined;
      try {
        await api.processDisbursement({
          treasury_account_id: treasuryAccountId,
          beneficiary_account_id: line.beneficiaryAccountId,
          amount: String(line.amount),
          currency_code: selectedCurrency,
          payment_method: paymentMethod.toLowerCase(),
          reference: lineReference,
          notes: line.notes.trim() || undefined,
          voucher_date: voucherDate || undefined,
        });
        savedCount += 1;
        disbursementLines = disbursementLines.filter(
          (item) => item.id !== line.id,
        );
      } catch (err) {
        failure = err;
        break;
      }
    }

    if (savedCount > 0) {
      toast.success(m.finance_ui_disbursement_posted_success());
      await loadData(false);
    }
    if (failure) {
      console.error("Failed to record a disbursement in the batch:", failure);
      formError = extractErrorMessage(failure) || m.finance_ui_disbursement_save_failed();
    } else {
      clearForm();
      showAddForm = false;
    }
  } finally {
    isSubmitting = false;
  }
}

// ── Grid Data ─────────────────────────────────────────────────────────
const gridSource = $derived(
  disbursements.map((d, idx) => {
    const beneficiaryId = d.debit_account_id || d.lines?.[0]?.account_id;
    const treasuryId = d.credit_account_id || d.lines?.[1]?.account_id;
    const amt = Number(d.amount || d.lines?.[0]?.debit_amount || 0);
    const curr = d.currency_code || orgContext.baseCurrency;

    return {
      ...d,
      __idx: idx + 1,
      receipt_display: d.reference_number || d.id.slice(0, 8),
      date_display: d.accounting_date ? formatDate(d.accounting_date) : "—",
      beneficiary_display: getAccountName(beneficiaryId),
      treasury_display: getAccountName(treasuryId),
      amount_formatted: formatCurrency(amt, curr),
      currency_display: curr,
      notes_display: d.description_ar || d.description || "—",
      status_display: m.finance_ui_disbursed_status(),
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
const selectedDisbursements = $derived(
  filteredSource.filter((disbursement) =>
    disbursementSelection.mode === "allMatching"
      ? !disbursementSelection.ids.has(disbursement.id)
      : disbursementSelection.ids.has(disbursement.id),
  ),
);
let printRows = $state.raw<(typeof gridSource)[number][]>([]);

function openPrint(rows: (typeof gridSource)[number][]) {
  if (!rows.length) return;
  printRows = rows;
  printDialogOpen = true;
}

async function printDetail() {
  if (!detailRow) return;
  const row = detailRow;
  detailsOpen = false;
  await tick();
  openPrint([row]);
}

const disbursementPrintContexts = $derived(
  printRows.map((disbursement) => ({
    voucher: {
      number: disbursement.reference_number || disbursement.id.slice(0, 8),
      date: disbursement.accounting_date || "",
      primaryName: disbursement.beneficiary_display,
      secondaryName: disbursement.treasury_display,
      amount: Number(
        disbursement.amount || disbursement.lines?.[0]?.debit_amount || 0,
      ),
      currency:
        disbursement.currency_code ||
        orgContext.baseCurrency,
      description:
        disbursement.description_ar || disbursement.description || "",
    },
    $org: {
      name: orgContext.name,
      name_ar: orgContext.nameAr,
      currency: disbursement.currency_code || orgContext.baseCurrency,
      logo_url: orgContext.logo,
      primary_color: orgContext.primaryColor,
    },
  })),
);

function handleDisbursementSelection(selection: SelectionSnapshot) {
  disbursementSelection = selection;
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
    id: "receipt_display",
    prop: "receipt_display",
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
    id: "beneficiary_display",
    prop: "beneficiary_display",
    header: m.finance_ui_disbursement_beneficiary(),
    name: m.finance_ui_disbursement_beneficiary(),
    width: 220,
    size: 220,
    readonly: true,
    cellClass: "font-medium",
  },
  {
    id: "treasury_display",
    prop: "treasury_display",
    header: m.finance_ui_treasury_account(),
    name: m.finance_ui_treasury_account(),
    width: 180,
    size: 180,
    readonly: true,
  },
  {
    id: "amount_formatted",
    prop: "amount_formatted",
    header: m.finance_ui_disbursed_amount(),
    name: m.finance_ui_disbursed_amount(),
    width: 140,
    size: 140,
    readonly: true,
    align: "end",
    cellClass: "font-mono font-bold text-rose-600 dark:text-rose-400",
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
    width: 220,
    size: 220,
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
  const rows = filteredSource.map((d) => [
    d.receipt_display,
    d.date_display,
    d.beneficiary_display,
    d.treasury_display,
    d.amount_formatted,
    d.currency_display,
    d.notes_display,
    d.status_display,
  ]);
  const csv = [
    [
      m.finance_ui_voucher_number(),
      m.finance_ui_date(),
      m.finance_ui_beneficiary(),
      m.finance_ui_treasury(),
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
  link.download = `disbursements-${orgId}.csv`;
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
            <span>{m.finance_ui_new_disbursement()}</span>
          {/if}
        </Button>
        <Button
          variant="outline"
          size="sm"
          class="h-8 px-3 text-xs font-semibold shrink-0 gap-1.5"
          disabled={selectedDisbursements.length === 0}
          onclick={() => openPrint(selectedDisbursements)}
        >
          <Printer size={14} />
          <span>{m.finance_ui_print_disbursement()}{selectedDisbursements.length ? ` (${selectedDisbursements.length})` : ""}</span>
        </Button>
      </div>

      <div class="flex items-center gap-2">
        <div class="relative w-48 sm:w-64">
          <Search size={14} class="absolute inset-s-2.5 top-1/2 -translate-y-1/2 text-muted-foreground pointer-events-none" />
          <Input
            type="text"
            placeholder={m.finance_ui_disbursement_search_placeholder()}
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
      <div transition:slide={{ duration: 200 }}>
        <FinanceInlineFormCard
          title={m.finance_ui_disbursement_create_heading()}
          accent="rose"
          contentClass="gap-5"
          busy={isSubmitting}
          errorMessage={formError}
          onsave={handleSaveDisbursement}
          onclose={() => { formError = ""; showAddForm = false; }}
        >
              <div class="grid min-w-0 grid-cols-1 gap-3 @min-[30rem]/workspace:grid-cols-2 @min-[72rem]/workspace:grid-cols-5">
                <div class="min-w-0">
                  <Label for="disbursement-voucher-number-optional" class="mb-1 block text-xs font-semibold text-foreground">{m.finance_ui_voucher_number_optional()}</Label>
                  <Input id="disbursement-voucher-number-optional"
                    type="text"
                    bind:value={referenceNumber}
                    placeholder={m.finance_ui_automatic()}
                    class="h-9 text-xs font-mono"
                    disabled={isSubmitting}
                  />
                </div>
                <div class="min-w-0">
                  <Label for="disbursement-date" class="mb-1 block text-xs font-semibold text-foreground">{m.finance_ui_date()}</Label>
                  <Input id="disbursement-date" type="date" bind:value={voucherDate} class="h-9 text-xs font-mono" disabled={isSubmitting} />
                </div>
                <div class="min-w-0">
                  <Label for="disbursement-disbursement-from-account" class="mb-1 block text-xs font-semibold text-foreground">{m.finance_ui_disbursement_from_account()}</Label>
                  <AccountSearchInput inlineResults id="disbursement-disbursement-from-account"
                    value={treasuryAccountId}
                    placeholder={m.finance_ui_treasury_select_placeholder()}
                    accounts={treasuryAccountsForCurrency}
                    onchange={(value) => handleTreasuryChange(value ?? "")}
                    disabled={isSubmitting}
                  />
                </div>
                <div class="min-w-0">
                  <Label class="mb-1 block text-xs font-semibold text-foreground">{m.finance_ui_payment_method()}</Label>
                  <Select
                    type="single"
                    value={paymentMethod}
                    onValueChange={(value) => {
                      if (value === "Cash" || value === "Bank" || value === "Check") paymentMethod = value;
                    }}
                    disabled={isSubmitting}
                  >
                    <SelectTrigger class="h-9 w-full bg-background text-xs">
                      {paymentMethod === "Cash"
                        ? m.finance_ui_payment_method_cash()
                        : paymentMethod === "Bank"
                          ? m.finance_ui_payment_method_bank_transfer()
                          : m.finance_ui_payment_method_check()}
                    </SelectTrigger>
                    <SelectContent>
                      <SelectItem value="Cash">{m.finance_ui_payment_method_cash()}</SelectItem>
                      <SelectItem value="Bank">{m.finance_ui_payment_method_bank_transfer()}</SelectItem>
                      <SelectItem value="Check">{m.finance_ui_payment_method_check()}</SelectItem>
                    </SelectContent>
                  </Select>
                </div>
                <div class="min-w-0">
                  <CurrencySelector
                    compact
                    title={m.finance_ui_currency()}
                    description=""
                    currentCurrency={selectedCurrency}
                    currencyOptions={availableCurrencies}
                    onchange={handleCurrencyChange}
                    disabled={isSubmitting || isLoading}
                  />
                </div>
              </div>

            <section class="flex flex-col gap-3">
              <div class="flex flex-wrap items-center justify-between gap-2">
                <div>
                  <h3 class="text-sm font-semibold text-foreground">{m.finance_ui_disbursement_to_account()}</h3>
                </div>
                <Button
                  type="button"
                  variant="outline"
                  size="sm"
                  class="h-8 gap-1.5 text-xs"
                  onclick={addDisbursementLineAndFocus}
                  disabled={isSubmitting}
                >
                  <Plus size={14} />
                  {m.common_action_add()}
                </Button>
              </div>

              {#each disbursementLines as line (line.id)}
                <div class="min-w-0 rounded-lg border border-border/60 bg-background p-3 sm:p-4">
                  <div class="mb-3 flex items-center justify-between gap-2">
                    <span class="text-xs font-semibold text-foreground">
                      #{line.sequence}
                    </span>
                    <Button
                      type="button"
                      variant="ghost"
                      size="icon-xs"
                      class="text-muted-foreground hover:text-destructive"
                      onclick={() => removeDisbursementLine(line.id)}
                      disabled={isSubmitting}
                      aria-label={m.common_action_delete()}
                    >
                      <XMark size={14} />
                    </Button>
                  </div>
                  <div class="grid min-w-0 grid-cols-1 gap-3 sm:grid-cols-2 xl:grid-cols-[minmax(0,1.4fr)_minmax(8rem,0.7fr)]">
                    <div class="min-w-0">
                      <Label for={`disbursement-line-account-${line.id}`} class="mb-1 block text-xs font-semibold text-foreground">{m.finance_ui_disbursement_to_account()}</Label>
                      <AccountSearchInput
                  inlineResults
                        id={`disbursement-line-account-${line.id}`}
                        value={line.beneficiaryAccountId}
                        accounts={disbursementAccounts}
                        placeholder={m.finance_ui_expense_account_select_placeholder()}
                        onchange={(value) => (line.beneficiaryAccountId = value ?? "")}
                        onkeydown={handleQuickAddDisbursementLine}
                        disabled={isSubmitting}
                      />
                    </div>
                    <div class="min-w-0">
                      <Label for={"disbursement-field-4-" + line.id} class="mb-1 block text-xs font-semibold text-foreground">{m.finance_ui_disbursed_amount()}</Label>
                      <Input id={"disbursement-field-4-" + line.id}
                        type="number"
                        min="0.01"
                        step="any"
                        bind:value={line.amount}
                        onkeydown={handleQuickAddDisbursementLine}
                        placeholder="0.00"
                        class="h-9 text-xs font-mono font-bold text-rose-600 dark:text-rose-400"
                        disabled={isSubmitting}
                      />
                    </div>
                  </div>
                  <div class="mt-3">
                    <Label for={"disbursement-field-5-" + line.id} class="mb-1 block text-xs font-semibold text-foreground">{m.finance_ui_disbursement_notes()}</Label>
                    <Input id={"disbursement-field-5-" + line.id}
                      type="text"
                      bind:value={line.notes}
                      onkeydown={handleQuickAddDisbursementLine}
                      placeholder={m.finance_ui_disbursement_notes_placeholder()}
                      class="h-9 text-xs"
                      disabled={isSubmitting}
                    />
                  </div>
                </div>
              {/each}
            </section>

          {#snippet actions()}
            <div class="flex flex-col-reverse gap-3 sm:flex-row sm:items-center sm:justify-between">
              <div class="text-sm font-semibold text-foreground">
                {m.finance_ui_disbursed_amount()}:
                <span class="font-mono text-rose-600 dark:text-rose-400">{formatCurrency(calculatedTotal, selectedCurrency)}</span>
              </div>
              <div class="flex flex-wrap items-center justify-end gap-2">
                <Button type="button" variant="ghost" size="sm" class="h-9 text-xs" onclick={clearForm} disabled={isSubmitting}>
                  {m.finance_ui_clear_fields()}
                </Button>
                <Button type="submit"
                  variant="default"
                  size="sm"
                  class="h-9 text-xs font-bold"
                  disabled={isSubmitting}
                >
                  {isSubmitting ? m.finance_ui_saving() : `${m.finance_ui_disbursement_save()} (${disbursementLines.length})`}
                </Button>
              </div>
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
        onselection={handleDisbursementSelection}
        rtl={isAr()}
        readonly
        emptyTitle={m.finance_ui_no_disbursements()}
      />
    </div>
  </div>
</div>


<FinanceRecordDialog
  bind:open={detailsOpen}
  title={m.finance_ui_record_details()}
  fields={detailRow ? [
    { label: m.finance_ui_voucher_number(), value: detailRow.receipt_display },
    { label: m.finance_ui_date(), value: detailRow.date_display },
    { label: m.finance_ui_debit_account(), value: detailRow.beneficiary_display },
    { label: m.finance_ui_credit_account(), value: detailRow.treasury_display },
    { label: m.finance_ui_amount(), value: detailRow.amount_formatted },
    { label: m.finance_ui_currency(), value: detailRow.currency_display },
    { label: m.finance_ui_description_notes(), value: detailRow.notes_display },
  ] : []}
  onprint={printDetail}
>
  {#if detailRow?.lines?.length}
    <dl class="divide-y divide-border border-y border-border">
      {#each detailRow.lines as line (line.id)}
        <div class="grid gap-2 py-3 sm:grid-cols-[minmax(0,1fr)_auto_auto] sm:items-center">
          <dt class="text-sm font-medium">{getAccountName(line.account_id)}</dt>
          <dd class="text-xs text-muted-foreground">{m.finance_ui_debit_account()}: {formatCurrency(Number(line.debit_amount), detailRow?.currency_code)}</dd>
          <dd class="text-xs text-muted-foreground">{m.finance_ui_credit_account()}: {formatCurrency(Number(line.credit_amount), detailRow?.currency_code)}</dd>
        </div>
      {/each}
    </dl>
  {/if}
</FinanceRecordDialog>

<PrintDialog
  bind:open={printDialogOpen}
  entityType="finance_disbursement"
  documentType="VOUCHER"
  entityIds={printRows.map((disbursement) => disbursement.id)}
  entityLabels={printRows.map((disbursement) => disbursement.receipt_display)}
  contexts={disbursementPrintContexts}
/>
