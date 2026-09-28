<!-- Payments management and voucher recording workspace component. -->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import { api, queryClient } from "$lib/api";
import { Badge } from "@orbis/ui/badge";
import {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardContent,
  CardFooter,
  CardAction,
} from "@orbis/ui/card";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
import { Skeleton } from "@orbis/ui/skeleton";
import SearchableSelect from "$lib/shared/components/searchable-select.svelte";
import { OrbisGrid, type OrbisColumnDef } from "@orbis/svelte-grid";
import { Select, SelectContent, SelectItem, SelectTrigger, toast } from "@orbis/ui";
import { CheckSquare, ChevronLeft, ChevronRight, Document, Plus, Printer, Search, SignOut, TaskList, Trash } from "@orbis/icons";
import { formatCurrency, formatDate } from "$lib/core/i18n";
import { fade, slide } from "svelte/transition";
import PdfReportViewer from "$lib/shared/components/pdf-report-viewer.svelte";

let { orgId }: { orgId: string } = $props();

// Core State
let payments = $state<any[]>([]);
let printDialogOpen = $state(false);
let showQrModal = $state(false);
let selectedPaymentIds = $state<Set<string>>(new Set());

function togglePaymentSelection(id: string) {
  if (selectedPaymentIds.has(id)) {
    selectedPaymentIds.delete(id);
    selectedPaymentIds = new Set(selectedPaymentIds);
  } else {
    selectedPaymentIds.add(id);
    selectedPaymentIds = new Set(selectedPaymentIds);
  }
}

function toggleSelectAllPayments() {
  const visible = filteredPayments;
  if (selectedPaymentIds.size === visible.length) {
    selectedPaymentIds = new Set();
  } else {
    selectedPaymentIds = new Set(visible.map((p) => String(p.id)));
  }
}
let treasuries = $state<any[]>([]);
let students = $state<any[]>([]);
let currencies = $state<{ code: string; name: string }[]>([
  { code: "YER", name: "ريال يمني" },
  { code: "USD", name: "دولار أمريكي" },
  { code: "SAR", name: "ريال سعودي" },
]);
let isLoading = $state(true);
let error = $state<string | null>(null);

// Helper to extract only the actual account name (ignoring parent prefixes)
function cleanAccountName(name: string): string {
  if (!name) return "";
  const parts = name.split(" - ");
  return parts[parts.length - 1];
}

// Form Fields State
let selectedPayment = $state<any | null>(null);
let receiptNumber = $state("");
let paymentDate = $state(new Date().toISOString().split("T")[0]);
let treasuryId = $state("");
let paymentMethod = $state("Cash");
let amount = $state<number | null>(null);
let currency = $state("YER");
let checkInfo = $state("");
let notes = $state("");

// AdjustmentsHorizontal & Sub-bar Options
let searchQuery = $state("");
let methodFilter = $state("all");
let showUserRecords = $state(true);
let filterMode = $state<"day" | "range" | "all">("day");
let selectedFilterDate = $state("");
let startDateFilter = $state("");
let endDateFilter = $state("");

interface PaymentRow {
  studentEnrollmentId: string;
  notes: string;
  availableFees: any[];
  isLoadingFees: boolean;
  selectedFees: Set<string>;
  feeAmounts: Record<string, number>;
}

let paymentRows = $state<PaymentRow[]>([
  {
    studentEnrollmentId: "",
    notes: "",
    availableFees: [],
    isLoadingFees: false,
    selectedFees: new Set<string>(),
    feeAmounts: {},
  },
]);

// Derived States for Dropdowns
const studentItems = $derived.by(() => {
  return students.map((s) => ({
    id: String(s.id),
    value: String(s.id),
    label: s.full_name || s.student_number || String(s.id),
  }));
});

const treasuryItems = $derived.by(() => {
  return treasuries.map((tr) => ({
    value: tr.id,
    label: `${tr.code || "12110013"} - ${cleanAccountName(tr.name_ar || tr.name)}`,
  }));
});

const studentName = $derived(
  selectedPayment?.student_name ||
    getStudentName(paymentRows[0]?.studentEnrollmentId) ||
    "",
);

// Dialog and Printing states
let printReportOpen = $state(false);
let printTemplateId = $state("");
let printParameters = $state({});

const paymentMethods = [
  { value: "Cash", label: m.finance_pos_cash() },
  { value: "Check", label: m.finance_payments_check() },
  { value: "Transfer", label: "حوالة بنكية" },
];

async function loadData() {
  isLoading = true;
  error = null;
  try {
    const [paymentsList, treasuriesList, studentsList] = await Promise.all([
      api.listPayments(null, null),
      api
        .listAccounts()
        .then((accounts) =>
          accounts.filter(
            (account: any) =>
              ["cash", "bank"].includes(account.sub_type) &&
              account.is_active === 1 &&
              account.is_control_account !== 1,
          ),
        ),
      api.listEnrollments(),
      loadCurrencies(),
    ]);

    payments = paymentsList || [];

    treasuries = treasuriesList;
    students = studentsList;

    if (treasuries.length > 0 && !treasuryId) {
      treasuryId = treasuries[0].id;
    }
  } catch (e) {
    console.error("Error loading data:", e);
    error = e instanceof Error ? e.message : String(e);
  } finally {
    isLoading = false;
  }
}

