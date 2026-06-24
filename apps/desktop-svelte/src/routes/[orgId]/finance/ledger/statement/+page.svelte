<!--
  @page AccountStatementPage
  Displays a professional, double-entry ledger statement for any selected account
  over an arbitrary date range, calculating running balances in real-time.
-->
<script lang="ts">
import { onMount } from "svelte";
import { page } from "$app/state";
import { commands } from "$lib/services/ipc";
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
import {
  ClipboardList,
  Calendar,
  Search,
  ArrowLeftRight,
  TrendingUp,
  ArrowUpRight,
  ArrowDownLeft,
  Printer,
  FileSpreadsheet,
} from "lucide-svelte";
import { getLocale } from "$lib/paraglide/runtime";
import { formatCurrency, formatDate } from "$lib/i18n-utils";
import { fade, slide } from "svelte/transition";
import * as m from "$lib/paraglide/messages";

let { data }: { data: { orgId: string } } = $props();

// State
let ledgers = $state<any[]>([]);
let selectedAccountId = $state<string>("");
let startDate = $state<string>("");
let endDate = $state<string>("");
let statement = $state<any | null>(null);

let isLoadingLedgers = $state(true);
let isLoadingStatement = $state(false);
let error = $state<string | null>(null);
let searchQuery = $state("");

// Date Defaults: past 30 days
onMount(async () => {
  const today = new Date();
  const thirtyDaysAgo = new Date();
  thirtyDaysAgo.setDate(today.getDate() - 30);

  startDate = thirtyDaysAgo.toISOString().split("T")[0];
  endDate = today.toISOString().split("T")[0];

  // Get accountId from query parameters if present
  const qAccountId = page.url.searchParams.get("accountId");
  if (qAccountId) {
    selectedAccountId = qAccountId;
  }

  await loadLedgers();
  if (selectedAccountId) {
    await fetchStatement();
  }
});

async function loadLedgers() {
  isLoadingLedgers = true;
  try {
    ledgers = await commands.finance.getLedgerAccounts(data.orgId);
    // Default to first account if none selected
    if (!selectedAccountId && ledgers.length > 0) {
      selectedAccountId = ledgers[0].id;
    }
  } catch (e) {
    console.error("Error loading ledger accounts:", e);
    error = e instanceof Error ? e.message : String(e);
  } finally {
    isLoadingLedgers = false;
  }
}

async function fetchStatement() {
  if (!selectedAccountId || !startDate || !endDate) return;
  isLoadingStatement = true;
  error = null;
  try {
    statement = await commands.finance.getAccountStatement(
      data.orgId,
      selectedAccountId,
      startDate,
      endDate
    );
  } catch (e) {
    console.error("Error fetching account statement:", e);
    error = e instanceof Error ? e.message : String(e);
    statement = null;
  } finally {
    isLoadingStatement = false;
  }
}

// Calculate totals from statement items
const totals = $derived.by(() => {
  if (!statement || !statement.items) return { debit: 0, credit: 0 };
  let debit = 0;
  let credit = 0;
  for (const item of statement.items) {
    if (item.debit) debit += item.debit;
    if (item.credit) credit += item.credit;
  }
  return { debit, credit };
});

// Search filter on items
const filteredItems = $derived.by(() => {
  if (!statement || !statement.items) return [];
  if (searchQuery.trim() === "") return statement.items;
  return statement.items.filter((item: any) =>
    String(item.reference || "").toLowerCase().includes(searchQuery.toLowerCase()) ||
    String(item.description || "").toLowerCase().includes(searchQuery.toLowerCase()) ||
    String(item.description_ar || "").toLowerCase().includes(searchQuery.toLowerCase())
  );
});

function printStatement() {
  window.print();
}
</script>

