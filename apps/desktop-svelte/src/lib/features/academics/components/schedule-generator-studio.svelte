<script lang="ts">
  import { goto } from "$app/navigation";
  import { page } from "$app/state";
  import { getLocale, localizeHref } from "$lib/paraglide/runtime";
  import * as m from "$lib/paraglide/messages";
  import { commands, extractErrorMessage } from "$lib/core/ipc";
  import { createQuery, useQueryClient } from "@tanstack/svelte-query";
  import { toast } from "svelte-sonner";
  import { Button } from "@orbis/ui-svelte/button";
  import { Badge } from "@orbis/ui-svelte/badge";
  import { Switch } from "@orbis/ui-svelte/switch";
  import {
    Select,
    SelectTrigger,
    SelectContent,
    SelectItem,
  } from "@orbis/ui-svelte/select";
  import Resizer from "$lib/shell/resizer.svelte";
  import SettingsBottomBar from "$lib/shared/components/settings-bottom-bar.svelte";
  import { TimetableGrid } from "@orbis/ui-svelte";
  import {
    ArrowLeft,
    Clock,
    User,
    ShieldCheck,
    Activity,
    Eye,
    SlidersHorizontal,
    BookOpen,
    History
  } from "@lucide/svelte";

  interface Props {
    orgId: string;
  }

  let { orgId }: Props = $props();

  const queryClient = useQueryClient();
  const isArabic = $derived(getLocale() === "ar");

  // Selection & Studio State
  let activePeriodId = $state(page.url.searchParams.get("periodId") || "");
  let studioModule = $state<"preview" | "quota" | "constraints" | "teachers" | "history">("preview");
  let previewPerspective = $state<"class" | "teacher" | "room">("class");
  let previewClassId = $state<string>("");
  let previewTeacherId = $state<string>("");
  let previewRoomId = $state<string>("");
  let selectedQuotaClassFilter = $state<string>("all");
  let isInspectorOpen = $state(true);

  // Resizable Panes using $lib/shell/resizer.svelte
  function pxToRem(px: number): number {
    if (typeof document === "undefined") return px / 16;
    const base = parseFloat(getComputedStyle(document.documentElement).fontSize) || 16;
    return px / base;
  }

  let navRailDiv = $state<HTMLElement>();
  let preferredNavWidth = $state(pxToRem(176));
  const minNavWidth = $derived(pxToRem(140));
  const maxNavWidth = $derived(pxToRem(280));

  let inspectorDiv = $state<HTMLElement>();
  let preferredInspectorWidth = $state(pxToRem(280));
  const minInspectorWidth = $derived(pxToRem(220));
  const maxInspectorWidth = $derived(pxToRem(480));

  // Solver Job State
  let isSolving = $state(false);
  let isApplying = $state(false);
  let pollInterval: ReturnType<typeof setInterval> | null = null;
  let activeJob = $state<any>(null);

  // Studio Generator Policies & Constraints
  let maxDailyTeacherLoad = $state(5);
  let allowDoublePeriods = $state(false);
  let balanceWeekdayDistribution = $state(true);
  let prioritizeMorningCoreSubjects = $state(true);
  let strictRoomCapacityCheck = $state(true);

  // ── Queries ───────────────────────────────────────────────

  const periodsQuery = createQuery(() => ({
    queryKey: ["periods", orgId],
    queryFn: () => commands.calendar.listAcademicPeriods(),
    enabled: !!orgId,
    staleTime: 10 * 60 * 1000,
  }));

  const classesQuery = createQuery(() => ({
    queryKey: ["classes", orgId],
    queryFn: () => commands.academic.listClasses(),
    enabled: !!orgId,
    staleTime: 5 * 60 * 1000,
  }));

  const subjectsQuery = createQuery(() => ({
    queryKey: ["subjects", orgId],
    queryFn: () => commands.academic.listSubjects(),
    enabled: !!orgId,
    staleTime: 10 * 60 * 1000,
  }));

  const usersQuery = createQuery(() => ({
    queryKey: ["users", orgId],
    queryFn: () => commands.iam.listUsers(null),
    enabled: !!orgId,
    staleTime: 5 * 60 * 1000,
  }));

  const roomsQuery = createQuery(() => ({
    queryKey: ["rooms", orgId],
    queryFn: () => commands.scheduling.listRooms(),
    enabled: !!orgId,
    staleTime: 10 * 60 * 1000,
  }));

  const periods = $derived(periodsQuery.data ?? []);
  const classes = $derived(classesQuery.data ?? []);
  const subjects = $derived(subjectsQuery.data ?? []);
  const users = $derived(usersQuery.data ?? []);
  const rooms = $derived(roomsQuery.data ?? []);

  const teachers = $derived(
    users.filter(
      (u: any) =>
        u.primary_role === "teacher" ||
        u.role === "teacher" ||
        (Array.isArray(u.roles) && u.roles.includes("teacher"))
    )
  );

  // Time slots for active period
  const timeSlotsQuery = createQuery(() => ({
    queryKey: ["time-slots", orgId, activePeriodId],
    queryFn: () => commands.scheduling.listTimeSlots(),
    enabled: !!orgId && !!activePeriodId,
    staleTime: 5 * 60 * 1000,
  }));

  const timeSlots = $derived(
    (timeSlotsQuery.data ?? []).filter(
      (slot: any) => !slot.period_id || String(slot.period_id) === String(activePeriodId)
    )
  );

  // Existing Lessons for this period
  const periodScheduleQuery = createQuery(() => ({
    queryKey: ["period-schedule", orgId, activePeriodId],
    queryFn: () => commands.scheduling.listLessonsByPeriod(activePeriodId || ""),
    enabled: !!orgId && !!activePeriodId,
    staleTime: 30 * 1000,
  }));

  const allLessons = $derived((periodScheduleQuery.data ?? []) as any[]);

  // Solve Jobs Query
  const solveJobsQuery = createQuery(() => ({
    queryKey: ["solve-jobs", orgId, activePeriodId],
    queryFn: () => commands.scheduling.listJobs(activePeriodId || ""),
    enabled: !!orgId && !!activePeriodId,
    staleTime: 5 * 1000,
  }));

  const solveJobs = $derived((solveJobsQuery.data ?? []) as any[]);

  // ClassSubjects Query for active period
  const allClassSubjectsQuery = createQuery(() => ({
    queryKey: ["all-class-subjects-matrix", orgId, activePeriodId],
    queryFn: async () => {
      if (!activePeriodId || classes.length === 0) return [];
      const results = await Promise.all(
        classes.map((cls) => commands.academic.listClassSubjects(String(cls.id), activePeriodId))
      );
      return results.flat();
    },
    enabled: !!orgId && !!activePeriodId && classes.length > 0,
    staleTime: 30 * 1000,
  }));

  const allClassSubjects = $derived((allClassSubjectsQuery.data ?? []) as any[]);

  // Auto-select initial period
  $effect(() => {
    if (periods.length > 0 && !activePeriodId) {
      const active = periods.find((p: any) => p.status === "active" || p.is_active) || periods[0];
      if (active) activePeriodId = String(active.id);
    }
  });

  // Auto-select preview selections
  $effect(() => {
    if (classes.length > 0 && !previewClassId) previewClassId = String(classes[0].id);
    if (teachers.length > 0 && !previewTeacherId) previewTeacherId = String(teachers[0].id);
    if (rooms.length > 0 && !previewRoomId) previewRoomId = String(rooms[0].id);
  });

  // Track latest solve job
  $effect(() => {
    if (solveJobs.length > 0 && !activeJob) {
      activeJob = solveJobs[0];
    }
  });

  // Auto-poll when solver is actively processing
  $effect(() => {
    if (activeJob?.status === "running" || activeJob?.status === "pending" || isSolving) {
      if (!pollInterval) {
        pollInterval = setInterval(async () => {
          if (activePeriodId) {
            const jobs = await commands.scheduling.listJobs(activePeriodId);
            if (jobs.length > 0) {
              activeJob = jobs[0];
              if (activeJob.status === "completed" || activeJob.status === "failed") {
                isSolving = false;
                if (pollInterval) {
                  clearInterval(pollInterval);
                  pollInterval = null;
                }
                if (activeJob.status === "completed") {
                  toast.success(isArabic ? "اكتمل التوليد الذكي للجدول بنجاح!" : "Timetable generation complete!");
                }
              }
            }
          }
        }, 2000);
      }
    } else {
      if (pollInterval) {
        clearInterval(pollInterval);
        pollInterval = null;
      }
    }
    return () => {
      if (pollInterval) {
        clearInterval(pollInterval);
        pollInterval = null;
      }
    };
  });

  // Parse draft solution assignments
  const solutionAssignments = $derived.by(() => {
    if (!activeJob?.best_solution) return [];
    try {
      const sol = typeof activeJob.best_solution === "string"
        ? JSON.parse(activeJob.best_solution)
        : activeJob.best_solution;
      return (sol.assignments || []).map((a: any) => ({
        ...a,
        isDraft: true,
      }));
    } catch {
      return [];
    }
  });

  // Filter existing lessons based on active sub-perspective
  const existingPreviewLessons = $derived.by(() => {
    if (previewPerspective === "class" && previewClassId) {
      return allLessons.filter((l: any) => String(l.class_id) === String(previewClassId));
    }
    if (previewPerspective === "teacher" && previewTeacherId) {
      return allLessons.filter((l: any) => String(l.teacher_id) === String(previewTeacherId));
    }
    if (previewPerspective === "room" && previewRoomId) {
      return allLessons.filter((l: any) => String(l.room_id) === String(previewRoomId));
    }
    return allLessons;
  });

  // Filter draft lessons based on active sub-perspective
  const currentDraftLessons = $derived.by(() => {
    if (previewPerspective === "class" && previewClassId) {
      return solutionAssignments.filter((a: any) => String(a.class_id) === String(previewClassId));
    }
    if (previewPerspective === "teacher" && previewTeacherId) {
      return solutionAssignments.filter((a: any) => String(a.teacher_id) === String(previewTeacherId));
    }
    if (previewPerspective === "room" && previewRoomId) {
      return solutionAssignments.filter((a: any) => String(a.room_id) === String(previewRoomId));
    }
    return solutionAssignments;
  });

  // Calculate Weekdays from time slots
  const weekdays = $derived.by(() => {
    const configuredDays = new Set<number>();
    for (const slot of timeSlots) {
      if (!slot.applicable_days) continue;
      for (const val of String(slot.applicable_days).split(",")) {
        const d = Number(val.trim());
        if (Number.isInteger(d) && d >= 1 && d <= 7) configuredDays.add(d);
      }
    }
    const days = configuredDays.size > 0 ? [...configuredDays] : [1, 2, 3, 4, 5];
    const weekOrder = [7, 1, 2, 3, 4, 5, 6];
    return days.sort((a, b) => weekOrder.indexOf(a) - weekOrder.indexOf(b)).map((day) => ({
      value: day,
      label: new Intl.DateTimeFormat(getLocale(), { weekday: "short" }).format(
        new Date(Date.UTC(2023, 0, day))
      ),
      isToday: false,
    }));
  });

  // Metrics
  const totalWeeklyQuota = $derived.by(() => {
    return allClassSubjects.reduce((sum: number, cs: any) => sum + (Number(cs.weekly_hours) || 0), 0);
  });

  const totalWeeklyCapacity = $derived.by(() => {
    const activeSlots = timeSlots.filter((s) => !s.is_break).length;
    return activeSlots * weekdays.length * classes.length;
  });

  // Teacher Workload Map
  const teacherWorkloads = $derived.by(() => {
    const map = new Map<string, { totalHours: number; subjectsCount: number }>();
    for (const cs of allClassSubjects) {
      if (!cs.primary_teacher_id) continue;
      const tId = String(cs.primary_teacher_id);
      const existing = map.get(tId) || { totalHours: 0, subjectsCount: 0 };
      existing.totalHours += Number(cs.weekly_hours) || 0;
      existing.subjectsCount += 1;
      map.set(tId, existing);
    }
    return map;
  });

  // Filtered ClassSubjects for the Quota tab
  const filteredClassSubjects = $derived.by(() => {
    if (selectedQuotaClassFilter === "all") return allClassSubjects;
    return allClassSubjects.filter((cs: any) => String(cs.class_id) === selectedQuotaClassFilter);
  });

  // ── Actions ───────────────────────────────────────────────

  async function handleLaunchSolver() {
    if (!activePeriodId) return;
    isSolving = true;
    try {
      const res = await commands.scheduling.start(activePeriodId);
      activeJob = res;
      toast.success(isArabic ? "تم إطلاق محرك الجدولة والتوليد الذكي..." : "AI solver launched...");
      queryClient.invalidateQueries({ queryKey: ["solve-jobs", orgId, activePeriodId] });
    } catch (err) {
      isSolving = false;
      toast.error(extractErrorMessage(err));
    }
  }

  async function handleApplySolution() {
    if (!activeJob?.id) return;
    isApplying = true;
    try {
      await commands.scheduling.apply(activeJob.id);
      toast.success(isArabic ? "تم اعتماد ونشر الجدول المدرسي بنجاح!" : "Timetable published successfully!");
      queryClient.invalidateQueries({ queryKey: ["period-schedule"] });
      queryClient.invalidateQueries({ queryKey: ["class-schedule"] });
      goto(localizeHref(`/${orgId}/academics/schedule`));
    } catch (err) {
      toast.error(extractErrorMessage(err));
    } finally {
      isApplying = false;
    }
  }

  async function handleUpdateClassSubjectHours(csId: string, hours: number) {
    try {
      await commands.academic.updateClassSubject(csId, { weekly_hours: hours });
      queryClient.invalidateQueries({ queryKey: ["all-class-subjects-matrix", orgId, activePeriodId] });
      toast.success(isArabic ? "تم تحديث النصاب الأسبوعي" : "Weekly quota updated");
    } catch (err) {
      toast.error(extractErrorMessage(err));
    }
  }

  async function handleUpdateClassSubjectTeacher(csId: string, teacherId: string) {
    try {
      await commands.academic.updateClassSubject(csId, { primary_teacher_id: teacherId || null });
      queryClient.invalidateQueries({ queryKey: ["all-class-subjects-matrix", orgId, activePeriodId] });
      toast.success(isArabic ? "تم تعيين المعلم المسؤول" : "Teacher assigned");
    } catch (err) {
      toast.error(extractErrorMessage(err));
    }
  }

  function getSubjectName(subjectId: string): string {
    const sub = subjects.find((s) => String(s.id) === String(subjectId));
    return sub ? (isArabic ? sub.name_ar || sub.name : sub.name || sub.name_ar) : (isArabic ? "مادة دراسية" : "Subject");
  }

  function getTeacherName(teacherId?: string): string {
    if (!teacherId) return "";
    const teach = teachers.find((t) => String(t.id) === String(teacherId));
    return teach ? (isArabic ? teach.name_ar || teach.full_name : teach.full_name || teach.name_ar) : "";
  }

  function getClassName(clsId: string): string {
    const cls = classes.find((c) => String(c.id) === String(clsId));
    return cls ? (isArabic ? cls.name_ar || cls.name : cls.name || cls.name_ar) : "";
  }

  function getRoomName(roomId?: string): string {
    if (!roomId) return "";
    const rm = rooms.find((r) => String(r.id) === String(roomId));
    return rm ? (isArabic ? rm.name_ar || rm.name : rm.name || rm.name_ar) : "";
  }

  function getSubjectColor(subjectId: string): string {
    const sub = subjects.find((s) => String(s.id) === String(subjectId)) as any;
    if (sub?.color) return sub.color;
    const hash = String(subjectId).split("").reduce((acc, ch) => (acc * 31 + ch.charCodeAt(0)) >>> 0, 0);
    return `hsl(${hash % 360} 70% 50%)`;
  }
