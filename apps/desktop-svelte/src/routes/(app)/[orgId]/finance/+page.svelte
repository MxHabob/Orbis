<!--
@component FinanceOverviewPage
Main dashboard for the organization's financial summary.
Displays KPIs (total fees, collections, balance due, treasury assets),
collection percentage progress, and quick links to submodules.
-->
<script lang="ts">
import { onMount } from "svelte";
import { commands } from "$lib/core/ipc";
import { Card } from "@orbis/ui-svelte";
import { Button } from "@orbis/ui-svelte/button";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import {
  Wallet,
  CreditCard,
  Landmark,
  BarChart3,
  ArrowRight,
  ShieldCheck,
  TrendingUp,
  HelpCircle,
} from "lucide-svelte";
import { getLocale } from "$lib/paraglide/runtime";
import { formatCurrency } from "$lib/core/i18n";
import { fade } from "svelte/transition";
import * as m from "$lib/paraglide/messages";

let { data }: { data: { orgId: string } } = $props();

// State
let isLoading = $state(true);
let totalFees = $state(0);
let totalPaid = $state(0);
let treasuryAssets = $state(0);
let error = $state<string | null>(null);

const balanceDue = $derived(Math.max(0, totalFees - totalPaid));
const collectionRate = $derived(
  totalFees > 0 ? Math.round((totalPaid / totalFees) * 100) : 0,
);

async function loadOverview() {
  isLoading = true;
  error = null;
  try {
    const [fees, payments, treasuries] = await Promise.all([
      commands.finance.getStudentFees(data.orgId),
      commands.finance.getPayments(data.orgId),
      commands.finance.getTreasuries(data.orgId),
    ]);

    totalFees = fees.reduce((sum, f) => sum + Number(f.amount || 0), 0);
    totalPaid = payments.reduce((sum, p) => sum + Number(p.amount || 0), 0);
    treasuryAssets = treasuries.reduce(
      (sum, t) => sum + Number(t.balance || 0),
      0,
    );
  } catch (e) {
    console.error("Error loading finance overview:", e);
    error = e instanceof Error ? e.message : String(e);
  } finally {
    isLoading = false;
  }
}

onMount(() => {
  if (data.orgId) {
    loadOverview();
  }
});
</script>

