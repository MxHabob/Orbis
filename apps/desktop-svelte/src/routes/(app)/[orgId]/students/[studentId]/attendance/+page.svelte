<!--
@component StudentAttendancePage
Renders attendance rates, present/absent stats, and detailed logs history.
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
import { Calendar, UserCheck, AlertCircle, Clock, Ban } from "lucide-svelte";
import { onMount } from "svelte";
import * as m from "$lib/paraglide/messages";
import { getLocale } from "$lib/paraglide/runtime";
import { formatNumber, formatPercent, formatDate } from "$lib/i18n-utils";

const studentId = $derived(page.params.studentId || "");

let report = $state<any>(null);
let isLoading = $state(true);
let error = $state<string | null>(null);

onMount(() => {
  loadAttendance();
});

async function loadAttendance() {
  isLoading = true;
  error = null;
  try {
    report = await commands.attendance.getReport(studentId);
  } catch (e) {
    error = e instanceof Error ? e.message : String(e);
  } finally {
    isLoading = false;
  }
}

// Map english status keys to arabic labels and color classes
function getStatusBadge(status: string) {
  const norm = String(status || "").toLowerCase();
  switch (norm) {
    case "present":
    case "حاضر":
      return {
        label: m.attendance_status_present(),
        class: "badge-brand-lime",
      };
    case "absent":
    case "غائب":
      return {
        label: m.attendance_status_absent(),
        class: "badge-brand-destructive",
      };
    case "late":
    case "متأخر":
      return {
        label: m.attendance_status_late(),
        class: "badge-brand-default",
      };
    case "excused":
    case "مستأذن":
      return {
        label: m.attendance_status_excused(),
        class: "badge-brand-primary",
      };
    default:
      return {
        label: status || m.attendance_status_unknown(),
        class: "badge-brand-default",
      };
  }
}
</script>

<div class="flex flex-col gap-6 text-xs text-start">
	<!-- Attendance metrics summary grid -->
	{#if report}
		<div class="grid grid-cols-4 gap-3 shrink-0 select-none">
			<div class="border border-border bg-card/45 p-4 rounded-xl flex items-center justify-between shadow-sm">
				<div class="flex flex-col">
					<span class="text-[9px] text-muted-foreground">{m.attendance_present_days()}</span>
					<span class="text-sm font-bold mt-0.5 text-emerald-500">{formatNumber(report.present_count || 0)} {m.attendance_day_unit()}</span>
				</div>
				<UserCheck size={16} class="text-emerald-500" />
			</div>
			<div class="border border-border bg-card/45 p-4 rounded-xl flex items-center justify-between shadow-sm">
				<div class="flex flex-col">
					<span class="text-[9px] text-muted-foreground">{m.attendance_absent_days()}</span>
					<span class="text-sm font-bold mt-0.5 text-destructive">{formatNumber(report.absent_count || 0)} {m.attendance_day_unit()}</span>
				</div>
				<Ban size={16} class="text-destructive" />
			</div>
			<div class="border border-border bg-card/45 p-4 rounded-xl flex items-center justify-between shadow-sm">
				<div class="flex flex-col">
					<span class="text-[9px] text-muted-foreground">{m.attendance_late_cases()}</span>
					<span class="text-sm font-bold mt-0.5 text-amber-500">{formatNumber(report.late_count || 0)} {m.attendance_times_unit()}</span>
				</div>
				<Clock size={16} class="text-amber-500" />
			</div>
			<div class="border border-border bg-card/45 p-4 rounded-xl flex items-center justify-between shadow-sm">
				<div class="flex flex-col">
					<span class="text-[9px] text-muted-foreground">{m.attendance_rate()}</span>
					<span class="text-sm font-bold mt-0.5 text-gradient-brand">{formatPercent(report.rate ?? 100.0)}</span>
				</div>
				<Calendar size={16} class="text-primary" />
			</div>
		</div>
	{/if}

	{#if isLoading}
		<div class="flex flex-col gap-3 py-6">
			<div class="skeleton h-12 w-full"></div>
			<div class="skeleton h-12 w-full"></div>
		</div>
	{:else if error}
		<div class="flex flex-col items-center justify-center p-8 border border-destructive/20 bg-destructive/5 rounded-xl text-destructive text-center gap-2">
			<h3 class="font-semibold text-xs">{m.attendance_error_title()}</h3>
			<p class="text-[10px] text-muted-foreground">{error}</p>
			<button onclick={loadAttendance} class="text-[10px] underline cursor-pointer mt-2 hover:text-foreground">{m.error_retry()}</button>
		</div>
	{:else}
		<!-- Log History table -->
		<div class="flex flex-col gap-2">
			<span class="font-bold text-[10px] text-muted-foreground uppercase tracking-wider px-1">{m.attendance_log_title()}</span>

			{#if !report || !report.logs || report.logs.length === 0}
				<div class="flex flex-col items-center justify-center p-12 text-center text-muted-foreground border border-dashed border-border rounded-xl bg-card/10 gap-3">
					<Calendar size={28} class="text-muted-foreground/30" />
					<div class="flex flex-col gap-0.5">
						<p class="text-xs font-semibold">
							{m.reports_no_attendance()}
						</p>
						<p class="text-[10px] text-muted-foreground/80">
							{getLocale() === 'ar' ? 'سجل غياب وحضور الطالب اليومي سيظهر هنا.' : 'Daily student attendance and absence records will appear here.'}
						</p>
					</div>
				</div>
			{:else}
				<div class="border border-border rounded-lg bg-card overflow-hidden">
					<Table>
						<TableHeader>
							<TableRow class="hover:bg-transparent">
								<TableHead class="text-start">{m.attendance_col_date()}</TableHead>
								<TableHead class="text-start">{m.attendance_col_status()}</TableHead>
								<TableHead class="text-start">{m.attendance_col_notes()}</TableHead>
							</TableRow>
						</TableHeader>
						<TableBody>
							{#each report.logs as log (log.id)}
								{@const status = getStatusBadge(log.status)}
								<TableRow class="hover:bg-accent/10">
									<TableCell class="font-mono text-foreground text-start">
										{formatDate(log.date) || '-'}
									</TableCell>
									<TableCell class="text-start">
										<span class="{status.class} py-0.5 px-2 text-[10px]">{status.label}</span>
									</TableCell>
									<TableCell class="text-muted-foreground text-start">{log.notes || '-'}</TableCell>
								</TableRow>
							{/each}
						</TableBody>
					</Table>
				</div>
			{/if}
		</div>
	{/if}
</div>

