<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import { formatDate } from "$lib/core/i18n";
import { Button } from "@orbis/ui/button";
import { Skeleton } from "@orbis/ui/skeleton";
import {
  OrbisGrid,
  type ActionItem,
  type OrbisColumnDef,
  type SelectionSnapshot,
} from "@orbis/svelte-grid";
import { ArrowLeft } from "@orbis/icons";
import { toast } from "@orbis/ui";
import { getLocale, localizeHref } from "$lib/paraglide/runtime";
import { fade } from "svelte/transition";
import { Card } from "@orbis/ui/card";

import ExamStatusBadge from "$lib/features/exams/components/status-badge.svelte";
import ExamSessionConfigDialog from "$lib/features/exams/components/exam-session-config-dialog.svelte";
import ExamProctorAssignDialog from "$lib/features/exams/components/exam-proctor-assign-dialog.svelte";
import SubjectAddSheet from "$lib/features/exams/components/subject-add-sheet.svelte";
import SessionCreateSheet from "$lib/features/exams/components/session-create-sheet.svelte";
import PrintDialog from "$lib/shared/components/print-dialog.svelte";
import { api } from "$lib/api";
import { aiContext, confirmStore } from "$lib/stores";

let { orgId, sessionId }: { orgId: string; sessionId: string } = $props();

// State
let session = $state<any>(null);
let subjects = $state<any[]>([]);
let sectionSubjects = $state<any[]>([]);
let schedules = $state<any[]>([]);
let seatingNumbers = $state<any[]>([]);
let students = $state<any[]>([]);
let enrollments = $state<any[]>([]);
let teachers = $state<any[]>([]);
let printDialogOpen = $state(false);
let selectedSeatIds = $state<Set<string>>(new Set());

function handleSeatingSelection(selection: SelectionSnapshot) {
  selectedSeatIds = new Set(selection.ids);
}

const selectedSeatingContexts = $derived.by(() => {
  return seatingNumbers
    .filter((sn) => selectedSeatIds.has(String(sn.id)))
    .map((sn) => ({
      exam: {
        seatingNumber: sn.seating_number,
        hallName: sn.room_name ?? (sn as any).hall_name ?? "",
        deskNumber: (sn as any).desk_number ?? "",
        sessionName: session?.name ?? "",
      },
      student: {
        name: getStudentName(sn.student_id),
        studentNumber: sn.seating_number,
        className: getStudentSectionName(sn.student_id),
        level: getStudentLevel(sn.student_id),
      },
    }));
});

const selectedSeatingLabels = $derived.by(() => {
  return seatingNumbers
    .filter((sn) => selectedSeatIds.has(String(sn.id)))
    .map((sn) => `${getStudentName(sn.student_id)} (${sn.seating_number})`);
});

let halls = $state<any[]>([]);
let proctors = $state<any[]>([]);
let rooms = $state<any[]>([]);
let isLoading = $state(true);
let activeTab = $state("subjects");
let isSubjectSheetOpen = $state(false);
let isEditSheetOpen = $state(false);
let isProctorAssignDialogOpen = $state(false);

let activeJob = $state<any>(null);
let solveJobs = $state<any[]>([]);
let isSolving = $state(false);
let timeElapsed = $state(0);
let pollingInterval = $state<any>(null);
let elapsedTimer = $state<any>(null);

let classes = $state<any[]>([]);
let schoolSubjects = $state<any[]>([]);
let previewDraft = $state(false);

let isConfigModalOpen = $state(false);
let activeConfigTab = $state("schedule");

const draftSchedulesInGrid = $derived.by(() => {
  if (!previewDraft || !activeJob || !activeJob.best_solution) return [];
  try {
    const solution = JSON.parse(activeJob.best_solution);
    const assignments = solution.assignments || [];
    return assignments.map((a: any) => {
      const slotParts = a.slot_id ? a.slot_id.split("_") : [];
      const exam_date = slotParts[0] || "";
      const period = slotParts[1] || "";
      return {
        id: a.id,
        exam_date,
        period,
        exam_subject_id: a.exam_subject_id,
        applicable_class_ids: a.class_id,
        start_time: period === "morning" ? "08:00" : "13:00",
        end_time: (() => {
          const exam_subject = subjects.find(
            (s: any) => s.id === a.exam_subject_id,
          );
          const duration = exam_subject ? exam_subject.duration_minutes : 90;
          const start_time = period === "morning" ? "08:00" : "13:00";
          return addMinutes(start_time, duration);
        })(),
        isDraft: true,
      };
    });
  } catch (e) {
    console.error("Error parsing draft solution for exams:", e);
    return [];
  }
});

const activeSchedules = $derived(
  previewDraft ? draftSchedulesInGrid : schedules,
);

const sessionDates = $derived(
  session ? generateDatesRange(session.start_date, session.end_date) : [],
);

function getSubjectName(subjectId: string) {
  const sub = schoolSubjects.find(
    (s: any) => String(s.id) === String(subjectId),
  );
  return sub ? sub.name_ar || sub.name : subjectId;
}

function getClassName(classId: string) {
  const cls = classes.find((c) => String(c.id) === String(classId));
  return cls ? cls.name_ar || cls.name : classId;
}

function getSectionSubject(sectionSubjectId: string) {
  return sectionSubjects.find(
    (sectionSubject) => String(sectionSubject.id) === String(sectionSubjectId),
  );
}

function getScheduleSubjectName(schedule: any) {
  const sectionSubject = getSectionSubject(schedule.section_subject_id);
  return sectionSubject
    ? getSubjectName(sectionSubject.subject_id)
    : m.common_word_subject();
}

function getScheduleSectionName(schedule: any) {
  const sectionSubject = getSectionSubject(schedule.section_subject_id);
  return sectionSubject
    ? getClassName(sectionSubject.section_id)
    : m.common_table_class();
}

function getStudentName(studentId: string) {
  const student = students.find(
    (item: any) => String(item.student?.id ?? item.id) === String(studentId),
  );
  return (
    student?.person?.full_name_ar ||
    student?.person?.full_name ||
    student?.student?.student_number ||
    student?.student_number ||
    ""
  );
}

function getStudentEnrollment(studentId: string) {
  return enrollments.find(
    (item: any) => String(item.student_id) === String(studentId),
  );
}

