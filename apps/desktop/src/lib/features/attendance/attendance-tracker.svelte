<script lang="ts">
import { page } from "$app/state";
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import {
  api,
  extractErrorMessage,
  type AttendanceDto,
  type ScheduledLessonDto,
  type SectionSubjectDto,
  type StudentDetailDto,
  type SubjectDto,
  type TimeSlotDto,
} from "$lib/api";
import { academicContext } from "$lib/stores";
import { l } from "$lib/core/i18n";
import { getLocale } from "$lib/paraglide/runtime";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Card, CardContent } from "@orbis/ui/card";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui/select";
import { toast } from "@orbis/ui";
import {
  Calendar,
  CheckCircle,
  ClockRotateRight,
  FloppyDisk,
  InfoCircle,
  Search,
  XMark,
} from "@orbis/icons";
import {
  OrbisGrid,
  type CellEditEvent,
  type OrbisColumnDef,
} from "@orbis/svelte-grid";
import { exportToCsv } from "$lib/shared/utils/export";

type AttendanceStatus = "present" | "absent" | "late" | "excused";

interface AttendanceRow {
  studentId: string;
  studentNumber: string;
  fullName: string;
  status: AttendanceStatus | "unmarked";
  remarks: string;
}

interface AttendanceLessonOption {
  id: string;
  sectionSubjectId: string;
  label: string;
}

let classes = $state<any[]>([]);
let selectedClassId = $state("");
let selectedDate = $state(new Date().toISOString().slice(0, 10));
let selectedLessonId = $state("");
let lessonOptions = $state<AttendanceLessonOption[]>([]);
let rosterStudents = $state<StudentDetailDto[]>([]);
let existingAttendance = $state<AttendanceDto[]>([]);
let attendanceRows = $state<AttendanceRow[]>([]);
let searchQuery = $state("");
let statusFilter = $state("all");
let isLoadingClasses = $state(false);
let isLoadingRoster = $state(false);
let isSaving = $state(false);
let classLoadError = $state(false);
let rosterLoadError = $state(false);

const columns: OrbisColumnDef[] = [
  {
    id: "fullName",
    prop: "fullName",
    header: m.operations_ui_student_name(),
    name: m.operations_ui_student_name(),
    width: 240,
    size: 240,
    type: "avatar",
    readonly: true,
    pin: "left",
  },
  {
    id: "studentNumber",
    prop: "studentNumber",
    header: m.operations_ui_student_number(),
    name: m.operations_ui_student_number(),
    width: 140,
    size: 140,
    readonly: true,
    cellClass: "font-mono text-muted-foreground",
  },
  {
    id: "status",
    prop: "status",
    header: m.operations_ui_attendance_status(),
    name: m.operations_ui_attendance_status(),
    width: 160,
    size: 160,
    type: "enum",
    align: "center",
    options: [
      { value: "present", label: m.operations_ui_attendance_present() },
      { value: "absent", label: m.operations_ui_attendance_absent() },
      { value: "late", label: m.operations_ui_attendance_late() },
      { value: "excused", label: m.operations_ui_attendance_excused() },
    ],
  },
  {
    id: "remarks",
    prop: "remarks",
    header: m.operations_ui_notes(),
    name: m.operations_ui_notes(),
    width: 260,
    size: 260,
    type: "text",
  },
];

async function loadClasses() {
  isLoadingClasses = true;
  classLoadError = false;
  try {
    classes = await api.listSections();
    if (classes.length > 0) {
      const requestedClassId = page.url.searchParams.get("classId");
      selectedClassId = String(
        classes.find((item) => String(item.id) === requestedClassId)?.id ??
          classes[0].id,
      );
      await loadRosterAndAttendance();
    }
  } catch (error) {
    classLoadError = true;
    console.error("Failed to load attendance classes:", error);
    toast.error(m.operations_ui_attendance_class_list_load_error());
  } finally {
    isLoadingClasses = false;
  }
}

onMount(() => {
  void loadClasses();
});

