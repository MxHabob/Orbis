<!--
  @component FeesWorkspace
  Minimal, elegant, and high-performance Student Fees & Tuition workspace.
  Adheres strictly to Orbis Clean Architecture and the FinanceOverview design system:
  - Top KPI metric cards for fees, collections, and outstanding balances
  - Sleek top toolbar with search, status filter, and export
  - In-page fee assignment, installment scheduling, and refund cards
  - High-performance OrbisGrid table
  - Real backend fee calculation and invoice tracking
-->
<script lang="ts">
import { createPaymentsQuery, refreshPaymentData } from "./queries";
import { Card, LoadingIndicator } from "@orbis/ui";
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import { createInfiniteQuery, createQuery } from "@tanstack/svelte-query";
import { api, extractErrorMessage, queryClient, queryKeys } from "$lib/api";
import { formatDateKey } from "$lib/core/i18n/format";
import { formatCurrency, formatDate, isAr } from "$lib/core/i18n";
import type {
  StudentFeeDto,
  FeeTemplateDto,
  AcademicPeriodDto,
  EnrollmentDto,
  StudentDetailDto,
} from "$lib/api/types";
import { orgContext } from "$lib/stores/org-context.svelte";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
} from "@orbis/ui/select";
import { toast } from "@orbis/ui";
import {
  Plus,
  Refresh,
  Download,
  XMark,
  Calendar,
  ArrowPath,
} from "@orbis/icons";
import {
  OrbisGrid,
  type GridActionEvent,
  type DataQuery,
  type OrbisColumnDef,
  type SortSpec,
} from "@orbis/svelte-grid";
import SearchableSelect from "$lib/shared/components/searchable-select.svelte";
import { buildInstallmentDates, hasOrderedInstallmentDates } from "./installment-schedule";
import { financeRecordColumn } from "./record-presentation";
import FinanceRecordDialog from "./components/finance-record-dialog.svelte";
import FinanceInlineFormCard from "$lib/features/finance/components/finance-inline-form-card.svelte";
import DateRangeFilter from "$lib/shared/components/date-range-filter.svelte";
import { createDefaultDateRange } from "$lib/shared/components/date-range";

let { orgId }: { orgId: string } = $props();

const asArray = <T>(value: T[] | null | undefined): T[] =>
  Array.isArray(value) ? value : [];

// ── State ──────────────────────────────────────────────────────────────
let feeTemplates = $state<FeeTemplateDto[]>([]);
let enrollments = $state<EnrollmentDto[]>([]);
let periods = $state<AcademicPeriodDto[]>([]);
let isLoading = $state(true);
let isSubmitting = $state(false);
let formError = $state("");

// Active Action Form
let activeForm = $state<"none" | "create" | "split" | "refund">("none");

// Create Fee Form
let formStudentId = $state("");
let formPeriodId = $state("");
let formTemplateId = $state("");
let formDiscountAmount = $state<number | null>(null);
let formDueDate = $state(
  new Date(Date.now() + 30 * 86400000).toISOString().slice(0, 10),
);

// Split Installments Form
let selectedFeeForSplit = $state<StudentFeeDto | null>(null);
let splitCount = $state(2);
let splitDueDates = $state<string[]>([]);

function setInstallmentCount(count: number) {
  splitDueDates = buildInstallmentDates(formatDateKey(new Date()), count, splitDueDates);
  splitCount = count;
}

// Refund Form
let selectedFeeForRefund = $state<StudentFeeDto | null>(null);
let refundAmount = $state<number | null>(null);
let refundPaymentId = $state<string | null>(null);
let refundNotes = $state("");

// Filters
let searchQuery = $state("");
let dateRange = $state(createDefaultDateRange());
let statusFilter = $state<
  "all" | "pending" | "partially_paid" | "paid" | "overdue"
>("all");
let gridSort = $state<SortSpec[]>([]);
const feePageSort = $derived.by(() => {
  const sort = gridSort[0];
  const fieldByColumn: Record<string, string> = {
    due_date_display: "due_date",
    status_display: "status",
    net_formatted: "net_amount",
    paid_formatted: "paid_amount",
    balance_formatted: "balance_due",
  };
  const sortBy = sort ? fieldByColumn[sort.id] : undefined;
  return sortBy ? { sort_by: sortBy, sort_dir: sort!.dir } : {};
});
function handleFeeGridQuery(query: DataQuery) {
  gridSort = query.sorts ?? [];
  searchQuery = query.search ?? "";
}

