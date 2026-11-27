<script lang="ts">
import { ClockRotateRight, Plus } from "@orbis/icons";
import BreakRow from "./break-row.svelte";
import CalendarHeader from "./calendar-header.svelte";
import SessionCard from "./session-card.svelte";
import TimeGutter from "./time-gutter.svelte";
import { enrichTimeSlots, formatTimeRange, type EnrichedTimeSlot } from "./utils.js";

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
  conflictLessonIds?: Set<string> | string[];
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
  onAttendanceClick?: (lesson: any) => void;
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
  conflictLessonIds: passedConflictLessonIds,
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
  onAttendanceClick,
  onMoveLesson,
}: Props = $props();

const isArabic = $derived(locale === "ar");
let gridElement = $state<HTMLDivElement>();
let isNarrow = $state(false);
let activeAgendaDay = $state<number | null>(null);
let dragOverCellKey = $state<string | null>(null);

$effect(() => {
  if (!gridElement) return;

  const updateLayout = () => {
    const rootFontSize =
      Number.parseFloat(getComputedStyle(document.documentElement).fontSize) ||
      16;
    isNarrow = gridElement!.clientWidth < 700 * (rootFontSize / 16);
  };

  updateLayout();
  const observer = new ResizeObserver(updateLayout);
  observer.observe(gridElement);

  return () => observer.disconnect();
});

$effect(() => {
  if (weekdays.length === 0) {
    activeAgendaDay = null;
    return;
  }

  const availableDays = weekdays.map((day, idx) =>
    Number(day.value ?? day.day ?? idx + 1),
  );
  if (activeAgendaDay === null || !availableDays.includes(activeAgendaDay)) {
    const todayDay = weekdays.find((d) => d.isToday);
    activeAgendaDay =
      (todayDay
        ? Number(todayDay.value ?? todayDay.day ?? availableDays[0])
        : availableDays[0]) ?? null;
  }
});

const enrichedSlots = $derived<EnrichedTimeSlot[]>(
  timeSlots.length > 0 && timeSlots[0]?.display_name !== undefined
    ? timeSlots
    : enrichTimeSlots(timeSlots, locale),
);

const slotIndexes = $derived(
  [
    ...new Set(
      enrichedSlots.map((slot) => Number(slot.slot_index ?? slot.slot_number)),
    ),
  ]
    .filter((slotIndex) => Number.isInteger(slotIndex) && slotIndex > 0)
    .sort((left, right) => left - right),
);

const slotsByDayAndIndex = $derived.by(() => {
  const slots = new Map<string, EnrichedTimeSlot>();
  for (const slot of enrichedSlots) {
    slots.set(
      `${Number(slot.day_of_week)}:${Number(slot.slot_index ?? slot.slot_number)}`,
      slot,
    );
  }
  return slots;
});

const scheduleRows = $derived.by(() => {
  const rows: {
    slotIndex: number;
    rowSlot: EnrichedTimeSlot;
    isBreakRow: boolean;
  }[] = [];

  for (const slotIndex of slotIndexes) {
    let rowSlot: EnrichedTimeSlot | undefined;
    let allBreaks = true;
    let hasAnySlot = false;

    for (let idx = 0; idx < weekdays.length; idx++) {
      const day = weekdays[idx];
      const dayValue = Number(day.value ?? day.day ?? idx + 1);
      const s = slotsByDayAndIndex.get(`${dayValue}:${slotIndex}`);
      if (s) {
        hasAnySlot = true;
        if (!rowSlot) rowSlot = s;
        if (!s.is_break) allBreaks = false;
      }
    }

    if (hasAnySlot && rowSlot) {
      rows.push({
        slotIndex,
        rowSlot,
        isBreakRow: allBreaks && Boolean(rowSlot.is_break),
      });
    }
  }
  return rows;
});

const gridRowsTemplate = $derived(
  [
    "auto",
    ...scheduleRows.map((r) =>
      r.isBreakRow
        ? "auto"
        : compact
          ? "minmax(4.5rem, 1fr)"
          : "minmax(5.75rem, 1fr)",
    ),
  ].join(" "),
);

const lessonMatrix = $derived.by(() => {
  const map = new Map<string, any[]>();
  const all =
    previewDraft && draftLessons.length > 0
      ? [...lessons, ...draftLessons]
      : lessons;

  for (const l of all) {
    const day = Number(l.day_of_week);
    const slotId = String(l.time_slot_id);
    const key = `${day}:${slotId}`;
    let bucket = map.get(key);
    if (!bucket) {
      bucket = [];
      map.set(key, bucket);
    }
    bucket.push(l);
  }
  return map;
});

const lessonsByDay = $derived.by(() => {
  const counts: Record<number, number> = {};
  for (const l of lessons) {
    const day = Number(l.day_of_week);
    counts[day] = (counts[day] || 0) + 1;
  }
  return counts;
});

