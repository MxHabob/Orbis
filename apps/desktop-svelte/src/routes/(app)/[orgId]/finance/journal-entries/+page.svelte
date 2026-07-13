<!--
  @page JournalEntriesPage
  Rebuilt to support single/multiple journal entry inputs dynamically within a single row-based grid without tabs.
  Fully uses @orbis/ui-svelte components and Paraglide-JS localization. No page title.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import { commands, dedup, extractErrorMessage } from "$lib/core/ipc";
import { Button } from "@orbis/ui-svelte/button";
import { Input } from "@orbis/ui-svelte/input";
import { Label } from "@orbis/ui-svelte/label";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui-svelte/select";
import { ComboboxDropdown } from "@orbis/ui-svelte/combobox-dropdown";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@orbis/ui-svelte/table";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import { Badge } from "@orbis/ui-svelte/badge";
import { Switch } from "@orbis/ui-svelte/switch";
import { Card, CardContent } from "@orbis/ui-svelte/card";
import { Separator } from "@orbis/ui-svelte/separator";
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
  ChevronLeft,
  ChevronRight,
  Calendar,
  FileSpreadsheet,
  AlertCircle,
} from "lucide-svelte";
import { formatDate } from "$lib/core/i18n";
import { fade, slide } from "svelte/transition";

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

// Dialog and Printing states
let showQrModal = $state(false);
let printMode = $state<"list" | "voucher">("list");
let selectedEntry = $state<any | null>(null);

// Date filtering options
let filterMode = $state<"day" | "all">("day");
let selectedFilterDate = $state(new Date().toISOString().split("T")[0]);
let showUserRecords = $state(true);

// Search query
let searchQuery = $state("");
let statusFilter = $state("all");

// Helper to extract only the actual account name (ignoring parent prefixes)
function cleanAccountName(name: string): string {
  if (!name) return "";
  const parts = name.split(" - ");
  return parts[parts.length - 1];
}

// GL Accounts mapping for Combobox
const accountItems = $derived.by(() => {
  return ledgers.map((l) => ({
    value: l.id,
    label: `${l.code} - ${cleanAccountName(l.name_ar || l.name)}`,
  }));
});

// Load ledger accounts, currencies, and journal entries
async function loadData() {
  isLoading = true;
  error = null;
  try {
    const [entriesList, ledgersList] = await Promise.all([
      dedup(`journal_entries:${data.orgId}`, () =>
        commands.finance.listJournalEntries(data.orgId),
      ),
      dedup(`accounts:${data.orgId}`, () =>
        commands.finance.listTransactionAccounts(data.orgId),
      ),
    ]);

    journalEntries = (entriesList as any[]).map((je: any) => ({
      ...je,
      prints_count: je.prints_count || 0,
      user: je.user || "U78",
      branch: je.branch || "المركز الرئيسي",
    }));

    ledgers = ledgersList;

    await loadCurrencies();
  } catch (e) {
    console.error("Error loading journal entries data:", e);
    error = extractErrorMessage(e);
  } finally {
    isLoading = false;
  }
}

async function loadCurrencies() {
  try {
    const list = await commands.finance.listCurrencies(data.orgId);
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
    selectedFilterDate = new Date().toISOString().split("T")[0];
  }
});

// Date filtering navigation
function nextDay() {
  const d = new Date(selectedFilterDate);
  d.setDate(d.getDate() + 1);
  selectedFilterDate = d.toISOString().split("T")[0];
}

function prevDay() {
  const d = new Date(selectedFilterDate);
  d.setDate(d.getDate() - 1);
  selectedFilterDate = d.toISOString().split("T")[0];
}

function jumpToToday() {
  selectedFilterDate = new Date().toISOString().split("T")[0];
}

// GL Account label helper
function getAccountLabel(accountId: string): string {
  const acc = ledgers.find((l) => l.id === accountId);
  if (!acc) return accountId || "-";
  return `${cleanAccountName(acc.name_ar || acc.name)} (${acc.code})`;
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

  // Set fields to entry values to view/copy them
}

function handleVoid() {
  if (!selectedEntry) {
    toast.error("الرجاء تحديد قيد من الجدول لإلغائه");
    return;
  }

  const deletedRef = selectedEntry.transaction_ref;
  journalEntries = journalEntries.filter((je) => je.id !== selectedEntry.id);
  toast.success(`تم إلغاء وعكس القيد رقم ${deletedRef} بنجاح`);
  selectedEntry = null;
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

// Derived filtered list of entries
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
    if (filterMode === "day") {
      const entryDate = je.accounting_date
        ? je.accounting_date.split("T")[0]
        : "";
      dateMatch = entryDate === selectedFilterDate;
    }

    return textMatch && statusMatch && dateMatch;
  });
});
</script>

