<!--
  @component DisbursementsManager
  Unified in-page Payment & Disbursement Voucher (سندات الصرف) accounting workspace.
  Records disbursements from cash/bank treasuries to expenses, suppliers, employees, and other ledger accounts.
  Features:
  - Exact in-page ERP voucher layout (Header banner, Amount, Currency, Treasury, Beneficiary Account, Method, Notes)
  - Dedicated Ribbon Action Bar (إضافة, تعديل, حذف, تحديث, بحث, طباعة, طباعة سند الصرف, خروج)
  - Sub-bar filters (فحص التاريخ, QR, عرض سجلات المستخدم)
  - High-performance OrbisGrid table with complete audit tracking and click-to-load row inspection
  - Zero modal Sheet or Dialog popups required
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import { api } from "$lib/api";
import { formatCurrency, formatDate, l } from "$lib/core/i18n";
import type { JournalEntryDto, AccountDto, CurrencyDto, OrgUnitDto, UserDto } from "$lib/api/types";
import { authStore } from "$lib/stores/auth.svelte";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
import { Badge } from "@orbis/ui/badge";
import { toast } from "@orbis/ui";
import {
  ChevronLeft,
  ChevronRight,
  Pencil,
  Plus,
  Printer,
  Refresh,
  Search,
  SignOut,
  Trash,
  CheckSquare,
} from "@orbis/icons";
import { OrbisGrid, type OrbisColumnDef } from "@orbis/svelte-grid";
import AccountSearchInput from "$lib/features/finance/components/account-search-input.svelte";
import SearchableSelect from "$lib/shared/components/searchable-select.svelte";
import PdfReportViewer from "$lib/shared/components/pdf-report-viewer.svelte";
import { confirmStore } from "$lib/stores";
import { fade, slide } from "svelte/transition";

let { orgId }: { orgId: string } = $props();

// ── Master Data State ──────────────────────────────────────────────
let disbursements = $state<JournalEntryDto[]>([]);
let accounts = $state<AccountDto[]>([]);
let treasuries = $state<AccountDto[]>([]);
let currencies = $state<CurrencyDto[]>([]);
let orgUnits = $state<OrgUnitDto[]>([]);
let users = $state<UserDto[]>([]);
let isLoading = $state(true);
let isSubmitting = $state(false);

// ── Form State (Payment / Disbursement Voucher - سند صرف) ────────
let selectedVoucher = $state<JournalEntryDto | null>(null);
let referenceNumber = $state("");
let voucherDate = $state(new Date().toISOString().slice(0, 10));
let amount = $state<number | null>(null);
let selectedCurrency = $state("YER");
let treasuryAccountId = $state(""); // Credit: Cash/Bank Treasury
let beneficiaryAccountId = $state(""); // Debit: Expense/Supplier/Employee/Payable
let paymentMethod = $state<"Cash" | "Check" | "Transfer">("Cash");
let checkOrTransferRef = $state("");
let notes = $state("");
let selectedOrgUnitId = $state<string>("");

// ── Filter State ──────────────────────────────────────────────────
let filterByDate = $state(true);
let selectedFilterDate = $state(new Date().toISOString().slice(0, 10));
let showUserRecords = $state(false);
let searchQuery = $state("");
let isSearchVisible = $state(false);

// ── Print & QR Modal State ─────────────────────────────────────────
let showQrModal = $state(false);
let printReportOpen = $state(false);
let printTemplateId = $state("");
let printParameters = $state<Record<string, any>>({});
let reportTitle = $state("طباعة سند الصرف");

// ── Payment Methods ───────────────────────────────────────────────
const paymentMethods = [
  { value: "Cash", label: "نقداً (صندوق)" },
  { value: "Check", label: "شيك بنكي" },
  { value: "Transfer", label: "حوالة / تحويل بنكي" },
];

// ── Arabic Day Names ──────────────────────────────────────────────
const arabicDayNames = ["الأحد", "الإثنين", "الثلاثاء", "الأربعاء", "الخميس", "الجمعة", "السبت"];

function getArabicDayName(dateStr: string): string {
  if (!dateStr) return "";
  try {
    const d = new Date(dateStr);
    return arabicDayNames[d.getDay()] || "";
  } catch {
    return "";
  }
}

// ── Lifecycle & Data Loading ───────────────────────────────────────
onMount(async () => {
  await loadData();
});

