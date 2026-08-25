<!--
@component PaymentPOSPage
A dedicated, keyboard-optimized screen for school cashiers to record student payments rapidly.
Provides barcode/ID lookup, outstanding fee checklist, and payment history.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import { commands } from "$lib/core/ipc";
import { authStore } from "$lib/stores";
import { Button } from "@orbis/ui-svelte/button";
import { Input } from "@orbis/ui-svelte/input";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import { toast } from "svelte-sonner";
import {
  Search,
  ArrowLeft,
  CreditCard,
  User,
  Hash,
  BookOpen,
  DollarSign,
  Calendar,
  CheckCircle,
  FileText,
  Keyboard,
  X,
  Plus,
} from "@lucide/svelte";
import { getLocale } from "$lib/paraglide/runtime";
import { matchStudent } from "$lib/shared/utils/search";
import { formatCurrency, formatDate } from "$lib/core/i18n";
import { fade, slide } from "svelte/transition";

let { data }: { data: { orgId: string } } = $props();

// Main Lists from DB
let students = $state<any[]>([]);
let treasuries = $state<any[]>([]);
let isLoadingData = $state(true);

// Selection States
let searchQuery = $state("");
let isSearchFocused = $state(false);
let selectedStudent = $state<any | null>(null);
let activeStudentFees = $state<any[]>([]);
let activeStudentPayments = $state<any[]>([]);
let isLoadingStudentDetails = $state(false);

// Active Selected Fee to Pay
let selectedFee = $state<any | null>(null);
let selectedFeeId = $state("");

// Search Dropdown Keyboard Navigation
let highlightedIndex = $state(-1);
let searchInputEl = $state<HTMLInputElement | null>(null);

// Form Fields
let selectedTreasuryId = $state("");
let paymentMethod = $state<"Cash" | "Bank Transfer" | "Card">("Cash");
let currency = $state("YER");
let amount = $state<number | null>(null);
let receiptNumber = $state("");
let referenceNumber = $state("");
let notes = $state("");
let isSubmitting = $state(false);

// Filter students locally based on search query
const filteredStudents = $derived.by(() => {
  if (!searchQuery.trim()) return [];
  return students.filter((s) => matchStudent(s, searchQuery)).slice(0, 8); // Limit to 8 quick results
});

// Load basic startup data
async function loadData() {
  if (!data.orgId) return;
  isLoadingData = true;
  try {
    const [studentList, treasuryList] = await Promise.all([
      commands.academic.listEnrollments(),
      commands.finance.listAccounts().then((accounts) => accounts.filter((account: any) => ["cash", "bank"].includes(account.sub_type))),
    ]);
    students = studentList;
    treasuries = treasuryList;

    if (treasuryList.length > 0) {
      selectedTreasuryId = String(treasuryList[0].id);
      currency = String((treasuryList[0] as any).currency || "YER");
    }
  } catch (e) {
    console.error("Error loading POS startup data:", e);
    toast.error(m.finance_pos_error_loading_lookup_data());
  } finally {
    isLoadingData = false;
    // Auto-focus search input
    setTimeout(() => {
      searchInputEl?.focus();
    }, 100);
  }
}

// Load selected student's fees and payments
async function loadStudentData(enrollId: string) {
  if (!enrollId || !data.orgId) return;
  isLoadingStudentDetails = true;
  try {
    const [feesList, paymentsList] = await Promise.all([
      commands.finance.listStudentFeesByEnrollment(enrollId),
      commands.finance.listPayments(),
    ]);

    // Unpaid/Partially paid fees
    activeStudentFees = feesList.filter((f) => f.status !== "paid");

    // Filter payments for this student
    activeStudentPayments = paymentsList
      .filter((p) => String(p.enrollment_id) === String(enrollId))
      .slice(0, 5); // last 5 payments

    // Auto-select first fee if available
    if (activeStudentFees.length > 0) {
      selectFeeItem(activeStudentFees[0]);
    } else {
      selectedFee = null;
      selectedFeeId = "";
      amount = null;
    }
  } catch (e) {
    console.error("Error loading details for student:", e);
    toast.error(m.finance_pos_error_loading_student_billing_info());
  } finally {
    isLoadingStudentDetails = false;
  }
}

function selectStudent(student: any) {
  selectedStudent = student;
  searchQuery = "";
  highlightedIndex = -1;
  loadStudentData(student.id);
}