async function loadRosterAndAttendance() {
  if (!selectedClassId) return;

  isLoadingRoster = true;
  rosterLoadError = false;
  try {
    lessonOptions = [];
    selectedLessonId = "";
    attendanceRows = [];
    const periodId = academicContext.effectivePeriodId;
    const [
      studentsPage,
      records,
      sectionSubjects,
      timeSlots,
      subjects,
      lessons,
    ] = await Promise.all([
      api.listStudents({
        class_id: selectedClassId,
        period_id: periodId || undefined,
        page: 1,
        page_size: 100,
      }),
      api.listAttendance(selectedClassId, selectedDate),
      periodId
        ? api.listSectionSubjects(selectedClassId, periodId)
        : Promise.resolve([] as SectionSubjectDto[]),
      periodId
        ? api.listTimeSlots(periodId)
        : Promise.resolve([] as TimeSlotDto[]),
      api.listSubjects(),
      periodId
        ? api.listScheduledLessons(periodId)
        : Promise.resolve([] as ScheduledLessonDto[]),
    ]);

    const students: StudentDetailDto[] = [...studentsPage.items];
    for (let page = 2; page <= studentsPage.total_pages; page += 5) {
      const pageNumbers = Array.from(
        { length: Math.min(5, studentsPage.total_pages - page + 1) },
        (_, index) => page + index,
      );
      const pages = await Promise.all(
        pageNumbers.map((pageNumber) =>
          api.listStudents({
            class_id: selectedClassId,
            period_id: periodId || undefined,
            page: pageNumber,
            page_size: 100,
          }),
        ),
      );
      students.push(...pages.flatMap((result) => result.items));
    }

    const sectionSubjectById = new Map(
      sectionSubjects.map((sectionSubject) => [
        sectionSubject.id,
        sectionSubject,
      ]),
    );
    const subjectById = new Map(
      subjects.map((subject) => [subject.id, subject]),
    );
    const timeSlotById = new Map(
      timeSlots.map((timeSlot) => [timeSlot.id, timeSlot]),
    );
    const weekday = new Date(`${selectedDate}T00:00:00`).getDay() + 1;
    const classSectionSubjectIds = new Set(
      sectionSubjects.map((item) => item.id),
    );

    lessonOptions = lessons
      .filter((lesson) => classSectionSubjectIds.has(lesson.section_subject_id))
      .flatMap((lesson) => {
        const sectionSubject = sectionSubjectById.get(
          lesson.section_subject_id,
        );
        const timeSlot = timeSlotById.get(lesson.time_slot_id);
        if (!sectionSubject || !timeSlot || timeSlot.day_of_week !== weekday) {
          return [];
        }

        const subject = subjectById.get(sectionSubject.subject_id);
        const subjectName =
          l(subject?.name_ar, subject?.name) ||
          subject?.name_ar ||
          subject?.name ||
          sectionSubject.subject_id;

        return [
          {
            id: lesson.id,
            sectionSubjectId: sectionSubject.id,
            label: [
              subjectName,
              `${timeSlot.start_time.slice(0, 5)} - ${timeSlot.end_time.slice(0, 5)}`,
            ]
              .filter(Boolean)
              .join(" · "),
          },
        ];
      })
      .sort((left, right) => left.label.localeCompare(right.label));

    rosterStudents = students;
    existingAttendance = records;
    selectedLessonId = lessonOptions.some(
      (lesson) => lesson.id === selectedLessonId,
    )
      ? selectedLessonId
      : lessonOptions.length === 1
        ? lessonOptions[0].id
        : "";
    buildAttendanceRows(selectedLessonId);
  } catch (error) {
    rosterLoadError = true;
    console.error("Failed to load attendance roster:", error);
    toast.error(m.operations_ui_attendance_roster_load_error());
  } finally {
    isLoadingRoster = false;
  }
}

function buildAttendanceRows(lessonId: string) {
  const lesson = lessonOptions.find((option) => option.id === lessonId);
  if (!lesson) {
    attendanceRows = [];
    return;
  }

  const attendanceByStudent = new Map(
    existingAttendance
      .filter(
        (record) =>
          record.scheduled_lesson_id === lesson.id &&
          record.section_subject_id === lesson.sectionSubjectId,
      )
      .map((record) => [String(record.student_id), record]),
  );

  attendanceRows = rosterStudents.map((student) => {
    const studentId = String(student.student.id);
    const existing = attendanceByStudent.get(studentId);
    const rawStatus = existing?.status?.toLowerCase();
    const status: AttendanceRow["status"] = !rawStatus
      ? "unmarked"
      : rawStatus === "absent" || rawStatus === "غائب"
        ? "absent"
        : rawStatus === "late" || rawStatus === "متأخر"
          ? "late"
          : rawStatus === "excused" || rawStatus === "مبرر"
            ? "excused"
            : "present";

    return {
      studentId,
      studentNumber:
        student.enrollment?.student_number ||
        student.student.student_number ||
        "—",
      fullName:
        student.person.full_name_ar ||
        student.person.full_name ||
        student.person.display_name ||
        student.student.student_number ||
        m.operations_ui_student_fallback(),
      status,
      remarks: existing?.notes || "",
    };
  });
}