function getStudentSectionName(studentId: string) {
  const enrollment = getStudentEnrollment(studentId);
  const section = classes.find(
    (item: any) => String(item.id) === String(enrollment?.section_id),
  );
  return section?.name_ar || section?.name || "";
}

function getStudentLevel(studentId: string) {
  const enrollment = getStudentEnrollment(studentId);
  const section = classes.find(
    (item: any) => String(item.id) === String(enrollment?.section_id),
  );
  return section?.level || "";
}

function formatDateWithDay(dateStr: string): string {
  return (
    formatDate(dateStr, "date", {
      weekday: "long",
      day: "numeric",
      month: "short",
    }) || dateStr
  );
}

function getExamsInCell(date: string, period: string) {
  return activeSchedules.filter(
    (s: any) =>
      String(s.exam_date) === String(date) &&
      (Number(String(s.start_time).split(":")[0]) < 12
        ? "morning"
        : "afternoon") === period,
  );
}

function addMinutes(timeStr: string, minutes: number): string {
  const parts = timeStr.split(":");
  if (parts.length < 2) return timeStr;
  const h = parseInt(parts[0], 10) || 0;
  const m = parseInt(parts[1], 10) || 0;
  const total = h * 60 + m + minutes;
  const rh = Math.floor(total / 60) % 24;
  const rm = total % 60;
  return `${String(rh).padStart(2, "0")}:${String(rm).padStart(2, "0")}`;
}

const tabs = $derived([
  {
    id: "subjects",
    label: m.common_exams_detail_subjects(),
    count: subjects.length,
  },
  {
    id: "schedule",
    label: m.common_exams_detail_schedule(),
    count: activeSchedules.length,
  },
  {
    id: "seating",
    label: m.common_exams_detail_seating(),
    count: seatingNumbers.length,
  },
  {
    id: "halls",
    label: m.common_exams_stat_halls(),
    count: halls.length,
  },
  {
    id: "proctors",
    label: m.common_exams_stat_proctors(),
    count: proctors.length,
  },
]);

const subjectGridSource = $derived(
  sectionSubjects.map((sectionSubject: any, index: number) => ({
    ...sectionSubject,
    __idx: index + 1,
    subject_name: getSubjectName(sectionSubject.subject_id),
    section_name: getClassName(sectionSubject.section_id),
  })),
);

const subjectColumns: OrbisColumnDef[] = [
  {
    id: "__idx",
    prop: "__idx",
    header: "#",
    name: "#",
    width: 48,
    size: 48,
    readonly: true,
    align: "center",
  },
  {
    id: "subject_name",
    prop: "subject_name",
    header: m.common_word_subject(),
    name: m.common_word_subject(),
    flex: 1,
    readonly: true,
    cellClass: "font-medium",
  },
  {
    id: "section_name",
    prop: "section_name",
    header: m.common_table_class(),
    name: m.common_table_class(),
    width: 180,
    size: 180,
    readonly: true,
  },
];

const seatActions: ActionItem[] = [
  { id: "delete", label: m.common_action_delete(), variant: "destructive" },
];

const seatingGridSource = $derived(
  seatingNumbers.map((seat: any, index: number) => {
    const student = students.find(
      (item: any) =>
        String(item.student?.id ?? item.id) === String(seat.student_id),
    );
    const enrollment = enrollments.find(
      (item: any) => String(item.student_id) === String(seat.student_id),
    );
    const section = classes.find(
      (item: any) => String(item.id) === String(enrollment?.section_id),
    );
    const room = rooms.find(
      (item: any) => String(item.id) === String(seat.room_id),
    );
    return {
      ...seat,
      __idx: index + 1,
      student_name_display:
        student?.person?.full_name_ar ||
        student?.person?.full_name ||
        student?.student?.student_number ||
        student?.student_number ||
        "—",
      room_display: room?.name_ar || room?.name || "—",
      class_display: section?.name_ar || section?.name || "—",
      level_display: section?.level || "—",
    };
  }),
);

const seatingColumns: OrbisColumnDef[] = [
  {
    id: "__selected",
    prop: "__selected",
    header: "",
    name: "",
    width: 40,
    size: 40,
    readonly: true,
    align: "center",
  },
  {
    id: "__idx",
    prop: "__idx",
    header: "#",
    name: "#",
    width: 48,
    size: 48,
    readonly: true,
    align: "center",
  },
  {
    id: "seating_number",
    prop: "seating_number",
    header: m.academics_exams_seat_number(),
    name: m.academics_exams_seat_number(),
    width: 130,
    size: 130,
    type: "number",
    readonly: true,
    cellClass: "font-mono font-semibold text-primary",
  },
  {
    id: "room_display",
    prop: "room_display",
    header: m.common_lesson_label_room(),
    name: m.common_lesson_label_room(),
    width: 130,
    size: 130,
    readonly: true,
  },
  {
    id: "student_name_display",
    prop: "student_name_display",
    header: m.common_word_student(),
    name: m.common_word_student(),
    flex: 1,
    readonly: true,
  },
  {
    id: "class_display",
    prop: "class_display",
    header: m.common_table_class(),
    name: m.common_table_class(),
    width: 130,
    size: 130,
    readonly: true,
  },
  {
    id: "level_display",
    prop: "level_display",
    header: m.academics_exams_level(),
    name: m.academics_exams_level(),
    width: 110,
    size: 110,
    readonly: true,
  },
  {
    id: "actions",
    prop: "actions",
    header: m.common_action_delete(),
    name: m.common_action_delete(),
    width: 76,
    size: 76,
    type: "action",
    readonly: true,
    align: "center",
    meta: { items: seatActions },
  },
];

const proctorActions: ActionItem[] = [
  {
    id: "delete",
    label: m.operations_ui_cancel_proctor_assignment(),
    variant: "destructive",
  },
];

const proctorGridSource = $derived(
  proctors.map((proctor: any) => ({
    ...proctor,
    teacher_display:
      teachers.find((teacher: any) => teacher.id === proctor.teacher_id)
        ?.full_name_ar ||
      teachers.find((teacher: any) => teacher.id === proctor.teacher_id)
        ?.full_name ||
      proctor.teacher_id ||
      "—",
    hall_display:
      rooms.find((room: any) => room.id === proctor.room_id)?.name_ar ||
      rooms.find((room: any) => room.id === proctor.room_id)?.name ||
      proctor.room_id ||
      "—",
    role_display: proctor.is_chief
      ? m.operations_ui_proctor_chief()
      : m.academics_exams_proctor(),
  })),
);

