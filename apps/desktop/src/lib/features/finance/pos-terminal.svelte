<!--
  @component PosTerminal
  High-speed cashier point-of-sale terminal for educational billing collections.
  Keyboard-optimized with barcode/ID lookup, outstanding fee checklist,
  canonical multi-channel payment method handling, and atomic General Ledger posting.
-->
<script lang="ts">
import { LoadingIndicator } from "@orbis/ui";
import { onMount } from "svelte";
import { slide } from "svelte/transition";
import * as m from "$lib/paraglide/messages";
import { getLocale } from "$lib/paraglide/runtime";
import { formatCurrency, formatDate } from "$lib/core/i18n";
import {
  createCashAccountsQuery,
  createStudentFeesQuery,
  createPaymentsQuery,
  createProcessPaymentMutation,
  createAvailablePaymentMethodsQuery,
  type StudentFeeDto,
  type PaymentDto,
} from "$lib/features/finance";
import { api, extractErrorMessage, type StudentDto } from "$lib/api";
import { Button } from "@orbis/ui/button";
import { Skeleton } from "@orbis/ui/skeleton";
import { toast } from "@orbis/ui";
import {
  Banknotes,
  BuildingLibrary,
  CheckCircle,
  CreditCard,
  Document,
  DocumentPage,
  Keyboard,
  Printer,
  XMark,
  ArrowPath,
} from "@orbis/icons";

let { orgId }: { orgId: string } = $props();

// ── State ─────────────────────────────────────────────────────────────

let searchInputEl = $state<HTMLInputElement | null>(null);
let searchQuery = $state("");
let isSearchFocused = $state(false);
let highlightedIndex = $state(-1);
let isSearchingStudents = $state(false);
let searchedStudents = $state<StudentDto[]>([]);
let selectedStudent = $state<StudentDto | null>(null);

// Payment Form State
let formAccountId = $state("");
let formMethod = $state("cash");
let formFeeId = $state("");
let formAmount = $state<number | null>(null);
let referenceNumber = $state("");
let sourceToken = $state("");
let notes = $state("");
let shouldPrintReceipt = $state(true);
let lastReceiptPayment = $state<PaymentDto | null>(null);

// ── Queries & Mutations ───────────────────────────────────────────────

const cashAccountsQuery = createCashAccountsQuery(() => orgId);
const cashAccounts = $derived(cashAccountsQuery.data ?? []);
const activeAccountId = $derived(formAccountId || cashAccounts[0]?.id || "");

const paymentMethodsQuery = createAvailablePaymentMethodsQuery(() => orgId);
const availablePaymentMethods = $derived(paymentMethodsQuery.data ?? []);
const selectedPaymentMethodObj = $derived(
  availablePaymentMethods.find((m) => m.id === formMethod) || availablePaymentMethods[0]
);

const studentFeesQuery = createStudentFeesQuery(
  () => orgId,
  () => (selectedStudent ? { studentId: selectedStudent.id } : undefined),
  () => Boolean(selectedStudent?.id),
);
const activeStudentFees = $derived(
  (studentFeesQuery.data ?? []).filter((f) => f.status !== "paid" && f.status !== "cancelled"),
);

const recentPaymentsQuery = createPaymentsQuery(
  () => orgId,
  () => (selectedStudent ? { studentId: selectedStudent.id } : undefined),
);
const activeStudentPayments = $derived(
  (recentPaymentsQuery.data ?? []).slice(0, 5),
);

const processPaymentMutation = createProcessPaymentMutation();

// ── Student Search Logic ──────────────────────────────────────────────

let searchDebounceTimer: ReturnType<typeof setTimeout> | null = null;

function handleSearchInput(query: string) {
  searchQuery = query;
  if (searchDebounceTimer) clearTimeout(searchDebounceTimer);

  if (!query.trim()) {
    searchedStudents = [];
    highlightedIndex = -1;
    return;
  }

  searchDebounceTimer = setTimeout(async () => {
    isSearchingStudents = true;
    try {
      const results = await api.listStudents({ query: query.trim() });
      searchedStudents = (results || []).slice(0, 8);
      highlightedIndex = searchedStudents.length > 0 ? 0 : -1;
    } catch {
      searchedStudents = [];
    } finally {
      isSearchingStudents = false;
    }
  }, 150);
}