<div class="h-full w-full overflow-y-auto p-6 flex flex-col gap-6 font-sans">
	<!-- Welcome / Header Banner -->
	<div class="bg-gradient-to-r from-primary/10 via-primary/5 to-transparent border border-primary/20 rounded-xl p-6 select-none text-start">
		<div class="flex items-center gap-3">
			<div class="h-10 w-10 rounded-lg bg-primary/20 flex items-center justify-center text-primary">
				<Landmark size={20} />
			</div>
			<div class="flex flex-col gap-0.5">
				<h1 class="text-sm font-bold text-foreground uppercase tracking-wider">
					{m.finance_overview_title()}
				</h1>
				<p class="text-[10px] text-muted-foreground">
					{m.finance_overview_subtitle()}
				</p>
			</div>
		</div>
	</div>

	{#if isLoading}
		<!-- Loading state skeletons -->
		<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
			{#each Array(4) as _}
				<Skeleton class="h-24 w-full rounded-xl" />
			{/each}
		</div>
		<div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
			<Skeleton class="h-64 lg:col-span-2 rounded-xl" />
			<Skeleton class="h-64 rounded-xl" />
		</div>
	{:else if error}
		<div class="flex flex-col items-center justify-center py-16 text-destructive border border-destructive/20 bg-destructive/5 rounded-xl gap-2">
			<h3 class="font-semibold text-xs">{m.finance_overview_error()}</h3>
			<p class="text-[10px] text-muted-foreground">{error}</p>
			<Button variant="outline" onclick={loadOverview} class="text-[10px] mt-2">{m.error_retry()}</Button>
		</div>
	{:else}
		<!-- Main Content Grid -->
		<div class="flex flex-col gap-6" in:fade={{ duration: 150 }}>
			<!-- Metric Cards -->
			<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
				<!-- Total Invoiced -->
				<div class="card-tactile bg-card/40 flex items-center justify-between p-4 h-24 select-none">
					<div class="flex flex-col gap-1 text-start">
						<span class="text-[9px] font-bold text-muted-foreground uppercase tracking-wider">
							{m.finance_overview_total_invoiced()}
						</span>
						<span class="text-xl font-bold font-mono text-foreground">{formatCurrency(totalFees)}</span>
					</div>
					<div class="h-9 w-9 rounded-lg bg-primary/10 flex items-center justify-center text-primary">
						<Wallet size={16} />
					</div>
				</div>

				<!-- Total Paid/Collected -->
				<div class="card-tactile bg-card/40 flex items-center justify-between p-4 h-24 select-none">
					<div class="flex flex-col gap-1 text-start">
						<span class="text-[9px] font-bold text-muted-foreground uppercase tracking-wider">
							{m.finance_overview_total_collected()}
						</span>
						<span class="text-xl font-bold font-mono text-emerald-500">{formatCurrency(totalPaid)}</span>
					</div>
					<div class="h-9 w-9 rounded-lg bg-emerald-500/10 flex items-center justify-center text-emerald-500">
						<CreditCard size={16} />
					</div>
				</div>

				<!-- Balance Due -->
				<div class="card-tactile bg-card/40 flex items-center justify-between p-4 h-24 select-none">
					<div class="flex flex-col gap-1 text-start">
						<span class="text-[9px] font-bold text-muted-foreground uppercase tracking-wider">
							{m.finance_overview_outstanding()}
						</span>
						<span class="text-xl font-bold font-mono text-rose-500">{formatCurrency(balanceDue)}</span>
					</div>
					<div class="h-9 w-9 rounded-lg bg-rose-500/10 flex items-center justify-center text-rose-500">
						<HelpCircle size={16} />
					</div>
				</div>

				<!-- Treasury Balance -->
				<div class="card-tactile bg-card/40 flex items-center justify-between p-4 h-24 select-none">
					<div class="flex flex-col gap-1 text-start">
						<span class="text-[9px] font-bold text-muted-foreground uppercase tracking-wider">
							{m.finance_overview_treasury_bal()}
						</span>
						<span class="text-xl font-bold font-mono text-foreground">{formatCurrency(treasuryAssets)}</span>
					</div>
					<div class="h-9 w-9 rounded-lg bg-brand-lime/10 flex items-center justify-center text-brand-lime">
						<Landmark size={16} />
					</div>
				</div>
			</div>

			<!-- Visual Distribution Summary -->
			<div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
				<!-- Collection progress bar -->
				<div class="card-tactile bg-card lg:col-span-2 p-6 flex flex-col gap-4 text-start">
					<h3 class="text-xs font-bold text-foreground uppercase tracking-wider">
						{m.finance_overview_performance_title()}
					</h3>
					
					<div class="flex items-end justify-between mt-2 select-none">
						<div class="flex flex-col">
							<span class="text-[9px] text-muted-foreground">{m.finance_overview_ratio()}</span>
							<span class="text-2xl font-bold font-mono text-emerald-500">{collectionRate}%</span>
						</div>
						<div class="flex flex-col text-end font-mono">
							<span class="text-[9px] text-muted-foreground">{m.finance_overview_target()}</span>
							<span class="text-xs font-semibold">100%</span>
						</div>
					</div>

					<!-- Visual Slider Progress -->
					<div class="w-full h-3 bg-muted rounded-full overflow-hidden relative border border-border/80 select-none">
						<div class="h-full bg-emerald-500 rounded-full transition-all duration-300" style="width: {collectionRate}%"></div>
					</div>

					<div class="flex items-center gap-3 pt-2 text-[9px] text-muted-foreground border-t border-border/50 mt-2 select-none">
						<span class="flex items-center gap-1">
							<span class="h-2 w-2 rounded-full bg-emerald-500"></span>
							{m.finance_overview_collected()}: {formatCurrency(totalPaid)}
						</span>
						<span class="flex items-center gap-1">
							<span class="h-2 w-2 rounded-full bg-rose-500"></span>
							{m.finance_overview_outstanding_label()}: {formatCurrency(balanceDue)}
						</span>
					</div>
				</div>

				<!-- Quick shortcuts / health summary -->
				<div class="card-tactile bg-card p-6 flex flex-col gap-4 text-start">
					<h3 class="text-xs font-bold text-foreground uppercase tracking-wider">
						{m.finance_overview_diagnostics_title()}
					</h3>

					<div class="flex flex-col gap-3 my-auto">
						<div class="flex items-start gap-2.5">
							<ShieldCheck size={14} class="text-emerald-500 shrink-0 mt-0.5" />
							<div class="flex flex-col">
								<span class="text-[10px] font-semibold text-foreground">{m.finance_overview_diag1_title()}</span>
								<span class="text-[8px] text-muted-foreground">{m.finance_overview_diag1_desc()}</span>
							</div>
						</div>

						<div class="flex items-start gap-2.5">
							<TrendingUp size={14} class="text-primary shrink-0 mt-0.5" />
							<div class="flex flex-col">
								<span class="text-[10px] font-semibold text-foreground">{m.finance_overview_diag2_title()}</span>
								<span class="text-[8px] text-muted-foreground">{m.finance_overview_diag2_desc()}</span>
							</div>
						</div>
					</div>
				</div>
			</div>
		</div>
	{/if}
</div>