const proctorColumns: OrbisColumnDef[] = [
  {
    id: "teacher_display",
    prop: "teacher_display",
    header: m.academics_exams_proctor(),
    name: m.academics_exams_proctor(),
    flex: 1,
    readonly: true,
  },
  {
    id: "hall_display",
    prop: "hall_display",
    header: m.academics_exams_committee(),
    name: m.academics_exams_committee(),
    width: 150,
    size: 150,
    readonly: true,
  },
  {
    id: "role_display",
    prop: "role_display",
    header: m.common_user_form_role(),
    name: m.common_user_form_role(),
    width: 140,
    size: 140,
    readonly: true,
  },
  {
    id: "actions",
    prop: "actions",
    header: m.common_action_delete(),
    name: m.common_action_delete(),
    width: 76,
    size: 76,
    type: "action",
    readonly: true,
    align: "center",
    meta: { items: proctorActions },
  },
];

$effect(() => {
  aiContext.setContext({
    domain: "exam",
    pageName: "Exam Session Details",
    recordId: sessionId,
    metadata: {
      sessionName: session?.name || "",
      status: session?.status || "",
      subjectsCount: subjects.length,
      schedulesCount: activeSchedules.length,
    },
  });
  return () => aiContext.clearContext();
});

async function loadSession() {
  if (!orgId || !sessionId) return;
  isLoading = true;
  try {
    session = await api.getExamSession(sessionId);

    const [
      loadedSchedules,
      loadedSeatingNumbers,
      loadedRooms,
      [classList, subjectList],
      loadedTeachers,
    ] = await Promise.all([
      api.listExamSchedules(sessionId),
      api.listSeating(sessionId),
      api.listRooms(),
      Promise.all([api.listSections(), api.listSubjects()]),
      loadActiveTeachers(),
    ]);
    const loadedEnrollments = await loadPeriodEnrollments(session.period_id);

    const activeSections = classList.filter(
      (section: any) => section.is_active,
    );
    const loadedSectionSubjects = (
      await Promise.all(
        activeSections.map((section: any) =>
          api.listSectionSubjects(section.id, session.period_id),
        ),
      )
    )
      .flat()
      .filter((sectionSubject: any) => sectionSubject.is_active);
    const loadedStudents = await loadPeriodStudents(session.period_id);

    let loadedProctors: any[] = [];
    if (loadedSchedules.length > 0) {
      const proctorArrays = await Promise.all(
        loadedSchedules.map((schedule: any) => api.listProctors(schedule.id)),
      );
      loadedProctors = proctorArrays.flat();
    }

    subjects = loadedSectionSubjects;
    sectionSubjects = loadedSectionSubjects;
    schedules = loadedSchedules;
    seatingNumbers = loadedSeatingNumbers;
    students = loadedStudents;
    enrollments = loadedEnrollments;
    halls = loadedRooms
      .filter((room: any) => room.is_exam_hall)
      .map((room: any) => ({
        room_id: room.id,
        capacity: room.capacity,
        allocated_count: loadedSeatingNumbers.filter(
          (seat: any) => String(seat.room_id) === String(room.id),
        ).length,
      }));
    proctors = loadedProctors;
    rooms = loadedRooms;
    teachers = loadedTeachers;
    classes = classList;
    schoolSubjects = subjectList;

    await loadSolveJobs();
  } catch (e) {
    toast.error("Failed to load session details");
    console.error(e);
  } finally {
    isLoading = false;
  }
}

async function loadPeriodStudents(periodId: string) {
  const pageSize = 500;
  const loaded: any[] = [];
  const studentIds = new Set<string>();

  for (let page = 1; ; page += 1) {
    const result = await api.listStudents({
      period_id: periodId,
      status: "active",
      page,
      page_size: pageSize,
    });
    const newStudents = result.items.filter((item: any) => {
      const id = String(item.student?.id ?? item.id);
      if (studentIds.has(id)) return false;
      studentIds.add(id);
      return true;
    });
    loaded.push(...newStudents);

    if (result.items.length < pageSize) break;
    if (newStudents.length === 0) {
      throw new Error("Student pagination returned a repeated page");
    }
  }

  return loaded;
}

async function loadPeriodEnrollments(periodId: string) {
  const pageSize = 500;
  const loaded: any[] = [];
  const enrollmentIds = new Set<string>();

  for (let page = 1; ; page += 1) {
    const result = await api.listEnrollments({
      period_id: periodId,
      status: "active",
      page,
      page_size: pageSize,
    });
    const newEnrollments = result.filter((item: any) => {
      if (enrollmentIds.has(item.id)) return false;
      enrollmentIds.add(item.id);
      return true;
    });
    loaded.push(...newEnrollments);

    if (result.length < pageSize) break;
    if (newEnrollments.length === 0) {
      throw new Error("Enrollment pagination returned a repeated page");
    }
  }

  return loaded;
}

async function loadActiveTeachers() {
  const pageSize = 500;
  const loaded: any[] = [];

  for (let page = 1; ; page += 1) {
    const result = await api.listEmployees({
      employee_type: "teacher",
      status: "active",
      page,
      page_size: pageSize,
    });
    loaded.push(...result.items);
    if (result.items.length < pageSize) break;
  }

  return loaded;
}

onMount(() => {
  if (orgId && sessionId) {
    loadSession();
  }
});

