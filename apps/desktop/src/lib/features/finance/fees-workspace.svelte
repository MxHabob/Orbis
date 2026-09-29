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
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import { api, queryClient } from "$lib/api";
import { formatCurrency, formatDate, isAr } from "$lib/core/i18n";
import type { StudentFeeDto, FeeTemplateDto, DiscountTemplateDto, AcademicPeriodDto } from "$lib/api/types";
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
  Calendar,
  ArrowPath,
} from "@orbis/icons";
import { OrbisGrid, type OrbisColumnDef } from "@orbis/svelte-grid";
import SearchableSelect from "$lib/shared/components/searchable-select.svelte";
import { fade, slide } from "svelte/transition";

let { orgId }: { orgId: string } = $props();

// ── State ──────────────────────────────────────────────────────────────
let fees = $state<StudentFeeDto[]>([]);
let feeTemplates = $state<FeeTemplateDto[]>([]);
let discountTemplates = $state<DiscountTemplateDto[]>([]);
let students = $state<any[]>([]);
let periods = $state<AcademicPeriodDto[]>([]);
let isLoading = $state(true);
let isSubmitting = $state(false);

// Active Action Form
let activeForm = $state<"none" | "create" | "split" | "refund">("none");

// Create Fee Form
let formStudentId = $state("");
let formPeriodId = $state("");
let formTemplateId = $state("");
let formDiscountAmount = $state<number | null>(null);
let formDueDate = $state(new Date(Date.now() + 30 * 86400000).toISOString().slice(0, 10));

// Split Installments Form
let selectedFeeForSplit = $state<StudentFeeDto | null>(null);
let splitCount = $state(2);
let splitDueDates = $state<string[]>([
  new Date(Date.now() + 30 * 86400000).toISOString().slice(0, 10),
  new Date(Date.now() + 60 * 86400000).toISOString().slice(0, 10),
]);

// Refund Form
let selectedFeeForRefund = $state<StudentFeeDto | null>(null);
let refundAmount = $state<number | null>(null);
let refundNotes = $state("");

// Filters
let searchQuery = $state("");
let statusFilter = $state<"all" | "pending" | "partially_paid" | "paid" | "overdue">("all");

onMount(async () => {
  if (orgId) {
    await loadData();
  }
});

async function loadData() {
  isLoading = true;
  try {
    const [feesList, templatesList, discountsList, enrollmentsList, periodsList] = await Promise.all([
      api.listStudentFees().catch(() => []),
      api.listFeeTemplates().catch(() => []),
      api.listDiscountTemplates().catch(() => []),
      api.listEnrollments ? api.listEnrollments().catch(() => []) : Promise.resolve([]),
      api.listAcademicPeriods ? api.listAcademicPeriods().catch(() => []) : Promise.resolve([]),
    ]);

    fees = feesList || [];
    feeTemplates = templatesList || [];
    discountTemplates = discountsList || [];
    students = (enrollmentsList || []).map((e: any) => ({
      id: String(e.id),
      full_name: e.full_name || e.name_ar || e.name_en || e.student_number || `طالب #${e.id}`,
    }));
    periods = periodsList || [];

    if (periods.length > 0 && !formPeriodId) {
      formPeriodId = periods[0].id;
    }
  } catch (e) {
    console.error("Failed to load fees master data:", e);
    toast.error("فشل تحميل بيانات الرسوم الدراسية");
  } finally {
    isLoading = false;
  }
}

// ── KPI Summary Calculations ───────────────────────────────────────────
const totalFeesAmount = $derived(
  fees.reduce((sum, f) => sum + Number(f.net_amount ?? f.original_amount ?? 0), 0)
);

const totalPaidAmount = $derived(
  fees.reduce((sum, f) => sum + Number(f.paid_amount ?? 0), 0)
);

const totalBalanceDue = $derived(
  Math.max(0, totalFeesAmount - totalPaidAmount)
);

const collectionRate = $derived(
  totalFeesAmount > 0 ? Math.round((totalPaidAmount / totalFeesAmount) * 100) : 0
);

