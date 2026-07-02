<!--
@component StudentFeesPage
Renders assigned fees, payment receipts history, and balance due summary.
-->
<script lang="ts">
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
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import {
  CreditCard,
  Wallet,
  Calendar,
  CheckCircle2,
  AlertCircle,
} from "lucide-svelte";
import { onMount } from "svelte";
import * as m from "$lib/paraglide/messages";
import { formatCurrency, formatDate } from "$lib/i18n-utils";

const orgId = $derived(page.params.orgId);
const studentId = $derived(page.params.studentId);

let fees = $state<any[]>([]);
let payments = $state<any[]>([]);
let isLoading = $state(true);
let error = $state<string | null>(null);

let totalFees = $derived(
  fees.reduce((sum, f) => sum + Number(f.total_amount || 0), 0),
);
let totalPaid = $derived(
  payments.reduce((sum, p) => sum + Number(p.amount || 0), 0),
);
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
      commands.finance.getPayments(orgId, studentId),
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

<div class="flex flex-col gap-6 text-xs text-start">
	<!-- Summary Statistics cards -->
	<div class="grid grid-cols-3 gap-3 shrink-0">
		<div class="border border-border bg-card/45 p-4 rounded-xl flex items-center justify-between shadow-sm">
			<div class="flex flex-col">
				<span class="text-[9px] text-muted-foreground">{m.fees_total_fees()}</span>
				<span class="text-sm font-bold mt-0.5 text-foreground">{formatCurrency(totalFees)}</span>
			</div>
			<Wallet size={16} class="text-primary" />
		</div>
		<div class="border border-border bg-card/45 p-4 rounded-xl flex items-center justify-between shadow-sm">
			<div class="flex flex-col">
				<span class="text-[9px] text-muted-foreground">{m.fees_total_paid()}</span>
				<span class="text-sm font-bold mt-0.5 text-emerald-500">{formatCurrency(totalPaid)}</span>
			</div>
			<CheckCircle2 size={16} class="text-emerald-500" />
		</div>
		<div class="border border-border bg-card/45 p-4 rounded-xl flex items-center justify-between shadow-sm">
			<div class="flex flex-col">
				<span class="text-[9px] text-muted-foreground">{m.fees_balance_due()}</span>
				<span class="text-sm font-bold mt-0.5" class:text-destructive={balanceDue > 0} class:text-emerald-500={balanceDue <= 0}>
					{formatCurrency(balanceDue)}
				</span>
			</div>
			<AlertCircle size={16} class={balanceDue > 0 ? 'text-destructive' : 'text-emerald-500'} />
		</div>
	</div>

	{#if isLoading}
		<div class="flex flex-col gap-3">
			<Skeleton class="h-12 w-full" />
			<Skeleton class="h-12 w-full" />
		</div>
	{:else if error}
		<div class="flex flex-col items-center justify-center p-8 border border-destructive/20 bg-destructive/5 rounded-xl text-destructive text-center gap-2">
			<h3 class="font-semibold text-xs">{m.fees_error_title()}</h3>
			<p class="text-[10px] text-muted-foreground">{error}</p>
			<Button variant="link" onclick={loadFinance} class="text-[10px] underline cursor-pointer mt-2 hover:text-foreground">{m.error_retry()}</Button>
		</div>
	{:else}
		<!-- Assigned Fees List -->
		<div class="flex flex-col gap-2">
			<span class="font-bold text-[10px] text-muted-foreground uppercase tracking-wider px-1">{m.fees_assigned_title()}</span>
			
			{#if fees.length === 0}
				<div class="border border-dashed border-border rounded-lg p-6 text-center text-muted-foreground bg-card/5">
					{m.fees_no_fees()}
				</div>
			{:else}
				<div class="border border-border rounded-lg bg-card overflow-hidden">
					<Table>
						<TableHeader>
							<TableRow class="hover:bg-transparent">
								<TableHead class="text-start">{m.fees_col_description()}</TableHead>
								<TableHead class="text-start">{m.fees_col_ledger()}</TableHead>
								<TableHead class="text-start">{m.fees_col_date()}</TableHead>
								<TableHead class="text-start">{m.fees_col_amount()}</TableHead>
							</TableRow>
						</TableHeader>
						<TableBody>
							{#each fees as fee (fee.id)}
								<TableRow class="hover:bg-accent/10">
									<TableCell class="font-bold text-foreground text-start">{fee.notes || m.fees_default_description()}</TableCell>
									<TableCell class="text-muted-foreground text-start">{fee.ledger_account_name || fee.ledger_account_id || '-'}</TableCell>
									<TableCell class="font-mono text-muted-foreground text-start">
										{formatDate(fee.created_at) || '-'}
									</TableCell>
									<TableCell class="font-bold font-mono text-start">{formatCurrency(Number(fee.total_amount || 0))}</TableCell>
								</TableRow>
							{/each}
						</TableBody>
					</Table>
				</div>
			{/if}
		</div>

		<!-- Payments History -->
		<div class="flex flex-col gap-2">
			<span class="font-bold text-[10px] text-muted-foreground uppercase tracking-wider px-1">{m.fees_payments_title()}</span>

			{#if payments.length === 0}
				<div class="border border-dashed border-border rounded-lg p-6 text-center text-muted-foreground bg-card/5">
					{m.fees_no_payments()}
				</div>
			{:else}
				<div class="border border-border rounded-lg bg-card overflow-hidden">
					<Table>
						<TableHeader>
							<TableRow class="hover:bg-transparent">
								<TableHead class="text-start">{m.fees_col_receipt()}</TableHead>
								<TableHead class="text-start">{m.fees_col_method()}</TableHead>
								<TableHead class="text-start">{m.fees_col_pay_date()}</TableHead>
								<TableHead class="text-start">{m.fees_col_paid_amount()}</TableHead>
							</TableRow>
						</TableHeader>
						<TableBody>
							{#each payments as pay (pay.id)}
								<TableRow class="hover:bg-accent/10">
									<TableCell class="font-bold text-foreground font-mono text-start">#{pay.receipt_number || pay.id.slice(0, 8)}</TableCell>
									<TableCell class="text-muted-foreground text-start">{pay.method || m.fees_default_method()}</TableCell>
									<TableCell class="font-mono text-muted-foreground text-start">
										{formatDate(pay.created_at) || '-'}
									</TableCell>
									<TableCell class="font-bold text-emerald-500 font-mono text-start">{formatCurrency(Number(pay.amount || 0))}</TableCell>
								</TableRow>
							{/each}
						</TableBody>
					</Table>
				</div>
			{/if}
		</div>
	{/if}
</div>

