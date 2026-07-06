<!--
@component FinanceOverviewPage
Main dashboard for the organization's financial summary.
Displays an interactive full-width SVG trend line chart at the top,
followed by grids displaying late-paying students, collection progress,
recent transactions, and treasury balances. Inline action buttons are placed
directly inside the cards' headers.
-->
<script lang="ts">
import { Card } from "@orbis/ui-svelte";
import { Button } from "@orbis/ui-svelte/button";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import { Badge } from "@orbis/ui-svelte/badge";
import { Avatar, AvatarFallback } from "@orbis/ui-svelte/avatar";
import {
  Table,
  TableHeader,
  TableRow,
  TableHead,
  TableBody,
  TableCell,
} from "@orbis/ui-svelte/table";
import {
  Wallet,
  CreditCard,
  Landmark,
  ArrowRight,
  ShieldCheck,
  TrendingUp,
  AlertCircle,
  RefreshCw,
  Layers,
  ArrowUpRight,
  Calendar,
  FileText,
  Plus,
} from "lucide-svelte";
import { getLocale } from "$lib/paraglide/runtime";
import { formatCurrency, formatDate } from "$lib/core/i18n";
import { fade } from "svelte/transition";
import { localizeHref } from "$lib/paraglide/runtime";
import * as m from "$lib/paraglide/messages";

import {
  createStudentFeesQuery,
  createTreasuriesQuery,
  createPaymentsQuery,
} from "$lib/features/finance";
import { createEnrollmentsQuery } from "$lib/features/students";
import { createClassesQuery } from "$lib/features/academics";
import CashFlowTrendCard from "$lib/features/metrics/cards/cash-flow-trend-card.svelte";

let { data }: { data: { orgId: string } } = $props();

// TanStack Queries
const feesQuery = createStudentFeesQuery(data.orgId);
const paymentsQuery = createPaymentsQuery(data.orgId);
const treasuriesQuery = createTreasuriesQuery(data.orgId);
const enrollmentsQuery = createEnrollmentsQuery(data.orgId);
const classesQuery = createClassesQuery(data.orgId);

const fees = $derived($feesQuery.data ?? []);
const payments = $derived($paymentsQuery.data ?? []);
const treasuries = $derived($treasuriesQuery.data ?? []);
const enrollments = $derived($enrollmentsQuery.data ?? []);
const classes = $derived($classesQuery.data ?? []);

const isLoading = $derived(
  $feesQuery.isLoading ||
    $paymentsQuery.isLoading ||
    $treasuriesQuery.isLoading ||
    $enrollmentsQuery.isLoading ||
    $classesQuery.isLoading,
);

const error = $derived(
  $feesQuery.error ||
    $paymentsQuery.error ||
    $treasuriesQuery.error ||
    $enrollmentsQuery.error ||
    $classesQuery.error,
);

function refetchAll() {
  $feesQuery.refetch();
  $paymentsQuery.refetch();
  $treasuriesQuery.refetch();
  $enrollmentsQuery.refetch();
  $classesQuery.refetch();
}

// ── Financial KPI Computations ───────────────────────────────────────────
const totalFees = $derived(
  fees.reduce(
    (sum, f) => sum + Number(f.net_amount || f.original_amount || 0),
    0,
  ),
);

const totalPaid = $derived(
  payments.reduce((sum, p) => sum + Number(p.amount || 0), 0),
);

const balanceDue = $derived(Math.max(0, totalFees - totalPaid));

const collectionRate = $derived(
  totalFees > 0 ? Math.round((totalPaid / totalFees) * 100) : 0,
);

const treasuryAssets = $derived(
  treasuries.reduce(
    (sum, t) => sum + Number(t.balance || t.current_balance || 0),
    0,
  ),
);

// ── Payments grouping mapping for Fee paid amounts ──────────────────────
const feePaymentsMap = $derived.by(() => {
  const map = new Map<string, number>();
  for (const p of payments) {
    if (p.fee_id) {
      const id = String(p.fee_id);
      map.set(id, (map.get(id) || 0) + Number(p.amount || 0));
    }
  }
  return map;
});

