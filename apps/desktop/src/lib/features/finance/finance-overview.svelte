<!--
@component FinanceOverview
Main dashboard for the organization's financial summary.
Displays an interactive full-width SVG trend line chart at the top,
followed by grids displaying late-paying students, collection progress,
recent transactions, and treasury balances. Inline action buttons are placed
directly inside the cards' headers.
-->
<script lang="ts">
import { createQuery } from "@tanstack/svelte-query";
import * as m from "$lib/paraglide/messages";

import { api, extractErrorMessage, queryKeys } from "$lib/api";
import type { StudentDetailDto } from "$lib/api/types";
import { l } from "$lib/core/i18n/locale";

import {
  createStudentFeesQuery,
  createCashAccountsQuery,
} from "$lib/features/finance";
import { createEnrollmentsQuery } from "$lib/features/students";
import { createClassesQuery } from "$lib/features/academics";
import CashFlowTrendCard from "$lib/features/metrics/cards/cash-flow-trend-card.svelte";
import LateStudentsCard from "$lib/features/finance/components/late-students-card.svelte";
import CollectionRateCard from "$lib/features/finance/components/collection-rate-card.svelte";
import RecentPaymentsCard from "$lib/features/finance/components/recent-payments-card.svelte";
import AccountBalancesCard from "$lib/features/finance/components/account-balances-card.svelte";
import { FinanceSkeleton } from "$lib/shared/components/skeletons";
import ErrorState from "$lib/shared/components/error-state.svelte";
import { fade } from "svelte/transition";

let { orgId }: { orgId: string } = $props();

// TanStack Queries
const feesQuery = createStudentFeesQuery(() => orgId, {
  is_overdue: true,
  sort_by: "balance_due",
  sort_dir: "desc",
});
const feeSummaryQuery = createQuery(() => ({
  queryKey: queryKeys.finance.studentFeeSummary(orgId),
  queryFn: () => api.getStudentFeeSummary(),
  enabled: !!orgId,
  staleTime: 2 * 60 * 1000,
}));
const paymentsFilter = {
  page: 1,
  page_size: 5,
  sort_by: "paid_at",
  sort_dir: "desc",
};
const paymentsQuery = createQuery(() => ({
  queryKey: queryKeys.finance.payments(orgId, paymentsFilter),
  queryFn: async () =>
    (
      await api.listPaymentsPaginated(paymentsFilter)
    ).items,
  enabled: !!orgId,
  staleTime: 2 * 60 * 1000,
}));
const treasuriesQuery = createCashAccountsQuery(() => orgId);
const enrollmentsQuery = createEnrollmentsQuery(() => orgId);
const classesQuery = createClassesQuery(() => orgId);

const fees = $derived(feesQuery.data ?? []);
const feeSummary = $derived(feeSummaryQuery.data ?? []);
const payments = $derived(paymentsQuery.data ?? []);
const treasuries = $derived(treasuriesQuery.data ?? []);
const enrollments = $derived(enrollmentsQuery.data ?? []);
const classes = $derived(classesQuery.data ?? []);
const enrollmentsById = $derived(
  new Map(enrollments.map((enrollment) => [enrollment.id, enrollment])),
);
const classesById = $derived(
  new Map(classes.map((classItem) => [classItem.id, classItem])),
);

const isLoading = $derived(
  feesQuery.isLoading ||
    feeSummaryQuery.isLoading ||
    paymentsQuery.isLoading ||
    treasuriesQuery.isLoading ||
    enrollmentsQuery.isLoading ||
    classesQuery.isLoading,
);

const error = $derived(
  feesQuery.error ||
    feeSummaryQuery.error ||
    paymentsQuery.error ||
    treasuriesQuery.error ||
    enrollmentsQuery.error ||
    classesQuery.error,
);

function refetchAll() {
  feesQuery.refetch();
  feeSummaryQuery.refetch();
  paymentsQuery.refetch();
  treasuriesQuery.refetch();
  enrollmentsQuery.refetch();
  classesQuery.refetch();
  studentDetailsQuery.refetch();
}

function toAmount(value: string | number | null | undefined): number {
  const amount = Number(value ?? 0);
  return Number.isFinite(amount) ? amount : 0;
}

// ── Financial KPI Computations ───────────────────────────────────────────
const totalFees = $derived(
  feeSummary.reduce((sum, item) => sum + toAmount(item.due), 0),
);

const totalPaid = $derived(
  feeSummary.reduce((sum, item) => sum + toAmount(item.paid), 0),
);