const feePageFilter = $derived({
  from_date: dateRange.fromDate || undefined,
  to_date: dateRange.toDate || undefined,
  status:
    statusFilter !== "all" && statusFilter !== "overdue"
      ? statusFilter
      : undefined,
  is_overdue: statusFilter === "overdue" ? true : undefined,
});
const feesQuery = createInfiniteQuery(() => ({
  queryKey: queryKeys.finance.studentFeesPaginated(orgId, {
    ...feePageFilter,
    search: searchQuery.trim() || undefined,
    ...feePageSort,
  }),
  queryFn: ({ pageParam }) =>
    api.listStudentFeesPaginated({
      ...feePageFilter,
      search: searchQuery.trim() || undefined,
      ...feePageSort,
      page: pageParam,
      page_size: 50,
    }),
  initialPageParam: 1,
  getNextPageParam: (lastPage) =>
    lastPage.page < lastPage.total_pages ? lastPage.page + 1 : undefined,
  enabled: !!orgId,
  staleTime: 2 * 60 * 1000,
}));
const refundPaymentsQuery = createPaymentsQuery(
  () => orgId,
  () => selectedFeeForRefund ? { fee_id: selectedFeeForRefund.id } : undefined,
  () => activeForm === "refund" && Boolean(selectedFeeForRefund),
);
const refundablePayments = $derived((refundPaymentsQuery.data ?? []).filter((payment) =>
  ["cash", "bank_transfer", "check", "other"].includes(payment.method.toLowerCase())
));
const refundPayment = $derived(refundablePayments.find((payment) => payment.id === refundPaymentId)
  ?? (refundablePayments.length === 1 ? refundablePayments[0] : undefined));

const feeSummaryQuery = createQuery(() => ({
  queryKey: queryKeys.finance.studentFeeSummary(orgId),
  queryFn: () => api.getStudentFeeSummary(),
  enabled: !!orgId,
  staleTime: 2 * 60 * 1000,
}));
const studentDirectoryQuery = createQuery(() => ({
  queryKey: queryKeys.finance.studentDirectory(orgId),
  queryFn: loadAllStudentDetails,
  enabled: !!orgId,
  staleTime: 5 * 60 * 1000,
}));
const studentNames = $derived.by(() => {
  const isArabic = isAr();
  return new Map(
    (studentDirectoryQuery.data ?? []).map(({ student, person }) => [
      String(student.id),
      isArabic
        ? person.full_name_ar || student.name_ar || person.full_name
        : person.full_name,
    ]),
  );
});
const students = $derived(
  enrollments.map((enrollment) => ({
    id: String(enrollment.id),
    student_id: String(enrollment.student_id),
    full_name: studentNames.get(String(enrollment.student_id)) ?? "",
    student_number: enrollment.student_number,
  })),
);
const fees = $derived(
  feesQuery.data?.pages.flatMap((page) => page.items) ?? [],
);
const feeTotals = $derived(feeSummaryQuery.data ?? []);
const gridLoading = $derived(
  isLoading ||
    (feesQuery.isLoading && !feesQuery.data),
);
const studentsById = $derived(
  new Map(students.map((student) => [student.student_id, student])),
);

async function loadMoreFees() {
  const result = await feesQuery.fetchNextPage();
  if (result.isError) throw result.error;
}

const feeTemplatesById = $derived(new Map(feeTemplates.map((template) => [template.id, template])));

function getFeeCurrency(fee: StudentFeeDto) {
  const template = feeTemplatesById.get(fee.fee_template_id);
  return (
    template?.currency ||
    (fee as StudentFeeDto & { currency?: string }).currency ||
    orgContext.baseCurrency ||
    ""
  );
}

function formatFeeAmount(amount: number, currency: string) {
  return currency ? formatCurrency(amount, currency) : amount.toLocaleString();
}

function getFeeTemplateName(fee: StudentFeeDto) {
  const template = feeTemplatesById.get(fee.fee_template_id);
  if (!template) return m.finance_ui_tuition_fees();
  return isAr() ? template.name_ar || template.name : template.name;
}

async function loadAllStudentDetails(): Promise<StudentDetailDto[]> {
  const pageSize = 100;
  const firstPage = await api.listStudents({ page: 1, page_size: pageSize });
  if (firstPage.total_pages <= 1) return firstPage.items;

  const items = [...firstPage.items];
  for (let page = 2; page <= firstPage.total_pages; page += 5) {
    const pages = await Promise.all(
      Array.from({ length: Math.min(5, firstPage.total_pages - page + 1) }, (_, index) =>
        api.listStudents({ page: page + index, page_size: pageSize }),
      ),
    );
    items.push(...pages.flatMap((result) => result.items));
  }
  return items;
}

onMount(async () => {
  if (orgId) {
    await loadData(false);
  }
});

function refreshFeeData() {
  return Promise.all([
    queryClient.invalidateQueries({ queryKey: queryKeys.finance.studentFeesForOrg(orgId) }),
    queryClient.invalidateQueries({ queryKey: queryKeys.finance.studentFeeSummary(orgId) }),
  ]);
}

async function loadData(refreshFees = true) {
  isLoading = true;
  try {
    const [templatesList, enrollmentsList, periodsList] =
      await Promise.all([
        api.listFeeTemplates(),
        api.listEnrollments ? api.listEnrollments() : Promise.resolve([]),
        api.listAcademicPeriods
          ? api.listAcademicPeriods()
          : Promise.resolve([]),
      ]);

    feeTemplates = asArray(templatesList);
    enrollments = asArray(enrollmentsList);
    periods = asArray(periodsList);

    if (periods.length > 0 && !formPeriodId) {
      formPeriodId = periods[0].id;
    }
    if (refreshFees) {
      await refreshFeeData();
    }
  } catch (e) {
    console.error("Failed to load fees master data:", e);
    toast.error(m.finance_ui_fees_load_failed());
  } finally {
    isLoading = false;
  }
}

