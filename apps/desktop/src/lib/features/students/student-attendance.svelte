<!--
  @component StudentAttendance
  Student attendance statistics and history log with OrbisGrid inside standardized Cards.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { api, type AttendanceDto } from "$lib/api";
import {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardAction,
  CardContent,
} from "@orbis/ui/card";
import {
  Calendar,
  ClockRotateRight,
  UserBadgeCheck,
  UserMinus,
} from "@orbis/icons";
import StudentMetricCard from "$lib/features/students/components/student-metric-card.svelte";
import StudentSectionState from "$lib/features/students/components/student-section-state.svelte";
import { onMount } from "svelte";
import { getLocale } from "$lib/paraglide/runtime";

import { formatNumber, formatPercent, formatDate } from "$lib/core/i18n";
import { OrbisGrid, type OrbisColumnDef } from "@orbis/svelte-grid";
import { academicContext } from "$lib/stores";
import { FilterTrigger, type UniversalFilter } from "@orbis/ui/filter-builder";
import { attendanceFilterFields } from "$lib/core/filter-configs";

let { orgId, studentId }: { orgId: string; studentId: string } = $props();

const periodId = $derived(academicContext.effectivePeriodId || "");

let attendanceRecords = $state<any[]>([]);
let isLoading = $state(true);

const presentCount = $derived(
  attendanceRecords.filter(
    (r) =>
      String(r.status || "")
        .toLowerCase()
        .includes("present") || String(r.status || "").includes("حاضر"),
  ).length,
);
const absentCount = $derived(
  attendanceRecords.filter(
    (r) =>
      String(r.status || "")
        .toLowerCase()
        .includes("absent") || String(r.status || "").includes("غائب"),
  ).length,
);
const lateCount = $derived(
  attendanceRecords.filter(
    (r) =>
      String(r.status || "")
        .toLowerCase()
        .includes("late") || String(r.status || "").includes("متأخر"),
  ).length,
);
const totalDays = $derived(presentCount + absentCount + lateCount);
const attendanceRate = $derived(
  totalDays > 0 ? ((presentCount + lateCount * 0.5) / totalDays) * 100 : 100,
);

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
  attendanceRecords.map((log: AttendanceDto, i: number) => ({
    ...log,
    __idx: i + 1,
    date: formatDate(log.date || (log as any).created_at) || "—",
    status: String(log.status || "")
      .toLowerCase()
      .replace("حاضر", "present")
      .replace("غائب", "absent")
      .replace("متأخر", "late")
      .replace("معذور", "excused"),
    subject_name: (log as any).subject_name || log.section_subject_id || "—",
    notes: log.notes || (log as any).remarks || "—",
  })),
);

const columns: OrbisColumnDef[] = [
  {
    id: "__idx",
    prop: "__idx",
    header: "#",
    name: "#",
    width: 48,
    size: 48,
    readonly: true,
    pin: "left",
    align: "center",
  },
  {
    id: "date",
    prop: "date",
    header: m.common_overview_date(),
    name: m.common_overview_date(),
    width: 130,
    size: 130,
    readonly: true,
    cellClass: "font-mono",
  },
  {
    id: "status",
    prop: "status",
    header: m.common_word_status(),
    name: m.common_word_status(),
    width: 110,
    size: 110,
    type: "badge",
    align: "center",
    readonly: true,
  },
  {
    id: "subject_name",
    prop: "subject_name",
    header: m.common_search_group_subjects(),
    name: m.common_search_group_subjects(),
    width: 140,
    size: 140,
    readonly: true,
  },
  {
    id: "notes",
    prop: "notes",
    header: m.academics_attendance_col_notes(),
    name: m.academics_attendance_col_notes(),
    width: 220,
    size: 220,
    readonly: true,
  },
];

const rtl = $derived(getLocale() === "ar");
</script>

<div class="flex flex-col gap-6 text-xs text-start font-sans">
  <!-- Attendance Stats -->
  <div class="grid grid-cols-2 gap-3 xl:grid-cols-4">
    <StudentMetricCard
      title={m.academics_attendance_present_days()}
      value={`${formatNumber(presentCount)} ${m.academics_attendance_day_unit()}`}
      icon={UserBadgeCheck}
      tone="success"
      loading={isLoading}
    />
    <StudentMetricCard
      title={m.academics_attendance_absent_days()}
      value={`${formatNumber(absentCount)} ${m.academics_attendance_day_unit()}`}
      icon={UserMinus}
      tone="danger"
      loading={isLoading}
    />
    <StudentMetricCard
      title={m.academics_attendance_late_cases()}
      value={`${formatNumber(lateCount)} ${m.academics_attendance_times_unit()}`}
      icon={ClockRotateRight}
      tone="warning"
      loading={isLoading}
    />
    <StudentMetricCard
      title={m.common_card_attendance_rate()}
      value={attendanceRecords.length ? formatPercent(attendanceRate) : "—"}
      icon={Calendar}
      loading={isLoading}
    />
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
      {#if !isLoading && attendanceRecords.length === 0}
        <StudentSectionState
          icon={Calendar}
          title={m.ui_copy_no_attendance_records_7b6831c5()}
          description={m.ui_copy_no_attendance_recorded_for_this_student_ye_f2c2d408()}
        />
      {:else}
      <div class="h-80 w-full">
        <OrbisGrid
          {columns}
          source={gridSource}
          loading={isLoading}
          clientQuery
          filter={activeFilter}
          {rtl}
          readonly
        />
      </div>
      {/if}
    </CardContent>
  </Card>
</div>
