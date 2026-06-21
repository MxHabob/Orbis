<!--
@component PaymentsListingPage
Renders a ledger database of all collected payments, transaction details, receipts,
methods, payment dates, and student accounts.
Allows launching PaymentRecordSheet.
-->
<script lang="ts">
import { onMount } from "svelte";
import { commands } from "$lib/services/ipc";
import { 
  Table, TableBody, TableCell, TableHead, TableHeader, TableRow 
} from "@orbis/ui-svelte";
import { Button } from "@orbis/ui-svelte/button";
import { Input } from "@orbis/ui-svelte/input";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import { Plus, CreditCard, Search, Calendar, FileText, ClipboardList } from "lucide-svelte";
import { getLocale } from "$lib/paraglide/runtime";
import { formatCurrency, formatDate } from "$lib/i18n-utils";
import { fade } from "svelte/transition";
import PaymentRecordSheet from "$lib/components/finance/payment-record-sheet.svelte";

let { data }: { data: { orgId: string } } = $props();

const isAr = $derived(getLocale() === "ar");

// State
let payments = $state<any[]>([]);
let isLoading = $state(true);
let error = $state<string | null>(null);

// Filters State
let searchQuery = $state("");
let methodFilter = $state<string>("all");

// Sheet trigger State
let isSheetOpen = $state(false);

async function loadPayments() {
  isLoading = true;
  error = null;
  try {
    const list = await commands.finance.getPayments(data.orgId);
    payments = list;
  } catch (e) {
    console.error("Error loading payments list:", e);
    error = e instanceof Error ? e.message : String(e);
  } finally {
    isLoading = false;
  }
}

onMount(() => {
  if (data.orgId) {
    loadPayments();
  }
});

// Derived Filtered List
const filteredPayments = $derived.by(() => {
  return payments.filter(p => {
    // 1. Search Query
    let matchesSearch = true;
    if (searchQuery) {
      const q = searchQuery.toLowerCase().trim();
      const studentName = String(p.student_name || "").toLowerCase();
      const studentNum = String(p.student_number || "").toLowerCase();
      const receipt = String(p.receipt_number || p.id || "").toLowerCase();
      matchesSearch = studentName.includes(q) || studentNum.includes(q) || receipt.includes(q);
    }

    // 2. Method
    let matchesMethod = true;
    if (methodFilter !== "all") {
      const meth = String(p.payment_method || "").toLowerCase();
      matchesMethod = meth === methodFilter.toLowerCase();
    }

    return matchesSearch && matchesMethod;
  });
});

// Calculations
const stats = $derived.by(() => {
  let total = 0;
  for (const p of filteredPayments) {
    total += Number(p.amount || 0);
  }
  return {
    total,
    count: filteredPayments.length
  };
});

function getMethodLabel(methodStr: string) {
  const m = String(methodStr).toLowerCase();
  if (m === "cash") return isAr ? "نقدي" : "Cash";
  if (m === "bank transfer") return isAr ? "تحويل بنكي" : "Bank Transfer";
  if (m === "card") return isAr ? "بطاقة" : "Card";
  return methodStr;
}

function getMethodBadgeClass(methodStr: string) {
  const m = String(methodStr).toLowerCase();
  if (m === "cash") return "bg-emerald-500/10 text-emerald-500 border-emerald-500/20";
  if (m === "bank transfer") return "bg-blue-500/10 text-blue-500 border-blue-500/20";
  return "bg-purple-500/10 text-purple-500 border-purple-500/20";
}
</script>