<div class="h-full flex flex-col overflow-hidden bg-background p-6">
  <!-- Header Section -->
  <div class="flex items-center justify-between pb-6 select-none border-b border-border print:hidden">
    <div class="flex flex-col gap-1 text-start">
      <h1 class="text-xl font-bold tracking-tight text-foreground flex items-center gap-2">
        <ClipboardList class="text-primary" size={20} />
        <span>{m.finance_layout_nav_ledger ? m.finance_layout_nav_ledger() : "كشف الحساب التفصيلي"}</span>
      </h1>
      <p class="text-xs text-muted-foreground">
        عرض كشف حساب تفصيلي للطلاب والموظفين والخزائن ومطابقة أرصدتهم المحاسبية بدقة
      </p>
    </div>

    <!-- Print / Export Buttons -->
    {#if statement}
      <div class="flex items-center gap-2">
        <Button variant="outline" size="sm" onclick={printStatement} class="h-9 text-xs flex items-center gap-1 cursor-pointer">
          <Printer size={13} />
          <span>طباعة</span>
        </Button>
      </div>
    {/if}
  </div>

  <!-- Filter & Selector Bar -->
  <div class="flex flex-wrap items-center justify-between gap-4 py-4 select-none border-b border-border/40 print:hidden">
    <div class="flex flex-wrap items-center gap-3 flex-1">
      <!-- Account Select -->
      <div class="flex flex-col gap-1 text-start min-w-[220px]">
        <label for="account-select" class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">الحساب الدفتري</label>
        {#if isLoadingLedgers}
          <Skeleton class="h-9 w-full rounded-md" />
        {:else}
          <select
            id="account-select"
            bind:value={selectedAccountId}
            onchange={fetchStatement}
            class="bg-background text-foreground border border-border rounded-md px-3 h-9 text-xs focus:ring-1 focus:ring-ring outline-none cursor-pointer w-full"
          >
            {#each ledgers as acc}
              <option value={acc.id}>
                {acc.code} - {acc.name_ar || acc.name} {acc.entity_type ? `(${acc.entity_type === 'student' ? 'طالب' : 'موظف'})` : ''}
              </option>
            {/each}
          </select>
        {/if}
      </div>

      <!-- Date Pickers -->
      <div class="flex items-center gap-2">
        <div class="flex flex-col gap-1 text-start">
          <label for="start-date" class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">من تاريخ</label>
          <div class="relative">
            <Input
              id="start-date"
              type="date"
              bind:value={startDate}
              onchange={fetchStatement}
              class="h-9 text-xs w-[140px]"
            />
          </div>
        </div>

        <div class="flex flex-col gap-1 text-start">
          <label for="end-date" class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">إلى تاريخ</label>
          <div class="relative">
            <Input
              id="end-date"
              type="date"
              bind:value={endDate}
              onchange={fetchStatement}
              class="h-9 text-xs w-[140px]"
            />
          </div>
        </div>
      </div>
    </div>

    <!-- Live Statement search -->
    {#if statement}
      <div class="relative w-full max-w-xs">
        <Search class="absolute left-2.5 top-2.5 h-4 w-4 text-muted-foreground" />
        <Input
          type="text"
          placeholder="البحث بالبيان أو الرقم المرجعي..."
          bind:value={searchQuery}
          class="pl-9 h-9 text-xs"
        />
      </div>
    {/if}
  </div>

  <!-- Print Title (Only visible in Print) -->
  <div class="hidden print:flex flex-col gap-2 text-center pb-8 border-b border-border select-none">
    <h1 class="text-2xl font-bold">كشف حساب تفصيلي</h1>
    {#if statement}
      <p class="text-sm font-semibold text-muted-foreground">
        الحساب: {statement.account_code} - {statement.account_name} | الفترة: {formatDate(statement.start_date)} إلى {formatDate(statement.end_date)}
      </p>
    {/if}
  </div>

  <!-- Content Section -->
  <div class="flex-1 overflow-y-auto py-4">
    {#if isLoadingStatement}
      <!-- Skeleton loader for statement -->
      <div class="grid grid-cols-1 md:grid-cols-4 gap-4 mb-6">
        {#each Array(4) as _}
          <Skeleton class="h-20 rounded-xl" />
        {/each}
      </div>
      <div class="flex flex-col gap-3">
        {#each Array(6) as _}
          <Skeleton class="h-12 w-full rounded-lg" />
        {/each}
      </div>
    {:else if error}
      <div in:fade={{ duration: 150 }} class="border border-destructive/20 bg-destructive/10 text-destructive rounded-xl p-4 flex items-center gap-3 text-sm text-start print:hidden">
        <div class="font-bold">خطأ في تحميل كشف الحساب:</div>
        <div>{error}</div>
      </div>
    {:else if !statement}
      <!-- Zero state -->
      <div class="border border-dashed border-border/80 rounded-2xl p-16 flex flex-col items-center justify-center text-center gap-3 select-none my-8">
        <div class="w-14 h-14 rounded-full bg-primary/10 flex items-center justify-center text-primary mb-2">
          <ClipboardList size={28} />
        </div>
        <h3 class="text-sm font-bold text-foreground">الرجاء اختيار الحساب والفترة الزمنية</h3>
        <p class="text-xs text-muted-foreground max-w-xs">
          حدد الحساب الدفتري الذي ترغب في مراجعته، ثم اختر نطاق التواريخ لعرض العمليات والقيود المالية التراكمية الخاصة به.
        </p>
      </div>
    {:else}
      <!-- Account Statement Cards -->
      <div in:slide={{ duration: 200 }} class="grid grid-cols-2 lg:grid-cols-4 gap-4 mb-6 select-none">
        <!-- Card 1: Opening Balance -->
        <div class="border border-border/60 bg-card/20 p-4 rounded-2xl text-start flex flex-col justify-between hover:border-primary/30 transition-all duration-200">
          <span class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">الرصيد الافتتاحي</span>
          <div class="flex items-baseline gap-1 mt-2">
            <span class="text-lg font-extrabold font-mono text-foreground">
              {formatCurrency(statement.opening_balance, statement.currency)}
            </span>
          </div>
          <span class="text-[9px] text-muted-foreground mt-1">قبل تاريخ {formatDate(statement.start_date)}</span>
        </div>

        <!-- Card 2: Total Debits -->
        <div class="border border-border/60 bg-card/20 p-4 rounded-2xl text-start flex flex-col justify-between hover:border-emerald-500/30 transition-all duration-200">
          <div class="flex items-center justify-between w-full">
            <span class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">إجمالي الحركات (مدين)</span>
            <div class="w-5 h-5 rounded-full bg-emerald-500/10 flex items-center justify-center text-emerald-500">
              <ArrowDownLeft size={10} />
            </div>
          </div>
          <div class="flex items-baseline gap-1 mt-2">
            <span class="text-lg font-extrabold font-mono text-emerald-500">
              {formatCurrency(totals.debit, statement.currency)}
            </span>
          </div>
          <span class="text-[9px] text-muted-foreground mt-1">مجموع المبالغ المحملة على الحساب</span>
        </div>

        <!-- Card 3: Total Credits -->
        <div class="border border-border/60 bg-card/20 p-4 rounded-2xl text-start flex flex-col justify-between hover:border-rose-500/30 transition-all duration-200">
          <div class="flex items-center justify-between w-full">
            <span class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">إجمالي الحركات (دائن)</span>
            <div class="w-5 h-5 rounded-full bg-rose-500/10 flex items-center justify-center text-rose-500">
              <ArrowUpRight size={10} />
            </div>
          </div>
          <div class="flex items-baseline gap-1 mt-2">
            <span class="text-lg font-extrabold font-mono text-rose-500">
              {formatCurrency(totals.credit, statement.currency)}
            </span>
          </div>
          <span class="text-[9px] text-muted-foreground mt-1">مجموع المبالغ المدفوعة أو المخصومة</span>
        </div>

        <!-- Card 4: Closing Balance -->
        <div class="border-2 border-primary bg-primary/5 p-4 rounded-2xl text-start flex flex-col justify-between shadow-sm">
          <div class="flex items-center justify-between w-full">
            <span class="text-[10px] font-bold text-primary uppercase tracking-wider">الرصيد الختامي</span>
            <TrendingUp size={12} class="text-primary" />
          </div>
          <div class="flex items-baseline gap-1 mt-2">
            <span class="text-xl font-extrabold font-mono text-primary">
              {formatCurrency(statement.closing_balance, statement.currency)}
            </span>
          </div>
          <span class="text-[9px] text-primary/80 mt-1">الرصيد الصافي المتبقي في {formatDate(statement.end_date)}</span>
        </div>
      </div>

      <!-- Statement Table -->
      {#if filteredItems.length === 0}
        <div class="border border-dashed border-border/80 rounded-2xl p-12 flex flex-col items-center justify-center text-center gap-2 select-none">
          <h4 class="text-xs font-bold text-muted-foreground">لا توجد حركات مالية في هذه الفترة</h4>
          <p class="text-[10px] text-muted-foreground/80">
            لم تسجل أي حركة مدين أو دائن على هذا الحساب ضمن نطاق التواريخ المحددة.
          </p>
        </div>
      {:else}
        <div in:fade={{ duration: 150 }} class="border border-border/60 rounded-xl overflow-hidden bg-card/30 shadow-sm print:border-none print:shadow-none">
          <Table>
            <TableHeader class="bg-muted/40 select-none print:bg-transparent">
              <TableRow>
                <TableHead class="text-start font-bold text-xs">التاريخ</TableHead>
                <TableHead class="text-start font-bold text-xs">المرجع</TableHead>
                <TableHead class="text-start font-bold text-xs">البيان / الحركة التفصيلية</TableHead>
                <TableHead class="text-end font-bold text-xs text-emerald-500">مدين (+)</TableHead>
                <TableHead class="text-end font-bold text-xs text-rose-500">دائن (-)</TableHead>
                <TableHead class="text-end font-bold text-xs">الرصيد التراكمي</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              <!-- Row for Opening Balance in table -->
              <TableRow class="bg-muted/10 font-medium hover:bg-muted/10 transition-colors">
                <TableCell class="text-start text-xs text-muted-foreground font-mono">{formatDate(statement.start_date)}</TableCell>
                <TableCell class="text-start text-xs font-semibold text-muted-foreground">OPENING</TableCell>
                <TableCell class="text-start text-xs text-muted-foreground font-semibold">رصيد أول المدة الافتتاحي</TableCell>
                <TableCell class="text-end text-xs font-mono">-</TableCell>
                <TableCell class="text-end text-xs font-mono">-</TableCell>
                <TableCell class="text-end text-xs font-bold font-mono text-foreground">
                  {formatCurrency(statement.opening_balance, statement.currency)}
                </TableCell>
              </TableRow>

              {#each filteredItems as item (item.id)}
                <TableRow class="hover:bg-muted/20 transition-all duration-150 print:hover:bg-transparent">
                  <TableCell class="text-start text-xs text-muted-foreground whitespace-nowrap">{formatDate(item.date)}</TableCell>
                  <TableCell class="text-start font-mono text-xs text-foreground font-bold whitespace-nowrap">{item.reference}</TableCell>
                  <TableCell class="text-start text-xs text-foreground max-w-xs md:max-w-md break-words font-semibold">
                    {item.description_ar || item.description}
                  </TableCell>
                  <TableCell class="text-end text-xs font-bold font-mono text-emerald-500 whitespace-nowrap">
                    {item.debit ? formatCurrency(item.debit, statement.currency) : "-"}
                  </TableCell>
                  <TableCell class="text-end text-xs font-bold font-mono text-rose-500 whitespace-nowrap">
                    {item.credit ? formatCurrency(item.credit, statement.currency) : "-"}
                  </TableCell>
                  <TableCell class="text-end text-xs font-bold font-mono text-foreground whitespace-nowrap">
                    {formatCurrency(item.running_balance, statement.currency)}
                  </TableCell>
                </TableRow>
              {/each}
            </TableBody>
          </Table>
        </div>
      {/if}
    {/if}
  </div>
</div>

<style>
/* CSS styles to make printing statements look incredibly clean and professional */
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
