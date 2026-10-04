<!--
  @component PaymentsManager
  Minimal, elegant, and high-performance Receipt Vouchers (سندات القبض) workspace.
  Adheres strictly to Orbis Clean Architecture and the FinanceOverview design system:
  - Clean card-based payment voucher entry
  - Quick student and unpaid fee allocation
  - Sleek top toolbar with search, method filter, and export
  - High-performance OrbisGrid table
  - Real backend payment recording and printable receipts
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import { api, extractErrorMessage } from "$lib/api";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
import { Card, CardHeader, CardTitle, CardContent } from "@orbis/ui/card";
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
import { OrbisGrid, type OrbisColumnDef } from "@orbis/svelte-grid";
import SearchableSelect from "$lib/shared/components/searchable-select.svelte";
import PdfReportViewer from "$lib/shared/components/pdf-report-viewer.svelte";
import { formatCurrency, formatDate, isAr } from "$lib/core/i18n";
import { fade, slide } from "svelte/transition";

import {
  createAvailablePaymentMethodsQuery,
  type AvailablePaymentMethod,
} from "$lib/features/finance";
    import CurrencySelector from "$lib/shared/components/currency-selector.svelte";

let { orgId }: { orgId: string } = $props();

// Dynamic payment methods from Integration Layer
const paymentMethodsQuery = createAvailablePaymentMethodsQuery(() => orgId);
const availablePaymentMethods = $derived(paymentMethodsQuery.data ?? []);

// ── State ──────────────────────────────────────────────────────────────
let payments = $state<any[]>([]);
let treasuries = $state<any[]>([]);
let students = $state<any[]>([]);
let isLoading = $state(true);
let isSubmitting = $state(false);
let showAddForm = $state(false);

// Form Fields
let receiptNumber = $state("");
let paymentDate = $state(new Date().toISOString().split("T")[0]);
let selectedStudentId = $state("");
let selectedTreasuryId = $state("");
let paymentMethod = $state("cash");
let selectedCurrency = $state("YER");
let checkInfo = $state("");
let sourceToken = $state("");
let notes = $state("");
let generalAmount = $state<number | null>(null);

const selectedPaymentMethodObj = $derived(
  availablePaymentMethods.find((m) => m.id === paymentMethod) || availablePaymentMethods[0]
);

// Student Fees for Allocation
let studentFees = $state<any[]>([]);
let isLoadingFees = $state(false);
let selectedFeeIds = $state<Set<string>>(new Set());
let feeAmounts = $state<Record<string, number>>({});

// Filters
let searchQuery = $state("");
let methodFilter = $state("all");
let filterDate = $state("");

// Print
let printDialogOpen = $state(false);
let printParameters = $state<Record<string, any>>({});
let printTitle = $state(m.finance_ui_print_receipt());

onMount(async () => {
  await loadData();
});

async function loadData() {
  isLoading = true;
  try {
    const [paymentsList, cashAccounts, enrollmentsList] = await Promise.all([
      api.listPayments().catch(() => []),
      api.listAccounts().catch(() => []),
      api.listEnrollments ? api.listEnrollments().catch(() => []) : Promise.resolve([]),
    ]);

    payments = paymentsList || [];
    treasuries = (cashAccounts || []).filter(
      (a: any) =>
        a.type === "asset" ||
        a.account_type === "asset" ||
        a.sub_type === "cash" ||
        a.sub_type === "bank"
    );

    students = (enrollmentsList || []).map((e: any) => ({
      id: String(e.id),
      full_name: e.full_name || e.name_ar || e.name_en || e.student_number || m.finance_ui_student_number_fallback({ id: e.id }),
      student_number: e.student_number || "",
    }));

    if (treasuries.length > 0 && !selectedTreasuryId) {
      selectedTreasuryId = treasuries[0].id;
    }
  } catch (e) {
    console.error("Failed to load payments data:", e);
    toast.error(m.finance_ui_payments_load_failed());
  } finally {
    isLoading = false;
  }
}

