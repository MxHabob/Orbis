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
  Phone,
  Mail,
  MapPin,
  Calendar,
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
				<Avatar class="h-16 w-16">
					<AvatarImage src={student.avatar ? String(student.avatar) : undefined} alt={student.full_name ? String(student.full_name) : ''} />
					<AvatarFallback class="text-lg font-bold">
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


			<div class="flex flex-col gap-3">
				<span class="text-[9px] font-bold uppercase tracking-wider px-1">{m.inspector_contact_details()}</span>
				
				<div class="p-3 flex flex-col gap-2.5">
					<div class="flex items-center gap-2">
						<Phone size={12} />
						<span class="font-mono text-foreground">{student.phone_number || m.inspector_no_phone()}</span>
					</div>
					<div class="flex items-center gap-2">
						<Mail size={12} />
						<span class="font-mono text-foreground">{student.email || m.inspector_no_email()}</span>
					</div>
					{#if student.address}
						<div class="flex items-center gap-2">
							<MapPin size={12}/>
							<span>{student.address}</span>
						</div>
					{/if}
					<div class="flex items-center gap-2">
						<Calendar size={12}/>
						<span>{m.inspector_registered_on({ date: formatDate(student.created_at as string) })}</span>
					</div>
				</div>
			</div>
		</div>
	</div>
{/if}

