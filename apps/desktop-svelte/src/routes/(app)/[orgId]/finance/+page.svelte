<!--
@component FinanceOverviewPage
Main dashboard for the organization's financial summary.
Displays an interactive full-width SVG trend line chart at the top,
followed by grids displaying late-paying students, collection progress,
recent transactions, and treasury balances. Inline action buttons are placed
directly inside the cards' headers.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import { AlertCircle } from "lucide-svelte";

import { extractErrorMessage } from "$lib/core/ipc";

import {
  createStudentFeesQuery,
  createTreasuriesQuery,
  createPaymentsQuery,
} from "$lib/features/finance";
import { createEnrollmentsQuery } from "$lib/features/students";
import { createClassesQuery } from "$lib/features/academics";
import CashFlowTrendCard from "$lib/features/metrics/cards/cash-flow-trend-card.svelte";
import LateStudentsCard from "$lib/features/finance/components/late-students-card.svelte";
import CollectionRateCard from "$lib/features/finance/components/collection-rate-card.svelte";
import RecentPaymentsCard from "$lib/features/finance/components/recent-payments-card.svelte";
import TreasuryBalancesCard from "$lib/features/finance/components/treasury-balances-card.svelte";
import { Button } from "@orbis/ui-svelte";
import { FinanceSkeleton } from "$lib/shared/components/skeletons";
import ErrorState from "$lib/shared/components/error-state.svelte";

let { data }: { data: { orgId: string } } = $props();

import { fade } from "svelte/transition";

// TanStack Queries
const feesQuery = createStudentFeesQuery(() => data.orgId);
const paymentsQuery = createPaymentsQuery(() => data.orgId);
const treasuriesQuery = createTreasuriesQuery(() => data.orgId);
const enrollmentsQuery = createEnrollmentsQuery(() => data.orgId);
const classesQuery = createClassesQuery(() => data.orgId);

const fees = $derived($feesQuery.data ?? []);
const payments = $derived($paymentsQuery.data ?? []);
const treasuries = $derived($treasuriesQuery.data ?? []);
const enrollments = $derived($enrollmentsQuery.data ?? []);
const classes = $derived($classesQuery.data ?? []);

const isLoading = $derived(
  $feesQuery.isLoading ||
    $paymentsQuery.isLoading ||
    $treasuriesQuery.isLoading ||
    $enrollmentsQuery.isLoading ||
    $classesQuery.isLoading,
);

const error = $derived(
  $feesQuery.error ||
    $paymentsQuery.error ||
    $treasuriesQuery.error ||
    $enrollmentsQuery.error ||
    $classesQuery.error,
);

function refetchAll() {
  $feesQuery.refetch();
  $paymentsQuery.refetch();
  $treasuriesQuery.refetch();
  $enrollmentsQuery.refetch();
  $classesQuery.refetch();
}

// ── Financial KPI Computations ───────────────────────────────────────────
const totalFees = $derived(
  fees.reduce(
    (sum, f) => sum + Number(f.net_amount || f.original_amount || 0),
    0,
  ),
);

const totalPaid = $derived(
  payments.reduce((sum, p) => sum + Number(p.amount || 0), 0),
);

const balanceDue = $derived(Math.max(0, totalFees - totalPaid));

const collectionRate = $derived(
  totalFees > 0 ? Math.round((totalPaid / totalFees) * 100) : 0,
);

const treasuryAssets = $derived(
  treasuries.reduce(
    (sum, t: any) => sum + Number(t.balance || t.current_balance || 0),
    0,
  ),
);

// ── Payments grouping mapping for Fee paid amounts ──────────────────────
const feePaymentsMap = $derived.by(() => {
  const map = new Map<string, number>();
  for (const p of payments) {
    if (p.fee_id) {
      const id = String(p.fee_id);
      map.set(id, (map.get(id) || 0) + Number(p.amount || 0));
    }
  }
  return map;
});

