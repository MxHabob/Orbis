<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { page } from "$app/state";
import { createQuery } from "@tanstack/svelte-query";
import { commands, extractErrorMessage } from "$lib/core/ipc";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import { Badge } from "@orbis/ui-svelte/badge";
import { Button } from "@orbis/ui-svelte/button";
import { MetricProgress } from "@orbis/ui-svelte";
import { 
  AlertCircle, 
  Users, 
  TrendingUp, 
  CheckCircle2, 
  FileText, 
  Bell, 
  CheckSquare, 
  MessageSquare,
  Sparkles,
  Share2,
  BookOpen,
  Calendar,
  ClipboardList,
  GraduationCap,
  ArrowRight,
  ArrowLeft,
  ChevronRight,
  HelpCircle,
  MessageSquareShare,
  Globe
} from "@lucide/svelte";
import { fade } from "svelte/transition";
import { academicContext } from "$lib/stores";
import { getLocale, localizeHref } from "$lib/paraglide/runtime";
import { toast } from "svelte-sonner";

import QuickActionButton from "$lib/shared/components/quick-action-button.svelte";
import ClassScheduleWidget from "$lib/shared/components/class-schedule-widget.svelte";
import ClassSubjectsWidget from "$lib/shared/components/class-subjects-widget.svelte";
import ClassRecommendationsWidget from "$lib/shared/components/class-recommendations-widget.svelte";
import AskOrbisButton from "$lib/shared/components/ask-orbis-button.svelte";
import RelatedActions, { type ActionGroup, type FooterAction } from "$lib/shared/components/related-actions.svelte";

const orgId = $derived(page.params.orgId ?? "");
const classId = $derived(page.params.classId ?? "");
const periodId = $derived(academicContext.effectivePeriodId);
const isArabic = $derived(getLocale() === "ar");

// ── TanStack Queries ──────────────────────────────────────────────────────────
const classQuery = createQuery(() => ({
  queryKey: ["class-detail-page", classId, periodId],
  queryFn: () => commands.academic.getClass(classId),
  enabled: !!classId,
}));

const classData = $derived(classQuery.data as any);
const classDisplayName = $derived.by(() => {
  if (!classData) return "";
  return isArabic ? classData.name_ar || classData.name : classData.name || classData.name_ar;
});

const classMetricsQuery = createQuery(() => ({
  queryKey: ["class-metrics", classId, periodId],
  queryFn: () => commands.assessment.classMetrics(periodId, classId).catch(() => []),
  enabled: !!classId && !!periodId,
  staleTime: 5 * 60 * 1000,
}));

const studentsQuery = createQuery(() => ({
  queryKey: ["class-students-overview", classId, periodId],
  queryFn: () => commands.academic.listEnrichedStudents(classId),
  enabled: !!classId,
}));

const subjectsQuery = createQuery(() => ({
  queryKey: ["class-subjects-overview", classId, periodId],
  queryFn: () => commands.academic.listClassSubjects(classId, periodId),
  enabled: !!classId && !!periodId,
}));

const studentsList = $derived((studentsQuery.data || []) as any[]);
const subjectsList = $derived((subjectsQuery.data || []) as any[]);

const isLoading = $derived(classMetricsQuery.isLoading);
const error = $derived(classMetricsQuery.error);

const attendanceRate = $derived.by(() => {
  const metrics = (classMetricsQuery.data || []) as any[];
  if (metrics.length === 0) return 0;
  const m = metrics[0];
  if (!m || !m.total_attendance_records || m.total_attendance_records === 0) return 0;
  return Math.round((m.present_records / m.total_attendance_records) * 100);
});

const presentCount = $derived.by(() => ((classMetricsQuery.data || [])[0] as any)?.present_records || 0);
const absentCount = $derived.by(() => ((classMetricsQuery.data || [])[0] as any)?.absent_records || 0);
const lateCount = $derived.by(() => ((classMetricsQuery.data || [])[0] as any)?.late_records || 0);
const excusedCount = $derived.by(() => ((classMetricsQuery.data || [])[0] as any)?.excused_records || 0);
const totalRecords = $derived(presentCount + absentCount + lateCount + excusedCount);

