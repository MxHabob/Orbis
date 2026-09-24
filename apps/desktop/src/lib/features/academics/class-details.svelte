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
  AppNotification,
  ChartBar,
  ChatBubble,
  CheckCircle,
  CheckSquare,
  DocumentPage,
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

import QuickActionButton from "$lib/shared/components/quick-action-button.svelte";
import ClassScheduleWidget from "$lib/shared/components/class-schedule-widget.svelte";
import ClassSubjectsWidget from "$lib/shared/components/class-subjects-widget.svelte";
import ClassRecommendationsWidget from "$lib/shared/components/class-recommendations-widget.svelte";
import AskOrbisButton from "$lib/shared/components/ask-orbis-button.svelte";

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
</script>

<div class="h-full w-full flex flex-col font-sans gap-4 pb-6 text-start">
  <!-- Quick Actions Row -->
  <div class="flex flex-wrap items-center gap-2 animate-fade-in">
    {#if classData}
      <AskOrbisButton
        query={m.academics_class_evaluation_ai_ask_orbis_query({ name: classDisplayName })}
        label={m.academics_class_evaluation_ask_orbis_btn({ name: classDisplayName })}
        variant="default"
        size="sm"
        class="h-9 px-3.5 text-xs font-bold gap-1.5 shadow-sm"
      />
    {/if}

    <QuickActionButton 
      label={m.common_nav_notifications()} 
      icon={AppNotification} 
      onclick={handleNotifications}
    />
    <QuickActionButton 
      label={m.common_action_share()} 
      icon={Share} 
      onclick={handleShareLink}
    />
    <QuickActionButton 
      label={m.common_nav_attendance()} 
      icon={Users} 
      onclick={() => toast.info(m.common_nav_attendance())}
    />
    <QuickActionButton 
      label={m.common_assessment_option_homework()} 
      icon={DocumentPage} 
      onclick={() => toast.info(m.common_assessment_option_homework())}
    />
    <QuickActionButton 
      label={m.common_nav_documents()} 
      icon={CheckSquare} 
      onclick={() => toast.info(m.common_nav_documents())}
    />
    <QuickActionButton 
      label={m.common_nav_announcements()} 
      icon={ChatBubble} 
      onclick={() => toast.info(m.common_nav_announcements())}
    />
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

    </div>
  {/if}
</div>
