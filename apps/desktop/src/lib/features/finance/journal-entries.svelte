<!--
  @component JournalEntries
  Minimal, elegant, and high-performance Journal Entries workspace.
  Adheres strictly to Orbis Clean Architecture and the FinanceOverview design system:
  - Clean card-based entry form with direct account selection
  - Sleek top toolbar with search, date filter, and export
  - High-performance OrbisGrid table for vouchers
  - Real backend double-entry posting and reversals
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import { api } from "$lib/api";
import { formatCurrency, formatDate, l, isAr } from "$lib/core/i18n";
import type { JournalEntryDto, AccountDto, CurrencyDto } from "$lib/api/types";
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
  Printer,
} from "@orbis/icons";
import { OrbisGrid, type OrbisColumnDef } from "@orbis/svelte-grid";
import AccountSearchInput from "$lib/features/finance/components/account-search-input.svelte";
import SearchableSelect from "$lib/shared/components/searchable-select.svelte";
import PdfReportViewer from "$lib/shared/components/pdf-report-viewer.svelte";
import { confirmStore } from "$lib/stores";
import { fade, slide } from "svelte/transition";

let { orgId }: { orgId: string } = $props();

// ── State ──────────────────────────────────────────────────────────────
let journalEntries = $state<JournalEntryDto[]>([]);
let accounts = $state<AccountDto[]>([]);
let currencies = $state<CurrencyDto[]>([]);
let isLoading = $state(true);
let isSubmitting = $state(false);
let showAddForm = $state(false);

// Form Fields
let referenceNumber = $state("");
let accountingDate = $state(new Date().toISOString().slice(0, 10));
let amount = $state<number | null>(null);
let selectedCurrency = $state("YER");
let debitAccountId = $state("");
let creditAccountId = $state("");
let notes = $state("");

// Filters
let searchQuery = $state("");
let filterDate = $state("");

// Print
let printReportOpen = $state(false);
let printParameters = $state<Record<string, any>>({});
let printTitle = $state("طباعة سند القيد");

onMount(async () => {
  await loadData();
});

async function loadData() {
  isLoading = true;
  try {
    const [entriesList, accountsList, currenciesList] = await Promise.all([
      api.listJournalEntries().catch(() => []),
      api.listAccounts().catch(() => []),
      api.listCurrencies().catch(() => []),
    ]);

    journalEntries = entriesList || [];
    accounts = accountsList || [];
    currencies = currenciesList || [];

    const base = currenciesList?.find((c) => c.is_base);
    if (base && !selectedCurrency) {
      selectedCurrency = base.code;
    }
  } catch (e) {
    console.error("Failed to load journal entries master data:", e);
    toast.error("فشل تحميل بيانات قيود اليومية");
  } finally {
    isLoading = false;
  }
}

function clearForm() {
  referenceNumber = "";
  accountingDate = new Date().toISOString().slice(0, 10);
  amount = null;
  debitAccountId = "";
  creditAccountId = "";
  notes = "";
}

function getAccountName(id?: string | null): string {
  if (!id) return "—";
  const acc = accounts.find((a) => a.id === id);
  if (!acc) return id.slice(0, 8);
  return acc.name_ar || acc.name || acc.code || "—";
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
    toast.error("يرجى تحديد حساب الدائن (إلى حساب)");
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
          description: notes.trim() || "إلى حساب",
        },
      ],
    };

    await api.createJournalEntry(payload as any);
    toast.success("تم حفظ وترحيل سند القيد بنجاح");
    clearForm();
    showAddForm = false;
    await loadData();
  } catch (err: any) {
    console.error("Failed to post journal entry:", err);
    toast.error(err?.message || "فشل ترحيل سند القيد");
  } finally {
    isSubmitting = false;
  }
}

async function handleReverse(entry: JournalEntryDto) {
  const confirmed = await confirmStore.show({
    title: "عكس سند القيد",
    message: `هل أنت متأكد من إنشاء قيد عكسي للسند رقم "${entry.reference_number || entry.id.slice(0, 8)}"؟`,
    confirmText: "عكس القيد",
    cancelText: "إلغاء",
  });

  if (!confirmed) return;

  try {
    await api.reverseJournalEntry(entry.id);
    toast.success("تم إنشاء القيد العكسي بنجاح");
    await loadData();
  } catch (err: any) {
    console.error("Failed to reverse journal entry:", err);
    toast.error(err?.message || "فشل عكس سند القيد");
  }
}

function handlePrint(entry: JournalEntryDto) {
  printParameters = {
    voucher_id: entry.id,
    reference_number: entry.reference_number,
    org_id: orgId,
  };
  printTitle = `سند قيد - ${entry.reference_number || entry.id.slice(0, 8)}`;
  printReportOpen = true;
}

