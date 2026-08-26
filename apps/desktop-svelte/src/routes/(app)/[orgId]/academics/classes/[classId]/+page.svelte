<!--
@component ClassOverviewPage
Redesigned diagnostic overview page for a single class.
Matches the Sentry Issue Details page layout EXACTLY, built using the design language and components of the Finance page:
- Sentry header with issue title, subtitle, event/user metrics (top right)
- Actions strip (Resolved, Unresolve, Priority, Assignee)
- Chart section (SVG attendance graph on the left, breakdown bars on the right)
- Sentry parameters bar & Highlights grid (Class attributes)
- Sentry Stack Trace container embedding the interactive Grade Matrix (DataTable)
- Sentry right sidebar containing Orbis AI Advisor (Seer Autofix), External Links, and Activity/Comments feed
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { page } from "$app/state";
import { createQuery } from "@tanstack/svelte-query";
import { commands, extractErrorMessage } from "$lib/core/ipc";
import { Card } from "@orbis/ui-svelte";
import { Button } from "@orbis/ui-svelte/button";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import { Badge } from "@orbis/ui-svelte/badge";
import { toast } from "svelte-sonner";
import AttendanceTrendsCard from "$lib/features/metrics/cards/attendance-trends-card.svelte";
import GradeDistributionCard from "$lib/features/metrics/cards/grade-distribution-card.svelte";
import {
  AlertTriangle,
  Activity,
  Sparkles,
  Landmark,
  FileText,
  AlertCircle,
  BookOpen,
} from "@lucide/svelte";
import { fade } from "svelte/transition";
import { localizeHref, getLocale } from "$lib/paraglide/runtime";

import { academicContext } from "$lib/stores";

const orgId = $derived(page.params.orgId ?? "");
const classId = $derived(page.params.classId ?? "");
const periodId = $derived(academicContext.effectivePeriodId);
const rtl = $derived(getLocale() === "ar");

// ── TanStack Queries ──────────────────────────────────────────────────────────
const studentsQuery = createQuery(() => ({
  queryKey: ["class-students-enriched", classId],
  queryFn: () => commands.academic.listEnrichedStudents(classId).catch(() => []),
  enabled: !!classId,
  staleTime: 5 * 60 * 1000,
}));

const classSubjectsQuery = createQuery(() => ({
  queryKey: ["class-subjects", classId, periodId],
  queryFn: () => commands.academic.listClassSubjects(classId, periodId).catch(() => []),
  enabled: !!classId && !!periodId,
  staleTime: 5 * 60 * 1000,
}));

const classMetricsQuery = createQuery(() => ({
  queryKey: ["class-metrics", classId, periodId],
  queryFn: () => commands.assessment.classMetrics(periodId, classId).catch(() => []),
  enabled: !!classId && !!periodId,
  staleTime: 5 * 60 * 1000,
}));

const filteredStudents = $derived((studentsQuery.data || []) as any[]);
const studentsCount = $derived(filteredStudents.length);
const subjectsCount = $derived(
  new Set(((classSubjectsQuery.data || []) as any[]).map((cs: any) => String(cs.subject_id))).size
);

const attendanceRate = $derived.by(() => {
  const metrics = (classMetricsQuery.data || []) as any[];
  if (metrics.length === 0) return 100;
  const m = metrics[0];
  if (!m || !m.total_attendance_records || m.total_attendance_records === 0) return 100;
  return Math.round((m.present_records / m.total_attendance_records) * 100);
});

const classHealthIndex = $derived(Math.min(100, Math.round(attendanceRate)));

const isLoading = $derived(
  studentsQuery.isLoading ||
  classSubjectsQuery.isLoading ||
  classMetricsQuery.isLoading
);

const error = $derived(
  studentsQuery.error ||
  classSubjectsQuery.error ||
  classMetricsQuery.error
);

function refetchAll() {
  studentsQuery.refetch();
  classSubjectsQuery.refetch();
  classMetricsQuery.refetch();
}

// Dynamic diagnostic alerts using real query states
let resolvedAlerts = $state<Record<string, boolean>>({});