function selectStudent(student: StudentDto) {
  selectedStudent = student;
  searchQuery = "";
  searchedStudents = [];
  highlightedIndex = -1;
  formFeeId = "";
  formAmount = null;
  lastReceiptPayment = null;
}

function deselectStudent() {
  selectedStudent = null;
  formFeeId = "";
  formAmount = null;
  searchQuery = "";
  searchedStudents = [];
  lastReceiptPayment = null;
  setTimeout(() => searchInputEl?.focus(), 50);
}

function selectFeeItem(fee: StudentFeeDto) {
  formFeeId = fee.id;
  formAmount = Number(fee.balance_due || fee.net_amount || 0);
}

// ── Payment Processing ────────────────────────────────────────────────

async function handlePaymentSubmit(e: SubmitEvent) {
  e.preventDefault();

  if (!selectedStudent) {
    toast.error(m.common_fee_toast_error_student());
    return;
  }
  if (!activeAccountId) {
    toast.error(m.common_payment_toast_error_cash_account());
    return;
  }
  if (!formAmount || formAmount <= 0) {
    toast.error(m.common_fee_toast_error_amount());
    return;
  }

  try {
    const payment = await processPaymentMutation.mutateAsync({
      fee_id: formFeeId || undefined,
      student_id: selectedStudent.id,
      account_id: activeAccountId,
      amount: Number(formAmount),
      method: formMethod,
      reference: referenceNumber.trim() || undefined,
      source: sourceToken.trim() || undefined,
      payer_name: `${selectedStudent.first_name || ""} ${selectedStudent.last_name || ""}`.trim() || undefined,
    });

    toast.success(m.common_payment_toast_success());
    lastReceiptPayment = payment;

    // Reset payment fields
    formAmount = null;
    formFeeId = "";
    referenceNumber = "";
    sourceToken = "";
    notes = "";

    // Refresh student fee list
    studentFeesQuery.refetch();
    recentPaymentsQuery.refetch();

    if (shouldPrintReceipt) {
      handlePrintReceipt(payment);
    }
  } catch (err) {
    toast.error(extractErrorMessage(err));
  }
}

async function handlePrintReceipt(payment: PaymentDto) {
  try {
    toast.info(m.finance_ui_receipt_print_preparing());
    const pdfUrl = await api.renderReport("doc_payment_receipt", {
      payment_id: payment.id,
      receipt_number: payment.receipt_number || payment.id.slice(0, 8),
      amount: String(payment.amount),
      currency: payment.currency,
      student_name: selectedStudent ? `${selectedStudent.first_name} ${selectedStudent.last_name}` : "",
      student_id: selectedStudent?.student_number || "",
      payment_method: payment.method,
      date: formatDate(payment.paid_at),
    });
    if (pdfUrl) {
      window.open(pdfUrl, "_blank");
    }
  } catch (e) {
    console.warn("DocForge direct preview failed:", e);
    toast.info(m.finance_ui_receipt_printed({ number: payment.receipt_number || payment.id.slice(0, 8) }));
  }
}

// ── Keyboard Shortcuts ────────────────────────────────────────────────

function handleGlobalKeyDown(e: KeyboardEvent) {
  if (e.key === "Escape") {
    if (searchQuery) {
      searchQuery = "";
      searchedStudents = [];
      highlightedIndex = -1;
    } else if (selectedStudent) {
      deselectStudent();
    }
  }

  if ((e.ctrlKey || e.metaKey) && (e.key === "f" || e.key === "k")) {
    e.preventDefault();
    searchInputEl?.focus();
  }

  if (isSearchFocused && searchedStudents.length > 0) {
    if (e.key === "ArrowDown") {
      e.preventDefault();
      highlightedIndex = (highlightedIndex + 1) % searchedStudents.length;
    } else if (e.key === "ArrowUp") {
      e.preventDefault();
      highlightedIndex = (highlightedIndex - 1 + searchedStudents.length) % searchedStudents.length;
    } else if (e.key === "Enter" && highlightedIndex >= 0) {
      e.preventDefault();
      selectStudent(searchedStudents[highlightedIndex]);
    }
  }
}

