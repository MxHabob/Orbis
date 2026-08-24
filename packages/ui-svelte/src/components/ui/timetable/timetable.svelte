<script lang="ts">
import { fade } from "svelte/transition";
import { Plus, Clock } from "@lucide/svelte";
import TimetableCard from "./timetable-card.svelte";

let {
  timeSlots = [],
  weekdays = [],
  lessons = [],
  draftLessons = [],
  previewDraft = false,
  viewMode = "class",
  locale = "ar",
  getSubjectName,
  getTeacherName,
  getClassName,
  getSubjectColor,
  class: className = "",
  onCellClick,
  onDeleteLesson,
  onMoveLesson,
}: {
  timeSlots: any[];
  weekdays: any[];
  lessons: any[];
  draftLessons: any[];
  previewDraft: boolean;
  viewMode: "class" | "teacher";
  locale: string;
  getSubjectName: (id: string) => string;
  getTeacherName: (id: string) => string;
  getClassName: (id: string) => string;
  getSubjectColor: (id: string) => string;
  class?: string;
  onCellClick: (day: number, slotId: string) => void;
  onDeleteLesson: (id: string) => void;
  onMoveLesson: (lesson: any, targetDay: number, targetSlotId: string) => void;
} = $props();

// Retrieve lesson (real or draft) inside a grid cell
function getLessonInCell(day: number, slotId: string) {
  if (previewDraft) {
    const draft = draftLessons.find(
      (l: any) =>
        Number(l.day_of_week) === day &&
        String(l.time_slot_id) === String(slotId),
    );
    if (draft) return draft;
  }
  return lessons.find(
    (l: any) =>
      Number(l.day_of_week) === day &&
      String(l.time_slot_id) === String(slotId),
  );
}

// Drag and Drop handlers
let draggedLesson = $state<any>(null);
let dragOverCell = $state<{ day: number; slotId: string } | null>(null);

function handleDragStart(e: DragEvent, lesson: any) {
  if (lesson.isDraft) {
    e.preventDefault();
    return;
  }
  draggedLesson = lesson;
  if (e.dataTransfer) {
    e.dataTransfer.effectAllowed = "move";
    e.dataTransfer.setData("text/plain", lesson.id);
  }
}

function handleDragOver(e: DragEvent, day: number, slotId: string) {
  e.preventDefault();
  if (draggedLesson) {
    dragOverCell = { day, slotId };
  }
}

function handleDragLeave() {
  dragOverCell = null;
}

function handleDrop(e: DragEvent, targetDay: number, targetSlotId: string) {
  e.preventDefault();
  dragOverCell = null;
  if (!draggedLesson) return;
  const lesson = draggedLesson;
  draggedLesson = null;

  if (
    Number(lesson.day_of_week) === targetDay &&
    String(lesson.time_slot_id) === String(targetSlotId)
  ) {
    return;
  }

  onMoveLesson(lesson, targetDay, targetSlotId);
}
</script>

<!-- Reusable responsive timetable board -->
<div class="flex-1 min-h-0 min-w-0 w-full h-full border border-border bg-card rounded-xl flex flex-col overflow-auto shadow-sm select-none relative scrollbar-thin {className}">
  
  <div 
    class="grid gap-px text-xs w-full min-w-max bg-border flex-1"
    style="grid-template-columns: minmax(108px, 0.9fr) repeat({weekdays.length}, minmax(112px, 1fr));"
    in:fade={{ duration: 150 }}
  >
    <!-- 1. Header Row (Sticky Top) -->
    <div class="bg-card text-muted-foreground p-3 md:p-4 font-bold text-center flex items-center justify-center border-b border-border uppercase text-[10px] tracking-wider sticky top-0 start-0 z-30 shadow-xs">
      {locale === "ar" ? "الحصة / الوقت" : "Period / Time"}
    </div>
    {#each weekdays as day (day.value)}
      <div class="bg-card text-muted-foreground p-3 md:p-4 font-bold text-center flex items-center justify-center border-b border-border uppercase text-[10px] tracking-wider sticky top-0 z-20 shadow-xs">
        {day.label}
      </div>
    {/each}

    <!-- 2. Rows (Periods & Breaks) -->
    {#each timeSlots as slot (slot.id)}
      {@const isBreak = slot.is_break === 1 || slot.is_break === true}

      <!-- Row Period Info Column (Sticky Start for Horizontal Scroll) -->
      <div class="bg-muted/30 border-border p-2.5 font-semibold text-center flex flex-col justify-center items-center gap-1 min-h-[96px] h-full border-b sticky start-0 z-10 backdrop-blur-xs">
        <span class="text-[11px] tracking-wide font-extrabold text-foreground truncate max-w-[125px]">
          {locale === "ar" ? (slot.name_ar || slot.name) : (slot.name || slot.name_ar)}
        </span>
        <span class="bg-card border border-border text-muted-foreground text-[9px] font-mono font-bold px-1.5 py-0.5 rounded flex items-center gap-1 shrink-0">
          <Clock size={10} class="opacity-70" />
          <span>{slot.start_time} - {slot.end_time}</span>
        </span>
      </div>

      <!-- Rows Data Columns -->
      {#if isBreak}
        <!-- Break row spans across all weekdays -->
        <div 
          class="bg-muted/50 text-muted-foreground border-b border-border font-extrabold uppercase text-[10px] tracking-widest flex items-center justify-center gap-2 min-h-[96px] h-full"
          style="grid-column: span {weekdays.length};"
        >
          <span>{locale === "ar" ? "فترة راحة" : "BREAK INTERVAL"}</span>
          <span class="text-[9px] font-mono font-bold opacity-80">({slot.start_time} - {slot.end_time})</span>
        </div>
      {:else}
        {#each weekdays as day (day.value)}
          {@const lesson = getLessonInCell(day.value, slot.id)}
          {@const isDragOver = dragOverCell?.day === day.value && dragOverCell?.slotId === slot.id}
          
          <!-- Cell Container -->
          <!-- svelte-ignore a11y_no_static_element_interactions -->
          <div 
            class="bg-card border-border p-2 text-center align-middle min-h-[96px] h-full relative flex items-center justify-center transition-all duration-200 border-b hover:bg-muted/40
              {isDragOver ? 'bg-primary/[0.08] ring-2 ring-primary/40 ring-inset border-dashed' : ''}"
            ondragover={(e) => handleDragOver(e, day.value, slot.id)}
            ondragleave={handleDragLeave}
            ondrop={(e) => handleDrop(e, day.value, slot.id)}
          >
            {#if lesson}
              <TimetableCard 
                {lesson} 
                {viewMode} 
                {locale} 
                {getSubjectName} 
                {getTeacherName} 
                {getClassName} 
                subjectColor={getSubjectColor(lesson.subject_id)}
                onDelete={onDeleteLesson}
                ondragstart={(e) => handleDragStart(e, lesson)}
              />
            {:else}
              <!-- Empty Cell Add Trigger (Class Mode only) -->
              <button 
                type="button"
                onclick={() => onCellClick(day.value, slot.id)}
                class="w-full h-full border border-dashed border-transparent hover:border-primary/30 hover:bg-primary/[0.03] rounded-xl flex items-center justify-center text-muted-foreground/0 hover:text-primary transition-all duration-200 cursor-pointer p-2 min-h-[70px]"
                disabled={viewMode !== "class"}
                title={locale === "ar" ? "إضافة حصة جديدة" : "Schedule new lesson"}
              >
                <Plus size={16} class="transition-transform duration-200 hover:scale-110" />
              </button>
            {/if}
          </div>
        {/each}
      {/if}
    {/each}
  </div>
</div>