async function loadCurrencies() {
  try {
    const list = await api.listCurrencies();
    if (list && list.length > 0) {
      currencies = list.map((c) => ({
        code: c.code,
        name: c.name_ar || c.name || c.code,
      }));
    }
  } catch (e) {
    console.error("Failed to load currencies:", e);
  }
}

onMount(() => {
  if (orgId) {
    loadData();
    const todayStr = new Date().toISOString().split("T")[0];
    selectedFilterDate = todayStr;

    const today = new Date();
    const thirtyDaysAgo = new Date();
    thirtyDaysAgo.setDate(today.getDate() - 30);
    startDateFilter = thirtyDaysAgo.toISOString().split("T")[0];
    endDateFilter = todayStr;
  }
});

// Date filtering navigation
function nextDay() {
  if (!selectedFilterDate) return;
  const d = new Date(selectedFilterDate);
  d.setDate(d.getDate() + 1);
  selectedFilterDate = d.toISOString().split("T")[0];
}

function prevDay() {
  if (!selectedFilterDate) return;
  const d = new Date(selectedFilterDate);
  d.setDate(d.getDate() - 1);
  selectedFilterDate = d.toISOString().split("T")[0];
}

function jumpToToday() {
  selectedFilterDate = new Date().toISOString().split("T")[0];
}

function getStudentName(enrollId: string): string {
  const s = students.find((student) => String(student.id) === enrollId);
  return s ? s.full_name : "";
}

function addPaymentRow() {
  paymentRows = [
    ...paymentRows,
    {
      studentEnrollmentId: "",
      notes: "",
      availableFees: [],
      isLoadingFees: false,
      selectedFees: new Set<string>(),
      feeAmounts: {},
    },
  ];
}

function deletePaymentRow(index: number) {
  if (paymentRows.length <= 1) {
    toast.warning("يجب أن يحتوي السند على سطر واحد على الأقل");
    return;
  }
  paymentRows = paymentRows.filter((_, i) => i !== index);
}

async function handleStudentChange(index: number, studentId: string) {
  paymentRows[index].studentEnrollmentId = studentId;
  paymentRows[index].notes = "";
  paymentRows[index].availableFees = [];
  paymentRows[index].selectedFees = new Set<string>();
  paymentRows[index].feeAmounts = {};

  if (!studentId || !orgId) return;

  paymentRows[index].isLoadingFees = true;
  try {
    const list = await api.listStudentFees(studentId);
    paymentRows[index].availableFees = list.filter((f) => f.status !== "paid");
  } catch (e) {
    console.error("Failed to load student fees for row:", e);
    paymentRows[index].availableFees = [];
  } finally {
    paymentRows[index].isLoadingFees = false;
  }
}

function handleToggleFee(rowIndex: number, fee: any) {
  const row = paymentRows[rowIndex];
  const feeId = String(fee.id);
  if (row.selectedFees.has(feeId)) {
    row.selectedFees.delete(feeId);
    delete row.feeAmounts[feeId];
  } else {
    row.selectedFees.add(feeId);
    row.feeAmounts[feeId] = fee.balance || fee.total_amount || 0;
  }
  row.selectedFees = new Set(row.selectedFees);
}

function handleFeeAmountChange(rowIndex: number, feeId: string, val: number) {
  paymentRows[rowIndex].feeAmounts[feeId] = val;
}

function calculateRowTotal(row: PaymentRow): number {
  let sum = 0;
  for (const feeId of row.selectedFees) {
    sum += Number(row.feeAmounts[feeId] || 0);
  }
  return sum;
}

async function selectRow(p: any) {
  selectedPayment = p;
  receiptNumber = p.receipt_number || p.id.slice(0, 8);
  paymentDate = p.payment_date
    ? p.payment_date.split("T")[0]
    : p.created_at
      ? p.created_at.split("T")[0]
      : "";
  treasuryId = p.cash_account_id || "";
  paymentMethod = p.payment_method || "Cash";
  currency = p.currency || "YER";
  checkInfo = p.check_info || "";

  let enrollId = "";
  const student = students.find((s) => s.full_name === p.student_name);
  if (student) {
    enrollId = String(student.id);
  } else {
    enrollId = p.enrollment_id || "";
  }

  let rowFees: any[] = [];
  if (enrollId && orgId) {
    try {
      const list = await api.listStudentFees(enrollId);
      rowFees = list.filter(
        (f) => f.status !== "paid" || String(f.id) === String(p.fee_id),
      );
    } catch (e) {
      console.error(e);
    }
  }

  const selectedSet = new Set<string>();
  const amountsMap: Record<string, number> = {};
  if (p.fee_id) {
    selectedSet.add(String(p.fee_id));
    amountsMap[String(p.fee_id)] = p.amount;
  }

  paymentRows = [
    {
      studentEnrollmentId: enrollId,
      notes: p.notes || "",
      availableFees: rowFees,
      isLoadingFees: false,
      selectedFees: selectedSet,
      feeAmounts: amountsMap,
    },
  ];
}

