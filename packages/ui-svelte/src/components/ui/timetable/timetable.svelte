<script lang="ts">
import { fade } from "svelte/transition";
import { Plus, Clock } from "lucide-svelte";
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

<!-- Period-Based School Timetable Component with Premium Aesthetics -->
<div class="timetable-container flex-1 w-full border rounded-2xl flex flex-col overflow-hidden shadow-sm select-none">
  
  <div 
    class="grid gap-px text-xs w-full bg-border-line"
    style="grid-template-columns: 145px repeat({weekdays.length}, 1fr);"
    in:fade={{ duration: 150 }}
  >
    <!-- 1. Header Row -->
    <div class="timetable-header p-4 font-bold text-center flex items-center justify-center border-b uppercase text-[10px] tracking-wider">
      {locale === "ar" ? "الحصة / الوقت" : "Period / Time"}
    </div>
    {#each weekdays as day (day.value)}
      <div class="timetable-header p-4 font-bold text-center flex items-center justify-center border-b uppercase text-[10px] tracking-wider">
        {day.label}
      </div>
    {/each}

    <!-- 2. Rows (Periods & Breaks) -->
    {#each timeSlots as slot (slot.id)}
      {@const isBreak = slot.is_break === 1 || slot.is_break === true}

      <!-- Row Period Info Column -->
      <div class="timetable-period-info p-3 font-semibold text-center flex flex-col justify-center items-center gap-1.5 h-[100px] border-b">
        <span class="text-[11px] tracking-wide font-extrabold text-foreground">
          {locale === "ar" ? (slot.name_ar || slot.name) : (slot.name || slot.name_ar)}
        </span>
        <span class="timetable-time-badge text-[9px] font-mono font-bold px-2 py-0.5 rounded border flex items-center gap-1">
          <Clock size={10} class="opacity-70" />
          <span>{slot.start_time} - {slot.end_time}</span>
        </span>
      </div>

      <!-- Rows Data Columns -->
      {#if isBreak}
        <!-- Break row spans across all weekdays -->
        <div 
          class="timetable-break-row bg-muted/40 text-muted-foreground border-b font-extrabold uppercase text-[10px] tracking-widest flex items-center justify-center gap-2 h-[100px]"
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
            class="timetable-cell p-2.5 text-center align-middle h-[100px] relative flex items-center justify-center transition-all duration-200 border-b
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
                class="timetable-add-btn w-full h-full border border-dashed border-transparent hover:border-primary/30 hover:bg-primary/[0.03] rounded-xl flex items-center justify-center text-muted-foreground/0 hover:text-primary transition-all duration-200 cursor-pointer"
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

<style>
  /* 1. Container styling */
  .timetable-container {
    background-color: #ffffff;
    border-color: #e5e7eb; /* border-gray-200 */
  }
  :global(.dark) .timetable-container {
    background-color: #121214 !important; /* dark bg zinc-950 */
    border-color: #27272a !important;
  }

  /* Grid line border colors */
  .bg-border-line {
    background-color: #e5e7eb;
  }
  :global(.dark) .bg-border-line {
    background-color: #27272a !important;
  }

  /* 2. Headers styling */
  .timetable-header {
    border-color: #e5e7eb;
    color: #6b7280; /* text-gray-500 */
    background-color: #ffffff;
  }
  :global(.dark) .timetable-header {
    border-color: #27272a;
    color: #a1a1aa; /* text-zinc-400 */
    background-color: #121214 !important;
  }

  /* 3. Period info column scale styling */
  .timetable-period-info {
    border-color: #e5e7eb;
    background-color: #fcfcfd;
  }
  :global(.dark) .timetable-period-info {
    border-color: #27272a;
    background-color: #151518 !important;
  }

  /* Period time range badges */
  .timetable-time-badge {
    border-color: #e5e7eb;
    background-color: #ffffff;
    color: #4b5563; /* gray-600 */
  }
  :global(.dark) .timetable-time-badge {
    border-color: #27272a;
    background-color: #121214 !important;
    color: #a1a1aa !important;
  }

  /* 4. Cell styling */
  .timetable-cell {
    border-color: #e5e7eb;
    background-color: #ffffff;
  }
  .timetable-cell:hover {
    background-color: #fafafa;
  }
  :global(.dark) .timetable-cell {
    border-color: #27272a;
    background-color: #121214 !important;
  }
  :global(.dark) .timetable-cell:hover {
    background-color: #161619 !important;
  }

  /* 5. Break Row styling */
  .timetable-break-row {
    border-color: #e5e7eb;
    background-color: #f9fafb;
    color: #6b7280;
  }
  :global(.dark) .timetable-break-row {
    border-color: #27272a;
    background-color: #18181b !important;
    color: #a1a1aa !important;
  }
</style>