async function handleGenerateSeating() {
  try {
    const enrolledStudentIds = new Set(
      enrollments
        .filter((enrollment: any) => enrollment.status === "active")
        .map((enrollment: any) => String(enrollment.student_id)),
    );
    const existingStudentIds = new Set(
      seatingNumbers.map((seat: any) => String(seat.student_id)),
    );
    const studentIds = students
      .map((student: any) => String(student.student?.id ?? student.id))
      .filter(
        (id: string) =>
          enrolledStudentIds.has(id) && !existingStudentIds.has(id),
      );
    if (!studentIds.length) {
      toast.error(
        students.length === 0
          ? m.operations_ui_seating_no_students()
          : m.operations_ui_seating_all_assigned(),
      );
      return;
    }
    const existingSequenceNumbers = seatingNumbers
      .map((seat: any) =>
        Number(String(seat.seating_number).replace(/^SEAT-/, "")),
      )
      .filter(Number.isFinite);
    const nextNumber = Math.max(1000, ...existingSequenceNumbers) + 1;
    const roomIds = rooms
      .filter((room: any) => room.is_exam_hall)
      .map((room: any) => room.id);

    const promise = api.generateSeating({
      exam_session_id: sessionId,
      student_ids: studentIds,
      room_ids: roomIds,
      prefix: "SEAT-",
      start_number: nextNumber,
    });

    toast.promise(promise, {
      loading: m.operations_ui_seating_generating(),
      success: m.operations_ui_seating_generate_success(),
      error: (err) =>
        (err as any)?.message || m.operations_ui_seating_generate_error(),
    });

    await promise;
    await loadSession();
  } catch (err) {
    console.error("Failed to generate seating", err);
  }
}

async function handleGenerateSecrets() {
  try {
    if (!schedules.length) {
      toast.error(m.operations_ui_secrets_no_schedules());
      return;
    }
    if (!students.length) {
      toast.error(m.operations_ui_secrets_no_students());
      return;
    }

    const pendingSecrets = await Promise.all(
      schedules.map(async (schedule: any) => {
        const sectionSubject = getSectionSubject(schedule.section_subject_id);
        if (!sectionSubject) {
          throw new Error(
            `Scheduled exam ${schedule.id} references an unavailable section subject`,
          );
        }

        const eligibleStudentIds = new Set(
          enrollments
            .filter(
              (enrollment: any) =>
                enrollment.status === "active" &&
                String(enrollment.section_id) ===
                  String(sectionSubject.section_id),
            )
            .map((enrollment: any) => String(enrollment.student_id)),
        );
        const scheduleStudents = students
          .map((student: any) => String(student.student?.id ?? student.id))
          .filter((studentId: string) => eligibleStudentIds.has(studentId));
        const existingSecrets = await api.listSecrets(schedule.id);
        const existingStudentIds = new Set(
          existingSecrets.map((secret: any) => String(secret.student_id)),
        );
        const missingStudentIds = scheduleStudents.filter(
          (studentId: string) => !existingStudentIds.has(studentId),
        );

        return { schedule, studentIds: missingStudentIds };
      }),
    );

    const assignments = pendingSecrets.filter(
      (pending) => pending.studentIds.length > 0,
    );
    if (assignments.length === 0) {
      toast.info(m.operations_ui_secrets_all_assigned());
      return;
    }

    const promise = Promise.all(
      assignments.map(({ schedule, studentIds }) =>
        api.generateSecrets({
          exam_schedule_id: schedule.id,
          student_ids: studentIds,
        }),
      ),
    );
    toast.promise(promise, {
      loading: m.operations_ui_secrets_generating(),
      success: m.operations_ui_secrets_generate_success(),
      error: (err) =>
        (err as any)?.message || m.operations_ui_secrets_generate_error(),
    });

    await promise;
    await loadSession();
  } catch (err) {
    console.error("Failed to generate secrets", err);
  }
}

function generateDatesRange(startStr: string, endStr: string): string[] {
  const dates: string[] = [];
  try {
    const start = new Date(startStr);
    const end = new Date(endStr);
    for (let d = new Date(start); d <= end; d.setDate(d.getDate() + 1)) {
      dates.push(d.toISOString().split("T")[0]);
    }
  } catch (e) {
    console.error(e);
  }
  return dates;
}

async function loadSolveJobs() {
  solveJobs = [];
  activeJob = null;
  isSolving = false;
}

async function handleAutoSchedule() {
  toast.error(m.operations_ui_auto_scheduling_desktop_unavailable());
}

function stopPolling() {
  if (pollingInterval) {
    clearInterval(pollingInterval);
    pollingInterval = null;
  }
  if (elapsedTimer) {
    clearInterval(elapsedTimer);
    elapsedTimer = null;
  }
}

async function handleCancelSolve() {
  toast.error(m.operations_ui_cancel_scheduling_desktop_unavailable());
}

async function handleApplySolve() {
  toast.error(m.operations_ui_apply_scheduling_desktop_unavailable());
}

async function handleDeleteSchedule(scheduleId: string) {
  const confirmed = await confirmStore.prompt({
    title: m.operations_ui_delete_question_title(),
    description: m.academics_exams_sessions_sessionId_are_you_sure_you_want(),
    confirmText: m.common_action_delete(),
    variant: "destructive",
  });
  if (!confirmed) return;
  const promise = api.deleteExamSchedule(scheduleId);
  toast.promise(promise, {
    loading: m.academics_exams_sessions_sessionId_deleting_scheduled_exam(),
    success: m.academics_exams_sessions_sessionId_deleted_exam_successfully(),
    error: (err: any) =>
      (err as any)?.message || m.operations_ui_exam_delete_error(),
  });
  try {
    await promise;
    await loadSession();
  } catch (err) {
    console.error(err);
  }
}

async function handleToggleScheduleStatus(sched: any) {
  const newStatus = sched.status === "cancelled" ? "scheduled" : "cancelled";
  const label =
    newStatus === "cancelled"
      ? m.academics_exams_sessions_sessionId_deactivating_exam()
      : m.academics_exams_sessions_sessionId_activating_exam();
  const labelSuccess =
    newStatus === "cancelled"
      ? m.academics_exams_sessions_sessionId_deactivated_successfully()
      : m.academics_exams_sessions_sessionId_activated_successfully();

  const promise = Promise.resolve();
  toast.promise(promise, {
    loading: label,
    success: labelSuccess,
    error: (err: any) =>
      (err as any)?.message || m.operations_ui_exam_status_update_error(),
  });
  try {
    await promise;
    await loadSession();
  } catch (err) {
    console.error(err);
  }
}

async function handleAllocateHalls() {
  toast.error(m.operations_ui_hall_assignment_desktop_unavailable());
}

