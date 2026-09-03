<script lang="ts">
  import { Clock, Plus } from "@lucide/svelte";
  import SessionCard from "./session-card.svelte";
  import TimeGutter from "./time-gutter.svelte";
  import CalendarHeader from "./calendar-header.svelte";
  import BreakRow from "./break-row.svelte";

  interface Props {
    timeSlots: any[];
    weekdays: any[];
    lessons?: any[];
    draftLessons?: any[];
    previewDraft?: boolean;
    viewMode?: "class" | "teacher" | "room" | "subject" | "matrix";
    locale?: string;
    compact?: boolean;
    conflicts?: any[];
    class?: string;
    getSubjectName?: (id: string) => string;
    getTeacherName?: (id: string) => string;
    getClassName?: (id: string) => string;
    getRoomName?: (id: string) => string;
    getSubjectColor?: (id: string) => string;
    onCellClick?: (day: number, slotId: string) => void;
    onDeleteLesson?: (id: string) => void;
    onEditLesson?: (lesson: any) => void;
    onLessonClick?: (lesson: any) => void;
    onMoveLesson?: (lesson: any, targetDay: number, targetSlotId: string) => void;
  }

  let {
    timeSlots = [],
    weekdays = [],
    lessons = [],
    draftLessons = [],
    previewDraft = false,
    viewMode = "class",
    locale = "ar",
    compact = false,
    conflicts = [],
    class: className = "",
    getSubjectName = (id) => id,
    getTeacherName = (id) => id,
    getClassName = (id) => id,
    getRoomName = (id) => id,
    getSubjectColor = () => "var(--primary)",
    onCellClick,
    onDeleteLesson,
    onEditLesson,
    onLessonClick,
    onMoveLesson,
  }: Props = $props();

  const isArabic = $derived(locale === "ar");

  // Build quick map of lessons by day and slot
  const lessonMatrix = $derived.by(() => {
    const map = new Map<string, any[]>();
    const all = previewDraft && draftLessons.length > 0 ? [...lessons, ...draftLessons] : lessons;

    for (const l of all) {
      const day = Number(l.day_of_week);
      const slotId = String(l.time_slot_id);
      const key = `${day}:${slotId}`;
      if (!map.has(key)) {
        map.set(key, []);
      }
      map.get(key)!.push(l);
    }
    return map;
  });

  // Calculate count of lessons per day for the header
  const lessonsByDay = $derived.by(() => {
    const counts: Record<number, number> = {};
    for (const l of lessons) {
      const day = Number(l.day_of_week);
      counts[day] = (counts[day] || 0) + 1;
    }
    return counts;
  });

  // Conflict lesson IDs set
  const conflictLessonIds = $derived.by(() => {
    const set = new Set<string>();
    for (const c of conflicts) {
      if (c.lesson_id) set.add(String(c.lesson_id));
      if (c.conflicting_lesson_id) set.add(String(c.conflicting_lesson_id));
    }
    return set;
  });

  let dragOverCellKey = $state<string | null>(null);

  function handleDragOver(e: DragEvent, day: number, slotId: string) {
    e.preventDefault();
    e.stopPropagation();
    dragOverCellKey = `${day}:${slotId}`;
  }

  function handleDragLeave(e: DragEvent) {
    e.preventDefault();
    dragOverCellKey = null;
  }

  function handleDrop(e: DragEvent, day: number, slotId: string) {
    e.preventDefault();
    dragOverCellKey = null;
    try {
      const raw = e.dataTransfer?.getData("text/plain");
      if (!raw) return;
      const data = JSON.parse(raw);
      if (data?.lesson) {
        onMoveLesson?.(data.lesson, day, slotId);
      }
    } catch (err) {
      console.error("Drop error:", err);
    }
  }
</script>

<div
  class="relative flex flex-1 min-h-0 w-full flex-col overflow-hidden rounded-xl border border-border/60 bg-card shadow-xs {className}"
