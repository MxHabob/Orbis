<!--
@component StudentInspector
Renders a slide-over/inspector side panel with details and actions for the selected student.
-->
<script lang="ts">
import { studentsStore } from "$lib/stores/students.svelte";
import { authStore } from "$lib/stores";
import { commands } from "$lib/services/ipc";
import { Button } from "@orbis/ui-svelte/button";
import { Avatar, AvatarFallback, AvatarImage } from "@orbis/ui-svelte";
import {
  X,
  ExternalLink,
  CreditCard,
  Award,
  Edit2,
} from "lucide-svelte";
import * as m from "$lib/paraglide/messages";
import { localizeHref } from "$lib/paraglide/runtime";
import { formatCurrency, formatDate, formatPercent } from "$lib/i18n-utils";

let {
  studentId,
  orgId,
  onClose,
}: { studentId: string; orgId: string; onClose: () => void } = $props();

const student = $derived(studentsStore.list.find((s) => s.id === studentId));

let balanceDue = $state<string>(m.loading());
let gradeSummary = $state<string>(m.loading());

$effect(() => {
  if (studentId) {
    fetchStats();
  }
});

async function fetchStats() {
  balanceDue = m.loading();
  gradeSummary = m.loading();
  try {
    const orgId = authStore.orgId;
    if (!orgId) return;

    // Fetch finances
    const fees = await commands.finance.getStudentFees(orgId, studentId);
    const payments = await commands.finance.getPayments(orgId, studentId);

    const totalFees = fees.reduce((sum, f) => sum + Number(f.amount || 0), 0);
    const totalPaid = payments.reduce(
      (sum, p) => sum + Number(p.amount || 0),
      0,
    );
    const diff = totalFees - totalPaid;
    balanceDue = diff <= 0 ? m.loading_no_dues() : formatCurrency(diff);

    // Fetch grades report card summary
    const report = await commands.grades.getReportCard(studentId);
    if (report && Array.isArray(report.grades)) {
      const avg =
        report.grades.reduce(
          (sum: number, g: any) => sum + Number(g.score || 0),
          0,
        ) / (report.grades.length || 1);
      gradeSummary = formatPercent(avg);
    } else {
      gradeSummary = m.loading_no_grades();
    }
  } catch (e) {
    balanceDue = m.loading_error();
    gradeSummary = m.loading_no_grades();
  }
}
</script>