const collectionRate = $derived(
  totalFees > 0 ? Math.round((totalPaid / totalFees) * 100) : 0,
);

const overdueDebtors = $derived.by(() => {
  const grouped = new Map<
    string,
    {
      studentId: string;
      studentNumber: string | null;
      className: string;
      remaining: number;
      daysLate: number;
      dueDate: string;
    }
  >();
  const today = new Date();
  today.setHours(0, 0, 0, 0);

  for (const fee of fees) {
    if (fee.status === "paid") continue;
    const dueDate = new Date(`${fee.due_date.slice(0, 10)}T00:00:00`);
    const remaining = toAmount(fee.balance_due);
    if (Number.isNaN(dueDate.getTime()) || remaining <= 0) continue;

    const daysLate = Math.floor(
      (today.getTime() - dueDate.getTime()) / 86_400_000,
    );
    if (daysLate <= 0) continue;

    const enrollment = fee.enrollment_id
      ? enrollmentsById.get(fee.enrollment_id)
      : undefined;
    const studentId = fee.student_id || enrollment?.student_id;
    if (!studentId) continue;

    const classItem = enrollment?.section_id
      ? classesById.get(enrollment.section_id)
      : undefined;
    const className = classItem
      ? l(classItem.name_ar, classItem.name) || "—"
      : "—";
    const existing = grouped.get(studentId);

    if (existing) {
      existing.remaining += remaining;
      if (daysLate > existing.daysLate) {
        existing.daysLate = daysLate;
        existing.dueDate = fee.due_date;
        existing.className = className;
      }
    } else {
      grouped.set(studentId, {
        studentId,
        studentNumber: enrollment?.student_number ?? null,
        className,
        remaining,
        daysLate,
        dueDate: fee.due_date,
      });
    }
  }

  return [...grouped.values()].sort((a, b) => b.remaining - a.remaining);
});
const topLateDebtors = $derived(overdueDebtors.slice(0, 3));
const studentDetailIds = $derived.by(() => {
  return topLateDebtors.map((student) => student.studentId).sort();
});
const studentDetailsQuery = createQuery(() => {
  const studentIds = studentDetailIds;
  return {
    queryKey: [
      ...queryKeys.student.all,
      "finance-overview-details",
      orgId,
      studentIds,
    ],
    queryFn: async () => {
      const results = await Promise.allSettled(
        studentIds.map((studentId) => api.getStudent(studentId)),
      );
      const profiles: Record<string, StudentDetailDto> = {};

      results.forEach((result, index) => {
        if (result.status === "fulfilled") {
          profiles[studentIds[index]!] = result.value;
        } else {
          console.error(
            "Failed to load student details for finance overview",
            result.reason,
          );
        }
      });

      return profiles;
    },
    enabled: !!orgId && studentIds.length > 0,
    staleTime: 5 * 60 * 1000,
  };
});
const studentProfiles = $derived(studentDetailsQuery.data ?? {});

function getStudentName(
  studentId: string | undefined,
  studentNumber: string | null | undefined,
): string {
  const profile = studentId ? studentProfiles[studentId] : undefined;
  if (profile) {
    const name = l(
      profile.person.full_name_ar || profile.student.name_ar,
      profile.person.full_name || profile.student.full_name,
    );
    if (typeof name === "string" && name.trim()) return name.trim();
  }
  return studentNumber || "—";
}

const lateStudents = $derived(
  topLateDebtors.map((student) => ({
    ...student,
    studentName: getStudentName(student.studentId, student.studentNumber),
  })),
);
const recentPayments = $derived(
  payments.map((payment) => {
    const enrollment = payment.enrollment_id
      ? enrollmentsById.get(payment.enrollment_id)
      : undefined;
    return {
      ...payment,
      studentName:
        payment.payer_name?.trim() ||
        enrollment?.student_number ||
        "—",
      receiptNumber: payment.receipt_number || payment.id.slice(0, 8),
    };
  }),
);
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
      <CashFlowTrendCard {orgId} />

      <!-- 2. Middle Section: Late Paying Students Grid & Donut Analytics -->
      <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
        <LateStudentsCard {lateStudents} {orgId} />
        <CollectionRateCard {collectionRate} {totalFees} {totalPaid} {orgId} />
      </div>

      <!-- 3. Bottom Section: Recent Payments & Treasuries list -->
      <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
        <RecentPaymentsCard {recentPayments} {orgId} />
        <AccountBalancesCard {treasuries} {orgId} />
      </div>
    </div>
  {/if}
</div>
