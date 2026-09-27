<!--
  @component StudentAttendance
  Student attendance statistics and history log with OrbisGrid inside standardized Cards.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { api } from "$lib/api";
import {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardAction,
  CardContent,
} from "@orbis/ui/card";
import { Calendar, ClockRotateRight, UserBadgeCheck, UserMinus } from "@orbis/icons";
import { onMount } from "svelte";
import { getLocale } from "$lib/paraglide/runtime";

import { formatNumber, formatPercent, formatDate, l } from "$lib/core/i18n";
import { OrbisGrid, type OrbisColumnDef } from "@orbis/svelte-grid";
import { academicContext } from "$lib/stores";
import { FilterTrigger, type UniversalFilter } from "@orbis/ui/filter-builder";
import { attendanceFilterFields } from "$lib/core/filter-configs";

let { orgId, studentId }: { orgId: string; studentId: string } = $props();

const periodId = $derived(academicContext.effectivePeriodId || "");

let attendanceRecords = $state<any[]>([]);
let isLoading = $state(true);

const presentCount = $derived(
  attendanceRecords.filter((r) => String(r.status || "").toLowerCase().includes("present") || String(r.status || "").includes("حاضر")).length,
);
const absentCount = $derived(
  attendanceRecords.filter((r) => String(r.status || "").toLowerCase().includes("absent") || String(r.status || "").includes("غائب")).length,
);
const lateCount = $derived(
  attendanceRecords.filter((r) => String(r.status || "").toLowerCase().includes("late") || String(r.status || "").includes("متأخر")).length,
);
const totalDays = $derived(presentCount + absentCount + lateCount);
const attendanceRate = $derived(totalDays > 0 ? ((presentCount + lateCount * 0.5) / totalDays) * 100 : 100);

onMount(() => loadAttendance());

async function loadAttendance() {
  isLoading = true;
  try {
    if (!orgId || !studentId) return;
    attendanceRecords = await api.listStudentAttendance(studentId);
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

const gridSource = $derived(
  attendanceRecords.map((log: any, i: number) => ({
    ...log,
    __idx: i + 1,
    date: formatDate(log.date || log.created_at) || "—",
    status: String(log.status || "")
      .toLowerCase()
      .replace("حاضر", "present")
      .replace("غائب", "absent")
      .replace("متأخر", "late"),
    subject_name: log.subject_name || log.subject_id || "—",
    notes: log.remarks || log.notes || "—",
    recorded_by_name: log.recorded_by_name || "—",
  })),
);

const columns: OrbisColumnDef[] = [
  { id: "__idx", prop: "__idx", header: "#", name: "#", width: 48, size: 48, readonly: true, pin: "left", align: "center" },
  { id: "date", prop: "date", header: m.common_overview_date(), name: m.common_overview_date(), width: 130, size: 130, readonly: true, cellClass: "font-mono" },
  { id: "status", prop: "status", header: m.common_word_status(), name: m.common_word_status(), width: 110, size: 110, type: "badge", align: "center", readonly: true },
  { id: "subject_name", prop: "subject_name", header: m.common_search_group_subjects(), name: "Subject", width: 140, size: 140, readonly: true },
  { id: "notes", prop: "notes", header: m.academics_attendance_col_notes(), name: m.academics_attendance_col_notes(), width: 220, size: 220, readonly: true },
];

const rtl = $derived(getLocale() === "ar");
</script>

<div class="flex flex-col gap-6 text-xs text-start font-sans">
  <!-- Attendance Stats -->
  <div class="grid grid-cols-2 sm:grid-cols-4 gap-3 shrink-0 select-none">
    <Card class="flex items-center justify-between p-4">
      <div class="flex flex-col">
        <span class="text-[9px] text-muted-foreground font-semibold">{m.academics_attendance_present_days()}</span>
        <span class="text-sm font-bold font-mono mt-0.5 text-emerald-600 dark:text-emerald-400">
          {formatNumber(presentCount)} {m.academics_attendance_day_unit()}
        </span>
      </div>
      <UserBadgeCheck size={18} class="text-emerald-500" />
    </Card>

    <Card class="flex items-center justify-between p-4">
      <div class="flex flex-col">
        <span class="text-[9px] text-muted-foreground font-semibold">{m.academics_attendance_absent_days()}</span>
        <span class="text-sm font-bold font-mono mt-0.5 text-destructive">
          {formatNumber(absentCount)} {m.academics_attendance_day_unit()}
        </span>
      </div>
      <UserMinus size={18} class="text-destructive" />
    </Card>

    <Card class="flex items-center justify-between p-4">
      <div class="flex flex-col">
        <span class="text-[9px] text-muted-foreground font-semibold">{m.academics_attendance_late_cases()}</span>
        <span class="text-sm font-bold font-mono mt-0.5 text-amber-500">
          {formatNumber(lateCount)} {m.academics_attendance_times_unit()}
        </span>
      </div>
      <ClockRotateRight size={18} class="text-amber-500" />
    </Card>

    <Card class="flex items-center justify-between p-4">
      <div class="flex flex-col">
        <span class="text-[9px] text-muted-foreground font-semibold">{m.common_card_attendance_rate()}</span>
        <span class="text-sm font-bold font-mono mt-0.5 text-primary">{formatPercent(attendanceRate)}</span>
      </div>
      <Calendar size={18} class="text-primary" />
    </Card>
  </div>

  <!-- Attendance Log Card -->
  <Card class="overflow-hidden">
    <CardHeader class="py-3 px-4 flex flex-row items-center justify-between">
      <div class="flex items-center gap-2">
        <Calendar size={14} class="text-primary" />
        <CardTitle class="text-xs font-bold uppercase tracking-wider text-foreground">
          {m.academics_attendance_log_title()}
        </CardTitle>
      </div>

      <CardAction>
        <FilterTrigger
          bind:filter={activeFilter}
          fields={attendanceFilterFields}
        />
      </CardAction>
    </CardHeader>

    <CardContent class="p-0">
      <div class="h-[350px] w-full">
        <OrbisGrid
          {columns}
          source={gridSource}
          loading={isLoading}
          clientQuery
          filter={activeFilter}
          emptyTitle={l("لا توجد سجلات حضور", "No attendance records")}
          emptyDescription={l("لم يتم تسجيل أي حضور لهذا الطالب بعد", "No attendance recorded for this student yet.")}
          {rtl}
          readonly
        />
      </div>
    </CardContent>
  </Card>
</div>
