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
import { Input } from "@orbis/ui-svelte/input";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui-svelte/select";
import {
  X,
  ExternalLink,
  RefreshCw,
  LogOut,
  Phone,
  Mail,
  MapPin,
  Calendar,
  CreditCard,
  Award,
  UserMinus,
  ShieldCheck,
  Edit2,
} from "lucide-svelte";
import { onMount } from "svelte";
import { toast } from "svelte-sonner";
import * as m from "$lib/paraglide/messages";
import { getLocale, localizeHref } from "$lib/paraglide/runtime";
import { formatCurrency, formatDate, formatPercent } from "$lib/i18n-utils";

let { studentId, orgId, onClose }: { studentId: string; orgId: string; onClose: () => void } =
  $props();

// Local reactivity for student data and stats
const student = $derived(studentsStore.list.find((s) => s.id === studentId));
const currentClass = $derived(
  student ? studentsStore.classes.find((c) => c.id === student.class_id) : null,
);

let balanceDue = $state<string>(m.loading());
let gradeSummary = $state<string>(m.loading());
let isProcessing = $state(false);

// Actions dropdowns toggles
let isTransferring = $state(false);
let isWithdrawing = $state(false);
let selectedNewClassId = $state("");
let withdrawReason = $state("");

const selectedClassName = $derived(
  studentsStore.classes.find((c) => String(c.id) === selectedNewClassId)?.name ||
    studentsStore.classes.find((c) => String(c.id) === selectedNewClassId)?.name_ar ||
    m.inspector_select_new_class(),
);


// Fetch student stats when studentId changes
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

async function handleTransfer() {
  if (!selectedNewClassId) return;
  isProcessing = true;
  try {
    await studentsStore.transfer(studentId, selectedNewClassId);
    toast.success(m.inspector_transfer_success());
    isTransferring = false;
  } catch (e) {
    toast.error(
      m.inspector_transfer_error({ error: e instanceof Error ? e.message : String(e) }),
    );
  } finally {
    isProcessing = false;
  }
}

async function handleWithdraw() {
  if (!withdrawReason) {
    toast.error(m.inspector_withdraw_reason_required());
    return;
  }
  isProcessing = true;
  try {
    await studentsStore.withdraw(studentId, withdrawReason);
    toast.success(m.inspector_withdraw_success());
    isWithdrawing = false;
  } catch (e) {
    toast.error(
      m.inspector_withdraw_error({ error: e instanceof Error ? e.message : String(e) }),
    );
  } finally {
    isProcessing = false;
  }
}
</script>