function clearForm() {
  selectedPayment = null;
  receiptNumber = "";
  paymentDate = new Date().toISOString().split("T")[0];
  if (treasuries.length > 0) {
    treasuryId = treasuries[0].id;
  }
  paymentMethod = "Cash";
  currency = "YER";
  checkInfo = "";
  paymentRows = [
    {
      studentEnrollmentId: "",
      notes: "",
      availableFees: [],
      isLoadingFees: false,
      selectedFees: new Set<string>(),
      feeAmounts: {},
    },
  ];
}

async function handleAdd() {
  let hasCheckedFee = false;
  for (let i = 0; i < paymentRows.length; i++) {
    const row = paymentRows[i];
    const rowNum = i + 1;
    if (!row.studentEnrollmentId) {
      toast.error(`الرجاء اختيار الطالب في السطر ${rowNum}`);
      return;
    }
    if (row.selectedFees.size > 0) {
      hasCheckedFee = true;
      for (const feeId of row.selectedFees) {
        const amt = row.feeAmounts[feeId];
        if (amt === undefined || amt === null || amt <= 0) {
          const fee = row.availableFees.find((f) => String(f.id) === feeId);
          toast.error(
            `الرجاء إدخال مبلغ دفع صحيح للرسم "${fee?.notes || "رسوم"}" في السطر ${rowNum}`,
          );
          return;
        }
      }
    }
  }

  if (!hasCheckedFee) {
    toast.error("الرجاء تحديد رسوم مستحقة واحدة على الأقل للسداد");
    return;
  }

  if (!treasuryId) {
    toast.error("الرجاء اختيار الصندوق");
    return;
  }

  const randomRef = String(Math.floor(11100 + Math.random() * 1000));
  const newRefNum = `005${Math.floor(1000 + Math.random() * 9000)}`;

  try {
    for (const row of paymentRows) {
      for (const feeId of row.selectedFees) {
        const feeAmount = row.feeAmounts[feeId];
        await api.processPayment({
          fee_id: feeId,
          amount: Number(feeAmount),
          paid_at: new Date().toISOString(),
          method:
            paymentMethod === "Cash"
              ? "cash"
              : paymentMethod === "Check"
                ? "bank_transfer"
                : "card",
          reference: newRefNum,
        });
      }
    }

    toast.success(`تم تسجيل سندات قبض بنجاح برقم ${randomRef}`);
    await Promise.all([
      queryClient.invalidateQueries({ queryKey: ["payments", orgId] }),
      queryClient.invalidateQueries({ queryKey: ["student-fees", orgId] }),
    ]);
    await loadData();
    clearForm();
  } catch (e) {
    console.error("Backend payment recording failed:", e);
    toast.error(e instanceof Error ? e.message : String(e));
  }
}

function handleVoid() {
  if (!selectedPayment) {
    toast.error("الرجاء تحديد سند من الجدول لإلغائه");
    return;
  }
  toast.error("إلغاء السند يتطلب إنشاء قيد عكسي من النظام المحاسبي");
}

function handlePrintList() {
  printTemplateId = "22222222-2222-2222-2222-222222222222";
  printParameters = {
    method: methodFilter,
    start_date: startDateFilter,
    end_date: endDateFilter,
  };
  printReportOpen = true;
}

function handlePrintVoucher() {
  if (!selectedPayment) {
    toast.error("الرجاء اختيار سند من الجدول أولاً");
    return;
  }
  printTemplateId = "11111111-1111-1111-1111-111111111111";
  printParameters = { payment_id: selectedPayment.id };
  printReportOpen = true;
}

function getMethodLabel(methodStr: string) {
  const methodLower = String(methodStr).toLowerCase();
  if (methodLower === "cash") return m.finance_pos_cash();
  if (
    methodLower === "check" ||
    methodLower === "bank_transfer" ||
    methodLower === "bank transfer"
  )
    return "شيك / تحويل";
  return methodStr;
}

// Calculations
const stats = $derived.by(() => {
  let total = 0;
  for (const p of filteredPayments) {
    total += Number(p.amount || 0);
  }
  return {
    total,
    count: filteredPayments.length,
  };
});

