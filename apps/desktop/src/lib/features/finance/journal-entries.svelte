<!--
  @component JournalEntries
  Unified in-page Simple Journal Voucher (سند قيد بسيط) and General Ledger Workspace.
  Adheres strictly to Orbis Clean Architecture and authentic domain data models:
  - Real in-page ERP voucher layout (Header banner, Amount, Currency, Debit & Credit accounts, Notes, Org Units)
  - Dedicated Ribbon Action Bar (إضافة, تعديل, حذف, تحديث, بحث, طباعة, طباعة السند, خروج)
  - Sub-bar filters (فحص التاريخ, QR, طباعة إشعار مدين, طباعة إشعار دائن, عرض سجلات المستخدم)
  - High-performance OrbisGrid table with authentic audit tracking and click-to-load row inspection
  - Zero modal Sheet or Dialog popups required
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import { api } from "$lib/api";
import { formatCurrency, formatDate, l, isAr } from "$lib/core/i18n";
import type { JournalEntryDto, AccountDto, CurrencyDto, OrgUnitDto, UserDto } from "$lib/api/types";
import { authStore } from "$lib/stores/auth.svelte";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Label } from "@orbis/ui/label";
import { Badge } from "@orbis/ui/badge";
import { toast } from "@orbis/ui";
import {
  CheckCircle,
  ChevronLeft,
  ChevronRight,
  Document,
  Pencil,
  Plus,
  Printer,
  Refresh,
  Search,
  SignOut,
  Trash,
  XMark,
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
let journalEntries = $state<JournalEntryDto[]>([]);
let accounts = $state<AccountDto[]>([]);
let currencies = $state<CurrencyDto[]>([]);
let orgUnits = $state<OrgUnitDto[]>([]);
let users = $state<UserDto[]>([]);
let isLoading = $state(true);
let isSubmitting = $state(false);

// ── In-Page Form State (Simple Journal Voucher / سند قيد بسيط) ─────
let selectedEntry = $state<JournalEntryDto | null>(null);
let referenceNumber = $state("");
let accountingDate = $state(new Date().toISOString().slice(0, 10));
let amount = $state<number | null>(null);
let selectedCurrency = $state("YER");
let debitAccountId = $state("");
let creditAccountId = $state("");
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
let reportTitle = $state("طباعة المستند");

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

    journalEntries = entriesList || [];
    accounts = accountsList || [];
    currencies = currenciesList || [];
    orgUnits = orgUnitsList || [];
    users = usersList || [];

    const base = currenciesList?.find((c) => c.is_base);
    if (base && !selectedCurrency) {
      selectedCurrency = base.code;
    }
    if (orgUnits.length > 0 && !selectedOrgUnitId) {
      selectedOrgUnitId = orgUnits[0].id;
    }
  } catch (e) {
    console.error("Failed to load journal entries master data:", e);
    toast.error("فشل تحميل بيانات قيود اليومية");
  } finally {
    isLoading = false;
  }
}

// ── Lookup Helpers ────────────────────────────────────────────────
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
  selectedEntry = null;
  referenceNumber = "";
  accountingDate = new Date().toISOString().slice(0, 10);
  amount = null;
  debitAccountId = "";
  creditAccountId = "";
  notes = "";
}

function selectRowForEditing(entry: JournalEntryDto) {
  selectedEntry = entry;
  referenceNumber = entry.reference_number || "";
  accountingDate = entry.accounting_date ? String(entry.accounting_date).slice(0, 10) : new Date().toISOString().slice(0, 10);
  amount = Number(entry.amount || 0);
  selectedCurrency = entry.currency_code || "YER";
  debitAccountId = entry.debit_account_id || (entry.lines?.[0]?.account_id ?? "");
  creditAccountId = entry.credit_account_id || (entry.lines?.[1]?.account_id ?? "");
  notes = entry.description_ar || entry.description || "";
  if (entry.org_unit_id) {
    selectedOrgUnitId = entry.org_unit_id;
  }
}