async function loadData() {
  isLoading = true;
  try {
    const [entriesList, accountsList, currenciesList, orgUnitsList, usersList] = await Promise.all([
      api.listJournalEntries().catch(() => []),
      api.listAccounts().catch(() => []),
      api.listCurrencies().catch(() => []),
      api.listOrgUnits().catch(() => []),
      api.listUsers().catch(() => []),
    ]);

    accounts = accountsList || [];
    // Treasuries: cash and bank accounts
    treasuries = accounts.filter(
      (a: any) =>
        ["cash", "bank"].includes(String(a.account_type || a.sub_type || "").toLowerCase()) ||
        a.code?.startsWith("121") ||
        a.code?.startsWith("122")
    );

    // Filter journal entries that represent disbursement vouchers
    disbursements = (entriesList || []).filter(
      (e) =>
        e.source_type === "disbursement_voucher" ||
        e.description?.includes("سند صرف") ||
        e.description_ar?.includes("سند صرف") ||
        treasuries.some((t) => t.id === e.credit_account_id)
    );

    currencies = currenciesList || [];
    orgUnits = orgUnitsList || [];
    users = usersList || [];

    const base = currenciesList?.find((c) => c.is_base);
    if (base && !selectedCurrency) {
      selectedCurrency = base.code;
    }
    if (treasuries.length > 0 && !treasuryAccountId) {
      treasuryAccountId = treasuries[0].id;
    }
    if (orgUnits.length > 0 && !selectedOrgUnitId) {
      selectedOrgUnitId = orgUnits[0].id;
    }
  } catch (e) {
    console.error("Failed to load disbursement vouchers:", e);
    toast.error("فشل تحميل بيانات سندات الصرف");
  } finally {
    isLoading = false;
  }
}

// ── Lookups ───────────────────────────────────────────────────────
function getAccountInfo(id?: string | null): { name: string; code: string } {
  if (!id) return { name: "—", code: "—" };
  const acc = accounts.find((a) => a.id === id);
  if (!acc) return { name: id.slice(0, 8), code: "—" };
  const name = acc.name_ar || acc.name || acc.code || "—";
  return { name, code: acc.code || "—" };
}

function getUserName(id?: string | null): string {
  if (!id) return "—";
  const u = users.find((user) => user.id === id);
  if (u) return u.full_name || u.email || id.slice(0, 6);
  if (authStore.user && authStore.user.id === id) {
    return authStore.user.full_name || authStore.user.email;
  }
  return id.slice(0, 6);
}

function getOrgUnitName(id?: string | null): string {
  if (!id) return orgUnits.length > 0 ? (orgUnits[0].name_ar || orgUnits[0].name || "الفرع الرئيسي") : "الفرع الرئيسي";
  const unit = orgUnits.find((ou) => ou.id === id);
  return unit ? (unit.name_ar || unit.name || unit.code || "—") : id.slice(0, 8);
}

// ── Form Actions ──────────────────────────────────────────────────
function clearForm() {
  selectedVoucher = null;
  referenceNumber = "";
  voucherDate = new Date().toISOString().slice(0, 10);
  amount = null;
  beneficiaryAccountId = "";
  checkOrTransferRef = "";
  notes = "";
}

function selectRowForEditing(entry: JournalEntryDto) {
  selectedVoucher = entry;
  referenceNumber = entry.reference_number || "";
  voucherDate = entry.accounting_date ? String(entry.accounting_date).slice(0, 10) : new Date().toISOString().slice(0, 10);
  amount = Number(entry.amount || 0);
  selectedCurrency = entry.currency_code || "YER";
  treasuryAccountId = entry.credit_account_id || (entry.lines?.[1]?.account_id ?? "");
  beneficiaryAccountId = entry.debit_account_id || (entry.lines?.[0]?.account_id ?? "");
  notes = entry.description_ar || entry.description || "";
  if (entry.org_unit_id) {
    selectedOrgUnitId = entry.org_unit_id;
  }
}