// Reactive filtering
const filteredPayments = $derived.by(() => {
  return payments.filter((p) => {
    let matchesSearch = true;
    if (searchQuery) {
      const q = searchQuery.toLowerCase().trim();
      const stName = String(p.student_name || "").toLowerCase();
      const receiptNum = String(p.receipt_number || p.id || "").toLowerCase();
      const noteStr = String(p.notes || "").toLowerCase();
      matchesSearch =
        stName.includes(q) || receiptNum.includes(q) || noteStr.includes(q);
    }

    let matchesMethod = true;
    if (methodFilter !== "all") {
      const meth = String(p.payment_method || "").toLowerCase();
      matchesMethod = meth === methodFilter.toLowerCase();
    }

    let dateMatch = true;
    const entryDate = p.payment_date
      ? p.payment_date.split("T")[0]
      : p.created_at
        ? p.created_at.split("T")[0]
        : "";
    if (filterMode === "day") {
      dateMatch = entryDate === selectedFilterDate;
    } else if (filterMode === "range" && startDateFilter && endDateFilter) {
      dateMatch = entryDate >= startDateFilter && entryDate <= endDateFilter;
    }

    return matchesSearch && matchesMethod && dateMatch;
  });
});

const paymentGridSource = $derived(
  filteredPayments.map((p, idx) => ({
    ...p,
    __idx: idx + 1,
    receipt_display: p.receipt_number || p.reference || p.id?.slice(0, 8),
    date_display: formatDate(p.payment_date || p.paid_at || p.created_at, "date"),
    method_display: getMethodLabel(p.payment_method || p.method || "Cash"),
    amount_display: formatCurrency(Number(p.amount || 0), p.currency || "YER"),
    currency_display: p.currency || "YER",
    cash_account_display: cleanAccountName(p.cash_account_name) || treasuries.find((t) => t.id === (p.cash_account_id || p.account_id))?.name || m.finance_payments_collections_cash_account(),
    student_display: p.student_name || p.payer_name || "—",
    notes_display: p.notes || "—",
    created_at_display: p.created_at ? formatDate(p.created_at, "datetime") : "—",
    updated_at_display: p.updated_at ? formatDate(p.updated_at, "datetime") : "—",
  }))
);

const paymentColumns: OrbisColumnDef<any>[] = [
  {
    id: "receipt_number",
    header: m.common_word_number(),
    accessorKey: "receipt_display",
    width: 100,
    align: "center",
    sortable: true,
  },
  {
    id: "date",
    header: m.common_overview_date(),
    accessorKey: "date_display",
    width: 110,
    align: "center",
    sortable: true,
  },
  {
    id: "method",
    header: m.finance_fees_col_method(),
    accessorKey: "method_display",
    width: 110,
    align: "center",
    sortable: true,
  },
  {
    id: "amount",
    header: m.common_ft_col_amount(),
    accessorKey: "amount_display",
    width: 130,
    align: "end",
    sortable: true,
  },
  {
    id: "currency",
    header: m.finance_ledger_currency(),
    accessorKey: "currency_display",
    width: 80,
    align: "center",
    sortable: true,
  },
  {
    id: "cash_account",
    header: m.common_ledger_tab_cash_account(),
    accessorKey: "cash_account_display",
    width: 180,
    sortable: true,
  },
  {
    id: "student_name",
    header: m.finance_payments_customer_account(),
    accessorKey: "student_display",
    width: 200,
    sortable: true,
  },
  {
    id: "notes",
    header: m.common_year_notes(),
    accessorKey: "notes_display",
    minWidth: 200,
    sortable: true,
  },
  {
    id: "created_at",
    header: m.common_audit_col_created_at?.() || "تاريخ الإنشاء",
    accessorKey: "created_at_display",
    width: 140,
    align: "center",
    sortable: true,
  },
  {
    id: "updated_at",
    header: m.common_audit_col_updated_at?.() || "آخر تعديل",
    accessorKey: "updated_at_display",
    width: 140,
    align: "center",
    sortable: true,
  },
];
</script>