// Summary metrics come from the full organization totals, independently of table pagination and filters.
const feeTotalsWithRate = $derived(
  feeTotals.map((total) => ({
    ...total,
    collectionRate:
      total.due > 0 ? Math.round((total.paid / total.due) * 100) : 0,
  })),
);

// ── Form Actions ───────────────────────────────────────────────────────
async function handleCreateFee() {
  if (isSubmitting) return;
  formError = "";
  if (!formStudentId) {
    formError = m.finance_ui_student_required();
    return;
  }
  if (!formTemplateId) {
    formError = m.finance_ui_fee_template_required();
    return;
  }
  if (!formPeriodId) {
    formError = m.finance_ui_period_placeholder();
    return;
  }

  const template = feeTemplates.find((item) => item.id === formTemplateId);
  const enrollment = students.find((student) => student.id === formStudentId);
  if (!template) {
    formError = m.finance_ui_fee_template_required();
    return;
  }
  if (!enrollment) {
    formError = m.finance_ui_student_required();
    return;
  }
  const origAmt = Number(template.default_amount);
  const discAmt = Number(formDiscountAmount || 0);
  if (!Number.isFinite(discAmt) || discAmt < 0 || discAmt > origAmt) {
    formError = m.finance_ui_discount_amount_invalid();
    return;
  }

  isSubmitting = true;
  try {
    await api.generateStudentFee({
      enrollment_id: formStudentId,
      student_id: enrollment.student_id,
      period_id: formPeriodId,
      fee_template_id: formTemplateId,
      discount_amount: discAmt > 0 ? String(discAmt) : undefined,
      due_date: formDueDate,
    });
    toast.success(m.finance_ui_fee_assigned_success());
    activeForm = "none";
    formStudentId = "";
    formTemplateId = "";
    formDiscountAmount = null;
    void refreshFeeData();
  } catch (err) {
    console.error("Failed to assign fee:", err);
    formError = extractErrorMessage(err) || m.finance_ui_fee_assignment_failed();
  } finally {
    isSubmitting = false;
  }
}

async function handleSplitInstallments() {
  if (isSubmitting) return;
  formError = "";
  if (!hasOrderedInstallmentDates(splitDueDates, splitCount)) {
    formError = m.finance_ui_installment_dates_invalid();
    return;
  }
  if (!selectedFeeForSplit) {
    formError = m.finance_ui_fee_installment_required();
    return;
  }

  isSubmitting = true;
  try {
    await api.splitStudentFee(selectedFeeForSplit.id, {
      num_installments: splitCount,
      due_dates: [...splitDueDates],
    });
    toast.success(m.finance_ui_fee_installment_success());
    activeForm = "none";
    selectedFeeForSplit = null;
    splitDueDates = [];
    void refreshFeeData();
  } catch (err) {
    console.error("Failed to split fee:", err);
    formError = extractErrorMessage(err) || m.finance_ui_fee_installment_failed();
  } finally {
    isSubmitting = false;
  }
}

async function handleRefundFee() {
  if (isSubmitting) return;
  formError = "";
  if (!selectedFeeForRefund) {
    formError = m.finance_ui_fee_refund_record_required();
    return;
  }
  if (!refundPayment) {
    formError = m.finance_ui_refund_payment_source();
    return;
  }
  if (refundAmount == null || !Number.isFinite(refundAmount) || refundAmount <= 0 || refundAmount > Number(selectedFeeForRefund.paid_amount)) {
    formError = m.finance_ui_refund_amount_invalid();
    return;
  }

  isSubmitting = true;
  try {
    await api.refundStudentFee(selectedFeeForRefund.id, {
      refund_amount: String(refundAmount),
      payment_id: refundPayment.id,
      notes: refundNotes.trim() || undefined,
    });
    toast.success(m.finance_ui_fee_refund_success());
    activeForm = "none";
    selectedFeeForRefund = null;
    refundAmount = null;
    refundPaymentId = null;
    void refreshPaymentData(queryClient, orgId);
    refundNotes = "";
    void refreshFeeData();
  } catch (err) {
    console.error("Failed to refund fee:", err);
    formError = extractErrorMessage(err) || m.finance_ui_fee_refund_failed();
  } finally {
    isSubmitting = false;
  }
}

// ── Grid Data ─────────────────────────────────────────────────────────
function toFeeGridRow(f: StudentFeeDto, idx: number, today: string) {
  const net = Number(f.net_amount ?? f.original_amount ?? 0);
  const paid = Number(f.paid_amount ?? 0);
  const balance = Number(f.balance_due ?? Math.max(0, net - paid));
  const curr = getFeeCurrency(f);

  let statusLabel = m.finance_ui_fee_status_unpaid();
  if (f.status === "paid" || balance <= 0) {
    statusLabel = m.finance_ui_fee_status_paid();
  } else if (paid > 0) {
    statusLabel = m.finance_ui_fee_status_partially_paid();
  } else if (f.due_date && f.due_date.slice(0, 10) < today) {
    statusLabel = m.finance_ui_fee_status_overdue();
  }

  return {
    ...f,
    __idx: idx + 1,
    student_display: studentNames.get(String(f.student_id)) ?? "—",
    status_key:
      f.status === "paid" || balance <= 0
        ? "paid"
        : paid > 0
          ? "partially_paid"
          : f.due_date && f.due_date.slice(0, 10) < today
            ? "overdue"
            : "pending",
    template_display: getFeeTemplateName(f),
    net_formatted: formatCurrency(net, curr),
    paid_formatted: formatCurrency(paid, curr),
    balance_formatted: formatCurrency(balance, curr),
    currency_display: curr,
    due_date_display: f.due_date ? formatDate(f.due_date) : "—",
    status_display: statusLabel,
  };
}

