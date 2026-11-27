<!--
  @component StudentFees
  Assigned fees, balance due, and payment records for a student.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import {
  extractErrorMessage,
  type PaymentDto,
  type StudentFeeDto,
} from "$lib/api";
import { CheckCircle, Document, InfoCircle } from "@orbis/icons";
import { Button } from "@orbis/ui/button";
import StudentMetricCard from "$lib/features/students/components/student-metric-card.svelte";
import StudentSectionState from "$lib/features/students/components/student-section-state.svelte";
import {
  createStudentFeesQuery,
  createStudentPaymentsQuery,
} from "$lib/features/students/queries";
import { getLocale } from "$lib/paraglide/runtime";
import { formatCurrency } from "$lib/core/i18n";
import { OrbisGrid, type OrbisColumnDef } from "@orbis/svelte-grid";

let { orgId, studentId }: { orgId: string; studentId: string } = $props();

const feesQuery = createStudentFeesQuery(() => studentId);
const paymentsQuery = createStudentPaymentsQuery(() => studentId);

const fees = $derived<StudentFeeDto[]>(feesQuery.data ?? []);
const payments = $derived<PaymentDto[]>(paymentsQuery.data ?? []);
const isLoading = $derived(feesQuery.isLoading || paymentsQuery.isLoading);
const isError = $derived(feesQuery.isError || paymentsQuery.isError);
const errorMessage = $derived(
  feesQuery.error
    ? extractErrorMessage(feesQuery.error)
    : paymentsQuery.error
      ? extractErrorMessage(paymentsQuery.error)
      : "",
);

const totalFees = $derived(
  fees.reduce((s, f) => s + Number(f.net_amount || 0), 0),
);
const totalPaid = $derived(
  fees.reduce((s, f) => s + Number(f.paid_amount || 0), 0),
);
const balanceDue = $derived(
  fees.reduce(
    (s, f) =>
      s +
      Number(
        f.balance_due ?? Number(f.net_amount || 0) - Number(f.paid_amount || 0),
      ),
    0,
  ),
);

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
      fee_name: m.common_ft_category_tuition(),
      due_date: f.due_date || null,
      total: originalAmount,
      paid: paidAmount,
      discount: discountAmount,
      balance,
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
    pin: "colPinStart",
    prop: "fee_name",
    header: m.finance_fees_col_description(),
    name: m.finance_fees_col_description(),
    width: 180,
    size: 180,
    readonly: true,
  },
  {
    id: "due_date",
    prop: "due_date",
    header: m.common_overview_date(),
    name: m.common_overview_date(),
    type: "date",
    sortable: true,
    width: 120,
    size: 120,
    readonly: true,
    cellClass: "font-mono",
  },
  {
    id: "total",
    prop: "total",
    header: m.common_ft_col_amount(),
    name: m.common_ft_col_amount(),
    type: "currency",
    aggregate: "sum",
    sortable: true,
    width: 120,
    size: 120,
    readonly: true,
    align: "end",
    cellClass: "font-mono font-medium",
  },
  {
    id: "paid",
    prop: "paid",
    header: m.finance_fees_total_collected(),
    name: m.finance_fees_total_collected(),
    type: "currency",
    aggregate: "sum",
    sortable: true,
    width: 120,
    size: 120,
    readonly: true,
    align: "end",
    cellClass: "font-mono text-emerald-600 font-medium",
  },
  {
    id: "balance",
    prop: "balance",
    header: m.finance_fees_total_due(),
    name: m.finance_fees_total_due(),
    type: "currency",
    aggregate: "sum",
    sortable: true,
    width: 120,
    size: 120,
    readonly: true,
    align: "end",
    cellClass: "font-mono font-bold text-rose-600",
  },
  {
    id: "status",
    prop: "status",
    header: m.common_word_status(),
    name: m.common_word_status(),
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
    name: m.finance_fees_col_receipt(),
    width: 120,
    size: 120,
    readonly: true,
    cellClass: "font-mono font-bold",
  },
  {
    id: "method",
    prop: "method",
    header: m.finance_fees_col_method(),
    name: m.finance_fees_col_method(),
    width: 110,
    size: 110,
    readonly: true,
    align: "center",
  },
  {
    id: "date",
    prop: "date",
    header: m.finance_fees_col_pay_date(),
    name: m.finance_fees_col_pay_date(),
    type: "date",
    sortable: true,
    width: 125,
    size: 125,
    readonly: true,
    cellClass: "font-mono",
  },
  {
    id: "amount",
    prop: "amount",
    header: m.finance_payments_col_amount(),
    name: m.finance_payments_col_amount(),
    type: "currency",
    aggregate: "sum",
    sortable: true,
    width: 125,
    size: 125,
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
    date: p.paid_at || p.created_at || null,
    amount: Number(p.amount || 0),
  })),
);

const rtl = $derived(getLocale() === "ar");
</script>

{#if isError}
  <StudentSectionState
    icon={Document}
    title={m.common_search_load_failed()}
    description={errorMessage}
  >
    {#snippet action()}
      <Button
        variant="outline"
        size="sm"
        onclick={() => {
          void feesQuery.refetch();
          void paymentsQuery.refetch();
        }}
      >
        {m.common_retry()}
      </Button>
    {/snippet}
  </StudentSectionState>
{:else}
  <div class="flex flex-col gap-6 text-xs text-start font-sans">
    <div class="grid grid-cols-1 gap-3 sm:grid-cols-3">
      <StudentMetricCard
        title={m.finance_fees_total_fees()}
        value={formatCurrency(totalFees)}
        icon={Document}
        loading={isLoading}
      />
      <StudentMetricCard
        title={m.finance_fees_total_paid()}
        value={formatCurrency(totalPaid)}
        icon={CheckCircle}
        tone="success"
        loading={isLoading}
      />
      <StudentMetricCard
        title={m.finance_fees_balance_due()}
        value={formatCurrency(balanceDue)}
        icon={InfoCircle}
        tone={balanceDue > 0 ? "danger" : "success"}
        loading={isLoading}
      />
    </div>

    <section class="flex flex-col gap-2">
      <h3 class="text-xs font-semibold text-foreground">
        {m.finance_fees_assigned_title()}
      </h3>
      <div class="h-[320px] w-full">
        <OrbisGrid
          columns={feesCols}
          source={feesSource}
          loading={isLoading}
          clientQuery
          emptyTitle={m.finance_fees_no_fees()}
          {rtl}
          readonly
        />
      </div>
    </section>

    <section class="flex flex-col gap-2">
      <h3 class="text-xs font-semibold text-foreground">
        {m.finance_fees_payments_title()}
      </h3>
      <div class="h-[320px] w-full">
        <OrbisGrid
          columns={paymentsCols}
          source={paymentsSource}
          loading={isLoading}
          clientQuery
          emptyTitle={m.finance_fees_no_payments()}
          {rtl}
          readonly
        />
      </div>
    </section>
  </div>
{/if}