{#if student}
	<div class="h-full flex flex-col overflow-hidden text-xs">
		<!-- Header -->
		<div class="flex items-center justify-between p-4 border-b border-border bg-card">
			<div class="flex items-center gap-2">
				<Button
					href={localizeHref(`/${orgId}/students/${studentId}/grades`)}
					variant="ghost"
					size="icon"
					class="h-7 w-7 text-muted-foreground hover:text-foreground"
					title={m.inspector_view_full()}
				>
					<ExternalLink size={14} />
				</Button>
				<Button
					href={localizeHref(`/${orgId}/students/${studentId}/edit`)}
					variant="ghost"
					size="icon"
					class="h-7 w-7 text-muted-foreground hover:text-foreground"
					title={m.inspector_edit_record()}
				>
					<Edit2 size={13} />
				</Button>
				<span class="font-bold text-muted-foreground uppercase text-[10px] ms-1">{m.inspector_title()}</span>
			</div>
			
			<Button
				variant="ghost"
				size="icon"
				class="h-7 w-7 text-muted-foreground hover:text-foreground"
				onclick={onClose}
			>
				<X size={14} />
			</Button>
		</div>

		<!-- Main Profile Section -->
		<div class="flex-1 overflow-y-auto p-5 flex flex-col gap-6 scrollbar-hide select-none">
			<div class="flex flex-col items-center text-center gap-3">
				<Avatar class="h-16 w-16 ring-2 ring-primary/10 glow-primary">
					<AvatarImage src={student.avatar ? String(student.avatar) : undefined} alt={student.full_name ? String(student.full_name) : ''} />
					<AvatarFallback class="bg-primary/5 text-primary text-lg font-bold">
						{String(student.full_name || "S").slice(0, 1)}
					</AvatarFallback>
				</Avatar>
				<div>
					<h3 class="text-sm font-bold text-foreground">{student.full_name || student.name_ar || ''}</h3>
					{#if student.name_en}
						<p class="text-[10px] text-muted-foreground font-mono mt-0.5">{student.name_en}</p>
					{/if}
				</div>
				<div>
					{#if String(student.status).toLowerCase() === 'active'}
						<span class="badge-brand-lime py-0.5 px-2.5 text-[9px] font-semibold">{m.status_active_enrollment()}</span>
					{:else if String(student.status).toLowerCase() === 'withdrawn'}
						<span class="badge-brand-destructive py-0.5 px-2.5 text-[9px] font-semibold">{m.status_withdrawn()}</span>
					{:else}
						<span class="badge-brand-default py-0.5 px-2.5 text-[9px] font-semibold">{student.status}</span>
					{/if}
				</div>
			</div>

			<div class="h-px bg-border"></div>

			<!-- Quick Indicators Cards -->
			<div class="grid grid-cols-2 gap-2 text-center">
				<div class="bg-card/50 border border-border/80 p-3 rounded-lg flex flex-col items-center justify-center">
					<Award size={14} class="text-primary mb-1" />
					<span class="text-[9px] text-muted-foreground">{m.inspector_grade_avg()}</span>
					<span class="font-bold text-foreground mt-0.5">{gradeSummary}</span>
				</div>
				<div class="bg-card/50 border border-border/80 p-3 rounded-lg flex flex-col items-center justify-center">
					<CreditCard size={14} class="text-emerald-500 mb-1" />
					<span class="text-[9px] text-muted-foreground">{m.inspector_balance_due()}</span>
					<span class="font-bold text-foreground mt-0.5">{balanceDue}</span>
				</div>
			</div>

			<!-- Dynamic Actions Box -->
			<div class="flex flex-col gap-2">
				<span class="text-[9px] font-bold text-muted-foreground uppercase tracking-wider px-1">{m.inspector_quick_actions()}</span>
				
				<!-- Transfer Class Action -->
				{#if isTransferring}
					<div class="bg-card border border-border p-3 rounded-lg flex flex-col gap-2">
						<span class="font-semibold text-[10px]">{m.inspector_select_new_class()}</span>
						<Select bind:value={selectedNewClassId} type="single" disabled={isProcessing}>
							<SelectTrigger class="w-full h-8 text-xs text-start flex items-center justify-between border border-input/60" id="classSelect">
								<span>{selectedClassName}</span>
							</SelectTrigger>
							<SelectContent class="text-start" align="end">
								{#each studentsStore.classes.filter(c => c.id !== student.class_id) as cls (cls.id)}
									<SelectItem value={String(cls.id)} class="text-xs text-start justify-start">
										{cls.name || cls.name_ar || cls.id}
									</SelectItem>
								{/each}
							</SelectContent>
						</Select>
						<div class="flex gap-2 justify-end mt-1">
							<Button variant="ghost" size="sm" class="h-7 text-[10px]" onclick={() => isTransferring = false}>{m.inspector_cancel()}</Button>
							<Button variant="default" size="sm" class="h-7 btn-tactile text-[10px]" onclick={handleTransfer} disabled={isProcessing}>{m.inspector_confirm_transfer()}</Button>
						</div>
					</div>
				{:else if isWithdrawing}
					<div class="bg-card border border-border p-3 rounded-lg flex flex-col gap-2">
						<span class="font-semibold text-[10px]">{m.inspector_enter_reason()}</span>
						<Input
							type="text"
							placeholder={m.inspector_reason_placeholder()}
							class="input-sunken text-xs py-1.5"
							bind:value={withdrawReason}
							disabled={isProcessing}
						/>
						<div class="flex gap-2 justify-end mt-1">
							<Button variant="ghost" size="sm" class="h-7 text-[10px]" onclick={() => isWithdrawing = false}>{m.inspector_cancel()}</Button>
							<Button variant="destructive" size="sm" class="h-7 btn-tactile-destructive text-[10px]" onclick={handleWithdraw} disabled={isProcessing}>{m.inspector_confirm_withdraw()}</Button>
						</div>
					</div>
				{:else}
					<div class="grid grid-cols-2 gap-2">
						<Button
							variant="ghost"
							class="btn-tactile text-center justify-center"
							onclick={() => isTransferring = true}
							disabled={String(student.status).toLowerCase() === 'withdrawn'}
						>
							<RefreshCw size={12} class="me-1.5" />
							{m.inspector_transfer_class()}
						</Button>
						<Button
							variant="ghost"
							class="btn-tactile-destructive text-center justify-center"
							onclick={() => isWithdrawing = true}
							disabled={String(student.status).toLowerCase() === 'withdrawn'}
						>
							<UserMinus size={12} class="me-1.5" />
							{m.inspector_withdraw()}
						</Button>
					</div>
				{/if}
			</div>

			<!-- Static Personal Details -->
			<div class="flex flex-col gap-3">
				<span class="text-[9px] font-bold text-muted-foreground uppercase tracking-wider px-1">{m.inspector_contact_details()}</span>
				
				<div class="bg-card/30 border border-border/60 rounded-lg p-3 flex flex-col gap-2.5">
					<div class="flex items-center gap-2">
						<Phone size={12} class="text-muted-foreground" />
						<span class="font-mono text-foreground">{student.phone_number || m.inspector_no_phone()}</span>
					</div>
					<div class="flex items-center gap-2">
						<Mail size={12} class="text-muted-foreground" />
						<span class="font-mono text-foreground">{student.email || m.inspector_no_email()}</span>
					</div>
					{#if student.address}
						<div class="flex items-center gap-2">
							<MapPin size={12} class="text-muted-foreground" />
							<span>{student.address}</span>
						</div>
					{/if}
					<div class="flex items-center gap-2">
						<Calendar size={12} class="text-muted-foreground" />
						<span>{m.inspector_registered_on({ date: formatDate(student.created_at as string) })}</span>
					</div>
				</div>
			</div>
		</div>
	</div>
{/if}

