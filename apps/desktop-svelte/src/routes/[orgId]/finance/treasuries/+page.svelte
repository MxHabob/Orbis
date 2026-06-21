<!--
@component TreasuriesManagementPage
A split administration page listing both:
1. Treasuries (cash safes, bank vaults) with their live balances.
2. Ledger Accounts (account chart classifications) with their codes and types.
Triggers TreasuryCreateSheet and LedgerCreateSheet.
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
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import {
  Plus,
  Landmark,
  Wallet,
  Layers,
  ShieldAlert,
  Award,
} from "lucide-svelte";
import { getLocale } from "$lib/paraglide/runtime";
import { formatCurrency } from "$lib/i18n-utils";
import { fade } from "svelte/transition";
import TreasuryCreateSheet from "$lib/components/finance/treasury-create-sheet.svelte";
import LedgerCreateSheet from "$lib/components/finance/ledger-create-sheet.svelte";
import * as m from "$lib/paraglide/messages";

let { data }: { data: { orgId: string } } = $props();

// State
let treasuries = $state<any[]>([]);
let ledgers = $state<any[]>([]);
let isLoading = $state(true);
let error = $state<string | null>(null);

// Sheets triggers
let isTreasurySheetOpen = $state(false);
let isLedgerSheetOpen = $state(false);

