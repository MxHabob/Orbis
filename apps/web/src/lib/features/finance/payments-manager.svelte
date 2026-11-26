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
import JournalLineDetails from "./components/journal-line-details.svelte";
import VoucherMetadata from "./components/voucher-metadata.svelte";
import { authStore } from "$lib/stores";
import { createCashierAssignmentQuery } from "./queries";
import { eligibleTreasuries, resolveTreasury } from "./treasury-policy";
import { LoadingIndicator } from "@orbis/ui";
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import { createInfiniteQuery, createQuery } from "@tanstack/svelte-query";
import {
  api,
  extractErrorMessage,
  queryClient,
  queryKeys,
  type AccountDto,
  type EnrollmentDto,
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
  Download,
  XMark,
  Printer,
  CheckCircle,
} from "@orbis/icons";
import {
  OrbisGrid,
  type GridActionEvent,
  type DataQuery,
  type OrbisColumnDef,
  type SelectionSnapshot,
  type SortSpec,
  selectionColumn,
} from "@orbis/svelte-grid";
import AccountSearchInput from "$lib/features/finance/components/account-search-input.svelte";
import { tick } from "svelte";
import { financeRecordColumn, paymentMethodLabel, receiptPrintContext } from "./record-presentation";
import FinanceRecordDialog from "./components/finance-record-dialog.svelte";
import FinanceInlineFormCard from "$lib/features/finance/components/finance-inline-form-card.svelte";
import DateRangeFilter from "$lib/shared/components/date-range-filter.svelte";
import { createDefaultDateRange } from "$lib/shared/components/date-range";
import CurrencySelector from "$lib/shared/components/currency-selector.svelte";
import PrintDialog from "$lib/shared/components/print-dialog.svelte";
import { formatDateKey } from "$lib/core/i18n/format";
import { formatCurrency, formatDate, isAr } from "$lib/core/i18n";
import { orgContext } from "$lib/stores/org-context.svelte";
import type { CurrencyDto } from "$lib/api/types";

import { paymentRequestFingerprint } from "./payment-attempt-key";
import PaymentMethodField from "./components/payment-method-field.svelte";
import { createAvailablePaymentMethodsQuery, manualPaymentMethods } from "./queries";
import { accountQueryOptions, isCashOrBankAccount, refreshPaymentData } from "./queries";

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


// Dynamic payment methods from Integration Layer
const paymentMethodsQuery = createAvailablePaymentMethodsQuery(() => orgId);
const availablePaymentMethods = $derived(paymentMethodsQuery.data ?? manualPaymentMethods());

// ── State ──────────────────────────────────────────────────────────────
let accounts = $state<AccountDto[]>([]);
let treasuries = $state<AccountDto[]>([]);
let currencies = $state<CurrencyDto[]>([]);
let students = $state<EnrollmentDto[]>([]);
let isLoading = $state(true);
let isSubmitting = $state(false);
let formError = $state("");
let showAddForm = $state(false);

