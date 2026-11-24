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
import type { FeeTemplateDto, StudentDetailDto } from "$lib/api/types";
import { l } from "$lib/core/i18n/locale";
import { formatCurrency } from "$lib/core/i18n";

import {
  createStudentFeesQuery,
  createCashAccountsQuery,
  createFeeTemplatesQuery,
} from "$lib/features/finance";
import { createEnrollmentsQuery } from "$lib/features/students";
import { createClassesQuery } from "$lib/features/academics";
import { createOrgUnitsQuery } from "$lib/features/settings/queries";
import CashFlowTrendCard from "$lib/features/metrics/cards/cash-flow-trend-card.svelte";
import LateStudentsCard from "$lib/features/finance/components/late-students-card.svelte";
import CollectionRateCard from "$lib/features/finance/components/collection-rate-card.svelte";
import RecentPaymentsCard from "$lib/features/finance/components/recent-payments-card.svelte";
import AccountBalancesCard from "$lib/features/finance/components/account-balances-card.svelte";
import BranchReceivablesCard from "$lib/features/finance/components/branch-receivables-card.svelte";
import { FinanceSkeleton } from "$lib/shared/components/skeletons";
import ErrorState from "$lib/shared/components/error-state.svelte";
import DateRangeFilter from "$lib/shared/components/date-range-filter.svelte";
import {
  getDateRangeForPreset,
  type DateRangeFilterValue,
} from "$lib/shared/components/date-range";
import { Card, CardContent } from "@orbis/ui/card";
import { Button } from "@orbis/ui/button";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
} from "@orbis/ui/select";
import { fade } from "svelte/transition";

let { orgId }: { orgId: string } = $props();
let dateRange = $state<DateRangeFilterValue>(
  getDateRangeForPreset("year_to_date"),
);
let selectedBranchId = $state("all");
let selectedCurrency = $state("");

// TanStack Queries
const orgUnitsQuery = createOrgUnitsQuery(() => orgId);
const orgUnitFilter = $derived(
  selectedBranchId === "all" ? undefined : selectedBranchId,
);
const dateFilter = $derived({
  from_date: dateRange.fromDate,
  to_date: dateRange.toDate,
  org_unit_id: orgUnitFilter,
});
const templatesQuery = createFeeTemplatesQuery(() => orgId);
const feeSummaryQuery = createQuery(() => ({
  queryKey: [...queryKeys.finance.studentFeeSummary(orgId), dateFilter],
  queryFn: () => api.getStudentFeeSummary(dateFilter),
  enabled: !!orgId,
  staleTime: 2 * 60 * 1000,
}));
const feeSummary = $derived(feeSummaryQuery.data ?? []);
const feeTemplates = $derived(templatesQuery.data ?? []);
const availableCurrencies = $derived(
  [
    ...new Set(
      feeTemplates.map((template) => template.currency).filter(Boolean),
    ),
  ].sort(),
);
const activeCurrency = $derived(
  availableCurrencies.includes(selectedCurrency)
    ? selectedCurrency
    : (availableCurrencies[0] ?? ""),
);
const hasActiveFilters = $derived(
  dateRange.preset !== "year_to_date" ||
    selectedBranchId !== "all" ||
    (selectedCurrency !== "" && selectedCurrency !== availableCurrencies[0]),
);
const selectedSummary = $derived(
  feeSummary.find((item) => item.currency === activeCurrency),
);
const feesQuery = createStudentFeesQuery(
  () => orgId,
  () => ({
    from_date: dateRange.fromDate,
    to_date: dateRange.toDate,
    currency_code: activeCurrency || undefined,
    is_overdue: true,
    sort_by: "balance_due",
    sort_dir: "desc",
  }),
  () => !!activeCurrency,
);
const paymentsFilter = $derived({
  ...dateFilter,
  currency_code: activeCurrency || undefined,
  page: 1,
  page_size: 20,
  sort_by: "paid_at",
  sort_dir: "desc",
});
const paymentsQuery = createQuery(() => ({
  queryKey: queryKeys.finance.payments(orgId, paymentsFilter),
  queryFn: async () => (await api.listPaymentsPaginated(paymentsFilter)).items,
  enabled: !!orgId && !!activeCurrency,
  staleTime: 2 * 60 * 1000,
}));
const treasuriesQuery = createCashAccountsQuery(() => orgId);
const enrollmentsQuery = createEnrollmentsQuery(() => orgId);
const classesQuery = createClassesQuery(() => orgId);

