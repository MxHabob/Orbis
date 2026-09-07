<!--
@component StudentFeesListingPage
Lists all student fees across the school, showing descriptions, student names,
due dates, ledger accounts, status indicators, and total amounts.
Allows triggering FeeAssignSheet to bill a student.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import { api, } from "$lib/api";
import { Button } from "@orbis/ui/button";
import { Skeleton } from "@orbis/ui/skeleton";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui/select";
import { Plus, Wallet, CheckSquare, HelpCircle } from "@lucide/svelte";
import { getLocale } from "$lib/paraglide/runtime";
import { formatCurrency, formatDate } from "$lib/core/i18n";
import { fade } from "svelte/transition";
import FeeAssignSheet from "$lib/features/finance/components/fee-assign-sheet.svelte";

import { DataTable } from "$lib/tables";
import { fees as feesDef, withIndex } from "$lib/tables/definitions";

let { data }: { data: { orgId: string } } = $props();

// State
let fees = $state<any[]>([]);
let isLoading = $state(true);
let error = $state<string | null>(null);

// Filters State
let searchQuery = $state("");
let statusFilter = $state<string>("all");

// Sheet trigger State
let isSheetOpen = $state(false);

async function loadFees() {
  isLoading = true;
  error = null;
  try {
    const list = await api.listStudentFees();
    fees = list;
  } catch (e) {
    console.error("Error loading fees:", e);
    error = e instanceof Error ? e.message : String(e);
  } finally {
    isLoading = false;
  }
}

onMount(() => {
  if (data.orgId) {
    loadFees();
  }
});

import { applyFilter } from "@orbis/ui/filter-builder";
import type { UniversalFilter } from "@orbis/ui/filter-builder";
import { studentFeeFilterFields } from "$lib/tables/definitions/filter-configs";

let activeFilter = $state<UniversalFilter>({ logic: "AND", conditions: [], groups: [] });

// Derived Filtered List
const filteredFees = $derived(
  applyFilter(
    fees.filter((f) => {
      let matchesSearch = true;
      if (searchQuery) {
        const q = searchQuery.toLowerCase().trim();
        const studentName = String(f.student_name || "").toLowerCase();
        const studentNum = String(f.student_number || "").toLowerCase();
        const desc = String(f.description || "").toLowerCase();
        matchesSearch =
          studentName.includes(q) || studentNum.includes(q) || desc.includes(q);
      }
      return matchesSearch;
    }),
    activeFilter
  )
);

// Calculations
const stats = $derived.by(() => {
  let invoiced = 0;
  let collected = 0;
  for (const f of filteredFees) {
    invoiced += Number(f.amount || 0);
    collected += Number(f.paid_amount || 0);
  }
  return {
    invoiced,
    collected,
    due: Math.max(0, invoiced - collected),
  };
});

function getStatusLabel(statusStr: string) {
  const s = String(statusStr).toLowerCase();
  if (s === "paid") return m.finance_fees_status_paid();
  if (s === "partially_paid") return m.finance_fees_status_partially_paid();
  return m.finance_fees_status_unpaid();
}

// Grid Source
const gridSource = $derived(
  withIndex(
    filteredFees.map((f) => ({
      ...f,
      student_name: f.student_name || f.enrollment_id || "—",
      student_avatar: f.student_avatar || f.avatar || null,
      template_name: f.description || m.common_ft_category_tuition(),
      total_amount: formatCurrency(Number(f.amount || 0), f.currency || "YER"),
      paid_amount: formatCurrency(
        Number(f.paid_amount || 0),
        f.currency || "YER",
      ),
      balance: formatCurrency(
        Math.max(0, Number(f.amount || 0) - Number(f.paid_amount || 0)),
        f.currency || "YER",
      ),
      payment_status: getStatusLabel(f.status || "unpaid"),
    })),
  ),
);

// Columns
const columns = $derived(
  feesDef.columns({
    idx: "#",
    student: m.common_word_student(),
    template: m.finance_fees_col_description(),
    total: m.common_ft_col_amount(),
    paid: m.finance_fees_total_collected(),
    balance: m.finance_fees_total_due(),
    status: m.finance_fees_col_status(),
  }),
);

const rtl = $derived(getLocale() === "ar");
</script>

<div class="h-full w-full overflow-y-auto p-6 flex flex-col gap-6 font-sans">
  <div class="flex items-center justify-end gap-4 shrink-0 select-none">
    <Button 
      variant="lime" 
      class="text-xs h-8 px-3 shrink-0" 
      onclick={() => isSheetOpen = true}
    >
      <Plus size={14} class="me-1.5" />
      {m.finance_fees_assign_button()}
    </Button>
  </div>

  <!-- Statistics summary row -->
  {#if !isLoading && fees.length > 0}
    <div class="grid grid-cols-1 sm:grid-cols-3 gap-4" in:fade={{ duration: 150 }}>
      <!-- Total Invoiced -->
      <div class="bg-card border border-border rounded-xl flex items-center justify-between p-4 h-20 select-none">
        <div class="flex flex-col gap-1 text-start">
          <span class="text-[9px] font-bold text-muted-foreground uppercase tracking-wider">
            {m.finance_fees_total_invoiced()}
          </span>
          <span class="text-base font-bold text-foreground font-mono">{formatCurrency(stats.invoiced, "YER")}</span>
        </div>
        <div class="h-8 w-8 rounded-lg bg-primary/10 flex items-center justify-center text-primary">
          <Wallet size={15} />
        </div>
      </div>

      <!-- Total Collected -->
      <div class="bg-card border border-border rounded-xl flex items-center justify-between p-4 h-20 select-none">
        <div class="flex flex-col gap-1 text-start">
          <span class="text-[9px] font-bold text-muted-foreground uppercase tracking-wider">
            {m.finance_fees_total_collected()}
          </span>
          <span class="text-base font-bold text-emerald-500 font-mono">{formatCurrency(stats.collected, "YER")}</span>
        </div>
        <div class="h-8 w-8 rounded-lg bg-emerald-500/10 flex items-center justify-center text-emerald-500">
          <CheckSquare size={14} />
        </div>
      </div>

      <!-- Balance Due -->
      <div class="bg-card border border-border rounded-xl flex items-center justify-between p-4 h-20 select-none">
        <div class="flex flex-col gap-1 text-start">
          <span class="text-[9px] font-bold text-muted-foreground uppercase tracking-wider">
            {m.finance_fees_total_due()}
          </span>
          <span class="text-base font-bold text-rose-500 font-mono">{formatCurrency(stats.due, "YER")}</span>
        </div>
        <div class="h-8 w-8 rounded-lg bg-rose-500/10 flex items-center justify-center text-rose-500">
          <HelpCircle size={15} />
        </div>
      </div>
    </div>
  {/if}

  <!-- Main Billing Table -->
  <div class="flex-1 min-h-0">
    <DataTable
      {columns}
      source={gridSource}
      loading={isLoading}
      searchable
      searchPlaceholder={m.finance_fees_search_placeholder()}
      bind:searchValue={searchQuery}
      filterable={true}
      filterFields={studentFeeFilterFields}
      bind:filterValue={activeFilter}
      emptyIcon={Wallet}
      emptyTitle={m.finance_fees_empty_title()}
      emptyDescription={m.finance_fees_empty_desc()}
      {rtl}
      refreshable
      onRefresh={loadFees}
    />
  </div>
</div>

<!-- Modal Form Dialog -->
<FeeAssignSheet 
  bind:open={isSheetOpen}
  orgId={data.orgId}
  onSuccess={loadFees}
/>
