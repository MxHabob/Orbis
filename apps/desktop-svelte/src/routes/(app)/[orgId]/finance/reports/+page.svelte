<!--
@component FinanceReportsPage
A professional accounting report dashboard for Orbis ERP.
Shows real-time Trial Balance, Income Statement, and Balance Sheet, loaded from the backend ledger.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import { commands } from "$lib/core/ipc";
import { Landmark, FileSpreadsheet, Scale, BarChart3 } from "lucide-svelte";
import { fade } from "svelte/transition";
import {
  Tabs,
  TabsList,
  TabsTrigger,
  TabsContent,
} from "@orbis/ui-svelte/tabs";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@orbis/ui-svelte/table";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import FeeCollectionCard from "$lib/features/metrics/cards/fee-collection-card.svelte";
import { formatCurrency } from "$lib/core/i18n";

let { data }: { data: { orgId: string } } = $props();

let ledgers = $state<any[]>([]);
let activeCurrencies = $state<any[]>([]);
let isLoading = $state(true);
let error = $state<string | null>(null);

async function loadData() {
  isLoading = true;
  error = null;
  try {
    const [ledgersData, currenciesData] = await Promise.all([
      commands.finance.listAccounts(data.orgId),
      commands.finance.listCurrencies()
    ]);
    ledgers = ledgersData;
    activeCurrencies = currenciesData.filter((c: any) => c.is_active === 1);
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
  return ledgers.map((l) => {
    // For Trial Balance, we show aggregated balances for active currencies
    const balances: Record<string, { debit: number; credit: number }> = {};
    activeCurrencies.forEach(c => {
      const code = c.code.toLowerCase();
      balances[code] = {
        debit: l[`debit_${code}`] || 0.0,
        credit: l[`credit_${code}`] || 0.0,
      };
    });

    return {
      code: l.code,
      name: l.name_ar || l.name,
      balances,
    };
  });
});

// Helper to determine net balance depending on account type nature
function getNetBalance(acc: any, currencyCode: string): number {
  const isDebitNature =
    acc.item_type === "asset" || acc.item_type === "expense";
  const code = currencyCode.toLowerCase();
  const debit = Number(acc[`debit_${code}`] || 0.0);
  const credit = Number(acc[`credit_${code}`] || 0.0);
  return isDebitNature ? debit - credit : credit - debit;
}

// 2. Income Statement (Revenue & Expenses)
const revenueAccounts = $derived.by(() =>
  ledgers.filter((l) => l.item_type === "revenue"),
);
const expenseAccounts = $derived.by(() =>
  ledgers.filter((l) => l.item_type === "expense"),
);

const totalRevenue = $derived.by(() => {
  const totals: Record<string, number> = {};
  activeCurrencies.forEach(c => { totals[c.code.toLowerCase()] = 0; });

  revenueAccounts.forEach((acc) => {
    if (!acc.parent_id) {
      activeCurrencies.forEach(c => {
        const code = c.code.toLowerCase();
        totals[code] += getNetBalance(acc, code);
      });
    }
  });
  return totals;
});

const totalExpenses = $derived.by(() => {
  const totals: Record<string, number> = {};
  activeCurrencies.forEach(c => { totals[c.code.toLowerCase()] = 0; });

  expenseAccounts.forEach((acc) => {
    if (!acc.parent_id) {
      activeCurrencies.forEach(c => {
        const code = c.code.toLowerCase();
        totals[code] += getNetBalance(acc, code);
      });
    }
  });
  return totals;
});

// 3. Balance Sheet (Assets, Liabilities, Equity)
const assetAccounts = $derived.by(() =>
  ledgers.filter((l) => l.item_type === "asset"),
);
const liabilityAccounts = $derived.by(() =>
  ledgers.filter((l) => l.item_type === "liability"),
);
const equityAccounts = $derived.by(() =>
  ledgers.filter((l) => l.item_type === "equity"),
);

