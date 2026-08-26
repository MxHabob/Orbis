<script lang="ts">
  import { Users, User, MapPin } from "@lucide/svelte";
  import { getLocale } from "$lib/paraglide/runtime";

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
  }: Props = $props();

  const isArabic = $derived(getLocale() === "ar");

  // Day Lessons
  const dayLessons = $derived(
    lessons.filter((l) => Number(l.day_of_week) === selectedDay)
  );

  function getLessonForClassSlot(classId: string, slotId: string) {
    return dayLessons.find(
      (l) => String(l.class_id) === String(classId) && String(l.time_slot_id) === String(slotId)
    );
  }
</script>

<div class="flex flex-1 min-h-0 w-full flex-col overflow-hidden rounded-xl border border-border/60 bg-card shadow-xs">
  <div class="flex-1 min-h-0 min-w-0 flex flex-col overflow-auto scrollbar-hide">
    <!-- Matrix Table -->
    <table class="w-full border-collapse text-xs select-none">
      <thead class="sticky top-0 z-20 bg-card shadow-2xs">
        <tr class="border-b border-border/60 bg-muted/20">
          <th class="p-2 border-e border-border/60 text-start w-32 font-bold text-muted-foreground">
            {isArabic ? "الفصل الدراسي" : "Class"}
          </th>
          {#each timeSlots as slot (slot.id)}
            {@const isBreak = Boolean(slot.is_break)}
            <th class="p-2 border-e border-border/40 text-start min-w-[120px] last:border-e-0 {isBreak ? 'bg-amber-500/10' : ''}">
              <div class="font-bold text-foreground text-[11px]">
                {slot.name_ar || slot.name || `P${slot.slot_number}`}
              </div>
              <div class="text-[9px] text-muted-foreground font-mono" dir="ltr">
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
                <span class="truncate">{cls.name_ar || cls.name}</span>
              </div>
            </td>

            <!-- Slot Cells for this class -->
            {#each timeSlots as slot (slot.id)}
              {@const isBreak = Boolean(slot.is_break)}
              {@const l = getLessonForClassSlot(cls.id, slot.id)}

              <td class="p-1 border-e border-border/40 last:border-e-0 align-top {isBreak ? 'bg-amber-500/5' : ''}">
                {#if isBreak}
                  <div class="h-full min-h-[44px] flex items-center justify-center text-[10px] font-semibold text-amber-600 dark:text-amber-400">
                    {slot.name_ar || slot.name || (isArabic ? "استراحة" : "Break")}
                  </div>
                {:else if l}
                  <!-- svelte-ignore a11y_click_events_have_key_events -->
                  <!-- svelte-ignore a11y_no_static_element_interactions -->
                  <div
                    onclick={() => onLessonClick?.(l)}
                    style={`--subject-color: ${getSubjectColor(l.subject_id)};`}
                    class="h-full min-h-[48px] rounded-md border border-border/60 bg-card p-1.5 cursor-pointer hover:border-primary/50 transition-all text-start relative overflow-hidden"
                  >
                    <div
                      class="absolute inset-s-0 top-0 bottom-0 w-1"
                      style="background-color: var(--subject-color);"
                    ></div>
                    <div class="ps-1.5 flex flex-col justify-between h-full">
                      <span class="font-bold text-[11px] text-foreground truncate" title={getSubjectName(l.subject_id)}>
                        {getSubjectName(l.subject_id)}
                      </span>
                      <div class="flex items-center justify-between text-[9px] text-muted-foreground mt-1">
                        <span class="truncate">{getTeacherName(l.teacher_id)}</span>
                        {#if l.room_id}
                          <span class="text-primary font-medium">{getRoomName(l.room_id)}</span>
                        {/if}
                      </div>
                    </div>
                  </div>
                {:else}
                  <div class="h-full min-h-[48px]"></div>
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
