<!--
  @component PaymentsManager
  Minimal, elegant, and high-performance Receipt Vouchers (سندات القبض) workspace.
  Adheres strictly to Orbis Clean Architecture and the FinanceOverview design system:
  - Clean card-based payment voucher entry
  - Direct receipt allocation to accounting accounts
  - Sleek top toolbar with search, method filter, and export
  - High-performance OrbisGrid table
  - Real backend payment recording and printable receipts
-->
<script lang="ts">
import { LoadingIndicator } from "@orbis/ui";
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import { createInfiniteQuery } from "@tanstack/svelte-query";
import { debounce } from "$lib/shared/utils/debounce";
import {
  api,
  extractErrorMessage,
  queryClient,
  queryKeys,
  type AccountDto,
  type PaymentDto,
} from "$lib/api";
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
import {
  Plus,
  Refresh,
  Search,
  Download,
  XMark,
  Printer,
  CheckCircle,
} from "@orbis/icons";
import {
  OrbisGrid,
  type DataQuery,
  type OrbisColumnDef,
  type SortSpec,
} from "@orbis/svelte-grid";
import AccountSearchInput from "$lib/features/finance/components/account-search-input.svelte";
import FinanceInlineFormCard from "$lib/features/finance/components/finance-inline-form-card.svelte";
import CurrencySelector from "$lib/shared/components/currency-selector.svelte";
import PdfReportViewer from "$lib/shared/components/pdf-report-viewer.svelte";
import { formatCurrency, formatDate, isAr } from "$lib/core/i18n";
import { fade, slide } from "svelte/transition";

import {
  createAvailablePaymentMethodsQuery,
} from "$lib/features/finance";
import { isCashOrBankAccount } from "./queries";

let { orgId }: { orgId: string } = $props();

type ReceiptLine = {
  id: number;
  sequence: number;
  accountId: string;
  amount: number | null;
  notes: string;
};

let nextReceiptLineId = 1;
function createReceiptLine(): ReceiptLine {
  const id = nextReceiptLineId++;
  return {
    id,
    sequence: id,
    accountId: "",
    amount: null,
    notes: "",
  };
}

function getLocalDateInputValue(date = new Date()) {
  return new Date(date.getTime() - date.getTimezoneOffset() * 60_000).toISOString().slice(0, 10);
}

// Dynamic payment methods from Integration Layer
const paymentMethodsQuery = createAvailablePaymentMethodsQuery(() => orgId);
const availablePaymentMethods = $derived(paymentMethodsQuery.data ?? []);

// ── State ──────────────────────────────────────────────────────────────
let accounts = $state<AccountDto[]>([]);
let treasuries = $state<AccountDto[]>([]);
let students = $state<any[]>([]);
let isLoading = $state(true);
let isSubmitting = $state(false);
let showAddForm = $state(false);

// Form Fields
let receiptNumber = $state("");
let paymentDate = $state(getLocalDateInputValue());
let selectedTreasuryId = $state("");
let paymentMethod = $state("cash");
let sourceToken = $state("");
let receiptLines = $state<ReceiptLine[]>([createReceiptLine()]);
const selectedCurrency = $derived(
  treasuries.find((account) => String(account.id) === String(selectedTreasuryId))?.currency || "YER",
);
const availableCurrencies = $derived(
  [...new Set(treasuries.map((account) => account.currency).filter((currency) => Boolean(currency)))],
);
const receivableAccounts = $derived(
  accounts.filter((account) =>
    account.is_active &&
    !isCashOrBankAccount(account) &&
    String(account.currency).toUpperCase() === String(selectedCurrency).toUpperCase()
  ),
);
const calculatedTotal = $derived(
  receiptLines.reduce((total, line) => total + (Number(line.amount) || 0), 0),
);

const selectedPaymentMethodObj = $derived(
  availablePaymentMethods.find((m) => m.id === paymentMethod) || availablePaymentMethods[0]
);

