<!--
@component StudentFeesListingPage
Lists all student fees across the school, showing descriptions, student names,
due dates, ledger accounts, status indicators, and total amounts.
Allows triggering FeeAssignSheet to bill a student.
-->
<script lang="ts">
import { onMount } from "svelte";
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
  Plus,
  Wallet,
  Search,
  FilterX,
  HelpCircle,
  Calendar,
  CheckSquare,
} from "lucide-svelte";
import { getLocale } from "$lib/paraglide/runtime";
import { formatCurrency, formatDate } from "$lib/i18n-utils";
import { fade } from "svelte/transition";
import FeeAssignSheet from "$lib/components/finance/fee-assign-sheet.svelte";
import * as m from "$lib/paraglide/messages";

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
    console.error("Error loading fees in directory:", e);
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

function getStatusBadgeClass(statusStr: string) {
  const s = String(statusStr).toLowerCase();
  if (s === "paid")
    return "bg-emerald-500/10 text-emerald-500 border-emerald-500/20";
  if (s === "partially_paid")
    return "bg-amber-500/10 text-amber-500 border-amber-500/20";
  return "bg-rose-500/10 text-rose-500 border-rose-500/20";
}

function getStatusLabel(statusStr: string) {
  const s = String(statusStr).toLowerCase();
  if (s === "paid") return m.fees_status_paid();
  if (s === "partially_paid") return m.fees_status_partially_paid();
  return m.fees_status_unpaid();
}
</script>

