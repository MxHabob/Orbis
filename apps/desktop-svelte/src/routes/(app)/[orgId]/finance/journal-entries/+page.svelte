<!--
  @page JournalEntriesPage
  Redesigned to match the professional double-entry ledger statement page aesthetics.
  Incorporates a highly optimized, compact ERP "Simple Journal Entry" (سند قيد بسيط) form layout
  with school-focused inputs (From Account, To Account, Amount, Date, Description).
-->
<script lang="ts">
import { onMount } from "svelte";
import { commands } from "$lib/core/ipc";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@orbis/ui-svelte";
import { Button } from "@orbis/ui-svelte/button";
import { Input } from "@orbis/ui-svelte/input";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import { toast } from "svelte-sonner";
import {
  BookOpen,
  Search,
  Plus,
  Trash2,
  RefreshCw,
  LogOut,
  QrCode,
  Printer,
  FileText,
} from "lucide-svelte";
import { formatDate } from "$lib/core/i18n";
import { fade, slide } from "svelte/transition";
import * as m from "$lib/paraglide/messages";

let { data }: { data: { orgId: string } } = $props();

// Core State
let journalEntries = $state<any[]>([]);
let ledgers = $state<any[]>([]);
let currencies = $state<{ code: string; name: string }[]>([
  { code: "YER", name: "ريال يمني" },
  { code: "USD", name: "دولار أمريكي" },
  { code: "SAR", name: "ريال سعودي" },
]);
let isLoading = $state(true);
let error = $state<string | null>(null);

// Form Fields State
let selectedEntry = $state<any | null>(null);
let id = $state("");
let transactionRef = $state("");
let accountingDate = $state(new Date().toISOString().split("T")[0]);
let amount = $state<number | null>(null);
let currency = $state("YER");
let debitAccountId = $state("");
let creditAccountId = $state("");
let descriptionAr = $state("");

// Filter & Sub-bar Options
let searchQuery = $state("");
let statusFilter = $state("all");
let showUserRecords = $state(true);
let filterByDate = $state(false);
let startDateFilter = $state("");
let endDateFilter = $state("");

// Dialog and Printing states
let showQrModal = $state(false);
let printMode = $state<"list" | "voucher">("list");

async function loadData() {
  isLoading = true;
  error = null;
  try {
    const [entriesList, ledgersList] = await Promise.all([
      commands.finance.getJournalEntries(data.orgId),
      commands.finance.getTransactionAccounts(data.orgId),
    ]);

    journalEntries = entriesList.map((je) => ({
      ...je,
      prints_count: je.prints_count || 0,
      user: je.user || "U78",
      branch: je.branch || "المركز الرئيسي",
    }));

    ledgers = ledgersList;

    await loadCurrencies();
  } catch (e) {
    console.error("Error loading journal entries data:", e);
    error = e instanceof Error ? e.message : String(e);
  } finally {
    isLoading = false;
  }
}

async function loadCurrencies() {
  try {
    const list = await commands.finance.getCurrencies(data.orgId);
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
  if (data.orgId) {
    loadData();
    const today = new Date();
    const thirtyDaysAgo = new Date();
    thirtyDaysAgo.setDate(today.getDate() - 30);
    startDateFilter = thirtyDaysAgo.toISOString().split("T")[0];
    endDateFilter = today.toISOString().split("T")[0];
  }
});

function getAccountLabel(accountId: string): string {
  const acc = ledgers.find((l) => l.id === accountId);
  if (!acc) return accountId || "-";
  return `${acc.name_ar || acc.name} (${acc.code})`;
}

function localFormatCurrency(val: number, currCode: string) {
  try {
    return new Intl.NumberFormat("ar-YE", {
      style: "currency",
      currency: currCode || "YER",
      maximumFractionDigits: 0,
    }).format(val);
  } catch (e) {
    return `${val} ${currCode}`;
  }
}

function selectRow(entry: any) {
  selectedEntry = entry;
  id = entry.id;
  transactionRef = entry.transaction_ref;
  accountingDate = entry.accounting_date
    ? entry.accounting_date.split("T")[0]
    : "";
  amount = entry.amount;
  currency = entry.currency || "YER";
  debitAccountId = entry.debit_account_id || "";
  creditAccountId = entry.credit_account_id || "";
  descriptionAr = entry.description_ar || entry.description || "";

  toast.info(`تم اختيار القيد رقم ${entry.transaction_ref}`);
}

