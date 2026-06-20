<!--
@component StudentGradesPage
Renders the subject grades and overall report card analytics for a student.
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
import { Progress } from "@orbis/ui-svelte";
import { Button } from "@orbis/ui-svelte/button";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import { GraduationCap, Award, Calendar, RefreshCcw } from "lucide-svelte";
import { onMount } from "svelte";
import * as m from "$lib/paraglide/messages";
import { formatDate, formatPercent } from "$lib/i18n-utils";

const studentId = $derived(page.params.studentId || "");

let report = $state<any>(null);
let isLoading = $state(true);
let error = $state<string | null>(null);

onMount(() => {
  loadGrades();
});

async function loadGrades() {
  isLoading = true;
  error = null;
  try {
    report = await commands.grades.getReportCard(studentId);
  } catch (e) {
    error = e instanceof Error ? e.message : String(e);
  } finally {
    isLoading = false;
  }
}

function getGradeColor(score: number): string {
  if (score >= 90) return "text-emerald-500";
  if (score >= 75) return "text-primary";
  if (score >= 50) return "text-amber-500";
  return "text-destructive";
}
</script>

<div class="flex flex-col gap-5 text-xs text-start">
	<!-- Summary header card -->
	<div class="flex items-center justify-between">
		<div>
			<h2 class="text-sm font-bold">{m.grades_title()}</h2>
			<p class="text-[10px] text-muted-foreground">{m.grades_subtitle()}</p>
		</div>
		<Button 
			variant="outline"
			size="icon-sm"
			class="h-7 w-7 text-muted-foreground hover:text-foreground"
			onclick={loadGrades}
			title={m.grades_refresh()}
		>
			<RefreshCcw size={12} class={isLoading ? 'animate-spin' : ''} />
		</Button>
	</div>

	{#if isLoading}
		<div class="flex flex-col gap-3 py-6">
			<Skeleton class="h-12 w-full" />
			<Skeleton class="h-10 w-full" />
			<Skeleton class="h-10 w-full" />
		</div>
	{:else if error}
		<div class="flex flex-col items-center justify-center p-8 border border-destructive/20 bg-destructive/5 rounded-xl text-destructive text-center gap-2">
			<h3 class="font-semibold text-xs">{m.grades_error_title()}</h3>
			<p class="text-[10px] text-muted-foreground">{error}</p>
			<Button variant="link" onclick={loadGrades} class="text-[10px] underline cursor-pointer mt-2 hover:text-foreground">{m.error_retry()}</Button>
		</div>
	{:else if !report || !report.grades || report.grades.length === 0}
		<div class="flex flex-col items-center justify-center p-12 text-center text-muted-foreground border border-dashed border-border rounded-xl bg-card/10 gap-3">
			<GraduationCap size={28} class="text-muted-foreground/30 animate-pulse" />
			<div class="flex flex-col gap-0.5">
				<p class="text-xs font-semibold">{m.grades_empty_title()}</p>
				<p class="text-[10px] text-muted-foreground/80">{m.grades_empty_subtitle()}</p>
			</div>
		</div>
	{:else}
		<!-- Main Grades Table -->
		<div class="border border-border rounded-lg bg-card overflow-hidden">
			<Table>
				<TableHeader>
					<TableRow class="hover:bg-transparent">
						<TableHead class="text-start">{m.grades_col_subject()}</TableHead>
						<TableHead class="text-start">{m.grades_col_assessment()}</TableHead>
						<TableHead class="text-start">{m.grades_col_date()}</TableHead>
						<TableHead class="text-start">{m.grades_col_score()}</TableHead>
						<TableHead class="text-start">{m.grades_col_indicator()}</TableHead>
					</TableRow>
				</TableHeader>
				<TableBody>
					{#each report.grades as grade (grade.id || grade.subject_id)}
						{@const score = Number(grade.score || 0)}
						<TableRow class="hover:bg-accent/20">
							<TableCell class="font-bold text-foreground text-start">{grade.subject_name || grade.subject_id}</TableCell>
							<TableCell class="text-start">{grade.assessment_name || m.grades_default_assessment()}</TableCell>
							<TableCell class="font-mono text-muted-foreground text-start">
								{formatDate(grade.graded_at) || '-'}
							</TableCell>
							<TableCell class="font-bold {getGradeColor(score)} font-mono text-start">
								{formatPercent(score)}
							</TableCell>
							<TableCell class="w-1/4">
								<Progress value={score} class="h-2 rounded bg-muted" />
							</TableCell>
						</TableRow>
					{/each}
				</TableBody>
			</Table>
		</div>

		<!-- Summary Analytics -->
		<div class="grid grid-cols-2 gap-3 mt-1">
			<div class="border border-border bg-card/30 p-4 rounded-xl flex items-center justify-between">
				<div class="flex flex-col">
					<span class="text-[10px] text-muted-foreground">{m.grades_overall_grade()}</span>
					<span class="text-sm font-bold text-gradient-brand mt-0.5">{m.grades_overall_grade_value()}</span>
				</div>
				<Award size={18} class="text-primary" />
			</div>
			<div class="border border-border bg-card/30 p-4 rounded-xl flex items-center justify-between">
				<div class="flex flex-col">
					<span class="text-[10px] text-muted-foreground">{m.grades_last_update()}</span>
					<span class="text-xs font-semibold text-foreground mt-0.5">{m.grades_last_update_value()}</span>
				</div>
				<Calendar size={18} class="text-muted-foreground" />
			</div>
		</div>
	{/if}
</div>