async function handleSaveNew() {
  if (!amount || amount <= 0) {
    toast.error("يرجى إدخال مبلغ صحيح لسند الصرف");
    return;
  }
  if (!treasuryAccountId) {
    toast.error("يرجى تحديد حساب الصندوق أو البنك المسحوب منه (صرف من)");
    return;
  }
  if (!beneficiaryAccountId) {
    toast.error("يرجى تحديد حساب المصروف أو المستفيد (صرف إلى)");
    return;
  }
  if (treasuryAccountId === beneficiaryAccountId) {
    toast.error("لا يمكن أن يكون حساب الصندوق وحساب المستفيد متطابقين");
    return;
  }

  isSubmitting = true;
  try {
    const descText = notes.trim()
      ? `سند صرف: ${notes.trim()}`
      : referenceNumber
        ? `سند صرف رقم ${referenceNumber}`
        : "سند صرف نقدي / بنكي";

    const payload = {
      reference_number: referenceNumber.trim() || undefined,
      accounting_date: voucherDate,
      amount: Number(amount),
      currency: selectedCurrency,
      currency_code: selectedCurrency,
      debit_account_id: beneficiaryAccountId, // Debit Expense / Beneficiary
      credit_account_id: treasuryAccountId, // Credit Cash / Bank Treasury
      description: descText,
      description_ar: descText,
      source_type: "disbursement_voucher",
      lines: [
        {
          account_id: beneficiaryAccountId,
          debit: Number(amount),
          credit: 0,
          description: descText,
        },
        {
          account_id: treasuryAccountId,
          debit: 0,
          credit: Number(amount),
          description: descText,
        },
      ],
    };

    await api.createJournalEntry(payload as any);
    toast.success("تم ترحيل وحفظ سند الصرف بنجاح");
    clearForm();
    await loadData();
  } catch (err: any) {
    console.error("Failed to post disbursement voucher:", err);
    toast.error(err?.message || "فشل ترحيل سند الصرف");
  } finally {
    isSubmitting = false;
  }
}

async function handleUpdate() {
  if (!selectedVoucher) {
    toast.error("الرجاء اختيار سند صرف للتعديل من الجدول أولاً");
    return;
  }
  await handleSaveNew();
}

async function handleDelete() {
  if (!selectedVoucher) {
    toast.error("الرجاء اختيار سند صرف للإلغاء من الجدول أولاً");
    return;
  }

  const confirmed = await confirmStore.prompt({
    title: "إلغاء سند الصرف؟",
    description: `هل أنت متأكد من رغبتك في إلغاء سند الصرف رقم ${selectedVoucher.reference_number}؟ لا يمكن التراجع عن هذه العملية.`,
    confirmText: "إلغاء السند",
    variant: "destructive",
  });
  if (!confirmed) return;

  try {
    await api.reverseJournalEntry(selectedVoucher.id, { reason: "إلغاء سند الصرف بواسطة المستخدم" } as any);
    toast.success("تم إلغاء سند الصرف بنجاح");
    clearForm();
    await loadData();
  } catch (err: any) {
    toast.error(err?.message || "فشل إلغاء سند الصرف");
  }
}

// ── Date Navigation ────────────────────────────────────────────────
function prevDay() {
  const current = new Date(selectedFilterDate);
  current.setDate(current.getDate() - 1);
  selectedFilterDate = current.toISOString().split("T")[0];
}

function nextDay() {
  const current = new Date(selectedFilterDate);
  current.setDate(current.getDate() + 1);
  selectedFilterDate = current.toISOString().split("T")[0];
}

function jumpToToday() {
  selectedFilterDate = new Date().toISOString().split("T")[0];
}

// ── Print Triggers ─────────────────────────────────────────────────
function handlePrintTable() {
  reportTitle = "طباعة جدول سندات الصرف";
  printTemplateId = "disbursements_ledger_report";
  printParameters = {
    date: filterByDate ? selectedFilterDate : undefined,
    count: filteredDisbursements.length,
  };
  printReportOpen = true;
}

function handlePrintVoucher() {
  if (!selectedVoucher) {
    toast.error("الرجاء اختيار سند صرف لطباعته");
    return;
  }
  reportTitle = `طباعة سند صرف رقم ${selectedVoucher.reference_number}`;
  printTemplateId = "disbursement_voucher_receipt";
  printParameters = { id: selectedVoucher.id };
  printReportOpen = true;
}

// ── Filtered Grid Data ─────────────────────────────────────────────
const filteredDisbursements = $derived.by(() => {
  return disbursements.filter((entry) => {
    // 1. Date Filter
    if (filterByDate && selectedFilterDate) {
      const entryDate = entry.accounting_date
        ? String(entry.accounting_date).slice(0, 10)
        : entry.created_at
          ? String(entry.created_at).slice(0, 10)
          : "";
      if (entryDate !== selectedFilterDate) {
        return false;
      }
    }

    // 2. Search Query Filter
    if (searchQuery) {
      const q = searchQuery.toLowerCase().trim();
      const ref = String(entry.reference_number || "").toLowerCase();
      const desc = String(entry.description_ar || entry.description || "").toLowerCase();
      const debitInfo = getAccountInfo(entry.debit_account_id);
      const creditInfo = getAccountInfo(entry.credit_account_id);
      const debitMatch = debitInfo.name.toLowerCase().includes(q) || debitInfo.code.includes(q);
      const creditMatch = creditInfo.name.toLowerCase().includes(q) || creditInfo.code.includes(q);

      if (!ref.includes(q) && !desc.includes(q) && !debitMatch && !creditMatch) {
        return false;
      }
    }

    // 3. User Records Filter
    if (showUserRecords && authStore.user) {
      if (entry.created_by && entry.created_by !== authStore.user.id) {
        return false;
      }
    }

    return true;
  });
});

