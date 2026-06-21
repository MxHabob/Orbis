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
  Table, TableBody, TableCell, TableHead, TableHeader, TableRow 
} from "@orbis/ui-svelte";
import { Button } from "@orbis/ui-svelte/button";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import { Plus, Landmark, Wallet, Layers, ShieldAlert, Award } from "lucide-svelte";
import { getLocale } from "$lib/paraglide/runtime";
import { formatCurrency } from "$lib/i18n-utils";
import { fade } from "svelte/transition";
import TreasuryCreateSheet from "$lib/components/finance/treasury-create-sheet.svelte";
import LedgerCreateSheet from "$lib/components/finance/ledger-create-sheet.svelte";

let { data }: { data: { orgId: string } } = $props();

const isAr = $derived(getLocale() === "ar");

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
  treasuries.reduce((sum, t) => sum + Number(t.balance || 0), 0)
);

function getLedgerTypeLabel(typeStr: string) {
  const t = String(typeStr).toLowerCase();
  if (t === "revenue") return isAr ? "إيرادات" : "Revenue";
  if (t === "expense") return isAr ? "مصروفات" : "Expense";
  if (t === "asset") return isAr ? "أصول" : "Asset";
  if (t === "liability") return isAr ? "التزامات" : "Liability";
  return typeStr;
}

function getLedgerTypeBadgeClass(typeStr: string) {
  const t = String(typeStr).toLowerCase();
  if (t === "revenue") return "bg-emerald-500/10 text-emerald-500 border-emerald-500/20";
  if (t === "expense") return "bg-rose-500/10 text-rose-500 border-rose-500/20";
  if (t === "asset") return "bg-blue-500/10 text-blue-500 border-blue-500/20";
  return "bg-purple-500/10 text-purple-500 border-purple-500/20";
}

function getTreasuryTypeLabel(typeStr: string) {
  const t = String(typeStr).toLowerCase();
  if (t === "bank") return isAr ? "حساب مصرفي" : "Bank Account";
  return isAr ? "صندوق كاش" : "Physical Cashbox";
}
</script>

<div class="h-full w-full overflow-y-auto p-6 flex flex-col gap-6 font-sans">
	<!-- Header Title -->
	<div class="text-start select-none">
		<h2 class="text-sm font-bold text-foreground uppercase tracking-wider">
			{isAr ? "إدارة الخزائن والحسابات الدفترية" : "Treasuries & Ledger Accounts"}
		</h2>
		<p class="text-[10px] text-muted-foreground mt-0.5">
			{isAr ? "تهيئة الصناديق وحسابات تصنيف الرسوم لتنظيم تدفق الموارد المالية." : "Configure money vaults, bank destinations, and accounting code definitions."}
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
			<h3 class="font-semibold text-xs">{isAr ? "خطأ في تحميل الحسابات" : "Error loading financial records"}</h3>
			<p class="text-[10px] text-muted-foreground">{error}</p>
			<Button variant="outline" onclick={loadData} class="text-[10px] mt-2">{isAr ? "إعادة المحاولة" : "Retry"}</Button>
		</div>
	{:else}
		<div class="grid grid-cols-1 lg:grid-cols-2 gap-6" in:fade={{ duration: 150 }}>
			<!-- Treasuries Section (Bank & Cash) -->
			<div class="card-tactile bg-card/45 p-5 flex flex-col gap-4 text-start">
				<div class="flex items-center justify-between border-b border-border/60 pb-3 select-none">
					<div class="flex flex-col">
						<h3 class="text-xs font-bold text-foreground uppercase tracking-wider flex items-center gap-1.5">
							<Landmark size={13} class="text-primary" />
							<span>{isAr ? "خزائن المدرسة والبنوك" : "Treasury Vaults"}</span>
						</h3>
						<span class="text-[8px] text-muted-foreground mt-0.5">
							{isAr 
								? `إجمالي الرصيد المتوفر: ${formatCurrency(totalTreasuryBalance, "YER")}` 
								: `Total Cash Assets: ${formatCurrency(totalTreasuryBalance, "YER")}`}
						</span>
					</div>
					
					<Button 
						variant="outline" 
						class="text-[10px] h-7 px-2"
						onclick={() => isTreasurySheetOpen = true}
					>
						<Plus size={12} class={isAr ? 'ml-1' : 'mr-1'} />
						{isAr ? "خزينة جديدة" : "New Treasury"}
					</Button>
				</div>

				{#if treasuries.length === 0}
					<div class="flex-1 flex flex-col items-center justify-center py-12 text-muted-foreground border border-dashed border-border rounded-lg bg-card/10 select-none">
						<Landmark size={32} class="text-muted-foreground/30 mb-2" />
						<span class="text-[10px]">{isAr ? "لا توجد صناديق مضافة" : "No treasuries created yet."}</span>
					</div>
				{:else}
					<div class="overflow-x-auto">
						<Table>
							<TableHeader>
								<TableRow class="hover:bg-transparent">
									<TableHead class="text-start">{isAr ? "الرمز" : "Code"}</TableHead>
									<TableHead class="text-start">{isAr ? "الاسم" : "Name"}</TableHead>
									<TableHead class="text-start">{isAr ? "النوع" : "Category"}</TableHead>
									<TableHead class="text-start">{isAr ? "الرصيد المتوفر" : "Current Balance"}</TableHead>
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
							<span>{isAr ? "دليل الحسابات الفئوية" : "Ledger Accounts Chart"}</span>
						</h3>
						<span class="text-[8px] text-muted-foreground mt-0.5">
							{isAr ? `${ledgers.length} بند مالي معرف` : `${ledgers.length} financial ledger charts defined`}
						</span>
					</div>
					
					<Button 
						variant="outline" 
						class="text-[10px] h-7 px-2"
						onclick={() => isLedgerSheetOpen = true}
					>
						<Plus size={12} class={isAr ? 'ml-1' : 'mr-1'} />
						{isAr ? "حساب جديد" : "New Account"}
					</Button>
				</div>

				{#if ledgers.length === 0}
					<div class="flex-1 flex flex-col items-center justify-center py-12 text-muted-foreground border border-dashed border-border rounded-lg bg-card/10 select-none">
						<Layers size={32} class="text-muted-foreground/30 mb-2" />
						<span class="text-[10px]">{isAr ? "لا توجد فئات حسابات مضافة" : "No ledger accounts created yet."}</span>
					</div>
				{:else}
					<div class="overflow-x-auto">
						<Table>
							<TableHeader>
								<TableRow class="hover:bg-transparent">
									<TableHead class="text-start">{isAr ? "الرمز" : "Code"}</TableHead>
									<TableHead class="text-start">{isAr ? "اسم البند" : "Account Name"}</TableHead>
									<TableHead class="text-start">{isAr ? "التصنيف الدفتري" : "Type"}</TableHead>
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