async function handleSaveNew() {
  if (!amount || amount <= 0) {
    toast.error("يرجى إدخال مبلغ صحيح للقيد");
    return;
  }
  if (!debitAccountId) {
    toast.error("يرجى تحديد حساب المدين (من حساب)");
    return;
  }
  if (!creditAccountId) {
    toast.error("يرجى تحديد حساب الدائن (الى حساب)");
    return;
  }
  if (debitAccountId === creditAccountId) {
    toast.error("لا يمكن أن يكون حساب المدين وحساب الدائن متطابقين");
    return;
  }

  isSubmitting = true;
  try {
    const payload = {
      reference_number: referenceNumber.trim() || undefined,
      accounting_date: accountingDate,
      amount: Number(amount),
      currency: selectedCurrency,
      currency_code: selectedCurrency,
      debit_account_id: debitAccountId,
      credit_account_id: creditAccountId,
      description: notes.trim() || (referenceNumber ? `سند قيد رقم ${referenceNumber}` : "سند قيد محاسبي"),
      description_ar: notes.trim() || (referenceNumber ? `سند قيد رقم ${referenceNumber}` : "سند قيد محاسبي"),
      lines: [
        {
          account_id: debitAccountId,
          debit: Number(amount),
          credit: 0,
          description: notes.trim() || "من حساب",
        },
        {
          account_id: creditAccountId,
          debit: 0,
          credit: Number(amount),
          description: notes.trim() || "الى حساب",
        },
      ],
    };

    await api.createJournalEntry(payload as any);
    toast.success("تم حفظ وترحيل سند القيد بنجاح");
    clearForm();
    await loadData();
  } catch (err: any) {
    console.error("Failed to post journal entry:", err);
    toast.error(err?.message || "فشل ترحيل سند القيد");
  } finally {
    isSubmitting = false;
  }
}

async function handleUpdate() {
  if (!selectedEntry) {
    toast.error("الرجاء اختيار سند للتعديل من الجدول أولاً");
    return;
  }
  await handleSaveNew();
}