function deselectStudent() {
  selectedStudent = null;
  activeStudentFees = [];
  activeStudentPayments = [];
  selectedFee = null;
  selectedFeeId = "";
  amount = null;
  receiptNumber = "";
  referenceNumber = "";
  notes = "";
  setTimeout(() => {
    searchInputEl?.focus();
  }, 50);
}

function selectFeeItem(fee: any) {
  selectedFee = fee;
  selectedFeeId = String(fee.id);
  amount = Number(fee.balance || fee.total_amount || 0);
}

// Submitting Payment Command
async function handlePaymentSubmit(e: SubmitEvent) {
  e.preventDefault();
  if (!selectedStudent) {
    toast.error(m.common_fee_toast_error_student());
    return;
  }
  if (!selectedFeeId) {
    toast.error(m.common_payment_toast_error_fee());
    return;
  }
  if (!selectedTreasuryId) {
    toast.error(m.common_payment_toast_error_treasury());
    return;
  }
  if (!amount || amount <= 0) {
    toast.error(m.common_fee_toast_error_amount());
    return;
  }
  if (!currency || currency.trim().length !== 3) {
    toast.error(m.common_ledger_toast_error_currency());
    return;
  }

  isSubmitting = true;
  try {
    const createdBy = authStore.userId || authStore.user?.id || "unknown";
    const request = {
      fee_id: selectedFeeId,
      amount: Number(amount),
      method:
        paymentMethod === "Cash"
          ? "cash"
          : paymentMethod === "Bank Transfer"
            ? "bank_transfer"
            : "card",
      reference: referenceNumber || undefined,
      notes: notes || undefined,
      cash_account_id: selectedTreasuryId,
      currency: currency.trim().toUpperCase(),
    };

    await commands.finance.recordPayment({
      ...request,
      exchange_rate: 1,
      account_id: "", // Let backend resolve if needed via fee_id
    } as any);
    toast.success(m.common_payment_toast_success());

    // Clean form elements
    amount = null;
    receiptNumber = "";
    referenceNumber = "";
    notes = "";

    // Reload student details to reflect new paid balance
    await loadStudentData(selectedStudent.id);

    // Refocus search for next transaction if current balance is cleared
    if (activeStudentFees.length === 0) {
      toast.info(m.finance_pos_all_outstanding_fees_for_this());
      deselectStudent();
    }
  } catch (e) {
    const errorMsg = e instanceof Error ? e.message : String(e);
    toast.error(m.common_payment_toast_error_submit({ error: errorMsg }));
  } finally {
    isSubmitting = false;
  }
}

// Global Keyboard Handler for POS layout
function handleGlobalKeyDown(e: KeyboardEvent) {
  // ESC key
  if (e.key === "Escape") {
    if (searchQuery) {
      searchQuery = "";
      highlightedIndex = -1;
    } else if (selectedStudent) {
      deselectStudent();
    }
  }

  // Ctrl + S or Ctrl + F to focus search input
  if ((e.ctrlKey || e.metaKey) && (e.key === "s" || e.key === "f")) {
    e.preventDefault();
    searchInputEl?.focus();
  }

  // Keyboard navigation for dropdown when focused
  if (isSearchFocused && filteredStudents.length > 0) {
    if (e.key === "ArrowDown") {
      e.preventDefault();
      highlightedIndex = (highlightedIndex + 1) % filteredStudents.length;
    } else if (e.key === "ArrowUp") {
      e.preventDefault();
      highlightedIndex =
        (highlightedIndex - 1 + filteredStudents.length) %
        filteredStudents.length;
    } else if (e.key === "Enter" && highlightedIndex >= 0) {
      e.preventDefault();
      selectStudent(filteredStudents[highlightedIndex]);
    }
  }
}

// Auto-select correct treasury currency when changed
$effect(() => {
  if (selectedTreasuryId && treasuries.length > 0) {
    const activeT = treasuries.find((t) => String(t.id) === selectedTreasuryId);
    if (activeT) {
      currency = String(activeT.currency || "YER");
    }
  }
});

onMount(() => {
  loadData();
});
</script>

<svelte:window onkeydown={handleGlobalKeyDown} />