// Filters
let searchQuery = $state("");
let searchInput = $state("");
let methodFilter = $state("all");
let filterDate = $state(getLocalDateInputValue());
let gridSort = $state<SortSpec[]>([]);
const updatePaymentSearch = debounce((value: string) => {
  searchQuery = value;
}, 250);
const paymentPageSort = $derived.by(() => {
  const sort = gridSort[0];
  const fieldByColumn: Record<string, string> = {
    receipt_display: "receipt_number",
    date_display: "paid_at",
    amount_formatted: "amount",
    currency_display: "currency_code",
    method_display: "method",
  };
  const sortBy = sort ? fieldByColumn[sort.id] : undefined;
  return sortBy ? { sort_by: sortBy, sort_dir: sort!.dir } : {};
});
function handlePaymentGridQuery(query: DataQuery) {
  gridSort = query.sorts;
}

const paymentPageFilter = $derived({
  method: methodFilter === "all" ? undefined : methodFilter,
  from_date: filterDate || undefined,
  to_date: filterDate || undefined,
});
const paymentsQuery = createInfiniteQuery(() => ({
  queryKey: queryKeys.finance.payments(orgId, {
    ...paymentPageFilter,
    search: searchQuery.trim() || undefined,
    ...paymentPageSort,
  }),
  queryFn: ({ pageParam }) =>
    api.listPaymentsPaginated({
      ...paymentPageFilter,
      search: searchQuery.trim() || undefined,
      ...paymentPageSort,
      page: pageParam,
      page_size: 50,
    }),
  initialPageParam: 1,
  getNextPageParam: (lastPage) =>
    lastPage.page < lastPage.total_pages ? lastPage.page + 1 : undefined,
  enabled: !!orgId,
  staleTime: 2 * 60 * 1000,
}));
const payments = $derived(
  paymentsQuery.data?.pages.flatMap((page) => page.items) ?? [],
);
const gridLoading = $derived(
  isLoading || (paymentsQuery.isLoading && !paymentsQuery.data),
);
async function loadMorePayments() {
  const result = await paymentsQuery.fetchNextPage();
  if (result.isError) throw result.error;
}

// Print
let printDialogOpen = $state(false);
let printParameters = $state<Record<string, any>>({});
let printTitle = $state(m.finance_ui_print_receipt());

onMount(async () => {
  await loadData(false);
});

async function loadData(refreshPayments = true) {
  isLoading = true;
  try {
    const [accountsList, enrollmentsList] = await Promise.all([
      api.listAccounts().catch(() => []),
      api.listEnrollments ? api.listEnrollments().catch(() => []) : Promise.resolve([]),
    ]);

    accounts = accountsList || [];
    treasuries = (accountsList || []).filter(isCashOrBankAccount);

    students = (enrollmentsList || []).map((e: any) => ({
      id: String(e.id),
      full_name: e.full_name || e.name_ar || e.name_en || e.student_number || m.finance_ui_student_number_fallback({ id: e.id }),
      student_number: e.student_number || "",
    }));

    if (treasuries.length > 0 && !selectedTreasuryId) {
      selectedTreasuryId = treasuries[0].id;
    }
    if (refreshPayments) {
      await queryClient.invalidateQueries({
        queryKey: queryKeys.finance.payments(orgId),
      });
    }
  } catch (e) {
    console.error("Failed to load payments data:", e);
    toast.error(m.finance_ui_payments_load_failed());
  } finally {
    isLoading = false;
  }
}

function handleCurrencyChange(currency: string | undefined) {
  if (!currency || currency === selectedCurrency) return;
  const matchingTreasury = treasuries.find((account) => account.currency === currency);
  if (matchingTreasury) handleTreasuryChange(String(matchingTreasury.id));
}

function handleTreasuryChange(accountId: string) {
  selectedTreasuryId = accountId;
  const currency = treasuries.find((account) => String(account.id) === accountId)?.currency;
  for (const line of receiptLines) {
    const receivableAccount = accounts.find((account) => String(account.id) === line.accountId);
    if (
      receivableAccount &&
      currency &&
      String(receivableAccount.currency).toUpperCase() !== String(currency).toUpperCase()
    ) {
      line.accountId = "";
    }
  }
}

function addReceiptLine() {
  const line = createReceiptLine();
  receiptLines = [...receiptLines, line];
  return line.id;
}