const getFeePaidAmount = (fId: string, status: string, netAmount: number) => {
  if (status === "paid") return netAmount;
  const fromPayments = feePaymentsMap.get(fId) || 0;
  if (status === "unpaid" && fromPayments === 0) return 0;
  return fromPayments;
};

// ── Late-Paying Students ──────────────────────────────────────────────────
const lateStudents = $derived.by(() => {
  const list = [];
  for (const f of fees) {
    if (f.status === "paid") continue;

    const netAmt = Number(f.net_amount || f.original_amount || 0);
    const paidAmt = getFeePaidAmount(f.id, f.status, netAmt);
    const remaining = Math.max(0, netAmt - paidAmt);

    if (remaining <= 0) continue;

    const enrollment = enrollments.find((e) => e.id === f.enrollment_id);
    const studentName = enrollment
      ? enrollment.full_name ||
        enrollment.name_ar ||
        enrollment.name_en ||
        enrollment.student_number ||
        "—"
      : "—";
    const studentNum = enrollment ? enrollment.student_number : "—";
    const avatar = enrollment ? enrollment.avatar : null;
    const classId = enrollment ? enrollment.class_id : null;

    const cls = classId ? classes.find((c) => c.id === classId) : null;
    const className = cls ? cls.name_ar || cls.name : "—";

    const dueDate = new Date(f.due_date);
    const diffTime = new Date().getTime() - dueDate.getTime();
    const diffDays = Math.ceil(diffTime / (1000 * 60 * 60 * 24));

    list.push({
      feeId: f.id,
      studentName,
      studentNum,
      avatar,
      className,
      remaining,
      dueDate: f.due_date,
      daysLate: diffDays > 0 ? diffDays : 0,
    });
  }

  // Sort by remaining due balance descending, take top 5
  return list.sort((a, b) => b.remaining - a.remaining).slice(0, 5);
});

// ── Recent Payments ──────────────────────────────────────────────────────
const recentPayments = $derived.by(() => {
  const sorted = [...payments].sort((a: any, b: any) => {
    const dateA = new Date(a.payment_date || a.created_at);
    const dateB = new Date(b.payment_date || b.created_at);
    return dateB.getTime() - dateA.getTime();
  });

  return sorted.slice(0, 5).map((p: any) => {
    const enrollment = enrollments.find((e) => e.id === p.enrollment_id);
    const studentName =
      p.student_name ||
      (enrollment
        ? enrollment.full_name ||
          enrollment.name_ar ||
          enrollment.name_en ||
          "—"
        : "—");

    return {
      ...p,
      studentName,
      receiptNumber: p.receipt_number || p.id.slice(0, 8),
    };
  });
});

// ── Reusable trend card configuration ──
</script>

<div class="h-full w-full overflow-y-auto bg-background/50 scrollbar-hide p-6 flex flex-col gap-6 font-sans">
  
  {#if isLoading}
    <FinanceSkeleton />
  {:else if error}
    <ErrorState 
      title={m.finance_overview_error()} 
      message={extractErrorMessage(error)} 
      onRetry={refetchAll} 
    />
  {:else}
    <!-- Main Content Grid -->
    <div class="flex flex-col gap-6" in:fade={{ duration: 150 }}>
      
      <!-- 1. Full-Width SVG Cash Flow Trend Chart at the very top -->
      <CashFlowTrendCard orgId={data.orgId} />

      <!-- 2. Middle Section: Late Paying Students Table & Donut Analytics -->
      <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
        <LateStudentsCard {lateStudents} orgId={data.orgId} />
        <CollectionRateCard {collectionRate} {totalFees} {totalPaid} orgId={data.orgId} />
      </div>

      <!-- 3. Bottom Section: Recent Payments & Treasuries list -->
      <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
        <RecentPaymentsCard {recentPayments} orgId={data.orgId} />
        <TreasuryBalancesCard {treasuries} orgId={data.orgId} />
      </div>
      
    </div>
  {/if}
</div>