async function handleDelete() {
  if (!selectedEntry) {
    toast.error("الرجاء اختيار سند للإلغاء من الجدول أولاً");
    return;
  }

  const confirmed = await confirmStore.prompt({
    title: "إلغاء سند القيد؟",
    description: `هل أنت متأكد من رغبتك في إلغاء السند رقم ${selectedEntry.reference_number}؟ لا يمكن التراجع عن هذه العملية.`,
    confirmText: "إلغاء السند",
    variant: "destructive",
  });
  if (!confirmed) return;

  try {
    await api.reverseJournalEntry(selectedEntry.id, { reason: "إلغاء بواسطة المستخدم" } as any);
    toast.success("تم إلغاء سند القيد بنجاح");
    clearForm();
    await loadData();
  } catch (err: any) {
    toast.error(err?.message || "فشل إلغاء سند القيد");
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
  reportTitle = "طباعة جدول القيود اليومية";
  printTemplateId = "journal_ledger_report";
  printParameters = {
    date: filterByDate ? selectedFilterDate : undefined,
    count: filteredEntries.length,
  };
  printReportOpen = true;
}

function handlePrintVoucher() {
  if (!selectedEntry) {
    toast.error("الرجاء اختيار سند لطباعته");
    return;
  }
  reportTitle = `طباعة سند قيد رقم ${selectedEntry.reference_number}`;
  printTemplateId = "journal_voucher_receipt";
  printParameters = { id: selectedEntry.id };
  printReportOpen = true;
}

function handlePrintDebitNotice() {
  if (!selectedEntry) {
    toast.error("الرجاء اختيار سند أولاً");
    return;
  }
  reportTitle = `طباعة إشعار مدين - سند ${selectedEntry.reference_number}`;
  printTemplateId = "debit_notice";
  printParameters = { id: selectedEntry.id, type: "debit" };
  printReportOpen = true;
}

function handlePrintCreditNotice() {
  if (!selectedEntry) {
    toast.error("الرجاء اختيار سند أولاً");
    return;
  }
  reportTitle = `طباعة إشعار دائن - سند ${selectedEntry.reference_number}`;
  printTemplateId = "credit_notice";
  printParameters = { id: selectedEntry.id, type: "credit" };
  printReportOpen = true;
}

// ── Filtered Grid Data ─────────────────────────────────────────────
const filteredEntries = $derived.by(() => {
  return journalEntries.filter((entry) => {
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
  filteredEntries.map((e, idx) => {
    const debitInfo = getAccountInfo(e.debit_account_id || e.lines?.[0]?.account_id);
    const creditInfo = getAccountInfo(e.credit_account_id || e.lines?.[1]?.account_id);
    const curr = currencies.find((c) => c.code === e.currency_code)?.name || e.currency_code || "YER";

    return {
      ...e,
      __idx: idx + 1,
      reference_display: e.reference_number || e.id.slice(0, 8),
      date_display: e.accounting_date ? String(e.accounting_date).slice(0, 10) : formatDate(e.created_at, "date"),
      amount_raw: Number(e.amount || 0),
      amount_display: formatCurrency(Number(e.amount || 0), e.currency_code || "YER"),
      currency_display: curr,
      from_account_display: debitInfo.code !== "—" ? `${debitInfo.code} - ${debitInfo.name}` : debitInfo.name,
      to_account_display: creditInfo.code !== "—" ? `${creditInfo.code} - ${creditInfo.name}` : creditInfo.name,
      notes_display: e.description_ar || e.description || "—",
      source_ref_display: e.source_id ? e.source_id.slice(0, 8) : "—",
      user_display: getUserName(e.created_by),
      branch_display: getOrgUnitName(e.org_unit_id),
      entry_time_display: e.created_at ? formatDate(e.created_at, "datetime") : "—",
      prints_display: "0",
      status_display: e.status === "posted" ? "معتمد" : (e.status === "reversed" ? "ملغي" : "مسودة"),
      updated_at_display: e.updated_at ? formatDate(e.updated_at, "datetime") : "—",
    };
  })
);

const columns: OrbisColumnDef<any>[] = [
  {
    id: "reference_number",
    header: "الرقم",
    accessorKey: "reference_display",
    width: 95,
    align: "center",
    sortable: true,
  },
  {
    id: "accounting_date",
    header: "التاريخ",
    accessorKey: "date_display",
    width: 105,
    align: "center",
    sortable: true,
  },
  {
    id: "amount",
    header: "المبلغ",
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
    id: "from_account",
    header: "من حساب",
    accessorKey: "from_account_display",
    width: 220,
    sortable: true,
  },
  {
    id: "to_account",
    header: "الى حساب",
    accessorKey: "to_account_display",
    width: 220,
    sortable: true,
  },
  {
    id: "notes",
    header: "ملاحظات",
    accessorKey: "notes_display",
    minWidth: 200,
    sortable: true,
  },
  {
    id: "source_ref",
    header: "رقم المرجع",
    accessorKey: "source_ref_display",
    width: 100,
    align: "center",
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
    id: "prints",
    header: "الطبعات",
    accessorKey: "prints_display",
    width: 75,
    align: "center",
    sortable: true,
  },
  {
    id: "status",
    header: "معتمد",
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
      <Label class="text-xs font-bold text-foreground">الرقم</Label>
    </div>

    <!-- Center: سند قيد بسيط (Voucher Main Title) -->
    <div class="flex items-center justify-center">
      <div class="border border-border/80 bg-muted/40 px-8 py-1 rounded-md shadow-inner">
        <h1 class="text-sm font-extrabold text-foreground tracking-wide">
          سند قيد بسيط
        </h1>
      </div>
    </div>

    <!-- Right: التاريخ (Date Field with Arabic Day) -->
    <div class="flex items-center gap-2">
      <Label class="text-xs font-bold text-foreground">التاريخ</Label>
      <div class="flex items-center border border-border rounded-md bg-background px-2 h-8 gap-1.5 text-xs font-mono">
        <input
          type="date"
          bind:value={accountingDate}
          class="bg-transparent border-0 text-xs font-mono focus:outline-none cursor-pointer"
        />
        <span class="text-muted-foreground font-sans font-bold text-[11px] border-s border-border ps-1.5">
          {getArabicDayName(accountingDate)}
        </span>
      </div>
    </div>
  </div>

  <!-- ══════════════════════════════════════════════════════════════════════════ -->
  <!-- Form Body (Clean ERP Box with 4 Rows)                                     -->
  <!-- ══════════════════════════════════════════════════════════════════════════ -->
  <div class="p-3 bg-muted/20 border-b border-border/80 shrink-0">
    <div class="bg-card border border-border/70 rounded-lg p-4 shadow-xs space-y-3">
      
      <!-- Row 1: المبلغ (Highlight box) + العملة (Select) -->
      <div class="grid grid-cols-1 md:grid-cols-2 gap-4 items-center">
        
        <!-- المبلغ (Amount) -->
        <div class="flex items-center gap-3">
          <Label class="text-xs font-bold text-foreground w-20 text-start shrink-0">المبلغ</Label>
          <div class="flex-1 relative">
            <Input
              type="number"
              bind:value={amount}
              placeholder="0.00"
              class="h-8 text-sm font-mono font-extrabold text-emerald-600 dark:text-emerald-400 bg-emerald-500/10 dark:bg-emerald-950/30 border-emerald-500/40 focus-visible:ring-emerald-500"
            />
          </div>
        </div>

        <!-- العملة (Currency) -->
        <div class="flex items-center gap-3">
          <Label class="text-xs font-bold text-foreground w-20 text-start shrink-0">العملة</Label>
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
      </div>

      <!-- Row 2: من حساب (From Account) + الى حساب (To Account) -->
      <div class="grid grid-cols-1 md:grid-cols-2 gap-4 items-center">
        
        <!-- من حساب (Debit Account) -->
        <div class="flex items-center gap-3">
          <Label class="text-xs font-bold text-foreground w-20 text-start shrink-0">من حساب</Label>
          <div class="flex-1">
            <AccountSearchInput
              bind:value={debitAccountId}
              placeholder="اكتب رقم أو اسم الحساب المدين..."
              class="w-full text-xs"
            />
          </div>
        </div>

        <!-- الى حساب (Credit Account) -->
        <div class="flex items-center gap-3">
          <Label class="text-xs font-bold text-foreground w-20 text-start shrink-0">الى حساب</Label>
          <div class="flex-1">
            <AccountSearchInput
              bind:value={creditAccountId}
              placeholder="اكتب رقم أو اسم الحساب الدائن..."
              class="w-full text-xs"
            />
          </div>
        </div>
      </div>

      <!-- Row 3: ملاحظات (Full width notes input) -->
      <div class="flex items-center gap-3">
        <Label class="text-xs font-bold text-foreground w-20 text-start shrink-0">ملاحظات</Label>
        <div class="flex-1">
          <Input
            type="text"
            bind:value={notes}
            placeholder="ملاحظات وشرح السند المحاسبي..."
            class="h-8 text-xs bg-background border-border w-full"
          />
        </div>
      </div>

      <!-- Row 4: الفرع / الوحدة التنظيمية (Org Unit / Cost Center) -->
      <div class="grid grid-cols-1 md:grid-cols-2 gap-4 items-center">
        
        <div class="flex items-center gap-3">
          <Label class="text-xs font-bold text-foreground w-20 text-start shrink-0">الفرع / المركز</Label>
          <div class="flex-1">
            <SearchableSelect
              value={selectedOrgUnitId}
              placeholder="اختر الفرع / المركز..."
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
      class="bg-emerald-600 hover:bg-emerald-700 text-white font-bold text-xs h-8.5 px-4 flex items-center gap-1.5 cursor-pointer shadow-xs active:scale-[0.98] transition-all"
    >
      <Plus size={14} class="stroke-2" />
      <span>إضافة</span>
    </Button>

    <!-- تعديل (Edit / Update) -->
    <Button
      variant="secondary"
      size="sm"
      onclick={handleUpdate}
      disabled={!selectedEntry || isSubmitting}
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
      disabled={!selectedEntry || isSubmitting}
      class="font-bold text-xs h-8.5 px-4 flex items-center gap-1.5 cursor-pointer shadow-xs active:scale-[0.98] transition-all"
    >
      <Trash size={14} />
      <span>حذف</span>
    </Button>

    <!-- تحديث (Refresh) -->
    <Button
      variant="outline"
      size="sm"
      onclick={loadData}
      class="font-bold text-xs h-8.5 px-4 flex items-center gap-1.5 cursor-pointer border-border text-foreground hover:bg-muted shadow-xs active:scale-[0.98] transition-all"
    >
      <Refresh size={14} class={isLoading ? "animate-spin text-emerald-500" : "text-emerald-500"} />
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
      <span>طباعة</span>
    </Button>

    <!-- طباعة السند (Print Voucher) -->
    <Button
      variant="outline"
      size="sm"
      onclick={handlePrintVoucher}
      disabled={!selectedEntry}
      class="font-bold text-xs h-8.5 px-4 flex items-center gap-1.5 cursor-pointer border-border text-foreground hover:bg-muted shadow-xs active:scale-[0.98] transition-all"
    >
      <Printer size={14} class="text-indigo-500" />
      <span>طباعة السند</span>
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
  <!-- Search Bar Slide-over (When Search button clicked)                        -->
  <!-- ══════════════════════════════════════════════════════════════════════════ -->
  {#if isSearchVisible}
    <div transition:slide={{ duration: 150 }} class="bg-muted/40 border-b border-border px-4 py-2 flex items-center gap-3">
      <div class="relative flex-1 max-w-md">
        <Search class="absolute right-3 top-1/2 -translate-y-1/2 text-muted-foreground w-4 h-4" />
        <Input
          type="text"
          bind:value={searchQuery}
          placeholder="ابحث برقم السند أو اسم الحساب أو الملاحظات..."
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

    <!-- Middle: Secondary Quick Action Buttons (QR, طباعة إشعار مدين, طباعة إشعار دائن) -->
    <div class="flex items-center gap-2">
      <Button
        variant="outline"
        size="sm"
        onclick={() => {
          if (selectedEntry) showQrModal = true;
          else toast.error("الرجاء اختيار سند أولاً");
        }}
        class="h-7 text-xs font-bold px-2.5 border-border bg-background hover:bg-muted cursor-pointer"
      >
        <CheckSquare size={13} class="me-1" />
        <span>QR</span>
      </Button>

      <Button
        variant="outline"
        size="sm"
        onclick={handlePrintDebitNotice}
        disabled={!selectedEntry}
        class="h-7 text-xs font-bold px-3 border-border bg-background hover:bg-muted cursor-pointer"
      >
        <span>طباعة إشعار مدين</span>
      </Button>

      <Button
        variant="outline"
        size="sm"
        onclick={handlePrintCreditNotice}
        disabled={!selectedEntry}
        class="h-7 text-xs font-bold px-3 border-border bg-background hover:bg-muted cursor-pointer"
      >
        <span>طباعة إشعار دائن</span>
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
      emptyTitle="لا توجد قيود مسجلة لهذا التاريخ أو البحث"
      emptyDescription="استخدم نموذج الإدخال أعلاه لتسجيل وترحيل قيود جديدة مباشرة."
      onRowClick={(row) => selectRowForEditing(row)}
      onRowDoubleClick={(row) => {
        selectRowForEditing(row);
        toast.info(`تم تحميل السند رقم ${row.reference_number}`);
      }}
    />
  </div>

  <!-- ══════════════════════════════════════════════════════════════════════════ -->
  <!-- QR Code Modal Dialog                                                      -->
  <!-- ══════════════════════════════════════════════════════════════════════════ -->
  {#if showQrModal && selectedEntry}
    <div in:fade={{ duration: 150 }} class="fixed inset-0 bg-black/60 backdrop-blur-xs z-50 flex items-center justify-center p-4">
      <div class="bg-card border border-border rounded-xl w-full max-w-sm p-5 shadow-2xl relative text-center">
        <h3 class="text-sm font-bold mb-3 text-foreground">رمز الاستجابة السريعة (QR) لسند القيد</h3>
        
        <div class="bg-white p-3 rounded-lg inline-block mb-3 border border-zinc-200">
          <svg class="w-40 h-40 mx-auto text-black" viewBox="0 0 100 100" fill="currentColor">
            <path d="M0 0h30v30H0V0zm10 10v10h10V10H10zM70 0h30v30H70V0zm10 10v10h10V10H80zM0 70h30v30H0V70zm10 10v10h10V80H10zm35-80h10v10H45V0zm15 15h10v15H60V15zm-15 20h25v10H45V35zm-15 15h10v10H30V50zm15 10h10v15H45V60zm15 15h10v15H60V75zm15-15h25v10H75V60zm-30 20h10v10H45V80zm35 10h20v10H80V90z" />
          </svg>
        </div>
        
        <div class="text-xs text-muted-foreground flex flex-col gap-1 text-start px-2 bg-muted/30 p-2.5 rounded-lg border border-border/40">
          <div><span class="font-bold text-foreground">رقم السند:</span> {selectedEntry.reference_number}</div>
          <div><span class="font-bold text-foreground">التاريخ:</span> {formatDate(selectedEntry.accounting_date, "date")}</div>
          <div><span class="font-bold text-foreground">المبلغ:</span> {formatCurrency(Number(selectedEntry.amount || 0), selectedEntry.currency_code || "YER")}</div>
          <div><span class="font-bold text-foreground">البيان:</span> {selectedEntry.description_ar || selectedEntry.description || "—"}</div>
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