async function loadData() {
  isLoading = true;
  error = null;
  try {
    const [treasuryList, ledgerList] = await Promise.all([
      commands.finance.getTreasuries(data.orgId),
      commands.finance.getLedgerAccounts(data.orgId),
    ]);
    treasuries = treasuryList;
    ledgers = ledgerList;
  } catch (e) {
    console.error("Error loading treasuries and ledgers:", e);
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

// Calculations
const totalTreasuryBalance = $derived(
  treasuries.reduce((sum, t) => sum + Number(t.balance || 0), 0),
);

function getLedgerTypeLabel(typeStr: string) {
  const t = String(typeStr).toLowerCase();
  if (t === "revenue") return m.finance_type_revenue();
  if (t === "expense") return m.finance_type_expense();
  if (t === "asset") return m.finance_type_asset();
  if (t === "liability") return m.finance_type_liability();
  return typeStr;
}

function getLedgerTypeBadgeClass(typeStr: string) {
  const t = String(typeStr).toLowerCase();
  if (t === "revenue")
    return "bg-emerald-500/10 text-emerald-500 border-emerald-500/20";
  if (t === "expense") return "bg-rose-500/10 text-rose-500 border-rose-500/20";
  if (t === "asset") return "bg-blue-500/10 text-blue-500 border-blue-500/20";
  return "bg-purple-500/10 text-purple-500 border-purple-500/20";
}

function getTreasuryTypeLabel(typeStr: string) {
  const t = String(typeStr).toLowerCase();
  if (t === "bank") return m.treasury_type_bank();
  return m.treasury_type_cash();
}
</script>

<div class="h-full w-full overflow-y-auto p-6 flex flex-col gap-6 font-sans">
	<!-- Header Title -->
	<div class="text-start select-none">
		<h2 class="text-sm font-bold text-foreground uppercase tracking-wider">
			{m.finance_page_title()}
		</h2>
		<p class="text-[10px] text-muted-foreground mt-0.5">
			{m.finance_page_subtitle()}
		</p>
	</div>

	{#if isLoading}
		<!-- Loading state Skeletons -->
		<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
			<Skeleton class="h-80 w-full rounded-xl" />
			<Skeleton class="h-80 w-full rounded-xl" />
		</div>
	{:else if error}
		<div class="flex flex-col items-center justify-center py-16 text-destructive border border-destructive/20 bg-destructive/5 rounded-xl gap-2">
			<h3 class="font-semibold text-xs">{m.finance_error_loading()}</h3>
			<p class="text-[10px] text-muted-foreground">{error}</p>
			<Button variant="outline" onclick={loadData} class="text-[10px] mt-2">{m.error_retry()}</Button>
		</div>
	{:else}
		<div class="grid grid-cols-1 lg:grid-cols-2 gap-6" in:fade={{ duration: 150 }}>
			<!-- Treasuries Section (Bank & Cash) -->
			<div class="card-tactile bg-card/45 p-5 flex flex-col gap-4 text-start">
				<div class="flex items-center justify-between border-b border-border/60 pb-3 select-none">
					<div class="flex flex-col">
						<h3 class="text-xs font-bold text-foreground uppercase tracking-wider flex items-center gap-1.5">
							<Landmark size={13} class="text-primary" />
							<span>{m.finance_vaults_title()}</span>
						</h3>
						<span class="text-[8px] text-muted-foreground mt-0.5">
							{m.finance_total_cash_assets({ balance: formatCurrency(totalTreasuryBalance, "YER") })}
						</span>
					</div>
					
					<Button 
						variant="outline" 
						class="text-[10px] h-7 px-2"
						onclick={() => isTreasurySheetOpen = true}
					>
						<Plus size={12} class={getLocale() === "ar" ? 'ml-1' : 'mr-1'} />
						{m.finance_new_treasury_button()}
					</Button>
				</div>

				{#if treasuries.length === 0}
					<div class="flex-1 flex flex-col items-center justify-center py-12 text-muted-foreground border border-dashed border-border rounded-lg bg-card/10 select-none">
						<Landmark size={32} class="text-muted-foreground/30 mb-2" />
						<span class="text-[10px]">{m.finance_no_treasuries()}</span>
					</div>
				{:else}
					<div class="overflow-x-auto">
						<Table>
							<TableHeader>
								<TableRow class="hover:bg-transparent">
									<TableHead class="text-start">{m.finance_col_code()}</TableHead>
									<TableHead class="text-start">{m.finance_col_name()}</TableHead>
									<TableHead class="text-start">{m.finance_col_category()}</TableHead>
									<TableHead class="text-start">{m.finance_col_balance()}</TableHead>
								</TableRow>
							</TableHeader>
							<TableBody>
								{#each treasuries as t (t.id)}
									<TableRow class="hover:bg-accent/5">
										<TableCell class="font-mono font-bold text-foreground">{t.code}</TableCell>
										<TableCell class="text-foreground">{t.name_ar || t.name}</TableCell>
										<TableCell class="text-muted-foreground text-[10px]">{getTreasuryTypeLabel(t.treasury_type)}</TableCell>
										<TableCell class="font-bold font-mono text-emerald-500">{formatCurrency(Number(t.balance || 0), t.currency)}</TableCell>
									</TableRow>
								{/each}
							</TableBody>
						</Table>
					</div>
				{/if}
			</div>

			<!-- Ledger Accounts Section -->
			<div class="card-tactile bg-card/45 p-5 flex flex-col gap-4 text-start">
				<div class="flex items-center justify-between border-b border-border/60 pb-3 select-none">
					<div class="flex flex-col">
						<h3 class="text-xs font-bold text-foreground uppercase tracking-wider flex items-center gap-1.5">
							<Layers size={13} class="text-primary" />
							<span>{m.finance_ledger_chart_title()}</span>
						</h3>
						<span class="text-[8px] text-muted-foreground mt-0.5">
							{m.finance_ledger_accounts_count({ count: ledgers.length })}
						</span>
					</div>
					
					<Button 
						variant="outline" 
						class="text-[10px] h-7 px-2"
						onclick={() => isLedgerSheetOpen = true}
					>
						<Plus size={12} class={getLocale() === "ar" ? 'ml-1' : 'mr-1'} />
						{m.finance_new_ledger_button()}
					</Button>
				</div>

				{#if ledgers.length === 0}
					<div class="flex-1 flex flex-col items-center justify-center py-12 text-muted-foreground border border-dashed border-border rounded-lg bg-card/10 select-none">
						<Layers size={32} class="text-muted-foreground/30 mb-2" />
						<span class="text-[10px]">{m.finance_no_ledgers()}</span>
					</div>
				{:else}
					<div class="overflow-x-auto">
						<Table>
							<TableHeader>
								<TableRow class="hover:bg-transparent">
									<TableHead class="text-start">{m.finance_col_code()}</TableHead>
									<TableHead class="text-start">{m.finance_col_name()}</TableHead>
									<TableHead class="text-start">{m.finance_col_type()}</TableHead>
								</TableRow>
							</TableHeader>
							<TableBody>
								{#each ledgers as l (l.id)}
									<TableRow class="hover:bg-accent/5">
										<TableCell class="font-mono font-bold text-foreground">{l.code}</TableCell>
										<TableCell class="text-foreground">{l.name_ar || l.name}</TableCell>
										<TableCell>
											<span class="px-1.5 py-0.5 rounded border text-[8px] font-bold uppercase tracking-wider {getLedgerTypeBadgeClass(l.account_type)}">
												{getLedgerTypeLabel(l.account_type)}
											</span>
										</TableCell>
									</TableRow>
								{/each}
							</TableBody>
						</Table>
					</div>
				{/if}
			</div>
		</div>
	{/if}
</div>

<!-- Modal Sheets -->
<TreasuryCreateSheet 
	bind:open={isTreasurySheetOpen}
	orgId={data.orgId}
	onSuccess={loadData}
/>

<LedgerCreateSheet 
	bind:open={isLedgerSheetOpen}
	orgId={data.orgId}
	onSuccess={loadData}
/>
