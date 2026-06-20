<!--
@component StudentFeesPage
Renders assigned fees, payment receipts history, and balance due summary.
-->
<script lang="ts">
import { page } from "$app/state";
import { commands } from "$lib/services/ipc";
import { 
  Table, TableBody, TableCell, TableHead, TableHeader, TableRow 
} from "@orbis/ui-svelte";
import { CreditCard, Wallet, Calendar, CheckCircle2, AlertCircle } from "lucide-svelte";
import { onMount } from "svelte";

const orgId = $derived(page.params.orgId);
const studentId = $derived(page.params.studentId);

let fees = $state<any[]>([]);
let payments = $state<any[]>([]);
let isLoading = $state(true);
let error = $state<string | null>(null);

let totalFees = $derived(fees.reduce((sum, f) => sum + Number(f.amount || 0), 0));
let totalPaid = $derived(payments.reduce((sum, p) => sum + Number(p.amount || 0), 0));
let balanceDue = $derived(totalFees - totalPaid);

onMount(() => {
  loadFinance();
});

async function loadFinance() {
  isLoading = true;
  error = null;
  try {
    if (!orgId || !studentId) return;
    const [feesRes, paymentsRes] = await Promise.all([
      commands.finance.getStudentFees(orgId, studentId),
      commands.finance.getPayments(orgId, studentId)
    ]);
    fees = feesRes;
    payments = paymentsRes;
  } catch (e) {
    error = e instanceof Error ? e.message : String(e);
  } finally {
    isLoading = false;
  }
}
</script>

<div class="flex flex-col gap-6 text-xs text-right">
	<!-- Summary Statistics cards -->
	<div class="grid grid-cols-3 gap-3 shrink-0">
		<div class="border border-border bg-card/45 p-4 rounded-xl flex items-center justify-between shadow-sm">
			<div class="flex flex-col">
				<span class="text-[9px] text-muted-foreground">إجمالي الرسوم المترتبة</span>
				<span class="text-sm font-bold mt-0.5 text-foreground">{totalFees.toLocaleString()} ر.س</span>
			</div>
			<Wallet size={16} class="text-primary" />
		</div>
		<div class="border border-border bg-card/45 p-4 rounded-xl flex items-center justify-between shadow-sm">
			<div class="flex flex-col">
				<span class="text-[9px] text-muted-foreground">إجمالي المبالغ المدفوعة</span>
				<span class="text-sm font-bold mt-0.5 text-emerald-500">{totalPaid.toLocaleString()} ر.س</span>
			</div>
			<CheckCircle2 size={16} class="text-emerald-500" />
		</div>
		<div class="border border-border bg-card/45 p-4 rounded-xl flex items-center justify-between shadow-sm">
			<div class="flex flex-col">
				<span class="text-[9px] text-muted-foreground">الرصيد المستحق (المتبقي)</span>
				<span class="text-sm font-bold mt-0.5" class:text-destructive={balanceDue > 0} class:text-emerald-500={balanceDue <= 0}>
					{balanceDue.toLocaleString()} ر.س
				</span>
			</div>
			<AlertCircle size={16} class={balanceDue > 0 ? 'text-destructive' : 'text-emerald-500'} />
		</div>
	</div>

	{#if isLoading}
		<div class="flex flex-col gap-3">
			<div class="skeleton h-12 w-full"></div>
			<div class="skeleton h-12 w-full"></div>
		</div>
	{:else if error}
		<div class="flex flex-col items-center justify-center p-8 border border-destructive/20 bg-destructive/5 rounded-xl text-destructive text-center gap-2">
			<h3 class="font-semibold text-xs">فشل تحميل التفاصيل المالية</h3>
			<p class="text-[10px] text-muted-foreground">{error}</p>
			<button onclick={loadFinance} class="text-[10px] underline cursor-pointer mt-2 hover:text-foreground">إعادة المحاولة</button>
		</div>
	{:else}
		<!-- Assigned Fees List -->
		<div class="flex flex-col gap-2">
			<span class="font-bold text-[10px] text-muted-foreground uppercase tracking-wider px-1">الرسوم المستحقة المقيدة</span>
			
			{#if fees.length === 0}
				<div class="border border-dashed border-border rounded-lg p-6 text-center text-muted-foreground bg-card/5">
					لا توجد رسوم مسجلة على هذا الطالب حالياً.
				</div>
			{:else}
				<div class="border border-border rounded-lg bg-card overflow-hidden">
					<Table>
						<TableHeader>
							<TableRow class="hover:bg-transparent">
								<TableHead class="text-right">وصف الرسم</TableHead>
								<TableHead class="text-right">الحساب المالي</TableHead>
								<TableHead class="text-right">التاريخ المقيد</TableHead>
								<TableHead class="text-right">المبلغ</TableHead>
							</TableRow>
						</TableHeader>
						<TableBody>
							{#each fees as fee (fee.id)}
								<TableRow class="hover:bg-accent/10">
									<TableCell class="font-bold text-foreground">{fee.description || 'رسوم دراسية'}</TableCell>
									<TableCell class="text-muted-foreground">{fee.ledger_account_name || fee.ledger_account_id || '-'}</TableCell>
									<TableCell class="font-mono text-muted-foreground">
										{fee.created_at ? new Date(String(fee.created_at)).toLocaleDateString('ar-SA') : '-'}
									</TableCell>
									<TableCell class="font-bold font-mono">{(Number(fee.amount || 0)).toLocaleString()} ر.س</TableCell>
								</TableRow>
							{/each}
						</TableBody>
					</Table>
				</div>
			{/if}
		</div>

		<!-- Payments History -->
		<div class="flex flex-col gap-2">
			<span class="font-bold text-[10px] text-muted-foreground uppercase tracking-wider px-1">سجل سندات القبض (المدفوعات)</span>

			{#if payments.length === 0}
				<div class="border border-dashed border-border rounded-lg p-6 text-center text-muted-foreground bg-card/5">
					لم يتم سداد أي مبالغ حتى الآن.
				</div>
			{:else}
				<div class="border border-border rounded-lg bg-card overflow-hidden">
					<Table>
						<TableHeader>
							<TableRow class="hover:bg-transparent">
								<TableHead class="text-right">رقم السند</TableHead>
								<TableHead class="text-right">طريقة السداد</TableHead>
								<TableHead class="text-right">تاريخ الدفع</TableHead>
								<TableHead class="text-right">المبلغ المدفوع</TableHead>
							</TableRow>
						</TableHeader>
						<TableBody>
							{#each payments as pay (pay.id)}
								<TableRow class="hover:bg-accent/10">
									<TableCell class="font-bold text-foreground font-mono">#{pay.receipt_number || pay.id.slice(0, 8)}</TableCell>
									<TableCell class="text-muted-foreground">{pay.payment_method || 'نقدي'}</TableCell>
									<TableCell class="font-mono text-muted-foreground">
										{pay.created_at ? new Date(String(pay.created_at)).toLocaleDateString('ar-SA') : '-'}
									</TableCell>
									<TableCell class="font-bold text-emerald-500 font-mono">{(Number(pay.amount || 0)).toLocaleString()} ر.س</TableCell>
								</TableRow>
							{/each}
						</TableBody>
					</Table>
				</div>
			{/if}
		</div>
	{/if}
</div>