function selectLesson(lessonId: string) {
  selectedLessonId = lessonId;
  buildAttendanceRows(lessonId);
}

function isAttendanceStatus(value: unknown): value is AttendanceStatus {
  return (
    value === "present" ||
    value === "absent" ||
    value === "late" ||
    value === "excused"
  );
}

function handleGridEdit(event: CellEditEvent) {
  const studentId = String(event.row.studentId ?? event.row.id ?? "");
  const row = attendanceRows.find((item) => item.studentId === studentId);
  if (!row) return;

  if (event.columnId === "status" && isAttendanceStatus(event.newValue)) {
    row.status = event.newValue;
  } else if (event.columnId === "remarks") {
    row.remarks = String(event.newValue ?? "");
  }
}

function markAll(status: AttendanceStatus) {
  attendanceRows = attendanceRows.map((row) => ({ ...row, status }));
  toast.success(
    status === "present"
      ? m.operations_ui_attendance_marked_all_present()
      : status === "absent"
        ? m.operations_ui_attendance_marked_all_absent()
        : m.operations_ui_attendance_status_updated_all(),
  );
}

async function handleSaveAttendance() {
  const lesson = lessonOptions.find((option) => option.id === selectedLessonId);
  if (!selectedClassId || !lesson || attendanceRows.length === 0) return;

  isSaving = true;
  try {
    const items = attendanceRows.flatMap((row) =>
      row.status === "unmarked"
        ? []
        : [
            {
              student_id: row.studentId,
              section_subject_id: lesson.sectionSubjectId,
              scheduled_lesson_id: lesson.id,
              status: row.status,
              notes: row.remarks || undefined,
            },
          ],
    );
    if (items.length === 0) return;
    await api.batchMarkAttendance({
      section_id: selectedClassId,
      date: selectedDate,
      items,
    });
    toast.success(m.operations_ui_attendance_save_success());
    await loadRosterAndAttendance();
  } catch (error) {
    console.error("Failed to save attendance:", error);
    toast.error(
      extractErrorMessage(error) || m.operations_ui_attendance_save_error(),
    );
  } finally {
    isSaving = false;
  }
}

function handleExport() {
  const selectedClass = classes.find(
    (item) => String(item.id) === selectedClassId,
  );
  const className = selectedClass
    ? l(selectedClass.name_ar, selectedClass.name) || selectedClass.name
    : m.operations_ui_class_fallback();

  exportToCsv(
    m.operations_ui_attendance_export_filename({
      className,
      date: selectedDate,
    }),
    attendanceRows,
    [
      {
        header: m.operations_ui_student_number(),
        accessor: (row) => row.studentNumber,
      },
      {
        header: m.operations_ui_student_name(),
        accessor: (row) => row.fullName,
      },
      {
        header: m.operations_ui_status(),
        accessor: (row) =>
          row.status === "present"
            ? m.operations_ui_attendance_present()
            : row.status === "absent"
              ? m.operations_ui_attendance_absent()
              : row.status === "late"
                ? m.operations_ui_attendance_late()
                : row.status === "excused"
                  ? m.operations_ui_attendance_excused()
                  : m.operations_ui_attendance_unmarked(),
      },
      { header: m.operations_ui_notes(), accessor: (row) => row.remarks },
    ],
  );
}

const selectedClass = $derived(
  classes.find((item) => String(item.id) === selectedClassId),
);
const filteredRows = $derived(
  attendanceRows.filter((row) => {
    const query = searchQuery.trim().toLocaleLowerCase();
    const matchesSearch =
      !query ||
      row.fullName.toLocaleLowerCase().includes(query) ||
      row.studentNumber.toLocaleLowerCase().includes(query);
    return (
      matchesSearch && (statusFilter === "all" || row.status === statusFilter)
    );
  }),
);
const gridSource = $derived(
  filteredRows.map((row) => ({
    id: row.studentId,
    studentId: row.studentId,
    fullName: row.fullName,
    studentNumber: row.studentNumber,
    status: row.status === "unmarked" ? "" : row.status,
    remarks: row.remarks,
  })),
);
const presentCount = $derived(
  attendanceRows.filter((row) => row.status === "present").length,
);
const absentCount = $derived(
  attendanceRows.filter((row) => row.status === "absent").length,
);
</script>