function addReceiptLineAndFocus() {
  const lineId = addReceiptLine();
  requestAnimationFrame(() => {
    const accountInput = document.getElementById(`receipt-line-account-${lineId}`);
    if (accountInput instanceof HTMLInputElement) accountInput.focus();
  });
}

function handleQuickAddReceiptLine(event: KeyboardEvent) {
  if (event.key !== "Enter" || !event.shiftKey || isSubmitting) return;
  event.preventDefault();
  event.stopPropagation();
  addReceiptLineAndFocus();
}

function removeReceiptLine(lineId: number) {
  if (receiptLines.length === 1) {
    receiptLines = [createReceiptLine()];
    return;
  }
  receiptLines = receiptLines.filter((line) => line.id !== lineId);
}

function clearForm() {
  receiptNumber = "";
  paymentDate = getLocalDateInputValue();
  sourceToken = "";
  receiptLines = [createReceiptLine()];
}

async function handleSavePayment() {
  if (receiptLines.some((line) => !line.accountId)) {
    toast.error(m.finance_ui_credit_account_required());
    return;
  }
  if (receiptLines.some((line) => !receivableAccounts.some((account) => String(account.id) === line.accountId))) {
    toast.error(m.finance_ui_credit_account_required());
    return;
  }
  if (!selectedTreasuryId) {
    toast.error(m.finance_ui_treasury_required());
    return;
  }

  if (receiptLines.some((line) => !line.amount || Number(line.amount) <= 0)) {
    toast.error(m.finance_ui_payment_amount_invalid());
    return;
  }

  if (selectedPaymentMethodObj?.requires_source_token && !sourceToken.trim()) {
    toast.error(m.finance_ui_payment_gateway_token_required());
    return;
  }

  isSubmitting = true;
  let savedCount = 0;
  let failure: unknown;
  const batchSize = receiptLines.length;
  try {
    for (const line of [...receiptLines]) {
    const selectedAccount = accounts.find((account) => String(account.id) === line.accountId);
      const baseReceiptNumber = receiptNumber.trim();
      const lineReceiptNumber = baseReceiptNumber
        ? batchSize > 1 || line.sequence > 1
          ? `${baseReceiptNumber}-${String(line.sequence).padStart(2, "0")}`
          : baseReceiptNumber
        : undefined;

      try {
        await api.processPayment({
          receivable_account_id: line.accountId,
          account_id: selectedTreasuryId,
          amount: String(line.amount),
          method: paymentMethod,
          source: sourceToken.trim() || undefined,
          payer_name: selectedAccount?.name_ar || selectedAccount?.name || undefined,
          receipt_number: lineReceiptNumber,
          payment_date: paymentDate || undefined,
          notes: line.notes.trim() || undefined,
        });
        savedCount += 1;
        receiptLines = receiptLines.filter((item) => item.id !== line.id);
      } catch (error) {
        failure = error;
        break;
      }
    }

    if (savedCount > 0) {
      toast.success(m.finance_ui_receipt_posted_success());
      await loadData();
    }
    if (failure) {
      console.error("Failed to record a receipt in the batch:", failure);
      toast.error(extractErrorMessage(failure) || m.finance_ui_receipt_save_failed());
    } else {
      clearForm();
      showAddForm = false;
    }
  } finally {
    isSubmitting = false;
  }
}

function handlePrint(p: any) {
  printParameters = {
    payment_id: p.id,
    receipt_number: p.receipt_number || p.id.slice(0, 8),
    org_id: orgId,
  };
  printTitle = m.finance_ui_receipt_print_title({ number: p.receipt_number || p.id.slice(0, 8) });
  printDialogOpen = true;
}