const presentPct = $derived(totalRecords > 0 ? (presentCount / totalRecords) * 100 : 0);
const absentPct = $derived(totalRecords > 0 ? (absentCount / totalRecords) * 100 : 0);
const latePct = $derived(totalRecords > 0 ? (lateCount / totalRecords) * 100 : 0);
const excusedPct = $derived(totalRecords > 0 ? (excusedCount / totalRecords) * 100 : 0);

function refetchAll() {
  classMetricsQuery.refetch();
  studentsQuery.refetch();
  subjectsQuery.refetch();
}

function handleShareLink() {
  navigator.clipboard.writeText(window.location.href);
  toast.success(isArabic ? "تم نسخ رابط الفصل إلى الحافظة" : "Class link copied to clipboard");
}

function handleNotifications() {
  toast.success(isArabic ? "تم تفعيل الإشعارات لهذا الفصل" : "Notifications enabled for this class");
}

// Related Actions Group
const relatedActionGroups = $derived<ActionGroup[]>([
  {
    id: "class-operations",
    title: isArabic ? "إجراءات الفصل والعمليات" : "Class operations",
    icon: Globe,
    defaultExpanded: true,
    subLinks: [
      { 
        label: isArabic ? "سجل وقائمة الطلاب" : "Students roster", 
        href: `/${orgId}/academics/classes/${classId}/students` 
      },
      { 
        label: isArabic ? "المواد والمناهج الدراسية" : "Curriculum & subjects", 
        href: `/${orgId}/academics/classes/${classId}/curriculum` 
      },
      { 
        label: isArabic ? "الجدول الدراسي العام" : "Full timetable", 
        href: `/${orgId}/academics/schedule` 
      },
    ],
  },
]);

const footerActions = $derived<FooterAction[]>([
  {
    label: isArabic ? "طلب مساعدة" : "Get help",
    icon: HelpCircle,
    onclick: () => toast.info(isArabic ? "جاري فتح دليل المساعدة الأكاديمي..." : "Opening help documentation..."),
  },
  {
    label: isArabic ? "تقديم ملاحظات" : "Give feedback",
    icon: MessageSquareShare,
    onclick: () => toast.info(isArabic ? "شكراً لملاحظاتك القيّمة!" : "Thank you for your feedback!"),
  },
]);
</script>

