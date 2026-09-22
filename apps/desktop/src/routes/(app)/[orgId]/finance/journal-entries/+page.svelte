<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import { api, extractErrorMessage } from "$lib/api";
import { formatCurrency, formatDate, l, isAr } from "$lib/core/i18n";
import type { JournalEntryDto, AccountDto, CurrencyDto } from "$lib/api/types";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Badge } from "@orbis/ui/badge";
import { Card, CardContent } from "@orbis/ui/card";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui/select";
import {
  Grid,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@orbis/ui/table";
import {
  Sheet,
  SheetContent,
  SheetHeader,
  SheetTitle,
  SheetDescription,
  SheetFooter,
} from "@orbis/ui/sheet";
import { toast } from "@orbis/ui";
import { AdjustmentsHorizontal, Calendar, CheckCircle, DocumentChartBar, Download, Eye, Grid, InfoCircle, Plus, Search, Trash } from "@orbis/icons";
import CreateJournalDialog from "$lib/features/finance/components/create-journal-dialog.svelte";
import { exportToCsv } from "$lib/shared/utils/export";

let { data }: { data: { orgId: string } } = $props();

let journalEntries = $state<JournalEntryDto[]>([]);
let accounts = $state<AccountDto[]>([]);
let currencies = $state<{ code: string; name: string }[]>([]);
let isLoading = $state(true);

let searchQuery = $state("");
let statusFilter = $state("all");
let dateFilter = $state("");

let isCreateOpen = $state(false);
let selectedEntry = $state<JournalEntryDto | null>(null);
let isDetailsOpen = $state(false);

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
    currencies = (currenciesList || []).map((c) => ({
      code: c.code,
      name: isAr()
        ? c.name_ar || c.name || c.code
        : c.name || c.name_ar || c.code,
    }));
  } catch (e) {
    console.error("Failed to load journal data:", e);
    toast.error("فشل تحميل قيود اليومية أو شجرة الحسابات");
  } finally {
    isLoading = false;
  }
}

async function handleCreateEntry(entryData: any) {
  // In a full implementation, this posts lines to backend accounting repository
  // We reload data to sync with latest state
  await loadData();
}

function openEntryDetails(entry: JournalEntryDto) {
  selectedEntry = entry;
  isDetailsOpen = true;
}

function handleExportCsv() {
  exportToCsv(
    `Journal_Entries_${data.orgId}`,
    filteredEntries,
    [
      { header: "رقم السند", accessor: (e) => e.reference_number },
      { header: "تاريخ القيد", accessor: (e) => formatDate(e.accounting_date, "date") },
      { header: "البيان", accessor: (e) => e.description_ar || e.description },
      { header: "المبلغ", accessor: (e) => Number(e.amount || 0) },
      { header: "العملة", accessor: (e) => e.currency },
      { header: "الحالة", accessor: (e) => e.status },
    ],
  );
}

// Helpers
function getAccountName(accId: string): string {
  const acc = accounts.find((a) => a.id === accId);
  return acc
    ? `${acc.code} - ${l(acc.name_ar, acc.name) || acc.name}`
    : accId || "—";
}

// Computations
const totalEntriesCount = $derived(journalEntries.length);
const totalDebitVolume = $derived(
  journalEntries.reduce((sum, e) => sum + Number(e.amount || 0), 0),
);

const filteredEntries = $derived(
  journalEntries.filter((je) => {
    const q = searchQuery.toLowerCase().trim();
    const matchesSearch =
      !q ||
      je.reference_number?.toLowerCase().includes(q) ||
      je.description?.toLowerCase().includes(q) ||
      je.description_ar?.toLowerCase().includes(q);

    const matchesStatus =
      statusFilter === "all" || je.status === statusFilter;

    const matchesDate =
      !dateFilter ||
      je.accounting_date?.slice(0, 10) === dateFilter;

    return matchesSearch && matchesStatus && matchesDate;
  }),
);
</script>

<svelte:head>
  <title>قيود اليومية المحاسبية | Orbis</title>
</svelte:head>

