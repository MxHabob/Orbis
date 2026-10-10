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
import { api, extractErrorMessage } from "$lib/api";
import type { AccountDto, CurrencyDto } from "$lib/api/types";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
import { Card, CardHeader, CardTitle, CardContent } from "@orbis/ui/card";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
} from "@orbis/ui/select";
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
import SearchableSelect from "$lib/shared/components/searchable-select.svelte";
import PdfReportViewer from "$lib/shared/components/pdf-report-viewer.svelte";
import { formatCurrency, formatDate, isAr } from "$lib/core/i18n";
import { fade, slide } from "svelte/transition";
import { isCashOrBankAccount } from "./queries";

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
  return { id, sequence: id, beneficiaryAccountId: "", amount: null, notes: "" };
}

// ── State ──────────────────────────────────────────────────────────────
let disbursements = $state<any[]>([]);
let accounts = $state<AccountDto[]>([]);
let treasuries = $state<AccountDto[]>([]);
let currencies = $state<CurrencyDto[]>([]);
let isLoading = $state(true);
let isSubmitting = $state(false);
let showAddForm = $state(false);

// Form Fields
let referenceNumber = $state("");
let voucherDate = $state(getLocalDateInputValue());
let treasuryAccountId = $state(""); // Credit: Cash/Bank
let paymentMethod = $state<"Cash" | "Bank" | "Check">("Cash");
let disbursementLines = $state<DisbursementLine[]>([createDisbursementLine()]);
function getLocalDateInputValue(date = new Date()) {
  return new Date(date.getTime() - date.getTimezoneOffset() * 60_000).toISOString().slice(0, 10);
}

const selectedCurrency = $derived(
  treasuries.find((account) => String(account.id) === String(treasuryAccountId))?.currency || "",
);
const availableCurrencies = $derived(
  currencies.filter((currency) =>
    treasuries.some((account) => account.currency.toUpperCase() === currency.code.toUpperCase())
  ),
);
const disbursementAccounts = $derived(
  accounts.filter((account) =>
    account.is_active &&
    !isCashOrBankAccount(account) &&
    account.currency.toUpperCase() === selectedCurrency.toUpperCase()
  ),
);
const calculatedTotal = $derived(
  disbursementLines.reduce((total, line) => total + (Number(line.amount) || 0), 0),
);

// Filters
let searchQuery = $state("");
let methodFilter = $state("all");
let filterDate = $state(getLocalDateInputValue());

// Print
let printDialogOpen = $state(false);
let printParameters = $state<Record<string, any>>({});
let printTitle = $state(m.finance_ui_print_disbursement());

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

    accounts = accountsList;
    treasuries = accountsList.filter(isCashOrBankAccount);
    currencies = currenciesList;

    if (treasuries.length > 0 && !treasuries.some((account) => account.id === treasuryAccountId)) {
      handleTreasuryChange(treasuries[0].id);
    }

    // Identify disbursement vouchers (entries where credit is a treasury/cash account)
    const treasuryIds = new Set(treasuries.map((t) => t.id));
    disbursements = entriesList.filter((e: any) => {
      const creditId = e.credit_account_id || e.lines?.[1]?.account_id;
      return creditId && treasuryIds.has(creditId);
    });
  } catch (e) {
    console.error("Failed to load disbursements data:", e);
    toast.error(m.finance_ui_disbursements_load_failed());
  } finally {
    isLoading = false;
  }
}

function clearForm() {
  referenceNumber = "";
  voucherDate = getLocalDateInputValue();
  paymentMethod = "Cash";
  disbursementLines = [createDisbursementLine()];
}

function handleTreasuryChange(accountId: string) {
  treasuryAccountId = accountId;
  const currency = treasuries.find((account) => account.id === accountId)?.currency;
  for (const line of disbursementLines) {
    const account = accounts.find((candidate) => candidate.id === line.beneficiaryAccountId);
    if (account && currency && account.currency.toUpperCase() !== currency.toUpperCase()) {
      line.beneficiaryAccountId = "";
    }
  }
}

function handleCurrencyChange(currencyCode: string | undefined) {
  if (!currencyCode || currencyCode === selectedCurrency) return;
  const treasury = treasuries.find((account) =>
    account.currency.toUpperCase() === currencyCode.toUpperCase()
  );
  if (treasury) handleTreasuryChange(treasury.id);
}

function addDisbursementLine() {
  disbursementLines = [...disbursementLines, createDisbursementLine()];
}

function removeDisbursementLine(lineId: number) {
  if (disbursementLines.length === 1) {
    disbursementLines = [createDisbursementLine()];
    return;
  }
  disbursementLines = disbursementLines.filter((line) => line.id !== lineId);
}

function getAccountName(id?: string | null): string {
  if (!id) return "—";
  const acc = accounts.find((a) => a.id === id);
  if (!acc) return id.slice(0, 8);
  return acc.name_ar || acc.name || acc.code || "—";
}