async function handleActivate() {
  if (!session) return;
  const targetStatus = session.status === "active" ? "completed" : "active";
  const promise = api.updateExamSession(sessionId, { status: targetStatus });

  toast.promise(promise, {
    loading: m.operations_ui_exam_session_status_updating(),
    success: m.operations_ui_exam_session_status_updated(),
    error: (err) =>
      (err as any)?.message ||
      m.operations_ui_exam_session_status_update_error(),
  });

  try {
    await promise;
    await loadSession();
  } catch (err) {
    console.error(err);
  }
}

async function handleDeleteSeating(seatingId: string) {
  const confirmed = await confirmStore.prompt({
    title: m.operations_ui_seating_delete_title(),
    description: m.operations_ui_seating_delete_confirm(),
    confirmText: m.common_action_delete(),
    variant: "destructive",
  });
  if (!confirmed) return;
  const promise = api.deleteSeating(seatingId);
  toast.promise(promise, {
    loading: m.operations_ui_seating_deleting(),
    success: m.operations_ui_seating_delete_success(),
    error: (err: any) =>
      (err as any)?.message || m.operations_ui_seating_delete_error(),
  });
  try {
    await promise;
    await loadSession();
  } catch (err) {
    console.error(err);
  }
}

async function handleDeleteProctor(proctorId: string) {
  const confirmed = await confirmStore.prompt({
    title: m.operations_ui_proctor_assignment_cancel_title(),
    description: m.operations_ui_proctor_assignment_cancel_confirm(),
    confirmText: m.operations_ui_proctor_assignment_cancel(),
    variant: "destructive",
  });
  if (!confirmed) return;
  const promise = api.deleteProctor(proctorId);
  toast.promise(promise, {
    loading: m.operations_ui_proctor_assignment_cancelling(),
    success: m.operations_ui_proctor_assignment_cancel_success(),
    error: (err: any) =>
      (err as any)?.message ||
      m.operations_ui_proctor_assignment_cancel_error(),
  });
  try {
    await promise;
    await loadSession();
  } catch (err) {
    console.error(err);
  }
}

async function handleToggleHall(room: any) {
  const isActivating = !room.is_exam_hall;
  const promise = api.updateRoom(room.id, { is_exam_hall: isActivating });

  toast.promise(promise, {
    loading: isActivating
      ? m.operations_ui_exam_room_activating({
          room: room.name_ar || room.name,
        })
      : m.operations_ui_exam_room_deactivating({
          room: room.name_ar || room.name,
        }),
    success: isActivating
      ? m.operations_ui_exam_room_activate_success()
      : m.operations_ui_exam_room_deactivate_success(),
    error: (err) =>
      (err as any)?.message ||
      (isActivating
        ? m.operations_ui_exam_room_activate_error()
        : m.operations_ui_exam_room_deactivate_error()),
  });

  try {
    await promise;
    await loadSession();
  } catch (err) {
    console.error("Failed to update exam hall availability", err);
  }
}
</script>