async function handleStudentChange(studentId: string) {
  selectedStudentId = studentId;
  selectedFeeIds = new Set();
  feeAmounts = {};
  studentFees = [];

  if (!studentId) return;

  isLoadingFees = true;
  try {
    const list = await api.listStudentFees(studentId);
    studentFees = (list || []).filter((f: any) => f.status !== "paid");
    // Pre-select first unpaid fee if available
    if (studentFees.length > 0) {
      const first = studentFees[0];
      const fid = String(first.id);
      selectedFeeIds.add(fid);
      feeAmounts[fid] = Number(first.balance_due ?? first.balance ?? first.net_amount ?? 0);
      selectedFeeIds = new Set(selectedFeeIds);
    }
  } catch (e) {
    console.error("Failed to load student fees:", e);
  } finally {
    isLoadingFees = false;
  }
}

function toggleFee(fee: any) {
  const fid = String(fee.id);
  if (selectedFeeIds.has(fid)) {
    selectedFeeIds.delete(fid);
    delete feeAmounts[fid];
  } else {
    selectedFeeIds.add(fid);
    feeAmounts[fid] = Number(fee.balance_due ?? fee.balance ?? fee.net_amount ?? 0);
  }
  selectedFeeIds = new Set(selectedFeeIds);
}

const calculatedTotal = $derived.by(() => {
  if (selectedFeeIds.size > 0) {
    let sum = 0;
    for (const fid of selectedFeeIds) {
      sum += Number(feeAmounts[fid] || 0);
    }
    return sum;
  }
  return Number(generalAmount || 0);
});

function clearForm() {
  receiptNumber = "";
  paymentDate = new Date().toISOString().split("T")[0];
  selectedStudentId = "";
  paymentMethod = "Cash";
  selectedCurrency = "YER";
  checkInfo = "";
  notes = "";
  generalAmount = null;
  selectedFeeIds = new Set();
  feeAmounts = {};
  studentFees = [];
}