async function handleSaveDisbursement() {
  if (disbursementLines.some((line) => !line.amount || line.amount <= 0)) {
    toast.error(m.finance_ui_disbursement_amount_invalid());
    return;
  }
  if (!treasuryAccountId) {
    toast.error(m.finance_ui_disbursement_treasury_required());
    return;
  }
  if (disbursementLines.some((line) => !line.beneficiaryAccountId)) {
    toast.error(m.finance_ui_disbursement_beneficiary_required());
    return;
  }
  if (disbursementLines.some((line) => !disbursementAccounts.some((account) => account.id === line.beneficiaryAccountId))) {
    toast.error(m.finance_ui_disbursement_beneficiary_required());
    return;
  }
  if (disbursementLines.some((line) => line.beneficiaryAccountId === treasuryAccountId)) {
    toast.error(m.finance_ui_disbursement_accounts_must_differ());
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
        disbursementLines = disbursementLines.filter((item) => item.id !== line.id);
      } catch (err) {
        failure = err;
        break;
      }
    }

    if (savedCount > 0) {
      toast.success(m.finance_ui_disbursement_posted_success());
      await loadData();
    }
    if (failure) {
      console.error("Failed to record a disbursement in the batch:", failure);
      toast.error(extractErrorMessage(failure) || m.finance_ui_disbursement_save_failed());
    } else {
      clearForm();
      showAddForm = false;
    }
  } finally {
    isSubmitting = false;
  }
}

function handlePrint(d: any) {
  printParameters = {
    voucher_id: d.id,
    reference_number: d.reference_number || d.id.slice(0, 8),
    org_id: orgId,
  };
  printTitle = m.finance_ui_disbursement_print_title({ number: d.reference_number || d.id.slice(0, 8) });
  printDialogOpen = true;
}