const getFeePaidAmount = (fId: string, status: string, netAmount: number) => {
  if (status === "paid") return netAmount;
  const fromPayments = feePaymentsMap.get(fId) || 0;
  if (status === "unpaid" && fromPayments === 0) return 0;
  return fromPayments;
};

// ── Late-Paying Students ──────────────────────────────────────────────────
const lateStudents = $derived.by(() => {
  const list = [];
  for (const f of fees) {
    if (f.status === "paid") continue;

    const netAmt = Number(f.net_amount || f.original_amount || 0);
    const paidAmt = getFeePaidAmount(f.id, f.status, netAmt);
    const remaining = Math.max(0, netAmt - paidAmt);

    if (remaining <= 0) continue;

    const enrollment = enrollments.find((e) => e.id === f.enrollment_id);
    const studentName = enrollment
      ? enrollment.full_name ||
        enrollment.name_ar ||
        enrollment.name_en ||
        enrollment.student_number ||
        "—"
      : "—";
    const studentNum = enrollment ? enrollment.student_number : "—";
    const avatar = enrollment ? enrollment.avatar : null;
    const classId = enrollment ? enrollment.class_id : null;

    const cls = classId ? classes.find((c) => c.id === classId) : null;
    const className = cls ? cls.name_ar || cls.name : "—";

    const dueDate = new Date(f.due_date);
    const diffTime = new Date().getTime() - dueDate.getTime();
    const diffDays = Math.ceil(diffTime / (1000 * 60 * 60 * 24));

    list.push({
      feeId: f.id,
      studentName,
      studentNum,
      avatar,
      className,
      remaining,
      dueDate: f.due_date,
      daysLate: diffDays > 0 ? diffDays : 0,
    });
  }

  // Sort by remaining due balance descending, take top 5
  return list.sort((a, b) => b.remaining - a.remaining).slice(0, 5);
});

// ── Recent Payments ──────────────────────────────────────────────────────
const recentPayments = $derived.by(() => {
  const sorted = [...payments].sort((a, b) => {
    const dateA = new Date(a.payment_date || a.created_at);
    const dateB = new Date(b.payment_date || b.created_at);
    return dateB.getTime() - dateA.getTime();
  });

  return sorted.slice(0, 5).map((p) => {
    const enrollment = enrollments.find((e) => e.id === p.enrollment_id);
    const studentName =
      p.student_name ||
      (enrollment
        ? enrollment.full_name ||
          enrollment.name_ar ||
          enrollment.name_en ||
          "—"
        : "—");

    return {
      ...p,
      studentName,
      receiptNumber: p.receipt_number || p.id.slice(0, 8),
    };
  });
});

// ── Reusable trend card configuration ──
</script>