// ── Form Actions ───────────────────────────────────────────────────────
async function handleCreateFee() {
  if (!formStudentId) {
    toast.error("يرجى اختيار الطالب");
    return;
  }
  if (!formTemplateId) {
    toast.error("يرجى اختيار قالب الرسوم");
    return;
  }

  isSubmitting = true;
  try {
    const template = feeTemplates.find((t) => t.id === formTemplateId);
    const origAmt = Number(template?.amount || 0);
    const discAmt = Number(formDiscountAmount || 0);
    const netAmt = Math.max(0, origAmt - discAmt);

    const payload = {
      enrollment_id: formStudentId,
      fee_template_id: formTemplateId,
      academic_period_id: formPeriodId || undefined,
      original_amount: origAmt,
      discount_amount: discAmt,
      net_amount: netAmt,
      paid_amount: 0,
      balance_due: netAmt,
      due_date: formDueDate,
      status: "unpaid",
    };

    await api.createStudentFee(payload as any);
    toast.success("تم تقييد الرسم الدراسي للطالب بنجاح");
    activeForm = "none";
    formStudentId = "";
    formTemplateId = "";
    formDiscountAmount = null;
    await loadData();
  } catch (err: any) {
    console.error("Failed to assign fee:", err);
    toast.error(err?.message || "فشل تقييد الرسم");
  } finally {
    isSubmitting = false;
  }
}

async function handleSplitInstallments() {
  if (!selectedFeeForSplit) {
    toast.error("يرجى اختيار الرسم المراد تقسيطه");
    return;
  }

  isSubmitting = true;
  try {
    await api.splitStudentFee(selectedFeeForSplit.id, {
      num_installments: splitCount,
      due_dates: splitDueDates.slice(0, splitCount),
    });
    toast.success("تم تقسيط الرسوم بنجاح");
    activeForm = "none";
    selectedFeeForSplit = null;
    await loadData();
  } catch (err: any) {
    console.error("Failed to split fee:", err);
    toast.error(err?.message || "فشل تقسيط الرسوم");
  } finally {
    isSubmitting = false;
  }
}

async function handleRefundFee() {
  if (!selectedFeeForRefund) {
    toast.error("يرجى اختيار السجل المراد استرداده");
    return;
  }
  if (!refundAmount || refundAmount <= 0) {
    toast.error("يرجى إدخال مبلغ استرداد صحيح");
    return;
  }

  isSubmitting = true;
  try {
    await api.refundStudentFee(selectedFeeForRefund.id, {
      refund_amount: Number(refundAmount),
      notes: refundNotes.trim() || undefined,
    });
    toast.success("تم تسجيل عملية الاسترداد بنجاح");
    activeForm = "none";
    selectedFeeForRefund = null;
    refundAmount = null;
    refundNotes = "";
    await loadData();
  } catch (err: any) {
    console.error("Failed to refund fee:", err);
    toast.error(err?.message || "فشل استرداد الرسوم");
  } finally {
    isSubmitting = false;
  }
}

// ── Grid Data ─────────────────────────────────────────────────────────
const gridSource = $derived(
  fees.map((f, idx) => {
    const student = students.find((s) => String(s.id) === String(f.enrollment_id));
    const template = feeTemplates.find((t) => t.id === f.fee_template_id);
    const templateName = template?.name_ar || template?.name || f.fee_type_name || "رسوم دراسية";

    const net = Number(f.net_amount ?? f.original_amount ?? 0);
    const paid = Number(f.paid_amount ?? 0);
    const balance = Number(f.balance_due ?? Math.max(0, net - paid));
    const curr = f.currency || "YER";

    let statusLabel = "غير مسدد";
    if (f.status === "paid" || balance <= 0) {
      statusLabel = "مسدد";
    } else if (paid > 0) {
      statusLabel = "مسدد جزئياً";
    } else if (f.due_date && new Date(f.due_date) < new Date()) {
      statusLabel = "متأخر";
    }

    return {
      ...f,
      __idx: idx + 1,
      student_display: student?.full_name || f.student_name || `طالب #${f.enrollment_id}`,
      template_display: templateName,
      net_formatted: formatCurrency(net, curr),
      paid_formatted: formatCurrency(paid, curr),
      balance_formatted: formatCurrency(balance, curr),
      currency_display: curr,
      due_date_display: f.due_date ? formatDate(f.due_date) : "—",
      status_display: statusLabel,
    };
  })
);

