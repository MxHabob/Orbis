<!--
@component ExamReports
Reports and printing page with 7 report type cards.
-->
<script lang="ts">
import { Button } from "@orbis/ui-svelte/button";
import { toast } from "svelte-sonner";
import { fade } from "svelte/transition";
import {
  FileText,
  Calendar,
  Hash,
  Building2,
  UserCheck,
  CreditCard,
  ClipboardCheck,
  BarChart3,
  Printer,
  Eye,
} from "lucide-svelte";
import * as m from "$lib/paraglide/messages";

let { data }: { data: { orgId: string } } = $props();

let selectedReport = $state<string | null>(null);
let reportData = $state<any>(null);
let isGenerating = $state(false);

const reportTypes = $derived([
  {
    id: "timetable",
    title: m.exams_reports_timetable(),
    icon: Calendar,
    color: "bg-blue-500/10 text-blue-500 border-blue-500/20",
  },
  {
    id: "seating_list",
    title: m.exams_reports_seating_list(),
    icon: Hash,
    color: "bg-emerald-500/10 text-emerald-500 border-emerald-500/20",
  },
  {
    id: "hall_map",
    title: m.exams_reports_hall_map(),
    icon: Building2,
    color: "bg-purple-500/10 text-purple-500 border-purple-500/20",
  },
  {
    id: "proctor_sheet",
    title: m.exams_reports_proctor_sheet(),
    icon: UserCheck,
    color: "bg-amber-500/10 text-amber-500 border-amber-500/20",
  },
  {
    id: "student_cards",
    title: m.exams_reports_student_cards(),
    icon: CreditCard,
    color: "bg-rose-500/10 text-rose-500 border-rose-500/20",
  },
  {
    id: "attendance",
    title: m.exams_reports_attendance(),
    icon: ClipboardCheck,
    color: "bg-cyan-500/10 text-cyan-500 border-cyan-500/20",
  },
  {
    id: "summary",
    title: m.exams_reports_summary(),
    icon: BarChart3,
    color: "bg-indigo-500/10 text-indigo-500 border-indigo-500/20",
  },
]);

async function handleSelectReport(id: string) {
  selectedReport = id;
  isGenerating = true;
  try {
    // TODO: Wire to IPC
    reportData = null;
    toast.info(
      `تقرير "${reportTypes.find((r) => r.id === id)?.title}" — قيد التحضير`,
    );
  } catch (e) {
    toast.error("Failed to generate report");
  } finally {
    isGenerating = false;
  }
}

function handlePrint() {
  window.print();
}
</script>

<div class="h-full w-full overflow-y-auto scrollbar-hide p-6 flex flex-col gap-5 select-none">
	<!-- Header -->
	<div class="card-tactile bg-gradient-to-br from-card to-card/50 p-5 flex items-center gap-4" in:fade={{ duration: 150 }}>
		<div class="h-10 w-10 rounded-xl bg-amber-500/10 text-amber-500 flex items-center justify-center shrink-0">
			<FileText size={20} />
		</div>
		<div class="flex flex-col">
			<h1 class="text-sm font-bold text-foreground">{m.exams_reports_title()}</h1>
			<p class="text-[10px] text-muted-foreground mt-0.5">
				اختر نوع التقرير لمعاينته وطباعته
			</p>
		</div>
	</div>

	<!-- Report Type Cards Grid -->
	<div class="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-4 gap-3" in:fade={{ delay: 50, duration: 200 }}>
		{#each reportTypes as report (report.id)}
			<Button
				variant="ghost"
				class="card-tactile bg-card p-4 flex flex-col items-center gap-3 text-center transition-all duration-200 hover:-translate-y-1 cursor-pointer group h-auto w-full
					{selectedReport === report.id ? 'ring-2 ring-primary ring-offset-2 ring-offset-background' : ''}"
				onclick={() => handleSelectReport(report.id)}
			>
				<div class="h-10 w-10 rounded-lg border flex items-center justify-center transition-transform duration-200 group-hover:scale-110 {report.color}">
					<report.icon size={18} />
				</div>
				<span class="text-[10px] font-bold text-foreground group-hover:text-primary transition-colors leading-tight">
					{report.title}
				</span>
			</Button>
		{/each}
	</div>

	<!-- Report Preview Area -->
	{#if selectedReport}
		<div class="card-tactile bg-card p-6 flex-1 min-h-[200px] flex flex-col gap-4" in:fade={{ duration: 200 }}>
			<div class="flex items-center justify-between">
				<h3 class="text-xs font-bold text-foreground flex items-center gap-2">
					<Eye size={14} class="text-primary" />
					{m.exams_reports_preview()}: {reportTypes.find(r => r.id === selectedReport)?.title}
				</h3>
				<Button
					variant="default"
					class="h-7 px-3 text-[10px] flex items-center gap-1 cursor-pointer"
					onclick={handlePrint}
				>
					<Printer size={12} />
					{m.exams_reports_print()}
				</Button>
			</div>

			{#if isGenerating}
				<div class="flex-1 flex items-center justify-center">
					<div class="h-6 w-6 border-2 border-primary border-t-transparent rounded-full animate-spin"></div>
				</div>
			{:else if reportData}
				<div class="flex-1 overflow-auto text-xs">
					<pre class="bg-muted/30 rounded-lg p-4 text-[10px] font-mono whitespace-pre-wrap">{JSON.stringify(reportData, null, 2)}</pre>
				</div>
			{:else}
				<div class="flex-1 flex items-center justify-center text-xs text-muted-foreground">
					سيتم عرض معاينة التقرير هنا بعد ربط API
				</div>
			{/if}
		</div>
	{/if}
</div>