{#if student}
	<div class="h-full flex flex-col overflow-hidden text-xs">
		<!-- Header -->
		<div class="flex items-center justify-between p-4">
			<div class="flex items-center gap-2">
				<Button
					href={localizeHref(`/${orgId}/students/${studentId}/grades`)}
					variant="ghost"
					// size="icon"
					class="text-muted-foreground hover:text-foreground"
					title={m.inspector_view_full()}
				>
					<ExternalLink size={14} />
				</Button>
				<Button
					href={localizeHref(`/${orgId}/students/${studentId}/edit`)}
					variant="ghost"
					// size="icon"
					class=" text-muted-foreground hover:text-foreground"
					title={m.inspector_edit_record()}
				>
					<Edit2 size={13} />
				</Button>
			</div>
			
			<Button
				variant="ghost"
				// size="icon"
				class="text-muted-foreground hover:text-foreground"
				onclick={onClose}
			>
				<X size={14} />
			</Button>
		</div>

		<!-- Main Profile Section -->
		<div class="flex-1 overflow-y-auto flex flex-col gap-6 scrollbar-hide p-4 select-none">
			<div class="flex flex-col items-center text-center gap-3">
				<Avatar class="w-[85%] max-w-[180px] min-w-[120px] aspect-square h-auto rounded-xl">
					<AvatarImage src={student.avatar ? String(student.avatar) : undefined} alt={student.full_name ? String(student.full_name) : ''} />
					<AvatarFallback class="text-5xl font-bold rounded-xl flex items-center justify-center">
						{String(student.full_name || "S").slice(0, 1)}
					</AvatarFallback>
				</Avatar>
				<div>
				  <h3 class="text-sm font-bold text-foreground">{student.full_name || student.name_ar || ''}</h3>
				</div>
			</div>

			<div class="grid grid-cols-2 gap-2 text-center">
				<div class="flex flex-col items-center justify-center">
					<Award size={14} class="text-primary mb-1" />
					<span class="text-[9px] text-muted-foreground">{m.inspector_grade_avg()}</span>
					<span class="font-bold text-foreground mt-0.5">{gradeSummary}</span>
				</div>
				<div class="flex flex-col items-center justify-center">
					<CreditCard size={14} class="text-emerald-500 mb-1" />
					<span class="text-[9px] text-muted-foreground">{m.inspector_balance_due()}</span>
					<span class="font-bold text-foreground mt-0.5">{balanceDue}</span>
				</div>
			</div>


			<!-- Student Metadata List -->
			<div class="flex flex-col gap-4 border-t border-border/40 pt-4 w-full text-start">
				{#if student.student_number || student.id}
					<div class="flex flex-col gap-0.5">
						<span class="text-[9px] text-muted-foreground uppercase font-bold tracking-wider">
							{m.form_student_number ? m.form_student_number() : "الرقم الأكاديمي"}
						</span>
						<span class="text-xs font-mono font-bold text-foreground">
							{student.student_number || student.id}
						</span>
					</div>
				{/if}

				{#if student.phone || student.phone_number}
					<div class="flex flex-col gap-0.5">
						<span class="text-[9px] text-muted-foreground uppercase font-bold tracking-wider">
							الهاتف
						</span>
						<span class="text-xs text-foreground font-mono">
							{student.phone || student.phone_number}
						</span>
					</div>
				{/if}

				{#if student.email}
					<div class="flex flex-col gap-0.5">
						<span class="text-[9px] text-muted-foreground uppercase font-bold tracking-wider">
							البريد الإلكتروني
						</span>
						<span class="text-xs text-foreground font-mono truncate max-w-full" title={student.email}>
							{student.email}
						</span>
					</div>
				{/if}

				{#if student.sex}
					<div class="flex flex-col gap-0.5">
						<span class="text-[9px] text-muted-foreground uppercase font-bold tracking-wider">
							الجنس
						</span>
						<span class="text-xs text-foreground font-medium">
							{student.sex === 'male' ? 'ذكر' : 'أنثى'}
						</span>
					</div>
				{/if}

				{#if student.birth_date}
					<div class="flex flex-col gap-0.5">
						<span class="text-[9px] text-muted-foreground uppercase font-bold tracking-wider">
							تاريخ الميلاد
						</span>
						<span class="text-xs text-foreground font-mono">
							{student.birth_date}
						</span>
					</div>
				{/if}

				{#if student.guardian_name}
					<div class="flex flex-col gap-0.5">
						<span class="text-[9px] text-muted-foreground uppercase font-bold tracking-wider">
							ولي الأمر
						</span>
						<span class="text-xs text-foreground font-medium">
							{student.guardian_name}
						</span>
					</div>
				{/if}

				{#if student.guardian_phone}
					<div class="flex flex-col gap-0.5">
						<span class="text-[9px] text-muted-foreground uppercase font-bold tracking-wider">
							هاتف ولي الأمر
						</span>
						<span class="text-xs text-foreground font-mono">
							{student.guardian_phone}
						</span>
					</div>
				{/if}

				{#if student.created_at}
					<div class="flex flex-col gap-0.5">
						<span class="text-[9px] text-muted-foreground uppercase font-bold tracking-wider">
							تاريخ التسجيل
						</span>
						<span class="text-xs text-foreground font-mono">
							{formatDate(student.created_at as string)}
						</span>
					</div>
				{/if}
			</div>
		</div>
	</div>
{/if}