const filteredSource = $derived(
  gridSource.filter((row) => {
    if (statusFilter === "pending" && row.status_display !== "غير مسدد") return false;
    if (statusFilter === "partially_paid" && row.status_display !== "مسدد جزئياً") return false;
    if (statusFilter === "paid" && row.status_display !== "مسدد") return false;
    if (statusFilter === "overdue" && row.status_display !== "متأخر") return false;
    return true;
  })
);

const columns: OrbisColumnDef[] = [
  { id: "__idx", prop: "__idx", header: "#", name: "#", width: 48, size: 48, readonly: true, pin: "left", align: "center" },
  { id: "student_display", prop: "student_display", header: "اسم الطالب", name: "اسم الطالب", width: 220, size: 220, readonly: true, cellClass: "font-medium" },
  { id: "template_display", prop: "template_display", header: "بند الرسوم", name: "بند الرسوم", width: 180, size: 180, readonly: true },
  { id: "net_formatted", prop: "net_formatted", header: "المبلغ الصافي", name: "المبلغ الصافي", width: 130, size: 130, readonly: true, align: "end", cellClass: "font-mono font-bold" },
  { id: "paid_formatted", prop: "paid_formatted", header: "المسدد", name: "المسدد", width: 130, size: 130, readonly: true, align: "end", cellClass: "font-mono text-emerald-600 dark:text-emerald-400 font-semibold" },
  { id: "balance_formatted", prop: "balance_formatted", header: "المتبقي", name: "المتبقي", width: 130, size: 130, readonly: true, align: "end", cellClass: "font-mono text-rose-600 dark:text-rose-400 font-bold" },
  { id: "currency_display", prop: "currency_display", header: "العملة", name: "العملة", width: 80, size: 80, readonly: true, align: "center" },
  { id: "due_date_display", prop: "due_date_display", header: "تاريخ الاستحقاق", name: "تاريخ الاستحقاق", width: 120, size: 120, readonly: true, align: "center" },
  { id: "status_display", prop: "status_display", header: "الحالة", name: "الحالة", width: 100, size: 100, type: "badge", align: "center", readonly: true },
];

function exportCsv() {
  const rows = filteredSource.map((f) => [
    f.student_display,
    f.template_display,
    f.net_formatted,
    f.paid_formatted,
    f.balance_formatted,
    f.currency_display,
    f.due_date_display,
    f.status_display,
  ]);
  const csv = [
    ["Student", "Fee Item", "Net Amount", "Paid Amount", "Balance Due", "Currency", "Due Date", "Status"],
    ...rows,
  ]
    .map((row) => row.map((v) => `"${String(v ?? "").replaceAll('"', '""')}"`).join(","))
    .join("\n");
  const url = URL.createObjectURL(new Blob(["\ufeff" + csv], { type: "text/csv;charset=utf-8" }));
  const link = document.createElement("a");
  link.href = url;
  link.download = `student-fees-${orgId}.csv`;
  link.click();
  URL.revokeObjectURL(url);
}
</script>