<div class="h-full w-full overflow-y-auto scrollbar-hide p-6 flex flex-col gap-6 select-none text-start">
	{#if isLoading}
		<Skeleton class="h-24 w-full rounded-xl" />
		<Skeleton class="h-12 w-full rounded-lg" />
		<Skeleton class="h-64 w-full rounded-xl" />
	{:else}
		<!-- Back + Header -->
		<div class="flex items-start gap-2" in:fade={{ duration: 150 }}>
			<Button
				href={localizeHref(`/${orgId}/academics/exams/sessions`)}
				variant="ghost"
				size="icon-sm"
				class="shrink-0 text-muted-foreground"
				title={m.common_action_back()}
				aria-label={m.common_action_back()}
			>
				<ArrowLeft size={16} class="rtl:rotate-180" />
			</Button>

			<div class="flex min-w-0 flex-1 flex-col gap-4 border-b border-border/60 pb-5 sm:flex-row sm:items-center sm:justify-between">
				<div class="flex flex-col gap-1.5">
					<div class="flex items-center gap-2">
						<span class="text-base font-semibold text-foreground">
							{session?.name_ar || session?.name || `Session ${sessionId}`}
						</span>
						<ExamStatusBadge status={session?.status || "draft"} />
					</div>
					{#if session}
						<div class="flex flex-wrap items-center gap-2 text-xs text-muted-foreground">
							<span>
								{formatDate(session.start_date, "date") || session.start_date}
								–
								{formatDate(session.end_date, "date") || session.end_date}
							</span>
						</div>
					{/if}
				</div>
				<div class="flex flex-wrap items-center gap-2">
					<Button
						variant="outline"
						class="h-9 px-3 text-xs"
						onclick={() => isEditSheetOpen = true}
					>
						{m.common_action_edit()}
					</Button>
					<Button
						variant="lime"
						class="h-9 px-3 text-xs font-semibold"
						onclick={handleActivate}
					>
						{m.common_exams_detail_activate()}
					</Button>
				</div>
			</div>
		</div>

		<!-- Tabs -->
		<div class="flex items-center gap-1 overflow-x-auto border-b border-border/60 pb-0 scrollbar-hide" in:fade={{ delay: 50, duration: 200 }}>
			{#each tabs as tab (tab.id)}
				<button
					class="flex shrink-0 cursor-pointer items-center gap-2 border-b-2 px-3 py-3 text-xs font-medium transition-colors
						{activeTab === tab.id
							? 'border-primary text-primary font-semibold'
							: 'border-transparent text-muted-foreground hover:text-foreground hover:border-border'}"
					onclick={() => activeTab = tab.id}
				>
					<span>{tab.label}</span>
					{#if tab.count > 0}
						<span class="rounded-md bg-muted px-1.5 py-0.5 font-mono text-[10px] text-muted-foreground">{tab.count}</span>
					{/if}
				</button>
			{/each}
		</div>

		<!-- Tab Content -->
		<div class="flex-1" in:fade={{ delay: 100, duration: 200 }}>

			<!-- ═══ Subjects Tab ═══ -->
			{#if activeTab === "subjects"}
				<div class="flex flex-col gap-4">
					{#if subjects.length === 0}
						<div class="flex flex-col items-center justify-center py-12 gap-3 text-center">
							<p class="text-xs text-muted-foreground">{m.academics_exams_no_subjects()}</p>
							<Button variant="outline" class="h-7 px-3 text-[10px] cursor-pointer" onclick={() => isSubjectSheetOpen = true}>
								{m.common_exams_subjects_add()}
							</Button>
						</div>
					{:else}
						<Card class="overflow-hidden rounded-xl border-border/60 bg-card shadow-xs">
							<div class="h-80 w-full">
								<OrbisGrid
									columns={subjectColumns}
									source={subjectGridSource}
									clientQuery
									readonly
									rtl={getLocale() === "ar"}
									emptyTitle={m.academics_exams_no_subjects()}
								/>
							</div>
						</Card>
						<Button variant="outline" class="w-fit h-7 px-3 text-[10px] cursor-pointer" onclick={() => isSubjectSheetOpen = true}>
							{m.common_exams_subjects_add()}
						</Button>
					{/if}
				</div>

			<!-- ═══ Schedule Tab ═══ -->
			{:else if activeTab === "schedule"}
				<div class="flex flex-col gap-4">
					{#if activeSchedules.length === 0}
						<div class="flex flex-col items-center justify-center py-12 gap-3 text-center">
							<p class="text-xs text-muted-foreground">{m.academics_exams_no_exams_scheduled()}</p>
							<Button variant="outline" class="h-7 px-3 text-[10px] cursor-pointer" onclick={handleAutoSchedule}>
								{m.common_exams_actions_auto_schedule()}
							</Button>
						</div>
					{:else}
						<Card class="timetable-container flex flex-col overflow-hidden rounded-xl border-border/60 shadow-xs select-none">
							<div 
								class="grid gap-px text-xs w-full bg-border-line"
								style="grid-template-columns: 180px 1fr 1fr;"
							>
								<!-- Header Row -->
								<div class="timetable-header p-4 font-bold text-center flex items-center justify-center border-b uppercase text-[10px] tracking-wider">
									{m.academics_exams_sessions_sessionId_day_date()}
								</div>
								<div class="timetable-header p-4 font-bold text-center flex items-center justify-center border-b uppercase text-[10px] tracking-wider">
									{m.academics_exams_sessions_sessionId_morning_period_08_00()}
								</div>
								<div class="timetable-header p-4 font-bold text-center flex items-center justify-center border-b uppercase text-[10px] tracking-wider">
									{m.academics_exams_sessions_sessionId_afternoon_period_13_00()}
								</div>

								<!-- Rows (Dates) -->
								{#each sessionDates as date (date)}
									<!-- Date Cell -->
									<div class="timetable-period-info p-3 font-semibold text-center flex flex-col justify-center items-center gap-1 min-h-22.5 border-b bg-muted/10 border-e">
										<span class="text-[11px] tracking-wide font-extrabold text-foreground text-center">
											{formatDateWithDay(date)}
										</span>
									</div>

									<!-- Morning Cell -->
									<div class="timetable-cell p-3 text-start h-full min-h-22.5 relative flex flex-col justify-start items-stretch gap-2 border-b bg-card border-e">
										{#each getExamsInCell(date, "morning") as sched (sched.id)}
											<div
												class="relative overflow-hidden w-full p-2.5 flex flex-row items-stretch justify-between gap-2.5 select-none transition-all duration-150 rounded-lg border
													{sched.isDraft ? 'border-dashed border-primary/50 bg-primary/2' : 'border-border/60 hover:border-primary/20 bg-card hover:bg-muted/10'} 
													{sched.status === 'cancelled' ? 'opacity-40 bg-muted/20 border-dashed border-muted-foreground/30 line-through' : ''}"
											>
												<div class="flex-1 flex flex-col justify-center items-start text-start gap-1 min-w-0">
													<div class="flex items-center gap-1.5 w-full">
														<span class="font-bold text-[11px] leading-tight text-foreground line-clamp-1">
															{getScheduleSubjectName(sched)}
														</span>
														{#if sched.isDraft}
															<span class="text-[8px] px-1 py-0.5 rounded bg-primary/10 text-primary font-bold">{m.common_status_draft()}</span>
														{/if}
													</div>
													
													<div class="text-[9px] text-muted-foreground mt-0.5 flex flex-wrap items-center gap-x-2 gap-y-0.5">
														<span class="font-bold text-primary">{getScheduleSectionName(sched)}</span>
														<span class="font-mono">{sched.start_time} - {sched.end_time}</span>
													</div>
												</div>

												{#if !sched.isDraft}
													<div class="flex items-center gap-1 shrink-0 self-center">
														<button
															type="button" 
															class="rounded px-2 py-1 text-[10px] text-muted-foreground transition-colors hover:text-foreground"
															onclick={() => handleToggleScheduleStatus(sched)}
															title={sched.status === "cancelled" ? m.operations_ui_reactivate_exam() : m.operations_ui_deactivate_exam()}
														>
															{sched.status === "cancelled" ? m.operations_ui_reactivate_exam() : m.operations_ui_deactivate_exam()}
														</button>
														<button
															type="button" 
															class="rounded px-2 py-1 text-[10px] text-muted-foreground transition-colors hover:text-destructive"
															onclick={() => handleDeleteSchedule(sched.id)}
															title={m.operations_ui_delete_exam_permanently()}
														>
															{m.common_action_delete()}
														</button>
													</div>
												{/if}
											</div>
										{/each}
									</div>

									<!-- Afternoon Cell -->
									<div class="timetable-cell p-3 text-start h-full min-h-22.5 relative flex flex-col justify-start items-stretch gap-2 border-b bg-card">
										{#each getExamsInCell(date, "afternoon") as sched (sched.id)}
											<div
												class="relative overflow-hidden w-full p-2.5 flex flex-row items-stretch justify-between gap-2.5 select-none transition-all duration-150 rounded-lg border
													{sched.isDraft ? 'border-dashed border-primary/50 bg-primary/2' : 'border-border/60 hover:border-primary/20 bg-card hover:bg-muted/10'} 
													{sched.status === 'cancelled' ? 'opacity-40 bg-muted/20 border-dashed border-muted-foreground/30 line-through' : ''}"
											>
												<div class="flex-1 flex flex-col justify-center items-start text-start gap-1 min-w-0">
													<div class="flex items-center gap-1.5 w-full">
														<span class="font-bold text-[11px] leading-tight text-foreground line-clamp-1">
															{getScheduleSubjectName(sched)}
														</span>
														{#if sched.isDraft}
															<span class="text-[8px] px-1 py-0.5 rounded bg-primary/10 text-primary font-bold">{m.common_status_draft()}</span>
														{/if}
													</div>
													
													<div class="text-[9px] text-muted-foreground mt-0.5 flex flex-wrap items-center gap-x-2 gap-y-0.5">
														<span class="font-bold text-primary">{getScheduleSectionName(sched)}</span>
														<span class="font-mono">{sched.start_time} - {sched.end_time}</span>
													</div>
												</div>

												{#if !sched.isDraft}
													<div class="flex items-center gap-1 shrink-0 self-center">
														<button
															type="button" 
															class="rounded px-2 py-1 text-[10px] text-muted-foreground transition-colors hover:text-foreground"
															onclick={() => handleToggleScheduleStatus(sched)}
															title={sched.status === "cancelled" ? m.operations_ui_reactivate_exam() : m.operations_ui_deactivate_exam()}
														>
															{sched.status === "cancelled" ? m.operations_ui_reactivate_exam() : m.operations_ui_deactivate_exam()}
														</button>
														<button
															type="button" 
															class="rounded px-2 py-1 text-[10px] text-muted-foreground transition-colors hover:text-destructive"
															onclick={() => handleDeleteSchedule(sched.id)}
															title={m.operations_ui_delete_exam_permanently()}
														>
															{m.common_action_delete()}
														</button>
													</div>
												{/if}
											</div>
										{/each}
									</div>
								{/each}
							</div>
						</Card>
					{/if}
				</div>

			<!-- ═══ Seating Tab ═══ -->
			{:else if activeTab === "seating"}
				<div class="flex flex-col gap-4">
					{#if seatingNumbers.length === 0}
						<div class="flex flex-col items-center justify-center py-12 gap-3 text-center">
							<p class="text-xs text-muted-foreground">{m.academics_exams_no_seating_numbers()}</p>
							<Button variant="default" class="h-7 px-3 text-[10px] cursor-pointer" onclick={handleGenerateSeating}>
								{m.common_exams_actions_generate_seating()}
							</Button>
						</div>
					{:else}
						<div class="flex items-center justify-between gap-3 px-1 select-none">
							<span class="text-xs text-muted-foreground">{m.academics_exams_issued()}{seatingNumbers.length}{m.academics_exams_seating_numbers_suffix()}</span>
							{#if selectedSeatIds.size > 0}
								<Button 
									variant="outline" 
									class="h-7 px-3 text-[10px] cursor-pointer border-primary/30 text-primary"
									onclick={() => printDialogOpen = true}
								>
									{m.common_print_templates()} ({selectedSeatIds.size})
								</Button>
							{/if}
						</div>
						<Card class="overflow-hidden rounded-xl border-border/60 bg-card shadow-xs">
							<div class="h-96 w-full">
								<OrbisGrid
									columns={seatingColumns}
									source={seatingGridSource}
									clientQuery
									readonly
									rowIdKey="id"
									rtl={getLocale() === "ar"}
									onselection={handleSeatingSelection}
									onaction={(event) => {
										if (event.actionId === "delete") {
											handleDeleteSeating(String(event.row.id));
										}
									}}
									emptyTitle={m.academics_exams_no_seating_numbers()}
								/>
							</div>
						</Card>
					{/if}
				</div>

			<!-- ═══ Halls Tab ═══ -->
			{:else if activeTab === "halls"}
				<div class="flex flex-col gap-4">
					<div class="flex items-center justify-between text-xs text-muted-foreground select-none px-1">
						<span>{m.academics_exams_activated()}{halls.length}{m.academics_exams_committees_out_of()}{rooms.length}{m.academics_exams_available_classrooms_suffix()}</span>
						<Button variant="outline" class="h-6 px-2 text-[9px] cursor-pointer" onclick={() => { activeConfigTab = "halls"; isConfigModalOpen = true; }}>
							{m.operations_ui_distribute_students_automatically()}
						</Button>
					</div>

					{#if rooms.length === 0}
						<div class="flex flex-col items-center justify-center py-12 gap-3 text-center">
							<p class="text-xs text-muted-foreground">{m.academics_exams_no_classrooms()}</p>
						</div>
					{:else}
						<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3">
							{#each rooms as room (room.id)}
								{@const activeHall = halls.find((h) => h.room_id === room.id)}
								{@const isAct = !!activeHall}
								<Card class="flex flex-col gap-3 rounded-xl border-border/60 bg-card p-4 shadow-xs transition-colors {isAct ? 'border-primary/40 ring-1 ring-primary/10' : 'hover:border-primary/30'}">
									<div class="flex items-center justify-between">
										<div class="flex flex-col">
											<span class="text-sm font-semibold text-foreground">{room.name_ar || room.name}</span>
											{#if room.floor != null}
												<span class="mt-1 text-xs text-muted-foreground">{m.academics_exams_floor_connector()}{room.floor}</span>
											{/if}
										</div>
										<span class="text-[9px] font-mono px-1.5 py-0.5 rounded {isAct ? 'bg-emerald-500/10 text-emerald-600 border border-emerald-500/20' : 'bg-muted text-muted-foreground border border-border/40'}">
											{isAct ? m.operations_ui_active_committee() : m.operations_ui_inactive()}
										</span>
									</div>

									{#if isAct}
										{@const utilPct = activeHall.capacity > 0 ? Math.round((activeHall.allocated_count / activeHall.capacity) * 100) : 0}
										<div class="flex flex-col gap-1.5">
											<div class="flex justify-between text-[9px] text-muted-foreground font-mono">
												<span>{m.academics_exams_allocated_capacity()}{activeHall.capacity}</span>
												<span>{m.academics_exams_allocated()}{activeHall.allocated_count}{m.academics_exams_student_suffix()}</span>
											</div>
											<div class="h-1 w-full rounded-full bg-muted/40 overflow-hidden">
												<div
													class="h-full rounded-full transition-all duration-300 {utilPct > 90 ? 'bg-red-500' : utilPct > 70 ? 'bg-amber-500' : 'bg-emerald-500'}"
													style="width: {utilPct}%"
												></div>
											</div>
											<Button
												variant="ghost"
												class="h-8 self-end px-2 text-xs text-muted-foreground hover:text-destructive"
												onclick={() => handleToggleHall(room)}
											>
												{m.operations_ui_deactivate_exam_committee()}
											</Button>
										</div>
									{:else}
										<div class="flex items-center justify-between mt-1">
											<span class="text-xs text-muted-foreground">{m.academics_exams_default_capacity()}{room.capacity || 30}</span>
											<Button
												variant="ghost"
												class="h-8 px-2 text-xs font-medium text-primary hover:bg-primary/5"
												onclick={() => handleToggleHall(room)}
											>
												{m.operations_ui_activate_as_committee()}
											</Button>
										</div>
									{/if}
								</Card>
							{/each}
						</div>
					{/if}
				</div>

			<!-- ═══ Proctors Tab ═══ -->
			{:else if activeTab === "proctors"}
				<div class="flex flex-col gap-4">
					{#if proctors.length === 0}
						<div class="flex flex-col items-center justify-center py-12 gap-3 text-center">
							<p class="text-xs text-muted-foreground">{m.academics_exams_no_proctors_assigned()}</p>
							<Button
								variant="outline"
								class="h-7 px-3 text-[10px] cursor-pointer"
								onclick={() => isProctorAssignDialogOpen = true}
							>
								{m.common_exams_actions_assign_proctors()}
							</Button>
						</div>
					{:else}
						<Card class="overflow-hidden rounded-xl border-border/60 bg-card shadow-xs">
							<div class="h-80 w-full">
								<OrbisGrid
									columns={proctorColumns}
									source={proctorGridSource}
									clientQuery
									readonly
									rowIdKey="id"
									rtl={getLocale() === "ar"}
									onaction={(event) => {
										if (event.actionId === "delete") {
											handleDeleteProctor(String(event.row.id));
										}
									}}
									emptyTitle={m.academics_exams_no_proctors_assigned()}
								/>
							</div>
						</Card>
					{/if}
				</div>
			{/if}
		</div>

		{#if activeJob}
			<Card class="mb-4 flex flex-col justify-between gap-4 rounded-xl border-border/60 bg-card p-4 shadow-xs sm:flex-row sm:items-center">
					<div class="flex items-center gap-3">
						<div>
							<p class="text-sm font-medium text-foreground">
							{#if activeJob.status === 'running'}
								{m.operations_ui_auto_schedule_optimizing()}
							{:else if activeJob.status === 'completed'}
								{m.operations_ui_auto_schedule_success()}
							{:else if activeJob.status === 'failed'}
								{m.operations_ui_auto_schedule_failure()}
							{:else}
								{m.operations_ui_auto_schedule_cancelled()}
							{/if}
						</p>
						<p class="text-[10px] text-muted-foreground mt-0.5 font-mono">
							{#if activeJob.status === 'running'}
								{m.operations_ui_scheduling_elapsed_score({ elapsed: timeElapsed, score: activeJob.score || "—" })}
							{:else if activeJob.status === 'completed'}
								{m.operations_ui_scheduling_final_score({ score: activeJob.score || "—" })}
							{/if}
						</p>
					</div>
				</div>
				<div class="flex items-center gap-2">
					{#if activeJob.status === 'running'}
						<Button size="sm" variant="destructive" class="h-8 text-[10px] px-3 font-bold cursor-pointer" onclick={handleCancelSolve}>
							{m.operations_ui_cancel_operation()}
						</Button>
					{:else if activeJob.status === 'completed'}
						<Button size="sm" variant={previewDraft ? "secondary" : "outline"} class="h-8 text-[10px] px-3 font-bold cursor-pointer" onclick={() => previewDraft = !previewDraft}>
							{previewDraft ? m.operations_ui_cancel_preview() : m.operations_ui_preview_proposed_schedule()}
						</Button>
						<Button size="sm" variant="default" class="h-8 text-[10px] px-3 font-bold cursor-pointer" onclick={handleApplySolve}>
							{m.operations_ui_apply_proposed_schedule()}
						</Button>
					{/if}
				</div>
		</Card>
		{/if}

		<!-- Quick Actions Bar -->
		<div class="flex flex-wrap items-center gap-2" in:fade={{ delay: 150, duration: 200 }}>
			<Button variant="outline" class="h-8 px-3 text-xs" onclick={handleGenerateSeating}>
				{m.operations_ui_seating_numbers()}
			</Button>
			<Button
				variant="outline"
				class="h-8 px-3 text-xs"
				disabled={schedules.length === 0}
				title={schedules.length === 0 ? m.operations_ui_secrets_no_schedules() : undefined}
				onclick={handleGenerateSecrets}
			>
				{m.operations_ui_secret_numbers()}
			</Button>
			<Button variant="outline" class="h-8 px-3 text-xs" onclick={() => { activeConfigTab = "schedule"; isConfigModalOpen = true; }}>
				{m.operations_ui_automatic_scheduling()}
			</Button>
			<Button variant="outline" class="h-8 px-3 text-xs" onclick={() => { activeConfigTab = "halls"; isConfigModalOpen = true; }}>
				{m.operations_ui_committee_distribution()}
			</Button>
		</div>
	{/if}
</div>

<ExamSessionConfigDialog
	bind:open={isConfigModalOpen}
	bind:activeTab={activeConfigTab}
	onAutoSchedule={handleAutoSchedule}
	onAllocateHalls={handleAllocateHalls}
/>

<ExamProctorAssignDialog
	bind:open={isProctorAssignDialogOpen}
	{orgId}
	{schedules}
	onSuccess={loadSession}
/>

<SubjectAddSheet
	bind:open={isSubjectSheetOpen}
	{orgId}
	{sessionId}
	periodId={session?.period_id || ""}
	examStartDate={session?.start_date || ""}
	examEndDate={session?.end_date || ""}
	onSuccess={loadSession}
/>

<SessionCreateSheet
	bind:open={isEditSheetOpen}
	{orgId}
	{session}
	onSuccess={loadSession}
/>

<PrintDialog
	bind:open={printDialogOpen}
	entityType="exam"
	documentType="EXAM_CARD"
	entityIds={Array.from(selectedSeatIds)}
	entityLabels={selectedSeatingLabels}
	contexts={selectedSeatingContexts}
/>