<div class="h-full w-full overflow-y-auto p-6 flex flex-col gap-6 font-sans">
	<!-- Header Bar & Action -->
	<div class="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4 shrink-0 select-none">
		<div class="text-start">
			<h2 class="text-sm font-bold text-foreground uppercase tracking-wider">
				{isAr ? "سجل المقبوضات والمدفوعات" : "Payments Register"}
			</h2>
			<p class="text-[10px] text-muted-foreground mt-0.5">
				{isAr ? "متابعة وتدقيق إيصالات الدفع المقيدة في صناديق وحسابات المدرسة." : "Review registered payment entries, receipt notes, and transaction methods."}
			</p>
		</div>

		<Button 
			variant="lime" 
			class="text-xs h-9 px-3 shrink-0" 
			onclick={() => isSheetOpen = true}
		>
			<Plus size={14} class={isAr ? 'ml-1.5' : 'mr-1.5'} />
			{isAr ? "تسجيل دفعة واردة" : "Record Payment"}
		</Button>
	</div>

	<!-- Statistics summary row -->
	{#if !isLoading && payments.length > 0}
		<div class="grid grid-cols-1 sm:grid-cols-2 gap-4" in:fade={{ duration: 150 }}>
			<!-- Total Invoiced -->
			<div class="card-tactile bg-card/40 flex items-center justify-between p-4 h-20 select-none">
				<div class="flex flex-col gap-1 text-start">
					<span class="text-[9px] font-bold text-muted-foreground uppercase tracking-wider">
						{isAr ? "إجمالي المحصل (المحدد)" : "Total Collected (Filtered)"}
					</span>
					<span class="text-base font-bold text-emerald-500 font-mono">{formatCurrency(stats.total, "YER")}</span>
				</div>
				<div class="h-8 w-8 rounded-lg bg-emerald-500/10 flex items-center justify-center text-emerald-500">
					<CreditCard size={15} />
				</div>
			</div>

			<!-- Transaction Count -->
			<div class="card-tactile bg-card/40 flex items-center justify-between p-4 h-20 select-none">
				<div class="flex flex-col gap-1 text-start">
					<span class="text-[9px] font-bold text-muted-foreground uppercase tracking-wider">
						{isAr ? "عدد المعاملات (المحدد)" : "Transactions Count (Filtered)"}
					</span>
					<span class="text-base font-bold text-foreground font-mono">{stats.count}</span>
				</div>
				<div class="h-8 w-8 rounded-lg bg-primary/10 flex items-center justify-center text-primary">
					<ClipboardList size={14} />
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
					placeholder={isAr ? "البحث برقم الإيصال أو اسم الطالب..." : "Search receipt code or student name..."}
					class="input-sunken ps-9 text-xs py-1.5"
					bind:value={searchQuery}
				/>
			</div>

			<!-- Method selector -->
			<div class="flex items-center gap-2 text-start">
				<span class="text-xs font-bold text-muted-foreground uppercase tracking-wider whitespace-nowrap">
					{isAr ? "طريقة الدفع:" : "Method:"}
				</span>
				<select 
					bind:value={methodFilter}
					class="bg-background text-foreground border border-border rounded-md px-3 py-1.5 text-xs focus:ring-1 focus:ring-ring outline-none min-w-32"
				>
					<option value="all">{isAr ? "جميع الطرق" : "All Methods"}</option>
					<option value="Cash">{isAr ? "نقدي" : "Cash"}</option>
					<option value="Bank Transfer">{isAr ? "تحويل بنكي" : "Bank Transfer"}</option>
					<option value="Card">{isAr ? "بطاقة دفع" : "Card"}</option>
				</select>
			</div>
		</div>
	</div>

	<!-- Main Payments Table Directory -->
	{#if isLoading}
		<div class="flex flex-col gap-3">
			<Skeleton class="h-10 w-full" />
			<Skeleton class="h-10 w-full" />
			<Skeleton class="h-10 w-full" />
		</div>
	{:else if error}
		<div class="flex flex-col items-center justify-center py-16 text-destructive border border-destructive/20 bg-destructive/5 rounded-xl gap-2">
			<h3 class="font-semibold text-xs">{isAr ? "خطأ في تحميل سجل المدفوعات" : "Error loading payment log"}</h3>
			<p class="text-[10px] text-muted-foreground">{error}</p>
			<Button variant="outline" onclick={loadPayments} class="text-[10px] mt-2">{isAr ? "إعادة المحاولة" : "Retry"}</Button>
		</div>
	{:else if filteredPayments.length === 0}
		<div class="flex flex-col items-center justify-center py-16 text-muted-foreground border border-border border-dashed rounded-xl gap-4 bg-card/10 select-none">
			<CreditCard size={48} class="text-muted-foreground/30" />
			<div class="flex flex-col gap-1 text-center">
				<p class="text-xs font-semibold">{isAr ? "لا توجد مقبوضات مسجلة" : "No payment logs found"}</p>
				<p class="text-[10px] text-muted-foreground/80">
					{isAr ? "لا توجد دفعات مطابقة لخيارات الفرز الحالية." : "There are no received payments matching the active search parameters."}
				</p>
			</div>
			<Button variant="lime" class="text-xs mt-2" onclick={() => isSheetOpen = true}>
				{isAr ? "تسجيل دفعة جديدة" : "Record Payment"}
			</Button>
		</div>
	{:else}
		<!-- Data Table -->
		<div class="border border-border rounded-xl bg-card overflow-hidden text-start" in:fade={{ duration: 150 }}>
			<Table>
				<TableHeader>
					<TableRow class="hover:bg-transparent">
						<TableHead class="text-start">{isAr ? "رقم الإيصال" : "Receipt #"}</TableHead>
						<TableHead class="text-start">{isAr ? "الطالب" : "Student"}</TableHead>
						<TableHead class="text-start">{isAr ? "طريقة الدفع" : "Method"}</TableHead>
						<TableHead class="text-start">{isAr ? "تاريخ الدفع" : "Payment Date"}</TableHead>
						<TableHead class="text-start">{isAr ? "الخزينة المستلمة" : "Treasury Safe"}</TableHead>
						<TableHead class="text-start">{isAr ? "المبلغ المدفوع" : "Amount Paid"}</TableHead>
					</TableRow>
				</TableHeader>
				<TableBody>
					{#each filteredPayments as pay (pay.id)}
						<TableRow class="hover:bg-accent/10">
							<TableCell class="font-bold text-foreground font-mono text-start">
								#{pay.receipt_number || pay.id.slice(0, 8)}
							</TableCell>
							<TableCell class="font-medium text-foreground">
								<div class="flex flex-col">
									<span>{pay.student_name || pay.enrollment_id || "-"}</span>
									{#if pay.student_number}
										<span class="text-[8px] text-muted-foreground font-mono">#{pay.student_number}</span>
									{/if}
								</div>
							</TableCell>
							<TableCell>
								<span class="px-2 py-0.5 rounded-full border text-[8px] font-bold uppercase tracking-wider {getMethodBadgeClass(pay.payment_method || 'Cash')}">
									{getMethodLabel(pay.payment_method || 'Cash')}
								</span>
							</TableCell>
							<TableCell class="font-mono text-muted-foreground">
								<span class="flex items-center gap-1">
									<Calendar size={10} />
									<span>{formatDate(pay.payment_date || pay.created_at)}</span>
								</span>
							</TableCell>
							<TableCell class="text-muted-foreground">
								{pay.treasury_name || pay.treasury_id || "-"}
							</TableCell>
							<TableCell class="font-bold font-mono text-emerald-500">{formatCurrency(Number(pay.amount || 0), pay.currency)}</TableCell>
						</TableRow>
					{/each}
				</TableBody>
			</Table>
		</div>
	{/if}
</div>

<!-- Modal Form Dialog -->
<PaymentRecordSheet 
	bind:open={isSheetOpen}
	orgId={data.orgId}
	onSuccess={loadPayments}
/>
