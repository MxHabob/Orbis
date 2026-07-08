<!--
@component StudentFeesListingPage
Lists all student fees across the school, showing descriptions, student names,
due dates, ledger accounts, status indicators, and total amounts.
Allows triggering FeeAssignSheet to bill a student.
-->
<script lang="ts">
import { onMount } from "svelte";
import { commands } from "$lib/core/ipc";
import { Button } from "@orbis/ui-svelte/button";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui-svelte/select";
import { Plus, Wallet, CheckSquare, HelpCircle } from "lucide-svelte";
import { getLocale } from "$lib/paraglide/runtime";
import { formatCurrency, formatDate } from "$lib/core/i18n";
import { fade } from "svelte/transition";
import FeeAssignSheet from "$lib/features/finance/components/fee-assign-sheet.svelte";
import * as m from "$lib/paraglide/messages";

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
    const list = await commands.finance.getStudentFees(data.orgId);
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

// Derived Filtered List
const filteredFees = $derived.by(() => {
  return fees.filter((f) => {
    // 1. Search Query
    let matchesSearch = true;
    if (searchQuery) {
      const q = searchQuery.toLowerCase().trim();
      const studentName = String(f.student_name || "").toLowerCase();
      const studentNum = String(f.student_number || "").toLowerCase();
      const desc = String(f.description || "").toLowerCase();
      matchesSearch =
        studentName.includes(q) || studentNum.includes(q) || desc.includes(q);
    }

    // 2. Status
    let matchesStatus = true;
    if (statusFilter !== "all") {
      const stat = String(f.status || "").toLowerCase();
      matchesStatus = stat === statusFilter;
    }

    return matchesSearch && matchesStatus;
  });
});

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
  if (s === "paid") return m.fees_status_paid();
  if (s === "partially_paid") return m.fees_status_partially_paid();
  return m.fees_status_unpaid();
}

// Grid Source
const gridSource = $derived(
  withIndex(
    filteredFees.map((f) => ({
      ...f,
      student_name: f.student_name || f.enrollment_id || "—",
      student_avatar: f.student_avatar || f.avatar || null,
      template_name: f.description || m.fees_default_description(),
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
    student: m.fees_col_student(),
    template: m.fees_col_description(),
    total: m.fees_col_amount(),
    paid: m.fees_total_collected(),
    balance: m.fees_total_due(),
    status: m.fees_col_status(),
  }),
);

const rtl = $derived(getLocale() === "ar");
</script>

<div class="h-full w-full overflow-y-auto p-6 flex flex-col gap-6 font-sans">
  <!-- Header Bar & Action -->
  <div class="flex items-center justify-between gap-4 shrink-0 select-none">
    <div class="text-start">
      <h2 class="text-sm font-bold text-foreground uppercase tracking-wider">
        {m.fees_page_title()}
      </h2>
      <p class="text-[10px] text-muted-foreground mt-0.5">
        {m.fees_page_subtitle()}
      </p>
    </div>

    <Button 
      variant="lime" 
      class="text-xs h-8 px-3 shrink-0" 
      onclick={() => isSheetOpen = true}
    >
      <Plus size={14} class="me-1.5" />
      {m.fees_assign_button()}
    </Button>
  </div>

  <!-- Statistics summary row -->
  {#if !isLoading && fees.length > 0}
    <div class="grid grid-cols-1 sm:grid-cols-3 gap-4" in:fade={{ duration: 150 }}>
      <!-- Total Invoiced -->
      <div class="bg-card border border-border rounded-xl flex items-center justify-between p-4 h-20 select-none">
        <div class="flex flex-col gap-1 text-start">
          <span class="text-[9px] font-bold text-muted-foreground uppercase tracking-wider">
            {m.fees_total_invoiced()}
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
            {m.fees_total_collected()}
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
            {m.fees_total_due()}
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
      searchPlaceholder={m.fees_search_placeholder()}
      bind:searchValue={searchQuery}
      emptyIcon={Wallet}
      emptyTitle={m.fees_empty_title()}
      emptyDescription={m.fees_empty_desc()}
      {rtl}
      refreshable
      onRefresh={loadFees}
    >
      {#snippet actions()}
        <div class="flex items-center gap-2 text-start select-none me-2">
          <span class="text-xs font-bold text-muted-foreground uppercase tracking-wider whitespace-nowrap">
            {m.fees_filter_status()}
          </span>
          <div class="w-36">
            <Select type="single" value={statusFilter} onValueChange={(v) => (statusFilter = v ?? "all")}>
              <SelectTrigger class="h-8 w-full text-xs bg-card border-border/60">
                {#if statusFilter === "all"}
                  {m.fees_filter_status_all()}
                {:else if statusFilter === "unpaid"}
                  {m.fees_status_unpaid()}
                {:else if statusFilter === "partially_paid"}
                  {m.fees_status_partially_paid()}
                {:else if statusFilter === "paid"}
                  {m.fees_status_paid()}
                {/if}
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="all">{m.fees_filter_status_all()}</SelectItem>
                <SelectItem value="unpaid">{m.fees_status_unpaid()}</SelectItem>
                <SelectItem value="partially_paid">{m.fees_status_partially_paid()}</SelectItem>
                <SelectItem value="paid">{m.fees_status_paid()}</SelectItem>
              </SelectContent>
            </Select>
          </div>
        </div>
      {/snippet}
    </DataTable>
  </div>
</div>

<!-- Modal Form Dialog -->
<FeeAssignSheet 
  bind:open={isSheetOpen}
  orgId={data.orgId}
  onSuccess={loadFees}
/>