<div class="h-full w-full flex flex-col font-sans gap-8 pb-10 text-start">
  
  <!-- Quick Actions Row -->
  <div class="flex flex-wrap items-center gap-2.5 animate-fade-in">
    {#if classData}
      <AskOrbisButton
        query={m.academics_class_evaluation_ai_ask_orbis_query ? m.academics_class_evaluation_ai_ask_orbis_query({ name: classDisplayName }) : `Analyze performance for ${classDisplayName}`}
        label={m.academics_class_evaluation_ask_orbis_btn ? m.academics_class_evaluation_ask_orbis_btn({ name: classDisplayName }) : "اسأل Orbis"}
        variant="default"
        size="sm"
        class="h-9 px-3.5 text-xs font-bold gap-1.5 shadow-sm"
      />
    {/if}

    <QuickActionButton 
      label={isArabic ? "الإشعارات" : "Notifications"} 
      icon={Bell} 
      onclick={handleNotifications}
    />
    <QuickActionButton 
      label={isArabic ? "مشاركة الرابط" : "Share"} 
      icon={Share2} 
      onclick={handleShareLink}
    />
    <QuickActionButton 
      label={isArabic ? "تسجيل الحضور" : "Attendance"} 
      icon={Users} 
      onclick={() => toast.info(isArabic ? "فتح سجل الحضور..." : "Opening attendance register...")}
    />
    <QuickActionButton 
      label={isArabic ? "الواجبات" : "Assignments"} 
      icon={FileText} 
      onclick={() => toast.info(isArabic ? "إدارة الواجبات..." : "Manage assignments...")}
    />
    <QuickActionButton 
      label={isArabic ? "الملفات" : "Files"} 
      icon={CheckSquare} 
      onclick={() => toast.info(isArabic ? "مستندات الفصل..." : "Class documents...")}
    />
    <QuickActionButton 
      label={isArabic ? "الإعلانات" : "Announcements"} 
      icon={MessageSquare} 
      onclick={() => toast.info(isArabic ? "إرسال إعلان للطلاب وأولياء الأمور..." : "Post class announcement...")}
    />
  </div>

  {#if isLoading}
    <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
      <Skeleton class="h-32 rounded-xl" />
      <Skeleton class="h-32 rounded-xl" />
      <Skeleton class="h-32 rounded-xl" />
    </div>
  {:else if error}
    <div class="flex flex-col items-center justify-center py-16 text-destructive border border-destructive/20 bg-destructive/5 rounded-xl gap-2 font-sans">
      <AlertCircle size={24} />
      <h3 class="font-semibold text-xs">{m.academics_class_overview_load_failed ? m.academics_class_overview_load_failed() : "Failed to load class metrics"}</h3>
      <p class="text-[10px] text-muted-foreground">{extractErrorMessage(error)}</p>
      <Button variant="outline" onclick={refetchAll} class="text-[10px] mt-2 h-8">{m.common_retry ? m.common_retry() : "Retry"}</Button>
    </div>
  {:else}
    <div class="flex flex-col animate-fade-in text-start gap-8" in:fade={{ duration: 150 }}>
      
      <!-- Top Metrics Row using official @orbis/ui-svelte MetricProgress -->
      <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
        <MetricProgress
          card={true}
          title={isArabic ? "نسبة الحضور الأسبوعي" : "Student Weekly Attendance"}
          subtitle={attendanceRate > 0 ? (isArabic ? `${attendanceRate}% من الطلاب حاضرون هذا الأسبوع` : `${attendanceRate}% of students were present this week`) : (isArabic ? "لا توجد سجلات حضور مسجلة" : "No attendance data yet")}
          displayValue={`${attendanceRate}%`}
          icon={Users}
          segments={[
            { label: isArabic ? "حاضر" : "Present", value: presentPct, tone: "success" },
            { label: isArabic ? "غائب" : "Absent", value: absentPct, tone: "destructive" },
            { label: isArabic ? "متأخر" : "Late", value: latePct, tone: "warning" },
            { label: isArabic ? "معذور" : "Excused", value: excusedPct, tone: "info" },
          ]}
        />

        <MetricProgress
          card={true}
          title={isArabic ? "المستوى الأكاديمي العام" : "Student Performance"}
          subtitle={isArabic ? "75% من طلاب الفصل في مستوى جيد وممتاز" : "75% of the students are doing well"}
          displayValue="75%"
          icon={TrendingUp}
          segments={[
            { label: isArabic ? "ممتاز" : "Excellent", value: 45, tone: "info" },
            { label: isArabic ? "جيد" : "Good", value: 30, colorClass: "bg-blue-400" },
            { label: isArabic ? "يحتاج تحسين" : "Poor", value: 25, colorClass: "bg-blue-200" },
          ]}
        />

        <MetricProgress
          card={true}
          title={isArabic ? "إنجاز المنهاج الدراسي" : "Curriculum Progress"}
          subtitle={isArabic ? "تم استيفاء 90% من خطة المنهاج المقررة" : "90% of the Curriculum has been attained"}
          displayValue="90%"
          icon={CheckCircle2}
          segments={[
            { label: isArabic ? "تم تحصيله" : "Attained", value: 60, tone: "purple" },
            { label: isArabic ? "تم تدريسه" : "Covered", value: 30, colorClass: "bg-purple-400" },
            { label: isArabic ? "متبقي" : "Uncovered", value: 10, colorClass: "bg-purple-200" },
          ]}
        />
      </div>

      <!-- Main Operational Workspace (70/30 Split) -->
      <div class="grid grid-cols-1 lg:grid-cols-10 gap-6">
        
        <!-- Left Column: Primary Content (70%) -->
        <div class="lg:col-span-7 flex flex-col gap-6">
          <!-- Timetable Component (Compact & Scaled) -->
          <ClassScheduleWidget {classId} {periodId} />
        </div>

        <!-- Right Column: Secondary Content (30%) -->
        <div class="lg:col-span-3 flex flex-col gap-6">
          <!-- Class Subjects List -->
          <ClassSubjectsWidget {classId} {periodId} />

          <!-- AI Recommendations -->
          <ClassRecommendationsWidget {classId} {periodId} />
        </div>

      </div>

      <!-- Clean Related Support Section -->
      <div class="pt-2">
        <RelatedActions
          groups={relatedActionGroups}
          {footerActions}
        />
      </div>

    </div>
  {/if}
</div>