const gridSource = $derived.by(() => {
  const today = formatDateKey(new Date());
  return fees.map((fee, index) => toFeeGridRow(fee, index, today));
});


const detailsColumn = $derived(financeRecordColumn());
let detailsOpen = $state(false);
let detailRow = $state<(typeof gridSource)[number] | undefined>();

function handleRecordAction(event: GridActionEvent) {
  if (event.actionId !== "dblclick" && event.actionId !== "view") return;
  const row = gridSource.find((item) => item.id === event.row.id);
  if (!row) return;
  detailRow = row;
  detailsOpen = true;
}

const columns: OrbisColumnDef[] = [
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
    id: "student_display",
    pin: "colPinStart",
    prop: "student_display",
    header: m.finance_ui_student_name(),
    name: m.finance_ui_student_name(),
    width: 220,
    size: 220,
    readonly: true,
    cellClass: "font-medium",
  },
  {
    id: "template_display",
    prop: "template_display",
    header: m.finance_ui_fee_item(),
    name: m.finance_ui_fee_item(),
    width: 180,
    size: 180,
    readonly: true,
  },
  {
    id: "net_formatted",
    prop: "net_formatted",
    header: m.finance_ui_net_amount(),
    name: m.finance_ui_net_amount(),
    width: 130,
    size: 130,
    readonly: true,
    sortable: true,
    align: "end",
    cellClass: "font-mono font-bold",
  },
  {
    id: "paid_formatted",
    prop: "paid_formatted",
    header: m.finance_ui_paid_amount(),
    name: m.finance_ui_paid_amount(),
    width: 130,
    size: 130,
    readonly: true,
    sortable: true,
    align: "end",
    cellClass: "font-mono text-emerald-600 dark:text-emerald-400 font-semibold",
  },
  {
    id: "balance_formatted",
    prop: "balance_formatted",
    header: m.finance_ui_remaining_amount(),
    name: m.finance_ui_remaining_amount(),
    width: 130,
    size: 130,
    readonly: true,
    sortable: true,
    align: "end",
    cellClass: "font-mono text-rose-600 dark:text-rose-400 font-bold",
  },
  {
    id: "currency_display",
    prop: "currency_display",
    header: m.finance_ui_currency(),
    name: m.finance_ui_currency(),
    width: 80,
    size: 80,
    readonly: true,
    align: "center",
  },
  {
    id: "due_date_display",
    prop: "due_date_display",
    header: m.finance_ui_due_date(),
    name: m.finance_ui_due_date(),
    width: 120,
    size: 120,
    readonly: true,
    sortable: true,
    align: "center",
  },
  {
    id: "status_display",
    prop: "status_display",
    header: m.finance_ui_status(),
    name: m.finance_ui_status(),
    width: 100,
    size: 100,
    type: "badge",
    align: "center",
    readonly: true,
  },
];

async function exportCsv() {
  try {
    const allFees = await api.listStudentFees({
      ...feePageFilter,
      search: searchQuery.trim() || undefined,
    });
    const today = formatDateKey(new Date());
    const rows = allFees
      .map((fee, index) => toFeeGridRow(fee, index, today))
      .map((fee) => [
        fee.student_display,
        fee.template_display,
        fee.net_formatted,
        fee.paid_formatted,
        fee.balance_formatted,
        fee.currency_display,
        fee.due_date_display,
        fee.status_display,
      ]);
    const csv = [
      [
        m.finance_ui_student_name(),
        m.finance_ui_fee_item(),
        m.finance_ui_net_amount(),
        m.finance_ui_paid_amount(),
        m.finance_ui_balance_due(),
        m.finance_ui_currency(),
        m.finance_ui_due_date(),
        m.finance_ui_status(),
      ],
      ...rows,
    ]
      .map((row) =>
        row.map((v) => `"${String(v ?? "").replaceAll('"', '""')}"`).join(","),
      )
      .join("\n");
    const url = URL.createObjectURL(
      new Blob(["\ufeff" + csv], { type: "text/csv;charset=utf-8" }),
    );
    const link = document.createElement("a");
    link.href = url;
    link.download = `student-fees-${orgId}.csv`;
    link.click();
    URL.revokeObjectURL(url);
  } catch (error) {
    toast.error(extractErrorMessage(error));
  }
}
</script>