const conflictLessonIds = $derived.by(() => {
  const set = new Set<string>();
  if (passedConflictLessonIds) {
    for (const id of passedConflictLessonIds) {
      set.add(String(id));
    }
  }
  for (const c of conflicts) {
    if (c.lesson_id) set.add(String(c.lesson_id));
    if (c.conflicting_lesson_id) set.add(String(c.conflicting_lesson_id));
  }
  return set;
});

function handleDragOver(e: DragEvent, day: number, slotId: string) {
  e.preventDefault();
  e.stopPropagation();
  const nextKey = `${day}:${slotId}`;
  if (dragOverCellKey !== nextKey) dragOverCellKey = nextKey;
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
  bind:this={gridElement}
  class="relative flex min-h-0 w-full flex-1 flex-col overflow-hidden rounded-md border border-border bg-card {className}"
>
  {#if isNarrow}
    <!-- Mobile / Narrow Agenda View -->
    <div class="flex min-h-0 min-w-0 flex-1 flex-col overflow-hidden">
      <div
        class="flex shrink-0 gap-1.5 overflow-x-auto border-b border-border bg-muted/20 p-2"
      >
        {#each weekdays as day, idx (day.value ?? day.day ?? idx)}
          {@const dayValue = Number(day.value ?? day.day ?? idx + 1)}
          {@const count = lessonsByDay[dayValue] ?? 0}
          <button
            type="button"
            class="flex shrink-0 items-center gap-1.5 rounded-md border px-3 py-1.5 text-xs font-semibold transition-colors {activeAgendaDay === dayValue
              ? 'border-primary bg-primary text-primary-foreground'
              : 'border-border bg-card text-muted-foreground hover:text-foreground'}"
            aria-pressed={activeAgendaDay === dayValue}
            onclick={() => (activeAgendaDay = dayValue)}
          >
            {#if day.isToday}
              <span
                class="h-1.5 w-1.5 rounded-full {activeAgendaDay === dayValue
                  ? 'bg-primary-foreground'
                  : 'bg-primary'}"
              ></span>
            {/if}
            <span>{day.label}</span>
            {#if count > 0}
              <span
                class="rounded-sm px-1.5 py-0.5 font-mono text-[10px] leading-none tabular-nums {activeAgendaDay === dayValue
                  ? 'bg-primary-foreground/20 text-primary-foreground'
                  : 'bg-muted text-muted-foreground'}"
              >
                {count}
              </span>
            {/if}
          </button>
        {/each}
      </div>

      <div class="min-h-0 flex-1 overflow-y-auto p-2.5 sm:p-3">
        {#each slotIndexes as slotIndex (slotIndex)}
          {@const slot = activeAgendaDay
            ? slotsByDayAndIndex.get(`${activeAgendaDay}:${slotIndex}`)
            : undefined}
          {#if slot}
            {@const key = `${activeAgendaDay}:${slot.id}`}
            {@const cellLessons = lessonMatrix.get(key) || []}
            <section
              class="mb-2 grid grid-cols-[5.5rem_minmax(0,1fr)] gap-2 last:mb-0 sm:grid-cols-[7rem_minmax(0,1fr)]"
            >
              <div
                class="flex flex-col justify-center rounded-md border border-border/70 bg-muted/20 px-2 py-2.5 text-center"
              >
                <div
                  class="text-xs font-semibold {slot.is_break
                    ? 'text-primary'
                    : 'text-foreground'}"
                >
                  {slot.is_break
                    ? slot.name_ar || slot.name || (isArabic ? "استراحة" : "Break")
                    : slot.name_ar ||
                      slot.name ||
                      slot.display_name ||
                      (isArabic ? `الحصة ${slotIndex}` : `Period ${slotIndex}`)}
                </div>
                {#if slot.start_time && slot.end_time}
                  <div
                    class="mt-1 font-mono text-[10px] text-muted-foreground tabular-nums"
                  >
                    {formatTimeRange(slot.start_time, slot.end_time, isArabic)}
                  </div>
                {/if}
              </div>

              {#if slot.is_break}
                <div
                  class="flex min-h-12 items-center justify-center rounded-md border border-dashed border-border bg-muted/20 text-xs font-medium text-muted-foreground"
                >
                  {slot.name_ar || slot.name || (isArabic ? "استراحة" : "Break")}
                </div>
              {:else if cellLessons.length > 0}
                <div class="flex min-h-14 flex-col gap-1.5">
                  {#each cellLessons as lesson (lesson.id)}
                    <SessionCard
                      {lesson}
                      timeSlot={slot}
                      {viewMode}
                      {locale}
                      compact={false}
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
                      onAttendance={onAttendanceClick}
                    />
                  {/each}
                </div>
              {:else}
                <button
                  type="button"
                  class="flex min-h-14 w-full items-center justify-center rounded-md border border-dashed border-border bg-card text-muted-foreground transition-colors hover:border-primary/50 hover:bg-primary/5 hover:text-primary focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
                  onclick={() =>
                    onCellClick?.(activeAgendaDay ?? 0, String(slot.id))}
                  aria-label={isArabic ? "إضافة حصة" : "Add lesson"}
                >
                  <Plus size={16} />
                </button>
              {/if}
            </section>
          {/if}
        {/each}
      </div>
    </div>
  {:else}
    <!-- Desktop Unified CSS Grid -->
    <div class="min-h-0 min-w-0 flex-1 overflow-auto">
      <div
        role="grid"
        class="grid min-h-full min-w-full"
        style:grid-template-columns="{compact ? '5rem' : '7rem'} repeat({Math.max(1, weekdays.length)}, minmax({compact ? '104px' : '136px'}, 1fr))"
        style:grid-template-rows={gridRowsTemplate}
      >
        <!-- Top-Start Corner Cell (Sticky top + start) -->
        <div
          class="sticky top-0 inset-s-0 z-30 flex h-11 items-center justify-center border-b border-e border-border bg-card text-muted-foreground"
        >
          <ClockRotateRight size={14} />
        </div>

        <!-- Sticky Weekday Headers -->
        <CalendarHeader
          {weekdays}
          activeDay={activeAgendaDay}
          {lessonsByDay}
          {locale}
          onDaySelect={(d) => (activeAgendaDay = d)}
        />

        <!-- Period & Break Rows -->
        {#each scheduleRows as row (row.slotIndex)}
          <TimeGutter slot={row.rowSlot} {locale} {compact} />

          {#if row.isBreakRow}
            <BreakRow
              slot={row.rowSlot}
              weekdaysCount={weekdays.length}
              {locale}
            />
          {:else}
            {#each weekdays as day, idx (day.value ?? day.day ?? idx)}
              {@const dayValue = Number(day.value ?? day.day ?? idx + 1)}
              {@const slot = slotsByDayAndIndex.get(
                `${dayValue}:${row.slotIndex}`,
              )}
              {@const key = slot
                ? `${dayValue}:${slot.id}`
                : `${dayValue}:${row.slotIndex}`}
              {@const cellLessons = slot ? lessonMatrix.get(key) || [] : []}
              {@const isOver = dragOverCellKey === key}
              <div
                role="gridcell"
                tabindex="-1"
                class="group relative grid min-w-0 auto-rows-fr gap-1 border-b border-e border-border/60 p-1.5 transition-colors last:border-e-0 {cellLessons.length > 1
                  ? 'grid-cols-[repeat(auto-fit,minmax(108px,1fr))]'
                  : 'grid-cols-1'} {isOver
                  ? 'bg-primary/10 ring-1 ring-inset ring-primary'
                  : day.isToday
                    ? 'bg-primary/[0.025] hover:bg-primary/[0.05]'
                    : 'hover:bg-muted/20'} {slot?.is_break ? 'bg-muted/20' : ''}"
                ondragover={slot && !slot.is_break
                  ? (e) => handleDragOver(e, dayValue, String(slot.id))
                  : undefined}
                ondragleave={slot && !slot.is_break ? handleDragLeave : undefined}
                ondrop={slot && !slot.is_break
                  ? (e) => handleDrop(e, dayValue, String(slot.id))
                  : undefined}
              >
                {#if !slot}
                  <div
                    class="flex flex-1 items-center justify-center text-xs text-muted-foreground/40"
                  >
                    —
                  </div>
                {:else if slot.is_break}
                  <div
                    class="flex flex-1 items-center justify-center rounded-sm text-xs font-medium text-muted-foreground"
                  >
                    {slot.name_ar ||
                      slot.name ||
                      (isArabic ? "استراحة" : "Break")}
                  </div>
                {:else if cellLessons.length > 0}
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
                      onAttendance={onAttendanceClick}
                    />
                  {/each}
                {:else if onCellClick}
                  <button
                    type="button"
                    class="flex flex-1 w-full items-center justify-center rounded-md border border-dashed border-transparent text-muted-foreground/70 opacity-0 transition-all hover:border-primary/40 hover:bg-primary/5 hover:text-primary group-hover:opacity-100 focus-visible:opacity-100 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
                    onclick={() => onCellClick(dayValue, String(slot.id))}
                    aria-label={isArabic ? "إضافة حصة" : "Add lesson"}
                    title={isArabic ? "إضافة حصة" : "Add lesson"}
                  >
                    <Plus size={16} />
                  </button>
                {/if}
              </div>
            {/each}
          {/if}
        {/each}
      </div>
    </div>
  {/if}
</div>
