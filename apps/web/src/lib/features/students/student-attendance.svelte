<!--
  @component StudentAttendance
  Student attendance statistics and history log.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { extractErrorMessage, type AttendanceDto } from "$lib/api";
import {
  Calendar,
  ClockRotateRight,
  UserBadgeCheck,
  UserMinus,
} from "@orbis/icons";
import { Button } from "@orbis/ui/button";
import StudentMetricCard from "$lib/features/students/components/student-metric-card.svelte";
import StudentSectionState from "$lib/features/students/components/student-section-state.svelte";
import { createStudentAttendanceQuery } from "$lib/features/students/queries";
import { getLocale } from "$lib/paraglide/runtime";
import { formatNumber, formatPercent } from "$lib/core/i18n";
import { OrbisGrid, type OrbisColumnDef } from "@orbis/svelte-grid";
import { FilterTrigger, type UniversalFilter } from "@orbis/ui/filter-builder";
import { attendanceFilterFields } from "$lib/core/filter-configs";

let { orgId, studentId }: { orgId: string; studentId: string } = $props();

const attendanceQuery = createStudentAttendanceQuery(() => studentId);
const attendanceRecords = $derived<AttendanceDto[]>(
  attendanceQuery.data ?? [],
);
const isLoading = $derived(attendanceQuery.isLoading);

const presentCount = $derived(
  attendanceRecords.filter((r) => {
    const status = String(r.status || "").toLowerCase();
    return status.includes("present") || status.includes("حاضر");
  }).length,
);
const absentCount = $derived(
  attendanceRecords.filter((r) => {
    const status = String(r.status || "").toLowerCase();
    return status.includes("absent") || status.includes("غائب");
  }).length,
);
const lateCount = $derived(
  attendanceRecords.filter((r) => {
    const status = String(r.status || "").toLowerCase();
    return status.includes("late") || status.includes("متأخر");
  }).length,
);
const totalDays = $derived(presentCount + absentCount + lateCount);
const attendanceRate = $derived(
  totalDays > 0 ? ((presentCount + lateCount * 0.5) / totalDays) * 100 : 100,
);

let activeFilter = $state<UniversalFilter>({
  logic: "AND",
  conditions: [],
  groups: [],
});

const gridSource = $derived(
  attendanceRecords.map((log, i) => ({
    ...log,
    __idx: i + 1,
    date: log.date || log.created_at || null,
    status: String(log.status || "")
      .toLowerCase()
      .replace("حاضر", "present")
      .replace("غائب", "absent")
      .replace("متأخر", "late")
      .replace("معذور", "excused"),
    subject_name: log.section_subject_id || "—",
    notes: log.notes || "—",
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
    type: "date",
    sortable: true,
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

{#if attendanceQuery.isError}
  <StudentSectionState
    icon={Calendar}
    title={m.common_search_load_failed()}
    description={extractErrorMessage(attendanceQuery.error)}
  >
    {#snippet action()}
      <Button
        variant="outline"
        size="sm"
        onclick={() => void attendanceQuery.refetch()}
      >
        {m.common_retry()}
      </Button>
    {/snippet}
  </StudentSectionState>
{:else}
  <div class="flex flex-col gap-6 text-xs text-start font-sans">
    <div
      class="grid grid-cols-1 gap-3 @min-[26rem]/workspace:grid-cols-2 @min-[56rem]/workspace:grid-cols-4"
    >
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

    <section class="flex flex-col gap-2">
      <h3 class="text-xs font-semibold text-foreground">
        {m.academics_attendance_log_title()}
      </h3>
      <div class="h-[400px] w-full">
        <OrbisGrid
          {columns}
          source={gridSource}
          loading={isLoading}
          clientQuery
          showFilterButton={false}
          filter={activeFilter}
          emptyTitle={m.ui_copy_no_attendance_records_7b6831c5()}
          emptyDescription={m.ui_copy_no_attendance_recorded_for_this_student_ye_f2c2d408()}
          {rtl}
          readonly
        >
          {#snippet toolbarFilters()}
            <FilterTrigger
              bind:filter={activeFilter}
              fields={attendanceFilterFields}
            />
          {/snippet}
        </OrbisGrid>
      </div>
    </section>
  </div>
{/if}