// ── Grid Data ─────────────────────────────────────────────────────────
function toPaymentGridRow(p: PaymentDto, idx: number) {
    const student = students.find((s) => String(s.id) === String(p.enrollment_id));
    const accountName = p.payer_name || p.student_name || student?.full_name || "—";
    const treasury = treasuries.find((t) => String(t.id) === String(p.account_id || p.cash_account_id));
    const treasuryName = treasury ? (treasury.name_ar || treasury.name || m.finance_ui_treasury()) : m.finance_ui_main_treasury();
    const amt = Number(p.amount || 0);
    const curr = p.currency_code || p.currency || "YER";

    return {
      ...p,
      payment_method: p.payment_method || p.method,
      __idx: idx + 1,
      receipt_display: p.receipt_number || p.id.slice(0, 8),
      date_display: p.paid_at ? formatDate(p.paid_at) : (p.payment_date ? formatDate(p.payment_date) : (p.created_at ? formatDate(p.created_at) : "—")),
      counter_account_display: accountName,
      treasury_display: treasuryName,
      amount_formatted: formatCurrency(amt, curr),
      currency_display: curr,
      method_display: (p.method || p.payment_method) === "bank_transfer" || p.payment_method === "Bank"
        ? m.finance_ui_payment_method_bank_transfer()
        : (p.method === "check" || p.payment_method === "Check"
          ? m.finance_ui_payment_method_check()
          : m.finance_ui_payment_method_cash()),
      notes_display: p.notes || "—",
      status_display: m.finance_ui_receipt_status(),
    };
}

const gridSource = $derived(payments.map(toPaymentGridRow));

const filteredSource = $derived(gridSource);

const columns: OrbisColumnDef[] = [
  { id: "__idx", prop: "__idx", header: "#", name: "#", width: 48, size: 48, readonly: true, pin: "left", align: "center" },
  { id: "receipt_display", prop: "receipt_display", header: m.finance_ui_voucher_number(), name: m.finance_ui_voucher_number(), width: 110, size: 110, readonly: true, sortable: true, cellClass: "font-mono font-bold" },
  { id: "date_display", prop: "date_display", header: m.finance_ui_date(), name: m.finance_ui_date(), width: 110, size: 110, readonly: true, sortable: true, align: "center" },
  { id: "counter_account_display", prop: "counter_account_display", header: m.finance_ui_credit_account(), name: m.finance_ui_credit_account(), width: 220, size: 220, readonly: true, cellClass: "font-medium" },
  { id: "treasury_display", prop: "treasury_display", header: m.finance_ui_receiving_account(), name: m.finance_ui_receiving_account(), width: 180, size: 180, readonly: true },
  { id: "amount_formatted", prop: "amount_formatted", header: m.finance_ui_received_amount(), name: m.finance_ui_received_amount(), width: 140, size: 140, readonly: true, sortable: true, align: "end", cellClass: "font-mono font-bold text-emerald-600 dark:text-emerald-400" },
  { id: "currency_display", prop: "currency_display", header: m.finance_ui_currency(), name: m.finance_ui_currency(), width: 80, size: 80, readonly: true, sortable: true, align: "center" },
  { id: "method_display", prop: "method_display", header: m.finance_ui_payment_method(), name: m.finance_ui_payment_method(), width: 110, size: 110, readonly: true, sortable: true, align: "center" },
  { id: "notes_display", prop: "notes_display", header: m.finance_ui_description_notes(), name: m.finance_ui_description_notes(), width: 220, size: 220, readonly: true },
  { id: "status_display", prop: "status_display", header: m.finance_ui_status(), name: m.finance_ui_status(), width: 90, size: 90, type: "badge", align: "center", readonly: true, sortable: false },
];

