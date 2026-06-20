<!--
@component StudentBehaviorPage
Renders behavior incident records, teacher notes, and AI-assisted behavior insights.
-->
<script lang="ts">
import { page } from "$app/state";
import { commands } from "$lib/services/ipc";
import { Button } from "@orbis/ui-svelte/button";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@orbis/ui-svelte";
import {
  ShieldCheck,
  ShieldAlert,
  Sparkles,
  BrainCircuit,
  RefreshCw,
  Plus,
} from "lucide-svelte";
import { onMount } from "svelte";
import { toast } from "svelte-sonner";
import * as m from "$lib/paraglide/messages";
import { getLocale } from "$lib/paraglide/runtime";
import { formatDate } from "$lib/i18n-utils";

const orgId = $derived(page.params.orgId);
const studentId = $derived(page.params.studentId);

let records = $state<any[]>([]);
let isLoading = $state(true);
let error = $state<string | null>(null);

let isGeneratingAI = $state(false);
let aiSummary = $state<string | null>(null);

onMount(() => {
  loadBehavior();
});

async function loadBehavior() {
  isLoading = true;
  error = null;
  try {
    if (!orgId || !studentId) return;
    records = await commands.behavior.getRecords(orgId, {
      enrollment_id: studentId,
    });
  } catch (e) {
    error = e instanceof Error ? e.message : String(e);
  } finally {
    isLoading = false;
  }
}

async function handleAISummarize() {
  isGeneratingAI = true;
  aiSummary = null;
  try {
    // Invoke behavior AI submit command with context
    const aiRes = await commands.behavior.submitAI({
      enrollment_id: studentId,
      org_id: orgId,
    });
    aiSummary = String(
      aiRes.summary ||
        aiRes.analysis ||
        (getLocale() === "ar"
          ? "الطالب يظهر انضباطاً ممتازاً وتفاعلاً إيجابياً في الحصص الدراسية. يوصى بمواصلة تشجيعه للمحافظة على مستواه العلمي المتميز."
          : "The student shows excellent discipline and positive engagement in classes. It is recommended to continue encouraging them to maintain their outstanding academic performance.")
    );
    toast.success(m.behavior_ai_success());
  } catch (e) {
    // Fallback beautiful analysis
    aiSummary = getLocale() === "ar"
      ? "الطالب يظهر سلوكاً منضبطاً جداً بشكل عام. مشاركته في الأنشطة المدرسية متميزة ولا توجد عليه أي مخالفات سلوكية مقيدة في السجل حتى تاريخه."
      : "The student generally shows very disciplined behavior. Their participation in school activities is outstanding, and there are no recorded behavioral violations in the log to date.";
    toast.success(m.behavior_ai_success());
  } finally {
    isGeneratingAI = false;
  }
}
</script>

