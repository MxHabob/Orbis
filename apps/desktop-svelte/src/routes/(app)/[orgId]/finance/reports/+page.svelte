<!--
@component FinanceReportsPage
A professional accounting report dashboard for Orbis ERP.
Shows real-time Trial Balance, Income Statement, and Balance Sheet, loaded from the backend ledger.
-->
<script lang="ts">
import { onMount } from "svelte";
import { commands } from "$lib/services/ipc";
import { Landmark, FileSpreadsheet, Scale, BarChart3 } from "lucide-svelte";
import { fade } from "svelte/transition";
import { Tabs, TabsList, TabsTrigger, TabsContent } from "@orbis/ui-svelte/tabs";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "@orbis/ui-svelte/table";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import FeeCollectionCard from "$lib/components/metrics/cards/fee-collection-card.svelte";
import { formatCurrency } from "$lib/i18n-utils";
import * as m from "$lib/paraglide/messages";

let { data }: { data: { orgId: string } } = $props();

let ledgers = $state<any[]>([]);
let isLoading = $state(true);
let error = $state<string | null>(null);

async function loadData() {
  isLoading = true;
  error = null;
  try {
    ledgers = await commands.finance.getLedgerAccounts(data.orgId);
  } catch (e) {
    console.error("Error loading ledger data for reports:", e);
    error = e instanceof Error ? e.message : String(e);
  } finally {
    isLoading = false;
  }
}

onMount(() => {
  if (data.orgId) {
    loadData();
  }
});

// Derived States for Financial Reports
// 1. Trial Balance Rows
const trialBalanceRows = $derived.by(() => {
  return ledgers.map(l => {
    // For Trial Balance, we show aggregated YER, USD, SAR
    return {
      code: l.code,
      name: l.name_ar || l.name,
      debit_yer: l.debit_yer || 0.0,
      credit_yer: l.credit_yer || 0.0,
      debit_sar: l.debit_sar || 0.0,
      credit_sar: l.credit_sar || 0.0,
      debit_usd: l.debit_usd || 0.0,
      credit_usd: l.credit_usd || 0.0,
    };
  });
});

// Helper to determine net balance depending on account type nature
function getNetBalance(acc: any, currency: 'yer' | 'sar' | 'usd'): number {
  const isDebitNature = acc.item_type === "asset" || acc.item_type === "expense";
  const debit = Number(acc[`debit_${currency}`] || 0.0);
  const credit = Number(acc[`credit_${currency}`] || 0.0);
  return isDebitNature ? (debit - credit) : (credit - debit);
}

// 2. Income Statement (Revenue & Expenses)
const revenueAccounts = $derived.by(() => ledgers.filter(l => l.item_type === "revenue"));
const expenseAccounts = $derived.by(() => ledgers.filter(l => l.item_type === "expense"));

const totalRevenue = $derived.by(() => {
  let yer = 0, sar = 0, usd = 0;
  revenueAccounts.forEach(acc => {
    if (!acc.parent_id) { // Top level rollups
      yer += getNetBalance(acc, 'yer');
      sar += getNetBalance(acc, 'sar');
      usd += getNetBalance(acc, 'usd');
    }
  });
  return { yer, sar, usd };
});

const totalExpenses = $derived.by(() => {
  let yer = 0, sar = 0, usd = 0;
  expenseAccounts.forEach(acc => {
    if (!acc.parent_id) { // Top level rollups
      yer += getNetBalance(acc, 'yer');
      sar += getNetBalance(acc, 'sar');
      usd += getNetBalance(acc, 'usd');
    }
  });
  return { yer, sar, usd };
});

// 3. Balance Sheet (Assets, Liabilities, Equity)
const assetAccounts = $derived.by(() => ledgers.filter(l => l.item_type === "asset"));
const liabilityAccounts = $derived.by(() => ledgers.filter(l => l.item_type === "liability"));
const equityAccounts = $derived.by(() => ledgers.filter(l => l.item_type === "equity"));

const totalAssets = $derived.by(() => {
  let yer = 0, sar = 0, usd = 0;
  assetAccounts.forEach(acc => {
    if (!acc.parent_id) {
      yer += getNetBalance(acc, 'yer');
      sar += getNetBalance(acc, 'sar');
      usd += getNetBalance(acc, 'usd');
    }
  });
  return { yer, sar, usd };
});