const diagnosticIssues = $derived.by(() => {
  const issues = [];

  if (studentsCount === 0) {
    issues.push({
      id: "no-students",
      type: "attendance",
      title: m.academics_class_overview_no_students_title(),
      desc: m.academics_class_overview_no_students_desc(),
      priority: "critical",
      actionText: m.academics_class_overview_view_students(),
    });
  }

  if (attendanceRate < 90) {
    issues.push({
      id: "low-attendance",
      type: "attendance",
      title: m.academics_class_overview_low_attendance_title(),
      desc: m.academics_class_overview_low_attendance_desc({
        rate: String(Math.round(attendanceRate)),
      }),
      priority: "critical",
      actionText: m.academics_class_overview_contact_parents(),
    });
  }

  return issues.filter((issue) => !resolvedAlerts[issue.id]);
});

function handleResolveIssue(id: string) {
  resolvedAlerts = { ...resolvedAlerts, [id]: true };
  toast.success(m.academics_class_overview_alert_resolved_toast());
}

function handleTriggerAction(issue: any) {
  toast.info(
    `${m.academics_class_overview_action_triggered()} ${issue.actionText}`,
  );
}
</script>

<div class="h-full w-full overflow-y-auto scrollbar-hide flex flex-col gap-6 font-sans">
  {#if isLoading}
    <Skeleton class="h-20 w-full rounded-xl animate-pulse" />
    <Skeleton class="h-10 w-full rounded-xl animate-pulse" />
    <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
      <Skeleton class="h-96 lg:col-span-2 rounded-xl" />
      <Skeleton class="h-96 rounded-xl" />
    </div>
  {:else if error}
    <div class="flex flex-col items-center justify-center py-16 text-destructive border border-destructive/20 bg-destructive/5 rounded-xl gap-2 font-sans">
      <AlertCircle size={24} />
      <h3 class="font-semibold text-xs">{m.academics_class_overview_load_failed()}</h3>
      <p class="text-[10px] text-muted-foreground">{extractErrorMessage(error)}</p>
      <Button variant="outline" onclick={refetchAll} class="text-[10px] mt-2 h-8">{m.common_retry()}</Button>
    </div>
  {:else}
    <div class="flex flex-col gap-5 animate-fade-in text-start" in:fade={{ duration: 150 }}>

      <!-- ── 3. Chart and Tag Breakdown Section (Split Layout) ── -->
      <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
        <!-- Left: SVG Timeline graph of Weekly Attendance (70%) -->
        <div class="lg:col-span-2">
          <AttendanceTrendsCard orgId={orgId} selectedClass={classId} />
        </div>

        <!-- Right: Grade Distribution Card (30%) -->
        <div>
          <GradeDistributionCard orgId={orgId} selectedClass={classId} />
        </div>
      </div>

      <!-- ── 5. Main Double Column Layout: Left Stack Trace / Right Sidebar ── -->
      <div class="grid grid-cols-1 lg:grid-cols-3 gap-6 font-sans">
        
        <!-- LEFT: Sentry Stack Trace Wrapper embedding Class Alerts (70%) -->
        <div class="lg:col-span-2 flex flex-col gap-6">

          <!-- Alert / Issues list card (like Sentry Events details) -->
          <Card class="p-5 bg-card border border-border/60 flex flex-col justify-between shadow-sm text-start card-tactile transition-all duration-200">
            <div class="flex items-center justify-between w-full select-none text-[9px] font-bold text-muted-foreground/80 mb-3.5">
              <div class="flex items-center gap-1.5 font-sans font-extrabold uppercase tracking-wider text-[8px] text-muted-foreground/75">
                <span class="h-1.5 w-1.5 rounded-full bg-destructive animate-pulse"></span>
                {m.academics_class_overview_active_alerts()}
              </div>
            </div>

            <div class="flex flex-col gap-3">
              {#each diagnosticIssues as issue (issue.id)}
                <div class="flex items-center justify-between p-3 rounded-lg border border-border/40 bg-card/40 hover:bg-card/70 transition-colors gap-4">
                  <div class="flex items-start gap-2.5 min-w-0">
                    <div class="h-5 w-5 rounded bg-destructive/10 border border-destructive/20 text-destructive flex items-center justify-center shrink-0 mt-0.5 select-none">
                      <AlertTriangle size={11} />
                    </div>
                    <div class="flex flex-col min-w-0">
                      <span class="font-bold text-foreground text-[11px] flex items-center gap-1.5">
                        {issue.title}
                        <Badge variant="destructive" class="text-[7px] py-0 px-1 font-bold">{issue.priority}</Badge>
                      </span>
                      <span class="text-[9px] text-muted-foreground mt-0.5 leading-relaxed">{issue.desc}</span>
                    </div>
                  </div>

                  <div class="flex items-center gap-2 shrink-0 select-none">
                    <Button variant="outline" size="xs" class="h-6.5 text-[9px] font-bold cursor-pointer" onclick={() => handleTriggerAction(issue)}>
                      {issue.actionText}
                    </Button>
                    <button class="text-[8px] text-muted-foreground hover:text-foreground font-bold cursor-pointer" onclick={() => handleResolveIssue(issue.id)}>
                      {m.academics_class_overview_resolve()}
                    </button>
                  </div>
                </div>
              {/each}
            </div>
          </Card>
        </div>

        <!-- RIGHT SIDEBAR: AI Advisor (Seer Autofix), External Links, Activities (30%) -->
        <div class="flex flex-col gap-6 text-start">
          
          <!-- Orbis AI Class Advisor (Sentry Seer Autofix panel) -->
          <div class="card-tactile bg-primary/5 border border-primary/20 p-4 relative overflow-hidden flex flex-col justify-between shadow-sm transition-all duration-200">
            <div class="absolute -top-10 -right-10 w-20 h-20 bg-primary/10 rounded-full blur-xl pointer-events-none"></div>
            
            <div class="select-none border-b border-primary/15 pb-2 mb-3">
              <h3 class="text-xs font-bold text-foreground flex items-center gap-1.5">
                <Sparkles size={13} class="text-primary animate-pulse" />
                {m.academics_overview_advisor_title()}
              </h3>
              <p class="text-[9px] text-muted-foreground/80 mt-1 leading-normal">
                {m.academics_class_overview_advisor_desc()}
              </p>
            </div>

            <div class="flex flex-col gap-2 mb-4 text-[10px] leading-relaxed">
              <div class="flex gap-2 p-2 rounded-lg bg-background/50 border border-border/30">
                <span class="text-primary font-bold">1.</span>
                {#if attendanceRate === 0}
                  <span>{m.academics_class_overview_advisor_attendance_none()}</span>
                {:else if attendanceRate < 80}
                  <span>{m.academics_class_overview_advisor_attendance_critical({ rate: String(Math.round(attendanceRate)) })}</span>
                {:else if attendanceRate < 93}
                  <span>{m.academics_class_overview_advisor_attendance_low({ rate: String(Math.round(attendanceRate)) })}</span>
                {:else if attendanceRate < 97}
                  <span>{m.academics_class_overview_advisor_attendance_medium({ rate: String(Math.round(attendanceRate)) })}</span>
                {:else}
                  <span>{m.academics_class_overview_advisor_attendance_high({ rate: String(Math.round(attendanceRate)) })}</span>
                {/if}
              </div>
            </div>

            <Button
              variant="default"
              size="sm"
              class="w-full text-[10px] gap-1.5 h-8.5 font-bold cursor-pointer justify-center"
              onclick={() => toast.success(m.academics_class_overview_optimizing_toast())}
            >
              <Activity size={12} />
              {m.academics_class_overview_diagnostic()}
            </Button>
          </div>

          <!-- External Links -->
          <Card class="p-4 bg-card border border-border/60 flex flex-col justify-between shadow-sm card-tactile transition-all duration-200">
            <div class="flex items-center justify-between w-full select-none text-[9px] font-bold text-muted-foreground/80 mb-2">
              <div class="flex items-center gap-1.5 font-sans font-extrabold uppercase tracking-wider text-[8px] text-muted-foreground/75">
                <span class="h-1.5 w-1.5 rounded-full bg-brand-lime"></span>
                {m.academics_class_overview_external_links()}
              </div>
            </div>
            <div class="flex flex-col gap-2 mt-1 text-[10px] font-bold text-start">
              <a href={localizeHref(`/${orgId}/academics/schedule?classId=${classId}`)} class="flex items-center gap-1.5 text-primary hover:underline">
                <Landmark size={12} />
                <span>{m.academics_class_overview_weekly_timetable()}</span>
              </a>
              <a href={localizeHref(`/${orgId}/reports`)} class="flex items-center gap-1.5 text-primary hover:underline">
                <FileText size={12} />
                <span>{m.academics_class_overview_perf_ledger()}</span>
              </a>
            </div>
          </Card>
        </div>

      </div>

    </div>
  {/if}
</div>