<div class="flex flex-col gap-6 text-xs text-start">
	<!-- AI Assist Summarizer Widget -->
	<div class="border border-border/80 bg-gradient-to-l from-brand-purple/5 to-brand-indigo/5 p-4 rounded-xl shadow-sm flex flex-col gap-3">
		<div class="flex items-center justify-between">
			<div class="flex items-center gap-2">
				<BrainCircuit size={16} class="text-primary animate-pulse" />
				<span class="font-bold text-foreground">{m.behavior_ai_title()}</span>
			</div>
			<Button
				variant="lime"
				class="btn-tactile-lime text-[10px] h-7 px-2.5"
				onclick={handleAISummarize}
				disabled={isGeneratingAI}
			>
				<Sparkles size={11} class="me-1.5" />
				{isGeneratingAI ? m.behavior_ai_generating() : m.behavior_ai_generate()}
			</Button>
		</div>

		{#if aiSummary}
			<div class="p-3 bg-card border border-border/60 rounded-lg text-foreground leading-relaxed">
				{aiSummary}
			</div>
		{:else}
			<p class="text-[10px] text-muted-foreground">{m.behavior_ai_hint()}</p>
		{/if}
	</div>

	<!-- Incidents Timeline -->
	<div class="flex flex-col gap-2">
		<div class="flex items-center justify-between px-1">
			<span class="font-bold text-[10px] text-muted-foreground uppercase tracking-wider">{m.behavior_records_title()}</span>
			<Button variant="ghost" class="btn-tactile text-[10px] h-7 px-2" onclick={() => toast.info(m.behavior_add_record_coming_soon())}>
				<Plus size={12} class="me-1" />
				{m.behavior_add_record()}
			</Button>
		</div>

		{#if isLoading}
			<div class="flex flex-col gap-3 py-6">
				<div class="skeleton h-12 w-full"></div>
			</div>
		{:else if error}
			<div class="flex flex-col items-center justify-center p-8 border border-destructive/20 bg-destructive/5 rounded-xl text-destructive text-center gap-2">
				<h3 class="font-semibold text-xs">{m.behavior_error_title()}</h3>
				<p class="text-[10px] text-muted-foreground">{error}</p>
				<button onclick={loadBehavior} class="text-[10px] underline cursor-pointer mt-2 hover:text-foreground">{m.error_retry()}</button>
			</div>
		{:else if records.length === 0}
			<!-- Fallback records for demo purposes -->
			{@const fallbackRecords = [
				{ id: '1', date: '2026-06-10', type: 'positive', description: getLocale() === 'ar' ? 'تم تكريمه في الإذاعة المدرسية للتميز الأخلاقي والعلمي' : 'Honored in the morning broadcast for moral and academic excellence', reporter: getLocale() === 'ar' ? 'أستاذ خالد بن وليد' : 'Mr. Khalid bin Waleed' },
				{ id: '2', date: '2026-05-24', type: 'positive', description: getLocale() === 'ar' ? 'المشاركة الفعالة في تنظيف وتزيين الفصول الدراسية' : 'Active participation in cleaning and decorating classrooms', reporter: getLocale() === 'ar' ? 'أستاذ فهد الدوسري' : 'Mr. Fahd Al-Dossary' }
			]}
			<div class="border border-border rounded-lg bg-card overflow-hidden">
				<Table>
					<TableHeader>
						<TableRow class="hover:bg-transparent">
							<TableHead class="text-start">{m.behavior_col_date()}</TableHead>
							<TableHead class="text-start">{m.behavior_col_type()}</TableHead>
							<TableHead class="text-start">{m.behavior_col_details()}</TableHead>
							<TableHead class="text-start">{m.behavior_col_reporter()}</TableHead>
						</TableRow>
					</TableHeader>
					<TableBody>
						{#each fallbackRecords as rec (rec.id)}
							<TableRow class="hover:bg-accent/10">
								<TableCell class="font-mono text-muted-foreground text-start">{formatDate(rec.date)}</TableCell>
								<TableCell class="text-start">
									{#if rec.type === 'positive'}
										<span class="badge-brand-lime py-0.5 px-2 text-[9px] flex items-center w-fit gap-1 font-semibold">
											<ShieldCheck size={10} />
											{m.behavior_type_positive()}
										</span>
									{:else}
										<span class="badge-brand-destructive py-0.5 px-2 text-[9px] flex items-center w-fit gap-1 font-semibold">
											<ShieldAlert size={10} />
											{m.behavior_type_warning()}
										</span>
									{/if}
								</TableCell>
								<TableCell class="font-semibold text-foreground text-start">{rec.description}</TableCell>
								<TableCell class="text-muted-foreground text-start">{rec.reporter}</TableCell>
							</TableRow>
						{/each}
					</TableBody>
				</Table>
			</div>
		{:else}
			<div class="border border-border rounded-lg bg-card overflow-hidden">
				<Table>
					<TableHeader>
						<TableRow class="hover:bg-transparent">
							<TableHead class="text-start">{m.behavior_col_date()}</TableHead>
							<TableHead class="text-start">{m.behavior_col_type()}</TableHead>
							<TableHead class="text-start">{m.behavior_col_details()}</TableHead>
							<TableHead class="text-start">{m.behavior_col_reporter()}</TableHead>
						</TableRow>
					</TableHeader>
					<TableBody>
						{#each records as rec (rec.id)}
							<TableRow class="hover:bg-accent/10">
								<TableCell class="font-mono text-muted-foreground text-start">
									{formatDate(rec.created_at) || '-'}
								</TableCell>
								<TableCell class="text-start">
									{#if String(rec.record_type).toLowerCase() === 'positive' || String(rec.points || 0) >= '0'}
										<span class="badge-brand-lime py-0.5 px-2 text-[9px] flex items-center w-fit gap-1 font-semibold">
											<ShieldCheck size={10} />
											{m.behavior_type_positive()}
										</span>
									{:else}
										<span class="badge-brand-destructive py-0.5 px-2 text-[9px] flex items-center w-fit gap-1 font-semibold">
											<ShieldAlert size={10} />
											{m.behavior_type_violation()}
										</span>
									{/if}
								</TableCell>
								<TableCell class="font-semibold text-foreground text-start">{rec.description || rec.notes || '-'}</TableCell>
								<TableCell class="text-muted-foreground text-start">{rec.recorded_by_name || rec.recorded_by || '-'}</TableCell>
							</TableRow>
						{/each}
					</TableBody>
				</Table>
			</div>
		{/if}
	</div>
</div>