// ── Grid Data ─────────────────────────────────────────────────────────
const gridSource = $derived(
  disbursements.map((d, idx) => {
    const beneficiaryId = d.debit_account_id || d.lines?.[0]?.account_id;
    const treasuryId = d.credit_account_id || d.lines?.[1]?.account_id;
    const amt = Number(d.amount || d.lines?.[0]?.debit || 0);
    const curr = d.currency_code || d.currency || "YER";

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
  { id: "receipt_display", prop: "receipt_display", header: m.finance_ui_voucher_number(), name: m.finance_ui_voucher_number(), width: 110, size: 110, readonly: true, cellClass: "font-mono font-bold" },
  { id: "date_display", prop: "date_display", header: m.finance_ui_date(), name: m.finance_ui_date(), width: 110, size: 110, readonly: true, align: "center" },
  { id: "beneficiary_display", prop: "beneficiary_display", header: m.finance_ui_disbursement_beneficiary(), name: m.finance_ui_disbursement_beneficiary(), width: 220, size: 220, readonly: true, cellClass: "font-medium" },
  { id: "treasury_display", prop: "treasury_display", header: m.finance_ui_treasury_account(), name: m.finance_ui_treasury_account(), width: 180, size: 180, readonly: true },
  { id: "amount_formatted", prop: "amount_formatted", header: m.finance_ui_disbursed_amount(), name: m.finance_ui_disbursed_amount(), width: 140, size: 140, readonly: true, align: "end", cellClass: "font-mono font-bold text-rose-600 dark:text-rose-400" },
  { id: "currency_display", prop: "currency_display", header: m.finance_ui_currency(), name: m.finance_ui_currency(), width: 80, size: 80, readonly: true, align: "center" },
  { id: "notes_display", prop: "notes_display", header: m.finance_ui_description_notes(), name: m.finance_ui_description_notes(), width: 220, size: 220, readonly: true },
  { id: "status_display", prop: "status_display", header: m.finance_ui_status(), name: m.finance_ui_status(), width: 90, size: 90, type: "badge", align: "center", readonly: true },
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
    [m.finance_ui_voucher_number(), m.finance_ui_date(), m.finance_ui_beneficiary(), m.finance_ui_treasury(), m.finance_ui_amount(), m.finance_ui_currency(), m.finance_ui_notes(), m.finance_ui_status()],
    ...rows,
  ]
    .map((row) => row.map((v) => `"${String(v ?? "").replaceAll('"', '""')}"`).join(","))
    .join("\n");
  const url = URL.createObjectURL(new Blob(["\ufeff" + csv], { type: "text/csv;charset=utf-8" }));
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
      <div transition:slide={{ duration: 200 }} class="min-h-0 max-h-[min(70vh,44rem)] shrink-0 overflow-x-hidden overflow-y-auto overscroll-contain">
        <Card class="border border-primary/20 bg-card shadow-sm text-start">
          <CardHeader class="py-3 px-4 border-b border-border/40">
            <div class="flex items-center justify-between">
              <div class="flex items-center gap-2">
                <span class="h-2 w-2 rounded-full bg-rose-500"></span>
                <CardTitle class="text-xs font-bold uppercase tracking-wider text-foreground">
                  {m.finance_ui_disbursement_create_heading()}
                </CardTitle>
              </div>
              <Button variant="ghost" size="icon-xs" onclick={() => showAddForm = false} class="text-muted-foreground">
                <XMark size={14} />
              </Button>
            </div>
          </CardHeader>

          <CardContent class="flex flex-col gap-5 p-4">
              <div class="grid min-w-0 grid-cols-1 gap-3 sm:grid-cols-2 xl:grid-cols-5">
                <div class="min-w-0">
                  <Label class="mb-1 block text-xs font-semibold text-foreground">{m.finance_ui_voucher_number_optional()}</Label>
                  <Input
                    type="text"
                    bind:value={referenceNumber}
                    placeholder={m.finance_ui_automatic()}
                    class="h-9 text-xs font-mono"
                    disabled={isSubmitting}
                  />
                </div>
                <div class="min-w-0">
                  <Label class="mb-1 block text-xs font-semibold text-foreground">{m.finance_ui_date()}</Label>
                  <Input type="date" bind:value={voucherDate} class="h-9 text-xs font-mono" disabled={isSubmitting} />
                </div>
                <div class="min-w-0">
                  <Label class="mb-1 block text-xs font-semibold text-foreground">{m.finance_ui_disbursement_from_account()}</Label>
                  <AccountSearchInput
                    value={treasuryAccountId}
                    placeholder={m.finance_ui_treasury_select_placeholder()}
                    accounts={treasuries}
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
                  <Label class="mb-1 block text-xs font-semibold text-foreground">{m.finance_ui_currency()}</Label>
                  <SearchableSelect
                    value={selectedCurrency}
                    placeholder={m.finance_ui_currency_placeholder()}
                    items={availableCurrencies}
                    getItemValue={(currency) => currency.code}
                    getItemLabel={(currency) => currency.name}
                    onchange={handleCurrencyChange}
                    disabled={isSubmitting}
                  />
                </div>
              </div>

            <section class="flex flex-col gap-3">
              <div class="flex flex-wrap items-center justify-between gap-2">
                <div>
                  <h3 class="text-sm font-semibold text-foreground">{m.finance_ui_disbursement_to_account()}</h3>
                  <p class="mt-0.5 text-[11px] text-muted-foreground">{disbursementLines.length} · {m.finance_ui_disbursement_save()}</p>
                </div>
                <Button
                  type="button"
                  variant="outline"
                  size="sm"
                  class="h-8 gap-1.5 text-xs"
                  onclick={addDisbursementLine}
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
                      {m.finance_ui_disbursement_save()} #{line.sequence}
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
                      <Label class="mb-1 block text-xs font-semibold text-foreground">{m.finance_ui_disbursement_to_account()}</Label>
                      <AccountSearchInput
                        value={line.beneficiaryAccountId}
                        accounts={disbursementAccounts}
                        placeholder={m.finance_ui_expense_account_select_placeholder()}
                        onchange={(value) => (line.beneficiaryAccountId = value ?? "")}
                        disabled={isSubmitting}
                      />
                    </div>
                    <div class="min-w-0">
                      <Label class="mb-1 block text-xs font-semibold text-foreground">{m.finance_ui_disbursed_amount()}</Label>
                      <Input
                        type="number"
                        min="0.01"
                        step="any"
                        bind:value={line.amount}
                        placeholder="0.00"
                        class="h-9 text-xs font-mono font-bold text-rose-600 dark:text-rose-400"
                        disabled={isSubmitting}
                      />
                    </div>
                  </div>
                  <div class="mt-3">
                    <Label class="mb-1 block text-xs font-semibold text-foreground">{m.finance_ui_disbursement_notes()}</Label>
                    <Input
                      type="text"
                      bind:value={line.notes}
                      placeholder={m.finance_ui_disbursement_notes_placeholder()}
                      class="h-9 text-xs"
                      disabled={isSubmitting}
                    />
                  </div>
                </div>
              {/each}
            </section>

            <div class="flex flex-col-reverse gap-3 border-t border-border/40 pt-3 sm:flex-row sm:items-center sm:justify-between">
              <div class="text-sm font-semibold text-foreground">
                {m.finance_ui_disbursed_amount()}:
                <span class="font-mono text-rose-600 dark:text-rose-400">{formatCurrency(calculatedTotal, selectedCurrency)}</span>
              </div>
              <div class="flex flex-wrap items-center justify-end gap-2">
                <Button variant="ghost" size="sm" class="h-9 text-xs" onclick={clearForm} disabled={isSubmitting}>
                  {m.finance_ui_clear_fields()}
                </Button>
                <Button
                  variant="default"
                  size="sm"
                  class="h-9 text-xs font-bold"
                  disabled={isSubmitting}
                  onclick={handleSaveDisbursement}
                >
                  {isSubmitting ? m.finance_ui_saving() : `${m.finance_ui_disbursement_save()} (${disbursementLines.length})`}
                </Button>
              </div>
            </div>
          </CardContent>
        </Card>
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
        emptyTitle={m.finance_ui_no_disbursements()}
      />
    </div>
  </div>
</div>

<PdfReportViewer
  bind:open={printDialogOpen}
  reportTemplateId="disbursement-voucher"
  parameters={printParameters}
  title={printTitle}
/>