// ── Grid Data ─────────────────────────────────────────────────────────
const gridSource = $derived(
  journalEntries.map((e, idx) => {
    const debitAcc = e.debit_account_id
      ? getAccountName(e.debit_account_id)
      : (e.lines && e.lines.length > 0 ? getAccountName(e.lines[0].account_id) : "—");

    const creditAcc = e.credit_account_id
      ? getAccountName(e.credit_account_id)
      : (e.lines && e.lines.length > 1 ? getAccountName(e.lines[1].account_id) : "—");

    const amt = Number(e.amount || (e.lines && e.lines.length > 0 ? e.lines[0].debit || e.lines[0].credit : 0));
    const curr = e.currency_code || e.currency || "YER";

    return {
      ...e,
      __idx: idx + 1,
      ref_display: e.reference_number || e.id.slice(0, 8),
      date_display: e.accounting_date ? formatDate(e.accounting_date) : "—",
      amount_formatted: formatCurrency(amt, curr),
      currency_display: curr,
      debit_account_display: debitAcc,
      credit_account_display: creditAcc,
      notes_display: e.description_ar || e.description || "—",
      status_display: e.is_posted ? "مرحل" : "مسودة",
    };
  })
);

const filteredSource = $derived(
  gridSource.filter((row) => {
    if (filterDate && row.accounting_date && !String(row.accounting_date).startsWith(filterDate)) {
      return false;
    }
    return true;
  })
);

const columns: OrbisColumnDef[] = [
  { id: "__idx", prop: "__idx", header: "#", name: "#", width: 48, size: 48, readonly: true, pin: "left", align: "center" },
  { id: "ref_display", prop: "ref_display", header: "رقم السند", name: "رقم السند", width: 110, size: 110, readonly: true, cellClass: "font-mono font-bold" },
  { id: "date_display", prop: "date_display", header: "التاريخ", name: "التاريخ", width: 110, size: 110, readonly: true, align: "center" },
  { id: "debit_account_display", prop: "debit_account_display", header: "من حساب (مدين)", name: "من حساب (مدين)", width: 220, size: 220, readonly: true, cellClass: "font-medium" },
  { id: "credit_account_display", prop: "credit_account_display", header: "إلى حساب (دائن)", name: "إلى حساب (دائن)", width: 220, size: 220, readonly: true, cellClass: "font-medium" },
  { id: "amount_formatted", prop: "amount_formatted", header: "المبلغ", name: "المبلغ", width: 140, size: 140, readonly: true, align: "end", cellClass: "font-mono font-bold text-primary" },
  { id: "currency_display", prop: "currency_display", header: "العملة", name: "العملة", width: 80, size: 80, readonly: true, align: "center" },
  { id: "notes_display", prop: "notes_display", header: "البيان / ملاحظات", name: "البيان / ملاحظات", width: 240, size: 240, readonly: true },
  { id: "status_display", prop: "status_display", header: "الحالة", name: "الحالة", width: 90, size: 90, type: "badge", align: "center", readonly: true },
];

function exportCsv() {
  const rows = filteredSource.map((e) => [
    e.ref_display,
    e.date_display,
    e.debit_account_display,
    e.credit_account_display,
    e.amount_formatted,
    e.currency_display,
    e.notes_display,
    e.status_display,
  ]);
  const csv = [
    ["Reference", "Date", "Debit Account", "Credit Account", "Amount", "Currency", "Notes", "Status"],
    ...rows,
  ]
    .map((row) => row.map((v) => `"${String(v ?? "").replaceAll('"', '""')}"`).join(","))
    .join("\n");
  const url = URL.createObjectURL(new Blob(["\ufeff" + csv], { type: "text/csv;charset=utf-8" }));
  const link = document.createElement("a");
  link.href = url;
  link.download = `journal-entries-${orgId}.csv`;
  link.click();
  URL.revokeObjectURL(url);
}
</script>