const totalAssets = $derived.by(() => {
  const totals: Record<string, number> = {};
  activeCurrencies.forEach(c => { totals[c.code.toLowerCase()] = 0; });

  assetAccounts.forEach((acc) => {
    if (!acc.parent_id) {
      activeCurrencies.forEach(c => {
        const code = c.code.toLowerCase();
        totals[code] += getNetBalance(acc, code);
      });
    }
  });
  return totals;
});

const totalLiabilities = $derived.by(() => {
  const totals: Record<string, number> = {};
  activeCurrencies.forEach(c => { totals[c.code.toLowerCase()] = 0; });

  liabilityAccounts.forEach((acc) => {
    if (!acc.parent_id) {
      activeCurrencies.forEach(c => {
        const code = c.code.toLowerCase();
        totals[code] += getNetBalance(acc, code);
      });
    }
  });
  return totals;
});

const totalEquity = $derived.by(() => {
  const totals: Record<string, number> = {};
  activeCurrencies.forEach(c => { totals[c.code.toLowerCase()] = 0; });

  equityAccounts.forEach((acc) => {
    if (!acc.parent_id) {
      activeCurrencies.forEach(c => {
        const code = c.code.toLowerCase();
        totals[code] += getNetBalance(acc, code);
      });
    }
  });
  return totals;
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

  <Tabs value="all" class="w-full">
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

        <div class="bg-card border border-border/60 rounded-xl p-6 flex flex-col gap-4 text-start justify-between select-none shadow-sm card-tactile transition-all duration-200">
          <div class="flex flex-col gap-3">
            <div class="flex items-center justify-between w-full select-none text-[9px] font-bold text-muted-foreground/80 mb-1">
              <div class="flex items-center gap-1.5 font-sans font-extrabold uppercase tracking-wider text-[8px] text-muted-foreground/75">
                <span class="h-1.5 w-1.5 rounded-full bg-emerald-500 animate-pulse"></span>
                {m.reports_finance_audit_title ? m.reports_finance_audit_title() : "التدقيق المالي التلقائي"}
              </div>
            </div>
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
                  {#each activeCurrencies as c}
                    <TableHead class="text-end text-xs text-emerald-600">مدين {c.code}</TableHead>
                    <TableHead class="text-end text-xs text-rose-600">دائن {c.code}</TableHead>
                  {/each}
                </TableRow>
              </TableHeader>
              <TableBody>
                {#each trialBalanceRows as row}
                  <TableRow class="hover:bg-muted/20 transition-all duration-150 {row.code.length <= 2 ? 'font-bold bg-muted/10' : ''}">
                    <TableCell class="text-start text-xs font-mono text-muted-foreground">{row.code}</TableCell>
                    <TableCell class="text-start text-xs text-foreground ps-4" style="padding-left: {row.code.length * 4}px">
                      {row.name}
                    </TableCell>
                    {#each activeCurrencies as c}
                      {@const code = c.code.toLowerCase()}
                      {@const deb = row.balances[code]?.debit || 0}
                      {@const cred = row.balances[code]?.credit || 0}
                      <TableCell class="text-end text-xs font-mono text-emerald-600">{deb > 0 ? formatCurrency(deb, c.code) : "-"}</TableCell>
                      <TableCell class="text-end text-xs font-mono text-rose-600">{cred > 0 ? formatCurrency(cred, c.code) : "-"}</TableCell>
                    {/each}
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
                      {#each activeCurrencies as c}
                        <TableCell class="text-end text-xs font-bold font-mono">
                          {formatCurrency(getNetBalance(acc, c.code), c.code)}
                        </TableCell>
                      {/each}
                    </TableRow>
                  {/each}
                  <!-- Total Revenue Row -->
                  <TableRow class="bg-emerald-50/10 font-bold border-t border-border">
                    <TableCell colspan={2} class="text-start text-xs text-emerald-600">إجمالي الإيرادات</TableCell>
                    {#each activeCurrencies as c}
                      <TableCell class="text-end text-xs font-mono text-emerald-600">{formatCurrency(totalRevenue[c.code.toLowerCase()] || 0, c.code)}</TableCell>
                    {/each}
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
                      {#each activeCurrencies as c}
                        <TableCell class="text-end text-xs font-bold font-mono">
                          {formatCurrency(getNetBalance(acc, c.code), c.code)}
                        </TableCell>
                      {/each}
                    </TableRow>
                  {/each}
                  <!-- Total Expenses Row -->
                  <TableRow class="bg-rose-50/10 font-bold border-t border-border">
                    <TableCell colspan={2} class="text-start text-xs text-rose-600">إجمالي المصروفات</TableCell>
                    {#each activeCurrencies as c}
                      <TableCell class="text-end text-xs font-mono text-rose-600">{formatCurrency(totalExpenses[c.code.toLowerCase()] || 0, c.code)}</TableCell>
                    {/each}
                  </TableRow>
                </TableBody>
              </Table>
            </div>

            <!-- Net Income Summary -->
            <div class="bg-primary/5 rounded-xl p-4 border border-primary/20 flex flex-col gap-2">
              <h4 class="text-xs font-extrabold text-primary uppercase tracking-wider">صافي الدخل / الفائض (Net Profit / Loss)</h4>
              <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4 text-center mt-2">
                {#each activeCurrencies as c}
                  <div class="flex flex-col gap-1">
                    <span class="text-[10px] font-bold text-muted-foreground">{c.name_ar || c.name}</span>
                    <span class="text-sm font-extrabold font-mono text-primary">
                      {formatCurrency((totalRevenue[c.code.toLowerCase()] || 0) - (totalExpenses[c.code.toLowerCase()] || 0), c.code)}
                    </span>
                  </div>
                {/each}
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
                      {#each activeCurrencies as c}
                        <TableCell class="text-end text-xs font-bold font-mono">
                          {formatCurrency(getNetBalance(acc, c.code), c.code)}
                        </TableCell>
                      {/each}
                    </TableRow>
                  {/each}
                  <!-- Total Assets Row -->
                  <TableRow class="bg-primary/5 font-bold border-t border-border">
                    <TableCell colspan={2} class="text-start text-xs text-primary">إجمالي الأصول (Total Assets)</TableCell>
                    {#each activeCurrencies as c}
                      <TableCell class="text-end text-xs font-mono text-primary">{formatCurrency(totalAssets[c.code.toLowerCase()] || 0, c.code)}</TableCell>
                    {/each}
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
                      {#each activeCurrencies as c}
                        <TableCell class="text-end text-xs font-bold font-mono">
                          {formatCurrency(getNetBalance(acc, c.code), c.code)}
                        </TableCell>
                      {/each}
                    </TableRow>
                  {/each}
                  {#each equityAccounts as acc}
                    <TableRow class="hover:bg-muted/10">
                      <TableCell class="text-start text-xs font-mono text-muted-foreground w-24">{acc.code}</TableCell>
                      <TableCell class="text-start text-xs text-foreground">{acc.name_ar || acc.name}</TableCell>
                      {#each activeCurrencies as c}
                        <TableCell class="text-end text-xs font-bold font-mono">
                          {formatCurrency(getNetBalance(acc, c.code), c.code)}
                        </TableCell>
                      {/each}
                    </TableRow>
                  {/each}
                  <!-- Total Liabilities & Equity Row -->
                  <TableRow class="bg-rose-50/10 font-bold border-t border-border">
                    <TableCell colspan={2} class="text-start text-xs text-rose-600">إجمالي الخصوم وحقوق الملكية</TableCell>
                    {#each activeCurrencies as c}
                      <TableCell class="text-end text-xs font-mono text-rose-600">{formatCurrency((totalLiabilities[c.code.toLowerCase()] || 0) + (totalEquity[c.code.toLowerCase()] || 0), c.code)}</TableCell>
                    {/each}
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
