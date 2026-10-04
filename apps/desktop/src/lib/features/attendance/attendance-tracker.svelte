<!-- Attendance Tracker workspace component. -->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import { api, extractErrorMessage } from "$lib/api";
import { academicContext } from "$lib/stores";
import { formatDate, formatPercent, formatNumber, l } from "$lib/core/i18n";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Badge } from "@orbis/ui/badge";
import { Card, CardContent } from "@orbis/ui/card";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui/select";
import {
  Grid,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@orbis/ui/table";
import { toast } from "@orbis/ui";
import { Calendar, CheckCircle, ClockRotateRight, FloppyDisk, InfoCircle, Search, Users, XMark } from "@orbis/icons";
import { exportToCsv } from "$lib/shared/utils/export";

let { orgId }: { orgId: string } = $props();

interface AttendanceRow {
  studentId: string;
  studentNumber: string;
  fullName: string;
  avatar?: string | null;
  status: "present" | "absent" | "late" | "excused";
  remarks: string;
  existingId?: string;
}

let classes = $state<any[]>([]);
let selectedClassId = $state<string>("");
let selectedDate = $state<string>(new Date().toISOString().slice(0, 10));
let attendanceRows = $state<AttendanceRow[]>([]);
let searchQuery = $state("");
let statusFilter = $state("all");

let isLoadingClasses = $state(false);
let isLoadingRoster = $state(false);
let isSaving = $state(false);

onMount(async () => {
  isLoadingClasses = true;
  try {
    const classList = await api.listClasses();
    classes = classList || [];
    if (classes.length > 0) {
      selectedClassId = String(classes[0].id);
      await loadRosterAndAttendance();
    }
  } catch (e) {
    console.error("Failed to load classes:", e);
    toast.error(m.operations_ui_attendance_class_list_load_error());
  } finally {
    isLoadingClasses = false;
  }
});

async function loadRosterAndAttendance() {
  if (!selectedClassId) return;

  isLoadingRoster = true;
  try {
    const periodId = academicContext.effectivePeriodId;
    const [enrollments, existingAttendance] = await Promise.all([
      api.listEnrollments({
        classId: selectedClassId,
        periodId: periodId || undefined,
        status: "active",
      }),
      api.listAttendance(selectedClassId, selectedDate).catch(() => []),
    ]);

    const attMap = new Map<string, any>();
    for (const a of existingAttendance || []) {
      attMap.set(String(a.student_id), a);
    }

    attendanceRows = (enrollments || []).map((e: any) => {
      const existing = attMap.get(String(e.student_id || e.id));
      const rawStatus = existing?.status?.toLowerCase();
      let status: "present" | "absent" | "late" | "excused" = "present";
      if (rawStatus === "absent" || rawStatus === "غائب") status = "absent";
      else if (rawStatus === "late" || rawStatus === "متأخر") status = "late";
      else if (rawStatus === "excused" || rawStatus === "مبرر") status = "excused";

      return {
        studentId: String(e.student_id || e.id),
        studentNumber: e.student_number || e.student?.student_number || "—",
        fullName:
          e.full_name ||
          e.name_ar ||
          e.name_en ||
          e.student?.full_name ||
          m.operations_ui_student_fallback(),
        avatar: e.avatar || null,
        status,
        remarks: existing?.remarks || "",
        existingId: existing?.id,
      };
    });
  } catch (e) {
    console.error("Failed to load attendance roster:", e);
    toast.error(m.operations_ui_attendance_roster_load_error());
  } finally {
    isLoadingRoster = false;
  }
}

function markAll(status: "present" | "absent" | "late" | "excused") {
  attendanceRows = attendanceRows.map((r) => ({ ...r, status }));
  toast.success(
    status === "present"
      ? m.operations_ui_attendance_marked_all_present()
      : status === "absent"
        ? m.operations_ui_attendance_marked_all_absent()
        : m.operations_ui_attendance_status_updated_all(),
  );
}

async function handleSaveAttendance() {
  if (!selectedClassId || attendanceRows.length === 0) return;

  isSaving = true;
  try {
    await api.batchMarkAttendance({
      class_id: selectedClassId,
      date: selectedDate,
      items: attendanceRows.map((row) => ({
        student_id: row.studentId,
        status: row.status,
        notes: row.remarks || undefined,
      })),
    });
    toast.success(m.operations_ui_attendance_save_success());
    await loadRosterAndAttendance();
  } catch (e) {
    console.error("Failed to save attendance:", e);
    toast.error(extractErrorMessage(e) || m.operations_ui_attendance_save_error());
  } finally {
    isSaving = false;
  }
}

function handleExport() {
  const selectedClass = classes.find((c) => String(c.id) === selectedClassId);
  const className = selectedClass
    ? l(selectedClass.name_ar, selectedClass.name) || selectedClass.name
    : m.operations_ui_class_fallback();

  exportToCsv(
    m.operations_ui_attendance_export_filename({ className, date: selectedDate }),
    attendanceRows,
    [
      { header: m.operations_ui_student_number(), accessor: (r) => r.studentNumber },
      { header: m.operations_ui_student_name(), accessor: (r) => r.fullName },
      {
        header: m.operations_ui_status(),
        accessor: (r) =>
          r.status === "present"
            ? m.operations_ui_attendance_present()
            : r.status === "absent"
              ? m.operations_ui_attendance_absent()
              : r.status === "late"
                ? m.operations_ui_attendance_late()
                : m.operations_ui_attendance_excused(),
      },
      { header: m.operations_ui_notes(), accessor: (r) => r.remarks },
    ],
  );
}

const totalCount = $derived(attendanceRows.length);
const presentCount = $derived(
  attendanceRows.filter((r) => r.status === "present").length,
);
const absentCount = $derived(
  attendanceRows.filter((r) => r.status === "absent").length,
);
const lateCount = $derived(
  attendanceRows.filter((r) => r.status === "late").length,
);
const excusedCount = $derived(
  attendanceRows.filter((r) => r.status === "excused").length,
);
const attendanceRate = $derived(
  totalCount > 0 ? Math.round((presentCount / totalCount) * 100) : 0,
);

const filteredRows = $derived(
  attendanceRows.filter((r) => {
    const matchesSearch =
      !searchQuery ||
      r.fullName.toLowerCase().includes(searchQuery.toLowerCase()) ||
      r.studentNumber.toLowerCase().includes(searchQuery.toLowerCase());
    const matchesStatus =
      statusFilter === "all" || r.status === statusFilter;
    return matchesSearch && matchesStatus;
  }),
);
</script>

<div class="p-6 max-w-7xl mx-auto space-y-6 text-start">
  <!-- Header Card -->
  <div class="relative overflow-hidden rounded-2xl border border-border/80 bg-linear-to-b from-card to-card/60 p-6 shadow-sm backdrop-blur-md">
    <div class="flex flex-col md:flex-row md:items-center justify-between gap-4">
      <div class="flex items-center gap-3.5">
        <div class="w-12 h-12 rounded-2xl bg-primary/10 border border-primary/20 flex items-center justify-center text-primary shadow-xs">
          <Calendar class="w-6 h-6" />
        </div>
        <div>
          <h1 class="text-xl font-bold text-foreground tracking-tight">
            {m.operations_ui_attendance_daily_title()}
          </h1>
          <p class="text-xs text-muted-foreground mt-0.5">
            {m.operations_ui_attendance_daily_description()}
          </p>
        </div>
      </div>

      <!-- Controls: Class & Date Pickers -->
      <div class="flex flex-wrap items-center gap-2.5">
        <div class="w-48">
          <Select
            value={selectedClassId}
            onValueChange={(val) => {
              selectedClassId = val;
              loadRosterAndAttendance();
            }}
          >
            <SelectTrigger class="h-9 text-xs">
              <span>
                {classes.find((c) => String(c.id) === selectedClassId)
                  ? l(
                      classes.find((c) => String(c.id) === selectedClassId)?.name_ar,
                      classes.find((c) => String(c.id) === selectedClassId)?.name,
                    )
                  : m.operations_ui_attendance_select_class()}
              </span>
            </SelectTrigger>
            <SelectContent>
              {#each classes as c}
                <SelectItem value={String(c.id)}>
                  {l(c.name_ar, c.name) || c.name || c.id}
                </SelectItem>
              {/each}
            </SelectContent>
          </Select>
        </div>

        <Input
          type="date"
          bind:value={selectedDate}
          onchange={loadRosterAndAttendance}
          class="h-9 text-xs w-36"
        />

        <Button
          variant="default"
          size="sm"
          onclick={handleSaveAttendance}
          disabled={isSaving || attendanceRows.length === 0}
          class="h-9 text-xs font-bold gap-1.5 shadow-sm"
        >
          <FloppyDisk class="w-4 h-4" />
          {isSaving ? m.operations_ui_saving() : m.operations_ui_attendance_save_record()}
        </Button>
      </div>
    </div>

    <!-- KPI Stats Bar -->
    <div class="grid grid-cols-2 sm:grid-cols-5 gap-3 mt-6 pt-4 border-t border-border/60">
      <div class="flex items-center gap-3">
        <div class="w-8 h-8 rounded-lg bg-primary/10 flex items-center justify-center text-primary">
          <Users class="w-4 h-4" />
        </div>
        <div>
          <div class="text-lg font-bold text-foreground font-mono leading-none">{formatNumber(totalCount)}</div>
          <div class="text-[11px] text-muted-foreground mt-0.5">{m.operations_ui_total_students()}</div>
        </div>
      </div>

      <div class="flex items-center gap-3">
        <div class="w-8 h-8 rounded-lg bg-emerald-500/10 flex items-center justify-center text-emerald-600 dark:text-emerald-400">
          <CheckCircle class="w-4 h-4" />
        </div>
        <div>
          <div class="text-lg font-bold text-emerald-600 dark:text-emerald-400 font-mono leading-none">{formatNumber(presentCount)}</div>
          <div class="text-[11px] text-muted-foreground mt-0.5">{m.operations_ui_attendance_present()} ({formatPercent(attendanceRate)})</div>
        </div>
      </div>

      <div class="flex items-center gap-3">
        <div class="w-8 h-8 rounded-lg bg-red-500/10 flex items-center justify-center text-red-600 dark:text-red-400">
          <XMark class="w-4 h-4" />
        </div>
        <div>
          <div class="text-lg font-bold text-red-600 dark:text-red-400 font-mono leading-none">{formatNumber(absentCount)}</div>
          <div class="text-[11px] text-muted-foreground mt-0.5">{m.operations_ui_attendance_absent()}</div>
        </div>
      </div>

      <div class="flex items-center gap-3">
        <div class="w-8 h-8 rounded-lg bg-amber-500/10 flex items-center justify-center text-amber-600 dark:text-amber-400">
          <ClockRotateRight class="w-4 h-4" />
        </div>
        <div>
          <div class="text-lg font-bold text-amber-600 dark:text-amber-400 font-mono leading-none">{formatNumber(lateCount)}</div>
          <div class="text-[11px] text-muted-foreground mt-0.5">{m.operations_ui_attendance_late()}</div>
        </div>
      </div>

      <div class="flex items-center gap-3">
        <div class="w-8 h-8 rounded-lg bg-blue-500/10 flex items-center justify-center text-blue-600 dark:text-blue-400">
          <InfoCircle class="w-4 h-4" />
        </div>
        <div>
          <div class="text-lg font-bold text-blue-600 dark:text-blue-400 font-mono leading-none">{formatNumber(excusedCount)}</div>
          <div class="text-[11px] text-muted-foreground mt-0.5">{m.operations_ui_attendance_excused_absence()}</div>
        </div>
      </div>
    </div>
  </div>

  <!-- Grid & Bulk Controls -->
  <Card class="border border-border/80 shadow-xs">
    <div class="p-4 border-b border-border/60 flex flex-col sm:flex-row items-start sm:items-center justify-between gap-3">
      <!-- Search & Filters -->
      <div class="flex items-center gap-2 w-full sm:w-auto">
        <div class="relative w-full sm:w-64">
          <Search class="absolute start-2.5 top-1/2 -translate-y-1/2 w-3.5 h-3.5 text-muted-foreground" />
          <Input
            type="search"
            placeholder={m.operations_ui_attendance_search_placeholder()}
            bind:value={searchQuery}
            class="h-8 ps-8 text-xs w-full"
          />
        </div>

        <Select value={statusFilter} onValueChange={(v) => (statusFilter = v)}>
          <SelectTrigger class="h-8 text-xs w-32">
            <span>
              {statusFilter === "all"
                ? m.operations_ui_all_statuses()
                : statusFilter === "present"
                  ? m.operations_ui_attendance_present()
                  : statusFilter === "absent"
                    ? m.operations_ui_attendance_absent()
                    : statusFilter === "late"
                      ? m.operations_ui_attendance_late()
                      : m.operations_ui_attendance_excused()}
            </span>
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="all">{m.operations_ui_all_statuses()}</SelectItem>
            <SelectItem value="present">{m.operations_ui_attendance_present()}</SelectItem>
            <SelectItem value="absent">{m.operations_ui_attendance_absent()}</SelectItem>
            <SelectItem value="late">{m.operations_ui_attendance_late()}</SelectItem>
            <SelectItem value="excused">{m.operations_ui_attendance_excused()}</SelectItem>
          </SelectContent>
        </Select>
      </div>

      <!-- Quick Batch Actions -->
      <div class="flex items-center gap-2">
        <Button
          variant="outline"
          size="sm"
          onclick={() => markAll("present")}
          class="h-8 text-xs text-emerald-600 hover:text-emerald-700 hover:bg-emerald-500/10 font-semibold"
        >
          <CheckCircle class="w-3.5 h-3.5 me-1" />
          {m.operations_ui_attendance_all_present()}
        </Button>

        <Button
          variant="outline"
          size="sm"
          onclick={() => markAll("absent")}
          class="h-8 text-xs text-red-600 hover:text-red-700 hover:bg-red-500/10 font-semibold"
        >
          <XMark class="w-3.5 h-3.5 me-1" />
          {m.operations_ui_attendance_all_absent()}
        </Button>

        <Button
          variant="outline"
          size="sm"
          onclick={handleExport}
          disabled={attendanceRows.length === 0}
          class="h-8 text-xs font-semibold"
        >
          {m.operations_ui_export_csv()}
        </Button>
      </div>
    </div>

    <!-- Roster Grid -->
    <CardContent class="p-0">
      <Grid>
        <TableHeader class="bg-muted/40">
          <TableRow>
            <TableHead class="text-xs font-bold w-12 text-center">#</TableHead>
            <TableHead class="text-xs font-bold w-32">{m.operations_ui_student_number()}</TableHead>
            <TableHead class="text-xs font-bold">{m.operations_ui_student_name()}</TableHead>
            <TableHead class="text-xs font-bold w-80 text-center">{m.operations_ui_attendance_status()}</TableHead>
            <TableHead class="text-xs font-bold w-64">{m.operations_ui_notes()}</TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          {#if isLoadingRoster}
            <TableRow>
              <TableCell colspan={5} class="text-center py-12 text-xs text-muted-foreground">
                {m.operations_ui_attendance_loading_students()}
              </TableCell>
            </TableRow>
          {:else if filteredRows.length === 0}
            <TableRow>
              <TableCell colspan={5} class="text-center py-12 text-xs text-muted-foreground">
                {m.operations_ui_attendance_no_students()}
              </TableCell>
            </TableRow>
          {:else}
            {#each filteredRows as row, idx}
              <TableRow class="hover:bg-muted/20 transition-colors">
                <TableCell class="text-center text-xs font-mono text-muted-foreground">
                  {idx + 1}
                </TableCell>
                <TableCell class="font-mono text-xs font-semibold text-foreground">
                  {row.studentNumber}
                </TableCell>
                <TableCell>
                  <div class="flex items-center gap-2.5">
                    <div class="w-7 h-7 rounded-full bg-primary/10 border border-primary/20 flex items-center justify-center text-xs font-bold text-primary shrink-0">
                      {row.fullName.charAt(0)}
                    </div>
                    <span class="text-xs font-bold text-foreground">
                      {row.fullName}
                    </span>
                  </div>
                </TableCell>
                <TableCell>
                  <div class="flex items-center justify-center gap-1.5">
                    <button
                      type="button"
                      onclick={() => (row.status = "present")}
                      class="px-2.5 py-1 rounded-lg text-xs font-semibold transition-all cursor-pointer {row.status === 'present' ? 'bg-emerald-600 text-white shadow-xs scale-105' : 'bg-muted/60 text-muted-foreground hover:bg-emerald-500/10 hover:text-emerald-600'}"
                    >
                      {m.operations_ui_attendance_present()}
                    </button>
                    <button
                      type="button"
                      onclick={() => (row.status = "absent")}
                      class="px-2.5 py-1 rounded-lg text-xs font-semibold transition-all cursor-pointer {row.status === 'absent' ? 'bg-red-600 text-white shadow-xs scale-105' : 'bg-muted/60 text-muted-foreground hover:bg-red-500/10 hover:text-red-600'}"
                    >
                      {m.operations_ui_attendance_absent()}
                    </button>
                    <button
                      type="button"
                      onclick={() => (row.status = "late")}
                      class="px-2.5 py-1 rounded-lg text-xs font-semibold transition-all cursor-pointer {row.status === 'late' ? 'bg-amber-600 text-white shadow-xs scale-105' : 'bg-muted/60 text-muted-foreground hover:bg-amber-500/10 hover:text-amber-600'}"
                    >
                      {m.operations_ui_attendance_late()}
                    </button>
                    <button
                      type="button"
                      onclick={() => (row.status = "excused")}
                      class="px-2.5 py-1 rounded-lg text-xs font-semibold transition-all cursor-pointer {row.status === 'excused' ? 'bg-blue-600 text-white shadow-xs scale-105' : 'bg-muted/60 text-muted-foreground hover:bg-blue-500/10 hover:text-blue-600'}"
                    >
                      {m.operations_ui_attendance_excused()}
                    </button>
                  </div>
                </TableCell>
                <TableCell>
                  <Input
                    type="text"
                    placeholder={m.operations_ui_attendance_notes_placeholder()}
                    bind:value={row.remarks}
                    class="h-7 text-xs"
                  />
                </TableCell>
              </TableRow>
            {/each}
          {/if}
        </TableBody>
      </Grid>
    </CardContent>
  </Card>
</div>
