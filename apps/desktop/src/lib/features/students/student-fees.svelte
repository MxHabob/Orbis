<!--
  @component StudentFees
  Assigned fees, balance due, and payment records for a student using OrbisGrid inside standardized Cards.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { api } from "$lib/api";
import { Skeleton } from "@orbis/ui/skeleton";
import {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardAction,
  CardContent,
} from "@orbis/ui/card";
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
  fees.reduce((s, f) => s + Number(f.paid_amount || 0), 0),
);
let balanceDue = $derived(
  fees.reduce(
    (s, f) =>
      s +
      Number(
        f.balance_due ?? Number(f.net_amount || 0) - Number(f.paid_amount || 0),
      ),
    0,
  ),
);

onMount(() => loadFinance());

async function loadFinance() {
  isLoading = true;
  try {
    if (!orgId || !studentId) return;
    const [f, p] = await Promise.all([
      api.listStudentFees({ student_id: studentId }),
      api.listPayments({ student_id: studentId }),
    ]);
    fees = f;
    payments = p;
  } catch (e) {
    console.error(e);
  } finally {
    isLoading = false;
  }
}

// ── Grid Sources ────────────────────────────────────────────────────────

const feesSource = $derived(
  fees.map((f, i) => {
    const paidAmount = Number(f.paid_amount ?? 0);
    const discountAmount = Number(f.discount_amount || 0);
    const originalAmount = Number(f.original_amount || 0);
    const balance = Number(
      f.balance_due ?? Math.max(0, Number(f.net_amount || 0) - paidAmount),
    );

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
    id: "fee_name",
    prop: "fee_name",
    header: m.finance_fees_col_description(),
    name: "Fee",
    width: 180,
    size: 180,
    readonly: true,
  },
  {
    id: "total",
    prop: "total",
    header: m.common_ft_col_amount(),
    name: "Total",
    width: 110,
    size: 110,
    readonly: true,
    align: "end",
    cellClass: "font-mono font-medium",
  },
  {
    id: "paid",
    prop: "paid",
    header: m.finance_fees_total_collected(),
    name: "Paid",
    width: 110,
    size: 110,
    readonly: true,
    align: "end",
    cellClass: "font-mono text-emerald-600 font-medium",
  },
  {
    id: "balance",
    prop: "balance",
    header: m.finance_fees_total_due(),
    name: "Balance",
    width: 110,
    size: 110,
    readonly: true,
    align: "end",
    cellClass: "font-mono font-bold text-rose-600",
  },
  {
    id: "status",
    prop: "status",
    header: "Status",
    name: "Status",
    width: 100,
    size: 100,
    type: "badge",
    align: "center",
    readonly: true,
  },
];

const paymentsCols: OrbisColumnDef[] = [
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
    id: "receipt",
    prop: "receipt",
    header: m.finance_fees_col_receipt(),
    name: "Receipt",
    width: 120,
    size: 120,
    readonly: true,
    cellClass: "font-mono font-bold",
  },
  {
    id: "method",
    prop: "method",
    header: m.finance_fees_col_method(),
    name: "Method",
    width: 110,
    size: 110,
    readonly: true,
    align: "center",
  },
  {
    id: "date",
    prop: "date",
    header: m.finance_fees_col_pay_date(),
    name: "Date",
    width: 120,
    size: 120,
    readonly: true,
    cellClass: "font-mono",
  },
  {
    id: "amount",
    prop: "amount",
    header: m.finance_payments_col_amount(),
    name: "Amount",
    width: 120,
    size: 120,
    readonly: true,
    align: "end",
    cellClass: "font-mono font-bold text-emerald-600",
  },
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
  <!-- Summary Statistics Cards -->
  <div class="grid grid-cols-1 sm:grid-cols-3 gap-3 shrink-0">
    <Card class="p-4 flex items-center justify-between">
      <div class="flex flex-col">
        <span class="text-[9px] text-muted-foreground font-semibold">{m.finance_fees_total_fees()}</span>
        <span class="text-sm font-bold font-mono mt-0.5 text-foreground">{formatCurrency(totalFees)}</span>
      </div>
      <Document size={18} class="text-primary" />
    </Card>

    <Card class="p-4 flex items-center justify-between">
      <div class="flex flex-col">
        <span class="text-[9px] text-muted-foreground font-semibold">{m.finance_fees_total_paid()}</span>
        <span class="text-sm font-bold font-mono mt-0.5 text-emerald-600 dark:text-emerald-400">{formatCurrency(totalPaid)}</span>
      </div>
      <CheckCircle size={18} class="text-emerald-500" />
    </Card>

    <Card class="p-4 flex items-center justify-between">
      <div class="flex flex-col">
        <span class="text-[9px] text-muted-foreground font-semibold">{m.finance_fees_balance_due()}</span>
        <span class="text-sm font-bold font-mono mt-0.5 {balanceDue > 0 ? 'text-destructive' : 'text-emerald-600 dark:text-emerald-400'}">
          {formatCurrency(balanceDue)}
        </span>
      </div>
      <InfoCircle size={18} class={balanceDue > 0 ? 'text-destructive' : 'text-emerald-500'} />
    </Card>
  </div>

  {#if isLoading}
    <Skeleton class="h-44 w-full rounded-xl" />
    <Skeleton class="h-44 w-full rounded-xl" />
  {:else}
    <!-- Assigned Fees Card -->
    <Card class="overflow-hidden">
      <CardHeader class="py-2.5 px-4">
        <CardTitle class="text-xs font-bold uppercase tracking-wider text-foreground">
          {m.finance_fees_assigned_title()}
        </CardTitle>
      </CardHeader>
      <CardContent class="p-0">
        <div class="h-[200px] w-full">
          <OrbisGrid
            columns={feesCols}
            source={feesSource}
            emptyTitle={m.finance_fees_no_fees()}
            {rtl}
            readonly
          />
        </div>
      </CardContent>
    </Card>

    <!-- Payments List Card -->
    <Card class="overflow-hidden">
      <CardHeader class="py-2.5 px-4">
        <CardTitle class="text-xs font-bold uppercase tracking-wider text-foreground">
          {m.finance_fees_payments_title()}
        </CardTitle>
      </CardHeader>
      <CardContent class="p-0">
        <div class="h-[200px] w-full">
          <OrbisGrid
            columns={paymentsCols}
            source={paymentsSource}
            emptyTitle={m.finance_fees_no_payments()}
            {rtl}
            readonly
          />
        </div>
      </CardContent>
    </Card>
  {/if}
</div>
