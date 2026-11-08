<script lang="ts">
import { Calendar, Pin, User, Users } from "@orbis/icons";
import * as m from "$lib/paraglide/messages";
import { getWeekdayOptions, l } from "$lib/core/i18n";
import { getLocale } from "$lib/paraglide/runtime";
import { enrichTimeSlots, type EnrichedTimeSlot } from "@orbis/ui";

interface Props {
  timeSlots: any[];
  classes: any[];
  selectedDay: number;
  lessons: any[];
  getSubjectName: (id: string) => string;
  getTeacherName: (id: string) => string;
  getClassName: (id: string) => string;
  getRoomName: (id: string) => string;
  getSubjectColor: (id: string) => string;
  onLessonClick?: (lesson: any) => void;
  onCellClick?: (classId: string, slotId: string, day: number) => void;
  onDayChange?: (day: number) => void;
}

let {
  timeSlots = [],
  classes = [],
  selectedDay = 1,
  lessons = [],
  getSubjectName,
  getTeacherName,
  getClassName,
  getRoomName,
  getSubjectColor,
  onLessonClick,
  onCellClick,
  onDayChange,
}: Props = $props();

const weekdayOptions = getWeekdayOptions();
const weekdayOrder = new Map(
  weekdayOptions.map((weekday, index) => [weekday.value, index]),
);
const availableDays = $derived(
  [...new Set(timeSlots.map((slot) => Number(slot.day_of_week)))]
    .filter((day) => Number.isInteger(day) && day >= 1 && day <= 7)
    .sort(
      (left, right) =>
        (weekdayOrder.get(left) ?? 0) - (weekdayOrder.get(right) ?? 0),
    ),
);
const weekdayLabelByValue = new Map(
  weekdayOptions.map((weekday) => [weekday.value, weekday.label]),
);
const enrichedSlots = $derived<EnrichedTimeSlot[]>(
  timeSlots.length > 0 && timeSlots[0]?.display_name !== undefined
    ? timeSlots.filter((slot) => Number(slot.day_of_week) === selectedDay)
    : enrichTimeSlots(
        timeSlots.filter((slot) => Number(slot.day_of_week) === selectedDay),
        getLocale(),
      ),
);

// Day Lessons
const dayLessons = $derived(
  lessons.filter((item) => Number(item.day_of_week) === selectedDay),
);

function getLessonForClassSlot(classId: string, slotId: string) {
  return dayLessons.find(
    (item) =>
      String(item.section_id ?? item.class_id) === String(classId) &&
      String(item.time_slot_id) === String(slotId),
  );
}
</script>

<div class="flex flex-1 min-h-0 w-full flex-col overflow-hidden rounded-xl border border-border/60 bg-card shadow-xs">
  <div class="flex shrink-0 items-center gap-2 overflow-x-auto border-b border-border/60 bg-muted/15 px-3 py-2">
    <Calendar size={14} class="shrink-0 text-muted-foreground" />
    {#each availableDays as day (day)}
      <button
        type="button"
        aria-pressed={selectedDay === day}
        class="shrink-0 rounded-md px-3 py-1.5 text-xs font-medium transition-colors {selectedDay === day
          ? 'bg-primary text-primary-foreground shadow-xs'
          : 'text-muted-foreground hover:bg-muted hover:text-foreground'}"
        onclick={() => onDayChange?.(day)}
      >
        {weekdayLabelByValue.get(day)}
      </button>
    {/each}
  </div>
  <div class="flex-1 min-h-0 min-w-0 flex flex-col overflow-auto scrollbar-hide">
    <!-- Matrix Grid -->
    <table class="w-full border-collapse text-xs select-none">
      <thead class="sticky top-0 z-20 bg-card shadow-2xs">
        <tr class="border-b border-border/60 bg-muted/20">
          <th class="p-2 border-e border-border/60 text-start w-32 font-bold text-muted-foreground">
            {m.common_table_class()}
          </th>
          {#each enrichedSlots as slot (slot.id)}
            {@const isBreak = Boolean(slot.is_break)}
            <th class="p-2 border-e border-border/40 text-start min-w-30 last:border-e-0 {isBreak ? 'bg-amber-500/10' : ''}">
              <div class="font-bold text-foreground text-[11px]">
                {slot.display_name}
              </div>
              <div class="text-[9px] text-muted-foreground font-mono">
                {slot.start_time?.slice(0, 5)} - {slot.end_time?.slice(0, 5)}
              </div>
            </th>
          {/each}
        </tr>
      </thead>
      <tbody>
        {#each classes as cls (cls.id)}
          <tr class="border-b border-border/40 last:border-b-0 hover:bg-muted/10 transition-colors">
            <!-- Class Column Header -->
            <td class="p-2 border-e border-border/60 font-bold text-foreground bg-muted/10">
              <div class="flex items-center gap-1.5 truncate">
                <Users size={12} class="text-primary shrink-0" />
                <span class="truncate">{l(cls.name_ar, cls.name)}</span>
              </div>
            </td>

            <!-- Slot Cells for this class -->
            {#each enrichedSlots as slot (slot.id)}
              {@const isBreak = Boolean(slot.is_break)}
              {@const lessonItem = getLessonForClassSlot(cls.id, slot.id)}

              <td class="p-1 border-e border-border/40 last:border-e-0 align-top {isBreak ? 'bg-amber-500/5' : ''}">
                {#if isBreak}
                  <div class="h-full min-h-11 flex items-center justify-center text-[10px] font-semibold text-amber-600 dark:text-amber-400">
                    {slot.display_name}
                  </div>
                {:else if lessonItem}
                  <!-- svelte-ignore a11y_click_events_have_key_events -->
                  <!-- svelte-ignore a11y_no_static_element_interactions -->
                  <div
                    onclick={() => onLessonClick?.(lessonItem)}
                    style={`--subject-color: ${getSubjectColor(lessonItem.subject_id)};`}
                    class="h-full min-h-12 rounded-md border border-border/60 bg-card p-1.5 cursor-pointer hover:border-primary/50 transition-all text-start relative overflow-hidden"
                  >
                    <div
                      class="absolute inset-s-0 top-0 bottom-0 w-1"
                      style="background-color: var(--subject-color);"
                    ></div>
                    <div class="ps-1.5 flex flex-col justify-between h-full">
                      <span class="font-bold text-[11px] text-foreground truncate" title={getSubjectName(lessonItem.subject_id)}>
                        {getSubjectName(lessonItem.subject_id)}
                      </span>
                      <div class="flex items-center justify-between text-[9px] text-muted-foreground mt-1">
                        <span class="truncate">{getTeacherName(lessonItem.teacher_id)}</span>
                        {#if lessonItem.room_id}
                          <span class="text-primary font-medium">{getRoomName(lessonItem.room_id)}</span>
                        {/if}
                      </div>
                    </div>
                  </div>
                {:else}
                  <button
                    type="button"
                    class="min-h-12 w-full rounded-md text-primary opacity-0 transition-opacity hover:bg-primary/5 group-hover:opacity-100 focus-visible:opacity-100 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-primary"
                    aria-label={m.common_lesson_sheet_title()}
                    onclick={() => onCellClick?.(String(cls.id), String(slot.id), selectedDay)}
                  ></button>
                {/if}
              </td>
            {/each}
          </tr>
        {/each}
      </tbody>
    </table>
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
