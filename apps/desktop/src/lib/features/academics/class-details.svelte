<!--
  @component ClassDetails
  Feature component for the Class Dashboard view.
  Displays class quick actions, real attendance metrics, scheduled lessons,
  class subject offerings, and pedagogical insights.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Skeleton } from "@orbis/ui/skeleton";
import { Button } from "@orbis/ui/button";
import { MetricProgress } from "@orbis/ui";
import {
  AcademicCap,
  AppNotification,
  BookOpen,
  Calendar,
  ChartBar,
  ChatBubble,
  CheckCircle,
  CheckSquare,
  DocumentPage,
  GlobeAlt,
  InfoCircle,
  Share,
  Users,
} from "@orbis/icons";
import { fade } from "svelte/transition";
import { academicContext } from "$lib/stores";
import { toast } from "@orbis/ui";
import { extractErrorMessage } from "$lib/api";


import {
  createClassQuery,
  createClassStudentsQuery,
  createClassAttendanceQuery,
} from "$lib/features/academics";

import RelatedActions, {
  type ActionGroup,
  type FooterAction,
} from "$lib/shared/components/related-actions.svelte";
import ClassScheduleWidget from "$lib/shared/components/class-schedule-widget.svelte";
import ClassSubjectsWidget from "$lib/shared/components/class-subjects-widget.svelte";
import ClassRecommendationsWidget from "$lib/shared/components/class-recommendations-widget.svelte";
import AskOrbisButton from "$lib/shared/components/ask-orbis-button.svelte";
    import QuickActionButton from "$lib/shared/components/quick-action-button.svelte";

let { orgId, classId }: { orgId: string; classId: string } = $props();

const periodId = $derived(academicContext.effectivePeriodId);
const today = $derived(new Date().toISOString().slice(0, 10));

// ── Feature Queries ──────────────────────────────────────────────────────────
const classQuery = createClassQuery(() => classId);
const studentsQuery = createClassStudentsQuery(() => classId);
const attendanceQuery = createClassAttendanceQuery(
  () => classId,
  () => today,
  );

const classData = $derived(classQuery.data);
const students = $derived(studentsQuery.data ?? []);
const attendanceRecords = $derived(attendanceQuery.data ?? []);

const classDisplayName = $derived.by(() => {
  if (!classData) return "";
  return classData.name_ar || classData.name;
});

const isLoading = $derived(
  classQuery.isLoading || studentsQuery.isLoading || attendanceQuery.isLoading,
);
const error = $derived(
  classQuery.error || studentsQuery.error || attendanceQuery.error,
);

// Attendance Calculations based on real attendance records
const totalAttendance = $derived(attendanceRecords.length);
const presentCount = $derived(
  attendanceRecords.filter((r) => r.status === "present").length,
);
const absentCount = $derived(
  attendanceRecords.filter((r) => r.status === "absent").length,
);
const lateCount = $derived(
  attendanceRecords.filter((r) => r.status === "late").length,
);
const excusedCount = $derived(
  attendanceRecords.filter((r) => r.status === "excused").length,
);

const attendanceRate = $derived(
  totalAttendance > 0 ? Math.round((presentCount / totalAttendance) * 100) : 0,
);

const presentPct = $derived(
  totalAttendance > 0 ? (presentCount / totalAttendance) * 100 : 0,
);
const absentPct = $derived(
  totalAttendance > 0 ? (absentCount / totalAttendance) * 100 : 0,
);
const latePct = $derived(
  totalAttendance > 0 ? (lateCount / totalAttendance) * 100 : 0,
);
const excusedPct = $derived(
  totalAttendance > 0 ? (excusedCount / totalAttendance) * 100 : 0,
);

function refetchAll() {
  classQuery.refetch();
  studentsQuery.refetch();
  attendanceQuery.refetch();
}

function handleShareLink() {
  if (typeof window !== "undefined") {
    navigator.clipboard.writeText(window.location.href);
    toast.success(m.common_copied_to_clipboard());
  }
}

function handleNotifications() {
  toast.success(m.common_notifications_enabled());
}

const relatedActionGroups = $derived<ActionGroup[]>(classData ? [
  {
    id: "academic_operations",
    title: m.ui_copy_academic_operations_d82a1c76(),
    icon: AcademicCap,
    defaultExpanded: true,
    subLinks: [
      {
        id: "class_roster",
        label: m.ui_copy_class_roster_students_7c634c5e(),
        icon: Users,
        href: `/${orgId}/academics/classes/${classId}/students`,
        badge: students.length,
      },
      {
        id: "class_curriculum",
        label: m.ui_copy_curriculum_subjects_70041ea1(),
        icon: BookOpen,
        href: `/${orgId}/academics/classes/${classId}/curriculum`,
      },
      {
        id: "class_schedule",
        label: m.ui_copy_weekly_schedule_e30b8a34(),
        icon: Calendar,
        href: `/${orgId}/academics/schedule?classId=${classId}`,
      },
    ],
  },
  {
    id: "classroom_records",
    title: m.ui_copy_classroom_records_grading_8d9c9224(),
    icon: CheckSquare,
    defaultExpanded: true,
    subLinks: [
      {
        id: "daily_attendance",
        label: m.ui_copy_daily_attendance_cc39a674(),
        icon: CheckSquare,
        href: `/${orgId}/academics/attendance?classId=${classId}`,
      },
      {
        id: "grades_exams",
        label: m.ui_copy_grades_assessment_entry_53bbff49(),
        icon: DocumentPage,
        href: `/${orgId}/academics/exams?classId=${classId}`,
      },
    ],
  },
] : []);

