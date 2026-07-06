<!--
@component StudentInspector
Renders a slide-over/inspector side panel with details and actions for the selected student.
Updated to support multi-guardian displaying and student lifecycle status transition actions.
-->
<script lang="ts">
import { authStore } from "$lib/stores";
import { commands } from "$lib/core/ipc";
import { Button } from "@orbis/ui-svelte/button";
import { Avatar, AvatarFallback, AvatarImage } from "@orbis/ui-svelte";
import { Badge } from "@orbis/ui-svelte/badge";
import {
  X,
  ExternalLink,
  CreditCard,
  Award
} from "lucide-svelte";
import * as m from "$lib/paraglide/messages";
import { localizeHref } from "$lib/paraglide/runtime";
import { formatCurrency, formatDate, formatPercent } from "$lib/core/i18n";

let {
  student,
  onClose,
}: { student: any; onClose: () => void } = $props();

const studentId = $derived(student?.id || "");
const orgId = $derived(authStore.orgId || "");

let balanceDue = $state<string>(m.loading());
let gradeSummary = $state<string>(m.loading());
let guardiansList = $state<any[]>([]);

$effect(() => {
  if (studentId) {
    fetchStats();
  }
});

async function fetchStats() {
  balanceDue = m.loading();
  gradeSummary = m.loading();
  guardiansList = [];
  try {
    if (!orgId || !student) return;

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

    // Fetch guardians list
    const links = await commands.guardians.getByStudent(student.student_id);
    if (links) {
      const resolved = [];
      for (const link of links) {
        try {
          const user = await commands.users.getUser(String(link.guardian_id));
          if (user) {
            resolved.push({
              name: user.full_name,
              phone: user.phone || "",
              relationship:
                link.relationship_ar || link.relationship || "ولي أمر",
              is_primary: link.is_primary === 1,
            });
          }
        } catch (err) {
          console.error(err);
        }
      }
      guardiansList = resolved;
    }
  } catch (e) {
    balanceDue = m.loading_error();
    gradeSummary = m.loading_no_grades();
  }
}

</script>

{#if student}
	<div class="h-full flex flex-col overflow-hidden text-xs text-foreground">
		<!-- Header -->
		<div class="flex items-center justify-between p-4 border-b border-border/40 shrink-0">
			<div class="flex items-center gap-2">
				<Button
					href={localizeHref(`/${orgId}/students/${studentId}`)}
					variant="ghost"
					class="text-muted-foreground hover:text-foreground h-8 px-2 cursor-pointer"
					title={m.inspector_view_full()}
				>
					<ExternalLink size={14} />
				</Button>
			</div>
			
			<Button
				variant="ghost"
				class="text-muted-foreground hover:text-foreground h-8 px-2 cursor-pointer"
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
					<AvatarFallback class="text-5xl font-bold rounded-xl flex items-center justify-center bg-muted">
						{String(student.full_name || "S").slice(0, 1)}
					</AvatarFallback>
				</Avatar>
				<div class="space-y-1">
				  <h3 class="text-sm font-bold text-foreground">{student.full_name || student.name_ar || ''}</h3>
          <div class="flex justify-center">
            <Badge variant={student.status === 'active' ? 'success' : student.status === 'draft' ? 'outline' : 'secondary'} class="text-[9px] scale-95 font-semibold px-2 py-0.5">
              {student.status === 'active' ? m.form_status_active() : 
               student.status === 'draft' ? m.form_status_draft() :
               student.status === 'pending_review' ? m.form_status_pending_review() :
               student.status === 'withdrawn' ? m.form_status_withdrawn() :
               student.status === 'graduated' ? m.form_status_graduated() :
               student.status}
            </Badge>
          </div>
				</div>
			</div>

			<div class="grid grid-cols-2 gap-2 text-center">
				<div class="flex flex-col items-center justify-center p-2 rounded-xl bg-card border border-border/40">
					<Award size={14} class="text-primary mb-1" />
					<span class="text-[9px] text-muted-foreground">{m.inspector_grade_avg()}</span>
					<span class="font-bold text-foreground mt-0.5">{gradeSummary}</span>
				</div>
				<div class="flex flex-col items-center justify-center p-2 rounded-xl bg-card border border-border/40">
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
							الرقم الأكاديمي
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
						<span class="text-xs text-foreground font-mono truncate max-w-full" title={String(student.email)}>
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
			</div>

      <!-- Linked Guardians Section -->
      {#if guardiansList.length > 0}
        <div class="flex flex-col gap-2 border-t border-border/40 pt-4 w-full text-start">
          <span class="text-[9px] text-muted-foreground uppercase font-bold tracking-wider">
            {m.inspector_guardians_title()}
          </span>
          <div class="flex flex-col gap-2 pt-1">
            {#each guardiansList as g}
              <div class="flex items-center justify-between p-2.5 border border-border/60 rounded-xl bg-muted/20">
                <div class="flex flex-col gap-0.5">
                  <span class="font-bold text-foreground text-xs">{g.name} ({g.relationship})</span>
                  <span class="text-[10px] text-muted-foreground font-mono">{g.phone || 'بلا هاتف'}</span>
                </div>
                {#if g.is_primary}
                  <Badge variant="success" class="text-[8px] scale-90 px-1.5 py-0.5 shrink-0 font-semibold uppercase">{m.form_guardian_role_primary()}</Badge>
                {/if}
              </div>
            {/each}
          </div>
        </div>
      {/if}


		</div>
	</div>
{/if}