<div class="h-full w-full overflow-y-auto bg-background/50 scrollbar-hide p-6 flex flex-col gap-6 font-sans">
  
  {#if isLoading}
    <!-- Loading skeleton matching layout exactly -->
    <Skeleton class="h-72 w-full rounded-xl animate-pulse" />
    <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
      <Skeleton class="h-64 lg:col-span-2 rounded-xl" />
      <Skeleton class="h-64 rounded-xl" />
    </div>
    <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
      <Skeleton class="h-64 lg:col-span-2 rounded-xl" />
      <Skeleton class="h-64 rounded-xl" />
    </div>
  {:else if error}
    <div class="flex flex-col items-center justify-center py-16 text-destructive border border-destructive/20 bg-destructive/5 rounded-xl gap-2">
      <AlertCircle size={24} />
      <h3 class="font-semibold text-xs">{m.finance_overview_error()}</h3>
      <p class="text-[10px] text-muted-foreground">{error instanceof Error ? error.message : String(error)}</p>
      <Button variant="outline" onclick={refetchAll} class="text-[10px] mt-2 h-8">{m.error_retry()}</Button>
    </div>
  {:else}
    <!-- Main Content Grid -->
    <div class="flex flex-col gap-6" in:fade={{ duration: 150 }}>
      
      <!-- 1. Full-Width SVG Cash Flow Trend Chart at the very top -->
      <CashFlowTrendCard orgId={data.orgId} />

      <!-- 2. Middle Section: Late Paying Students Table & Donut Analytics -->
      <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
        <!-- Late-Paying Students -->
        <Card class="lg:col-span-2 p-5 bg-card/60 border border-border flex flex-col justify-between text-start shadow-sm min-h-[250px]">
          <div class="flex items-center justify-between select-none border-b border-border/40 pb-2">
            <h3 class="text-xs font-bold text-foreground">
              {m.finance_overview_late_students ? m.finance_overview_late_students() : "الطلاب المتأخرين عن السداد"}
            </h3>
            <div class="flex items-center gap-2">
              <a href={localizeHref(`/${data.orgId}/finance/fees`)}>
                <Button variant="outline" size="xs" class="h-6 text-[9px] font-bold cursor-pointer">
                  {m.finance_overview_btn_assign_fee ? m.finance_overview_btn_assign_fee() : "فرض رسوم"}
                </Button>
              </a>
              <a href={localizeHref(`/${data.orgId}/finance/fees`)}>
                <Button variant="outline" size="xs" class="h-6 text-[9px] font-bold cursor-pointer">
                  {m.finance_overview_view_all ? m.finance_overview_view_all() : "عرض الكل"}
                </Button>
              </a>
            </div>
          </div>

          <div class="w-full overflow-hidden border border-border/50 rounded-xl bg-card/20 mt-3 flex-grow">
            <Table class="w-full text-start border-collapse text-xs">
              <TableHeader>
                <TableRow class="border-b border-border/75 bg-muted/20 text-muted-foreground text-[10px] font-bold uppercase tracking-wider select-none hover:bg-transparent">
                  <TableHead class="p-2.5 text-start h-auto text-muted-foreground">{m.finance_overview_student ? m.finance_overview_student() : "الطالب"}</TableHead>
                  <TableHead class="p-2.5 text-start h-auto text-muted-foreground">{m.finance_overview_class ? m.finance_overview_class() : "الصف"}</TableHead>
                  <TableHead class="p-2.5 text-start h-auto text-muted-foreground">{m.finance_overview_remaining ? m.finance_overview_remaining() : "المبلغ المتبقي"}</TableHead>
                  <TableHead class="p-2.5 text-start h-auto text-muted-foreground">{m.finance_overview_status ? m.finance_overview_status() : "الحالة"}</TableHead>
                </TableRow>
              </TableHeader>
              <TableBody>
                {#if lateStudents.length === 0}
                  <TableRow class="hover:bg-transparent">
                    <TableCell colspan="4" class="p-8 text-center text-muted-foreground select-none">
                      {m.finance_overview_no_late_students()}
                    </TableCell>
                  </TableRow>
                {:else}
                  {#each lateStudents as item, i (item.feeId)}
                    <TableRow class="border-b border-border/50 hover:bg-muted/10 transition-colors">
                      <TableCell class="p-2 flex items-center gap-2.5">
                        <Avatar class="h-6 w-6 border border-border/70 select-none">
                          <AvatarFallback class="text-[9px] font-bold bg-primary/5 text-primary">
                            {item.studentName.slice(0, 2).toUpperCase()}
                          </AvatarFallback>
                        </Avatar>
                        <div class="flex flex-col text-start min-w-0">
                          <span class="font-bold text-foreground truncate max-w-[140px] text-[11px]">{item.studentName}</span>
                          <span class="text-[8px] text-muted-foreground font-mono">{item.studentNum}</span>
                        </div>
                      </TableCell>
                      <TableCell class="p-2 font-semibold text-muted-foreground select-none text-[11px]">
                        {item.className}
                      </TableCell>
                      <TableCell class="p-2 font-bold font-mono text-foreground select-none text-[11px]">
                        {formatCurrency(item.remaining, "YER")}
                      </TableCell>
                      <TableCell class="p-2 select-none">
                        {#if item.daysLate > 0}
                          <Badge variant="destructive" class="text-[8px] px-1.5 py-0.5 font-bold">
                            {m.finance_overview_days_overdue ? m.finance_overview_days_overdue({ days: item.daysLate }) : `متأخر ${item.daysLate} يوم`}
                          </Badge>
                        {:else}
                          <Badge variant="outline" class="text-[8px] px-1.5 py-0.5 text-muted-foreground font-semibold">
                            يستحق قريباً
                          </Badge>
                        {/if}
                      </TableCell>
                    </TableRow>
                  {/each}
                {/if}
              </TableBody>
            </Table>
          </div>
        </Card>

        <!-- Donut Diagnostics Ratio -->
        <Card class="p-5 bg-card/60 border border-border flex flex-col justify-between text-start relative overflow-hidden shadow-sm min-h-[250px]">
          <div class="flex items-center justify-between select-none border-b border-border/40 pb-2">
            <h3 class="text-xs font-bold text-foreground">
              {m.finance_overview_performance_title ? m.finance_overview_performance_title() : "معدل تحصيل الرسوم"}
            </h3>
          </div>

          <!-- Circular Ring -->
          <div class="flex flex-col items-center justify-center py-2 relative select-none flex-grow mt-1">
            <svg class="w-20 h-20 transform -rotate-90" viewBox="0 0 100 100">
              <circle cx="50" cy="50" r="40" class="stroke-muted/40" stroke-width="7.5" fill="transparent" />
              <circle cx="50" cy="50" r="40" class="stroke-emerald-500 transition-all duration-700 ease-out" stroke-width="7.5" fill="transparent" stroke-dasharray={2 * Math.PI * 40} stroke-dashoffset={2 * Math.PI * 40 * (1 - collectionRate / 100)} stroke-linecap="round" />
            </svg>
            <div class="absolute flex flex-col items-center justify-center text-center">
              <span class="text-xl font-black text-foreground font-mono">{collectionRate}%</span>
              <span class="text-[8px] font-bold text-muted-foreground uppercase tracking-wider">{m.finance_overview_ratio ? m.finance_overview_ratio() : "Ratio"}</span>
            </div>
          </div>

          <!-- Sentry-style link list inside the body -->
          <div class="flex flex-col gap-2 pt-2 border-t border-border/40 w-full mt-2 text-[10px]">
            <a href={localizeHref(`/${data.orgId}/finance/reports`)} class="flex items-center gap-2 hover:underline text-primary font-bold">
              <FileText size={12} />
              <span>{m.finance_overview_btn_statement ? m.finance_overview_btn_statement() : "عرض التقارير المالية التفصيلية"}</span>
            </a>
          </div>
        </Card>
      </div>

      <!-- 3. Bottom Section: Recent Payments & Treasuries list -->
      <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
        <!-- Recent Payments Received -->
        <Card class="lg:col-span-2 p-5 bg-card/60 border border-border flex flex-col justify-between text-start shadow-sm min-h-[250px]">
          <div class="flex items-center justify-between select-none border-b border-border/40 pb-2">
            <h3 class="text-xs font-bold text-foreground">
              {m.finance_overview_recent_payments ? m.finance_overview_recent_payments() : "آخر المدفوعات المستلمة"}
            </h3>
            <div class="flex items-center gap-2">
              <a href={localizeHref(`/${data.orgId}/finance/payments`)}>
                <Button variant="outline" size="xs" class="h-6 text-[9px] font-bold cursor-pointer">
                  {m.finance_overview_btn_new_payment ? m.finance_overview_btn_new_payment() : "سند جديد"}
                </Button>
              </a>
              <a href={localizeHref(`/${data.orgId}/finance/payments`)}>
                <Button variant="outline" size="xs" class="h-6 text-[9px] font-bold cursor-pointer">
                  {m.finance_overview_view_all ? m.finance_overview_view_all() : "عرض الكل"}
                </Button>
              </a>
            </div>
          </div>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-2.5 mt-3 flex-grow">
            {#if recentPayments.length === 0}
              <div class="col-span-2 py-8 text-center text-muted-foreground text-xs select-none">
                {m.finance_overview_no_payments()}
              </div>
            {:else}
              {#each recentPayments as p (p.id)}
                <div class="flex items-center justify-between p-2 rounded-xl border border-border/40 bg-card/30 hover:bg-card/50 transition-colors">
                  <div class="flex items-center gap-2 text-start min-w-0">
                    <div class="h-6 w-6 rounded-lg bg-emerald-500/10 border border-emerald-500/20 flex items-center justify-center text-emerald-500 shrink-0 select-none">
                      <ArrowUpRight size={12} />
                    </div>
                    <div class="flex flex-col min-w-0">
                      <span class="font-bold text-foreground truncate max-w-[150px] text-[10px]">{p.studentName}</span>
                      <span class="text-[8px] text-muted-foreground font-mono flex items-center gap-1.5 select-none">
                        <span>{p.receiptNumber}</span>
                        <span>•</span>
                        <span>{p.payment_method === "Cash" ? "نقداً" : p.payment_method === "Check" ? "شيك" : "حوالة"}</span>
                      </span>
                    </div>
                  </div>
                  
                  <div class="flex flex-col items-end shrink-0 text-end">
                    <span class="font-bold font-mono text-emerald-500 text-[10px]">+{formatCurrency(p.amount, p.currency || "YER")}</span>
                    <span class="text-[7px] text-muted-foreground select-none font-mono mt-0.5">
                      {p.payment_date ? p.payment_date.split("T")[0] : p.created_at ? p.created_at.split("T")[0] : ""}
                    </span>
                  </div>
                </div>
              {/each}
            {/if}
          </div>
        </Card>

        <!-- Treasury balances list -->
        <Card class="p-5 bg-card/60 border border-border flex flex-col justify-between text-start relative overflow-hidden shadow-sm min-h-[250px]">
          <div class="flex items-center justify-between select-none border-b border-border/40 pb-2">
            <h3 class="text-xs font-bold text-foreground">
              {m.finance_overview_treasuries_list ? m.finance_overview_treasuries_list() : "أرصدة الخزائن والبنوك"}
            </h3>
          </div>

          <div class="flex flex-col gap-2 mt-3 w-full flex-grow">
            {#if treasuries.length === 0}
              <div class="py-8 text-center text-muted-foreground text-xs select-none">
                {m.finance_overview_no_treasuries()}
              </div>
            {:else}
              {#each treasuries.slice(0, 3) as t (t.id)}
                <div class="flex items-center justify-between p-2 rounded-xl border border-border/40 bg-card/30 hover:bg-card/50 transition-colors">
                  <div class="flex items-center gap-2 text-start min-w-0">
                    <div class="h-6 w-6 rounded-lg bg-brand-lime/10 border border-brand-lime/20 flex items-center justify-center text-brand-lime shrink-0 select-none">
                      <Landmark size={12} />
                    </div>
                    <div class="flex flex-col min-w-0">
                      <span class="font-bold text-foreground truncate max-w-[130px] text-[10px]">{t.name_ar || t.name}</span>
                      <span class="text-[8px] text-muted-foreground font-mono select-none">
                        {t.item_type === "Bank" ? "حساب بنكي" : "خزينة نقدية"}
                      </span>
                    </div>
                  </div>
                  
                  <div class="flex flex-col items-end shrink-0 text-end">
                    <span class="font-bold font-mono text-foreground text-[10px]">{formatCurrency(t.balance || t.current_balance || 0, t.currency || "YER")}</span>
                  </div>
                </div>
              {/each}
            {/if}
          </div>

          <!-- Sentry-style link list inside the body -->
          <div class="flex flex-col gap-2 pt-2 border-t border-border/40 w-full mt-2 text-[10px]">
            <a href={localizeHref(`/${data.orgId}/finance/treasuries`)} class="flex items-center gap-2 hover:underline text-primary font-bold">
              <Wallet size={12} />
              <span>{m.finance_overview_btn_manage_treasuries ? m.finance_overview_btn_manage_treasuries() : "إدارة الحسابات البنكية والخزائن"}</span>
            </a>
          </div>
        </Card>
      </div>
      
    </div>
  {/if}
</div>