async function exportCsv() {
  try {
    const allPayments = await api.listPayments({
      ...paymentPageFilter,
      search: searchQuery.trim() || undefined,
    });
    const rows = allPayments.map(toPaymentGridRow).map((payment) => [
      payment.receipt_display,
      payment.date_display,
      payment.counter_account_display,
      payment.treasury_display,
      payment.amount_formatted,
      payment.currency_display,
      payment.method_display,
      payment.notes_display,
      payment.status_display,
    ]);
    const csv = [
      [m.finance_ui_receipt(), m.finance_ui_date(), m.finance_ui_credit_account(), m.finance_ui_treasury(), m.finance_ui_amount(), m.finance_ui_currency(), m.finance_ui_payment_method(), m.finance_ui_notes(), m.finance_ui_status()],
      ...rows,
    ]
      .map((row) => row.map((v) => `"${String(v ?? "").replaceAll('"', '""')}"`).join(","))
      .join("\n");
    const url = URL.createObjectURL(new Blob(["\ufeff" + csv], { type: "text/csv;charset=utf-8" }));
    const link = document.createElement("a");
    link.href = url;
    link.download = `payments-${orgId}.csv`;
    link.click();
    URL.revokeObjectURL(url);
  } catch (error) {
    toast.error(extractErrorMessage(error));
  }
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
            <span>{m.finance_ui_new_receipt()}</span>
          {/if}
        </Button>
      </div>

      <div class="flex items-center gap-2">
        <div class="relative w-48 sm:w-64">
          <Search size={14} class="absolute inset-s-2.5 top-1/2 -translate-y-1/2 text-muted-foreground pointer-events-none" />
          <Input
            type="text"
            placeholder={m.finance_ui_receipt_search_placeholder()}
            bind:value={searchInput}
            oninput={() => updatePaymentSearch(searchInput)}
            class="h-8 ps-8 text-xs bg-card border-border/60"
          />
        </div>

        <Select
          type="single"
          value={methodFilter}
          onValueChange={(value) => (methodFilter = value ?? "all")}
        >
          <SelectTrigger class="h-8 w-auto min-w-36 bg-card text-xs">
            {methodFilter === "all"
              ? m.finance_ui_all_payment_methods()
              : availablePaymentMethods.find((method) => method.id === methodFilter)?.name}
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="all">{m.finance_ui_all_payment_methods()}</SelectItem>
          {#each availablePaymentMethods as m (m.id)}
              <SelectItem value={m.id}>{m.name}</SelectItem>
          {/each}
          </SelectContent>
        </Select>

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
          title={m.finance_ui_receipt_create_heading()}
          accent="emerald"
          contentClass="gap-5"
          onclose={() => (showAddForm = false)}
        >
            <div class="grid min-w-0 grid-cols-1 gap-4 sm:grid-cols-2 xl:grid-cols-5">
              <div class="min-w-0">
                <Label class="mb-1 block text-xs font-semibold text-foreground">{m.finance_ui_voucher_number_optional()}</Label>
                <Input
                  type="text"
                  bind:value={receiptNumber}
                  placeholder={m.finance_ui_automatic()}
                  class="h-9 text-xs font-mono"
                  disabled={isSubmitting}
                />
              </div>
              <div class="min-w-0">
                <Label class="mb-1 block text-xs font-semibold text-foreground">{m.finance_ui_date()}</Label>
                <Input
                  type="date"
                  bind:value={paymentDate}
                  class="h-9 text-xs font-mono"
                  disabled={isSubmitting}
                />
              </div>
              <div class="min-w-0">
                <Label class="mb-1 block text-xs font-semibold text-foreground">{m.finance_ui_receiving_account()}</Label>
                <AccountSearchInput
                  value={selectedTreasuryId}
                  placeholder={m.finance_ui_treasury_select_placeholder()}
                  accounts={treasuries}
                  onchange={(value) => handleTreasuryChange(value ?? "")}
                  disabled={isSubmitting}
                  class="h-9 w-full text-xs"
                />
              </div>
              <div class="min-w-0">
                <Label class="mb-1 block text-xs font-semibold text-foreground">{m.finance_ui_payment_method()}</Label>
                <Select
                  type="single"
                  value={paymentMethod}
                  onValueChange={(value) => (paymentMethod = value ?? "cash")}
                  disabled={isSubmitting}
                >
                  <SelectTrigger class="h-9 w-full bg-background text-xs">
                    {selectedPaymentMethodObj?.name ?? m.finance_ui_payment_method()}
                  </SelectTrigger>
                  <SelectContent>
                    {#each availablePaymentMethods as method (method.id)}
                      <SelectItem value={method.id}>
                        {method.name}{method.is_digital_gateway ? ` (${method.provider_name ?? ""})` : ""}
                      </SelectItem>
                    {/each}
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
                  disabled={isSubmitting || availableCurrencies.length < 2}
                />
              </div>
            </div>
            {#if selectedPaymentMethodObj?.requires_source_token}
              <div class="min-w-0">
                <Label class="mb-1 block text-xs font-semibold text-foreground">{m.finance_ui_payment_gateway_token_label()}</Label>
                <Input
                  type="text"
                  bind:value={sourceToken}
                  placeholder={m.finance_ui_payment_gateway_token_placeholder()}
                  class="h-9 text-xs font-mono"
                  disabled={isSubmitting}
                />
              </div>
            {/if}
            <section class="flex flex-col gap-3">
              <div class="flex flex-wrap items-center justify-between gap-2">
                <div>
                  <h3 class="text-sm font-semibold text-foreground">{m.finance_ui_allocate_student_fees()}</h3>
                </div>
                <Button
                  type="button"
                  variant="outline"
                  size="sm"
                  class="h-8 gap-1.5 text-xs"
                  onclick={addReceiptLineAndFocus}
                  disabled={isSubmitting}
                >
                  <Plus size={14} />
                  {m.common_action_add()}
                </Button>
              </div>

              {#each receiptLines as line (line.id)}
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
                      onclick={() => removeReceiptLine(line.id)}
                      disabled={isSubmitting && receiptLines.length === 1}
                      aria-label={m.common_action_delete()}
                    >
                      <XMark size={14} />
                    </Button>
                  </div>

                  <div class="grid min-w-0 grid-cols-1 gap-3 sm:grid-cols-2 xl:grid-cols-[minmax(0,1.5fr)_minmax(8rem,0.7fr)]">
                      <div class="min-w-0">
                        <Label class="mb-1 block text-xs font-semibold text-foreground">{m.finance_ui_credit_account()}</Label>
                        <AccountSearchInput
                          id={`receipt-line-account-${line.id}`}
                          value={line.accountId}
                          placeholder={m.finance_ui_credit_account_select_placeholder()}
                          accounts={receivableAccounts}
                          onchange={(value) => (line.accountId = value ?? "")}
                          onkeydown={handleQuickAddReceiptLine}
                          disabled={isSubmitting}
                          class="h-9 w-full text-xs"
                        />
                      </div>
                      <div class="min-w-0">
                      <Label class="mb-1 block text-xs font-semibold text-foreground">{m.finance_ui_total_received_amount()}</Label>
                      <Input
                        type="number"
                        min="0.01"
                        step="any"
                        bind:value={line.amount}
                        onkeydown={handleQuickAddReceiptLine}
                        placeholder="0.00"
                        class="h-9 text-xs font-mono font-bold text-emerald-600 dark:text-emerald-400"
                        disabled={isSubmitting}
                      />
                    </div>
                  </div>
                  <div class="mt-3">
                    <Label class="mb-1 block text-xs font-semibold text-foreground">{m.finance_ui_description_notes()}</Label>
                    <Input
                      type="text"
                      bind:value={line.notes}
                      onkeydown={handleQuickAddReceiptLine}
                      placeholder={m.finance_ui_receipt_notes_placeholder()}
                      class="h-9 text-xs"
                      disabled={isSubmitting}
                    />
                  </div>
                </div>
              {/each}
            </section>

            <div class="flex flex-col-reverse gap-3 border-t border-border/40 pt-3 sm:flex-row sm:items-center sm:justify-between">
              <div class="text-sm font-semibold text-foreground">
                {m.finance_ui_total_received_amount()}:
                <span class="font-mono text-emerald-600 dark:text-emerald-400">
                  {formatCurrency(calculatedTotal, selectedCurrency)}
                </span>
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
                  onclick={handleSavePayment}
                >
                  {isSubmitting ? m.finance_ui_saving() : `${m.finance_ui_receipt_save()} (${receiptLines.length})`}
                </Button>
              </div>
            </div>
        </FinanceInlineFormCard>
      </div>
    {/if}

    <!-- Grid Container -->
    <div class="flex-1 min-h-0 overflow-hidden">
      <OrbisGrid
        columns={columns}
        source={filteredSource}
        loading={gridLoading}
        serverQuery
        rtl={isAr()}
        search={searchQuery}
        onquerychange={handlePaymentGridQuery}
        readonly
        emptyTitle={m.finance_ui_no_receipts()}
        hasMore={paymentsQuery.hasNextPage ?? false}
        onloadmore={loadMorePayments}
      />
    </div>
  </div>
</div>

<PdfReportViewer
  bind:open={printDialogOpen}
  templateId="payment-receipt"
  parameters={printParameters}
  reportTitle={printTitle}
/>