<div class="flex h-full min-h-0 w-full flex-col overflow-hidden p-6 font-sans bg-background/50">
  <div class="flex min-h-0 w-full flex-1 flex-col gap-4" in:fade={{ duration: 150 }}>
    
    <!-- Top KPI Metric Cards (Matching FinanceOverview Aesthetic) -->
    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4 shrink-0">
      <Card class="p-4 shadow-sm border-border/40 bg-card flex flex-col justify-between">
        <div class="flex items-center justify-between text-muted-foreground text-xs font-semibold">
          <span>إجمالي الرسوم المستحقة</span>
          <span class="h-2 w-2 rounded-full bg-primary"></span>
        </div>
        <div class="mt-2 text-xl font-mono font-bold text-foreground">
          {formatCurrency(totalFeesAmount, "YER")}
        </div>
      </Card>

      <Card class="p-4 shadow-sm border-border/40 bg-card flex flex-col justify-between">
        <div class="flex items-center justify-between text-muted-foreground text-xs font-semibold">
          <span>إجمالي المحصل</span>
          <span class="h-2 w-2 rounded-full bg-emerald-500"></span>
        </div>
        <div class="mt-2 text-xl font-mono font-bold text-emerald-600 dark:text-emerald-400">
          {formatCurrency(totalPaidAmount, "YER")}
        </div>
      </Card>

      <Card class="p-4 shadow-sm border-border/40 bg-card flex flex-col justify-between">
        <div class="flex items-center justify-between text-muted-foreground text-xs font-semibold">
          <span>المتبقي للتحصيل</span>
          <span class="h-2 w-2 rounded-full bg-rose-500"></span>
        </div>
        <div class="mt-2 text-xl font-mono font-bold text-rose-600 dark:text-rose-400">
          {formatCurrency(totalBalanceDue, "YER")}
        </div>
      </Card>

      <Card class="p-4 shadow-sm border-border/40 bg-card flex flex-col justify-between">
        <div class="flex items-center justify-between text-muted-foreground text-xs font-semibold">
          <span>نسبة التحصيل</span>
          <span class="h-2 w-2 rounded-full bg-indigo-500"></span>
        </div>
        <div class="mt-2 text-xl font-mono font-bold text-indigo-600 dark:text-indigo-400">
          {collectionRate}%
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
            <span>إلغاء</span>
          {:else}
            <Plus size={14} />
            <span>تقييد رسم جديد</span>
          {/if}
        </Button>

        <Button
          variant={activeForm === "split" ? "secondary" : "outline"}
          size="sm"
          class="h-8 px-3 text-xs font-semibold shrink-0 gap-1.5"
          onclick={() => { activeForm = activeForm === "split" ? "none" : "split"; }}
        >
          <Calendar size={14} />
          <span>تقسيط الرسوم</span>
        </Button>

        <Button
          variant={activeForm === "refund" ? "secondary" : "outline"}
          size="sm"
          class="h-8 px-3 text-xs font-semibold shrink-0 gap-1.5"
          onclick={() => { activeForm = activeForm === "refund" ? "none" : "refund"; }}
        >
          <ArrowPath size={14} />
          <span>استرداد رسوم</span>
        </Button>
      </div>

      <div class="flex items-center gap-2">
        <div class="relative w-48 sm:w-64">
          <Search size={14} class="absolute inset-s-2.5 top-1/2 -translate-y-1/2 text-muted-foreground pointer-events-none" />
          <Input
            type="text"
            placeholder="بحث باسم الطالب أو بند الرسوم..."
            bind:value={searchQuery}
            class="h-8 ps-8 text-xs bg-card border-border/60"
          />
        </div>

        <select
          bind:value={statusFilter}
          class="h-8 text-xs rounded-md bg-card border border-border/60 px-2 text-foreground focus:outline-none"
        >
          <option value="all">جميع الحالات</option>
          <option value="pending">غير مسدد</option>
          <option value="partially_paid">مسدد جزئياً</option>
          <option value="paid">مسدد بالكامل</option>
          <option value="overdue">متأخر</option>
        </select>

        <Button variant="ghost" size="icon-sm" onclick={loadData} class="text-muted-foreground">
          <Refresh size={14} class={isLoading ? "animate-spin" : ""} />
        </Button>

        <Button variant="ghost" size="icon-sm" onclick={exportCsv} class="text-muted-foreground">
          <Download size={14} />
        </Button>
      </div>
    </header>

    <!-- In-Page Creation Form Card -->
    {#if activeForm === "create"}
      <div transition:slide={{ duration: 200 }} class="shrink-0">
        <Card class="border border-primary/20 bg-card shadow-sm text-start">
          <CardHeader class="py-3 px-4 border-b border-border/40">
            <div class="flex items-center justify-between">
              <div class="flex items-center gap-2">
                <span class="h-2 w-2 rounded-full bg-primary"></span>
                <CardTitle class="text-xs font-bold uppercase tracking-wider text-foreground">
                  تقييد رسم دراسي جديد على طالب
                </CardTitle>
              </div>
              <Button variant="ghost" size="icon-xs" onclick={() => activeForm = "none"} class="text-muted-foreground">
                <XMark size={14} />
              </Button>
            </div>
          </CardHeader>

          <CardContent class="p-4 flex flex-col gap-4">
            <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
              <div>
                <Label class="text-xs font-semibold text-foreground mb-1 block">الطالب</Label>
                <SearchableSelect
                  value={formStudentId}
                  placeholder="اختر الطالب..."
                  items={students}
                  getItemValue={(s) => s.id}
                  getItemLabel={(s) => s.full_name}
                  onchange={(val) => { if (val) formStudentId = val; }}
                  class="w-full h-8 text-xs bg-background border-border/60"
                />
              </div>

              <div>
                <Label class="text-xs font-semibold text-foreground mb-1 block">قالب الرسوم</Label>
                <SearchableSelect
                  value={formTemplateId}
                  placeholder="اختر بند الرسوم..."
                  items={feeTemplates}
                  getItemValue={(t) => t.id}
                  getItemLabel={(t) => `${t.name_ar || t.name} (${formatCurrency(Number(t.amount || 0), "YER")})`}
                  onchange={(val) => { if (val) formTemplateId = val; }}
                  class="w-full h-8 text-xs bg-background border-border/60"
                />
              </div>

              <div>
                <Label class="text-xs font-semibold text-foreground mb-1 block">الفترة الأكاديمية</Label>
                <SearchableSelect
                  value={formPeriodId}
                  placeholder="الفترة..."
                  items={periods}
                  getItemValue={(p) => p.id}
                  getItemLabel={(p) => p.name_ar || p.name}
                  onchange={(val) => { if (val) formPeriodId = val; }}
                  class="w-full h-8 text-xs bg-background border-border/60"
                />
              </div>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div>
                <Label class="text-xs font-semibold text-foreground mb-1 block">مبلغ الخصم (اختياري)</Label>
                <Input
                  type="number"
                  bind:value={formDiscountAmount}
                  placeholder="0.00"
                  class="h-8 text-xs font-mono bg-background border-border/60"
                />
              </div>

              <div>
                <Label class="text-xs font-semibold text-foreground mb-1 block">تاريخ الاستحقاق</Label>
                <Input
                  type="date"
                  bind:value={formDueDate}
                  class="h-8 text-xs font-mono bg-background border-border/60"
                />
              </div>
            </div>

            <div class="flex items-center justify-end gap-2 pt-2 border-t border-border/40">
              <Button variant="ghost" size="sm" class="h-8 text-xs" onclick={() => activeForm = "none"}>
                إلغاء
              </Button>
              <Button
                variant="default"
                size="sm"
                class="h-8 text-xs font-bold px-4"
                disabled={isSubmitting}
                onclick={handleCreateFee}
              >
                {isSubmitting ? "جاري الحفظ..." : "حفظ وتقييد الرسم"}
              </Button>
            </div>
          </CardContent>
        </Card>
      </div>
    {/if}

    <!-- In-Page Installment Split Form Card -->
    {#if activeForm === "split"}
      <div transition:slide={{ duration: 200 }} class="shrink-0">
        <Card class="border border-indigo-500/20 bg-card shadow-sm text-start">
          <CardHeader class="py-3 px-4 border-b border-border/40">
            <div class="flex items-center justify-between">
              <div class="flex items-center gap-2">
                <span class="h-2 w-2 rounded-full bg-indigo-500"></span>
                <CardTitle class="text-xs font-bold uppercase tracking-wider text-foreground">
                  تقسيط الرسوم الدراسية إلى دفعات
                </CardTitle>
              </div>
              <Button variant="ghost" size="icon-xs" onclick={() => activeForm = "none"} class="text-muted-foreground">
                <XMark size={14} />
              </Button>
            </div>
          </CardHeader>

          <CardContent class="p-4 flex flex-col gap-4">
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div>
                <Label class="text-xs font-semibold text-foreground mb-1 block">اختر الرسم غير المسدد</Label>
                <SearchableSelect
                  value={selectedFeeForSplit?.id || ""}
                  placeholder="اختر الرسم المراد تقسيطه..."
                  items={fees.filter((f) => f.status === "unpaid" && Number(f.paid_amount || 0) === 0)}
                  getItemValue={(f) => f.id}
                  getItemLabel={(f) => {
                    const st = students.find((s) => String(s.id) === String(f.enrollment_id));
                    return `${st?.full_name || 'طالب'} - ${f.fee_type_name || 'رسوم'} (${formatCurrency(Number(f.net_amount || 0), 'YER')})`;
                  }}
                  onchange={(val) => {
                    selectedFeeForSplit = fees.find((f) => f.id === val) || null;
                  }}
                  class="w-full h-8 text-xs bg-background border-border/60"
                />
              </div>

              <div>
                <Label class="text-xs font-semibold text-foreground mb-1 block">عدد الأقساط</Label>
                <select
                  bind:value={splitCount}
                  class="w-full h-8 text-xs rounded-md bg-background border border-border/60 px-2 text-foreground focus:outline-none"
                >
                  <option value={2}>قسطين (2)</option>
                  <option value={3}>3 أقساط</option>
                  <option value={4}>4 أقساط</option>
                  <option value={5}>5 أقساط</option>
                  <option value={6}>6 أقساط</option>
                </select>
              </div>
            </div>

            <div class="flex items-center justify-end gap-2 pt-2 border-t border-border/40">
              <Button variant="ghost" size="sm" class="h-8 text-xs" onclick={() => activeForm = "none"}>
                إلغاء
              </Button>
              <Button
                variant="default"
                size="sm"
                class="h-8 text-xs font-bold px-4"
                disabled={isSubmitting || !selectedFeeForSplit}
                onclick={handleSplitInstallments}
              >
                {isSubmitting ? "جاري المعالجة..." : "تطبيق التقسيط"}
              </Button>
            </div>
          </CardContent>
        </Card>
      </div>
    {/if}

    <!-- In-Page Refund Form Card -->
    {#if activeForm === "refund"}
      <div transition:slide={{ duration: 200 }} class="shrink-0">
        <Card class="border border-rose-500/20 bg-card shadow-sm text-start">
          <CardHeader class="py-3 px-4 border-b border-border/40">
            <div class="flex items-center justify-between">
              <div class="flex items-center gap-2">
                <span class="h-2 w-2 rounded-full bg-rose-500"></span>
                <CardTitle class="text-xs font-bold uppercase tracking-wider text-foreground">
                  استرداد رسوم دراسية مسددة
                </CardTitle>
              </div>
              <Button variant="ghost" size="icon-xs" onclick={() => activeForm = "none"} class="text-muted-foreground">
                <XMark size={14} />
              </Button>
            </div>
          </CardHeader>

          <CardContent class="p-4 flex flex-col gap-4">
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div>
                <Label class="text-xs font-semibold text-foreground mb-1 block">اختر الرسم المسدد</Label>
                <SearchableSelect
                  value={selectedFeeForRefund?.id || ""}
                  placeholder="اختر الرسم المراد استرداده..."
                  items={fees.filter((f) => Number(f.paid_amount || 0) > 0)}
                  getItemValue={(f) => f.id}
                  getItemLabel={(f) => {
                    const st = students.find((s) => String(s.id) === String(f.enrollment_id));
                    return `${st?.full_name || 'طالب'} - ${f.fee_type_name || 'رسوم'} (مسدد: ${formatCurrency(Number(f.paid_amount || 0), 'YER')})`;
                  }}
                  onchange={(val) => {
                    selectedFeeForRefund = fees.find((f) => f.id === val) || null;
                    if (selectedFeeForRefund) {
                      refundAmount = Number(selectedFeeForRefund.paid_amount || 0);
                    }
                  }}
                  class="w-full h-8 text-xs bg-background border-border/60"
                />
              </div>

              <div>
                <Label class="text-xs font-semibold text-foreground mb-1 block">مبلغ الاسترداد</Label>
                <Input
                  type="number"
                  bind:value={refundAmount}
                  placeholder="0.00"
                  class="h-8 text-xs font-mono font-bold text-rose-600 bg-background border-border/60"
                />
              </div>
            </div>

            <div>
              <Label class="text-xs font-semibold text-foreground mb-1 block">سبب الاسترداد / ملاحظات</Label>
              <Input
                type="text"
                bind:value={refundNotes}
                placeholder="أدخل سبب الاسترداد..."
                class="h-8 text-xs bg-background border-border/60"
              />
            </div>

            <div class="flex items-center justify-end gap-2 pt-2 border-t border-border/40">
              <Button variant="ghost" size="sm" class="h-8 text-xs" onclick={() => activeForm = "none"}>
                إلغاء
              </Button>
              <Button
                variant="default"
                size="sm"
                class="h-8 text-xs font-bold px-4"
                disabled={isSubmitting || !selectedFeeForRefund}
                onclick={handleRefundFee}
              >
                {isSubmitting ? "جاري التنفيذ..." : "تأكيد الاسترداد"}
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
        emptyTitle="لا توجد رسوم دراسية مسجلة"
      />
    </div>
  </div>
</div>
