<script lang="ts">
  import { page } from "$app/state";
  import { goto } from "$app/navigation";
  import { commands, extractErrorMessage } from "$lib/core/ipc";
  import { getLocale, localizeHref } from "$lib/paraglide/runtime";
  import { toast } from "svelte-sonner";
  import { createQuery, useQueryClient } from "@tanstack/svelte-query";
  import { TimetableGrid } from "@orbis/ui-svelte";
  import { Button } from "@orbis/ui-svelte/button";
  import { Skeleton } from "@orbis/ui-svelte/skeleton";
  import { Badge } from "@orbis/ui-svelte/badge";
  import { AlertTriangle, Clock, Users, User, MapPin, Sparkles } from "@lucide/svelte";

  // Feature Components
  import {
    ScheduleToolbar,
    ScheduleMatrix,
    LessonSheet,
    TimeSlotSheet,
    SolverDrawer,
    ScheduleExportDialog,
  } from "$lib/features/academics";
  import ErrorState from "$lib/shared/components/error-state.svelte";

  let { data }: { data: { orgId: string } } = $props();

  const queryClient = useQueryClient();
  const isArabic = $derived(getLocale() === "ar");

  // Selection & View State
  let activePeriodId = $state<string>("");
  let viewMode = $state<"class" | "teacher" | "room" | "subject" | "matrix">("class");
  let selectedClassId = $state<string>("");
  let selectedTeacherId = $state<string>("");
  let selectedRoomId = $state<string>("");
  let selectedSubjectId = $state<string>("");
  let searchTerm = $state("");
  let compactView = $state(false);
  let selectedMatrixDay = $state(1);

  // Dialog & Sheet States
  let isLessonSheetOpen = $state(false);
  let isTimeSlotsSheetOpen = $state(false);
  let isSolverDrawerOpen = $state(false);
  let isExportDialogOpen = $state(false);

  // Active Lesson for editing / creating
  let targetLesson = $state<any | null>(null);
  let activeCellDay = $state(1);
  let activeCellSlotId = $state("");

  // AI Solver Draft State
  let previewDraft = $state(false);
  let activeSolveJob = $state<any>(null);

  // ── Queries ───────────────────────────────────────────────

  const periodsQuery = createQuery(() => ({
    queryKey: ["periods", data.orgId],
    queryFn: () => commands.calendar.listAcademicPeriods(),
    enabled: !!data.orgId,
    staleTime: 10 * 60 * 1000,
  }));

  const classesQuery = createQuery(() => ({
    queryKey: ["classes", data.orgId],
    queryFn: () => commands.academic.listClasses(),
    enabled: !!data.orgId,
    staleTime: 5 * 60 * 1000,
  }));

  const subjectsQuery = createQuery(() => ({
    queryKey: ["subjects", data.orgId],
    queryFn: () => commands.academic.listSubjects(),
    enabled: !!data.orgId,
    staleTime: 10 * 60 * 1000,
  }));

  const usersQuery = createQuery(() => ({
    queryKey: ["users", data.orgId],
    queryFn: () => commands.iam.listUsers(null),
    enabled: !!data.orgId,
    staleTime: 5 * 60 * 1000,
  }));

  const roomsQuery = createQuery(() => ({
    queryKey: ["rooms", data.orgId],
    queryFn: () => commands.scheduling.listRooms(),
    enabled: !!data.orgId,
    staleTime: 10 * 60 * 1000,
  }));

  const periods = $derived(periodsQuery.data ?? []);
  const classes = $derived(classesQuery.data ?? []);
  const subjects = $derived(subjectsQuery.data ?? []);
  const users = $derived(usersQuery.data ?? []);
  const rooms = $derived(roomsQuery.data ?? []);

  // Filter teachers exclusively
  const teachers = $derived(
    users.filter(
      (u: any) =>
        u.primary_role === "teacher" ||
        u.role === "teacher" ||
        (Array.isArray(u.roles) && u.roles.includes("teacher"))
    )
  );

  // Auto-select active period
  $effect(() => {
    if (periods.length > 0 && !activePeriodId) {
      const active = periods.find((p: any) => p.status === "active" || p.is_active) || periods[0];
      if (active) {
        activePeriodId = String(active.id);
      }
    }
  });

  // URL query param initializers & defaults
  $effect(() => {
    if (classes.length > 0 && !selectedClassId) {
      const param = page.url.searchParams.get("classId") || page.url.searchParams.get("class");
      if (param && classes.some((c: any) => String(c.id) === param)) {
        selectedClassId = param;
        viewMode = "class";
      } else {
        selectedClassId = String(classes[0].id);
      }
    }
  });

  $effect(() => {
    if (teachers.length > 0 && !selectedTeacherId) {
      const param = page.url.searchParams.get("teacherId") || page.url.searchParams.get("teacher");
      if (param && teachers.some((t: any) => String(t.id) === param)) {
        selectedTeacherId = param;
        viewMode = "teacher";
      } else {
        selectedTeacherId = String(teachers[0].id);
      }
    }
  });

  $effect(() => {
    if (rooms.length > 0 && !selectedRoomId) {
      selectedRoomId = String(rooms[0].id);
    }
  });

  $effect(() => {
    if (subjects.length > 0 && !selectedSubjectId) {
      selectedSubjectId = String(subjects[0].id);
    }
  });

  // Time Slots Query
  const timeSlotsQuery = createQuery(() => ({
    queryKey: ["time-slots", data.orgId, activePeriodId],
    queryFn: () => commands.scheduling.listTimeSlots(),
    enabled: !!data.orgId && !!activePeriodId,
    staleTime: 5 * 60 * 1000,
  }));

  const timeSlots = $derived(
    (timeSlotsQuery.data ?? []).filter(
      (slot: any) => !slot.period_id || String(slot.period_id) === String(activePeriodId)
    )
  );

  // Schedule Query for active period
  const periodScheduleQuery = createQuery(() => ({
    queryKey: ["period-schedule", data.orgId, activePeriodId],
    queryFn: () => commands.scheduling.listLessonsByPeriod(activePeriodId),
    enabled: !!data.orgId && !!activePeriodId,
    staleTime: 60 * 1000,
  }));

  const allLessons = $derived((periodScheduleQuery.data ?? []) as any[]);

  // Detect duplicate slots / collisions across loaded lessons
  const conflictLessonIds = $derived.by(() => {
    const ids = new Set<string>();
    const seenTeacherSlots = new Map<string, string>();
    const seenClassSlots = new Map<string, string>();
    const seenRoomSlots = new Map<string, string>();

    for (const l of allLessons) {
      const slotKey = `${l.day_of_week}_${l.time_slot_id}`;
      if (l.teacher_id) {
        const teacherKey = `${l.teacher_id}_${slotKey}`;
        if (seenTeacherSlots.has(teacherKey)) {
          ids.add(String(l.id));
          ids.add(seenTeacherSlots.get(teacherKey)!);
        } else {
          seenTeacherSlots.set(teacherKey, String(l.id));
        }
      }
      if (l.class_id) {
        const classKey = `${l.class_id}_${slotKey}`;
        if (seenClassSlots.has(classKey)) {
          ids.add(String(l.id));
          ids.add(seenClassSlots.get(classKey)!);
        } else {
          seenClassSlots.set(classKey, String(l.id));
        }
      }
      if (l.room_id) {
        const roomKey = `${l.room_id}_${slotKey}`;
        if (seenRoomSlots.has(roomKey)) {
          ids.add(String(l.id));
          ids.add(seenRoomSlots.get(roomKey)!);
        } else {
          seenRoomSlots.set(roomKey, String(l.id));
        }
      }
    }
    return ids;
  });

  // Filter lessons based on active view mode and selections
  const filteredLessons = $derived.by(() => {
    let list = allLessons;
    if (viewMode === "class") {
      list = allLessons.filter((l) => String(l.class_id) === selectedClassId);
    } else if (viewMode === "teacher") {
      list = allLessons.filter((l) => String(l.teacher_id) === selectedTeacherId);
    } else if (viewMode === "room") {
      list = allLessons.filter((l) => String(l.room_id) === selectedRoomId);
    } else if (viewMode === "subject") {
      list = allLessons.filter((l) => String(l.subject_id) === selectedSubjectId);
    }

    // Text search filter
    const query = searchTerm.trim().toLowerCase();
    if (query) {
      list = list.filter((l) => {
        const sub = getSubjectName(l.subject_id).toLowerCase();
        const teach = getTeacherName(l.teacher_id).toLowerCase();
        const cls = getClassName(l.class_id).toLowerCase();
        const rm = getRoomName(l.room_id).toLowerCase();
        return sub.includes(query) || teach.includes(query) || cls.includes(query) || rm.includes(query);
      });
    }

    return list;
  });

  // Calculate Weekdays from configured slots or lessons
  const weekdays = $derived.by(() => {
    const configuredDays = new Set<number>();

    for (const slot of timeSlots) {
      if (!slot.applicable_days) continue;
      for (const val of String(slot.applicable_days).split(",")) {
        const d = Number(val.trim());
        if (Number.isInteger(d) && d >= 1 && d <= 7) configuredDays.add(d);
      }
    }

    for (const l of allLessons) {
      const d = Number(l.day_of_week);
      if (Number.isInteger(d) && d >= 1 && d <= 7) configuredDays.add(d);
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

  // Draft solution lessons from solver job
  const draftLessons = $derived.by(() => {
    if (!previewDraft || !activeSolveJob?.best_solution) return [];
    try {
      const sol = typeof activeSolveJob.best_solution === "string"
        ? JSON.parse(activeSolveJob.best_solution)
        : activeSolveJob.best_solution;
      const assignments = sol.assignments || [];
      return assignments.map((a: any) => ({
        ...a,
        isDraft: true,
      }));
    } catch {
      return [];
    }
  });

  // Name Resolution Helpers
  function getSubjectName(subjectId: string): string {
    const sub = subjects.find((s) => String(s.id) === String(subjectId));
    return sub ? (isArabic ? sub.name_ar || sub.name : sub.name || sub.name_ar) : (isArabic ? "مادة غير معروفة" : "Subject");
  }

  function getTeacherName(teacherId: string): string {
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
    return "var(--primary)";
  }

  // Workload summary for Teacher view
  const teacherWorkload = $derived.by(() => {
    if (viewMode !== "teacher" || !selectedTeacherId) return null;
    const teacherLessons = allLessons.filter((l) => String(l.teacher_id) === selectedTeacherId);
    const nonBreakSlotsCount = timeSlots.filter((s) => !s.is_break).length;
    const totalWeeklySlots = nonBreakSlotsCount * weekdays.length;
    const teachingCount = teacherLessons.length;
    const freeCount = Math.max(0, totalWeeklySlots - teachingCount);
    const assignedRooms = new Set(teacherLessons.map((l) => l.room_id).filter(Boolean)).size;
    return { teachingCount, freeCount, assignedRooms, totalWeeklySlots };
  });

  // Occupancy summary for Room view
  const roomOccupancy = $derived.by(() => {
    if (viewMode !== "room" || !selectedRoomId) return null;
    const roomLessons = allLessons.filter((l) => String(l.room_id) === selectedRoomId);
    const nonBreakSlotsCount = timeSlots.filter((s) => !s.is_break).length;
    const totalWeeklySlots = nonBreakSlotsCount * weekdays.length;
    const occupancyRate = totalWeeklySlots > 0 ? Math.round((roomLessons.length / totalWeeklySlots) * 100) : 0;
    const rm = rooms.find((r) => String(r.id) === selectedRoomId);
    return { occupiedCount: roomLessons.length, totalSlots: totalWeeklySlots, occupancyRate, capacity: rm?.capacity || null };
  });

  // Action Handlers
  function refetchSchedule() {
    queryClient.invalidateQueries({ queryKey: ["period-schedule", data.orgId, activePeriodId] });
    queryClient.invalidateQueries({ queryKey: ["time-slots", data.orgId, activePeriodId] });
  }

  function openNewLessonSheet(day?: number, slotId?: string) {
    targetLesson = null;
    activeCellDay = day || 1;
    activeCellSlotId = slotId || (timeSlots[0]?.id ? String(timeSlots[0].id) : "");
    isLessonSheetOpen = true;
  }

  function openEditLessonSheet(lesson: any) {
    targetLesson = lesson;
    activeCellDay = Number(lesson?.day_of_week) || 1;
    activeCellSlotId = String(lesson?.time_slot_id || "");
    isLessonSheetOpen = true;
  }

  async function handleMoveLesson(lesson: any, targetDay: number, targetSlotId: string) {
    try {
      const conflicts = await commands.scheduling.checkConflicts(
        activePeriodId,
        lesson.class_id,
        lesson.teacher_id,
        targetSlotId,
        targetDay,
        lesson.room_id || null
      );

      const actualConflicts = conflicts.filter((c) => c.lesson_id !== lesson.id);
      if (actualConflicts.length > 0) {
        toast.error(isArabic ? "تعذر نقل الحصة بسبب تعارض في الموعد" : "Schedule conflict detected", {
          description: actualConflicts.map((c) => c.message).join("\n"),
        });
        return;
      }

      await commands.scheduling.updateLesson(lesson.id, {
        day_of_week: targetDay,
        time_slot_id: targetSlotId,
      } as any);

      toast.success(isArabic ? "تم نقل الحصة بنجاح" : "Lesson moved successfully");
      refetchSchedule();
    } catch (err) {
      toast.error(extractErrorMessage(err));
    }
  }

  async function handleDeleteLesson(lessonId: string) {
    try {
      await commands.scheduling.deleteLesson(lessonId);
      toast.success(isArabic ? "تم حذف الحصة من الجدول" : "Lesson removed from schedule");
      refetchSchedule();
    } catch (err) {
      toast.error(extractErrorMessage(err));
    }
  }

  const isLoading = $derived(
    periodsQuery.isLoading ||
    classesQuery.isLoading ||
    subjectsQuery.isLoading ||
    usersQuery.isLoading ||
    roomsQuery.isLoading ||
    timeSlotsQuery.isLoading ||
    periodScheduleQuery.isLoading
  );

  const error = $derived(
    periodsQuery.error ||
    classesQuery.error ||
    timeSlotsQuery.error ||
    periodScheduleQuery.error
  );
</script>

<div class="h-full w-full flex flex-col items-stretch overflow-hidden">
  <!-- Top Horizontal Command & Filter Toolbar -->
  <ScheduleToolbar
    bind:activePeriodId
    {periods}
    bind:viewMode
    bind:selectedClassId
    bind:selectedTeacherId
    bind:selectedRoomId
    bind:selectedSubjectId
    bind:searchTerm
    bind:compactView
    {classes}
    {teachers}
    {rooms}
    {subjects}
    onPeriodChange={refetchSchedule}
    onAutoScheduleClick={() => goto(localizeHref(`/${data.orgId}/academics/schedule/generator?periodId=${activePeriodId}`))}
    onTimeSlotsClick={() => (isTimeSlotsSheetOpen = true)}
    onExportClick={() => (isExportDialogOpen = true)}
    onNewLessonClick={() => openNewLessonSheet()}
  />

  <!-- Main Body: Full-Width Workspace Canvas -->
  <div class="flex flex-1 min-h-0 min-w-0 w-full overflow-hidden">
    <!-- Central Schedule Workspace Canvas -->
    <main class="flex flex-1 min-h-0 min-w-0 flex-col p-3 gap-2 overflow-hidden">
      {#if isLoading}
        <div class="flex-1 w-full flex flex-col bg-card border border-border/60 rounded-xl overflow-hidden p-4 gap-3">
          <Skeleton class="h-10 w-full" />
          <Skeleton class="flex-1 w-full" />
        </div>
      {:else if error}
        <ErrorState
          title={isArabic ? "تعذر تحميل الجدول الدراسي" : "Failed to load schedule"}
          message={error instanceof Error ? error.message : "Error"}
          onRetry={refetchSchedule}
        />
      {:else if timeSlots.length === 0}
        <div class="flex-1 flex flex-col items-center justify-center p-12 border border-dashed border-border/60 bg-card rounded-2xl text-center gap-4 max-w-md mx-auto my-auto">
          <div class="p-3.5 bg-primary/10 rounded-full text-primary">
            <Clock size={28} />
          </div>
          <div class="flex flex-col gap-1">
            <h3 class="font-bold text-sm text-foreground">
              {isArabic ? "لم يتم تعريف حصص دراسية" : "No Time Slots Configured"}
            </h3>
            <p class="text-xs text-muted-foreground leading-relaxed">
              {isArabic
                ? "يرجى إضافة الحصص وأوقات الأجراس لهذه الفترة الأكاديمية لبدء جدولة الدروس."
                : "Add instructional periods and bells to begin scheduling classes."}
            </p>
          </div>
          <Button size="sm" onclick={() => (isTimeSlotsSheetOpen = true)} class="gap-1.5 text-xs font-semibold">
            <Clock size={13} />
            <span>{isArabic ? "إعداد جدول الحصص الآن" : "Configure Time Slots"}</span>
          </Button>
        </div>
      {:else}
        <!-- Contextual Header Info Banner (For Teacher Workload / Room Occupancy) -->
        {#if teacherWorkload}
          <div class="flex items-center justify-between px-3 py-2 rounded-lg border border-blue-500/20 bg-blue-500/5 text-xs">
            <div class="flex items-center gap-2">
              <User size={14} class="text-blue-500" />
              <span class="font-bold text-foreground">{getTeacherName(selectedTeacherId)}</span>
            </div>
            <div class="flex items-center gap-4 text-[11px] text-muted-foreground">
              <span>{isArabic ? "الحصص الأسبوعية" : "Teaching Load"}: <strong class="text-foreground">{teacherWorkload.teachingCount}</strong></span>
              <span>{isArabic ? "الحصص الشاغرة" : "Free Periods"}: <strong class="text-emerald-600 dark:text-emerald-400">{teacherWorkload.freeCount}</strong></span>
              <span>{isArabic ? "القاعات المعينة" : "Rooms"}: <strong class="text-foreground">{teacherWorkload.assignedRooms}</strong></span>
            </div>
          </div>
        {:else if roomOccupancy}
          <div class="flex items-center justify-between px-3 py-2 rounded-lg border border-emerald-500/20 bg-emerald-500/5 text-xs">
            <div class="flex items-center gap-2">
              <MapPin size={14} class="text-emerald-500" />
              <span class="font-bold text-foreground">{getRoomName(selectedRoomId)}</span>
              {#if roomOccupancy.capacity}
                <Badge variant="outline" class="h-4 text-[9px]">{isArabic ? `السعة: ${roomOccupancy.capacity}` : `Capacity: ${roomOccupancy.capacity}`}</Badge>
              {/if}
            </div>
            <div class="flex items-center gap-4 text-[11px] text-muted-foreground">
              <span>{isArabic ? "الحصص المشغولة" : "Occupied Periods"}: <strong class="text-foreground">{roomOccupancy.occupiedCount}</strong></span>
              <span>{isArabic ? "نسبة الإشغال" : "Occupancy Rate"}: <strong class="text-primary">{roomOccupancy.occupancyRate}%</strong></span>
            </div>
          </div>
        {/if}

        <!-- Scheduling Grid View / Matrix View -->
        {#if viewMode === "matrix"}
          <ScheduleMatrix
            {timeSlots}
            {classes}
            selectedDay={selectedMatrixDay}
            lessons={allLessons}
            {getSubjectName}
            {getTeacherName}
            {getClassName}
            {getRoomName}
            {getSubjectColor}
            onLessonClick={openEditLessonSheet}
            onCellClick={(clsId, slotId, day) => {
              selectedClassId = clsId;
              openNewLessonSheet(day, slotId);
            }}
          />
        {:else}
          <TimetableGrid
            {timeSlots}
            {weekdays}
            lessons={filteredLessons}
            {conflictLessonIds}
            {draftLessons}
            {previewDraft}
            {viewMode}
            compact={compactView}
            locale={getLocale()}
            {getSubjectName}
            {getTeacherName}
            {getClassName}
            {getRoomName}
            {getSubjectColor}
            onCellClick={(day, slotId) => openNewLessonSheet(day, slotId)}
            onEditLesson={openEditLessonSheet}
            onLessonClick={openEditLessonSheet}
            onDeleteLesson={handleDeleteLesson}
            onMoveLesson={handleMoveLesson}
          />
        {/if}
      {/if}
    </main>
  </div>
</div>

<!-- Lesson Create / Edit Side Sheet -->
<LessonSheet
  bind:open={isLessonSheetOpen}
  periodId={activePeriodId}
  classId={selectedClassId}
  dayOfWeek={activeCellDay}
  timeSlotId={activeCellSlotId}
  lesson={targetLesson}
  {classes}
  {subjects}
  {teachers}
  {rooms}
  {timeSlots}
  onSuccess={refetchSchedule}
/>

<!-- Time Slots Management Sheet -->
<TimeSlotSheet
  bind:open={isTimeSlotsSheetOpen}
  periodId={activePeriodId}
  {timeSlots}
  onSuccess={refetchSchedule}
/>

<!-- AI Scheduler Drawer -->
<SolverDrawer
  bind:open={isSolverDrawerOpen}
  periodId={activePeriodId}
  bind:previewDraft
  bind:activeJob={activeSolveJob}
  onApply={refetchSchedule}
/>

<!-- Schedule Export & Print Dialog -->
<ScheduleExportDialog
  bind:open={isExportDialogOpen}
  periodName={periods.find((p) => String(p.id) === String(activePeriodId))?.name_ar || periods.find((p) => String(p.id) === String(activePeriodId))?.name || ""}
  {viewMode}
  {selectedClassId}
  {selectedTeacherId}
  {selectedRoomId}
  {classes}
  {teachers}
  {rooms}
  {timeSlots}
  {weekdays}
  lessons={allLessons}
  {getSubjectName}
  {getTeacherName}
  {getClassName}
  {getRoomName}
/>