const fees = $derived(feesQuery.data ?? []);
const payments = $derived(paymentsQuery.data ?? []);
const treasuries = $derived(treasuriesQuery.data ?? []);
const enrollments = $derived(enrollmentsQuery.data ?? []);
const classes = $derived(classesQuery.data ?? []);
const orgUnits = $derived(orgUnitsQuery.data ?? []);
const templatesById = $derived(
  new Map(feeTemplates.map((template) => [template.id, template])),
);
const branchUnits = $derived(
  orgUnits.filter((unit) => unit.unit_type === "branch"),
);
const selectedBranchName = $derived(
  selectedBranchId === "all"
    ? m.finance_overview_current_branch()
    : branchUnits.find((unit) => String(unit.id) === selectedBranchId)
        ?.name_ar ||
        branchUnits.find((unit) => String(unit.id) === selectedBranchId)
          ?.name ||
        m.common_header_org_branch(),
);
const enrollmentsById = $derived(
  new Map(enrollments.map((enrollment) => [enrollment.id, enrollment])),
);
const classesById = $derived(
  new Map(classes.map((classItem) => [classItem.id, classItem])),
);

const isLoading = $derived(
  feesQuery.isLoading ||
    orgUnitsQuery.isLoading ||
    feeSummaryQuery.isLoading ||
    paymentsQuery.isLoading ||
    treasuriesQuery.isLoading ||
    enrollmentsQuery.isLoading ||
    classesQuery.isLoading ||
    templatesQuery.isLoading,
);

const error = $derived(
  feesQuery.error ||
    orgUnitsQuery.error ||
    feeSummaryQuery.error ||
    paymentsQuery.error ||
    treasuriesQuery.error ||
    enrollmentsQuery.error ||
    classesQuery.error ||
    templatesQuery.error,
);

function refetchAll() {
  feesQuery.refetch();
  orgUnitsQuery.refetch();
  feeSummaryQuery.refetch();
  paymentsQuery.refetch();
  treasuriesQuery.refetch();
  enrollmentsQuery.refetch();
  classesQuery.refetch();
  templatesQuery.refetch();
  studentDetailsQuery.refetch();
}

function toAmount(value: string | number | null | undefined): number {
  const amount = Number(value ?? 0);
  return Number.isFinite(amount) ? amount : 0;
}

function formatSelectedAmount(value: number): string {
  return activeCurrency ? formatCurrency(value, activeCurrency) : "—";
}

// ── Financial KPI Computations ───────────────────────────────────────────
const totalFees = $derived(toAmount(selectedSummary?.due));

const totalPaid = $derived(toAmount(selectedSummary?.paid));

const totalBalance = $derived(toAmount(selectedSummary?.balance));

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
    if (
      activeCurrency &&
      templatesById.get(fee.fee_template_id)?.currency !== activeCurrency
    ) {
      continue;
    }
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
    if (
      selectedBranchId !== "all" &&
      String(classItem?.unit_id ?? "") !== selectedBranchId
    ) {
      continue;
    }
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