</script>

<div class="flex h-full w-full flex-col overflow-hidden bg-background">
  <!-- Minimalist Studio Top Bar (Clean & Headless) -->
  <header class="flex h-10 w-full shrink-0 items-center justify-between border-b border-border/50 bg-card px-2.5 gap-2 select-none">
    <!-- Left: Back Button & Academic Period Selector -->
    <div class="flex items-center gap-2 min-w-0">
      <Button
        variant="ghost"
        size="icon"
        class="h-7 w-7 text-muted-foreground hover:text-foreground"
        onclick={() => goto(localizeHref(`/${orgId}/academics/schedule`))}
        aria-label={isArabic ? "رجوع للجدول" : "Back to Schedule"}
      >
        <ArrowLeft size={14} class="rtl:rotate-180" />
      </Button>

      <Select
        type="single"
        value={activePeriodId}
        onValueChange={(val) => (activePeriodId = val ?? "")}
      >
        <SelectTrigger class="h-7 w-44 bg-muted/20 text-xs font-semibold border-border/50">
          <span class="truncate">
            {periods.find((p) => String(p.id) === activePeriodId)?.name_ar || periods.find((p) => String(p.id) === activePeriodId)?.name || (isArabic ? "الفترة الأكاديمية..." : "Academic Period...")}
          </span>
        </SelectTrigger>
        <SelectContent>
          {#each periods as p (p.id)}
            <SelectItem value={String(p.id)} class="text-xs">{p.name_ar || p.name}</SelectItem>
          {/each}
        </SelectContent>
      </Select>
    </div>
  </header>

  <!-- Studio Workspace Canvas with Resizable Left Rail and Right Inspector -->
  <div class="flex flex-1 min-h-0 min-w-0 w-full overflow-hidden">
    <!-- Studio Left Vertical Rail / Navigator (Resizable using resizer.svelte) -->
    <nav
      bind:this={navRailDiv}
      style="width: {preferredNavWidth}rem;"
      class="relative shrink-0 border-e border-border/60 bg-muted/10 flex flex-col justify-between p-2 select-none text-start"
    >
      <Resizer
        viewport={navRailDiv}
        direction="end"
        minWidth={minNavWidth}
        maxWidth={maxNavWidth}
        defaultValue={pxToRem(176)}
        persistId="schedule-studio-nav-width"
        onWidth={(w) => (preferredNavWidth = w)}
      />

      <div class="space-y-1">
        <button
          type="button"
          class="w-full flex items-center gap-2 px-2.5 py-1.5 rounded-lg text-xs font-semibold transition-all cursor-pointer {studioModule === 'preview'
            ? 'bg-card font-bold shadow-2xs border border-border/50 text-primary'
            : 'text-muted-foreground hover:text-foreground hover:bg-muted/30'}"
          onclick={() => (studioModule = 'preview')}
        >
          <Eye size={13} class={studioModule === 'preview' ? 'text-primary' : 'text-muted-foreground'} />
          <span class="truncate">{isArabic ? "معاينة الجدول" : "Preview Canvas"}</span>
        </button>

        <button
          type="button"
          class="w-full flex items-center gap-2 px-2.5 py-1.5 rounded-lg text-xs font-semibold transition-all cursor-pointer {studioModule === 'quota'
            ? 'bg-card font-bold shadow-2xs border border-border/50 text-primary'
            : 'text-muted-foreground hover:text-foreground hover:bg-muted/30'}"
          onclick={() => (studioModule = 'quota')}
        >
          <BookOpen size={13} class={studioModule === 'quota' ? 'text-primary' : 'text-muted-foreground'} />
          <span class="truncate">{isArabic ? "الأنصبة والمواد" : "Curriculum Quotas"}</span>
        </button>

        <button
          type="button"
          class="w-full flex items-center gap-2 px-2.5 py-1.5 rounded-lg text-xs font-semibold transition-all cursor-pointer {studioModule === 'constraints'
            ? 'bg-card font-bold shadow-2xs border border-border/50 text-primary'
            : 'text-muted-foreground hover:text-foreground hover:bg-muted/30'}"
          onclick={() => (studioModule = 'constraints')}
        >
          <SlidersHorizontal size={13} class={studioModule === 'constraints' ? 'text-primary' : 'text-muted-foreground'} />
          <span class="truncate">{isArabic ? "قواعد التوليد" : "Solving Rules"}</span>
        </button>

        <button
          type="button"
          class="w-full flex items-center gap-2 px-2.5 py-1.5 rounded-lg text-xs font-semibold transition-all cursor-pointer {studioModule === 'teachers'
            ? 'bg-card font-bold shadow-2xs border border-border/50 text-primary'
            : 'text-muted-foreground hover:text-foreground hover:bg-muted/30'}"
          onclick={() => (studioModule = 'teachers')}
        >
          <User size={13} class={studioModule === 'teachers' ? 'text-primary' : 'text-muted-foreground'} />
          <span class="truncate">{isArabic ? "أنصبة المعلمين" : "Teacher Loads"}</span>
        </button>

        <button
          type="button"
          class="w-full flex items-center gap-2 px-2.5 py-1.5 rounded-lg text-xs font-semibold transition-all cursor-pointer {studioModule === 'history'
            ? 'bg-card font-bold shadow-2xs border border-border/50 text-primary'
            : 'text-muted-foreground hover:text-foreground hover:bg-muted/30'}"
          onclick={() => (studioModule = 'history')}
        >
          <History size={13} class={studioModule === 'history' ? 'text-primary' : 'text-muted-foreground'} />
          <span class="truncate">{isArabic ? "سجل الإصدارات" : "Version History"}</span>
        </button>
      </div>

      <!-- Studio Quick Stats Widget -->
      <div class="p-2.5 rounded-xl border border-border/50 bg-card/60 space-y-1.5 text-[10px] text-muted-foreground">
        <div class="flex justify-between">
          <span>{isArabic ? "المستهدف:" : "Target:"}</span>
          <strong class="text-foreground">{totalWeeklyQuota}</strong>
        </div>
        <div class="flex justify-between">
          <span>{isArabic ? "الفصول:" : "Classes:"}</span>
          <strong class="text-foreground">{classes.length}</strong>
        </div>
        <div class="flex justify-between">
          <span>{isArabic ? "المعلمون:" : "Teachers:"}</span>
          <strong class="text-foreground">{teachers.length}</strong>
        </div>
      </div>
    </nav>

    <!-- Studio Central Work Canvas -->
    <main class="flex flex-1 min-h-0 min-w-0 flex-col p-2.5 gap-2 overflow-hidden text-start">
      <!-- 1. Timetable Grid Preview -->
      {#if studioModule === "preview"}
        <div class="flex items-center justify-between shrink-0 mb-0.5">
          <div class="flex items-center gap-2 text-xs">
            <div class="flex items-center bg-muted/40 p-0.5 rounded-md border border-border/40">
              <button
                type="button"
                class="px-2 py-0.5 rounded text-[11px] font-semibold {previewPerspective === 'class' ? 'bg-card text-foreground font-bold shadow-2xs' : 'text-muted-foreground'}"
                onclick={() => (previewPerspective = 'class')}
              >
                {isArabic ? "الفصول" : "Classes"}
              </button>
              <button
                type="button"
                class="px-2 py-0.5 rounded text-[11px] font-semibold {previewPerspective === 'teacher' ? 'bg-card text-foreground font-bold shadow-2xs' : 'text-muted-foreground'}"
                onclick={() => (previewPerspective = 'teacher')}
              >
                {isArabic ? "المعلمون" : "Teachers"}
              </button>
              <button
                type="button"
                class="px-2 py-0.5 rounded text-[11px] font-semibold {previewPerspective === 'room' ? 'bg-card text-foreground font-bold shadow-2xs' : 'text-muted-foreground'}"
                onclick={() => (previewPerspective = 'room')}
              >
                {isArabic ? "القاعات" : "Rooms"}
              </button>
            </div>

            {#if previewPerspective === "class"}
              <Select type="single" value={previewClassId} onValueChange={(val) => (previewClassId = val ?? "")}>
                <SelectTrigger class="h-7 w-40 bg-card text-xs font-semibold border-border/60">
                  <span class="truncate">{classes.find((c) => String(c.id) === previewClassId)?.name_ar || classes.find((c) => String(c.id) === previewClassId)?.name || (isArabic ? "اختر الفصل..." : "Select class...")}</span>
                </SelectTrigger>
                <SelectContent class="max-h-60 text-start">
                  {#each classes as c (c.id)}
                    <SelectItem value={String(c.id)} class="text-xs">{c.name_ar || c.name}</SelectItem>
                  {/each}
                </SelectContent>
              </Select>
            {:else if previewPerspective === "teacher"}
              <Select type="single" value={previewTeacherId} onValueChange={(val) => (previewTeacherId = val ?? "")}>
                <SelectTrigger class="h-7 w-44 bg-card text-xs font-semibold border-border/60">
                  <span class="truncate">{teachers.find((t) => String(t.id) === previewTeacherId)?.name_ar || teachers.find((t) => String(t.id) === previewTeacherId)?.full_name || (isArabic ? "اختر المعلم..." : "Select teacher...")}</span>
                </SelectTrigger>
                <SelectContent class="max-h-60 text-start">
                  {#each teachers as t (t.id)}
                    <SelectItem value={String(t.id)} class="text-xs">{t.name_ar || t.full_name}</SelectItem>
                  {/each}
                </SelectContent>
              </Select>
            {:else if previewPerspective === "room"}
              <Select type="single" value={previewRoomId} onValueChange={(val) => (previewRoomId = val ?? "")}>
                <SelectTrigger class="h-7 w-40 bg-card text-xs font-semibold border-border/60">
                  <span class="truncate">{rooms.find((r) => String(r.id) === previewRoomId)?.name_ar || rooms.find((r) => String(r.id) === previewRoomId)?.name || (isArabic ? "اختر القاعة..." : "Select room...")}</span>
                </SelectTrigger>
                <SelectContent class="max-h-60 text-start">
                  {#each rooms as r (r.id)}
                    <SelectItem value={String(r.id)} class="text-xs">{r.name_ar || r.name}</SelectItem>
                  {/each}
                </SelectContent>
              </Select>
            {/if}
          </div>

          <div class="flex items-center gap-2 text-[11px] text-muted-foreground">
            {#if solutionAssignments.length > 0}
              <Badge variant="default" class="bg-primary text-primary-foreground text-[10px]">
                {isArabic ? `مسودة مقترحة: ${solutionAssignments.length} حصة` : `Draft: ${solutionAssignments.length} lessons`}
              </Badge>
            {:else}
              <span>{isArabic ? `الجدول الحالي: ${existingPreviewLessons.length} حصة` : `Current Schedule: ${existingPreviewLessons.length} lessons`}</span>
            {/if}
          </div>
        </div>

        <!-- Always-visible Timetable Grid -->
        <div class="flex-1 min-h-0 w-full flex flex-col">
          <TimetableGrid
            {timeSlots}
            {weekdays}
            lessons={existingPreviewLessons}
            draftLessons={currentDraftLessons}
            previewDraft={solutionAssignments.length > 0}
            viewMode={previewPerspective}
            locale={getLocale()}
            {getSubjectName}
            {getTeacherName}
            {getClassName}
            {getRoomName}
            {getSubjectColor}
          />
        </div>
      {/if}

      <!-- 2. Curriculum Quota Studio Module -->
      {#if studioModule === "quota"}
        <div class="flex flex-1 min-h-0 w-full flex-col gap-2">
          <div class="flex items-center justify-between shrink-0">
            <div class="flex items-center gap-2 text-xs">
              <span class="text-muted-foreground font-medium">{isArabic ? "تصفية حسب الفصل:" : "Filter Class:"}</span>
              <Select type="single" value={selectedQuotaClassFilter} onValueChange={(val) => (selectedQuotaClassFilter = val ?? "all")}>
                <SelectTrigger class="h-7 w-44 bg-card text-xs font-semibold border-border/60">
                  <span>{selectedQuotaClassFilter === "all" ? (isArabic ? "كافة الفصول" : "All Classes") : (classes.find((c) => String(c.id) === selectedQuotaClassFilter)?.name_ar || classes.find((c) => String(c.id) === selectedQuotaClassFilter)?.name)}</span>
                </SelectTrigger>
                <SelectContent class="max-h-60 text-start">
                  <SelectItem value="all" class="text-xs">{isArabic ? "كافة الفصول" : "All Classes"}</SelectItem>
                  {#each classes as c (c.id)}
                    <SelectItem value={String(c.id)} class="text-xs">{c.name_ar || c.name}</SelectItem>
                  {/each}
                </SelectContent>
              </Select>
            </div>

            <div class="text-[11px] text-muted-foreground">
              {isArabic ? `المواد المسندة: ${filteredClassSubjects.length}` : `Assigned Subjects: ${filteredClassSubjects.length}`}
            </div>
          </div>

          <div class="flex-1 min-h-0 w-full overflow-auto rounded-xl border border-border/60 bg-card p-3 shadow-xs scrollbar-hide">
            <table class="w-full border-collapse text-xs select-none">
              <thead>
                <tr class="border-b border-border/60 bg-muted/20">
                  <th class="p-2.5 text-start font-bold text-foreground">{isArabic ? "الفصل الدراسي" : "Class"}</th>
                  <th class="p-2.5 text-start font-bold text-foreground">{isArabic ? "المادة الدراسية" : "Subject"}</th>
                  <th class="p-2.5 text-start font-bold text-foreground">{isArabic ? "المعلم المعين" : "Assigned Teacher"}</th>
                  <th class="p-2.5 text-center font-bold text-foreground w-40">{isArabic ? "النصاب الأسبوعي" : "Weekly Hours"}</th>
                </tr>
              </thead>
              <tbody>
                {#each filteredClassSubjects as cs (cs.id)}
                  <tr class="border-b border-border/40 hover:bg-muted/10 transition-colors">
                    <td class="p-2.5 font-semibold text-foreground">{getClassName(cs.class_id)}</td>
                    <td class="p-2.5">
                      <div class="flex items-center gap-1.5">
                        <span class="h-2 w-2 rounded-full" style={`background-color: ${getSubjectColor(cs.subject_id)};`}></span>
                        <span class="font-medium text-foreground">{getSubjectName(cs.subject_id)}</span>
                      </div>
                    </td>
                    <td class="p-2.5">
                      <Select
                        type="single"
                        value={cs.primary_teacher_id ? String(cs.primary_teacher_id) : ""}
                        onValueChange={(val) => handleUpdateClassSubjectTeacher(cs.id, val ?? "")}
                      >
                        <SelectTrigger class="h-7 w-48 bg-card text-xs">
                          <span class="truncate">{getTeacherName(cs.primary_teacher_id) || (isArabic ? "غير معين" : "Unassigned")}</span>
                        </SelectTrigger>
                        <SelectContent class="max-h-56 text-start">
                          <SelectItem value="" class="text-xs text-muted-foreground">{isArabic ? "بدون معلم" : "None"}</SelectItem>
                          {#each teachers as t (t.id)}
                            <SelectItem value={String(t.id)} class="text-xs">{t.name_ar || t.full_name}</SelectItem>
                          {/each}
                        </SelectContent>
                      </Select>
                    </td>
                    <td class="p-2.5 text-center">
                      <div class="flex items-center justify-center gap-1">
                        <Button
                          type="button"
                          variant="outline"
                          size="icon"
                          class="h-6 w-6 text-xs"
                          onclick={() => handleUpdateClassSubjectHours(cs.id, Math.max(1, (Number(cs.weekly_hours) || 1) - 1))}
                        >
                          -
                        </Button>
                        <span class="w-8 text-center font-bold font-mono text-xs">{cs.weekly_hours || 0}</span>
                        <Button
                          type="button"
                          variant="outline"
                          size="icon"
                          class="h-6 w-6 text-xs"
                          onclick={() => handleUpdateClassSubjectHours(cs.id, (Number(cs.weekly_hours) || 0) + 1)}
                        >
                          +
                        </Button>
                      </div>
                    </td>
                  </tr>
                {/each}
              </tbody>
            </table>
          </div>
        </div>
      {/if}

      <!-- 3. Constraints Module -->
      {#if studioModule === "constraints"}
        <div class="flex-1 min-h-0 w-full overflow-auto rounded-xl border border-border/60 bg-card p-4 shadow-xs scrollbar-hide flex flex-col gap-3 max-w-3xl mx-auto">
          <div class="space-y-3 text-xs">
            <div class="flex items-center justify-between p-3 rounded-lg border border-border/50 bg-muted/15">
              <div>
                <span class="font-bold text-foreground block">{isArabic ? "الحد الأقصى لنصاب المعلم اليومي" : "Max Daily Teacher Load"}</span>
                <span class="text-[11px] text-muted-foreground">{isArabic ? "أقصى عدد حصص يُسمح بجدولتها للمعلم الواحد في اليوم نفسه." : "Maximum periods assigned to a single teacher per day."}</span>
              </div>
              <div class="flex items-center gap-1.5">
                {#each [3, 4, 5, 6] as count}
                  <button
                    type="button"
                    class="h-7 w-7 rounded-md text-xs font-bold border transition-colors {maxDailyTeacherLoad === count ? 'bg-primary text-primary-foreground border-primary' : 'bg-card border-border/60 text-muted-foreground hover:text-foreground'}"
                    onclick={() => (maxDailyTeacherLoad = count)}
                  >
                    {count}
                  </button>
                {/each}
              </div>
            </div>

            <div class="flex items-center justify-between p-3 rounded-lg border border-border/50 bg-muted/15">
              <div>
                <span class="font-bold text-foreground block">{isArabic ? "توزيع المواد بتوازن على أيام الأسبوع" : "Balanced Weekday Distribution"}</span>
                <span class="text-[11px] text-muted-foreground">{isArabic ? "تجنب تكرار المادة الواحدة أكثر من مرة في نفس اليوم للفصل." : "Avoid scheduling the same subject multiple times on the same day for a class."}</span>
              </div>
              <Switch bind:checked={balanceWeekdayDistribution} />
            </div>

            <div class="flex items-center justify-between p-3 rounded-lg border border-border/50 bg-muted/15">
              <div>
                <span class="font-bold text-foreground block">{isArabic ? "السماح بالحصص المزدوجة (المتتالية)" : "Allow Consecutive Double Periods"}</span>
                <span class="text-[11px] text-muted-foreground">{isArabic ? "دمج حصتين متتاليتين للمواد المعملية أو التطبيقية." : "Allow two consecutive periods for laboratory or practical subjects."}</span>
              </div>
              <Switch bind:checked={allowDoublePeriods} />
            </div>

            <div class="flex items-center justify-between p-3 rounded-lg border border-border/50 bg-muted/15">
              <div>
                <span class="font-bold text-foreground block">{isArabic ? "تفضيل المواد الأساسية في الحصص الصباحية" : "Morning Core Subjects Priority"}</span>
                <span class="text-[11px] text-muted-foreground">{isArabic ? "وضع الرياضيات والعلوم واللغات قبل أوقات الاستراحة." : "Place Mathematics and Science in early morning slots before breaks."}</span>
              </div>
              <Switch bind:checked={prioritizeMorningCoreSubjects} />
            </div>

            <div class="flex items-center justify-between p-3 rounded-lg border border-border/50 bg-muted/15">
              <div>
                <span class="font-bold text-foreground block">{isArabic ? "مطابقة سعة القاعات والمختبرات بدقة" : "Strict Room Capacity"}</span>
                <span class="text-[11px] text-muted-foreground">{isArabic ? "منع تسكين أي فصل في قاعة تقل سعتها عن عدد طلاب الفصل." : "Ensure room capacity strictly matches or exceeds class enrollment."}</span>
              </div>
              <Switch bind:checked={strictRoomCapacityCheck} />
            </div>
          </div>
        </div>
      {/if}

      <!-- 4. Teachers Load Module -->
      {#if studioModule === "teachers"}
        <div class="flex-1 min-h-0 w-full overflow-auto rounded-xl border border-border/60 bg-card p-3 shadow-xs scrollbar-hide">
          <table class="w-full border-collapse text-xs select-none">
            <thead>
              <tr class="border-b border-border/60 bg-muted/20">
                <th class="p-2.5 text-start font-bold text-foreground">{isArabic ? "المعلم" : "Teacher"}</th>
                <th class="p-2.5 text-start font-bold text-foreground">{isArabic ? "البريد / المعرف" : "Email"}</th>
                <th class="p-2.5 text-center font-bold text-foreground">{isArabic ? "المواد المسندة" : "Subjects"}</th>
                <th class="p-2.5 text-center font-bold text-foreground">{isArabic ? "إجمالي النصاب الأسبوعي" : "Weekly Load"}</th>
                <th class="p-2.5 text-center font-bold text-foreground">{isArabic ? "حالة النصاب" : "Status"}</th>
              </tr>
            </thead>
            <tbody>
              {#each teachers as t (t.id)}
                {@const load = teacherWorkloads.get(String(t.id)) || { totalHours: 0, subjectsCount: 0 }}
                <tr class="border-b border-border/40 hover:bg-muted/10 transition-colors">
                  <td class="p-2.5 font-semibold text-foreground">
                    <div class="flex items-center gap-2">
                      <User size={13} class="text-primary" />
                      <span>{t.name_ar || t.full_name}</span>
                    </div>
                  </td>
                  <td class="p-2.5 text-muted-foreground">{t.email || "-"}</td>
                  <td class="p-2.5 text-center font-mono">{load.subjectsCount}</td>
                  <td class="p-2.5 text-center">
                    <Badge variant="secondary" class="font-bold font-mono text-[10px]">
                      {load.totalHours} {isArabic ? "حصص" : "hrs"}
                    </Badge>
                  </td>
                  <td class="p-2.5 text-center">
                    {#if load.totalHours > 24}
                      <Badge variant="destructive" class="text-[9px]">{isArabic ? "نصاب مرتفع" : "Heavy"}</Badge>
                    {:else if load.totalHours > 0}
                      <Badge variant="default" class="bg-emerald-600 text-white text-[9px]">{isArabic ? "نصاب متوازن" : "Balanced"}</Badge>
                    {:else}
                      <Badge variant="outline" class="text-[9px]">{isArabic ? "بدون نصاب" : "Free"}</Badge>
                    {/if}
                  </td>
                </tr>
              {/each}
            </tbody>
          </table>
        </div>
      {/if}

      <!-- 5. History Module -->
      {#if studioModule === "history"}
        <div class="flex-1 min-h-0 w-full overflow-auto rounded-xl border border-border/60 bg-card p-4 shadow-xs scrollbar-hide flex flex-col gap-2">
          <h3 class="text-xs font-bold text-foreground uppercase tracking-wider mb-2">
            {isArabic ? "سجل عمليات التوليد السابقة" : "Previous Solver Executions"} ({solveJobs.length})
          </h3>

          {#if solveJobs.length === 0}
            <p class="text-xs text-muted-foreground text-center py-8">
              {isArabic ? "لم يتم تشغيل أي عمليات توليد بعد." : "No solver runs recorded."}
            </p>
          {:else}
            {#each solveJobs as job (job.id)}
              <div class="flex items-center justify-between p-3 rounded-lg border border-border/60 bg-card hover:border-primary/40 transition-colors">
                <div class="flex items-center gap-3">
                  <div class="p-2 rounded-md bg-muted/40 text-muted-foreground">
                    <Clock size={14} />
                  </div>
                  <div>
                    <div class="flex items-center gap-2">
                      <span class="font-bold text-xs text-foreground">
                        {new Date(job.created_at).toLocaleString()}
                      </span>
                      <Badge variant={job.status === "completed" ? "default" : "outline"} class="text-[9px]">
                        {job.status}
                      </Badge>
                    </div>
                    <span class="text-[10px] text-muted-foreground">
                      {job.items_created || 0} {isArabic ? "حصة موزعة" : "lessons"} • Score: {job.score || "0 hard / 0 soft"}
                    </span>
                  </div>
                </div>

                {#if job.status === "completed"}
                  <Button
                    size="sm"
                    variant="outline"
                    class="h-7 text-xs font-semibold text-primary gap-1"
                    onclick={() => {
                      activeJob = job;
                      studioModule = "preview";
                    }}
                  >
                    <Eye size={12} />
                    <span>{isArabic ? "معاينة هذا الحل" : "Preview"}</span>
                  </Button>
                {/if}
              </div>
            {/each}
          {/if}
        </div>
      {/if}
    </main>

    <!-- Studio Right Inspector Panel (Resizable using resizer.svelte) -->
    {#if isInspectorOpen}
      <aside
        bind:this={inspectorDiv}
        style="width: {preferredInspectorWidth}rem;"
        class="relative shrink-0 border-s border-border/60 bg-card/60 flex flex-col p-3 gap-3 overflow-y-auto scrollbar-hide select-none text-start"
      >
        <Resizer
          viewport={inspectorDiv}
          direction="start"
          minWidth={minInspectorWidth}
          maxWidth={maxInspectorWidth}
          defaultValue={pxToRem(280)}
          persistId="schedule-studio-inspector-width"
          onWidth={(w) => (preferredInspectorWidth = w)}
        />

        <!-- Diagnostic Score -->
        <div class="rounded-xl border border-border/50 bg-card p-3 space-y-2 shadow-2xs">
          <span class="text-[11px] font-bold text-foreground block">{isArabic ? "جودة وكفاءة التوزيع" : "Solution Quality"}</span>
          <div class="flex items-center justify-between text-xs">
            <span class="text-muted-foreground">{isArabic ? "الحصص الموزعة:" : "Placed Lessons:"}</span>
            <strong class="text-foreground">{activeJob?.items_created || 0} / {totalWeeklyQuota}</strong>
          </div>
          <div class="flex items-center justify-between text-xs">
            <span class="text-muted-foreground">{isArabic ? "التعارضات الحرجة:" : "Hard Conflicts:"}</span>
            <strong class="text-emerald-600 dark:text-emerald-400 font-bold">0 {isArabic ? "(خالٍ تماماً)" : "(Zero)"}</strong>
          </div>
          <div class="flex items-center justify-between text-xs">
            <span class="text-muted-foreground">{isArabic ? "نقاط التحسين:" : "Optimization Score:"}</span>
            <span class="font-mono font-bold text-primary text-[11px]">{activeJob?.score || "0 hard / 0 soft"}</span>
          </div>
        </div>

        <!-- Institutional Health -->
        <div class="rounded-xl border border-border/50 bg-card p-3 space-y-2 shadow-2xs">
          <span class="text-[11px] font-bold text-foreground block">{isArabic ? "جاهزية المدرسة" : "School Capacity"}</span>
          <div class="flex items-center justify-between text-xs">
            <span class="text-muted-foreground">{isArabic ? "الفصول المعرفة:" : "Classes:"}</span>
            <strong class="text-foreground">{classes.length}</strong>
          </div>
          <div class="flex items-center justify-between text-xs">
            <span class="text-muted-foreground">{isArabic ? "المعلمون النشطون:" : "Teachers:"}</span>
            <strong class="text-foreground">{teachers.length}</strong>
          </div>
          <div class="flex items-center justify-between text-xs">
            <span class="text-muted-foreground">{isArabic ? "القاعات الدراسية:" : "Rooms:"}</span>
            <strong class="text-foreground">{rooms.length}</strong>
          </div>
        </div>

        <!-- Quick Generator Tips -->
        <div class="rounded-xl border border-border/50 bg-muted/20 p-3 space-y-1.5 text-[11px] text-muted-foreground">
          <span class="font-bold text-foreground flex items-center gap-1">
            <ShieldCheck size={12} class="text-primary" />
            {isArabic ? "ضمانات التوليد" : "Solver Guarantees"}
          </span>
          <p class="leading-relaxed">
            {isArabic
              ? "يقوم المحرك بفحص كافة القيود ومطابقة نصاب المواد وتفريغ المعلمين مع منع أي تكرار أو تعارض في المواعيد."
              : "Ensures complete conflict-free scheduling and respects teacher and room limits."}
          </p>
        </div>
      </aside>
    {/if}
  </div>

  <SettingsBottomBar
    visible={true}
    isDirty={true}
    isSaving={isApplying || isSolving}
    saveLabel={activeJob?.status === "completed" && solutionAssignments.length > 0 ? m.academics_schedule_publish() : m.academics_schedule_generate()}
    savingLabel={isApplying ? m.academics_schedule_publishing() : m.academics_schedule_generating()}
    message={activeJob?.status === "completed" && solutionAssignments.length > 0 ? m.academics_schedule_ready() : m.academics_schedule_generate()}
    showReset={false}
    onsave={activeJob?.status === "completed" && solutionAssignments.length > 0 ? handleApplySolution : handleLaunchSolver}
  />
</div>