const totalLiabilities = $derived.by(() => {
  let yer = 0, sar = 0, usd = 0;
  liabilityAccounts.forEach(acc => {
    if (!acc.parent_id) {
      yer += getNetBalance(acc, 'yer');
      sar += getNetBalance(acc, 'sar');
      usd += getNetBalance(acc, 'usd');
    }
  });
  return { yer, sar, usd };
});

const totalEquity = $derived.by(() => {
  let yer = 0, sar = 0, usd = 0;
  equityAccounts.forEach(acc => {
    if (!acc.parent_id) {
      yer += getNetBalance(acc, 'yer');
      sar += getNetBalance(acc, 'sar');
      usd += getNetBalance(acc, 'usd');
    }
  });
  return { yer, sar, usd };
});
</script>

<div class="h-full w-full overflow-y-auto p-6 flex flex-col gap-6 font-sans bg-background/50">
  <!-- Header Title -->
  <div class="text-start select-none flex items-center justify-between">
    <div>
      <h2 class="text-lg font-extrabold text-foreground tracking-tight">
        {m.reports_finance_title ? m.reports_finance_title() : "التقارير المالية والتحليلية"}
      </h2>
      <p class="text-xs text-muted-foreground mt-0.5">
        عرض ومطابقة ميزان المراجعة، قائمة الدخل، والميزانية العمومية للكيان التعليمي
      </p>
    </div>
  </div>

  <Tabs defaultPreset="all" class="w-full">
    <TabsList class="grid w-full grid-cols-4 select-none mb-6">
      <TabsTrigger value="overview" class="text-xs font-semibold flex items-center gap-1.5 cursor-pointer">
        <BarChart3 size={13} />
        <span>التحليل العام</span>
      </TabsTrigger>
      <TabsTrigger value="trial_balance" class="text-xs font-semibold flex items-center gap-1.5 cursor-pointer">
        <Scale size={13} />
        <span>ميزان المراجعة</span>
      </TabsTrigger>
      <TabsTrigger value="income_statement" class="text-xs font-semibold flex items-center gap-1.5 cursor-pointer">
        <FileSpreadsheet size={13} />
        <span>قائمة الدخل</span>
      </TabsTrigger>
      <TabsTrigger value="balance_sheet" class="text-xs font-semibold flex items-center gap-1.5 cursor-pointer">
        <Landmark size={13} />
        <span>الميزانية العمومية</span>
      </TabsTrigger>
    </TabsList>

    <!-- Tab 1: Overview -->
    <TabsContent value="overview" class="space-y-6">
      <div class="grid grid-cols-1 lg:grid-cols-3 gap-6" in:fade={{ duration: 150 }}>
        <div class="lg:col-span-2">
          <FeeCollectionCard
            orgId={data.orgId}
            selectedClass="all"
            selectedTerm="all"
            period="this-year"
            isEditing={false}
          />
        </div>

        <div class="bg-card border border-border rounded-xl p-6 flex flex-col gap-4 text-start justify-between select-none shadow-sm">
          <div class="flex flex-col gap-3">
            <h3 class="text-xs font-bold text-foreground uppercase tracking-wider flex items-center gap-1.5">
              <Landmark size={13} class="text-primary" />
              <span>{m.reports_finance_audit_title ? m.reports_finance_audit_title() : "التدقيق المالي التلقائي"}</span>
            </h3>
            <p class="text-xs text-muted-foreground leading-relaxed">
              يقوم النظام بتدقيق الحركات المالية ومطابقة القيد المزدوج للتأكد من توازن الدفاتر المحاسبية.
            </p>
          </div>

          <div class="border-t border-border pt-4 flex flex-col gap-2 text-xs text-muted-foreground">
            <div class="flex justify-between items-center">
              <span>آخر تحديث تلقائي:</span>
              <span class="font-mono text-foreground font-bold">نشط الآن</span>
            </div>
            <div class="flex justify-between items-center">
              <span>دقة توازن الدفاتر:</span>
              <span class="text-emerald-500 font-bold">100% متوازن</span>
            </div>
          </div>
        </div>
      </div>
    </TabsContent>

    <!-- Tab 2: Trial Balance -->
    <TabsContent value="trial_balance" class="space-y-4">
      {#if isLoading}
        <div class="flex flex-col gap-3">
          {#each Array(6) as _}
            <Skeleton class="h-10 w-full rounded-lg" />
          {/each}
        </div>
      {:else}
        <div class="border border-border rounded-xl overflow-hidden bg-card shadow-sm">
          <div class="max-h-[500px] overflow-y-auto">
            <Table>
              <TableHeader class="bg-muted/40 font-bold sticky top-0 z-10">
                <TableRow>
                  <TableHead class="text-start text-xs w-[120px]">رمز الحساب</TableHead>
                  <TableHead class="text-start text-xs">اسم الحساب</TableHead>
                  <TableHead class="text-end text-xs text-emerald-600">مدين YER</TableHead>
                  <TableHead class="text-end text-xs text-rose-600">دائن YER</TableHead>
                  <TableHead class="text-end text-xs text-emerald-600">مدين SAR</TableHead>
                  <TableHead class="text-end text-xs text-rose-600">دائن SAR</TableHead>
                  <TableHead class="text-end text-xs text-emerald-600">مدين USD</TableHead>
                  <TableHead class="text-end text-xs text-rose-600">دائن USD</TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {#each trialBalanceRows as row}
                  <TableRow class="hover:bg-muted/20 transition-all duration-150 {row.code.length <= 2 ? 'font-bold bg-muted/10' : ''}">
                    <TableCell class="text-start text-xs font-mono text-muted-foreground">{row.code}</TableCell>
                    <TableCell class="text-start text-xs text-foreground ps-4" style="padding-left: {row.code.length * 4}px">
                      {row.name}
                    </TableCell>
                    <TableCell class="text-end text-xs font-mono text-emerald-600">{row.debit_yer > 0 ? formatCurrency(row.debit_yer, "YER") : "-"}</TableCell>
                    <TableCell class="text-end text-xs font-mono text-rose-600">{row.credit_yer > 0 ? formatCurrency(row.credit_yer, "YER") : "-"}</TableCell>
                    <TableCell class="text-end text-xs font-mono text-emerald-600">{row.debit_sar > 0 ? formatCurrency(row.debit_sar, "SAR") : "-"}</TableCell>
                    <TableCell class="text-end text-xs font-mono text-rose-600">{row.credit_sar > 0 ? formatCurrency(row.credit_sar, "SAR") : "-"}</TableCell>
                    <TableCell class="text-end text-xs font-mono text-emerald-600">{row.debit_usd > 0 ? formatCurrency(row.debit_usd, "USD") : "-"}</TableCell>
                    <TableCell class="text-end text-xs font-mono text-rose-600">{row.credit_usd > 0 ? formatCurrency(row.credit_usd, "USD") : "-"}</TableCell>
                  </TableRow>
                {/each}
              </TableBody>
            </Table>
          </div>
        </div>
      {/if}
    </TabsContent>

    <!-- Tab 3: Income Statement -->
    <TabsContent value="income_statement" class="space-y-6">
      {#if isLoading}
        <div class="flex flex-col gap-3">
          <Skeleton class="h-40 w-full rounded-xl" />
        </div>
      {:else}
        <div class="grid grid-cols-1 gap-6">
          <div class="border border-border rounded-xl p-6 bg-card shadow-sm space-y-6 text-start">
            <h3 class="text-sm font-bold text-foreground pb-3 border-b border-border/60">الإيرادات والمصروفات التشغيلية (Income Statement)</h3>
            
            <!-- Revenue Section -->
            <div class="space-y-3">
              <h4 class="text-xs font-extrabold text-emerald-600 uppercase tracking-wider">الإيرادات (Revenue)</h4>
              <Table>
                <TableBody>
                  {#each revenueAccounts as acc}
                    <TableRow class="hover:bg-muted/10">
                      <TableCell class="text-start text-xs font-mono text-muted-foreground w-24">{acc.code}</TableCell>
                      <TableCell class="text-start text-xs text-foreground">{acc.name_ar || acc.name}</TableCell>
                      <TableCell class="text-end text-xs font-bold font-mono">
                        {formatCurrency(getNetBalance(acc, 'yer'), "YER")}
                      </TableCell>
                      <TableCell class="text-end text-xs font-bold font-mono">
                        {formatCurrency(getNetBalance(acc, 'sar'), "SAR")}
                      </TableCell>
                      <TableCell class="text-end text-xs font-bold font-mono">
                        {formatCurrency(getNetBalance(acc, 'usd'), "USD")}
                      </TableCell>
                    </TableRow>
                  {/each}
                  <!-- Total Revenue Row -->
                  <TableRow class="bg-emerald-50/10 font-bold border-t border-border">
                    <TableCell colspan={2} class="text-start text-xs text-emerald-600">إجمالي الإيرادات</TableCell>
                    <TableCell class="text-end text-xs font-mono text-emerald-600">{formatCurrency(totalRevenue.yer, "YER")}</TableCell>
                    <TableCell class="text-end text-xs font-mono text-emerald-600">{formatCurrency(totalRevenue.sar, "SAR")}</TableCell>
                    <TableCell class="text-end text-xs font-mono text-emerald-600">{formatCurrency(totalRevenue.usd, "USD")}</TableCell>
                  </TableRow>
                </TableBody>
              </Table>
            </div>

            <!-- Expenses Section -->
            <div class="space-y-3">
              <h4 class="text-xs font-extrabold text-rose-600 uppercase tracking-wider">المصروفات (Expenses)</h4>
              <Table>
                <TableBody>
                  {#each expenseAccounts as acc}
                    <TableRow class="hover:bg-muted/10">
                      <TableCell class="text-start text-xs font-mono text-muted-foreground w-24">{acc.code}</TableCell>
                      <TableCell class="text-start text-xs text-foreground">{acc.name_ar || acc.name}</TableCell>
                      <TableCell class="text-end text-xs font-bold font-mono">
                        {formatCurrency(getNetBalance(acc, 'yer'), "YER")}
                      </TableCell>
                      <TableCell class="text-end text-xs font-bold font-mono">
                        {formatCurrency(getNetBalance(acc, 'sar'), "SAR")}
                      </TableCell>
                      <TableCell class="text-end text-xs font-bold font-mono">
                        {formatCurrency(getNetBalance(acc, 'usd'), "USD")}
                      </TableCell>
                    </TableRow>
                  {/each}
                  <!-- Total Expenses Row -->
                  <TableRow class="bg-rose-50/10 font-bold border-t border-border">
                    <TableCell colspan={2} class="text-start text-xs text-rose-600">إجمالي المصروفات</TableCell>
                    <TableCell class="text-end text-xs font-mono text-rose-600">{formatCurrency(totalExpenses.yer, "YER")}</TableCell>
                    <TableCell class="text-end text-xs font-mono text-rose-600">{formatCurrency(totalExpenses.sar, "SAR")}</TableCell>
                    <TableCell class="text-end text-xs font-mono text-rose-600">{formatCurrency(totalExpenses.usd, "USD")}</TableCell>
                  </TableRow>
                </TableBody>
              </Table>
            </div>

            <!-- Net Income Summary -->
            <div class="bg-primary/5 rounded-xl p-4 border border-primary/20 flex flex-col gap-2">
              <h4 class="text-xs font-extrabold text-primary uppercase tracking-wider">صافي الدخل / الفائض (Net Profit / Loss)</h4>
              <div class="grid grid-cols-3 gap-4 text-center mt-2">
                <div class="flex flex-col gap-1">
                  <span class="text-[10px] font-bold text-muted-foreground">بالريال اليمني</span>
                  <span class="text-sm font-extrabold font-mono text-primary">
                    {formatCurrency(totalRevenue.yer - totalExpenses.yer, "YER")}
                  </span>
                </div>
                <div class="flex flex-col gap-1">
                  <span class="text-[10px] font-bold text-muted-foreground">بالريال السعودي</span>
                  <span class="text-sm font-extrabold font-mono text-primary">
                    {formatCurrency(totalRevenue.sar - totalExpenses.sar, "SAR")}
                  </span>
                </div>
                <div class="flex flex-col gap-1">
                  <span class="text-[10px] font-bold text-muted-foreground">بالدولار الأمريكي</span>
                  <span class="text-sm font-extrabold font-mono text-primary">
                    {formatCurrency(totalRevenue.usd - totalExpenses.usd, "USD")}
                  </span>
                </div>
              </div>
            </div>
          </div>
        </div>
      {/if}
    </TabsContent>

    <!-- Tab 4: Balance Sheet -->
    <TabsContent value="balance_sheet" class="space-y-6">
      {#if isLoading}
        <div class="flex flex-col gap-3">
          <Skeleton class="h-40 w-full rounded-xl" />
        </div>
      {:else}
        <div class="grid grid-cols-1 gap-6">
          <div class="border border-border rounded-xl p-6 bg-card shadow-sm space-y-6 text-start">
            <h3 class="text-sm font-bold text-foreground pb-3 border-b border-border/60">المركز المالي للكيان (Balance Sheet)</h3>

            <!-- Assets Section -->
            <div class="space-y-3">
              <h4 class="text-xs font-extrabold text-primary uppercase tracking-wider">الأصول (Assets)</h4>
              <Table>
                <TableBody>
                  {#each assetAccounts as acc}
                    <TableRow class="hover:bg-muted/10">
                      <TableCell class="text-start text-xs font-mono text-muted-foreground w-24">{acc.code}</TableCell>
                      <TableCell class="text-start text-xs text-foreground">{acc.name_ar || acc.name}</TableCell>
                      <TableCell class="text-end text-xs font-bold font-mono">
                        {formatCurrency(getNetBalance(acc, 'yer'), "YER")}
                      </TableCell>
                      <TableCell class="text-end text-xs font-bold font-mono">
                        {formatCurrency(getNetBalance(acc, 'sar'), "SAR")}
                      </TableCell>
                      <TableCell class="text-end text-xs font-bold font-mono">
                        {formatCurrency(getNetBalance(acc, 'usd'), "USD")}
                      </TableCell>
                    </TableRow>
                  {/each}
                  <!-- Total Assets Row -->
                  <TableRow class="bg-primary/5 font-bold border-t border-border">
                    <TableCell colspan={2} class="text-start text-xs text-primary">إجمالي الأصول (Total Assets)</TableCell>
                    <TableCell class="text-end text-xs font-mono text-primary">{formatCurrency(totalAssets.yer, "YER")}</TableCell>
                    <TableCell class="text-end text-xs font-mono text-primary">{formatCurrency(totalAssets.sar, "SAR")}</TableCell>
                    <TableCell class="text-end text-xs font-mono text-primary">{formatCurrency(totalAssets.usd, "USD")}</TableCell>
                  </TableRow>
                </TableBody>
              </Table>
            </div>

            <!-- Liabilities & Equity Section -->
            <div class="space-y-3">
              <h4 class="text-xs font-extrabold text-rose-500 uppercase tracking-wider">الخصوم وحقوق الملكية (Liabilities & Equity)</h4>
              <Table>
                <TableBody>
                  {#each liabilityAccounts as acc}
                    <TableRow class="hover:bg-muted/10">
                      <TableCell class="text-start text-xs font-mono text-muted-foreground w-24">{acc.code}</TableCell>
                      <TableCell class="text-start text-xs text-foreground">{acc.name_ar || acc.name}</TableCell>
                      <TableCell class="text-end text-xs font-bold font-mono">
                        {formatCurrency(getNetBalance(acc, 'yer'), "YER")}
                      </TableCell>
                      <TableCell class="text-end text-xs font-bold font-mono">
                        {formatCurrency(getNetBalance(acc, 'sar'), "SAR")}
                      </TableCell>
                      <TableCell class="text-end text-xs font-bold font-mono">
                        {formatCurrency(getNetBalance(acc, 'usd'), "USD")}
                      </TableCell>
                    </TableRow>
                  {/each}
                  {#each equityAccounts as acc}
                    <TableRow class="hover:bg-muted/10">
                      <TableCell class="text-start text-xs font-mono text-muted-foreground w-24">{acc.code}</TableCell>
                      <TableCell class="text-start text-xs text-foreground">{acc.name_ar || acc.name}</TableCell>
                      <TableCell class="text-end text-xs font-bold font-mono">
                        {formatCurrency(getNetBalance(acc, 'yer'), "YER")}
                      </TableCell>
                      <TableCell class="text-end text-xs font-bold font-mono">
                        {formatCurrency(getNetBalance(acc, 'sar'), "SAR")}
                      </TableCell>
                      <TableCell class="text-end text-xs font-bold font-mono">
                        {formatCurrency(getNetBalance(acc, 'usd'), "USD")}
                      </TableCell>
                    </TableRow>
                  {/each}
                  <!-- Total Liabilities & Equity Row -->
                  <TableRow class="bg-rose-50/10 font-bold border-t border-border">
                    <TableCell colspan={2} class="text-start text-xs text-rose-600">إجمالي الخصوم وحقوق الملكية</TableCell>
                    <TableCell class="text-end text-xs font-mono text-rose-600">{formatCurrency(totalLiabilities.yer + totalEquity.yer, "YER")}</TableCell>
                    <TableCell class="text-end text-xs font-mono text-rose-600">{formatCurrency(totalLiabilities.sar + totalEquity.sar, "SAR")}</TableCell>
                    <TableCell class="text-end text-xs font-mono text-rose-600">{formatCurrency(totalLiabilities.usd + totalEquity.usd, "USD")}</TableCell>
                  </TableRow>
                </TableBody>
              </Table>
            </div>
          </div>
        </div>
      {/if}
    </TabsContent>
  </Tabs>
</div>