const branchReceivables = $derived.by(() => {
  const balances = new Map<string, number>();
  for (const fee of fees) {
    if (
      activeCurrency &&
      templatesById.get(fee.fee_template_id)?.currency !== activeCurrency
    ) {
      continue;
    }
    const enrollment = fee.enrollment_id
      ? enrollmentsById.get(fee.enrollment_id)
      : undefined;
    const classItem = enrollment?.section_id
      ? classesById.get(enrollment.section_id)
      : undefined;
    const branchId = String(classItem?.unit_id ?? "");
    const remaining = toAmount(fee.balance_due);
    if (!branchId || remaining <= 0) continue;
    balances.set(branchId, (balances.get(branchId) ?? 0) + remaining);
  }

  return branchUnits
    .map((unit) => ({
      id: String(unit.id),
      name: l(unit.name_ar, unit.name) || "—",
      balance: balances.get(String(unit.id)) ?? 0,
    }))
    .filter((branch) => branch.balance > 0)
    .sort((a, b) => b.balance - a.balance);
});
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
  payments
    .filter(
      (payment) => !activeCurrency || payment.currency_code === activeCurrency,
    )
    .slice(0, 5)
    .map((payment) => {
      const enrollment = payment.enrollment_id
        ? enrollmentsById.get(payment.enrollment_id)
        : undefined;
      return {
        ...payment,
        studentName:
          payment.payer_name?.trim() || enrollment?.student_number || "—",
        receiptNumber: payment.receipt_number || payment.id.slice(0, 8),
      };
    }),
);
const visibleTreasuries = $derived(
  activeCurrency
    ? treasuries.filter((account) => account.currency === activeCurrency)
    : treasuries,
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
    <div class="flex flex-col gap-6" in:fade={{ duration: 150 }}>
      <div class="flex w-full min-w-0 flex-wrap items-center gap-2">
        <Select type="single" bind:value={selectedBranchId}>
          <SelectTrigger
            class="h-9 min-w-0 max-w-full gap-2 bg-background px-3 text-xs sm:min-w-44"
            aria-label={m.common_header_org_branch()}
          >
            <span class="shrink-0 text-[11px] text-muted-foreground sm:text-xs">{m.common_header_org_branch()}</span>
            <span class="min-w-0 truncate font-medium text-foreground">{selectedBranchName}</span>
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="all">{m.finance_overview_current_branch()}</SelectItem>
            {#each branchUnits as unit (unit.id)}
              <SelectItem value={String(unit.id)}>
                {l(unit.name_ar, unit.name)}
              </SelectItem>
            {/each}
          </SelectContent>
        </Select>
        {#if availableCurrencies.length > 1}
          <Select
            type="single"
            value={activeCurrency}
            onValueChange={(value) => {
              if (value) selectedCurrency = value;
            }}
          >
            <SelectTrigger
              class="h-9 min-w-0 max-w-full gap-2 bg-background px-3 text-[11px] sm:min-w-28 sm:text-xs"
              aria-label={m.finance_ui_currency()}
            >
              <span class="shrink-0 text-muted-foreground">{m.finance_ui_currency()}</span>
              <span class="truncate font-medium text-foreground">{activeCurrency}</span>
            </SelectTrigger>
            <SelectContent>
              {#each availableCurrencies as currency (currency)}
                <SelectItem value={currency}>{currency}</SelectItem>
              {/each}
            </SelectContent>
          </Select>
        {:else if activeCurrency}
          <span class="inline-flex h-9 items-center gap-2 rounded-md border border-input bg-background px-3 text-xs">
            <span class="text-muted-foreground">{m.finance_ui_currency()}</span>
            <span class="font-medium text-foreground">{activeCurrency}</span>
          </span>
        {/if}
        <DateRangeFilter bind:value={dateRange} />
        {#if hasActiveFilters}
          <Button
            variant="ghost"
            size="sm"
            class="h-9 px-2 text-xs text-muted-foreground hover:text-foreground"
            onclick={() => {
              dateRange = getDateRangeForPreset("year_to_date");
              selectedBranchId = "all";
              selectedCurrency = "";
            }}
          >
            {m.finance_overview_reset_filters()}
          </Button>
        {/if}
      </div>

      <div class="grid grid-cols-1 gap-3 sm:grid-cols-3">
        <Card class="shadow-sm">
          <CardContent class="p-4">
            <p class="text-xs font-medium text-muted-foreground">
              {m.finance_overview_trend_invoiced()}
            </p>
            <p class="mt-2 truncate font-mono text-lg font-bold text-foreground">
              {formatSelectedAmount(totalFees)}
            </p>
          </CardContent>
        </Card>
        <Card class="shadow-sm">
          <CardContent class="p-4">
            <p class="text-xs font-medium text-muted-foreground">
              {m.finance_overview_trend_collected()}
            </p>
            <p class="mt-2 truncate font-mono text-lg font-bold text-primary">
              {formatSelectedAmount(totalPaid)}
            </p>
          </CardContent>
        </Card>
        <Card class="shadow-sm">
          <CardContent class="p-4">
            <p class="text-xs font-medium text-muted-foreground">
              {m.finance_ui_current_balance()}
            </p>
            <p class="mt-2 truncate font-mono text-lg font-bold text-foreground">
              {formatSelectedAmount(totalBalance)}
            </p>
          </CardContent>
        </Card>
      </div>

      <CashFlowTrendCard
        {orgId}
        {dateRange}
        branchId={orgUnitFilter}
        selectedCurrency={activeCurrency}
        {feeTemplates}
      />

      <div class="grid grid-cols-1 gap-6 lg:grid-cols-4">
        <BranchReceivablesCard
          branches={branchReceivables}
          {selectedBranchId}
          currency={activeCurrency}
          onSelect={(branchId) => (selectedBranchId = branchId)}
        />
        <LateStudentsCard {lateStudents} {orgId} currency={activeCurrency || "YER"} />
        {#if activeCurrency}
          <CollectionRateCard
            {collectionRate}
            {totalFees}
            {totalPaid}
            {orgId}
            currency={activeCurrency}
          />
        {/if}
      </div>

      <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
        <RecentPaymentsCard {recentPayments} {orgId} />
        <AccountBalancesCard treasuries={visibleTreasuries} {orgId} />
      </div>
    </div>
  {/if}
</div>