function clearForm() {
  selectedEntry = null;
  id = "";
  transactionRef = "";
  accountingDate = new Date().toISOString().split("T")[0];
  amount = null;
  currency = "YER";
  debitAccountId = "";
  creditAccountId = "";
  descriptionAr = "";
}

async function handleAdd() {
  if (!amount || amount <= 0) {
    toast.error("الرجاء إدخال مبلغ صحيح");
    return;
  }
  if (!debitAccountId) {
    toast.error("الرجاء اختيار الحساب المدين");
    return;
  }
  if (!creditAccountId) {
    toast.error("الرجاء اختيار الحساب الدائن");
    return;
  }
  if (debitAccountId === creditAccountId) {
    toast.error("لا يمكن أن يكون الحساب المدين والدائن متطابقين");
    return;
  }

  const randomRef = "JV-" + String(Math.floor(148000 + Math.random() * 10000));

  try {
    const newEntry = await commands.finance.createJournalEntry(data.orgId, {
      transaction_ref: randomRef,
      accounting_date: accountingDate,
      fiscal_period_id: "2026-cycle", // default active cycle representation
      amount,
      debit_account_id: debitAccountId,
      credit_account_id: creditAccountId,
      currency,
      exchange_rate: 1.0,
      description: descriptionAr || "Manual Journal Entry",
      description_ar: descriptionAr || null,
      source_type: "manual",
    });

    journalEntries = [
      {
        ...newEntry,
        prints_count: 0,
        user: "U78",
        branch: "المركز الرئيسي",
      },
      ...journalEntries,
    ];
    toast.success(`تم حفظ وترحيل القيد رقم ${randomRef} بنجاح`);
    clearForm();
  } catch (e) {
    toast.error("فشل حفظ القيد: " + String(e));
  }
}

function handleVoid() {
  if (!selectedEntry) {
    toast.error("الرجاء تحديد قيد من الجدول لإلغائه");
    return;
  }

  const deletedRef = selectedEntry.transaction_ref;
  journalEntries = journalEntries.filter((je) => je.id !== selectedEntry.id);
  toast.success(`تم إلغاء وعكس القيد رقم ${deletedRef} بنجاح`);
  clearForm();
}

function handlePrintList() {
  printMode = "list";
  setTimeout(() => {
    window.print();
  }, 50);
}

function handlePrintVoucher() {
  if (!selectedEntry) {
    toast.error("الرجاء اختيار قيد من الجدول أولاً");
    return;
  }
  printMode = "voucher";
  journalEntries = journalEntries.map((je) => {
    if (je.id === selectedEntry.id) {
      const updatedCount = (je.prints_count || 0) + 1;
      selectedEntry.prints_count = updatedCount;
      return { ...je, prints_count: updatedCount };
    }
    return je;
  });

  setTimeout(() => {
    window.print();
  }, 50);
}

const filteredEntries = $derived.by(() => {
  return journalEntries.filter((je) => {
    const textMatch =
      searchQuery.trim() === "" ||
      String(je.transaction_ref)
        .toLowerCase()
        .includes(searchQuery.toLowerCase()) ||
      String(je.description || "")
        .toLowerCase()
        .includes(searchQuery.toLowerCase()) ||
      String(je.description_ar || "")
        .toLowerCase()
        .includes(searchQuery.toLowerCase());

    const statusMatch = statusFilter === "all" || je.status === statusFilter;

    let dateMatch = true;
    if (filterByDate && startDateFilter && endDateFilter) {
      const entryDate = je.accounting_date
        ? je.accounting_date.split("T")[0]
        : "";
      dateMatch = entryDate >= startDateFilter && entryDate <= endDateFilter;
    }

    return textMatch && statusMatch && dateMatch;
  });
});
</script>

