<!--
@component StudentGradesPage
Renders the subject grades and overall report card analytics for a student.
-->
<script lang="ts">
import { page } from "$app/state";
import { commands } from "$lib/services/ipc";
import { 
  Table, TableBody, TableCell, TableHead, TableHeader, TableRow 
} from "@orbis/ui-svelte";
import { Progress } from "@orbis/ui-svelte";
import { GraduationCap, Award, Calendar, RefreshCcw } from "lucide-svelte";
import { onMount } from "svelte";

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

<div class="flex flex-col gap-5 text-xs text-right">
	<!-- Summary header card -->
	<div class="flex items-center justify-between">
		<div>
			<h2 class="text-sm font-bold">الدرجات الحالية والتقييمات</h2>
			<p class="text-[10px] text-muted-foreground">تفاصيل درجات الطالب المكتسبة في المواد الدراسية المختلفة خلال الدورة الحالية.</p>
		</div>
		<button 
			class="h-7 w-7 rounded-md border border-border flex items-center justify-center text-muted-foreground hover:text-foreground cursor-pointer transition-colors"
			onclick={loadGrades}
			title="تحديث البيانات"
		>
			<RefreshCcw size={12} class={isLoading ? 'animate-spin' : ''} />
		</button>
	</div>

	{#if isLoading}
		<div class="flex flex-col gap-3 py-6">
			<div class="skeleton h-12 w-full"></div>
			<div class="skeleton h-10 w-full"></div>
			<div class="skeleton h-10 w-full"></div>
		</div>
	{:else if error}
		<div class="flex flex-col items-center justify-center p-8 border border-destructive/20 bg-destructive/5 rounded-xl text-destructive text-center gap-2">
			<h3 class="font-semibold text-xs">فشل تحميل كشف الدرجات</h3>
			<p class="text-[10px] text-muted-foreground">{error}</p>
			<button onclick={loadGrades} class="text-[10px] underline cursor-pointer mt-2 hover:text-foreground">إعادة المحاولة</button>
		</div>
	{:else if !report || !report.grades || report.grades.length === 0}
		<div class="flex flex-col items-center justify-center p-12 text-center text-muted-foreground border border-dashed border-border rounded-xl bg-card/10 gap-3">
			<GraduationCap size={28} class="text-muted-foreground/30 animate-pulse" />
			<div class="flex flex-col gap-0.5">
				<p class="text-xs font-semibold">لا توجد درجات مسجلة بعد</p>
				<p class="text-[10px] text-muted-foreground/80">لم يقم المعلمون برصد أي تقييمات لهذا الطالب في هذه الدورة.</p>
			</div>
		</div>
	{:else}
		<!-- Main Grades Table -->
		<div class="border border-border rounded-lg bg-card overflow-hidden">
			<Table>
				<TableHeader>
					<TableRow class="hover:bg-transparent">
						<TableHead class="text-right">المادة الدراسية</TableHead>
						<TableHead class="text-right">التقييم</TableHead>
						<TableHead class="text-right">تاريخ الرصد</TableHead>
						<TableHead class="text-right">الدرجة المكتسبة</TableHead>
						<TableHead class="text-right">مؤشر النسبة</TableHead>
					</TableRow>
				</TableHeader>
				<TableBody>
					{#each report.grades as grade (grade.id || grade.subject_id)}
						{@const score = Number(grade.score || 0)}
						<TableRow class="hover:bg-accent/20">
							<TableCell class="font-bold text-foreground">{grade.subject_name || grade.subject_id}</TableCell>
							<TableCell>{grade.assessment_name || 'تقييم فصلي'}</TableCell>
							<TableCell class="font-mono text-muted-foreground">
								{grade.graded_at ? new Date(String(grade.graded_at)).toLocaleDateString('ar-SA') : '-'}
							</TableCell>
							<TableCell class="font-bold {getGradeColor(score)} font-mono">
								{score}%
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
					<span class="text-[10px] text-muted-foreground">التقدير العام</span>
					<span class="text-sm font-bold text-gradient-brand mt-0.5">ممتاز (أ+)</span>
				</div>
				<Award size={18} class="text-primary" />
			</div>
			<div class="border border-border bg-card/30 p-4 rounded-xl flex items-center justify-between">
				<div class="flex flex-col">
					<span class="text-[10px] text-muted-foreground">آخر تحديث للرصد</span>
					<span class="text-xs font-semibold text-foreground mt-0.5">منذ يومين</span>
				</div>
				<Calendar size={18} class="text-muted-foreground" />
			</div>
		</div>
	{/if}
</div>
