<!--
@component StudentFeesPage
Assigned fees + payment history using OrbisGrid, with summary statistics.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { page } from "$app/state";
import { api, } from "$lib/api";
import { Skeleton } from "@orbis/ui/skeleton";
import { CreditCard, Wallet, CheckCircle2, AlertCircle } from "@lucide/svelte";
import { onMount } from "svelte";
import { getLocale } from "$lib/paraglide/runtime";

import { formatCurrency, formatDate } from "$lib/core/i18n";

import { DataTable } from "$lib/tables";
import { studentFees as sfDef, withIndex } from "$lib/tables/definitions";
import type { OrbisColumnDef } from "@orbis/svelte-grid";

const orgId = $derived(page.params.orgId);
const studentId = $derived(page.params.studentId);

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
        api.listPayments(null, null),
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
  withIndex(
    fees.map((f) => {
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
        fee_name: f.notes || m.common_ft_category_tuition(),
        total: formatCurrency(originalAmount),
        paid: formatCurrency(paidAmount),
        discount: formatCurrency(discountAmount),
        balance: formatCurrency(balance),
        status: displayStatus,
      };
    }),
  ),
);

const feesCols = $derived(
  sfDef.columns({
    idx: "#",
    fee: m.finance_fees_col_description(),
    total: m.common_ft_col_amount(),
    status: m.finance_fees_col_date(),
  }),
);

const paymentsCols: OrbisColumnDef[] = [
  {
    prop: "__idx",
    name: "#",
    size: 48,
    readonly: true,
    pin: "colPinStart",
    cellProperties: () => ({ class: "orbis-cell-index" }),
  },
  {
    prop: "receipt",
    name: m.finance_fees_col_receipt(),
    size: 120,
    readonly: true,
    pin: "colPinStart",
    cellProperties: () => ({ class: "orbis-cell-student" }),
  },
  {
    prop: "method",
    name: m.finance_fees_col_method(),
    size: 110,
    readonly: true,
  },
  {
    prop: "date",
    name: m.finance_fees_col_pay_date(),
    size: 110,
    readonly: true,
  },
  {
    prop: "amount",
    name: m.finance_payments_col_amount(),
    size: 110,
    readonly: true,
    cellProperties: () => ({ class: "orbis-cell-success orbis-cell-number" }),
  },
];

const paymentsSource = $derived(
  withIndex(
    payments.map((p) => ({
      ...p,
      receipt: `#${p.receipt_number || p.id?.slice(0, 8)}`,
      method: p.method || m.finance_pos_cash(),
      date: formatDate(p.created_at) || "—",
      amount: formatCurrency(Number(p.amount || 0)),
    })),
  ),
);

const rtl = $derived(getLocale() === "ar");
</script>

<div class="flex flex-col gap-6 text-xs text-start">
  <!-- Summary Statistics -->
  <div class="grid grid-cols-3 gap-3 shrink-0">
    <div class="border border-border bg-card/45 p-4 rounded-xl flex items-center justify-between shadow-sm">
      <div class="flex flex-col">
        <span class="text-[9px] text-muted-foreground">{m.finance_fees_total_fees()}</span>
        <span class="text-sm font-bold mt-0.5 text-foreground">{formatCurrency(totalFees)}</span>
      </div>
      <Wallet size={16} class="text-primary" />
    </div>
    <div class="border border-border bg-card/45 p-4 rounded-xl flex items-center justify-between shadow-sm">
      <div class="flex flex-col">
        <span class="text-[9px] text-muted-foreground">{m.finance_fees_total_paid()}</span>
        <span class="text-sm font-bold mt-0.5 text-emerald-500">{formatCurrency(totalPaid)}</span>
      </div>
      <CheckCircle2 size={16} class="text-emerald-500" />
    </div>
    <div class="border border-border bg-card/45 p-4 rounded-xl flex items-center justify-between shadow-sm">
      <div class="flex flex-col">
        <span class="text-[9px] text-muted-foreground">{m.finance_fees_balance_due()}</span>
        <span class="text-sm font-bold mt-0.5" class:text-destructive={balanceDue > 0} class:text-emerald-500={balanceDue <= 0}>
          {formatCurrency(balanceDue)}
        </span>
      </div>
      <AlertCircle size={16} class={balanceDue > 0 ? 'text-destructive' : 'text-emerald-500'} />
    </div>
  </div>

  {#if isLoading}
    <Skeleton class="h-40 w-full rounded-xl" />
    <Skeleton class="h-40 w-full rounded-xl" />
  {:else}
    <!-- Assigned Fees -->
    <div class="flex flex-col gap-2">
      <span class="font-bold text-[10px] text-muted-foreground uppercase tracking-wider px-1">{m.finance_fees_assigned_title()}</span>
      <DataTable
        columns={feesCols}
        source={feesSource}
        emptyIcon={Wallet}
        emptyTitle={m.finance_fees_no_fees()}
        {rtl}
        height="200px"
      />
    </div>

    <!-- Payments History -->
    <div class="flex flex-col gap-2">
      <span class="font-bold text-[10px] text-muted-foreground uppercase tracking-wider px-1">{m.finance_fees_payments_title()}</span>
      <DataTable
        columns={paymentsCols}
        source={paymentsSource}
        emptyIcon={CreditCard}
        emptyTitle={m.finance_fees_no_payments()}
        {rtl}
        height="200px"
      />
    </div>
  {/if}
</div>