<div class="p-6 max-w-7xl mx-auto space-y-6 text-start">
  <!-- Hero Card -->
  <div class="relative overflow-hidden rounded-2xl border border-border/80 bg-linear-to-b from-card to-card/60 p-6 shadow-sm backdrop-blur-md">
    <div class="flex flex-col md:flex-row md:items-center justify-between gap-4">
      <div class="flex items-center gap-3.5">
        <div class="w-12 h-12 rounded-2xl bg-primary/10 border border-primary/20 flex items-center justify-center text-primary shadow-xs">
          <Scale class="w-6 h-6" />
        </div>
        <div>
          <h1 class="text-xl font-bold text-foreground tracking-tight">
            دفتر قيود اليومية العامة (Journal Entries)
          </h1>
          <p class="text-xs text-muted-foreground mt-0.5">
            تسجيل وترحيل قيود المحاسبة المزدوجة، التسويات المالية، ومتابعة الأرصدة الافتتاحية
          </p>
        </div>
      </div>

      <!-- Action Buttons -->
      <div class="flex items-center gap-2">
        <Button
          variant="outline"
          size="sm"
          onclick={handleExportCsv}
          disabled={filteredEntries.length === 0}
          class="h-9 text-xs font-semibold gap-1.5"
        >
          <DocumentChartBar class="w-4 h-4" />
          تصدير CSV
        </Button>

        <Button
          variant="default"
          size="sm"
          onclick={() => (isCreateOpen = true)}
          class="h-9 text-xs font-bold gap-1.5 shadow-sm"
        >
          <Plus class="w-4 h-4" />
          إنشاء قيد محاسبي جديد
        </Button>
      </div>
    </div>

    <!-- Quick Stats -->
    <div class="grid grid-cols-2 sm:grid-cols-3 gap-3 mt-6 pt-4 border-t border-border/60">
      <div class="flex items-center gap-3">
        <div class="w-8 h-8 rounded-lg bg-primary/10 flex items-center justify-center text-primary">
          <Scale class="w-4 h-4" />
        </div>
        <div>
          <div class="text-lg font-bold text-foreground font-mono leading-none">{totalEntriesCount}</div>
          <div class="text-[11px] text-muted-foreground mt-0.5">إجمالي سندات القيود</div>
        </div>
      </div>

      <div class="flex items-center gap-3">
        <div class="w-8 h-8 rounded-lg bg-emerald-500/10 flex items-center justify-center text-emerald-600 dark:text-emerald-400">
          <CheckCircle class="w-4 h-4" />
        </div>
        <div>
          <div class="text-lg font-bold text-foreground font-mono leading-none">
            {formatCurrency(totalDebitVolume)}
          </div>
          <div class="text-[11px] text-muted-foreground mt-0.5">إجمالي حركة القيود المرحلة</div>
        </div>
      </div>
    </div>
  </div>

  <!-- Filters & Grid Card -->
  <Card class="border border-border/80 shadow-xs">
    <div class="p-4 border-b border-border/60 flex flex-col sm:flex-row items-start sm:items-center justify-between gap-3">
      <div class="flex flex-wrap items-center gap-2 w-full sm:w-auto">
        <div class="relative w-full sm:w-64">
          <Search class="absolute start-2.5 top-1/2 -translate-y-1/2 w-3.5 h-3.5 text-muted-foreground" />
          <Input
            type="search"
            placeholder="بحث برقم السند أو البيان..."
            bind:value={searchQuery}
            class="h-8 ps-8 text-xs w-full"
          />
        </div>

        <Input
          type="date"
          bind:value={dateFilter}
          class="h-8 text-xs w-36"
        />

        <Select value={statusFilter} onValueChange={(v) => (statusFilter = v)}>
          <SelectTrigger class="h-8 text-xs w-32">
            <span>{statusFilter === "all" ? "جميع الحالات" : statusFilter === "posted" ? "مرحل" : "مسودة"}</span>
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="all">جميع الحالات</SelectItem>
            <SelectItem value="posted">مرحل</SelectItem>
            <SelectItem value="draft">مسودة</SelectItem>
          </SelectContent>
        </Select>
      </div>
    </div>

    <!-- Data Grid -->
    <CardContent class="p-0">
      <Grid>
        <TableHeader class="bg-muted/40">
          <TableRow>
            <TableHead class="text-xs font-bold w-12 text-center">#</TableHead>
            <TableHead class="text-xs font-bold w-36">رقم السند المرجعي</TableHead>
            <TableHead class="text-xs font-bold w-28">تاريخ القيد</TableHead>
            <TableHead class="text-xs font-bold">البيان والشرح</TableHead>
            <TableHead class="text-xs font-bold w-36 text-end">المبلغ الإجمالي</TableHead>
            <TableHead class="text-xs font-bold w-24 text-center">الحالة</TableHead>
            <TableHead class="w-16 text-end"></TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          {#if isLoading}
            <TableRow>
              <TableCell colspan={7} class="text-center py-12 text-xs text-muted-foreground">
                جاري تحميل قيود اليومية...
              </TableCell>
            </TableRow>
          {:else if filteredEntries.length === 0}
            <TableRow>
              <TableCell colspan={7} class="text-center py-12 text-xs text-muted-foreground">
                لا توجد قيود يومية مسجلة حالياً
              </TableCell>
            </TableRow>
          {:else}
            {#each filteredEntries as je, idx}
              <TableRow class="hover:bg-muted/20 transition-colors">
                <TableCell class="text-center text-xs font-mono text-muted-foreground">
                  {idx + 1}
                </TableCell>
                <TableCell class="font-mono text-xs font-bold text-foreground">
                  {je.reference_number}
                </TableCell>
                <TableCell class="text-xs text-muted-foreground whitespace-nowrap">
                  {formatDate(je.accounting_date, "date")}
                </TableCell>
                <TableCell class="text-xs font-medium text-foreground">
                  {je.description_ar || je.description || "قيد يومية عام"}
                </TableCell>
                <TableCell class="text-end font-mono text-xs font-bold text-foreground">
                  {formatCurrency(Number(je.amount || 0), je.currency)}
                </TableCell>
                <TableCell class="text-center">
                  <Badge variant={je.status === "posted" ? "secondary" : "outline"} class="text-[10px]">
                    {je.status === "posted" ? "مرحل" : je.status}
                  </Badge>
                </TableCell>
                <TableCell class="text-end">
                  <Button
                    variant="ghost"
                    size="sm"
                    onclick={() => openEntryDetails(je)}
                    class="h-7 w-7 p-0 text-muted-foreground hover:text-foreground"
                  >
                    <Eye class="w-3.5 h-3.5" />
                  </Button>
                </TableCell>
              </TableRow>
            {/each}
          {/if}
        </TableBody>
      </Grid>
    </CardContent>
  </Card>
</div>

<!-- Create Journal Dialog -->
<CreateJournalDialog
  bind:open={isCreateOpen}
  {accounts}
  {currencies}
  onsubmit={handleCreateEntry}
/>

<!-- Details Drawer -->
{#if selectedEntry}
  <Sheet bind:open={isDetailsOpen}>
    <SheetContent side="right" class="w-full sm:max-w-lg flex flex-col justify-between p-6 text-start">
      <div class="space-y-6 overflow-y-auto pr-1">
        <SheetHeader class="space-y-1">
          <div class="flex items-center gap-2">
            <div class="w-8 h-8 rounded-lg bg-primary/10 flex items-center justify-center text-primary">
              <Scale class="w-4 h-4" />
            </div>
            <div>
              <SheetTitle class="text-base font-bold text-foreground">
                تفاصيل القيد: {selectedEntry.reference_number}
              </SheetTitle>
              <SheetDescription class="text-xs text-muted-foreground">
                بتاريخ {formatDate(selectedEntry.accounting_date, "date")}
              </SheetDescription>
            </div>
          </div>
        </SheetHeader>

        <!-- Entry Summary Meta -->
        <div class="p-4 rounded-xl bg-muted/40 border border-border/60 space-y-2 text-xs">
          <div class="flex justify-between">
            <span class="text-muted-foreground">البيان:</span>
            <span class="font-bold text-foreground">{selectedEntry.description_ar || selectedEntry.description}</span>
          </div>
          <div class="flex justify-between">
            <span class="text-muted-foreground">إجمالي القيد:</span>
            <span class="font-mono font-bold text-foreground">{formatCurrency(Number(selectedEntry.amount || 0), selectedEntry.currency)}</span>
          </div>
          <div class="flex justify-between">
            <span class="text-muted-foreground">الحالة:</span>
            <Badge variant="secondary" class="text-[10px]">{selectedEntry.status}</Badge>
          </div>
        </div>

        <!-- Lines Details -->
        {#if selectedEntry.lines && selectedEntry.lines.length > 0}
          <div class="space-y-2">
            <div class="text-xs font-bold text-foreground">أسطر القيد المحاسبي:</div>
            <div class="rounded-xl border border-border/70 divide-y overflow-hidden">
              {#each selectedEntry.lines as line}
                <div class="p-3 text-xs flex justify-between items-center bg-card">
                  <div>
                    <div class="font-bold text-foreground">{getAccountName(line.account_id)}</div>
                    {#if line.description}
                      <div class="text-[10px] text-muted-foreground">{line.description}</div>
                    {/if}
                  </div>
                  <div class="text-end font-mono">
                    {#if Number(line.debit_amount) > 0}
                      <div class="text-emerald-600 font-bold">مدين: {formatCurrency(Number(line.debit_amount), selectedEntry.currency)}</div>
                    {/if}
                    {#if Number(line.credit_amount) > 0}
                      <div class="text-blue-600 font-bold">دائن: {formatCurrency(Number(line.credit_amount), selectedEntry.currency)}</div>
                    {/if}
                  </div>
                </div>
              {/each}
            </div>
          </div>
        {/if}
      </div>

      <SheetFooter class="pt-4 border-t mt-4 flex justify-end">
        <Button variant="outline" size="sm" onclick={() => (isDetailsOpen = false)}>
          إغلاق
        </Button>
      </SheetFooter>
    </SheetContent>
  </Sheet>
{/if}
