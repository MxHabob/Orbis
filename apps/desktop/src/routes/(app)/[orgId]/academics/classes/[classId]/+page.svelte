<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { page } from "$app/state";
import { createQuery } from "@tanstack/svelte-query";
import { api, extractErrorMessage } from "$lib/api";
import { Skeleton } from "@orbis/ui/skeleton";
import { Badge } from "@orbis/ui/badge";
import { Button } from "@orbis/ui/button";
import { MetricProgress } from "@orbis/ui";
import { AdjustmentsHorizontal, AppNotification, BookOpen, ChartBar, ChatBubble, CheckCircle, CheckSquare, DocumentPage, InfoCircle, Share, Sparkles, Users } from "@orbis/icons";
import { fade } from "svelte/transition";
import { academicContext } from "$lib/stores";
import { getLocale } from "$lib/paraglide/runtime";
import { toast } from "@orbis/ui";

import QuickActionButton from "$lib/shared/components/quick-action-button.svelte";
import ClassScheduleWidget from "$lib/shared/components/class-schedule-widget.svelte";
import ClassSubjectsWidget from "$lib/shared/components/class-subjects-widget.svelte";
import ClassRecommendationsWidget from "$lib/shared/components/class-recommendations-widget.svelte";
import AskOrbisButton from "$lib/shared/components/ask-orbis-button.svelte";

const classId = $derived(page.params.classId ?? "");
const periodId = $derived(academicContext.effectivePeriodId);
const isArabic = $derived(getLocale() === "ar");

// ── TanStack Queries ──────────────────────────────────────────────────────────
const classQuery = createQuery(() => ({
  queryKey: ["class-detail-page", classId, periodId],
  queryFn: () => api.getClass(classId),
  enabled: !!classId,
}));

const classData = $derived(classQuery.data as any);
const classDisplayName = $derived.by(() => {
  if (!classData) return "";
  return isArabic
    ? classData.name_ar || classData.name
    : classData.name || classData.name_ar;
});

const classMetricsQuery = createQuery(() => ({
  queryKey: ["class-metrics", classId, periodId],
  queryFn: () => Promise.resolve([]),
  enabled: !!classId && !!periodId,
  staleTime: 5 * 60 * 1000,
}));

const isLoading = $derived(classMetricsQuery.isLoading);
const error = $derived(classMetricsQuery.error);

const attendanceRate = $derived.by(() => {
  const metrics = (classMetricsQuery.data || []) as any[];
  if (metrics.length === 0) return 0;
  const m = metrics[0];
  if (!m || !m.total_attendance_records || m.total_attendance_records === 0)
    return 0;
  return Math.round((m.present_records / m.total_attendance_records) * 100);
});

const presentCount = $derived.by(
  () => ((classMetricsQuery.data || [])[0] as any)?.present_records || 0,
);
const absentCount = $derived.by(
  () => ((classMetricsQuery.data || [])[0] as any)?.absent_records || 0,
);
const lateCount = $derived.by(
  () => ((classMetricsQuery.data || [])[0] as any)?.late_records || 0,
);
const excusedCount = $derived.by(
  () => ((classMetricsQuery.data || [])[0] as any)?.excused_records || 0,
);
const totalRecords = $derived(
  presentCount + absentCount + lateCount + excusedCount,
);

const presentPct = $derived(
  totalRecords > 0 ? (presentCount / totalRecords) * 100 : 0,
);
const absentPct = $derived(
  totalRecords > 0 ? (absentCount / totalRecords) * 100 : 0,
);
const latePct = $derived(
  totalRecords > 0 ? (lateCount / totalRecords) * 100 : 0,
);
const excusedPct = $derived(
  totalRecords > 0 ? (excusedCount / totalRecords) * 100 : 0,
);

function refetchAll() {
  classMetricsQuery.refetch();
}

function handleShareLink() {
  navigator.clipboard.writeText(window.location.href);
  toast.success(
    isArabic
      ? "تم نسخ رابط الفصل إلى الحافظة"
      : "Class link copied to clipboard",
  );
}

function handleNotifications() {
  toast.success(
    isArabic
      ? "تم تفعيل الإشعارات لهذا الفصل"
      : "Notifications enabled for this class",
  );
}
</script>

<div class="h-full w-full flex flex-col font-sans gap-4 pb-6 text-start">
  <!-- Quick Actions Row -->
  <div class="flex flex-wrap items-center gap-2 animate-fade-in">
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
      icon={AppNotification} 
      onclick={handleNotifications}
    />
    <QuickActionButton 
      label={isArabic ? "مشاركة الرابط" : "Share"} 
      icon={Share} 
      onclick={handleShareLink}
    />
    <QuickActionButton 
      label={isArabic ? "تسجيل الحضور" : "Attendance"} 
      icon={Users} 
      onclick={() => toast.info(isArabic ? "فتح سجل الحضور..." : "Opening attendance register...")}
    />
    <QuickActionButton 
      label={isArabic ? "الواجبات" : "Assignments"} 
      icon={DocumentPage} 
      onclick={() => toast.info(isArabic ? "إدارة الواجبات..." : "Manage assignments...")}
    />
    <QuickActionButton 
      label={isArabic ? "الملفات" : "Files"} 
      icon={CheckSquare} 
      onclick={() => toast.info(isArabic ? "مستندات الفصل..." : "Class documents...")}
    />
    <QuickActionButton 
      label={isArabic ? "الإعلانات" : "Announcements"} 
      icon={ChatBubble} 
      onclick={() => toast.info(isArabic ? "إرسال إعلان للطلاب وأولياء الأمور..." : "Post class announcement...")}
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
      <h3 class="font-semibold text-xs">{m.academics_class_overview_load_failed ? m.academics_class_overview_load_failed() : "Failed to load class metrics"}</h3>
      <p class="text-[10px] text-muted-foreground">{extractErrorMessage(error)}</p>
      <Button variant="outline" onclick={refetchAll} class="text-[10px] mt-2 h-8">{m.common_retry ? m.common_retry() : "Retry"}</Button>
    </div>
  {:else}
    <div class="flex flex-col animate-fade-in text-start gap-4" in:fade={{ duration: 150 }}>
      
      <!-- Top Metrics Row using official @orbis/ui MetricProgress -->
      <div class="grid grid-cols-1 gap-3 md:grid-cols-3">
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
          subtitle={isArabic ? "لا توجد بيانات أداء" : "No performance data"}
          displayValue="0%"
          icon={ChartBar}
          segments={[]}
        />

        <MetricProgress
          card={true}
          title={isArabic ? "إنجاز المنهاج الدراسي" : "Curriculum Progress"}
          subtitle={isArabic ? "لا توجد بيانات للمنهاج" : "No curriculum data"}
          displayValue="0%"
          icon={CheckCircle}
          segments={[]}
        />

      </div>

      <!-- Main Operational Workspace (70/30 Split) -->
      <div class="grid grid-cols-1 gap-4 lg:grid-cols-10">
        
        <!-- Left Column: Primary Content (70%) -->
        <div class="lg:col-span-7 flex min-w-0 flex-col gap-4">
          <!-- Timetable Component (Compact & Scaled) -->
          <ClassScheduleWidget {classId} {periodId} />
        </div>

        <!-- Right Column: Secondary Content (30%) -->
        <div class="lg:col-span-3 flex min-w-0 flex-col gap-4">
          <!-- Class Subjects List -->
          <ClassSubjectsWidget {classId} {periodId} />

          <!-- AI Recommendations -->
          <ClassRecommendationsWidget {classId} {periodId} />
        </div>

      </div>

    </div>
  {/if}
</div>