<div class="flex h-full min-h-0 w-full flex-col overflow-hidden p-6 font-sans bg-background/50">
  <div class="flex min-h-0 w-full flex-1 flex-col gap-4">
    
    <!-- Top KPI Metric Cards (Matching FinanceOverview Aesthetic) -->
    <div class="grid grid-cols-1 @min-[30rem]/workspace:grid-cols-2 @min-[60rem]/workspace:grid-cols-4 gap-4 shrink-0">
      <Card class="p-4 shadow-sm border-border/40 bg-card flex flex-col justify-between">
        <div class="flex items-center justify-between text-muted-foreground text-xs font-semibold">
          <span>{m.finance_ui_total_fees_due()}</span>
          <span class="h-2 w-2 rounded-full bg-primary"></span>
        </div>
        <div class="mt-2 text-xl font-mono font-bold text-foreground">
          {#if feeTotalsWithRate.length}
            {#each feeTotalsWithRate as totals (totals.currency || "unspecified")}
              <span class="block">{formatFeeAmount(totals.due, totals.currency)}</span>
            {/each}
          {:else}
            —
          {/if}
        </div>
      </Card>

      <Card class="p-4 shadow-sm border-border/40 bg-card flex flex-col justify-between">
        <div class="flex items-center justify-between text-muted-foreground text-xs font-semibold">
          <span>{m.finance_ui_total_collected()}</span>
          <span class="h-2 w-2 rounded-full bg-emerald-500"></span>
        </div>
        <div class="mt-2 text-xl font-mono font-bold text-emerald-600 dark:text-emerald-400">
          {#if feeTotalsWithRate.length}
            {#each feeTotalsWithRate as totals (totals.currency || "unspecified")}
              <span class="block">{formatFeeAmount(totals.paid, totals.currency)}</span>
            {/each}
          {:else}
            —
          {/if}
        </div>
      </Card>

      <Card class="p-4 shadow-sm border-border/40 bg-card flex flex-col justify-between">
        <div class="flex items-center justify-between text-muted-foreground text-xs font-semibold">
          <span>{m.finance_ui_remaining_to_collect()}</span>
          <span class="h-2 w-2 rounded-full bg-rose-500"></span>
        </div>
        <div class="mt-2 text-xl font-mono font-bold text-rose-600 dark:text-rose-400">
          {#if feeTotalsWithRate.length}
            {#each feeTotalsWithRate as totals (totals.currency || "unspecified")}
              <span class="block">{formatFeeAmount(totals.balance, totals.currency)}</span>
            {/each}
          {:else}
            —
          {/if}
        </div>
      </Card>

      <Card class="p-4 shadow-sm border-border/40 bg-card flex flex-col justify-between">
        <div class="flex items-center justify-between text-muted-foreground text-xs font-semibold">
          <span>{m.finance_ui_collection_rate()}</span>
          <span class="h-2 w-2 rounded-full bg-indigo-500"></span>
        </div>
        <div class="mt-2 text-xl font-mono font-bold text-indigo-600 dark:text-indigo-400">
          {#if feeTotalsWithRate.length}
            {#each feeTotalsWithRate as totals (totals.currency || "unspecified")}
              <span class="block">{totals.collectionRate}%{totals.currency ? ` (${totals.currency})` : ""}</span>
            {/each}
          {:else}
            —
          {/if}
        </div>
      </Card>
    </div>

    <!-- Action Toolbar -->
    <header class="flex shrink-0 items-center justify-between gap-4">
      <div class="flex items-center gap-2">
        <Button
          variant={activeForm === "create" ? "secondary" : "default"}
          size="sm"
          class="h-8 px-3 text-xs font-bold shrink-0 gap-1.5"
          onclick={() => { activeForm = activeForm === "create" ? "none" : "create"; }}
        >
          {#if activeForm === "create"}
            <XMark size={14} />
            <span>{m.finance_ui_cancel()}</span>
          {:else}
            <Plus size={14} />
            <span>{m.finance_ui_assign_new_fee()}</span>
          {/if}
        </Button>

        <Button
          variant={activeForm === "split" ? "secondary" : "outline"}
          size="sm"
          class="h-8 px-3 text-xs font-semibold shrink-0 gap-1.5"
          disabled={isSubmitting}
          onclick={() => { formError = ""; if (activeForm !== "split") setInstallmentCount(splitCount); activeForm = activeForm === "split" ? "none" : "split"; }}
        >
          <Calendar size={14} />
          <span>{m.finance_ui_installment_plan()}</span>
        </Button>

        <Button
          variant={activeForm === "refund" ? "secondary" : "outline"}
          size="sm"
          class="h-8 px-3 text-xs font-semibold shrink-0 gap-1.5"
          onclick={() => { activeForm = activeForm === "refund" ? "none" : "refund"; }}
        >
          <ArrowPath size={14} />
          <span>{m.finance_ui_refund_fees()}</span>
        </Button>
      </div>

      <div class="flex items-center gap-2">
        <Select
          type="single"
          value={statusFilter}
          onValueChange={(value) => {
            if (value === "all" || value === "pending" || value === "partially_paid" || value === "paid" || value === "overdue") {
              statusFilter = value;
            }
          }}
        >
          <SelectTrigger class="h-8 w-auto min-w-36 bg-card text-xs">
            {#if statusFilter === "all"}
              {m.finance_ui_all_statuses()}
            {:else if statusFilter === "pending"}
              {m.finance_ui_fee_status_unpaid()}
            {:else if statusFilter === "partially_paid"}
              {m.finance_ui_fee_status_partially_paid()}
            {:else if statusFilter === "paid"}
              {m.finance_ui_fee_status_paid_full()}
            {:else}
              {m.finance_ui_fee_status_overdue()}
            {/if}
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="all">{m.finance_ui_all_statuses()}</SelectItem>
            <SelectItem value="pending">{m.finance_ui_fee_status_unpaid()}</SelectItem>
            <SelectItem value="partially_paid">{m.finance_ui_fee_status_partially_paid()}</SelectItem>
            <SelectItem value="paid">{m.finance_ui_fee_status_paid_full()}</SelectItem>
            <SelectItem value="overdue">{m.finance_ui_fee_status_overdue()}</SelectItem>
          </SelectContent>
        </Select>

        <DateRangeFilter bind:value={dateRange} />

        <Button variant="ghost" size="icon-sm" onclick={() => loadData()} class="text-muted-foreground">
          {#if isLoading}<LoadingIndicator  size={14} />{:else}<Refresh size={14}   />{/if}
        </Button>

        <Button variant="ghost" size="icon-sm" onclick={exportCsv} class="text-muted-foreground">
          <Download size={14} />
        </Button>
      </div>
    </header>

    <!-- In-Page Creation Form Card -->
    {#if activeForm === "create"}
      <div>
        <FinanceInlineFormCard
          title={m.finance_ui_assign_fee_heading()}
          busy={isSubmitting}
          errorMessage={formError}
          onsave={handleCreateFee}
          onclose={() => { formError = ""; activeForm = "none"; }}
        >
            <div class="grid min-w-0 grid-cols-1 gap-4 sm:grid-cols-2 xl:grid-cols-3">
              <div class="min-w-0">
                <Label for="fee-student" class="text-xs font-semibold text-foreground mb-1 block">{m.finance_ui_student()}</Label>
                <SearchableSelect id="fee-student"
                  value={formStudentId}
                  placeholder={m.finance_ui_student_select_placeholder()}
                  items={students}
                  getItemValue={(s) => s.id}
                  getItemLabel={(s) =>
                    s.full_name || s.student_number || m.finance_ui_student()}
                  onchange={(val) => { if (val) formStudentId = val; }}
                  disabled={studentDirectoryQuery.isLoading || studentDirectoryQuery.isError}
                  class="w-full h-8 text-xs bg-background border-border/60"
                />
              </div>

              <div class="min-w-0">
                <Label for="fee-fee-template" class="text-xs font-semibold text-foreground mb-1 block">{m.finance_ui_fee_template()}</Label>
                <SearchableSelect id="fee-fee-template"
                  value={formTemplateId}
                  placeholder={m.finance_ui_fee_item_select_placeholder()}
                  items={feeTemplates}
                  getItemValue={(t) => t.id}
                  getItemLabel={(t) => `${t.name_ar || t.name} (${formatFeeAmount(Number(t.default_amount || 0), t.currency)})`}
                  onchange={(val) => { if (val) formTemplateId = val; }}
                  class="w-full h-8 text-xs bg-background border-border/60"
                />
              </div>

              <div class="min-w-0">
                <Label for="fee-academic-period" class="text-xs font-semibold text-foreground mb-1 block">{m.finance_ui_academic_period()}</Label>
                <SearchableSelect id="fee-academic-period"
                  value={formPeriodId}
                  placeholder={m.finance_ui_period_placeholder()}
                  items={periods}
                  getItemValue={(p) => p.id}
                  getItemLabel={(p) => p.name_ar || p.name}
                  onchange={(val) => { if (val) formPeriodId = val; }}
                  class="w-full h-8 text-xs bg-background border-border/60"
                />
              </div>
            </div>

            <div class="grid min-w-0 grid-cols-1 gap-4 sm:grid-cols-2">
              <div class="min-w-0">
                <Label for="fee-discount-amount-optional" class="text-xs font-semibold text-foreground mb-1 block">{m.finance_ui_discount_amount_optional()}</Label>
                <Input id="fee-discount-amount-optional"
                  type="number"
                  bind:value={formDiscountAmount}
                  placeholder="0.00"
                  class="h-8 text-xs font-mono bg-background border-border/60"
                />
              </div>

              <div class="min-w-0">
                <Label for="fee-due-date" class="text-xs font-semibold text-foreground mb-1 block">{m.finance_ui_due_date()}</Label>
                <Input id="fee-due-date"
                  type="date"
                  bind:value={formDueDate}
                  class="h-8 text-xs font-mono bg-background border-border/60"
                />
              </div>
            </div>

          {#snippet actions()}
            <div class="flex items-center justify-end gap-2 pt-2 border-t border-border/40">
              <Button type="button" variant="ghost" size="sm" class="h-8 text-xs" onclick={() => activeForm = "none"} disabled={isSubmitting}>
                {m.finance_ui_cancel()}
              </Button>
              <Button type="submit"
                variant="default"
                size="sm"
                class="h-8 text-xs font-bold px-4"
                disabled={isSubmitting}
              >
                {isSubmitting ? m.finance_ui_saving() : m.finance_ui_assign_fee_save()}
              </Button>
            </div>
          {/snippet}
        </FinanceInlineFormCard>
      </div>
    {/if}

    <!-- In-Page Installment Split Form Card -->
    {#if activeForm === "split"}
      <div>
        <FinanceInlineFormCard
          title={m.finance_ui_installment_heading()}
          accent="indigo"
          busy={isSubmitting}
          errorMessage={formError}
          onsave={handleSplitInstallments}
          onclose={() => { formError = ""; activeForm = "none"; }}
        >
            <div class="grid min-w-0 grid-cols-1 gap-4 sm:grid-cols-2">
              <div class="min-w-0">
                <Label for="fee-select-unpaid-fee" class="text-xs font-semibold text-foreground mb-1 block">{m.finance_ui_select_unpaid_fee()}</Label>
                <SearchableSelect id="fee-select-unpaid-fee"
                  value={selectedFeeForSplit?.id || ""}
                  placeholder={m.finance_ui_installment_fee_placeholder()}
                  items={fees.filter((f) =>
                    Number(f.paid_amount || 0) === 0 &&
                    Number(f.balance_due || f.net_amount || 0) > 0 &&
                    f.status !== "cancelled"
                  )}
                  getItemValue={(f) => f.id}
                  getItemLabel={(f) => {
                    const st = studentsById.get(String(f.student_id));
                    return m.finance_ui_installment_fee_option({
                      student: st?.full_name || st?.student_number || m.finance_ui_student(),
                      fee: getFeeTemplateName(f),
                      amount: formatFeeAmount(Number(f.net_amount || 0), getFeeCurrency(f)),
                    });
                  }}
                  onchange={(val) => {
                    selectedFeeForSplit = fees.find((f) => f.id === val) || null;
                  }}
                  class="w-full h-8 text-xs bg-background border-border/60"
                />
              </div>

              <div>
                <Label class="text-xs font-semibold text-foreground mb-1 block">{m.finance_ui_installment_count()}</Label>
                <Select
                  type="single"
                  value={String(splitCount)}
                  onValueChange={(value) => {
                    const count = Number(value);
                    if (Number.isInteger(count) && count >= 2 && count <= 6) setInstallmentCount(count);
                  }}
                >
                  <SelectTrigger class="h-8 w-full bg-background text-xs">
                    {m.finance_ui_installment_count_value({ count: splitCount })}
                  </SelectTrigger>
                  <SelectContent>
                    {#each [2, 3, 4, 5, 6] as count (count)}
                      <SelectItem value={String(count)}>
                        {m.finance_ui_installment_count_value({ count })}
                      </SelectItem>
                    {/each}
                  </SelectContent>
                </Select>
              </div>
            </div>

          <div class="grid gap-3 sm:grid-cols-2 lg:grid-cols-3">
            {#each Array(splitCount) as _, index (index)}
              <div class="space-y-1">
                <Label for={"installment-date-" + index}>{m.finance_ui_installment_due_date({ number: index + 1 })}</Label>
                <Input id={"installment-date-" + index} type="date" required value={splitDueDates[index] ?? ""}
                  onchange={(event) => {
                    if (!splitDueDates.length) setInstallmentCount(splitCount);
                    splitDueDates[index] = event.currentTarget.value;
                  }}
                />
              </div>
            {/each}
          </div>
          {#snippet actions()}
            <div class="flex items-center justify-end gap-2 pt-2 border-t border-border/40">
              <Button type="button" variant="ghost" size="sm" class="h-8 text-xs" onclick={() => activeForm = "none"} disabled={isSubmitting}>
                {m.finance_ui_cancel()}
              </Button>
              <Button type="submit"
                variant="default"
                size="sm"
                class="h-8 text-xs font-bold px-4"
                disabled={isSubmitting || !selectedFeeForSplit}
              >
                {isSubmitting ? m.finance_ui_processing() : m.finance_ui_apply_installment_plan()}
              </Button>
            </div>
          {/snippet}
        </FinanceInlineFormCard>
      </div>
    {/if}

    <!-- In-Page Refund Form Card -->
    {#if activeForm === "refund"}
      <div>
        <FinanceInlineFormCard
          title={m.finance_ui_refund_heading()}
          accent="rose"
          busy={isSubmitting}
          errorMessage={formError}
          onsave={handleRefundFee}
          onclose={() => { formError = ""; activeForm = "none"; }}
        >
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div>
                <Label for="fee-select-paid-fee" class="text-xs font-semibold text-foreground mb-1 block">{m.finance_ui_select_paid_fee()}</Label>
                <SearchableSelect id="fee-select-paid-fee"
                  value={selectedFeeForRefund?.id || ""}
                  placeholder={m.finance_ui_refund_fee_placeholder()}
                  items={fees.filter((f) => Number(f.paid_amount || 0) > 0)}
                  getItemValue={(f) => f.id}
                  getItemLabel={(f) => {
                    const st = studentsById.get(String(f.student_id));
                    return m.finance_ui_refund_fee_option({
                      student: st?.full_name || st?.student_number || m.finance_ui_student(),
                      fee: getFeeTemplateName(f),
                      amount: formatFeeAmount(Number(f.paid_amount || 0), getFeeCurrency(f)),
                    });
                  }}
                  onchange={(val) => {
                    selectedFeeForRefund = fees.find((f) => f.id === val) || null;
                    refundPaymentId = null;
                    if (selectedFeeForRefund) {
                      refundAmount = Number(selectedFeeForRefund.paid_amount || 0);
                    }
                  }}
                  class="w-full h-8 text-xs bg-background border-border/60"
                />
              </div>

              <div>
                <Label for="fee-refund-payment" class="mb-1 block text-xs font-semibold">{m.finance_ui_refund_payment_source()}</Label>
                <SearchableSelect id="fee-refund-payment"
                  items={refundablePayments}
                  value={refundPayment?.id ?? null}
                  disabled={!selectedFeeForRefund || refundPaymentsQuery.isFetching}
                  placeholder={refundPaymentsQuery.isFetching ? m.common_loading() : m.finance_ui_refund_payment_source()}
                  emptyText={m.finance_ui_refund_no_manual_payments()}
                  getItemLabel={(payment) => (payment.receipt_number || payment.id.slice(0, 8)) + " / " + formatFeeAmount(Number(payment.amount), payment.currency_code ?? "")}
                  onchange={(value) => {
                    refundPaymentId = value;
                    const payment = refundablePayments.find((item) => item.id === value);
                    if (payment && selectedFeeForRefund) refundAmount = Math.min(Number(payment.amount), Number(selectedFeeForRefund.paid_amount));
                  }}
                />
              </div>
              <div>
                <Label for="fee-refund-amount" class="text-xs font-semibold text-foreground mb-1 block">{m.finance_ui_refund_amount()}</Label>
                <Input id="fee-refund-amount"
                  type="number"
                  bind:value={refundAmount}
                  placeholder="0.00"
                  class="h-8 text-xs font-mono font-bold text-rose-600 bg-background border-border/60"
                />
              </div>
            </div>

            <div>
              <Label for="fee-refund-reason-notes" class="text-xs font-semibold text-foreground mb-1 block">{m.finance_ui_refund_reason_notes()}</Label>
              <Input id="fee-refund-reason-notes"
                type="text"
                bind:value={refundNotes}
                placeholder={m.finance_ui_refund_reason_placeholder()}
                class="h-8 text-xs bg-background border-border/60"
              />
            </div>

          {#snippet actions()}
            <div class="flex items-center justify-end gap-2 pt-2 border-t border-border/40">
              <Button type="button" variant="ghost" size="sm" class="h-8 text-xs" onclick={() => activeForm = "none"} disabled={isSubmitting}>
                {m.finance_ui_cancel()}
              </Button>
              <Button type="submit"
                variant="default"
                size="sm"
                class="h-8 text-xs font-bold px-4"
                disabled={isSubmitting || !selectedFeeForRefund}
              >
                {isSubmitting ? m.finance_ui_processing() : m.finance_ui_confirm_refund()}
              </Button>
            </div>
          {/snippet}
        </FinanceInlineFormCard>
      </div>
    {/if}

    {#if studentDirectoryQuery.isError}
      <div
        class="flex shrink-0 items-center justify-between gap-3 rounded-lg border border-destructive/30 bg-destructive/5 px-4 py-3 text-sm"
        role="alert"
      >
        <span>
          {m.finance_ui_fees_load_failed()}
          {extractErrorMessage(studentDirectoryQuery.error)}
        </span>
        <Button
          variant="outline"
          size="sm"
          onclick={() => studentDirectoryQuery.refetch()}
        >
          <Refresh size={14} />
          {m.finance_ui_refresh()}
        </Button>
      </div>
    {:else}
      <!-- Grid Container -->
      <div class="flex-1 min-h-0 overflow-hidden">
        <OrbisGrid
          columns={[...columns, detailsColumn]}
          onaction={handleRecordAction}
          source={gridSource}
          loading={gridLoading}
          serverQuery
          search={searchQuery}
          searchPlaceholder={m.finance_ui_fee_search_placeholder()}
          onquerychange={handleFeeGridQuery}
          rtl={isAr()}
          readonly
          emptyTitle={m.finance_ui_no_student_fees()}
          hasMore={feesQuery.hasNextPage ?? false}
          onloadmore={loadMoreFees}
        />
      </div>
    {/if}
  </div>
</div>

<FinanceRecordDialog
  bind:open={detailsOpen}
  title={m.finance_ui_record_details()}
  fields={detailRow ? [
    { label: m.finance_ui_student(), value: detailRow.student_display },
    { label: m.finance_ui_fee_template(), value: detailRow.template_display },
    { label: m.finance_ui_due_date(), value: detailRow.due_date_display },
    { label: m.finance_ui_net_amount(), value: detailRow.net_formatted },
    { label: m.finance_ui_paid_amount_label(), value: detailRow.paid_formatted },
    { label: m.finance_ui_full_balance(), value: detailRow.balance_formatted },
    { label: m.finance_ui_currency(), value: detailRow.currency_display },
    { label: m.finance_ui_receipt_status(), value: detailRow.status_display },
  ] : []}
/>
