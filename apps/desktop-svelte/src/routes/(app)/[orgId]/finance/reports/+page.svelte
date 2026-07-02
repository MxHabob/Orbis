<!--
@component FinanceReportsPage
A visual analysis dashboard showing collection trends over time.
Integrates the high-fidelity FeeCollectionCard.
-->
<script lang="ts">
import FeeCollectionCard from "$lib/components/metrics/cards/fee-collection-card.svelte";
import { Landmark } from "lucide-svelte";
import { fade } from "svelte/transition";
import * as m from "$lib/paraglide/messages";

let { data }: { data: { orgId: string } } = $props();
</script>

<div class="h-full w-full overflow-y-auto p-6 flex flex-col gap-6 font-sans">
	<!-- Header Title -->
	<div class="text-start select-none">
		<h2 class="text-sm font-bold text-foreground uppercase tracking-wider">
			{m.reports_finance_title()}
		</h2>
		<p class="text-[10px] text-muted-foreground mt-0.5">
			{m.reports_finance_subtitle()}
		</p>
	</div>

	<!-- Graphs Grid Layout -->
	<div class="grid grid-cols-1 lg:grid-cols-3 gap-6" in:fade={{ duration: 150 }}>
		<!-- Col-span-2: Fee Collection Card Chart -->
		<div class="lg:col-span-2">
			<FeeCollectionCard
				orgId={data.orgId}
				selectedClass="all"
				selectedTerm="all"
				period="this-year"
				isEditing={false}
			/>
		</div>

		<!-- Diagnostics Panel -->
		<div class="card-tactile bg-card/45 p-6 flex flex-col gap-4 text-start justify-between select-none">
			<div class="flex flex-col gap-3">
				<h3 class="text-xs font-bold text-foreground uppercase tracking-wider flex items-center gap-1.5">
					<Landmark size={13} class="text-primary" />
					<span>{m.reports_finance_audit_title()}</span>
				</h3>
				<p class="text-[10px] text-muted-foreground leading-relaxed">
					{m.reports_finance_audit_desc()}
				</p>
			</div>

			<div class="border-t border-border/50 pt-4 flex flex-col gap-1 text-[9px] text-muted-foreground">
				<div class="flex justify-between items-center">
					<span>{m.reports_finance_last_refresh()}</span>
					<span class="font-mono">{m.reports_finance_last_refresh_val()}</span>
				</div>
				<div class="flex justify-between items-center">
					<span>{m.reports_finance_accuracy()}</span>
					<span class="text-emerald-500 font-bold">100%</span>
				</div>
			</div>
		</div>
	</div>
</div>
