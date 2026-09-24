<!--
  @component StudentFees
  Assigned fees, balance due, and payment records for a student using OrbisGrid.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { api } from "$lib/api";
import { Skeleton } from "@orbis/ui/skeleton";
import { CheckCircle, Document, InfoCircle } from "@orbis/icons";
import { onMount } from "svelte";
import { getLocale } from "$lib/paraglide/runtime";
import { formatCurrency, formatDate } from "$lib/core/i18n";
import { OrbisGrid, type OrbisColumnDef } from "@orbis/svelte-grid";

let { orgId, studentId }: { orgId: string; studentId: string } = $props();

let fees = $state<any[]>([]);
let payments = $state<any[]>([]);
let isLoading = $state(true);

let totalFees = $derived(
  fees.reduce((s, f) => s + Number(f.net_amount || 0), 0),
);
let totalPaid = $derived(
  payments.reduce((s, p) => s + Number(p.amount || 0), 0),
);
let balanceDue = $derived(totalFees - totalPaid);

onMount(() => loadFinance());

async function loadFinance() {
  isLoading = true;
  try {
    if (!orgId || !studentId) return;
    const enrollments = await api.listEnrollments();
    const activeEnrollment = enrollments.find(
      (e: any) => e.student_id === studentId,
    );
    if (activeEnrollment) {
      const [f, p] = await Promise.all([
        api.listStudentFees(activeEnrollment.id),
        api.listPayments(),
      ]);
      fees = f;
      payments = p;
    }
  } catch (e) {
    console.error(e);
  } finally {
    isLoading = false;
  }
}

// ── Grid Sources ────────────────────────────────────────────────────────

const feesSource = $derived(
  fees.map((f, i) => {
    const paidAmount = payments
      .filter((p) => p.fee_id === f.id)
      .reduce((sum, p) => sum + Number(p.amount || 0), 0);
    const discountAmount = Number(f.discount_amount || 0);
    const originalAmount = Number(f.original_amount || 0);
    const balance = Math.max(0, Number(f.net_amount || 0) - paidAmount);

    let displayStatus = f.status || "unpaid";
    if (displayStatus === "partially_paid") {
      displayStatus = "partial";
    }

    return {
      ...f,
      __idx: i + 1,
      fee_name: f.notes || m.common_ft_category_tuition(),
      total: formatCurrency(originalAmount),
      paid: formatCurrency(paidAmount),
      discount: formatCurrency(discountAmount),
      balance: formatCurrency(balance),
      status: displayStatus,
    };
  }),
);

const feesCols: OrbisColumnDef[] = [
  { id: "__idx", prop: "__idx", header: "#", name: "#", width: 48, size: 48, readonly: true, pin: "left", align: "center" },
  { id: "fee_name", prop: "fee_name", header: m.finance_fees_col_description(), name: "Fee", width: 180, size: 180, readonly: true },
  { id: "total", prop: "total", header: m.common_ft_col_amount(), name: "Total", width: 110, size: 110, readonly: true, align: "end", cellClass: "font-mono" },
  { id: "paid", prop: "paid", header: m.finance_fees_total_collected(), name: "Paid", width: 110, size: 110, readonly: true, align: "end", cellClass: "font-mono text-emerald-600" },
  { id: "balance", prop: "balance", header: m.finance_fees_total_due(), name: "Balance", width: 110, size: 110, readonly: true, align: "end", cellClass: "font-mono font-bold text-rose-600" },
  { id: "status", prop: "status", header: "Status", name: "Status", width: 100, size: 100, type: "badge", align: "center", readonly: true },
];

const paymentsCols: OrbisColumnDef[] = [
  { id: "__idx", prop: "__idx", header: "#", name: "#", width: 48, size: 48, readonly: true, pin: "left", align: "center" },
  { id: "receipt", prop: "receipt", header: m.finance_fees_col_receipt(), name: "Receipt", width: 120, size: 120, readonly: true, cellClass: "font-mono" },
  { id: "method", prop: "method", header: m.finance_fees_col_method(), name: "Method", width: 110, size: 110, readonly: true, align: "center" },
  { id: "date", prop: "date", header: m.finance_fees_col_pay_date(), name: "Date", width: 120, size: 120, readonly: true, cellClass: "font-mono" },
  { id: "amount", prop: "amount", header: m.finance_payments_col_amount(), name: "Amount", width: 120, size: 120, readonly: true, align: "end", cellClass: "font-mono font-bold text-emerald-600" },
];

const paymentsSource = $derived(
  payments.map((p, i) => ({
    ...p,
    __idx: i + 1,
    receipt: `#${p.receipt_number || p.id?.slice(0, 8)}`,
    method: p.method || m.finance_pos_cash(),
    date: formatDate(p.created_at) || "—",
    amount: formatCurrency(Number(p.amount || 0)),
  })),
);

const rtl = $derived(getLocale() === "ar");
</script>

<div class="flex flex-col gap-6 text-xs text-start font-sans">
  <!-- Summary Statistics -->
  <div class="grid grid-cols-3 gap-3 shrink-0">
    <div class="border border-border bg-card/45 p-4 rounded-xl flex items-center justify-between shadow-sm">
      <div class="flex flex-col">
        <span class="text-[9px] text-muted-foreground">{m.finance_fees_total_fees()}</span>
        <span class="text-sm font-bold mt-0.5 text-foreground">{formatCurrency(totalFees)}</span>
      </div>
      <Document size={16} class="text-primary" />
    </div>
    <div class="border border-border bg-card/45 p-4 rounded-xl flex items-center justify-between shadow-sm">
      <div class="flex flex-col">
        <span class="text-[9px] text-muted-foreground">{m.finance_fees_total_paid()}</span>
        <span class="text-sm font-bold mt-0.5 text-emerald-500">{formatCurrency(totalPaid)}</span>
      </div>
      <CheckCircle size={16} class="text-emerald-500" />
    </div>
    <div class="border border-border bg-card/45 p-4 rounded-xl flex items-center justify-between shadow-sm">
      <div class="flex flex-col">
        <span class="text-[9px] text-muted-foreground">{m.finance_fees_balance_due()}</span>
        <span class="text-sm font-bold mt-0.5" class:text-destructive={balanceDue > 0} class:text-emerald-500={balanceDue <= 0}>
          {formatCurrency(balanceDue)}
        </span>
      </div>
      <InfoCircle size={16} class={balanceDue > 0 ? 'text-destructive' : 'text-emerald-500'} />
    </div>
  </div>

  {#if isLoading}
    <Skeleton class="h-40 w-full rounded-xl" />
    <Skeleton class="h-40 w-full rounded-xl" />
  {:else}
    <!-- Assigned Fees -->
    <div class="flex flex-col gap-2">
      <span class="font-bold text-[10px] text-muted-foreground uppercase tracking-wider px-1">{m.finance_fees_assigned_title()}</span>
      <div class="h-[200px]">
        <OrbisGrid
          columns={feesCols}
          source={feesSource}
          emptyTitle={m.finance_fees_no_fees()}
          {rtl}
          readonly
        />
      </div>
    </div>

    <!-- Payments List -->
    <div class="flex flex-col gap-2">
      <span class="font-bold text-[10px] text-muted-foreground uppercase tracking-wider px-1">{m.finance_fees_payments_title()}</span>
      <div class="h-[200px]">
        <OrbisGrid
          columns={paymentsCols}
          source={paymentsSource}
          emptyTitle={m.finance_fees_no_payments()}
          {rtl}
          readonly
        />
      </div>
    </div>
  {/if}
</div>