>
  <!-- Main Grid Viewport -->
  <div class="flex-1 min-h-0 min-w-0 flex flex-col overflow-auto scrollbar-hide">
    <!-- Header Row (Sticky top) -->
    <div class="sticky top-0 z-20 flex min-w-[700px] border-b border-border/60 bg-card shadow-2xs">
      <div class="h-10 shrink-0 border-e border-border/60 bg-muted/20 flex items-center justify-center {compact ? 'w-20' : 'w-28'} text-muted-foreground/60">
        <Clock size={13} />
      </div>
      <CalendarHeader {weekdays} {lessonsByDay} {locale} />
    </div>

    <!-- Body: Time Gutter + Days Columns -->
    <div class="flex min-w-[700px] flex-1">
      <!-- Time Gutter Axis -->
      <TimeGutter {timeSlots} {locale} {compact} />

      <!-- Day Columns Grid Container -->
      <div class="flex flex-1 flex-col min-w-0">
        {#each timeSlots as slot (slot.id)}
          {@const isBreak = Boolean(slot.is_break)}
          {#if isBreak}
            <!-- Break row spanning all days -->
            <BreakRow {slot} weekdaysCount={weekdays.length} {locale} />
          {:else}
            <!-- Lesson Row for this TimeSlot across Weekdays -->
            <div class="flex flex-1 border-b border-border/40 last:border-b-0 min-h-[76px]">
              {#each weekdays as day (day.value)}
                {@const key = `${day.value}:${slot.id}`}
                {@const cellLessons = lessonMatrix.get(key) || []}
                {@const isOver = dragOverCellKey === key}

                <!-- Cell Drop Target -->
                <!-- svelte-ignore a11y_no_static_element_interactions -->
                <div
                  ondragover={(e) => handleDragOver(e, day.value, String(slot.id))}
                  ondragleave={handleDragLeave}
                  ondrop={(e) => handleDrop(e, day.value, String(slot.id))}
                  class="group relative flex-1 min-w-0 border-e border-border/40 p-1 text-center transition-colors last:border-e-0 flex flex-col gap-1 {isOver
                    ? 'bg-primary/10 ring-2 ring-primary ring-inset'
                    : 'hover:bg-muted/20'}"
                >
                  {#if cellLessons.length > 0}
                    <!-- Session Cards rendered in this cell -->
                    {#each cellLessons as lesson (lesson.id)}
                      <SessionCard
                        {lesson}
                        timeSlot={slot}
                        {viewMode}
                        {locale}
                        {compact}
                        hasConflict={conflictLessonIds.has(String(lesson.id))}
                        isDraft={Boolean(lesson.isDraft)}
                        subjectColor={getSubjectColor(lesson.subject_id)}
                        {getSubjectName}
                        {getTeacherName}
                        {getClassName}
                        {getRoomName}
                        onDelete={onDeleteLesson}
                        onEdit={onEditLesson}
                        onClick={onLessonClick}
                      />
                    {/each}
                  {:else}
                    <button
                      type="button"
                      class="min-h-16 flex-1 w-full flex items-center justify-center rounded-md text-primary opacity-0 transition-opacity hover:bg-primary/10 group-hover:opacity-100 focus-visible:opacity-100 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-primary"
                      onclick={() => onCellClick?.(day.value, String(slot.id))}
                      aria-label={isArabic ? "إضافة حصة" : "Add lesson"}
                      title={isArabic ? "إضافة حصة" : "Add lesson"}
                    >
                      <Plus size={18} />
                    </button>
                  {/if}
                </div>
              {/each}
            </div>
          {/if}
        {/each}
      </div>
    </div>
  </div>
</div>

<style>
  .scrollbar-hide {
    -ms-overflow-style: none;
    scrollbar-width: none;
  }
  .scrollbar-hide::-webkit-scrollbar {
    display: none;
  }
</style>