</script>

<div class="h-full w-full flex flex-col font-sans gap-4 pb-6 text-start">
  <!-- Top Bar with Primary Query / Share Actions -->
  <div class="flex flex-wrap items-center justify-between gap-2 animate-fade-in">
    {#if classData}
      <AskOrbisButton
        query={m.academics_class_evaluation_ai_ask_orbis_query({ name: classDisplayName })}
        label={m.academics_class_evaluation_ask_orbis_btn({ name: classDisplayName })}
        variant="default"
        size="sm"
        class="h-9 px-3.5 text-xs font-bold gap-1.5 shadow-sm"
      />
    {/if}

    <div class="flex items-center gap-1.5 ms-auto">
      <Button
        variant="outline"
        size="sm"
        class="h-8 px-2.5 text-xs font-semibold gap-1.5"
        onclick={handleShareLink}
      >
        <Share size={12} />
        {m.common_action_share()}
      </Button>
    </div>
  </div>

  {#if isLoading}
    <div class="grid grid-cols-1 gap-3 md:grid-cols-3">
      <Skeleton class="h-24 rounded-lg" />
      <Skeleton class="h-24 rounded-lg" />
      <Skeleton class="h-24 rounded-lg" />
    </div>
  {:else if error}
    <div class="flex flex-col items-center justify-center py-16 text-destructive border border-destructive/20 bg-destructive/5 rounded-xl gap-2 font-sans">
      <InfoCircle size={24} />
      <h3 class="font-semibold text-xs">{m.academics_class_overview_load_failed()}</h3>
      <p class="text-[10px] text-muted-foreground">{extractErrorMessage(error)}</p>
      <Button variant="outline" onclick={refetchAll} class="text-[10px] mt-2 h-8">{m.common_retry()}</Button>
    </div>
  {:else}
    <div class="flex flex-col animate-fade-in text-start gap-4" in:fade={{ duration: 150 }}>
      <!-- Top Metrics Row using official @orbis/ui MetricProgress -->
      <div class="grid grid-cols-1 gap-3 md:grid-cols-3">
        <MetricProgress
          card={true}
          title={m.common_nav_attendance()}
          subtitle={totalAttendance > 0 
            ? `${attendanceRate}%`
            : m.common_attendance_empty_title()}
          displayValue={totalAttendance > 0 ? `${attendanceRate}%` : "—"}
          icon={Users}
          segments={totalAttendance > 0 ? [
            { label: m.common_attendance_status_present(), value: presentPct, tone: "success" },
            { label: m.common_attendance_status_absent(), value: absentPct, tone: "destructive" },
            { label: m.common_attendance_status_late(), value: latePct, tone: "warning" },
            { label: m.common_attendance_status_excused(), value: excusedPct, tone: "info" },
          ] : []}
        />

        <MetricProgress
          card={true}
          title={m.common_classes_enrolled_students()}
          subtitle={`${students.length} ${m.common_word_student()}`}
          displayValue={String(students.length)}
          icon={Users}
          segments={[]}
        />

        <MetricProgress
          card={true}
          title={m.academics_class_label_capacity()}
          subtitle={classData?.capacity ? `${classData.capacity}` : "—"}
          displayValue={classData?.capacity ? `${Math.round((students.length / classData.capacity) * 100)}%` : "—"}
          icon={CheckCircle}
          segments={[]}
        />
      </div>

      <!-- Main Operational Workspace (70/30 Split) -->
      <div class="grid grid-cols-1 gap-4 lg:grid-cols-10">
        <!-- Left Column: Timetable Grid (70%) -->
        <div class="lg:col-span-7 flex min-w-0 flex-col gap-4">
          <ClassScheduleWidget {classId} {periodId} />
        </div>

        <!-- Right Column: Subjects & AI Insights (30%) -->
        <div class="lg:col-span-3 flex min-w-0 flex-col gap-4">
          <ClassSubjectsWidget {classId} {periodId} />
          <ClassRecommendationsWidget {classId} {periodId} />
        </div>
      </div>

      <!-- Contextual Related Actions (Full Width Footer) -->
      <RelatedActions
        title={m.ui_copy_related_support_bf36e16f()}
        groups={relatedActionGroups}
      />
    </div>
  {/if}
</div>
