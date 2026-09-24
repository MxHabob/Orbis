<!--
  @component StudentAttendance
  Student attendance statistics and history log.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { api } from "$lib/api";
import { Calendar, ClockRotateRight, UserBadgeCheck, UserMinus } from "@orbis/icons";
import { onMount } from "svelte";
import { getLocale } from "$lib/paraglide/runtime";

import { formatNumber, formatPercent, formatDate, l } from "$lib/core/i18n";
import { DataTable } from "$lib/tables";
import { studentAttendance as saDef, withIndex } from "$lib/tables/definitions";
import { academicContext } from "$lib/stores";
import { applyFilter } from "@orbis/ui/filter-builder";
import type { UniversalFilter } from "@orbis/ui/filter-builder";
import { attendanceFilterFields } from "$lib/tables/definitions/filter-configs";

let { orgId, studentId }: { orgId: string; studentId: string } = $props();

const periodId = $derived(academicContext.effectivePeriodId || "");

let report = $state<any>(null);
let isLoading = $state(true);

onMount(() => loadAttendance());

async function loadAttendance() {
  isLoading = true;
  try {
    if (!orgId || !studentId || !periodId) return;
    const enrollments = await api.listEnrollments();
    const activeEnrollment = enrollments.find(
      (e: any) => e.student_id === studentId,
    );
    if (activeEnrollment) {
      report = null;
    }
  } catch (e) {
    console.error(e);
  } finally {
    isLoading = false;
  }
}

let activeFilter = $state<UniversalFilter>({
  logic: "AND",
  conditions: [],
  groups: [],
});

const filteredAttendance = $derived(
  applyFilter(report?.logs || [], activeFilter),
);

const gridSource = $derived(
  withIndex(
    filteredAttendance.map((log: any) => ({
      ...log,
      date: formatDate(log.date) || "—",
      status: String(log.status || "")
        .toLowerCase()
        .replace("حاضر", "present")
        .replace("غائب", "absent")
        .replace("متأخر", "late"),
      subject_name: log.subject_name || "—",
      notes: log.notes || "—",
      recorded_by_name: log.recorded_by_name || "—",
    })),
  ),
);

const columns = $derived(
  saDef.columns({
    idx: "#",
    date: m.common_overview_date(),
    status: m.common_word_status(),
    notes: m.academics_attendance_col_notes(),
  }),
);

const rtl = $derived(getLocale() === "ar");
</script>

<div class="flex flex-col gap-6 text-xs text-start">
  <!-- Attendance Stats -->
  {#if report}
    <div class="grid grid-cols-4 gap-3 shrink-0 select-none">
      <div class="border border-border bg-card/45 p-4 rounded-xl flex items-center justify-between shadow-sm">
        <div class="flex flex-col">
          <span class="text-[9px] text-muted-foreground">{m.academics_attendance_present_days()}</span>
          <span class="text-sm font-bold mt-0.5 text-emerald-500">{formatNumber(report.present_count || 0)} {m.academics_attendance_day_unit()}</span>
        </div>
        <UserBadgeCheck size={16} class="text-emerald-500" />
      </div>
      <div class="border border-border bg-card/45 p-4 rounded-xl flex items-center justify-between shadow-sm">
        <div class="flex flex-col">
          <span class="text-[9px] text-muted-foreground">{m.academics_attendance_absent_days()}</span>
          <span class="text-sm font-bold mt-0.5 text-destructive">{formatNumber(report.absent_count || 0)} {m.academics_attendance_day_unit()}</span>
        </div>
        <UserMinus size={16} class="text-destructive" />
      </div>
      <div class="border border-border bg-card/45 p-4 rounded-xl flex items-center justify-between shadow-sm">
        <div class="flex flex-col">
          <span class="text-[9px] text-muted-foreground">{m.academics_attendance_late_cases()}</span>
          <span class="text-sm font-bold mt-0.5 text-amber-500">{formatNumber(report.late_count || 0)} {m.academics_attendance_times_unit()}</span>
        </div>
        <ClockRotateRight size={16} class="text-amber-500" />
      </div>
      <div class="border border-border bg-card/45 p-4 rounded-xl flex items-center justify-between shadow-sm">
        <div class="flex flex-col">
          <span class="text-[9px] text-muted-foreground">{m.common_card_attendance_rate()}</span>
          <span class="text-sm font-bold mt-0.5 text-gradient-brand">{formatPercent(report.rate ?? 100.0)}</span>
        </div>
        <Calendar size={16} class="text-primary" />
      </div>
    </div>
  {/if}

  <!-- Attendance Log -->
  <div class="flex flex-col gap-2">
    <span class="font-bold text-[10px] text-muted-foreground uppercase tracking-wider px-1">{m.academics_attendance_log_title()}</span>
    <DataTable
      {columns}
      source={gridSource}
      loading={isLoading}
      filterable={true}
      filterFields={attendanceFilterFields}
      bind:filterValue={activeFilter}
      emptyIcon={Calendar}
      emptyTitle={l("لا توجد سجلات حضور", "No attendance records")}
      emptyDescription={l("لم يتم تسجيل أي حضور لهذا الطالب بعد", "No attendance recorded for this student yet.")}
      {rtl}
      height="350px"
    />
  </div>
</div>