<div class="h-full w-full overflow-hidden flex flex-col font-sans bg-background">
  <!-- Top Bar / Navigation Header -->
  <div class="flex items-center justify-between px-6 py-4 border-b border-border bg-card/60 backdrop-blur shrink-0 select-none">
    <div class="flex items-center gap-3">
      <a 
        href={`/${data.orgId}/finance/payments`} 
        class="h-8 w-8 rounded-lg hover:bg-accent flex items-center justify-center border border-border duration-100"
        title={m.finance_pos_back_to_payments()}
      >
        <ArrowLeft size={16} class={getLocale() === "ar" ? "rotate-180" : ""} />
      </a>
      <div class="text-start">
        <h2 class="text-sm font-bold text-foreground uppercase tracking-wider flex items-center gap-2">
          <CreditCard size={15} class="text-lime-500" />
          <span>{m.finance_pos_quick_pos_payment_register()}</span>
        </h2>
        <p class="text-[10px] text-muted-foreground mt-0.5">
          {m.finance_pos_record_student_payments_instantly_with()}
        </p>
      </div>
    </div>

    <!-- Quick Shortcuts Legend -->
    <div class="hidden md:flex items-center gap-4 text-[10px] text-muted-foreground bg-accent/30 px-3 py-1.5 rounded-lg border border-border">
      <span class="font-bold text-foreground flex items-center gap-1">
        <Keyboard size={12} class="text-primary" />
        <span>{m.finance_pos_shortcuts()}</span>
      </span>
      <span class="flex items-center gap-1">
        <kbd class="px-1.5 py-0.5 rounded bg-card border border-border font-mono text-[8px]">Ctrl + S</kbd>
        <span>{m.finance_pos_focus_search()}</span>
      </span>
      <span class="flex items-center gap-1">
        <kbd class="px-1.5 py-0.5 rounded bg-card border border-border font-mono text-[8px]">ESC</kbd>
        <span>{m.finance_pos_clear_student()}</span>
      </span>
      <span class="flex items-center gap-1">
        <kbd class="px-1.5 py-0.5 rounded bg-card border border-border font-mono text-[8px]">Enter</kbd>
        <span>{m.finance_pos_submit()}</span>
      </span>
    </div>
  </div>

  <!-- Main POS Workspace (Split Screen) -->
  <div class="flex-1 min-h-0 flex flex-col lg:flex-row overflow-hidden">
    
    <!-- LEFT COLUMN: Search & Student Card (Occupies 55%) -->
    <div class="w-full lg:w-[55%] flex flex-col border-r border-border min-h-0 p-6 overflow-y-auto gap-5">
      
      <!-- Student Search Bar -->
      <div class="relative w-full text-start">
        <label for="pos_search" class="text-xs font-bold text-muted-foreground uppercase tracking-wider mb-2 block">
          {m.finance_pos_search_verify_student()}
        </label>
        
        <div class="relative">
          <Search class="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
          <input
            id="pos_search"
            type="text"
            placeholder={m.finance_pos_search_by_student_number_or()}
            class="w-full bg-card/45 border border-border rounded-xl ps-9 pe-9 py-2.5 text-sm focus:ring-1 focus:ring-lime-500 focus:border-lime-500 outline-none transition-all"
            bind:value={searchQuery}
            bind:this={searchInputEl}
            onfocus={() => { isSearchFocused = true; highlightedIndex = -1; }}
            onblur={() => setTimeout(() => { isSearchFocused = false; }, 180)}
            disabled={isLoadingData}
            autocomplete="off"
          />
          {#if searchQuery}
            <button 
              type="button" 
              onclick={() => { searchQuery = ""; highlightedIndex = -1; }}
              class="absolute right-3 top-1/2 -translate-y-1/2 text-muted-foreground/60 hover:text-foreground"
            >
              <X size={14} />
            </button>
          {/if}
        </div>

        <!-- Autocomplete Suggestions dropdown -->
        {#if isSearchFocused && filteredStudents.length > 0}
          <div 
            class="absolute left-0 right-0 mt-1.5 bg-card border border-border rounded-xl shadow-2xl z-50 overflow-hidden max-h-[300px] overflow-y-auto"
            transition:slide={{ duration: 100 }}
          >
            {#each filteredStudents as s, index}
              <button
                type="button"
                class="w-full text-start px-4 py-3 flex items-center justify-between border-b border-border/40 duration-75 text-xs {index === highlightedIndex ? 'bg-lime-500/10 text-primary-foreground font-semibold border-lime-500/20' : 'hover:bg-accent/40 text-foreground'}"
                onclick={() => selectStudent(s)}
              >
                <div class="flex items-center gap-2.5">
                  <div class="h-7 w-7 rounded-full bg-primary/10 flex items-center justify-center text-primary font-bold">
                    {s.full_name?.charAt(0) || "?"}
                  </div>
                  <div class="flex flex-col">
                    <span class="font-bold">{s.full_name}</span>
                    <span class="text-[9px] text-muted-foreground">{s.student_number || s.id}</span>
                  </div>
                </div>
                {#if s.student_number}
                  <span class="font-mono bg-accent/60 px-2 py-0.5 rounded text-[9px] font-semibold border border-border">
                    #{s.student_number}
                  </span>
                {/if}
              </button>
            {/each}
          </div>
        {/if}
      </div>

      <!-- Left Bottom Workspace Contents -->
      {#if isLoadingData}
        <div class="flex-1 flex flex-col gap-4">
          <Skeleton class="h-32 w-full rounded-xl" />
          <Skeleton class="h-44 w-full rounded-xl" />
        </div>
      {:else if !selectedStudent}
        <!-- POS Splash / Empty State -->
        <div class="flex-1 flex flex-col items-center justify-center border border-dashed border-border rounded-2xl p-8 bg-card/5 text-center min-h-[300px]">
          <div class="h-16 w-16 rounded-full bg-lime-500/10 flex items-center justify-center text-lime-500 mb-4 select-none">
            <Search size={32} />
          </div>
          <h3 class="font-bold text-sm text-foreground mb-1">
            {m.finance_pos_select_student_to_begin()}
          </h3>
          <p class="text-[10px] text-muted-foreground max-w-sm mb-4">
            {m.finance_pos_search_using_full_name_or()}
          </p>
          <div class="flex items-center gap-2">
            <span class="text-[9px] px-2 py-1 bg-accent/50 rounded text-muted-foreground border border-border">
              {m.finance_pos_press_ctrl_s_to_focus_search()}
            </span>
          </div>
        </div>
      {:else}
        <!-- Selected Student Profile Box -->
        <div class="card-tactile bg-card/30 border border-border p-4 rounded-xl flex flex-col gap-4 text-start relative" in:fade={{ duration: 150 }}>
          <button
            type="button"
            onclick={deselectStudent}
            class="absolute top-3 right-3 text-muted-foreground/60 hover:text-foreground h-6 w-6 rounded-md flex items-center justify-center hover:bg-accent border border-border/50"
            title={m.finance_pos_deselect_student()}
          >
            <X size={12} />
          </button>
          
          <div class="flex items-center gap-4">
            <div class="h-12 w-12 rounded-xl bg-lime-500/10 text-lime-500 flex items-center justify-center border border-lime-500/20 font-bold text-lg select-none">
              {selectedStudent.full_name?.charAt(0) || "?"}
            </div>
            <div class="flex flex-col gap-0.5">
              <span class="text-sm font-bold text-foreground">{selectedStudent.full_name}</span>
              <div class="flex items-center gap-3 text-[10px] text-muted-foreground">
                <span class="flex items-center gap-0.5 font-mono">
                  <Hash size={10} />
                  <span>#{selectedStudent.student_number || selectedStudent.id.slice(0,8)}</span>
                </span>
                {#if selectedStudent.class_name}
                  <span class="flex items-center gap-0.5">
                    <BookOpen size={10} />
                    <span>{selectedStudent.class_name}</span>
                  </span>
                {/if}
              </div>
            </div>
          </div>
        </div>

        <!-- Student Outstanding Fees section -->
        <div class="flex flex-col gap-2.5 text-start">
          <h4 class="text-xs font-bold text-foreground flex items-center justify-between select-none">
            <span>{m.finance_pos_outstanding_fees_unpaid()}</span>
            <span class="text-[9.5px] px-2 py-0.5 rounded bg-rose-500/10 text-rose-500 font-mono">
              {activeStudentFees.length} {m.finance_pos_pending()}
            </span>
          </h4>

          {#if isLoadingStudentDetails}
            <div class="flex flex-col gap-2">
              <Skeleton class="h-12 w-full rounded-lg" />
              <Skeleton class="h-12 w-full rounded-lg" />
            </div>
          {:else if activeStudentFees.length === 0}
            <div class="border border-border/80 border-dashed rounded-xl p-6 text-center bg-emerald-500/5 text-emerald-500">
              <CheckCircle size={24} class="mx-auto mb-2 text-emerald-500/80" />
              <p class="text-xs font-bold">{m.finance_pos_no_pending_fees()}</p>
              <p class="text-[9px] text-muted-foreground mt-0.5">
                {m.finance_pos_all_invoiced_amounts_have_been()}
              </p>
            </div>
          {:else}
            <!-- Tactical Interactive list of fees -->
            <div class="flex flex-col gap-2">
              {#each activeStudentFees as fee (fee.id)}
                {@const isSelected = selectedFeeId === String(fee.id)}
                <button
                  type="button"
                  onclick={() => selectFeeItem(fee)}
                  class="w-full card-tactile border p-3.5 rounded-xl flex items-center justify-between text-start duration-100 {isSelected ? 'bg-lime-500/10 border-lime-500 text-foreground ring-1 ring-lime-500/50' : 'bg-card hover:bg-accent/40 border-border'}"
                >
                  <div class="flex flex-col gap-0.5 max-w-[65%]">
                    <span class="text-xs font-bold text-foreground truncate">
                      {fee.description || m.common_ft_category_tuition()}
                    </span>
                    <span class="text-[9px] text-muted-foreground font-mono flex items-center gap-1">
                      <Calendar size={10} />
                      <span>{m.finance_pos_due_date()}{fee.due_date ? formatDate(fee.due_date) : "-"}</span>
                    </span>
                  </div>

                  <!-- Amount due details -->
                  <div class="flex flex-col items-end gap-0.5">
                    <span class="text-xs font-bold font-mono text-rose-500">
                      {formatCurrency(Number(fee.balance || fee.amount || 0), fee.currency || "YER")}
                    </span>
                    {#if Number(fee.paid_amount) > 0}
                      <span class="text-[8.5px] text-emerald-500 font-mono">
                        {m.finance_pos_paid()}{formatCurrency(Number(fee.paid_amount), fee.currency)}
                      </span>
                    {/if}
                  </div>
                </button>
              {/each}
            </div>
          {/if}
        </div>

        <!-- Student Recent Payment Ledger Logs -->
        <div class="flex flex-col gap-2.5 text-start mt-2">
          <h4 class="text-xs font-bold text-foreground select-none">
            {m.finance_pos_recent_recorded_payments()}
          </h4>

          {#if isLoadingStudentDetails}
            <div class="flex flex-col gap-2">
              <Skeleton class="h-10 w-full rounded-lg" />
            </div>
          {:else if activeStudentPayments.length === 0}
            <p class="text-[10px] text-muted-foreground border border-border border-dashed p-4 text-center rounded-xl">
              {m.finance_pos_no_payment_logs_on_file()}
            </p>
          {:else}
            <div class="border border-border rounded-xl bg-card overflow-hidden">
              <table class="w-full text-xs text-start">
                <thead>
                  <tr class="bg-accent/40 border-b border-border/80 text-[10px] text-muted-foreground select-none">
                    <th class="p-2.5 text-start font-semibold">{m.finance_pos_receipt()}</th>
                    <th class="p-2.5 text-start font-semibold">{m.common_overview_date()}</th>
                    <th class="p-2.5 text-start font-semibold">{m.finance_pos_method()}</th>
                    <th class="p-2.5 text-end font-semibold">{m.common_ft_col_amount()}</th>
                  </tr>
                </thead>
                <tbody>
                  {#each activeStudentPayments as pay (pay.id)}
                    <tr class="border-b border-border/40 hover:bg-accent/10">
                      <td class="p-2.5 font-bold font-mono">#{pay.receipt_number || pay.id.slice(0,6)}</td>
                      <td class="p-2.5 text-muted-foreground text-[10px]">{formatDate(pay.payment_date || pay.created_at)}</td>
                      <td class="p-2.5 text-muted-foreground text-[10px] uppercase">{pay.payment_method || "Cash"}</td>
                      <td class="p-2.5 text-end font-bold font-mono text-emerald-500">{formatCurrency(Number(pay.amount), pay.currency)}</td>
                    </tr>
                  {/each}
                </tbody>
              </table>
            </div>
          {/if}
        </div>
      {/if}

    </div>

    <!-- RIGHT COLUMN: Payment Register Form Panel (Occupies 45%) -->
    <div class="w-full lg:w-[45%] flex flex-col bg-card/30 p-6 overflow-y-auto min-h-0">
      {#if !selectedStudent}
        <!-- Form Disabled Overlay state -->
        <div class="flex-1 flex flex-col items-center justify-center text-center text-muted-foreground select-none">
          <FileText size={40} class="text-muted-foreground/30 mb-3" />
          <p class="text-xs font-semibold">{m.finance_pos_payment_form_offline()}</p>
          <p class="text-[9.5px] text-muted-foreground/80 mt-1 max-w-[250px]">
            {m.finance_pos_search_for_a_student_and()}
          </p>
        </div>
      {:else if !selectedFee}
        <!-- Fee Select Overlay State -->
        <div class="flex-1 flex flex-col items-center justify-center text-center text-muted-foreground select-none">
          <Plus size={40} class="text-muted-foreground/30 mb-3" />
          <p class="text-xs font-semibold">{m.finance_pos_select_a_fee_to_pay()}</p>
          <p class="text-[9.5px] text-muted-foreground/80 mt-1 max-w-[250px]">
            {m.finance_pos_please_pick_which_outstanding_item()}
          </p>
        </div>
      {:else}
        <!-- Payment Recording Form -->
        <form onsubmit={handlePaymentSubmit} class="flex flex-col gap-5 text-start h-full" in:fade={{ duration: 150 }}>
          <h3 class="text-sm font-bold text-foreground border-b border-border/80 pb-2 select-none flex items-center gap-2">
            <DollarSign size={15} class="text-lime-500" />
            <span>{m.finance_pos_record_payment_receipt()}</span>
          </h3>

          <!-- Fee Information Preview -->
          <div class="bg-accent/40 border border-border p-3.5 rounded-xl text-xs flex flex-col gap-1.5 select-none">
            <div class="flex justify-between">
              <span class="text-muted-foreground">{m.finance_pos_active_item()}</span>
              <span class="font-bold text-foreground max-w-[200px] truncate">{selectedFee.description || m.common_ft_category_tuition()}</span>
            </div>
            <div class="flex justify-between">
              <span class="text-muted-foreground">{m.finance_pos_total_invoice()}</span>
              <span class="font-medium text-foreground font-mono">{formatCurrency(Number(selectedFee.total_amount || selectedFee.amount), selectedFee.currency)}</span>
            </div>
            <div class="flex justify-between border-t border-border/60 pt-1.5 mt-0.5">
              <span class="text-rose-500 font-semibold">{m.finance_pos_remaining_balance()}</span>
              <span class="font-bold text-rose-500 font-mono">{formatCurrency(Number(selectedFee.balance || selectedFee.amount), selectedFee.currency)}</span>
            </div>
          </div>

          <!-- Treasury Select -->
          <div class="flex flex-col gap-2">
            <label for="pos_treasury" class="text-xs font-bold text-muted-foreground uppercase tracking-wider">
              {m.common_payment_label_treasury()}
            </label>
            <select 
              id="pos_treasury"
              bind:value={selectedTreasuryId}
              class="bg-background text-foreground border border-border rounded-xl px-3 py-2 text-xs focus:ring-1 focus:ring-lime-500 outline-none h-10 w-full"
              disabled={isSubmitting}
            >
              {#each treasuries as t (t.id)}
                <option value={String(t.id)}>{t.name_ar || t.name || t.code} ({t.currency})</option>
              {/each}
            </select>
          </div>

          <!-- Payment Method -->
          <div class="flex flex-col gap-2">
            <span class="text-xs font-bold text-muted-foreground uppercase tracking-wider select-none">
              {m.common_payment_label_method()}
            </span>
            <div class="grid grid-cols-3 gap-3.5">
              {#each ['Cash', 'Bank Transfer', 'Card'] as method}
                <button
                  type="button"
                  class="h-14 rounded-xl border flex flex-col items-center justify-center gap-1.5 duration-100 font-semibold text-xs select-none {paymentMethod === method ? 'bg-lime-500/10 border-lime-500 text-foreground ring-1 ring-lime-500/40' : 'bg-background hover:bg-accent border-border text-muted-foreground'}"
                  onclick={() => paymentMethod = method as any}
                  disabled={isSubmitting}
                >
                  <span class="text-[10px]">
                    {#if method === 'Cash'}{m.finance_pos_cash()}
                    {:else if method === 'Bank Transfer'}{m.finance_pos_bank_transfer()}
                    {:else}{m.finance_pos_card()}{/if}
                  </span>
                </button>
              {/each}
            </div>
          </div>

          <!-- Currency Section -->
          <div class="flex flex-col gap-2">
            <label for="pos_currency" class="text-xs font-bold text-muted-foreground uppercase tracking-wider">
              {m.common_payment_label_currency()}
            </label>
            <div class="flex gap-3">
              <Input 
                id="pos_currency"
                type="text" 
                bind:value={currency}
                placeholder="e.g., YER"
                class="uppercase font-mono w-[90px] h-10 text-center font-bold"
                required
                maxlength={3}
                disabled={isSubmitting}
              />
              <div class="flex gap-2 items-center">
                {#each ['YER', 'USD', 'SAR'] as curr}
                  <button
                    type="button"
                    class="h-10 px-4 rounded-xl border text-xs font-bold transition-all {currency.toUpperCase() === curr ? 'bg-primary text-primary-foreground border-primary' : 'bg-background hover:bg-accent text-muted-foreground border-border'}"
                    onclick={() => currency = curr}
                    disabled={isSubmitting}
                  >
                    {curr}
                  </button>
                {/each}
              </div>
            </div>
          </div>

          <!-- Amount -->
          <div class="flex flex-col gap-2">
            <label for="pos_amount" class="text-xs font-bold text-muted-foreground uppercase tracking-wider">
              {m.common_payment_label_amount()} ({currency})
            </label>
            <div class="relative">
              <Input 
                id="pos_amount"
                type="number" 
                bind:value={amount}
                min="1"
                step="any"
                placeholder="0.00"
                class="h-10 font-bold font-mono"
                required
                disabled={isSubmitting}
              />
              <!-- Helper button to auto-fill full remaining balance -->
              {#if amount !== Number(selectedFee.balance)}
                <button
                  type="button"
                  onclick={() => amount = Number(selectedFee.balance)}
                  class="absolute right-3 top-1/2 -translate-y-1/2 text-[9px] bg-accent/80 hover:bg-accent border border-border px-2 py-1 rounded font-bold text-muted-foreground duration-100"
                  disabled={isSubmitting}
                >
                  {m.finance_pos_pay_full()}
                </button>
              {/if}
            </div>
          </div>

          <!-- Receipt & Ref details -->
          <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
            <!-- Receipt Number -->
            <div class="flex flex-col gap-2">
              <label for="pos_receipt" class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">
                {m.common_payment_label_receipt()}
              </label>
              <Input 
                id="pos_receipt"
                type="text" 
                bind:value={receiptNumber}
                placeholder="e.g., REC-104"
                class="h-10 text-xs"
                disabled={isSubmitting}
              />
            </div>
            
            <!-- Reference Number -->
            <div class="flex flex-col gap-2">
              <label for="pos_ref" class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">
                {m.common_payment_label_ref()}
              </label>
              <Input 
                id="pos_ref"
                type="text" 
                bind:value={referenceNumber}
                placeholder="e.g., TXN-998"
                class="h-10 text-xs"
                disabled={isSubmitting}
              />
            </div>
          </div>

          <!-- Notes -->
          <div class="flex flex-col gap-2">
            <label for="pos_notes" class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">
              {m.common_payment_label_notes()}
            </label>
            <Input 
              id="pos_notes"
              type="text" 
              bind:value={notes}
              placeholder={m.common_payment_placeholder_notes()}
              class="h-10 text-xs"
              disabled={isSubmitting}
            />
          </div>

          <!-- Form Submit Buttons -->
          <div class="flex gap-3 mt-auto pt-4 shrink-0 select-none">
            <Button 
              type="button" 
              variant="ghost" 
              class="w-1/3 h-11 text-xs border border-border"
              onclick={deselectStudent}
              disabled={isSubmitting}
            >
              {m.common_action_cancel()}
            </Button>
            <Button 
              type="submit" 
              variant="lime" 
              class="flex-1 h-11 text-xs font-bold"
              disabled={isSubmitting}
            >
              {#if isSubmitting}
                {m.common_loading()}
              {:else}
                <span class="flex items-center justify-center gap-1.5">
                  <CreditCard size={15} />
                  <span>{m.finance_pos_record_post_payment()}</span>
                </span>
              {/if}
            </Button>
          </div>
        </form>
      {/if}
    </div>

  </div>
</div>