<div class="flex h-full min-h-0 w-full flex-col overflow-hidden p-6 font-sans bg-background/50">
  <div class="flex min-h-0 w-full flex-1 flex-col gap-4" in:fade={{ duration: 150 }}>
    
    <!-- Top Action Bar -->
    <header class="flex shrink-0 items-center justify-between gap-4">
      <div class="flex items-center gap-2">
        <Button
          variant={showAddForm ? "secondary" : "default"}
          size="sm"
          class="h-8 px-3 text-xs font-bold shrink-0 gap-1.5"
          onclick={() => { showAddForm = !showAddForm; }}
        >
          {#if showAddForm}
            <XMark size={14} />
            <span>إلغاء</span>
          {:else}
            <Plus size={14} />
            <span>سند قيد جديد</span>
          {/if}
        </Button>
      </div>

      <div class="flex items-center gap-2">
        <div class="relative w-48 sm:w-64">
          <Search size={14} class="absolute inset-s-2.5 top-1/2 -translate-y-1/2 text-muted-foreground pointer-events-none" />
          <Input
            type="text"
            placeholder="بحث بالرقم أو الحساب أو البيان..."
            bind:value={searchQuery}
            class="h-8 ps-8 text-xs bg-card border-border/60"
          />
        </div>

        <Input
          type="date"
          bind:value={filterDate}
          class="h-8 w-36 text-xs bg-card border-border/60 font-mono"
        />

        <Button variant="ghost" size="icon-sm" onclick={loadData} class="text-muted-foreground">
          <Refresh size={14} class={isLoading ? "animate-spin" : ""} />
        </Button>

        <Button variant="ghost" size="icon-sm" onclick={exportCsv} class="text-muted-foreground">
          <Download size={14} />
        </Button>
      </div>
    </header>

    <!-- In-Page Creation Card (Matching FinanceOverview Card Style) -->
    {#if showAddForm}
      <div transition:slide={{ duration: 200 }} class="shrink-0">
        <Card class="border border-primary/20 bg-card shadow-sm text-start">
          <CardHeader class="py-3 px-4 border-b border-border/40">
            <div class="flex items-center justify-between">
              <div class="flex items-center gap-2">
                <span class="h-2 w-2 rounded-full bg-primary"></span>
                <CardTitle class="text-xs font-bold uppercase tracking-wider text-foreground">
                  إنشاء سند قيد محاسبي
                </CardTitle>
              </div>
              <Button variant="ghost" size="icon-xs" onclick={() => showAddForm = false} class="text-muted-foreground">
                <XMark size={14} />
              </Button>
            </div>
          </CardHeader>

          <CardContent class="p-4 flex flex-col gap-4">
            <div class="grid grid-cols-1 md:grid-cols-4 gap-4">
              <div>
                <Label class="text-xs font-semibold text-foreground mb-1 block">رقم السند (اختياري)</Label>
                <Input
                  type="text"
                  bind:value={referenceNumber}
                  placeholder="تلقائي"
                  class="h-8 text-xs font-mono bg-background border-border/60"
                />
              </div>

              <div>
                <Label class="text-xs font-semibold text-foreground mb-1 block">التاريخ</Label>
                <Input
                  type="date"
                  bind:value={accountingDate}
                  class="h-8 text-xs font-mono bg-background border-border/60"
                />
              </div>

              <div>
                <Label class="text-xs font-semibold text-foreground mb-1 block">المبلغ</Label>
                <Input
                  type="number"
                  bind:value={amount}
                  placeholder="0.00"
                  class="h-8 text-xs font-mono font-bold text-primary bg-background border-border/60"
                />
              </div>

              <div>
                <Label class="text-xs font-semibold text-foreground mb-1 block">العملة</Label>
                <SearchableSelect
                  value={selectedCurrency}
                  placeholder="اختر العملة..."
                  items={currencies}
                  getItemValue={(c) => c.code}
                  getItemLabel={(c) => `${c.name || c.code} (${c.code})`}
                  onchange={(val) => { if (val) selectedCurrency = val; }}
                  class="w-full h-8 text-xs bg-background border-border/60"
                />
              </div>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div>
                <Label class="text-xs font-semibold text-foreground mb-1 block">من حساب (مدين)</Label>
                <AccountSearchInput
                  bind:value={debitAccountId}
                  {accounts}
                  placeholder="اختر حساب المدين..."
                />
              </div>

              <div>
                <Label class="text-xs font-semibold text-foreground mb-1 block">إلى حساب (دائن)</Label>
                <AccountSearchInput
                  bind:value={creditAccountId}
                  {accounts}
                  placeholder="اختر حساب الدائن..."
                />
              </div>
            </div>

            <div>
              <Label class="text-xs font-semibold text-foreground mb-1 block">البيان / ملاحظات</Label>
              <Input
                type="text"
                bind:value={notes}
                placeholder="أدخل تفاصيل وبيان السند..."
                class="h-8 text-xs bg-background border-border/60"
              />
            </div>

            <div class="flex items-center justify-end gap-2 pt-2 border-t border-border/40">
              <Button variant="ghost" size="sm" class="h-8 text-xs" onclick={clearForm}>
                مسح الحقول
              </Button>
              <Button
                variant="default"
                size="sm"
                class="h-8 text-xs font-bold px-4"
                disabled={isSubmitting}
                onclick={handleSaveNew}
              >
                {isSubmitting ? "جاري الحفظ..." : "حفظ وترحيل القيد"}
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
        emptyTitle="لا توجد قيود يومية مسجلة"
      />
    </div>
  </div>
</div>

<PdfReportViewer
  bind:open={printReportOpen}
  reportTemplateId="journal-voucher"
  parameters={printParameters}
  title={printTitle}
/>