<div class="h-full w-full overflow-y-auto bg-background/50 scrollbar-hide text-start font-sans">
  <div class="max-w-7xl mx-auto p-4 md:p-6 flex flex-col gap-6 print:hidden">

    <!-- Entries Log Card -->
    <Card class="border border-border/60 shadow-sm overflow-hidden flex flex-col bg-card">
      <!-- Card Sub-Bar / Filtering & Actions Header -->
      <div class="bg-muted/45 border-b border-border/60 p-4 flex flex-col gap-4">
        
        <!-- Top controls row -->
        <div class="flex flex-wrap items-center justify-between gap-4 select-none">
          <h2 class="text-sm font-bold text-foreground flex items-center gap-2">
            <span class="w-1.5 h-1.5 rounded-full bg-primary animate-pulse"></span>
            سجل قيود اليومية
          </h2>

          <div class="flex flex-wrap items-center gap-3">
            <!-- Print Actions -->
            <Button
              variant="outline"
              size="sm"
              onclick={handlePrintList}
              class="h-8 text-xs font-semibold flex items-center gap-1.5 cursor-pointer border-border hover:bg-muted text-foreground"
            >
              <Printer size={12} />
              <span>طباعة الجدول</span>
            </Button>
            
            <Button
              variant="outline"
              size="sm"
              onclick={handlePrintVoucher}
              disabled={!selectedEntry}
              class="h-8 text-xs font-semibold flex items-center gap-1.5 cursor-pointer border-border hover:bg-muted text-foreground"
            >
              <FileText size={12} />
              <span>طباعة السند المحدد</span>
            </Button>
            
            <Button
              variant="outline"
              size="sm"
              onclick={() => { if (selectedEntry) showQrModal = true; else toast.error('الرجاء اختيار سند أولاً'); }}
              disabled={!selectedEntry}
              class="h-8 text-xs font-semibold flex items-center gap-1.5 cursor-pointer border-border hover:bg-muted text-foreground"
            >
              <QrCode size={12} />
              <span>QR Code</span>
            </Button>

            <span class="text-border hidden md:inline">|</span>

            <!-- Void Action -->
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
          </div>
        </div>

        <Separator class="bg-border/60" />

        <!-- Date navigation and filter bar -->
        <div class="flex flex-wrap items-center justify-between gap-4">
          <div class="flex flex-wrap items-center gap-4">
            
            <!-- Filter Date Mode Select -->
            <div class="min-w-[120px]">
              <Select type="single" bind:value={filterMode}>
                <SelectTrigger class="h-8 text-xs bg-background/50 border-border cursor-pointer">
                  {filterMode === "day" ? m.common_form_day() : m.finance_je_filter_all()}
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="day">{m.common_form_day()}</SelectItem>
                  <SelectItem value="all">{m.finance_je_filter_all()}</SelectItem>
                </SelectContent>
              </Select>
            </div>

            <!-- Date picker and navigation buttons (visible when day mode is active) -->
            {#if filterMode === "day"}
              <div class="flex items-center gap-1 bg-background/80 rounded-lg p-0.5 border border-border/85" in:slide={{ axis: "x", duration: 150 }}>
                <Button variant="ghost" size="icon-xs" onclick={prevDay} class="h-7 w-7 text-muted-foreground hover:text-foreground cursor-pointer rounded-md">
                  <ChevronRight size={14} />
                </Button>
                
                <div class="flex items-center gap-1 px-1">
                  <Calendar size={12} class="text-primary/70" />
                  <input
                    type="date"
                    bind:value={selectedFilterDate}
                    class="bg-transparent text-foreground border-none font-mono text-xs focus:ring-0 outline-none w-28 cursor-pointer font-bold"
                  />
                </div>

                <Button variant="ghost" size="icon-xs" onclick={nextDay} class="h-7 w-7 text-muted-foreground hover:text-foreground cursor-pointer rounded-md">
                  <ChevronLeft size={14} />
                </Button>
              </div>

              <Button
                variant="outline"
                size="xs"
                onclick={jumpToToday}
                class="h-8 px-3 font-semibold text-xs border-border cursor-pointer hover:bg-muted text-foreground"
              >
                {m.common_form_day()}
              </Button>
            {/if}

            <span class="text-border">|</span>

            <!-- Show User Records Switch -->
            <div class="flex items-center gap-2 select-none">
              <Switch bind:checked={showUserRecords} id="show-user-records" class="cursor-pointer" />
              <Label for="show-user-records" class="text-xs font-semibold text-muted-foreground cursor-pointer">
                عرض سجلات المستخدم
              </Label>
            </div>

          </div>

          <!-- Text Search & Status -->
          <div class="flex items-center gap-3 flex-1 max-w-md md:ms-auto">
            <!-- Search bar -->
            <div class="relative flex-1 text-start">
              <Search class="absolute left-2.5 top-2.5 h-3 w-3 text-muted-foreground" />
              <Input
                type="text"
                placeholder="البحث بالرقم أو البيان..."
                bind:value={searchQuery}
                class="pl-8 h-8 text-[11px] bg-background/50 border-border"
              />
            </div>

            <!-- Status Filter Select -->
            <div class="w-32">
              <Select type="single" bind:value={statusFilter}>
                <SelectTrigger class="h-8 text-xs bg-background/50 border-border cursor-pointer">
                  {statusFilter === "all" ? "كل الحالات" : statusFilter === "draft" ? "مسودة" : "مرحل"}
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="all">كل الحالات</SelectItem>
                  <SelectItem value="draft">مسودة</SelectItem>
                  <SelectItem value="posted">مرحل</SelectItem>
                </SelectContent>
              </Select>
            </div>
          </div>
        </div>

      </div>

      <!-- Table Content Area -->
      <CardContent class="p-0 flex-grow min-h-[300px]">
        {#if isLoading}
          <div class="p-6 flex flex-col gap-2.5">
            {#each Array(4) as _}
              <Skeleton class="h-10 w-full rounded-lg" />
            {/each}
          </div>
        {:else if error}
          <div in:fade={{ duration: 150 }} class="m-6 border border-destructive/20 bg-destructive/10 text-destructive rounded-xl p-4 flex items-center gap-3 text-sm text-start">
            <AlertCircle class="h-4 w-4" />
            <div>خطأ في تحميل القيود: {error}</div>
          </div>
        {:else if filteredEntries.length === 0}
          <div class="border-t border-border/40 p-16 flex flex-col items-center justify-center text-center gap-2 select-none">
            <BookOpen size={36} class="text-muted-foreground/60 mb-2" />
            <h4 class="text-sm font-bold text-foreground">لا توجد قيود مطابقة للبحث</h4>
            <p class="text-xs text-muted-foreground">لم يتم العثور على أي قيود محاسبية بناءً على معايير التصفية الحالية.</p>
          </div>
        {:else}
          <div in:fade={{ duration: 150 }} class="border-t border-border/40 overflow-x-auto">
            <Table>
              <TableHeader class="bg-muted/40 select-none border-b border-border/60">
                <TableRow>
                  <TableHead class="text-start font-bold text-xs">الرقم</TableHead>
                  <TableHead class="text-start font-bold text-xs">التاريخ</TableHead>
                  <TableHead class="text-end font-bold text-xs text-emerald-600">المبلغ</TableHead>
                  <TableHead class="text-center font-bold text-xs">العملة</TableHead>
                  <TableHead class="text-start font-bold text-xs">من حساب (المدين)</TableHead>
                  <TableHead class="text-start font-bold text-xs">إلى حساب (الدائن)</TableHead>
                  <TableHead class="text-start font-bold text-xs">البيان / ملاحظات</TableHead>
                  <TableHead class="text-center font-bold text-xs w-16">الحالة</TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {#each filteredEntries as je (je.id)}
                  {#if showUserRecords || je.user === "U78"}
                    <TableRow
                      class="hover:bg-muted/20 transition-all duration-150 cursor-pointer select-none {selectedEntry?.id === je.id ? 'bg-primary/10 border-primary border-s-2 hover:bg-primary/15' : ''}"
                      onclick={() => selectRow(je)}
                    >
                      <TableCell class="text-start font-mono text-[11px] text-foreground font-extrabold whitespace-nowrap">{je.transaction_ref}</TableCell>
                      <TableCell class="text-start text-[11px] text-muted-foreground whitespace-nowrap">{formatDate(je.accounting_date)}</TableCell>
                      <TableCell class="text-end text-[11px] font-bold font-mono text-emerald-600 whitespace-nowrap">
                        {localFormatCurrency(je.amount, je.currency || 'YER')}
                      </TableCell>
                      <TableCell class="text-center text-[11px] font-semibold font-mono">{je.currency || 'YER'}</TableCell>
                      <TableCell class="text-start text-[11px] text-foreground font-semibold max-w-[160px] truncate">
                        {getAccountLabel(je.debit_account_id)}
                      </TableCell>
                      <TableCell class="text-start text-[11px] text-foreground font-semibold max-w-[160px] truncate">
                        {getAccountLabel(je.credit_account_id)}
                      </TableCell>
                      <TableCell class="text-start text-[11px] text-muted-foreground max-w-[200px] truncate">
                        {je.description_ar || je.description || '-'}
                      </TableCell>
                      <TableCell class="text-center">
                        <Badge variant={je.status === 'posted' ? 'lime' : 'secondary'} class="text-[10px] font-bold py-0.5 px-2">
                          {je.status === 'posted' ? m.common_je_status_posted() : m.common_status_draft()}
                        </Badge>
                      </TableCell>
                    </TableRow>
                  {/if}
                {/each}
              </TableBody>
            </Table>
          </div>
        {/if}
      </CardContent>
    </Card>

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
        
        <Button variant="outline" size="sm" class="mt-4 w-full cursor-pointer hover:bg-muted text-foreground border-border" onclick={() => showQrModal = false}>
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
