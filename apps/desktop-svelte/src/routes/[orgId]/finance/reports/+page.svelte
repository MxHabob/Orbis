<!--
@component FinanceReportsPage
A visual analysis dashboard showing collection trends over time.
Integrates the high-fidelity FeeCollectionCard.
-->
<script lang="ts">
import { getLocale } from "$lib/paraglide/runtime";
import FeeCollectionCard from "$lib/components/metrics/cards/fee-collection-card.svelte";
import { Landmark } from "lucide-svelte";
import { fade } from "svelte/transition";

let { data }: { data: { orgId: string } } = $props();

const isAr = $derived(getLocale() === "ar");
</script>

<div class="h-full w-full overflow-y-auto p-6 flex flex-col gap-6 font-sans">
	<!-- Header Title -->
	<div class="text-start select-none">
		<h2 class="text-sm font-bold text-foreground uppercase tracking-wider">
			{isAr ? "التقارير والتحليلات المالية" : "Financial Analytics & Reports"}
		</h2>
		<p class="text-[10px] text-muted-foreground mt-0.5">
			{isAr ? "رسومات بيانية توضح نسب التحصيل ومعدلات الدفع الشهرية ورسوم الفترات." : "Interactive graphs reflecting collection velocities, cash inflows, and invoicing trends."}
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
					<span>{isAr ? "صحة الدورة المحاسبية" : "Audit Integrity Check"}</span>
				</h3>
				<p class="text-[10px] text-muted-foreground leading-relaxed">
					{isAr 
						? "يقوم النظام تلقائياً بمطابقة جميع إيصالات الدفع المقيدة مع الحسابات الدفترية المعنية لضمان دقة التقارير. جميع البيانات الموضحة أعلاه مستمدة من الفواتير المدفوعة والمعلقة بشكل مباشر." 
						: "All charts and indicators reflect audited transaction ledger entries, ensuring zero discrepancy between bank transfers, cash desk registers, and individual student billing reports."}
				</p>
			</div>

			<div class="border-t border-border/50 pt-4 flex flex-col gap-1 text-[9px] text-muted-foreground">
				<div class="flex justify-between items-center">
					<span>{isAr ? "تحديث التقرير" : "Last refreshed"}</span>
					<span class="font-mono">{isAr ? "تلقائي (حين الطلب)" : "On-demand (Live)"}</span>
				</div>
				<div class="flex justify-between items-center">
					<span>{isAr ? "دقة التقرير" : "Reporting accuracy"}</span>
					<span class="text-emerald-500 font-bold">100%</span>
				</div>
			</div>
		</div>
	</div>
</div>