<div class="flex h-full min-h-0 w-full flex-col gap-3 p-3 font-sans text-start sm:p-4">
  <div class="flex flex-wrap items-center gap-2">
    <Select
      type="single"
      value={selectedClassId}
      onValueChange={(value) => {
        selectedClassId = value;
        void loadRosterAndAttendance();
      }}
    >
      <SelectTrigger
        class="h-9 w-full text-xs sm:w-56"
        aria-label={m.operations_ui_attendance_select_class()}
      >
        <span class="truncate">
          {selectedClass
            ? l(selectedClass.name_ar, selectedClass.name) ||
              selectedClass.name ||
              selectedClass.id
            : m.operations_ui_attendance_select_class()}
        </span>
      </SelectTrigger>
      <SelectContent>
        {#each classes as classItem (classItem.id)}
          <SelectItem value={String(classItem.id)}>
            {l(classItem.name_ar, classItem.name) ||
              classItem.name ||
              classItem.id}
          </SelectItem>
        {/each}
      </SelectContent>
    </Select>

    <div class="relative w-full sm:w-40">
      <Calendar
        size={14}
        class="pointer-events-none absolute start-2.5 top-1/2 -translate-y-1/2 text-muted-foreground"
      />
      <Input
        type="date"
        bind:value={selectedDate}
        onchange={() => void loadRosterAndAttendance()}
        aria-label={m.operations_ui_attendance_date()}
        class="h-9 ps-8 text-xs"
      />
    </div>

      <Select
        type="single"
        value={selectedLessonId}
        onValueChange={selectLesson}
        disabled={isLoadingRoster || lessonOptions.length === 0}
      >
        <SelectTrigger
          class="h-9 w-full text-xs sm:w-72"
          aria-label={m.operations_ui_attendance_select_lesson()}
        >
          <span class="truncate">
            {lessonOptions.find((lesson) => lesson.id === selectedLessonId)
              ?.label || m.operations_ui_attendance_select_lesson()}
          </span>
        </SelectTrigger>
        <SelectContent>
          {#each lessonOptions as lesson (lesson.id)}
            <SelectItem value={lesson.id}>{lesson.label}</SelectItem>
          {/each}
        </SelectContent>
      </Select>

      <Button
        variant="default"
        size="sm"
        onclick={handleSaveAttendance}
        disabled={
          isSaving ||
          !selectedLessonId ||
          !attendanceRows.some((row) => row.status !== "unmarked")
        }
        class="h-9 gap-1.5 text-xs font-semibold sm:ms-auto"
      >
      <FloppyDisk size={14} />
      {isSaving
        ? m.operations_ui_saving()
        : m.operations_ui_attendance_save_record()}
    </Button>
  </div>

  {#if isLoadingClasses}
    <div class="flex flex-1 items-center justify-center">
      <span class="text-xs text-muted-foreground">
        {m.operations_ui_attendance_loading_students()}
      </span>
    </div>
  {:else if classLoadError}
    <div
      class="flex flex-1 flex-col items-center justify-center gap-2 text-center"
      role="alert"
    >
      <InfoCircle size={20} class="text-destructive" />
      <p class="text-xs text-muted-foreground">
        {m.operations_ui_attendance_class_list_load_error()}
      </p>
      <Button variant="outline" size="sm" onclick={() => void loadClasses()}>
        {m.common_retry()}
      </Button>
    </div>
  {:else if classes.length === 0}
    <div class="flex flex-1 items-center justify-center text-center">
      <p class="text-xs text-muted-foreground">
        {m.operations_ui_attendance_no_classes()}
      </p>
    </div>
  {:else if !isLoadingRoster && !rosterLoadError && lessonOptions.length === 0}
    <div class="flex flex-1 items-center justify-center text-center">
      <p class="text-xs text-muted-foreground">
        {m.operations_ui_attendance_no_lessons_today()}
      </p>
    </div>
  {:else}
    <Card class="flex min-h-0 flex-1 flex-col overflow-hidden border-border/60 shadow-none">
      <div class="flex flex-wrap items-center gap-2 border-b border-border/50 p-3">
        <div class="relative min-w-40 flex-1 sm:max-w-xs">
          <Search
            size={14}
            class="pointer-events-none absolute start-2.5 top-1/2 -translate-y-1/2 text-muted-foreground"
          />
          <Input
            type="search"
            placeholder={m.operations_ui_attendance_search_placeholder()}
            bind:value={searchQuery}
            class="h-9 ps-8 text-xs"
          />
        </div>

        <Select
          type="single"
          value={statusFilter}
          onValueChange={(value) => (statusFilter = value)}
        >
          <SelectTrigger
            class="h-9 w-full text-xs sm:w-36"
            aria-label={m.operations_ui_attendance_filter_status()}
          >
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
            <SelectItem value="present">
              {m.operations_ui_attendance_present()}
            </SelectItem>
            <SelectItem value="absent">
              {m.operations_ui_attendance_absent()}
            </SelectItem>
            <SelectItem value="late">
              {m.operations_ui_attendance_late()}
            </SelectItem>
            <SelectItem value="excused">
              {m.operations_ui_attendance_excused()}
            </SelectItem>
          </SelectContent>
        </Select>

        <span
          class="px-1 text-[11px] text-muted-foreground"
          aria-live="polite"
        >
          {filteredRows.length} / {attendanceRows.length}
        </span>

        <span class="hidden h-4 w-px bg-border sm:block"></span>

        <span
          class="inline-flex items-center gap-1 text-[11px] text-emerald-700 dark:text-emerald-400"
          title={m.operations_ui_attendance_present()}
        >
          <CheckCircle size={13} />
          {presentCount}
        </span>
        <span
          class="inline-flex items-center gap-1 text-[11px] text-red-700 dark:text-red-400"
          title={m.operations_ui_attendance_absent()}
        >
          <XMark size={13} />
          {absentCount}
        </span>
        <span
          class="inline-flex items-center gap-1 text-[11px] text-amber-700 dark:text-amber-400"
          title={m.operations_ui_attendance_late()}
        >
          <ClockRotateRight size={13} />
          {attendanceRows.filter((row) => row.status === "late").length}
        </span>
        <span
          class="inline-flex items-center gap-1 text-[11px] text-blue-700 dark:text-blue-400"
          title={m.operations_ui_attendance_excused()}
        >
          <InfoCircle size={13} />
          {attendanceRows.filter((row) => row.status === "excused").length}
        </span>
        <span class="text-[11px] text-muted-foreground">
          {m.operations_ui_attendance_unmarked()}: {attendanceRows.filter(
            (row) => row.status === "unmarked",
          ).length}
        </span>

        <div class="flex w-full flex-wrap items-center gap-1.5 sm:ms-auto sm:w-auto">
          <Button
            variant="ghost"
            size="sm"
            onclick={() => markAll("present")}
            class="h-8 px-2 text-[11px] text-muted-foreground"
          >
            {m.operations_ui_attendance_all_present()}
          </Button>
          <Button
            variant="ghost"
            size="sm"
            onclick={() => markAll("absent")}
            class="h-8 px-2 text-[11px] text-muted-foreground"
          >
            {m.operations_ui_attendance_all_absent()}
          </Button>
          <Button
            variant="outline"
            size="sm"
            onclick={handleExport}
            disabled={attendanceRows.length === 0}
            class="h-8 px-2.5 text-[11px]"
          >
            {m.operations_ui_export_csv()}
          </Button>
        </div>
      </div>

      <CardContent class="min-h-0 flex-1 p-0">
        {#if rosterLoadError}
          <div
            class="flex h-full flex-col items-center justify-center gap-2 text-center"
            role="alert"
          >
            <InfoCircle size={20} class="text-destructive" />
            <p class="text-xs text-muted-foreground">
              {m.operations_ui_attendance_roster_load_error()}
            </p>
            <Button
              variant="outline"
              size="sm"
              onclick={() => void loadRosterAndAttendance()}
            >
              {m.common_retry()}
            </Button>
          </div>
        {:else}
          {#if !selectedLessonId}
            <div class="flex h-full items-center justify-center p-6 text-center">
              <p class="text-xs text-muted-foreground">
                {m.operations_ui_attendance_select_lesson()}
              </p>
            </div>
          {:else}
            <OrbisGrid
              {columns}
              source={gridSource}
              loading={isLoadingRoster}
              height="100%"
              rowSize={44}
              rtl={getLocale() === "ar"}
              oncelledit={handleGridEdit}
              emptyTitle={m.operations_ui_attendance_no_students()}
            />
          {/if}
        {/if}
      </CardContent>
    </Card>
  {/if}
</div>