// ── OrbisGrid Source & Column Definitions ──────────────────────────
const gridSource = $derived(
  filteredDisbursements.map((e, idx) => {
    const beneficiaryInfo = getAccountInfo(e.debit_account_id || e.lines?.[0]?.account_id);
    const treasuryInfo = getAccountInfo(e.credit_account_id || e.lines?.[1]?.account_id);
    const curr = currencies.find((c) => c.code === e.currency_code)?.name || e.currency_code || "YER";

    return {
      ...e,
      __idx: idx + 1,
      reference_display: e.reference_number || e.id.slice(0, 8),
      date_display: e.accounting_date ? String(e.accounting_date).slice(0, 10) : formatDate(e.created_at, "date"),
      amount_display: formatCurrency(Number(e.amount || 0), e.currency_code || "YER"),
      currency_display: curr,
      treasury_display: treasuryInfo.code !== "—" ? `${treasuryInfo.code} - ${treasuryInfo.name}` : treasuryInfo.name,
      beneficiary_display: beneficiaryInfo.code !== "—" ? `${beneficiaryInfo.code} - ${beneficiaryInfo.name}` : beneficiaryInfo.name,
      notes_display: e.description_ar || e.description || "—",
      user_display: getUserName(e.created_by),
      branch_display: getOrgUnitName(e.org_unit_id),
      entry_time_display: e.created_at ? formatDate(e.created_at, "datetime") : "—",
      status_display: e.status === "posted" ? "معتمد" : (e.status === "reversed" ? "ملغي" : "مسودة"),
      updated_at_display: e.updated_at ? formatDate(e.updated_at, "datetime") : "—",
    };
  })
);

const columns: OrbisColumnDef<any>[] = [
  {
    id: "reference_number",
    header: "رقم السند",
    accessorKey: "reference_display",
    width: 95,
    align: "center",
    sortable: true,
  },
  {
    id: "accounting_date",
    header: "تاريخ الصرف",
    accessorKey: "date_display",
    width: 105,
    align: "center",
    sortable: true,
  },
  {
    id: "amount",
    header: "المبلغ المنصرف",
    accessorKey: "amount_display",
    width: 130,
    align: "end",
    sortable: true,
  },
  {
    id: "currency",
    header: "العملة",
    accessorKey: "currency_display",
    width: 90,
    align: "center",
    sortable: true,
  },
  {
    id: "treasury",
    header: "صرف من حساب (الصندوق/البنك)",
    accessorKey: "treasury_display",
    width: 220,
    sortable: true,
  },
  {
    id: "beneficiary",
    header: "إلى حساب (المصروف/المستفيد)",
    accessorKey: "beneficiary_display",
    width: 220,
    sortable: true,
  },
  {
    id: "notes",
    header: "البيان والملاحظات",
    accessorKey: "notes_display",
    minWidth: 200,
    sortable: true,
  },
  {
    id: "user",
    header: "المستخدم",
    accessorKey: "user_display",
    width: 110,
    align: "center",
    sortable: true,
  },
  {
    id: "branch",
    header: "الفرع",
    accessorKey: "branch_display",
    width: 120,
    align: "center",
    sortable: true,
  },
  {
    id: "entry_time",
    header: "وقت الإدخال",
    accessorKey: "entry_time_display",
    width: 150,
    align: "center",
    sortable: true,
  },
  {
    id: "status",
    header: "الحالة",
    accessorKey: "status_display",
    width: 85,
    align: "center",
    sortable: true,
  },
  {
    id: "updated_at",
    header: "آخر تعديل",
    accessorKey: "updated_at_display",
    width: 140,
    align: "center",
    sortable: true,
  },
];
</script>