<div class="h-full w-full overflow-y-auto p-6 flex flex-col gap-6 font-sans">
	<!-- Header Bar & Action -->
	<div class="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4 shrink-0 select-none">
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
			class="text-xs h-9 px-3 shrink-0" 
			onclick={() => isSheetOpen = true}
		>
			<Plus size={14} class={getLocale() === "ar" ? 'ml-1.5' : 'mr-1.5'} />
			{m.fees_assign_button()}
		</Button>
	</div>

	<!-- Statistics summary row -->
	{#if !isLoading && fees.length > 0}
		<div class="grid grid-cols-1 sm:grid-cols-3 gap-4" in:fade={{ duration: 150 }}>
			<!-- Total Invoiced -->
			<div class="card-tactile bg-card/40 flex items-center justify-between p-4 h-20 select-none">
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
			<div class="card-tactile bg-card/40 flex items-center justify-between p-4 h-20 select-none">
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
			<div class="card-tactile bg-card/40 flex items-center justify-between p-4 h-20 select-none">
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

	<!-- Search & Filters Container -->
	<div class="flex flex-col md:flex-row md:items-center md:justify-between gap-4 bg-card/25 p-4 rounded-xl border border-border/80 shrink-0">
		<div class="flex flex-col sm:flex-row sm:items-center gap-4 w-full">
			<!-- Search query -->
			<div class="relative w-full sm:max-w-xs text-start">
				<Search class="absolute left-3 top-1/2 h-3.5 w-3.5 -translate-y-1/2 text-muted-foreground" />
				<Input
					type="text"
					placeholder={m.fees_search_placeholder()}
					class="input-sunken ps-9 text-xs py-1.5"
					bind:value={searchQuery}
				/>
			</div>

			<!-- Status selector -->
			<div class="flex items-center gap-2 text-start">
				<span class="text-xs font-bold text-muted-foreground uppercase tracking-wider whitespace-nowrap">
					{m.fees_filter_status()}
				</span>
				<select 
					bind:value={statusFilter}
					class="bg-background text-foreground border border-border rounded-md px-3 py-1.5 text-xs focus:ring-1 focus:ring-ring outline-none min-w-32"
				>
					<option value="all">{m.fees_filter_status_all()}</option>
					<option value="unpaid">{m.fees_status_unpaid()}</option>
					<option value="partially_paid">{m.fees_status_partially_paid()}</option>
					<option value="paid">{m.fees_status_paid()}</option>
				</select>
			</div>
		</div>
	</div>

	<!-- Main Billing Table Directory -->
	{#if isLoading}
		<div class="flex flex-col gap-3">
			<Skeleton class="h-10 w-full" />
			<Skeleton class="h-10 w-full" />
			<Skeleton class="h-10 w-full" />
		</div>
	{:else if error}
		<div class="flex flex-col items-center justify-center py-16 text-destructive border border-destructive/20 bg-destructive/5 rounded-xl gap-2">
			<h3 class="font-semibold text-xs">{m.fees_error_loading()}</h3>
			<p class="text-[10px] text-muted-foreground">{error}</p>
			<Button variant="outline" onclick={loadFees} class="text-[10px] mt-2">{m.error_retry()}</Button>
		</div>
	{:else if filteredFees.length === 0}
		<div class="flex flex-col items-center justify-center py-16 text-muted-foreground border border-border border-dashed rounded-xl gap-4 bg-card/10 select-none">
			<Wallet size={48} class="text-muted-foreground/30" />
			<div class="flex flex-col gap-1 text-center">
				<p class="text-xs font-semibold">{m.fees_empty_title()}</p>
				<p class="text-[10px] text-muted-foreground/80">
					{m.fees_empty_desc()}
				</p>
			</div>
			<Button variant="lime" class="text-xs mt-2" onclick={() => isSheetOpen = true}>
				{m.fees_assign_button()}
			</Button>
		</div>
	{:else}
		<!-- Data Table -->
		<div class="border border-border rounded-xl bg-card overflow-hidden text-start" in:fade={{ duration: 150 }}>
			<Table>
				<TableHeader>
					<TableRow class="hover:bg-transparent">
						<TableHead class="text-start">{m.fees_col_description()}</TableHead>
						<TableHead class="text-start">{m.fees_col_student()}</TableHead>
						<TableHead class="text-start">{m.fees_col_ledger()}</TableHead>
						<TableHead class="text-start">{m.fees_col_due_date()}</TableHead>
						<TableHead class="text-start">{m.fees_col_status()}</TableHead>
						<TableHead class="text-start">{m.fees_col_amount()}</TableHead>
					</TableRow>
				</TableHeader>
				<TableBody>
					{#each filteredFees as fee (fee.id)}
						<TableRow class="hover:bg-accent/10">
							<TableCell class="font-bold text-foreground max-w-[200px] truncate">
								{fee.description || m.fees_default_description()}
							</TableCell>
							<TableCell class="font-medium text-foreground">
								<div class="flex flex-col">
									<span>{fee.student_name || fee.enrollment_id || "-"}</span>
									{#if fee.student_number}
										<span class="text-[8px] text-muted-foreground font-mono">#{fee.student_number}</span>
									{/if}
								</div>
							</TableCell>
							<TableCell class="text-muted-foreground">{fee.ledger_account_name || fee.ledger_account_id || "-"}</TableCell>
							<TableCell class="font-mono text-muted-foreground">
								{#if fee.due_date}
									<span class="flex items-center gap-1">
										<Calendar size={10} />
										<span>{formatDate(fee.due_date)}</span>
									</span>
								{:else}
									<span class="text-muted-foreground/55">-</span>
								{/if}
							</TableCell>
							<TableCell>
								<span class="px-2 py-0.5 rounded-full border text-[8px] font-bold uppercase tracking-wider {getStatusBadgeClass(fee.status || 'unpaid')}">
									{getStatusLabel(fee.status || 'unpaid')}
								</span>
							</TableCell>
							<TableCell class="font-bold font-mono text-foreground">{formatCurrency(Number(fee.amount || 0), fee.currency || "YER")}</TableCell>
						</TableRow>
					{/each}
				</TableBody>
			</Table>
		</div>
	{/if}
</div>

<!-- Modal Form Dialog -->
<FeeAssignSheet 
	bind:open={isSheetOpen}
	orgId={data.orgId}
	onSuccess={loadFees}
/>