<div class="h-full flex flex-col overflow-hidden bg-background p-4 font-sans text-start">
  <!-- Main Content Scroll Area (Form + Grid) -->
  <div class="flex-1 overflow-y-auto py-3 space-y-4">
    <!-- Voucher Input Form Card -->
    <Card>
      <!-- Voucher Header & Shared Controls -->
      <CardHeader>
        <div class="flex items-center gap-2">
          <span class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">{m.common_payment_label_receipt()}</span>
          <Badge variant="lime" class="text-[11px] font-mono font-bold px-2 py-0.5">
            {receiptNumber || "قبض جديد"}
          </Badge>
        </div>

        <CardAction>
          <div class="flex flex-wrap items-center gap-3">
            <!-- Treasury select -->
            <div class="flex flex-col gap-1 text-start">
              <Label class="text-[9px] font-bold text-muted-foreground">{m.common_ledger_tab_cash_account()}</Label>
              <SearchableSelect
                value={treasuryId}
                placeholder="اختر الصندوق..."
                searchPlaceholder="ابحث باسم الصندوق أو رمزه..."
                items={treasuryItems}
                getItemValue={(tr) => tr.value}
                getItemLabel={(tr) => tr.label}
                onchange={(val) => { if (val) treasuryId = val; }}
                class="h-8 text-xs w-48 border-border bg-background"
              />
            </div>

            <!-- Method select -->
            <div class="flex flex-col gap-1 text-start">
              <Label class="text-[9px] font-bold text-muted-foreground">{m.finance_fees_col_method()}</Label>
              <SearchableSelect
                value={paymentMethod}
                placeholder="اختر الطريقة..."
                items={paymentMethods}
                getItemValue={(pm) => pm.value}
                getItemLabel={(pm) => pm.label}
                onchange={(val) => { if (val) paymentMethod = val; }}
                class="h-8 text-xs w-32 border-border bg-background"
              />
            </div>

            <!-- Currency select -->
            <div class="flex flex-col gap-1 text-start">
              <Label class="text-[9px] font-bold text-muted-foreground">{m.finance_ledger_currency()}</Label>
              <SearchableSelect
                value={currency}
                placeholder="العملة"
                items={currencies}
                getItemValue={(curr) => curr.code}
                getItemLabel={(curr) => `${curr.code} - ${curr.name}`}
                onchange={(val) => { if (val) currency = val; }}
                class="h-8 text-xs w-32 border-border bg-background"
              />
            </div>

            <!-- Date picker -->
            <div class="flex flex-col gap-1 text-start">
              <Label class="text-[9px] font-bold text-muted-foreground">{m.common_overview_date()}</Label>
              <Input 
                type="date" 
                bind:value={paymentDate} 
                class="h-8 text-xs font-mono border-border w-32" 
              />
            </div>

            <!-- Check Info (if method is Check) -->
            {#if paymentMethod === 'Check'}
              <div in:slide={{ duration: 150 }} class="flex flex-col gap-1 text-start">
                <Label class="text-[9px] font-bold text-muted-foreground">{m.finance_payments_check_info()}</Label>
                <Input 
                  type="text" 
                  bind:value={checkInfo} 
                  class="h-8 text-xs border-border w-40" 
                  placeholder="رقم الشيك والبنك..." 
                />
              </div>
            {/if}
          </div>
        </CardAction>
      </CardHeader>

      <CardContent class="p-4 md:p-6 flex flex-col gap-4">
        <!-- Dynamic Row Grid -->
        <div class="border border-border rounded-lg bg-muted/10 p-4 shadow-xs flex flex-col gap-2">
          <!-- Headers -->
          <div class="hidden md:grid grid-cols-[30px_1.2fr_2.5fr_1fr_40px] gap-4 px-2 text-xs font-bold text-muted-foreground select-none pb-2 border-b border-border">
            <div class="text-center">#</div>
            <div>{m.finance_payments_account_name_notes()}</div>
            <div>{m.finance_payments_due_paid_fees()}</div>
            <div class="text-center">{m.finance_payments_total_paid()}</div>
            <div class="text-center">{m.common_je_col_actions()}</div>
          </div>

          <!-- Input Rows -->
          <div class="flex flex-col gap-3 md:gap-2">
            {#each paymentRows as row, index}
              <div class="grid grid-cols-1 md:grid-cols-[30px_1.2fr_2.5fr_1fr_40px] gap-4 items-start bg-muted/20 md:bg-transparent p-4 md:p-0 rounded-lg border border-border md:border-none">
                <!-- Row Number -->
                <div class="text-start md:text-center font-mono text-xs font-bold text-muted-foreground pt-2">
                  <span class="md:hidden">{m.finance_payments_row()}</span>{index + 1}
                </div>
                
                <!-- Student Selection & Notes -->
                <div class="flex flex-col gap-2">
                  <div class="flex flex-col gap-1">
                    <span class="text-[10px] font-bold text-muted-foreground md:hidden">{m.finance_payments_account_student_name()}</span>
                    <SearchableSelect
                      value={row.studentEnrollmentId}
                      placeholder="-- اختر حساب الطالب --"
                      searchPlaceholder="ابحث باسم الطالب..."
                      items={students}
                      getItemValue={(s) => String(s.id)}
                      getItemLabel={(s) => s.full_name || s.student_number || String(s.id)}
                      getItemSubtitle={(s) => s.student_number ? `#${s.student_number}` : undefined}
                      onchange={(val) => { handleStudentChange(index, val || ""); }}
                      class="w-full text-xs h-8.5 bg-background border-border"
                    />
                  </div>
                  <div class="flex flex-col gap-1">
                    <span class="text-[10px] font-bold text-muted-foreground md:hidden">{m.finance_payments_receipt_notes()}</span>
                    <Input
                      type="text"
                      bind:value={row.notes}
                      class="h-8.5 text-xs w-full bg-background border-border"
                      placeholder="ملاحظات التحصيل لهذا الطالب..."
                    />
                  </div>
                </div>

                <!-- Fees List checkboxes & amount inputs -->
                <div class="flex flex-col gap-1 text-start">
                  <span class="text-[10px] font-bold text-muted-foreground md:hidden">{m.finance_payments_due_paid_fees()}</span>
                  {#if row.isLoadingFees}
                    <div class="space-y-1">
                      <Skeleton class="h-8 w-full rounded-md" />
                      <Skeleton class="h-8 w-full rounded-md" />
                    </div>
                  {:else if row.availableFees.length === 0}
                    <div class="text-xs text-muted-foreground/70 italic py-2">
                      {row.studentEnrollmentId ? "لا توجد رسوم مستحقة غير مدفوعة لهذا الطالب" : "الرجاء اختيار طالب لعرض رسومه"}
                    </div>
                  {:else}
                    <div class="flex flex-col gap-2 bg-background border border-border p-2 rounded-lg max-h-48 overflow-y-auto">
                      {#each row.availableFees as f}
                        <div class="flex items-center justify-between gap-3 text-xs border-b border-border/30 last:border-b-0 pb-1.5 last:pb-0">
                          <!-- Checkbox & Label -->
                          <label class="flex items-center gap-2 cursor-pointer select-none flex-1">
                            <input
                              type="checkbox"
                              checked={row.selectedFees.has(String(f.id))}
                              onchange={() => handleToggleFee(index, f)}
                              class="rounded border-border text-primary focus:ring-primary w-3.5 h-3.5 cursor-pointer"
                            />
                            <div class="flex flex-col text-start">
                              <span class="font-bold text-foreground text-[11px]">{f.notes || m.finance_payments_tuition_fees()}</span>
                              <span class="text-[10px] text-muted-foreground">{m.finance_payments_due()}{formatCurrency(f.balance, currency)}</span>
                            </div>
                          </label>

                          <!-- Amount Input -->
                          <div class="w-32 flex items-center">
                            <Input
                              type="number"
                              disabled={!row.selectedFees.has(String(f.id))}
                              value={row.feeAmounts[String(f.id)] || ""}
                              oninput={(e) => handleFeeAmountChange(index, String(f.id), Number(e.currentTarget.value))}
                              class="h-8 text-xs font-mono text-emerald-500 font-extrabold text-end w-full bg-background border-border"
                              placeholder="0.00"
                            />
                          </div>
                        </div>
                      {/each}
                    </div>
                  {/if}
                </div>

                <!-- Row Total Paid -->
                <div class="flex flex-col gap-1 items-stretch md:items-center">
                  <span class="text-[10px] font-bold text-muted-foreground md:hidden text-center">{m.finance_payments_total_paid()}</span>
                  <div class="bg-background border border-border px-2 py-2 rounded-lg h-8.5 flex items-center justify-center font-mono font-extrabold text-emerald-500 text-xs w-full text-center">
                    {formatCurrency(calculateRowTotal(row), currency)}
                  </div>
                </div>

                <!-- Actions -->
                <div class="flex justify-end md:justify-center pt-1">
                  <Button
                    variant="ghost"
                    size="icon-xs"
                    onclick={() => deletePaymentRow(index)}
                    class="text-destructive hover:bg-destructive/10 cursor-pointer h-8 w-8"
                  >
                    <Trash size={14} />
                  </Button>
                </div>
              </div>
            {/each}
          </div>
        </div>
      </CardContent>

      <!-- Add Row and Action Ribbon in CardFooter -->
      <CardFooter class="justify-between flex-wrap gap-4 py-3">
        <Button
          variant="outline"
          size="sm"
          onclick={addPaymentRow}
          class="font-bold text-xs h-8 px-4 flex items-center gap-1.5 cursor-pointer border-border text-foreground hover:bg-muted"
        >
          <Plus size={14} />
          <span>{m.finance_payments_add_row()}</span>
        </Button>
        
        <div class="flex items-center gap-2.5">
          <Button
            variant="default"
            size="sm"
            onclick={handleAdd}
            class="bg-emerald-600 hover:bg-emerald-700 text-white font-bold text-xs h-8 px-5 flex items-center gap-1.5 cursor-pointer shadow-sm active:scale-[0.98] transition-all"
          >
            <Plus size={14} />
            <span>{m.finance_payments_post_receipts()}</span>
          </Button>
          
          <Button
            variant="destructive"
            size="sm"
            onclick={handleVoid}
            disabled={!selectedPayment}
            class="font-bold text-xs h-8 px-4 flex items-center gap-1.5 cursor-pointer active:scale-[0.98] transition-all"
          >
            <Trash size={14} />
            <span>{m.finance_payments_void_receipt()}</span>
          </Button>

          <Button
            variant="outline"
            size="sm"
            onclick={clearForm}
            class="font-bold text-xs h-8 px-4 flex items-center gap-1.5 cursor-pointer active:scale-[0.98] transition-all border-border text-foreground hover:bg-muted"
          >
            <SignOut size={14} />
            <span>{m.finance_payments_clear_fields()}</span>
          </Button>
        </div>
      </CardFooter>
    </Card>

    <!-- Statistics summary row -->
    {#if !isLoading && payments.length > 0}
      <div class="grid grid-cols-1 sm:grid-cols-2 gap-4" in:fade={{ duration: 150 }}>
        <Card>
          <CardContent class="p-4 flex items-center justify-between">
            <div class="flex flex-col gap-0.5 text-start">
              <span class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">
                إجمالي المقبوضات المحصلة
              </span>
              <span class="text-base font-bold text-emerald-500 font-mono">{formatCurrency(stats.total, "YER")}</span>
            </div>
            <div class="h-8 w-8 rounded-md bg-emerald-500/10 flex items-center justify-center text-emerald-500 shrink-0">
              <Document size={14} />
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardContent class="p-4 flex items-center justify-between">
            <div class="flex flex-col gap-0.5 text-start">
              <span class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">
                عدد سندات المقبوضات
              </span>
              <span class="text-base font-bold text-foreground font-mono">
                {stats.count} سند
              </span>
            </div>
            <div class="h-8 w-8 rounded-md bg-primary/10 flex items-center justify-center text-primary shrink-0">
              <TaskList size={14} />
            </div>
          </CardContent>
        </Card>
      </div>
    {/if}

    <!-- AdjustmentsHorizontal & Sub-Action Bar -->
    <div class="flex flex-wrap items-center justify-between gap-3 py-2 select-none border-b border-border/40">
      <div class="flex flex-wrap items-center gap-3 flex-1">
        <!-- Search input -->
        <div class="relative w-full max-w-xs text-start">
          <Search class="absolute left-2 top-2 h-3.5 w-3.5 text-muted-foreground" />
          <Input
            type="text"
            placeholder="البحث بالاسم أو السند أو الملاحظات..."
            bind:value={searchQuery}
            class="pl-8 h-8 text-[11px]"
          />
        </div>

        <!-- Method filter -->
        <Select type="single" bind:value={methodFilter}>
          <SelectTrigger class="h-8 text-xs bg-background border-border w-40 cursor-pointer text-start">
            {methodFilter === "all" ? m.finance_payments_all_methods() : (methodFilter === "Cash" ? m.finance_pos_cash() : m.finance_payments_check())}
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="all">{m.finance_payments_all_methods()}</SelectItem>
            <SelectItem value="Cash">{m.finance_pos_cash()}</SelectItem>
            <SelectItem value="Check">{m.finance_payments_check()}</SelectItem>
          </SelectContent>
        </Select>

        <span class="text-border hidden md:inline">|</span>

        <!-- AdjustmentsHorizontal Mode Select -->
        <Select type="single" bind:value={filterMode}>
          <SelectTrigger class="h-8 text-xs bg-background border-border w-28 cursor-pointer text-start">
            {filterMode === "day" ? m.finance_payments_today() : (filterMode === "range" ? m.finance_payments_specific_period() : m.finance_je_filter_all())}
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="day">{m.finance_payments_today()}</SelectItem>
            <SelectItem value="range">{m.finance_payments_specific_period()}</SelectItem>
            <SelectItem value="all">{m.finance_je_filter_all()}</SelectItem>
          </SelectContent>
        </Select>

        <!-- Day AdjustmentsHorizontal controls -->
        {#if filterMode === "day"}
          <div class="flex items-center gap-1.5 bg-muted/40 p-0.5 rounded-lg border border-border/50">
            <Button
              variant="ghost"
              size="icon-xs"
              onclick={prevDay}
              class="h-7 w-7 cursor-pointer hover:bg-background"
            >
              <ChevronRight size={14} class="rtl:rotate-180" />
            </Button>
            
            <Input
              type="date"
              bind:value={selectedFilterDate}
              class="h-7 text-xs font-mono bg-transparent border-0 w-28 focus-visible:ring-0 p-0 text-center"
            />
            
            <Button
              variant="ghost"
              size="icon-xs"
              onclick={nextDay}
              class="h-7 w-7 cursor-pointer hover:bg-background"
            >
              <ChevronLeft size={14} class="rtl:rotate-180" />
            </Button>

            <Button
              variant="ghost"
              size="xs"
              onclick={jumpToToday}
              class="h-7 text-[10px] font-bold px-2 cursor-pointer hover:bg-background"
            >
              اليوم
            </Button>
          </div>
        {:else if filterMode === "range"}
          <div class="flex items-center gap-2" in:slide={{ axis: "x", duration: 150 }}>
            <Input type="date" bind:value={startDateFilter} class="h-7 text-xs w-[120px]" />
            <span class="text-xs text-muted-foreground">{m.finance_payments_to()}</span>
            <Input type="date" bind:value={endDateFilter} class="h-7 text-xs w-[120px]" />
          </div>
        {/if}

        <span class="text-border">|</span>

        <!-- Checkbox: User Records -->
        <label class="flex items-center gap-1.5 text-xs font-semibold text-muted-foreground cursor-pointer select-none">
          <input 
            type="checkbox" 
            bind:checked={showUserRecords} 
            class="rounded border-border text-primary focus:ring-primary w-3.5 h-3.5 cursor-pointer" 
          />
          <span>{m.finance_payments_show_user_logs()}</span>
        </label>
      </div>

      <!-- Action buttons -->
      <div class="flex items-center gap-2">
        {#if selectedPaymentIds.size > 0}
          <Button 
            variant="outline" 
            size="sm" 
            onclick={() => { printTemplateId = "22222222-2222-2222-2222-222222222222"; printParameters = { ids: Array.from(selectedPaymentIds) }; printReportOpen = true; }} 
            class="h-8 text-[11px] flex items-center gap-1.5 cursor-pointer hover:bg-muted border-primary/30 text-primary"
          >
            <Printer size={12} />
            <span>{m.common_print_templates()} ({selectedPaymentIds.size})</span>
          </Button>
        {/if}
        <Button 
          variant="outline" 
          size="sm" 
          onclick={handlePrintList} 
          class="h-8 text-[11px] flex items-center gap-1.5 cursor-pointer hover:bg-muted"
        >
          <Printer size={12} />
          <span>{m.finance_journal_print_table()}</span>
        </Button>
        <Button 
          variant="outline" 
          size="sm" 
          onclick={handlePrintVoucher} 
          disabled={!selectedPayment} 
          class="h-8 text-[11px] flex items-center gap-1.5 cursor-pointer hover:bg-muted"
        >
          <Printer size={12} />
          <span>{m.finance_payments_print_receipt()}</span>
        </Button>
        <Button 
          variant="outline" 
          size="sm" 
          onclick={() => { if(selectedPayment) showQrModal = true; else toast.error('الرجاء اختيار سند أولاً'); }} 
          class="h-8 text-[11px] flex items-center gap-1.5 cursor-pointer hover:bg-muted"
        >
          <CheckSquare size={12} />
          <span>QR</span>
        </Button>
      </div>
    </div>

    <!-- OrbisGrid Table container -->
    <div class="h-96 min-h-64 flex flex-col bg-card rounded-xl border border-border/80 overflow-hidden p-2">
      <OrbisGrid
        columns={paymentColumns}
        source={paymentGridSource}
        loading={isLoading}
        clientQuery
        search={searchQuery}
        rtl
        readonly
        emptyTitle="لا توجد سندات قبض مسجلة لهذا البحث أو التاريخ"
        onRowClick={(row) => selectRow(row)}
        onRowDoubleClick={(row) => {
          selectRow(row);
          toast.info(`تم تحميل السند رقم ${row.receipt_number || row.id}`);
        }}
      />
    </div>
  </div>

  <!-- QR Code Modal -->
  {#if showQrModal && selectedPayment}
    <div in:fade={{ duration: 150 }} class="fixed inset-0 bg-black/60 backdrop-blur-sm z-50 flex items-center justify-center p-4">
      <div class="bg-card border border-border rounded-xl w-full max-w-sm p-5 shadow-2xl relative text-center">
        <h3 class="text-sm font-bold mb-3 text-foreground">{m.finance_payments_qr_code()}</h3>
        <div class="bg-white p-3 rounded-lg inline-block mb-3 border border-zinc-200">
          <svg class="w-40 h-40 mx-auto text-black" viewBox="0 0 100 100" fill="currentColor">
            <path d="M0 0h30v30H0V0zm10 10v10h10V10H10zM70 0h30v30H70V0zm10 10v10h10V10H80zM0 70h30v30H0V70zm10 10v10h10V80H10zm35-80h10v10H45V0zm15 15h10v15H60V15zm-15 20h25v10H45V35zm-15 15h10v10H30V50zm15 10h10v15H45V60zm15 15h10v15H60V75zm15-15h25v10H75V60zm-30 20h10v10H45V80zm35 10h20v10H80V90z" />
          </svg>
        </div>
        
        <div class="text-xs text-muted-foreground flex flex-col gap-1 text-start px-2 bg-muted/30 p-2.5 rounded-lg border border-border/40">
          <div><span class="font-bold text-foreground">{m.finance_payments_receipt_no_label()}</span> {receiptNumber}</div>
          <div><span class="font-bold text-foreground">{m.finance_journal_date_label()}</span> {formatDate(paymentDate)}</div>
          <div><span class="font-bold text-foreground">{m.finance_payments_customer_account_label()}</span> {studentName}</div>
          <div><span class="font-bold text-foreground">{m.finance_journal_amount_label()}</span> {formatCurrency(amount || 0, currency || 'YER')}</div>
          <div><span class="font-bold text-foreground">{m.common_docs_detail_desc_label()}</span> {notes || '-'}</div>
        </div>
        
        <Button variant="outline" size="sm" class="mt-4 w-full cursor-pointer hover:bg-muted" onclick={() => showQrModal = false}>
          إغلاق النافذة
        </Button>
      </div>
    </div>
  {/if}

  <PdfReportViewer
    bind:open={printReportOpen}
    templateId={printTemplateId}
    parameters={printParameters}
    reportTitle="طباعة المقبوضات"
  />
</div>