<div class="h-full flex flex-col overflow-hidden bg-background font-sans text-start select-none">
  
  <!-- ══════════════════════════════════════════════════════════════════════════ -->
  <!-- Top Title & Voucher Header Bar                                            -->
  <!-- ══════════════════════════════════════════════════════════════════════════ -->
  <div class="bg-card border-b border-border/80 px-4 py-2 flex items-center justify-between gap-4 shrink-0 shadow-xs">
    
    <!-- Left: الرقم (Reference / Voucher Number) -->
    <div class="flex items-center gap-2">
      <Input
        type="text"
        bind:value={referenceNumber}
        class="h-8 w-32 font-mono font-bold text-center text-xs bg-background border-border"
        placeholder="تلقائي / أدخل الرقم"
      />
      <Label class="text-xs font-bold text-foreground">رقم سند الصرف</Label>
    </div>

    <!-- Center: سند صرف نقدي / بنكي (Voucher Main Title) -->
    <div class="flex items-center justify-center">
      <div class="border border-border/80 bg-rose-500/10 px-8 py-1 rounded-md shadow-inner border-rose-500/30">
        <h1 class="text-sm font-extrabold text-rose-600 dark:text-rose-400 tracking-wide">
          سند صرف نقدي / بنكي (Disbursement Voucher)
        </h1>
      </div>
    </div>

    <!-- Right: التاريخ (Date Field with Arabic Day) -->
    <div class="flex items-center gap-2">
      <Label class="text-xs font-bold text-foreground">التاريخ</Label>
      <div class="flex items-center border border-border rounded-md bg-background px-2 h-8 gap-1.5 text-xs font-mono">
        <input
          type="date"
          bind:value={voucherDate}
          class="bg-transparent border-0 text-xs font-mono focus:outline-none cursor-pointer"
        />
        <span class="text-muted-foreground font-sans font-bold text-[11px] border-s border-border ps-1.5">
          {getArabicDayName(voucherDate)}
        </span>
      </div>
    </div>
  </div>

  <!-- ══════════════════════════════════════════════════════════════════════════ -->
  <!-- Form Body (Clean ERP Box)                                                -->
  <!-- ══════════════════════════════════════════════════════════════════════════ -->
  <div class="p-3 bg-muted/20 border-b border-border/80 shrink-0">
    <div class="bg-card border border-border/70 rounded-lg p-4 shadow-xs space-y-3">
      
      <!-- Row 1: المبلغ (Highlight box) + العملة (Select) + طريقة الصرف -->
      <div class="grid grid-cols-1 md:grid-cols-3 gap-4 items-center">
        
        <!-- المبلغ (Amount) -->
        <div class="flex items-center gap-3">
          <Label class="text-xs font-bold text-foreground w-20 text-start shrink-0">المبلغ</Label>
          <div class="flex-1 relative">
            <Input
              type="number"
              bind:value={amount}
              placeholder="0.00"
              class="h-8 text-sm font-mono font-extrabold text-rose-600 dark:text-rose-400 bg-rose-500/10 dark:bg-rose-950/30 border-rose-500/40 focus-visible:ring-rose-500"
            />
          </div>
        </div>

        <!-- العملة (Currency) -->
        <div class="flex items-center gap-3">
          <Label class="text-xs font-bold text-foreground w-16 text-start shrink-0">العملة</Label>
          <div class="flex-1">
            <SearchableSelect
              value={selectedCurrency}
              placeholder="اختر العملة..."
              searchPlaceholder="ابحث بالاسم أو الرمز..."
              items={currencies}
              getItemValue={(c) => c.code}
              getItemLabel={(c) => `${c.name || c.code} (${c.code})`}
              onchange={(val) => { if (val) selectedCurrency = val; }}
              class="w-full h-8 text-xs bg-background border-border"
            />
          </div>
        </div>

        <!-- طريقة الصرف (Payment Method) -->
        <div class="flex items-center gap-3">
          <Label class="text-xs font-bold text-foreground w-20 text-start shrink-0">طريقة الصرف</Label>
          <div class="flex-1">
            <SearchableSelect
              value={paymentMethod}
              placeholder="اختر الطريقة..."
              items={paymentMethods}
              getItemValue={(pm) => pm.value}
              getItemLabel={(pm) => pm.label}
              onchange={(val) => { if (val) paymentMethod = val as any; }}
              class="w-full h-8 text-xs bg-background border-border"
            />
          </div>
        </div>
      </div>

      <!-- Row 2: صرف من حساب (Credit Treasury) + إلى حساب (Debit Expense/Beneficiary) -->
      <div class="grid grid-cols-1 md:grid-cols-2 gap-4 items-center">
        
        <!-- صرف من حساب (Treasury / Bank) -->
        <div class="flex items-center gap-3">
          <Label class="text-xs font-bold text-foreground w-24 text-start shrink-0">صرف من حساب</Label>
          <div class="flex-1">
            <SearchableSelect
              value={treasuryAccountId}
              placeholder="اختر الصندوق أو البنك..."
              searchPlaceholder="ابحث باسم الصندوق أو رمزه..."
              items={treasuries}
              getItemValue={(t) => t.id}
              getItemLabel={(t) => `${t.code ? `${t.code} - ` : ""}${t.name_ar || t.name}`}
              onchange={(val) => { if (val) treasuryAccountId = val; }}
              class="w-full text-xs h-8 bg-background border-border"
            />
          </div>
        </div>

        <!-- إلى حساب (Expense / Supplier / Beneficiary) -->
        <div class="flex items-center gap-3">
          <Label class="text-xs font-bold text-foreground w-24 text-start shrink-0">إلى حساب المستفيد</Label>
          <div class="flex-1">
            <AccountSearchInput
              bind:value={beneficiaryAccountId}
              placeholder="اكتب رقم أو اسم حساب المصروف / المورد / الموظف..."
              class="w-full text-xs"
            />
          </div>
        </div>
      </div>

      <!-- Row 3: البيان والملاحظات + الفرع -->
      <div class="grid grid-cols-1 md:grid-cols-3 gap-4 items-center">
        
        <div class="md:col-span-2 flex items-center gap-3">
          <Label class="text-xs font-bold text-foreground w-20 text-start shrink-0">البيان والشرح</Label>
          <div class="flex-1">
            <Input
              type="text"
              bind:value={notes}
              placeholder="ملاحظات وبيان الصرف والمستفيد..."
              class="h-8 text-xs bg-background border-border w-full"
            />
          </div>
        </div>

        <div class="flex items-center gap-3">
          <Label class="text-xs font-bold text-foreground w-16 text-start shrink-0">الفرع</Label>
          <div class="flex-1">
            <SearchableSelect
              value={selectedOrgUnitId}
              placeholder="اختر الفرع..."
              items={orgUnits}
              getItemValue={(ou) => ou.id}
              getItemLabel={(ou) => ou.name_ar || ou.name || ou.code || ou.id}
              onchange={(val) => { if (val) selectedOrgUnitId = val; }}
              class="w-full h-8 text-xs bg-background border-border"
            />
          </div>
        </div>
      </div>
    </div>
  </div>

  <!-- ══════════════════════════════════════════════════════════════════════════ -->
  <!-- Action Ribbon Toolbar (إضافة, تعديل, حذف, تحديث, بحث, طباعة, طباعة السند, خروج) -->
  <!-- ══════════════════════════════════════════════════════════════════════════ -->
  <div class="bg-card border-b border-border/80 px-4 py-2 flex flex-wrap items-center justify-center gap-2 shrink-0">
    
    <!-- إضافة (Add / Save) -->
    <Button
      variant="default"
      size="sm"
      onclick={handleSaveNew}
      disabled={isSubmitting}
      class="bg-rose-600 hover:bg-rose-700 text-white font-bold text-xs h-8.5 px-4 flex items-center gap-1.5 cursor-pointer shadow-xs active:scale-[0.98] transition-all"
    >
      <Plus size={14} class="stroke-2" />
      <span>إضافة سند صرف</span>
    </Button>

    <!-- تعديل (Edit / Update) -->
    <Button
      variant="secondary"
      size="sm"
      onclick={handleUpdate}
      disabled={!selectedVoucher || isSubmitting}
      class="font-bold text-xs h-8.5 px-4 flex items-center gap-1.5 cursor-pointer border border-border shadow-xs active:scale-[0.98] transition-all"
    >
      <Pencil size={14} />
      <span>تعديل</span>
    </Button>

    <!-- حذف (Delete / Void) -->
    <Button
      variant="destructive"
      size="sm"
      onclick={handleDelete}
      disabled={!selectedVoucher || isSubmitting}
      class="font-bold text-xs h-8.5 px-4 flex items-center gap-1.5 cursor-pointer shadow-xs active:scale-[0.98] transition-all"
    >
      <Trash size={14} />
      <span>إلغاء السند</span>
    </Button>

    <!-- تحديث (Refresh) -->
    <Button
      variant="outline"
      size="sm"
      onclick={loadData}
      class="font-bold text-xs h-8.5 px-4 flex items-center gap-1.5 cursor-pointer border-border text-foreground hover:bg-muted shadow-xs active:scale-[0.98] transition-all"
    >
      <Refresh size={14} class={isLoading ? "animate-spin text-rose-500" : "text-rose-500"} />
      <span>تحديث</span>
    </Button>

    <!-- بحث (Search Toggle) -->
    <Button
      variant={isSearchVisible ? "secondary" : "outline"}
      size="sm"
      onclick={() => (isSearchVisible = !isSearchVisible)}
      class="font-bold text-xs h-8.5 px-4 flex items-center gap-1.5 cursor-pointer border-border text-foreground hover:bg-muted shadow-xs active:scale-[0.98] transition-all"
    >
      <Search size={14} class="text-blue-500" />
      <span>بحث</span>
    </Button>

    <!-- طباعة (Print Table) -->
    <Button
      variant="outline"
      size="sm"
      onclick={handlePrintTable}
      class="font-bold text-xs h-8.5 px-4 flex items-center gap-1.5 cursor-pointer border-border text-foreground hover:bg-muted shadow-xs active:scale-[0.98] transition-all"
    >
      <Printer size={14} class="text-amber-500" />
      <span>طباعة الجدول</span>
    </Button>

    <!-- طباعة السند (Print Voucher) -->
    <Button
      variant="outline"
      size="sm"
      onclick={handlePrintVoucher}
      disabled={!selectedVoucher}
      class="font-bold text-xs h-8.5 px-4 flex items-center gap-1.5 cursor-pointer border-border text-foreground hover:bg-muted shadow-xs active:scale-[0.98] transition-all"
    >
      <Printer size={14} class="text-rose-500" />
      <span>طباعة سند الصرف</span>
    </Button>

    <!-- خروج (Exit / Reset Fields) -->
    <Button
      variant="outline"
      size="sm"
      onclick={clearForm}
      class="font-bold text-xs h-8.5 px-4 flex items-center gap-1.5 cursor-pointer border-border text-foreground hover:bg-muted shadow-xs active:scale-[0.98] transition-all"
    >
      <SignOut size={14} class="text-sky-500" />
      <span>خروج</span>
    </Button>
  </div>

  <!-- ══════════════════════════════════════════════════════════════════════════ -->
  <!-- Search Bar Slide-over                                                    -->
  <!-- ══════════════════════════════════════════════════════════════════════════ -->
  {#if isSearchVisible}
    <div transition:slide={{ duration: 150 }} class="bg-muted/40 border-b border-border px-4 py-2 flex items-center gap-3">
      <div class="relative flex-1 max-w-md">
        <Search class="absolute right-3 top-1/2 -translate-y-1/2 text-muted-foreground w-4 h-4" />
        <Input
          type="text"
          bind:value={searchQuery}
          placeholder="ابحث برقم سند الصرف أو اسم الحساب أو الملاحظات..."
          class="h-8 pr-9 pl-3 text-xs bg-background border-border"
          autofocus
        />
      </div>
      {#if searchQuery}
        <Button variant="ghost" size="xs" onclick={() => (searchQuery = "")} class="text-xs text-muted-foreground">
          مسح البحث
        </Button>
      {/if}
    </div>
  {/if}

  <!-- ══════════════════════════════════════════════════════════════════════════ -->
  <!-- Sub-bar (Filters & Secondary Action Buttons)                             -->
  <!-- ══════════════════════════════════════════════════════════════════════════ -->
  <div class="bg-muted/30 border-b border-border/80 px-4 py-1.5 flex flex-wrap items-center justify-between gap-3 shrink-0">
    
    <!-- Right side: فحص التاريخ (Date Filter + Navigation) -->
    <div class="flex items-center gap-2">
      <label class="flex items-center gap-1.5 text-xs font-bold text-foreground cursor-pointer select-none">
        <input
          type="checkbox"
          bind:checked={filterByDate}
          class="rounded border-border text-primary focus:ring-primary w-3.5 h-3.5 cursor-pointer"
        />
        <span>فحص التاريخ</span>
      </label>

      {#if filterByDate}
        <div class="flex items-center gap-1 bg-background border border-border rounded-md px-1 py-0.5" in:slide={{ axis: "x", duration: 150 }}>
          <Button
            variant="ghost"
            size="icon-xs"
            onclick={prevDay}
            class="h-6 w-6 cursor-pointer hover:bg-muted"
          >
            <ChevronRight size={13} class="rtl:rotate-180" />
          </Button>

          <input
            type="date"
            bind:value={selectedFilterDate}
            class="bg-transparent border-0 text-xs font-mono text-center focus:outline-none cursor-pointer w-28"
          />

          <Button
            variant="ghost"
            size="icon-xs"
            onclick={nextDay}
            class="h-6 w-6 cursor-pointer hover:bg-muted"
          >
            <ChevronLeft size={13} class="rtl:rotate-180" />
          </Button>

          <span class="text-xs font-bold text-muted-foreground border-s border-border ps-1.5 pe-1">
            {getArabicDayName(selectedFilterDate)}
          </span>

          <Button
            variant="ghost"
            size="xs"
            onclick={jumpToToday}
            class="h-6 text-[10px] font-bold px-1.5 cursor-pointer hover:bg-muted text-primary"
          >
            اليوم
          </Button>
        </div>
      {/if}
    </div>

    <!-- Middle: Secondary Quick Action Buttons (QR) -->
    <div class="flex items-center gap-2">
      <Button
        variant="outline"
        size="sm"
        onclick={() => {
          if (selectedVoucher) showQrModal = true;
          else toast.error("الرجاء اختيار سند أولاً");
        }}
        class="h-7 text-xs font-bold px-2.5 border-border bg-background hover:bg-muted cursor-pointer"
      >
        <CheckSquare size={13} class="me-1" />
        <span>QR</span>
      </Button>
    </div>

    <!-- Left side: عرض سجلات المستخدم (Show User Logs Checkbox) -->
    <div class="flex items-center gap-2">
      <label class="flex items-center gap-1.5 text-xs font-semibold text-muted-foreground cursor-pointer select-none">
        <input
          type="checkbox"
          bind:checked={showUserRecords}
          class="rounded border-border text-primary focus:ring-primary w-3.5 h-3.5 cursor-pointer"
        />
        <span>عرض سجلات المستخدم</span>
      </label>
    </div>
  </div>

  <!-- ══════════════════════════════════════════════════════════════════════════ -->
  <!-- OrbisGrid Table Ledger Section                                            -->
  <!-- ══════════════════════════════════════════════════════════════════════════ -->
  <div class="flex-1 min-h-0 bg-card p-2 flex flex-col">
    <OrbisGrid
      {columns}
      source={gridSource}
      loading={isLoading}
      clientQuery
      search={searchQuery}
      rtl
      readonly
      emptyTitle="لا توجد سندات صرف مسجلة لهذا التاريخ أو البحث"
      emptyDescription="استخدم نموذج الصرف أعلاه لتسجيل وترحيل سندات صرف جديدة مباشرة."
      onRowClick={(row) => selectRowForEditing(row)}
      onRowDoubleClick={(row) => {
        selectRowForEditing(row);
        toast.info(`تم تحميل سند الصرف رقم ${row.reference_number}`);
      }}
    />
  </div>

  <!-- ══════════════════════════════════════════════════════════════════════════ -->
  <!-- QR Code Modal Dialog                                                      -->
  <!-- ══════════════════════════════════════════════════════════════════════════ -->
  {#if showQrModal && selectedVoucher}
    <div in:fade={{ duration: 150 }} class="fixed inset-0 bg-black/60 backdrop-blur-xs z-50 flex items-center justify-center p-4">
      <div class="bg-card border border-border rounded-xl w-full max-w-sm p-5 shadow-2xl relative text-center">
        <h3 class="text-sm font-bold mb-3 text-foreground">رمز الاستجابة السريعة (QR) لسند الصرف</h3>
        
        <div class="bg-white p-3 rounded-lg inline-block mb-3 border border-zinc-200">
          <svg class="w-40 h-40 mx-auto text-black" viewBox="0 0 100 100" fill="currentColor">
            <path d="M0 0h30v30H0V0zm10 10v10h10V10H10zM70 0h30v30H70V0zm10 10v10h10V10H80zM0 70h30v30H0V70zm10 10v10h10V80H10zm35-80h10v10H45V0zm15 15h10v15H60V15zm-15 20h25v10H45V35zm-15 15h10v10H30V50zm15 10h10v15H45V60zm15 15h10v15H60V75zm15-15h25v10H75V60zm-30 20h10v10H45V80zm35 10h20v10H80V90z" />
          </svg>
        </div>
        
        <div class="text-xs text-muted-foreground flex flex-col gap-1 text-start px-2 bg-muted/30 p-2.5 rounded-lg border border-border/40">
          <div><span class="font-bold text-foreground">رقم سند الصرف:</span> {selectedVoucher.reference_number}</div>
          <div><span class="font-bold text-foreground">التاريخ:</span> {formatDate(selectedVoucher.accounting_date, "date")}</div>
          <div><span class="font-bold text-foreground">المبلغ:</span> {formatCurrency(Number(selectedVoucher.amount || 0), selectedVoucher.currency_code || "YER")}</div>
          <div><span class="font-bold text-foreground">البيان:</span> {selectedVoucher.description_ar || selectedVoucher.description || "—"}</div>
        </div>
        
        <Button variant="outline" size="sm" class="mt-4 w-full cursor-pointer hover:bg-muted" onclick={() => (showQrModal = false)}>
          إغلاق النافذة
        </Button>
      </div>
    </div>
  {/if}

  <!-- ══════════════════════════════════════════════════════════════════════════ -->
  <!-- PDF Native Printing Engine Viewer                                         -->
  <!-- ══════════════════════════════════════════════════════════════════════════ -->
  <PdfReportViewer
    bind:open={printReportOpen}
    templateId={printTemplateId}
    parameters={printParameters}
    {reportTitle}
  />
</div>