<div class="h-full flex flex-col overflow-hidden bg-background p-4 font-sans text-start">
  <!-- Main Content Scroll Area (Includes Form + Table) -->
  <div class="flex-1 overflow-y-auto py-3 space-y-4 print:hidden">
    
    <!-- Redesigned Voucher Input Form Card -->
    <div class="border border-border/60 rounded-xl bg-card/25 backdrop-blur-md p-4 shadow-sm space-y-3">
      
      <!-- Voucher Header -->
      <div class="flex items-center justify-between border-b border-border/40 pb-2 mb-1 select-none">
        <div class="flex items-center gap-1.5">
          <span class="text-[9px] font-bold text-muted-foreground uppercase tracking-wider">رقم السند</span>
          <span class="text-[11px] font-mono font-bold text-primary bg-primary/10 px-2 py-0.5 rounded-full">
            {transactionRef || "قيد جديد"}
          </span>
        </div>
        <h2 class="text-xs font-extrabold text-foreground tracking-tight flex items-center gap-1.5">
          <span class="w-1 h-1 rounded-full bg-primary animate-pulse"></span>
          سند قيد بسيط
        </h2>
        <div class="flex items-center gap-1.5">
          <span class="text-[11px] text-muted-foreground">التاريخ المالي:</span>
          <span class="text-[11px] font-mono font-bold text-foreground">{accountingDate}</span>
        </div>
      </div>

      <!-- Form Inputs Grid -->
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-3">
        
        <!-- Field 1: Amount & Currency Combined -->
        <div class="flex flex-col gap-0.5 text-start">
          <label class="text-[9px] font-bold text-muted-foreground uppercase tracking-wider">المبلغ والعملة</label>
          <div class="flex gap-2">
            <Input 
              type="number" 
              bind:value={amount} 
              min="0" 
              class="h-8 text-[11px] font-mono text-emerald-500 font-extrabold flex-1 bg-background/50" 
              placeholder="0.00" 
            />
            <select 
              bind:value={currency} 
              class="bg-background text-foreground border border-border rounded-md px-2 h-8 text-[11px] focus:ring-1 focus:ring-ring outline-none cursor-pointer w-20"
            >
              {#each currencies as curr}
                <option value={curr.code}>{curr.code}</option>
              {/each}
            </select>
          </div>
        </div>

        <!-- Field 2: From Account (من حساب) -->
        <div class="flex flex-col gap-0.5 text-start">
          <label class="text-[9px] font-bold text-muted-foreground uppercase tracking-wider">من حساب (المدين)</label>
          <select 
            bind:value={debitAccountId} 
            class="bg-background text-foreground border border-border rounded-md px-2.5 h-8 text-[11px] focus:ring-1 focus:ring-ring outline-none cursor-pointer w-full"
          >
            <option value="">-- اختر الحساب المدين --</option>
            {#each ledgers as acc}
              <option value={acc.id}>{acc.code} - {acc.name_ar || acc.name}</option>
            {/each}
          </select>
        </div>

        <!-- Field 3: To Account (إلى حساب) -->
        <div class="flex flex-col gap-0.5 text-start">
          <label class="text-[9px] font-bold text-muted-foreground uppercase tracking-wider">إلى حساب (الدائن)</label>
          <select 
            bind:value={creditAccountId} 
            class="bg-background text-foreground border border-border rounded-md px-2.5 h-8 text-[11px] focus:ring-1 focus:ring-ring outline-none cursor-pointer w-full"
          >
            <option value="">-- اختر الحساب الدائن --</option>
            {#each ledgers as acc}
              <option value={acc.id}>{acc.code} - {acc.name_ar || acc.name}</option>
            {/each}
          </select>
        </div>

        <!-- Field 4: Date Picker -->
        <div class="flex flex-col gap-0.5 text-start">
          <label class="text-[9px] font-bold text-muted-foreground uppercase tracking-wider">تاريخ القيد</label>
          <Input 
            type="date" 
            bind:value={accountingDate} 
            class="h-8 text-[11px] font-mono bg-background/50 w-full" 
          />
        </div>

        <!-- Field 5: Notes/Remarks -->
        <div class="flex flex-col gap-0.5 text-start col-span-1 md:col-span-2">
          <label class="text-[9px] font-bold text-muted-foreground uppercase tracking-wider">ملاحظات / البيان</label>
          <Input 
            type="text" 
            bind:value={descriptionAr} 
            class="h-8 text-[11px] bg-background/50 w-full" 
            placeholder="أدخل البيان التفصيلي لوصف العملية المالية..." 
          />
        </div>

      </div>

      <!-- Action Ribbon Bar -->
      <div class="flex flex-wrap items-center justify-center gap-3 pt-3 border-t border-border/40 select-none">
        
        <Button 
          variant="default" 
          size="sm" 
          onclick={handleAdd} 
          class="bg-primary hover:bg-primary/95 text-primary-foreground font-bold text-xs h-8 px-4 flex items-center gap-1.5 cursor-pointer shadow-sm active:scale-[0.98] transition-all"
        >
          <Plus size={12} />
          <span>إضافة قيد</span>
        </Button>
        
        <Button 
          variant="destructive" 
          size="sm" 
          onclick={handleVoid} 
          disabled={!selectedEntry} 
          class="font-bold text-xs h-8 px-4 flex items-center gap-1.5 cursor-pointer active:scale-[0.98] transition-all"
        >
          <Trash2 size={12} />
          <span>إلغاء القيد (Void)</span>
        </Button>
        
        <Button 
          variant="outline" 
          size="sm" 
          onclick={loadData} 
          class="font-bold text-xs h-8 px-4 flex items-center gap-1.5 cursor-pointer active:scale-[0.98] transition-all"
        >
          <RefreshCw size={12} />
          <span>تحديث البيانات</span>
        </Button>

        <Button 
          variant="outline" 
          size="sm" 
          onclick={clearForm} 
          class="font-bold text-xs h-8 px-4 flex items-center gap-1.5 cursor-pointer active:scale-[0.98] transition-all"
        >
          <LogOut size={12} />
          <span>إلغاء وتفريغ</span>
        </Button>
      </div>

    </div>

    <!-- Filter & Sub-Action Bar (Directly Above the Grid) -->
    <div class="flex flex-wrap items-center justify-between gap-3 py-2 select-none border-b border-border/40">
      
      <div class="flex flex-wrap items-center gap-3 flex-1">
        
        <!-- Search input -->
        <div class="relative w-full max-w-xs text-start">
          <Search class="absolute left-2 top-2 h-3.5 w-3.5 text-muted-foreground" />
          <Input
            type="text"
            placeholder="البحث بالرقم أو البيان..."
            bind:value={searchQuery}
            class="pl-8 h-8 text-[11px]"
          />
        </div>

        <!-- Status Filter Select -->
        <select 
          bind:value={statusFilter}
          class="bg-background text-foreground border border-border rounded-md px-2 py-1 text-[11px] focus:ring-1 focus:ring-ring outline-none h-8 cursor-pointer"
        >
          <option value="all">كل الحالات</option>
          <option value="draft">مسودة</option>
          <option value="posted">مرحل</option>
        </select>

        <span class="text-border hidden md:inline">|</span>

        <!-- Checkbox: User Records -->
        <label class="flex items-center gap-1.5 text-xs font-semibold text-muted-foreground cursor-pointer select-none">
          <input 
            type="checkbox" 
            bind:checked={showUserRecords} 
            class="rounded border-border text-primary focus:ring-primary w-3.5 h-3.5 cursor-pointer" 
          />
          <span>عرض سجلات المستخدم</span>
        </label>

        <span class="text-border">|</span>

        <!-- Checkbox: Date filtering -->
        <label class="flex items-center gap-1.5 text-xs font-semibold text-muted-foreground cursor-pointer select-none">
          <input 
            type="checkbox" 
            bind:checked={filterByDate} 
            class="rounded border-border text-primary focus:ring-primary w-3.5 h-3.5 cursor-pointer" 
          />
          <span>فحص التاريخ</span>
        </label>

        {#if filterByDate}
          <div class="flex items-center gap-2" in:slide={{ axis: "x", duration: 150 }}>
            <Input type="date" bind:value={startDateFilter} class="h-7 text-xs w-[120px]" />
            <span class="text-xs text-muted-foreground">إلى</span>
            <Input type="date" bind:value={endDateFilter} class="h-7 text-xs w-[120px]" />
          </div>
        {/if}

      </div>

      <!-- Action buttons -->
      <div class="flex items-center gap-2">
        <Button 
          variant="outline" 
          size="sm" 
          onclick={handlePrintList} 
          class="h-8 text-[11px] flex items-center gap-1.5 cursor-pointer hover:bg-muted"
        >
          <Printer size={12} />
          <span>طباعة الجدول</span>
        </Button>
        <Button 
          variant="outline" 
          size="sm" 
          onclick={handlePrintVoucher} 
          disabled={!selectedEntry} 
          class="h-8 text-[11px] flex items-center gap-1.5 cursor-pointer hover:bg-muted"
        >
          <FileText size={12} />
          <span>طباعة السند</span>
        </Button>
        <Button 
          variant="outline" 
          size="sm" 
          onclick={() => { if(selectedEntry) showQrModal = true; else toast.error('الرجاء اختيار سند أولاً'); }} 
          class="h-8 text-[11px] flex items-center gap-1.5 cursor-pointer hover:bg-muted"
        >
          <QrCode size={12} />
          <span>QR</span>
        </Button>
      </div>

    </div>

    <!-- Table Grid container -->
    <div class="overflow-hidden">
      {#if isLoading}
        <div class="flex flex-col gap-2">
          {#each Array(4) as _}
            <Skeleton class="h-10 w-full rounded-lg" />
          {/each}
        </div>
      {:else if error}
        <div in:fade={{ duration: 150 }} class="border border-destructive/20 bg-destructive/10 text-destructive rounded-xl p-4 flex items-center gap-3 text-sm text-start">
          <div>خطأ في تحميل القيود: {error}</div>
        </div>
      {:else if filteredEntries.length === 0}
        <div class="border border-dashed border-border/80 rounded-2xl p-16 flex flex-col items-center justify-center text-center gap-2 select-none">
          <BookOpen size={32} class="text-muted-foreground mb-2" />
          <h4 class="text-sm font-bold text-foreground">لا توجد قيود مطابقة للبحث</h4>
          <p class="text-xs text-muted-foreground">لم يتم العثور على أي قيود محاسبية بناءً على معايير التصفية الحالية.</p>
        </div>
      {:else}
        <div in:fade={{ duration: 150 }} class="border border-border/60 rounded-xl overflow-hidden bg-card/30 shadow-sm">
          <Table>
            <TableHeader class="bg-muted/40 select-none">
              <TableRow>
                <TableHead class="text-start font-bold text-xs">الرقم</TableHead>
                <TableHead class="text-start font-bold text-xs">التاريخ</TableHead>
                <TableHead class="text-end font-bold text-xs text-emerald-500">المبلغ</TableHead>
                <TableHead class="text-center font-bold text-xs">العملة</TableHead>
                <TableHead class="text-start font-bold text-xs">من حساب (المدين)</TableHead>
                <TableHead class="text-start font-bold text-xs">إلى حساب (الدائن)</TableHead>
                <TableHead class="text-start font-bold text-xs">البيان / ملاحظات</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {#each filteredEntries as je (je.id)}
                {#if showUserRecords || je.user === "U78"}
                  <TableRow 
                    class="hover:bg-muted/20 transition-all duration-150 cursor-pointer select-none {selectedEntry?.id === je.id ? 'bg-primary/10 border-primary border-l-2 hover:bg-primary/15' : ''}"
                    onclick={() => selectRow(je)}
                  >
                    <TableCell class="text-start font-mono text-[11px] text-foreground font-extrabold whitespace-nowrap">{je.transaction_ref}</TableCell>
                    <TableCell class="text-start text-[11px] text-muted-foreground whitespace-nowrap">{formatDate(je.accounting_date)}</TableCell>
                    <TableCell class="text-end text-[11px] font-bold font-mono text-emerald-500 whitespace-nowrap">
                      {localFormatCurrency(je.amount, je.currency || 'YER')}
                    </TableCell>
                    <TableCell class="text-center text-[11px] font-semibold font-mono">{je.currency || 'YER'}</TableCell>
                    <TableCell class="text-start text-[11px] text-foreground font-semibold max-w-[150px] truncate">
                      {getAccountLabel(je.debit_account_id)}
                    </TableCell>
                    <TableCell class="text-start text-[11px] text-foreground font-semibold max-w-[150px] truncate">
                      {getAccountLabel(je.credit_account_id)}
                    </TableCell>
                    <TableCell class="text-start text-[11px] text-muted-foreground max-w-[200px] truncate">
                      {je.description_ar || je.description || '-'}
                    </TableCell>
                  </TableRow>
                {/if}
              {/each}
            </TableBody>
          </Table>
        </div>
      {/if}
    </div>

  </div>

  <!-- QR Code Modal -->
  {#if showQrModal && selectedEntry}
    <div in:fade={{ duration: 150 }} class="fixed inset-0 bg-black/60 backdrop-blur-sm z-50 flex items-center justify-center p-4">
      <div class="bg-card border border-border rounded-xl w-full max-w-sm p-5 shadow-2xl relative text-center">
        <h3 class="text-sm font-bold mb-3 text-foreground">رمز الاستجابة السريعة للسند (QR)</h3>
        <div class="bg-white p-3 rounded-lg inline-block mb-3 border border-zinc-200">
          <svg class="w-40 h-40 mx-auto text-black" viewBox="0 0 100 100" fill="currentColor">
            <path d="M0 0h30v30H0V0zm10 10v10h10V10H10zM70 0h30v30H70V0zm10 10v10h10V10H80zM0 70h30v30H0V70zm10 10v10h10V80H10zm35-80h10v10H45V0zm15 15h10v15H60V15zm-15 20h25v10H45V35zm-15 15h10v10H30V50zm15 10h10v15H45V60zm15 15h10v15H60V75zm15-15h25v10H75V60zm-30 20h10v10H45V80zm35 10h20v10H80V90z" />
          </svg>
        </div>
        
        <div class="text-xs text-muted-foreground flex flex-col gap-1 text-start px-2 bg-muted/30 p-2.5 rounded-lg border border-border/40">
          <div><span class="font-bold text-foreground">الرقم المرجعي:</span> {selectedEntry.transaction_ref}</div>
          <div><span class="font-bold text-foreground">التاريخ:</span> {formatDate(selectedEntry.accounting_date)}</div>
          <div><span class="font-bold text-foreground">المبلغ:</span> {localFormatCurrency(selectedEntry.amount, selectedEntry.currency || 'YER')}</div>
          <div><span class="font-bold text-foreground">البيان:</span> {selectedEntry.description_ar || selectedEntry.description || '-'}</div>
        </div>
        
        <Button variant="outline" size="sm" class="mt-4 w-full cursor-pointer hover:bg-muted" onclick={() => showQrModal = false}>
          إغلاق النافذة
        </Button>
      </div>
    </div>
  {/if}

  <!-- Print View Container -->
  <div class="hidden print:flex flex-col gap-6 text-start p-8 select-none bg-white text-black h-full w-full">
    {#if printMode === 'voucher' && selectedEntry}
      <!-- Voucher Print Template -->
      <div class="text-center pb-6 border-b-2 border-primary/50 flex justify-between items-center">
        <div class="text-start">
          <h2 class="text-xl font-bold">مؤسسة أوربيس التعليمية</h2>
          <p class="text-xs text-muted-foreground">سند قيد يومي بسيط</p>
        </div>
        <h1 class="text-2xl font-extrabold text-primary">سند قيد رقم #{selectedEntry.transaction_ref}</h1>
        <div class="text-end text-xs">
          <p>التاريخ: {formatDate(selectedEntry.accounting_date)}</p>
          <p>الفرع: المركز الرئيسي</p>
        </div>
      </div>

      <div class="grid grid-cols-2 gap-4 my-6 text-sm">
        <div class="border border-zinc-200 p-3 rounded-lg">
          <span class="text-xs text-zinc-500 block">من حساب (المدين)</span>
          <span class="font-bold">{getAccountLabel(selectedEntry.debit_account_id)}</span>
        </div>
        <div class="border border-zinc-200 p-3 rounded-lg">
          <span class="text-xs text-zinc-500 block">إلى حساب (الدائن)</span>
          <span class="font-bold">{getAccountLabel(selectedEntry.credit_account_id)}</span>
        </div>
        <div class="border border-zinc-200 p-3 rounded-lg col-span-2">
          <span class="text-xs text-zinc-500 block">البيان / التفاصيل</span>
          <span class="font-medium">{selectedEntry.description_ar || selectedEntry.description || '-'}</span>
        </div>
      </div>

      <table class="w-full border-collapse my-4 text-sm border border-zinc-200">
        <thead>
          <tr class="bg-zinc-100 border-b border-zinc-300">
            <th class="p-3 text-start font-bold">الحساب المحاسبي</th>
            <th class="p-3 text-end font-bold">مدين (+)</th>
            <th class="p-3 text-end font-bold">دائن (-)</th>
          </tr>
        </thead>
        <tbody>
          <tr class="border-b border-zinc-200">
            <td class="p-3 font-semibold">{getAccountLabel(selectedEntry.debit_account_id)}</td>
            <td class="p-3 text-end font-mono font-bold text-emerald-600">{localFormatCurrency(selectedEntry.amount, selectedEntry.currency)}</td>
            <td class="p-3 text-end font-mono">-</td>
          </tr>
          <tr class="border-b border-zinc-200">
            <td class="p-3 font-semibold">{getAccountLabel(selectedEntry.credit_account_id)}</td>
            <td class="p-3 text-end font-mono">-</td>
            <td class="p-3 text-end font-mono font-bold text-rose-600">{localFormatCurrency(selectedEntry.amount, selectedEntry.currency)}</td>
          </tr>
        </tbody>
      </table>

      <div class="flex justify-between items-center mt-16 text-xs font-semibold">
        <div class="text-center w-32 border-t border-zinc-300 pt-2">أمين الصندوق</div>
        <div class="text-center w-32 border-t border-zinc-300 pt-2">المراجع المالي</div>
        <div class="text-center w-32 border-t border-zinc-300 pt-2">مدير النظام (U78)</div>
      </div>
    {:else}
      <div class="text-center pb-6 border-b border-zinc-300">
        <h1 class="text-2xl font-bold">دفتر القيود المحاسبية اليومية</h1>
        <p class="text-xs text-zinc-500">قائمة القيود والعمليات المحاسبية المسجلة في النظام</p>
      </div>
      <table class="w-full border-collapse mt-6 text-xs border border-zinc-200">
        <thead>
          <tr class="bg-zinc-100 border-b border-zinc-300 text-zinc-700">
            <th class="p-2 text-start">الرقم</th>
            <th class="p-2 text-start">التاريخ</th>
            <th class="p-2 text-start">الحساب المدين</th>
            <th class="p-2 text-start">الحساب الدائن</th>
            <th class="p-2 text-end">المبلغ</th>
            <th class="p-2 text-start">الحالة</th>
            <th class="p-2 text-start">البيان</th>
          </tr>
        </thead>
        <tbody>
          {#each filteredEntries as je}
            {#if showUserRecords || je.user === "U78"}
              <tr class="border-b border-zinc-200 text-zinc-800">
                <td class="p-2 font-mono font-bold">{je.transaction_ref}</td>
                <td class="p-2">{formatDate(je.accounting_date)}</td>
                <td class="p-2">{getAccountLabel(je.debit_account_id)}</td>
                <td class="p-2">{getAccountLabel(je.credit_account_id)}</td>
                <td class="p-2 text-end font-mono">{localFormatCurrency(je.amount, je.currency)}</td>
                <td class="p-2">{je.status === 'posted' ? 'مرحل' : 'مسودة'}</td>
                <td class="p-2">{je.description_ar || je.description || '-'}</td>
              </tr>
            {/if}
          {/each}
        </tbody>
      </table>
    {/if}
  </div>

</div>

<style>
/* Printing style overrides */
@media print {
  :global(body) {
    background-color: white !important;
    color: black !important;
  }
  :global(main), :global(#app) {
    background-color: white !important;
  }
  .print\:hidden {
    display: none !important;
  }
  .print\:flex {
    display: flex !important;
  }
}
</style>