async function handleSavePayment() {
  if (!selectedStudentId) {
    toast.error(m.finance_ui_student_required());
    return;
  }
  if (!selectedTreasuryId) {
    toast.error(m.finance_ui_treasury_required());
    return;
  }

  const total = calculatedTotal;
  if (!total || total <= 0) {
    toast.error(m.finance_ui_payment_amount_invalid());
    return;
  }

  isSubmitting = true;
  try {
    const selectedStudent = students.find((s) => s.id === selectedStudentId);
    if (selectedFeeIds.size > 0) {
      // Create payment for each allocated fee
      for (const feeId of selectedFeeIds) {
        const amt = feeAmounts[feeId];
        if (amt && amt > 0) {
          await api.processPayment({
            fee_id: feeId,
            student_id: selectedStudentId,
            account_id: selectedTreasuryId,
            amount: amt,
            method: paymentMethod,
            reference: (checkInfo || receiptNumber).trim() || undefined,
            source: sourceToken.trim() || undefined,
            payer_name: selectedStudent?.full_name || undefined,
          });
        }
      }
    } else {
      // General payment without specific fee allocation
      await api.processPayment({
        student_id: selectedStudentId,
        account_id: selectedTreasuryId,
        amount: total,
        method: paymentMethod,
        reference: (checkInfo || receiptNumber).trim() || undefined,
        source: sourceToken.trim() || undefined,
        payer_name: selectedStudent?.full_name || undefined,
      });
    }

    toast.success(m.finance_ui_receipt_posted_success());
    clearForm();
    showAddForm = false;
    await loadData();
  } catch (err: any) {
    console.error("Failed to record payment:", err);
    toast.error(extractErrorMessage(err) || m.finance_ui_receipt_save_failed());
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
const gridSource = $derived(
  payments.map((p, idx) => {
    const student = students.find((s) => String(s.id) === String(p.enrollment_id));
    const studentName = p.student_name || student?.full_name || "—";
    const treasury = treasuries.find((t) => String(t.id) === String(p.cash_account_id));
    const treasuryName = treasury ? (treasury.name_ar || treasury.name || m.finance_ui_treasury()) : m.finance_ui_main_treasury();
    const amt = Number(p.amount || 0);
    const curr = p.currency || "YER";

    return {
      ...p,
      __idx: idx + 1,
      receipt_display: p.receipt_number || p.id.slice(0, 8),
      date_display: p.payment_date ? formatDate(p.payment_date) : (p.created_at ? formatDate(p.created_at) : "—"),
      student_display: studentName,
      treasury_display: treasuryName,
      amount_formatted: formatCurrency(amt, curr),
      currency_display: curr,
      method_display: p.payment_method === "Bank" ? m.finance_ui_payment_method_bank_transfer() : (p.payment_method === "Check" ? m.finance_ui_payment_method_check() : m.finance_ui_payment_method_cash()),
      notes_display: p.notes || "—",
      status_display: m.finance_ui_receipt_status(),
    };
  })
);

const filteredSource = $derived(
  gridSource.filter((row) => {
    if (methodFilter !== "all" && row.payment_method?.toLowerCase() !== methodFilter.toLowerCase()) {
      return false;
    }
    if (filterDate && row.payment_date && !String(row.payment_date).startsWith(filterDate)) {
      return false;
    }
    return true;
  })
);

const columns: OrbisColumnDef[] = [
  { id: "__idx", prop: "__idx", header: "#", name: "#", width: 48, size: 48, readonly: true, pin: "left", align: "center" },
  { id: "receipt_display", prop: "receipt_display", header: m.finance_ui_voucher_number(), name: m.finance_ui_voucher_number(), width: 110, size: 110, readonly: true, cellClass: "font-mono font-bold" },
  { id: "date_display", prop: "date_display", header: m.finance_ui_date(), name: m.finance_ui_date(), width: 110, size: 110, readonly: true, align: "center" },
  { id: "student_display", prop: "student_display", header: m.finance_ui_student_name(), name: m.finance_ui_student_name(), width: 220, size: 220, readonly: true, cellClass: "font-medium" },
  { id: "treasury_display", prop: "treasury_display", header: m.finance_ui_receiving_account(), name: m.finance_ui_receiving_account(), width: 180, size: 180, readonly: true },
  { id: "amount_formatted", prop: "amount_formatted", header: m.finance_ui_received_amount(), name: m.finance_ui_received_amount(), width: 140, size: 140, readonly: true, align: "end", cellClass: "font-mono font-bold text-emerald-600 dark:text-emerald-400" },
  { id: "currency_display", prop: "currency_display", header: m.finance_ui_currency(), name: m.finance_ui_currency(), width: 80, size: 80, readonly: true, align: "center" },
  { id: "method_display", prop: "method_display", header: m.finance_ui_payment_method(), name: m.finance_ui_payment_method(), width: 110, size: 110, readonly: true, align: "center" },
  { id: "notes_display", prop: "notes_display", header: m.finance_ui_description_notes(), name: m.finance_ui_description_notes(), width: 220, size: 220, readonly: true },
  { id: "status_display", prop: "status_display", header: m.finance_ui_status(), name: m.finance_ui_status(), width: 90, size: 90, type: "badge", align: "center", readonly: true },
];

function exportCsv() {
  const rows = filteredSource.map((p) => [
    p.receipt_display,
    p.date_display,
    p.student_display,
    p.treasury_display,
    p.amount_formatted,
    p.currency_display,
    p.method_display,
    p.notes_display,
    p.status_display,
  ]);
  const csv = [
    [m.finance_ui_receipt(), m.finance_ui_date(), m.finance_ui_student_name(), m.finance_ui_treasury(), m.finance_ui_amount(), m.finance_ui_currency(), m.finance_ui_payment_method(), m.finance_ui_notes(), m.finance_ui_status()],
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
            bind:value={searchQuery}
            class="h-8 ps-8 text-xs bg-card border-border/60"
          />
        </div>

        <select
          bind:value={methodFilter}
          class="h-8 text-xs rounded-md bg-card border border-border/60 px-2 text-foreground focus:outline-none"
        >
          <option value="all">{m.finance_ui_all_payment_methods()}</option>
          {#each availablePaymentMethods as m (m.id)}
            <option value={m.id}>{m.name}</option>
          {/each}
        </select>

        <Input
          type="date"
          bind:value={filterDate}
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

    <!-- In-Page Creation Card (Matching FinanceOverview Card Style) -->
    {#if showAddForm}
      <div transition:slide={{ duration: 200 }} class="shrink-0">
        <Card class="border border-primary/20 bg-card shadow-sm text-start">
          <CardHeader class="py-3 px-4 border-b border-border/40">
            <div class="flex items-center justify-between">
              <div class="flex items-center gap-2">
                <span class="h-2 w-2 rounded-full bg-emerald-500"></span>
                <CardTitle class="text-xs font-bold uppercase tracking-wider text-foreground">
                  {m.finance_ui_receipt_create_heading()}
                </CardTitle>
              </div>
              <Button variant="ghost" size="icon-xs" onclick={() => showAddForm = false} class="text-muted-foreground">
                <XMark size={14} />
              </Button>
            </div>
          </CardHeader>

          <CardContent class="p-4 flex flex-col gap-4">
            <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
              <div>
                <Label class="text-xs font-semibold text-foreground mb-1 block">{m.finance_ui_student()}</Label>
                <SearchableSelect
                  value={selectedStudentId}
                  placeholder={m.finance_ui_student_select_placeholder()}
                  searchPlaceholder={m.finance_ui_student_search_placeholder()}
                  items={students}
                  getItemValue={(s) => s.id}
                  getItemLabel={(s) => s.full_name}
                  onchange={(val) => { if (val) handleStudentChange(val); }}
                  class="w-full h-8 text-xs bg-background border-border/60"
                />
              </div>

              <div>
                <Label class="text-xs font-semibold text-foreground mb-1 block">{m.finance_ui_receiving_account()}</Label>
                <SearchableSelect
                  value={selectedTreasuryId}
                  placeholder={m.finance_ui_treasury_select_placeholder()}
                  items={treasuries}
                  getItemValue={(t) => t.id}
                  getItemLabel={(t) => `${t.name_ar || t.name} (${t.currency || "YER"})`}
                  onchange={(val) => { if (val) selectedTreasuryId = val; }}
                  class="w-full h-8 text-xs bg-background border-border/60"
                />
              </div>

              <div>
                <div class="flex items-center justify-between mb-1">
                  <Label class="text-xs font-semibold text-foreground">{m.finance_ui_payment_method()}</Label>
                  {#if selectedPaymentMethodObj?.is_digital_gateway}
                    <span class="text-[9px] text-primary font-medium bg-primary/10 px-1.5 py-0.5 rounded">
                      {m.finance_ui_gateway_provider({ provider: selectedPaymentMethodObj.provider_name ?? "" })}
                    </span>
                  {/if}
                </div>
                <select
                  bind:value={paymentMethod}
                  class="w-full h-8 text-xs rounded-md bg-background border border-border/60 px-2 text-foreground focus:outline-none"
                >
                  {#each availablePaymentMethods as method (method.id)}
                    <option value={method.id}>
                      {method.name} {method.is_digital_gateway ? m.finance_ui_gateway_provider({ provider: method.provider_name ?? "" }) : ""}
                    </option>
                  {/each}
                </select>
              </div>
            </div>

            {#if selectedPaymentMethodObj?.requires_source_token}
              <div transition:slide={{ duration: 120 }}>
                <Label class="text-xs font-semibold text-foreground mb-1 block">{m.finance_ui_payment_gateway_token_label()}</Label>
                <Input
                  type="text"
                  bind:value={sourceToken}
                  placeholder={m.finance_ui_payment_gateway_token_placeholder()}
                  class="h-8 text-xs font-mono bg-background border-border/60"
                />
              </div>
            {/if}

            <div class="grid grid-cols-1 md:grid-cols-4 gap-4">
              <div>
                <Label class="text-xs font-semibold text-foreground mb-1 block">{m.finance_ui_voucher_number_optional()}</Label>
                <Input
                  type="text"
                  bind:value={receiptNumber}
                  placeholder={m.finance_ui_automatic()}
                  class="h-8 text-xs font-mono bg-background border-border/60"
                />
              </div>

              <div>
                <Label class="text-xs font-semibold text-foreground mb-1 block">{m.finance_ui_date()}</Label>
                <Input
                  type="date"
                  bind:value={paymentDate}
                  class="h-8 text-xs font-mono bg-background border-border/60"
                />
              </div>

              <div>
                <Label class="text-xs font-semibold text-foreground mb-1 block">{m.finance_ui_currency()}</Label>
                <CurrencySelector/>
              </div>

              <div>
                <Label class="text-xs font-semibold text-foreground mb-1 block">{m.finance_ui_total_received_amount()}</Label>
                <Input
                  type="number"
                  value={calculatedTotal}
                  oninput={(e) => { generalAmount = Number((e.target as HTMLInputElement).value); }}
                  placeholder="0.00"
                  class="h-8 text-xs font-mono font-bold text-emerald-600 dark:text-emerald-400 bg-background border-border/60"
                  readonly={selectedFeeIds.size > 0}
                />
              </div>
            </div>

            <!-- Unpaid Fees Allocation Table (if student selected) -->
            {#if selectedStudentId && studentFees.length > 0}
              <div class="rounded-lg border border-border/50 bg-background/50 p-3 flex flex-col gap-2">
                <span class="text-[11px] font-bold text-foreground">{m.finance_ui_allocate_student_fees()}</span>
                <div class="space-y-1.5 max-h-40 overflow-y-auto">
                  {#each studentFees as fee (fee.id)}
                    {@const fid = String(fee.id)}
                    {@const isChecked = selectedFeeIds.has(fid)}
                    <div class="flex items-center justify-between p-2 rounded-md border border-border/40 bg-card text-xs">
                      <label class="flex items-center gap-2 cursor-pointer select-none">
                        <input
                          type="checkbox"
                          checked={isChecked}
                          onchange={() => toggleFee(fee)}
                          class="rounded border-border text-primary focus:ring-0"
                        />
                        <span class="font-medium text-foreground">{fee.fee_type_name || fee.notes || m.finance_ui_tuition_fees()}</span>
                      </label>

                      <div class="flex items-center gap-3">
                        <span class="text-muted-foreground font-mono text-[11px]">
                          {m.finance_ui_due_amount()} {formatCurrency(Number(fee.balance_due ?? fee.balance ?? fee.net_amount ?? 0), selectedCurrency)}
                        </span>
                        {#if isChecked}
                          <Input
                            type="number"
                            bind:value={feeAmounts[fid]}
                            class="h-7 w-28 text-xs font-mono font-bold text-end bg-background border-border"
                          />
                        {/if}
                      </div>
                    </div>
                  {/each}
                </div>
              </div>
            {/if}

            <div>
              <Label class="text-xs font-semibold text-foreground mb-1 block">{m.finance_ui_description_notes()}</Label>
              <Input
                type="text"
                bind:value={notes}
                placeholder={m.finance_ui_receipt_notes_placeholder()}
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
                onclick={handleSavePayment}
              >
                {isSubmitting ? m.finance_ui_saving() : m.finance_ui_receipt_save()}
              </Button>
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
        emptyTitle={m.finance_ui_no_receipts()}
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