onMount(() => {
  searchInputEl?.focus();
});

const rtl = $derived(getLocale() === "ar");
</script>

<svelte:window onkeydown={handleGlobalKeyDown} />

<div class="h-full w-full flex flex-col bg-background overflow-hidden select-none text-xs" dir={rtl ? "rtl" : "ltr"}>
  <!-- Top Bar: Header & Cashier Status -->
  <header class="h-12 border-b border-border/60 bg-muted/20 px-4 flex items-center justify-between shrink-0">
    <div class="flex items-center gap-2.5">
      <div class="h-7 w-7 rounded-md bg-primary/10 text-primary flex items-center justify-center font-bold">
        <CreditCard size={15} />
      </div>
      <div>
        <h1 class="text-xs font-bold text-foreground leading-none">
          {m.finance_pos_quick_pos_payment_register()}
        </h1>
        <p class="text-[10px] text-muted-foreground mt-0.5">
          {m.finance_ui_pos_subtitle()}
        </p>
      </div>
    </div>

    <!-- Active Cashier & Treasury Dropdown -->
    <div class="flex items-center gap-2">
      <div class="flex items-center gap-1.5 bg-background border border-border/80 rounded-md px-2.5 py-1 text-[11px]">
        <BuildingLibrary size={13} class="text-muted-foreground" />
        <span class="text-muted-foreground">{m.finance_ui_receiving_treasury_label()}</span>
        <select
          bind:value={formAccountId}
          class="bg-transparent font-medium text-foreground outline-none cursor-pointer text-[11px]"
        >
          {#each cashAccounts as account (account.id)}
            <option value={account.id}>{account.name} ({account.currency})</option>
          {/each}
        </select>
      </div>

      <div class="flex items-center gap-1 text-[10px] text-muted-foreground bg-muted/40 border border-border/40 rounded px-2 py-1">
        <Keyboard size={12} />
        <span>{m.finance_ui_pos_search_shortcut()}</span>
      </div>
    </div>
  </header>

  <!-- Main POS Workspace Body: Split Grid -->
  <div class="flex-1 grid grid-cols-12 gap-0 overflow-hidden">
    
    <!-- Left 7 Cols: Student Lookup & Fee Invoices Breakdown -->
    <div class="col-span-7 border-e border-border/60 flex flex-col h-full overflow-hidden bg-background">
      
      <!-- Student Search Input Box -->
      <div class="p-3 border-b border-border/50 bg-muted/10 relative shrink-0">
        <div class="relative">
          <div class="absolute inset-y-0 inset-s-0 ps-3 flex items-center pointer-events-none text-muted-foreground">
          </div>
          <input
            bind:this={searchInputEl}
            type="text"
            value={searchQuery}
            oninput={(e) => handleSearchInput((e.target as HTMLInputElement).value)}
            onfocus={() => (isSearchFocused = true)}
            onblur={() => setTimeout(() => (isSearchFocused = false), 200)}
            placeholder={m.finance_ui_pos_student_search_placeholder()}
            class="w-full bg-background border border-border/80 rounded-lg ps-9 pe-9 py-2 text-xs text-foreground placeholder:text-muted-foreground/60 outline-none focus:border-primary focus:ring-1 focus:ring-primary font-mono"
          />
          {#if searchQuery}
            <button
              type="button"
              onclick={() => { searchQuery = ""; searchedStudents = []; }}
              class="absolute inset-y-0 inset-e-0 pe-3 flex items-center text-muted-foreground hover:text-foreground"
            >
              <XMark size={14} />
            </button>
          {/if}
        </div>

        <!-- Search Results Dropdown -->
        {#if isSearchFocused && searchedStudents.length > 0}
          <div
            transition:slide={{ duration: 120 }}
            class="absolute top-full inset-s-3 inset-e-3 mt-1 bg-popover text-popover-foreground border border-border rounded-lg shadow-lg z-50 overflow-hidden"
          >
            <div class="max-h-56 overflow-y-auto divide-y divide-border/40">
              {#each searchedStudents as student, idx (student.id)}
                <button
                  type="button"
                  onmousedown={() => selectStudent(student)}
                  class="w-full px-3 py-2 text-start flex items-center justify-between hover:bg-muted/80 transition-colors text-xs {idx === highlightedIndex ? 'bg-primary/10 text-primary font-medium' : ''}"
                >
                  <div class="flex items-center gap-2">
                    <div class="h-6 w-6 rounded-full bg-muted flex items-center justify-center font-bold text-[10px] text-foreground shrink-0">
                      {student.first_name?.[0] || m.finance_ui_student_initial()}
                    </div>
                    <div>
                      <div class="font-medium text-foreground">{student.first_name} {student.last_name}</div>
                      <div class="text-[10px] text-muted-foreground font-mono">#{student.student_number || student.id.slice(0, 8)}</div>
                    </div>
                  </div>
                  <div class="text-[10px] text-muted-foreground font-mono">
                    {student.national_id || "—"}
                  </div>
                </button>
              {/each}
            </div>
          </div>
        {/if}
      </div>

      <!-- Student Context & Fee Breakdown Area -->
      <div class="flex-1 overflow-y-auto p-4 flex flex-col gap-4">
        {#if selectedStudent}
          <!-- Selected Student Profile Banner -->
          <div class="bg-muted/30 border border-border/80 rounded-lg p-3 flex items-center justify-between shrink-0">
            <div class="flex items-center gap-3">
              <div class="h-10 w-10 rounded-full bg-primary/10 text-primary flex items-center justify-center font-bold text-sm">
                {selectedStudent.first_name?.[0] || m.finance_ui_student_initial()}
              </div>
              <div>
                <div class="text-sm font-bold text-foreground">
                  {selectedStudent.first_name} {selectedStudent.last_name}
                </div>
                <div class="text-[11px] text-muted-foreground flex items-center gap-3 mt-0.5 font-mono">
                  <span>{m.finance_ui_student_number_label()} #{selectedStudent.student_number || selectedStudent.id.slice(0, 8)}</span>
                  <span>{m.finance_ui_national_id_label()} {selectedStudent.national_id || "—"}</span>
                </div>
              </div>
            </div>

            <Button variant="ghost" size="sm" onclick={deselectStudent} class="h-7 text-xs text-muted-foreground hover:text-destructive">
              <XMark size={13} class="me-1" />
              {m.finance_ui_change_student_escape()}
            </Button>
          </div>

          <!-- Outstanding Fees Checklist -->
          <div class="flex flex-col gap-2">
            <div class="flex items-center justify-between">
              <h2 class="text-xs font-bold text-foreground flex items-center gap-1.5">
                <DocumentPage size={14} class="text-primary" />
                {m.finance_ui_due_fees_claims({ count: activeStudentFees.length })}
              </h2>
              <span class="text-[11px] text-muted-foreground">{m.finance_ui_select_fee_to_collect()}</span>
            </div>

            {#if studentFeesQuery.isLoading}
              <div class="space-y-2">
                <Skeleton class="h-12 w-full rounded-md" />
                <Skeleton class="h-12 w-full rounded-md" />
              </div>
            {:else if activeStudentFees.length === 0}
              <div class="bg-muted/20 border border-dashed border-border rounded-lg p-6 text-center text-muted-foreground">
                <CheckCircle size={24} class="mx-auto mb-1.5 text-emerald-500 opacity-80" />
                <p class="font-medium text-foreground">{m.finance_ui_no_fees_due_for_student()}</p>
                <p class="text-[10px] mt-0.5">{m.finance_ui_all_student_fees_paid()}</p>
              </div>
            {:else}
              <div class="space-y-1.5">
                {#each activeStudentFees as fee (fee.id)}
                  <button
                    type="button"
                    onclick={() => selectFeeItem(fee)}
                    class="w-full text-start p-2.5 rounded-lg border transition-all flex items-center justify-between {formFeeId === fee.id ? 'bg-primary/10 border-primary text-foreground shadow-sm' : 'bg-background border-border/70 hover:border-primary/50 text-muted-foreground'}"
                  >
                    <div class="flex items-center gap-2.5">
                      <div class="h-4 w-4 rounded-full border flex items-center justify-center {formFeeId === fee.id ? 'border-primary bg-primary text-primary-foreground' : 'border-muted-foreground/40'}">
                        {#if formFeeId === fee.id}
                          <div class="h-1.5 w-1.5 rounded-full bg-background"></div>
                        {/if}
                      </div>
                      <div>
                        <div class="font-semibold text-foreground text-xs">{fee.notes || m.finance_ui_tuition_fees()}</div>
                        <div class="text-[10px] text-muted-foreground font-mono mt-0.5">
                          {m.finance_ui_due_date_label()} {formatDate(fee.due_date) || "—"}
                          {#if fee.installment_number}
                            • {m.finance_ui_installment_number({ number: fee.installment_number })}
                          {/if}
                        </div>
                      </div>
                    </div>

                    <div class="text-end">
                      <div class="font-mono font-bold text-xs text-rose-600 dark:text-rose-400">
                        {formatCurrency(Number(fee.balance_due || fee.net_amount || 0))}
                      </div>
                      <div class="text-[10px] text-muted-foreground font-mono">
                        {m.finance_ui_paid_amount_label()} {formatCurrency(Number(fee.paid_amount || 0))}
                      </div>
                    </div>
                  </button>
                {/each}
              </div>
            {/if}
          </div>

          <!-- Recent Student Receipts Table -->
          {#if activeStudentPayments.length > 0}
            <div class="mt-2 border-t border-border/50 pt-3 flex flex-col gap-2">
              <h2 class="text-[11px] font-bold text-muted-foreground uppercase tracking-wider">
                {m.finance_ui_student_previous_receipts()}
              </h2>
              <div class="border border-border/60 rounded-md overflow-hidden bg-background">
                <table class="w-full text-start text-[11px]">
                  <thead class="bg-muted/40 border-b border-border/60 text-muted-foreground">
                    <tr>
                      <th class="py-1.5 px-3 font-semibold text-start">{m.finance_ui_voucher_number()}</th>
                      <th class="py-1.5 px-3 font-semibold text-start">{m.finance_ui_payment_method()}</th>
                      <th class="py-1.5 px-3 font-semibold text-start">{m.finance_ui_date()}</th>
                      <th class="py-1.5 px-3 font-semibold text-end">{m.finance_ui_amount()}</th>
                      <th class="py-1.5 px-2 font-semibold text-center w-10">{m.finance_ui_print()}</th>
                    </tr>
                  </thead>
                  <tbody class="divide-y divide-border/40">
                    {#each activeStudentPayments as payment (payment.id)}
                      <tr class="hover:bg-muted/20">
                        <td class="py-1.5 px-3 font-mono font-medium text-foreground">
                          #{payment.receipt_number || payment.id.slice(0, 8)}
                        </td>
                        <td class="py-1.5 px-3 text-muted-foreground">{payment.method}</td>
                        <td class="py-1.5 px-3 font-mono text-muted-foreground">{formatDate(payment.paid_at)}</td>
                        <td class="py-1.5 px-3 font-mono font-bold text-emerald-600 dark:text-emerald-400 text-end">
                          {formatCurrency(Number(payment.amount))}
                        </td>
                        <td class="py-1.5 px-2 text-center">
                          <button
                            type="button"
                            onclick={() => handlePrintReceipt(payment)}
                            class="p-1 text-muted-foreground hover:text-primary transition-colors"
                            title={m.finance_ui_print_receipt()}
                          >
                            <Printer size={13} />
                          </button>
                        </td>
                      </tr>
                    {/each}
                  </tbody>
                </table>
              </div>
            </div>
          {/if}

        {:else}
          <!-- Empty Search State -->
          <div class="h-full flex flex-col items-center justify-center text-center p-8 text-muted-foreground">
            <div class="h-14 w-14 rounded-full bg-muted/40 flex items-center justify-center text-muted-foreground/60 mb-3">
              <MagnifyingGlass size={28} />
            </div>
            <h3 class="text-sm font-bold text-foreground">{m.finance_ui_select_student_to_collect()}</h3>
            <p class="text-[11px] text-muted-foreground max-w-sm mt-1">
              {m.finance_ui_pos_select_student_instructions()}
            </p>
          </div>
        {/if}
      </div>
    </div>

    <!-- Right 5 Cols: Payment Collection Keypad & Execution -->
    <div class="col-span-5 bg-muted/5 flex flex-col h-full overflow-hidden">
      <form onsubmit={handlePaymentSubmit} class="flex flex-col h-full overflow-y-auto p-4 gap-4">
        
        <div class="border-b border-border/60 pb-2">
          <h2 class="text-xs font-bold text-foreground flex items-center gap-1.5">
            <Banknotes size={15} class="text-emerald-600" />
            {m.finance_ui_receipt_and_payment_details()}
          </h2>
          <p class="text-[10px] text-muted-foreground mt-0.5">
            {m.finance_ui_pos_payment_accounting_note()}
          </p>
        </div>

        <!-- Payment Method Tabs (Dynamically Resolved via Integration Layer) -->
        <div class="flex flex-col gap-1.5">
          <div class="flex items-center justify-between">
            <p class="text-[11px] font-semibold text-foreground">{m.finance_pos_payment_method_integrations()}</p>
            {#if selectedPaymentMethodObj?.is_digital_gateway}
              <span class="text-[9px] bg-primary/10 text-primary border border-primary/20 rounded px-1.5 py-0.5 font-medium">
                {m.finance_ui_gateway_provider({ provider: selectedPaymentMethodObj.provider_name ?? "" })}
              </span>
            {/if}
          </div>

          <div class="grid grid-cols-2 sm:grid-cols-3 gap-1.5">
            {#each availablePaymentMethods as methodItem (methodItem.id)}
              <button
                type="button"
                onclick={() => (formMethod = methodItem.id)}
                class="flex items-center justify-center gap-1.5 py-2 px-2 rounded-md border text-[11px] font-medium transition-all {formMethod === methodItem.id ? 'bg-primary text-primary-foreground border-primary shadow-xs' : 'bg-background border-border/80 text-muted-foreground hover:bg-muted/60'}"
              >
                {#if methodItem.category === "manual"}
                  <Banknotes size={13} />
                {:else if methodItem.category === "bank"}
                  {#if methodItem.id === "check"}
                    <Document size={13} />
                  {:else}
                    <BuildingLibrary size={13} />
                  {/if}
                {:else}
                  <CreditCard size={13} />
                {/if}
                <span class="truncate">{methodItem.name}</span>
              </button>
            {/each}
          </div>

          {#if availablePaymentMethods.filter((m) => m.is_digital_gateway).length === 0}
            <div class="text-[10px] text-muted-foreground bg-muted/20 border border-border/40 rounded p-1.5 mt-1 flex items-center justify-between">
              <span>{m.finance_ui_payment_gateway_activation_required()}</span>
              <a href="/{orgId}/settings/integrations" class="text-primary hover:underline font-medium">
                {m.finance_ui_integrations_store()}
              </a>
            </div>
          {/if}
        </div>

        <!-- Amount Entry Field -->
        <div class="flex flex-col gap-1.5">
          <div class="flex items-center justify-between">
            <label for="pos_amount" class="text-[11px] font-semibold text-foreground">{m.finance_ui_amount_received_to_collect()}</label>
            {#if formFeeId}
              {@const selectedFee = activeStudentFees.find(f => f.id === formFeeId)}
              {#if selectedFee}
                <button
                  type="button"
                  onclick={() => (formAmount = Number(selectedFee.balance_due || 0))}
                  class="text-[10px] text-primary hover:underline font-mono"
                >
                  {m.finance_ui_full_balance()} ({formatCurrency(Number(selectedFee.balance_due || 0))})
                </button>
              {/if}
            {/if}
          </div>
          <div class="relative">
            <input
              id="pos_amount"
              type="number"
              step="any"
              min="0.01"
              required
              bind:value={formAmount}
              placeholder="0.00"
              class="w-full bg-background border border-border/80 rounded-lg px-3 py-2.5 text-base font-bold font-mono text-foreground placeholder:text-muted-foreground/40 outline-none focus:border-primary focus:ring-1 focus:ring-primary"
            />
          </div>
        </div>

        <!-- Electronic Source Token (if Gateway method requires token) -->
        {#if selectedPaymentMethodObj?.requires_source_token}
          <div class="flex flex-col gap-1.5" transition:slide={{ duration: 120 }}>
            <label for="pos_token" class="text-[11px] font-semibold text-foreground flex items-center justify-between">
              <span>{m.finance_ui_payment_gateway_token_label()}</span>
              <span class="text-[9px] text-muted-foreground font-mono">{m.finance_ui_gateway_source_token()}</span>
            </label>
            <input
              id="pos_token"
              type="text"
              bind:value={sourceToken}
              placeholder={m.finance_ui_pos_payment_token_placeholder()}
              class="w-full bg-background border border-border/80 rounded-md px-3 py-1.5 text-xs text-foreground placeholder:text-muted-foreground/50 outline-none focus:border-primary font-mono"
            />
          </div>
        {/if}

        <!-- Reference / Check Number -->
        {#if formMethod !== "cash"}
          <div class="flex flex-col gap-1.5" transition:slide={{ duration: 120 }}>
            <label for="pos_ref" class="text-[11px] font-semibold text-foreground">
              {formMethod === "check" ? m.finance_ui_check_reference_label() : m.finance_ui_transfer_reference_label()}
            </label>
            <input
              id="pos_ref"
              type="text"
              bind:value={referenceNumber}
              placeholder={formMethod === "check" ? m.finance_ui_check_reference_placeholder() : m.finance_ui_transfer_reference_placeholder()}
              class="w-full bg-background border border-border/80 rounded-md px-3 py-1.5 text-xs text-foreground placeholder:text-muted-foreground/50 outline-none focus:border-primary font-mono"
            />
          </div>
        {/if}

        <!-- Notes -->
        <div class="flex flex-col gap-1.5">
          <label for="pos_notes" class="text-[11px] font-semibold text-foreground">{m.finance_ui_receipt_notes_optional()}</label>
          <input
            id="pos_notes"
            type="text"
            bind:value={notes}
            placeholder={m.finance_ui_receipt_notes_placeholder()}
            class="w-full bg-background border border-border/80 rounded-md px-3 py-1.5 text-xs text-foreground placeholder:text-muted-foreground/50 outline-none focus:border-primary"
          />
        </div>

        <!-- Receipt Print Option -->
        <div class="flex items-center gap-2 pt-1">
          <input
            type="checkbox"
            id="auto_print"
            bind:checked={shouldPrintReceipt}
            class="rounded border-border text-primary focus:ring-primary h-3.5 w-3.5"
          />
          <label for="auto_print" class="text-[11px] text-muted-foreground cursor-pointer select-none">
            {m.finance_ui_auto_print_receipt_docforge()}
          </label>
        </div>

        <!-- Submission Action Buttons -->
        <div class="mt-auto pt-4 flex flex-col gap-2">
          <Button
            type="submit"
            disabled={!selectedStudent || !formAmount || formAmount <= 0 || processPaymentMutation.isPending}
            class="w-full h-10 text-xs font-bold bg-emerald-600 hover:bg-emerald-700 text-white flex items-center justify-center gap-2 shadow-sm"
          >
            {#if processPaymentMutation.isPending}
              <LoadingIndicator   size={14} />
              <span>{m.finance_ui_posting_entry_and_issuing_receipt()}</span>
            {:else}
              <CheckCircle size={15} />
              <span>{m.finance_ui_post_receipt_now_enter()}</span>
            {/if}
          </Button>

          {#if lastReceiptPayment}
            <Button
              type="button"
              variant="outline"
              size="sm"
              onclick={() => handlePrintReceipt(lastReceiptPayment!)}
              class="w-full h-8 text-xs text-foreground"
            >
              <Printer size={13} class="me-1.5 text-primary" />
              {m.finance_ui_reprint_last_receipt({ number: lastReceiptPayment.receipt_number || lastReceiptPayment.id.slice(0, 8) })}
            </Button>
          {/if}
        </div>

      </form>
    </div>

  </div>
</div>
