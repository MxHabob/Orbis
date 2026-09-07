<script lang="ts">
  import { createQuery } from "@tanstack/svelte-query";
  import { api, } from "$lib/api";
  import { Skeleton } from "@orbis/ui/skeleton";
  import { TimetableGrid } from "@orbis/ui";
  import { Clock, Calendar, ExternalLink } from "@lucide/svelte";
  import { getLocale, localizeHref } from "$lib/paraglide/runtime";
  import WorkspaceCard from "./workspace-card.svelte";
  import { page } from "$app/state";

  interface Props {
    classId: string;
    periodId: string;
  }

  let { classId, periodId }: Props = $props();

  const orgId = $derived(String(page.params.orgId || ""));
  const isArabic = $derived(getLocale() === "ar");

  const scheduleQuery = createQuery(() => ({
    queryKey: ["class-schedule", classId, periodId],
    queryFn: () => api.listScheduledLessons(periodId || "", classId || ""),
    enabled: !!classId && !!periodId,
    staleTime: 2 * 60 * 1000,
  }));

  const timeSlotsQuery = createQuery(() => ({
    queryKey: ["time-slots", periodId],
    queryFn: () => api.listTimeSlots(),
    enabled: !!periodId,
    staleTime: 5 * 60 * 1000,
  }));

  const subjectsQuery = createQuery(() => ({
    queryKey: ["all-subjects"],
    queryFn: () => api.listSubjects(),
    staleTime: 10 * 60 * 1000,
  }));

  const usersQuery = createQuery(() => ({
    queryKey: ["all-users"],
    queryFn: () => api.listUsers(null),
    staleTime: 5 * 60 * 1000,
  }));

  const roomsQuery = createQuery(() => ({
    queryKey: ["all-rooms"],
    queryFn: () => api.listRooms(),
    staleTime: 10 * 60 * 1000,
  }));

  const lessons = $derived((scheduleQuery.data || []) as any[]);
  const allTimeSlots = $derived((timeSlotsQuery.data || []) as any[]);
  const subjects = $derived((subjectsQuery.data || []) as any[]);
  const users = $derived((usersQuery.data || []) as any[]);
  const rooms = $derived((roomsQuery.data || []) as any[]);

  const timeSlots = $derived(
    allTimeSlots.filter(
      (slot: any) => !slot.period_id || String(slot.period_id) === String(periodId)
    )
  );

  const weekdays = $derived.by(() => {
    const configuredDays = new Set<number>();

    for (const slot of timeSlots) {
      if (!slot.applicable_days) continue;
      for (const val of String(slot.applicable_days).split(",")) {
        const d = Number(val.trim());
        if (Number.isInteger(d) && d >= 1 && d <= 7) configuredDays.add(d);
      }
    }

    for (const l of lessons) {
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

  function getSubjectName(subjectId: string): string {
    const sub = subjects.find((s) => String(s.id) === String(subjectId));
    if (!sub) return isArabic ? "مادة غير معروفة" : "Unknown Subject";
    return isArabic ? sub.name_ar || sub.name : sub.name || sub.name_ar;
  }

  function getTeacherName(teacherId: string): string {
    const teacher = users.find((u) => String(u.id) === String(teacherId));
    if (!teacher) return "";
    return isArabic ? teacher.name_ar || teacher.full_name : teacher.full_name || teacher.name_ar;
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

  const isLoading = $derived(scheduleQuery.isLoading || timeSlotsQuery.isLoading);
</script>

<WorkspaceCard title={isArabic ? "الجدول الدراسي الأسبوعي" : "Weekly Class Timetable"}>
  {#snippet actions()}
    <a
      href={localizeHref(`/${orgId}/academics/schedule?classId=${classId}`)}
      class="flex items-center gap-1.5 text-xs text-primary hover:underline font-semibold"
    >
      <Calendar size={13} />
      <span>{isArabic ? "عرض وإدارة الجدول" : "Full Schedule"}</span>
      <ExternalLink size={11} class="opacity-70" />
    </a>
  {/snippet}

  {#if isLoading}
    <div class="flex flex-col gap-3 py-4">
      <Skeleton class="h-8 w-full" />
      <Skeleton class="h-48 w-full rounded-xl" />
    </div>
  {:else if lessons.length === 0 && timeSlots.length === 0}
    <div class="flex flex-col items-center justify-center p-8 text-center border border-dashed border-border/60 rounded-xl bg-muted/10 my-2">
      <div class="p-3 bg-primary/10 rounded-full text-primary mb-2">
        <Clock size={24} />
      </div>
      <p class="text-sm font-bold text-foreground">{isArabic ? "لا توجد حصص مجدولة" : "No Lessons Scheduled"}</p>
      <p class="text-[11px] text-muted-foreground mt-1 max-w-sm">
        {isArabic ? "لم يتم تعيين حصص دراسية لهذا الفصل في هذه الفترة الأكاديمية." : "No lesson timetable has been configured for this class in this period."}
      </p>
    </div>
  {:else}
    <!-- Compact, responsive viewport container -->
    <div class="w-full max-h-95 overflow-hidden rounded-md bg-card">
      <TimetableGrid
        {timeSlots}
        {weekdays}
        {lessons}
        viewMode="class"
        compact={true}
        locale={getLocale()}
        {getSubjectName}
        {getTeacherName}
        {getRoomName}
        {getSubjectColor}
      />
    </div>
  {/if}
</WorkspaceCard>