// Form Fields
let receiptNumber = $state("");
let paymentDate = $state(formatDateKey(new Date()));
let selectedTreasuryId = $state("");
let selectedCurrency = $state(orgContext.baseCurrency || "");
let paymentMethod = $state("cash");
let sourceToken = $state("");
let methodReference = $state("");
const showsMethodReference = $derived(paymentMethod === "bank_transfer" || paymentMethod === "check");
let receiptLines = $state<ReceiptLine[]>([createReceiptLine()]);
const paymentAttemptKeys = new Map<string, string>();
const availableCurrencies = $derived([
  ...currencies,
  ...Array.from(
    new Set(
      treasuries
        .map((account) => account.currency)
        .filter((currency) => Boolean(currency))
        .filter(
          (currency) =>
            !currencies.some(
              (option) => option.code.toUpperCase() === currency.toUpperCase(),
            ),
        ),
    ),
  ).map(
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
const settingsQuery = createCashierAssignmentQuery(() => orgId);
const cashAssignment = $derived(settingsQuery.data && !settingsQuery.isError
  ? settingsQuery.data[authStore.userId ?? ""] ?? null
  : undefined);
const methodTreasuries = $derived(eligibleTreasuries(treasuries, paymentMethod, cashAssignment));
const treasuryAccountsForCurrency = $derived(methodTreasuries.filter((account) => account.currency.toUpperCase() === selectedCurrency.toUpperCase()));
const activeTreasuryId = $derived(resolveTreasury(treasuryAccountsForCurrency, selectedTreasuryId));

let initialTreasuryResolved = $state(false);
$effect(() => {
  if (initialTreasuryResolved || cashAssignment === undefined || !treasuries.length) return;
  initialTreasuryResolved = true;
  const initialId = activeTreasuryId || methodTreasuries[0]?.id;
  if (initialId) handleTreasuryChange(initialId);
});
const receivableAccounts = $derived(
  accounts.filter(
    (account) =>
      account.is_active &&
      !isCashOrBankAccount(account) &&
      String(account.currency).toUpperCase() ===
        String(selectedCurrency).toUpperCase(),
  ),
);
const calculatedTotal = $derived(
  receiptLines.reduce((total, line) => total + (Number(line.amount) || 0), 0),
);

const selectedPaymentMethodObj = $derived(
  availablePaymentMethods.find((method) => method.id === paymentMethod),
);

// Filters
let searchQuery = $state("");
let methodFilter = $state("all");
let dateRange = $state(createDefaultDateRange());
let gridSort = $state<SortSpec[]>([]);
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
  gridSort = query.sorts ?? [];
  searchQuery = query.search ?? "";
}

const paymentPageFilter = $derived({
  method: methodFilter === "all" ? undefined : methodFilter,
  from_date: dateRange.fromDate || undefined,
  to_date: dateRange.toDate || undefined,
});
const paymentsQuery = createInfiniteQuery(() => {
  const filters = { ...paymentPageFilter, search: searchQuery.trim() || undefined, ...paymentPageSort };
  return {
    queryKey: [...queryKeys.finance.payments(orgId, filters), "paginated"],
    queryFn: ({ pageParam }) => api.listPaymentsPaginated({ ...filters, page: pageParam, page_size: 50 }),
    initialPageParam: 1,
    getNextPageParam: (lastPage) =>
      lastPage.page < lastPage.total_pages ? lastPage.page + 1 : undefined,
    enabled: Boolean(orgId),
    staleTime: 2 * 60 * 1000,
  };
});
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
let paymentSelection = $state<SelectionSnapshot>({
  mode: "explicit",
  ids: new Set(),
  range: null,
  active: null,
});

onMount(async () => {
  await loadData(false);
});

async function loadData(refreshPayments = true) {
  isLoading = true;
  try {
    const [accountsList, enrollmentsList, currenciesList] = await Promise.all([
      queryClient.fetchQuery(accountQueryOptions(orgId)),
      api.listEnrollments
        ? api.listEnrollments().catch(() => [])
        : Promise.resolve([]),
      queryClient.fetchQuery({
        queryKey: queryKeys.finance.currencies(),
        queryFn: () => api.listCurrencies(),
        staleTime: 10 * 60 * 1000,
      }),
    ]);

    accounts = accountsList || [];
    treasuries = (accountsList || []).filter(isCashOrBankAccount);
    currencies = currenciesList || [];

    students = enrollmentsList || [];

    if (refreshPayments) {
      await queryClient.invalidateQueries({
        queryKey: queryKeys.finance.paymentsForOrg(orgId),
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
  selectedCurrency = currency.toUpperCase();
  const matchingTreasury = methodTreasuries.find(
    (account) =>
      String(account.currency).toUpperCase() === String(currency).toUpperCase(),
  );
  if (matchingTreasury) handleTreasuryChange(String(matchingTreasury.id));
  else {
    selectedTreasuryId = "";
    for (const line of receiptLines) line.accountId = "";
  }
}

function handleTreasuryChange(accountId: string) {
  selectedTreasuryId = accountId;
  const currency = treasuries.find(
    (account) => String(account.id) === accountId,
  )?.currency;
  if (currency) selectedCurrency = currency.toUpperCase();
  for (const line of receiptLines) {
    const receivableAccount = accounts.find(
      (account) => String(account.id) === line.accountId,
    );
    if (
      receivableAccount &&
      currency &&
      String(receivableAccount.currency).toUpperCase() !==
        String(currency).toUpperCase()
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
  if (isSubmitting || selectedPaymentMethodObj?.is_digital_gateway) return;
  const lineId = addReceiptLine();
  requestAnimationFrame(() => {
    const accountInput = document.getElementById(
      `receipt-line-account-${lineId}`,
    );
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
  paymentDate = formatDateKey(new Date());
  sourceToken = "";
  methodReference = "";
  paymentAttemptKeys.clear();
  receiptLines = [createReceiptLine()];
  formError = "";
}

async function getPaymentAttemptKey(request: Parameters<typeof api.processPayment>[0]) {
  const fingerprint = await paymentRequestFingerprint(request);
  const existing = paymentAttemptKeys.get(fingerprint);
  if (existing) return existing;
  const key = crypto.randomUUID();
  paymentAttemptKeys.set(fingerprint, key);
  return key;
}

const receivableAccountIds = $derived(new Set(receivableAccounts.map((account) => String(account.id))));
const accountsById = $derived(new Map(accounts.map((account) => [String(account.id), account])));
const enrollmentsById = $derived(new Map(students.map((item) => [item.id, item])));
const enrollmentByStudentId = $derived.by(() => {
  const index = new Map<string, EnrollmentDto>();
  for (const item of students) {
    if (!index.has(item.student_id)) index.set(item.student_id, item);
  }
  return index;
});

async function handleSavePayment() {
  if (isSubmitting) return;
  formError = "";
  if (receiptLines.some((line) => !line.accountId)) {
    formError = m.finance_ui_credit_account_required();
    return;
  }
  if (receiptLines.some((line) => !receivableAccountIds.has(line.accountId))) {
    formError = m.finance_ui_credit_account_required();
    return;
  }
  if (!activeTreasuryId) {
    formError = m.finance_ui_treasury_required();
    return;
  }

  if (receiptLines.some((line) => line.amount == null || !Number.isFinite(line.amount) || line.amount <= 0)) {
    formError = m.finance_ui_payment_amount_invalid();
    return;
  }

  if (!selectedPaymentMethodObj || (paymentMethodsQuery.isError && selectedPaymentMethodObj.is_digital_gateway)) {
    formError = m.finance_ui_payment_method_unavailable();
    return;
  }
  if (selectedPaymentMethodObj.is_digital_gateway && receiptLines.length !== 1) {
    formError = m.finance_ui_payment_gateway_single_line();
    return;
  }
  if (selectedPaymentMethodObj.requires_source_token && !sourceToken.trim()) {
    formError = m.finance_ui_payment_gateway_token_required();
    return;
  }

  isSubmitting = true;
  try {
    const [first, ...additional] = receiptLines.map((line) => ({
      account_id: line.accountId,
      amount: String(line.amount),
      notes: line.notes.trim() || undefined,
    }));
    if (!first) return;
    const payer = additional.length ? undefined : accountsById.get(first.account_id);
    const request = {
      receivable_account_id: first.account_id,
      account_id: activeTreasuryId,
      amount: first.amount,
      additional_allocations: additional,
      method: paymentMethod,
      source: selectedPaymentMethodObj.requires_source_token ? sourceToken.trim() : undefined,
      payer_name: payer?.name_ar || payer?.name || undefined,
      receipt_number: receiptNumber.trim() || undefined,
      payment_date: paymentDate || undefined,
      reference: showsMethodReference ? methodReference.trim() || undefined : undefined,
      notes: first.notes,
    };
    await api.processPayment({ ...request, idempotency_key: await getPaymentAttemptKey(request) });
    toast.success(m.finance_ui_receipt_posted_success());
    void refreshPaymentData(queryClient, orgId);
    clearForm();
    showAddForm = false;
  } catch (error) {
    formError = extractErrorMessage(error) || m.finance_ui_receipt_save_failed();
  } finally {
    isSubmitting = false;
  }
}

// ── Grid Data ─────────────────────────────────────────────────────────
function toPaymentGridRow(p: PaymentDto, idx: number) {
  const enrollment = p.enrollment_id
    ? enrollmentsById.get(p.enrollment_id)
    : p.student_id ? enrollmentByStudentId.get(p.student_id) : undefined;
  const accountName = p.payer_name || enrollment?.student_number || "—";
  const treasury = accountsById.get(String(p.account_id));
  const treasuryName = treasury
    ? treasury.name_ar || treasury.name || m.finance_ui_treasury()
    : m.finance_ui_main_treasury();
  const amt = Number(p.amount || 0);
  const curr = p.currency_code || orgContext.baseCurrency || "";

  return {
    ...p,
    payment_method: p.method,
    __idx: idx + 1,
    receipt_display: p.receipt_number || p.id.slice(0, 8),
    date_display: formatDate(p.paid_at || p.created_at) || "—",
    counter_account_display: accountName,
    treasury_display: treasuryName,
    amount_formatted: formatCurrency(amt, curr),
    currency_display: curr,
    method_display: paymentMethodLabel(p.method),
    reference_display: p.reference || "—",
    status_display: m.finance_ui_receipt_status(),
  };
}

const gridSource = $derived(payments.map(toPaymentGridRow));

const filteredSource = $derived(gridSource);
const selectedPayments = $derived(
  filteredSource.filter((payment) =>
    paymentSelection.mode === "allMatching"
      ? !paymentSelection.ids.has(payment.id)
      : paymentSelection.ids.has(payment.id),
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

const paymentPrintContexts = $derived(
  printRows.map((payment) => receiptPrintContext(payment, payment.counter_account_display)),
);

function handlePaymentSelection(selection: SelectionSnapshot) {
  paymentSelection = selection;
}


const detailsColumn = $derived(financeRecordColumn());
let detailsOpen = $state(false);
let detailRow = $state<(typeof gridSource)[number] | undefined>();
const receiptJournal = createQuery(() => {
  const journalId = detailRow?.journal_entry_id ?? "";
  return {
    queryKey: [...queryKeys.finance.journalEntries(orgId), journalId],
    queryFn: () => api.getJournalEntry(journalId),
    enabled: detailsOpen && Boolean(journalId),
  };
});
function receiptAccountName(accountId: string) {
  const account = accountsById.get(accountId);
  return account?.name_ar || account?.name || accountId;
}

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
    sortable: true,
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
    sortable: true,
    align: "center",
  },
  {
    id: "counter_account_display",
    prop: "counter_account_display",
    header: m.finance_ui_credit_account(),
    name: m.finance_ui_credit_account(),
    width: 220,
    size: 220,
    readonly: true,
    cellClass: "font-medium",
  },
  {
    id: "treasury_display",
    prop: "treasury_display",
    header: m.finance_ui_receiving_account(),
    name: m.finance_ui_receiving_account(),
    width: 180,
    size: 180,
    readonly: true,
  },
  {
    id: "amount_formatted",
    prop: "amount_formatted",
    header: m.finance_ui_received_amount(),
    name: m.finance_ui_received_amount(),
    width: 140,
    size: 140,
    readonly: true,
    sortable: true,
    align: "end",
    cellClass: "font-mono font-bold text-emerald-600 dark:text-emerald-400",
  },
  {
    id: "currency_display",
    prop: "currency_display",
    header: m.finance_ui_currency(),
    name: m.finance_ui_currency(),
    width: 80,
    size: 80,
    readonly: true,
    sortable: true,
    align: "center",
  },
  {
    id: "method_display",
    prop: "method_display",
    header: m.finance_ui_payment_method(),
    name: m.finance_ui_payment_method(),
    width: 110,
    size: 110,
    readonly: true,
    sortable: true,
    align: "center",
  },
  {
    id: "reference_display",
    prop: "reference_display",
    header: m.finance_ui_reference(),
    name: m.finance_ui_reference(),
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
    sortable: false,
  },
];

async function exportCsv() {
  try {
    const allPayments = await api.listPayments({
      ...paymentPageFilter,
      search: searchQuery.trim() || undefined,
    });
    const rows = allPayments
      .map(toPaymentGridRow)
      .map((payment) => [
        payment.receipt_display,
        payment.date_display,
        payment.counter_account_display,
        payment.treasury_display,
        payment.amount_formatted,
        payment.currency_display,
        payment.method_display,
        payment.reference_display,
        payment.status_display,
      ]);
    const csv = [
      [
        m.finance_ui_receipt(),
        m.finance_ui_date(),
        m.finance_ui_credit_account(),
        m.finance_ui_treasury(),
        m.finance_ui_amount(),
        m.finance_ui_currency(),
        m.finance_ui_payment_method(),
        m.finance_ui_reference(),
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
    link.download = `payments-${orgId}.csv`;
    link.click();
    URL.revokeObjectURL(url);
  } catch (error) {
    toast.error(extractErrorMessage(error));
  }
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
            <span>{m.finance_ui_new_receipt()}</span>
          {/if}
        </Button>
        <Button
          variant="outline"
          size="sm"
          class="h-8 px-3 text-xs font-semibold shrink-0 gap-1.5"
          disabled={selectedPayments.length === 0}
          onclick={() => openPrint(selectedPayments)}
        >
          <Printer size={14} />
          <span>{m.finance_ui_print_receipt()}{selectedPayments.length ? ` (${selectedPayments.length})` : ""}</span>
        </Button>
      </div>

      <div class="flex items-center gap-2">
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

        <DateRangeFilter bind:value={dateRange} />

        <Button variant="ghost" size="icon-sm" onclick={() => { void loadData(); }} disabled={isLoading || isSubmitting} aria-label={m.common_control_refresh()} class="text-muted-foreground">
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
          title={m.finance_ui_receipt_create_heading()}
          accent="emerald"
          contentClass="gap-5"
          busy={isSubmitting}
          errorMessage={formError}
          onsave={handleSavePayment}
          onclose={() => { formError = ""; showAddForm = false; }}
        >
          {#snippet metadata()}
            <VoucherMetadata bind:number={receiptNumber} bind:date={paymentDate} dateLabel={m.template_schema_receipt_date()} />
          {/snippet}
            <h3 class="text-sm font-semibold">{m.finance_ui_payment_details()}</h3>
            <div class="grid min-w-0 grid-cols-1 gap-4 @min-[30rem]/workspace:grid-cols-2 @min-[50rem]/workspace:grid-cols-[minmax(9rem,0.7fr)_minmax(0,1.4fr)_minmax(0,1fr)]">
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
              <PaymentMethodField
                methods={availablePaymentMethods}
                value={paymentMethod}
                disabled={isSubmitting}
                loading={paymentMethodsQuery.isFetching}
                loadFailed={paymentMethodsQuery.isError}
                onretry={() => { void paymentMethodsQuery.refetch(); }}
                onchange={(value) => { paymentMethod = value; selectedTreasuryId = ""; sourceToken = ""; methodReference = ""; }}
              />
              <div class="min-w-0">
                <Label for="receipt-receiving-account" class="mb-1 block text-xs font-semibold text-foreground">{paymentMethod === "cash" ? m.finance_ui_cash_treasury() : m.finance_ui_account_type_bank()}</Label>
                <AccountSearchInput inlineResults id="receipt-receiving-account"
                  value={activeTreasuryId}
                  placeholder={treasuryAccountsForCurrency.length ? m.finance_ui_treasury_select_placeholder() : m.finance_ui_no_matching_treasury()}
                  accounts={treasuryAccountsForCurrency}
                  onchange={(value) => handleTreasuryChange(value ?? "")}
                  disabled={isSubmitting}
                  class="h-9 w-full text-xs"
                />
              </div>
            </div>
            {#if showsMethodReference}
              <div class="min-w-0">
                <Label for="receipt-method-reference" class="mb-1 block text-xs font-semibold">
                  {paymentMethod === "check" ? m.finance_ui_check_reference_label() : m.finance_ui_transfer_reference_label()}
                </Label>
                <Input id="receipt-method-reference"
                  bind:value={methodReference}
                  placeholder={paymentMethod === "check" ? m.finance_ui_check_reference_placeholder() : m.finance_ui_transfer_reference_placeholder()}
                  class="h-9 text-xs"
                  disabled={isSubmitting}
                />
              </div>
            {/if}
            {#if selectedPaymentMethodObj?.requires_source_token}
              <div class="min-w-0">
                <Label for="receipt-payment-gateway-token-label" class="mb-1 block text-xs font-semibold text-foreground">{m.finance_ui_payment_gateway_token_label()}</Label>
                <Input id="receipt-payment-gateway-token-label"
                  type="text"
                  autocomplete="off"
                  required
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
                  <h3 class="text-sm font-semibold text-foreground">{m.finance_ui_accounts_amounts()}</h3>
                </div>
                <Button
                  type="button"
                  variant="outline"
                  size="sm"
                  class="h-8 gap-1.5 text-xs"
                  onclick={addReceiptLineAndFocus}
                  disabled={isSubmitting || selectedPaymentMethodObj?.is_digital_gateway}
                >
                  <Plus size={14} />
                  {m.common_action_add()}
                </Button>
              </div>

              {#if selectedPaymentMethodObj?.is_digital_gateway && receiptLines.length > 1}
                <p class="text-sm text-destructive" role="status">{m.finance_ui_payment_gateway_single_line()}</p>
              {/if}
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
                      disabled={isSubmitting}
                      aria-label={m.common_action_delete()}
                    >
                      <XMark size={14} />
                    </Button>
                  </div>

                  <div class="grid min-w-0 items-start gap-3 @min-[36rem]/workspace:grid-cols-[minmax(0,1.5fr)_minmax(8rem,0.65fr)] @min-[56rem]/workspace:grid-cols-[minmax(0,1.5fr)_minmax(8rem,0.65fr)_minmax(0,1fr)]">
                    <div class="min-w-0">
                      <Label for={"receipt-line-account-" + line.id} class="mb-1 block text-xs font-semibold">{m.finance_ui_credit_account()}</Label>
                      <AccountSearchInput
                        inlineResults
                        id={"receipt-line-account-" + line.id}
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
                      <Label for={"receipt-line-amount-" + line.id} class="mb-1 block text-xs font-semibold">{m.finance_ui_amount()}</Label>
                      <Input id={"receipt-line-amount-" + line.id}
                        type="number"
                        min="0.01"
                        step="any"
                        bind:value={line.amount}
                        onkeydown={handleQuickAddReceiptLine}
                        placeholder="0.00"
                        class="h-9 text-xs tabular-nums"
                        disabled={isSubmitting}
                      />
                    </div>
                    <div class="min-w-0 @min-[36rem]/workspace:col-span-2 @min-[56rem]/workspace:col-span-1">
                      <Label for={"receipt-line-notes-" + line.id} class="mb-1 block text-xs font-semibold">{m.finance_ui_receipt_notes_optional()}</Label>
                      <Input id={"receipt-line-notes-" + line.id}
                        bind:value={line.notes}
                        onkeydown={handleQuickAddReceiptLine}
                        placeholder={m.finance_ui_receipt_notes_placeholder()}
                        class="h-9 text-xs"
                        disabled={isSubmitting}
                      />
                    </div>
                  </div>
                </div>
              {/each}
            </section>

          {#snippet actions()}
            <div class="flex flex-col-reverse gap-3 sm:flex-row sm:items-center sm:justify-between">
              <div class="text-sm font-semibold text-foreground">
                {m.finance_ui_total_received_amount()}:
                <span class="font-mono text-emerald-600 dark:text-emerald-400">
                  {formatCurrency(calculatedTotal, selectedCurrency)}
                </span>
              </div>
              <div class="flex flex-wrap items-center justify-end gap-2">
                <Button type="button" variant="ghost" size="sm" class="h-9 text-xs" onclick={clearForm} disabled={isSubmitting}>
                  {m.finance_ui_clear_fields()}
                </Button>
                <Button type="submit"
                  variant="default"
                  size="sm"
                  class="h-9 text-xs font-bold"
                  disabled={isSubmitting || !selectedPaymentMethodObj || (selectedPaymentMethodObj.is_digital_gateway && receiptLines.length !== 1)}
                >
                  {isSubmitting ? m.finance_ui_saving() : m.finance_ui_receipt_save()}
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
        loading={gridLoading}
        serverQuery
        rtl={isAr()}
        search={searchQuery}
        searchPlaceholder={m.finance_ui_receipt_search_placeholder()}
        onquerychange={handlePaymentGridQuery}
        onselection={handlePaymentSelection}
        readonly
        emptyTitle={paymentsQuery.isError ? m.finance_ui_payments_load_failed() : m.finance_ui_no_receipts()}
        hasMore={paymentsQuery.hasNextPage ?? false}
        onloadmore={loadMorePayments}
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
    { label: m.finance_ui_credit_account(), value: detailRow.counter_account_display },
    { label: m.finance_ui_receiving_account(), value: detailRow.treasury_display },
    { label: m.finance_ui_payment_method(), value: detailRow.method_display },
    { label: m.finance_ui_amount(), value: detailRow.amount_formatted },
    { label: m.finance_ui_currency(), value: detailRow.currency_display },
    { label: m.finance_ui_description_notes(), value: detailRow.reference },
  ] : []}
  onprint={printDetail}
>
  <JournalLineDetails entry={receiptJournal.data} accountName={receiptAccountName} />
</FinanceRecordDialog>

<PrintDialog
  bind:open={printDialogOpen}
  entityType="finance"
  documentType="RECEIPT"
  entityIds={printRows.map((payment) => payment.id)}
  entityLabels={printRows.map((payment) => payment.receipt_display)}
  contexts={paymentPrintContexts}
/>
