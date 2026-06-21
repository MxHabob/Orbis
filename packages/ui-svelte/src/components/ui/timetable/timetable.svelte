<script lang="ts">
  import { Clock, Plus } from "lucide-svelte";
  import { fade } from "svelte/transition";
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
    onCellClick: (day: number, slotId: string) => void;
    onDeleteLesson: (id: string) => void;
    onMoveLesson: (lesson: any, targetDay: number, targetSlotId: string) => void;
  } = $props();

  // Local Drag & Drop State
  let draggedLesson = $state<any>(null);
  let dragOverCell = $state<{ day: number; slotId: string } | null>(null);

  // Retrieve lesson (real or draft) inside a grid cell
  function getLessonInCell(day: number, slotId: string) {
    if (previewDraft) {
      const draft = draftLessons.find(
        (l: any) => Number(l.day_of_week) === day && String(l.time_slot_id) === String(slotId)
      );
      if (draft) return draft;
    }
    return lessons.find(
      (l: any) => Number(l.day_of_week) === day && String(l.time_slot_id) === String(slotId)
    );
  }

  // Drag and Drop handlers
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

    // Do nothing if dropped on the same cell
    if (Number(lesson.day_of_week) === targetDay && String(lesson.time_slot_id) === String(targetSlotId)) {
      return;
    }

    onMoveLesson(lesson, targetDay, targetSlotId);
  }
</script>

<!-- CSS Grid-based Timetable Component -->
<div class="flex-1 w-full overflow-x-auto rounded-xl border border-border/80 bg-card/25 shadow-sm scrollbar-thin">
  <div 
    class="min-w-[850px] grid bg-border/40 gap-px select-none text-xs"
    style="grid-template-columns: 145px repeat(5, 1fr);"
    in:fade={{ duration: 150 }}
  >
    <!-- 1. Header Row -->
    <div class="bg-card/90 p-3.5 font-bold text-muted-foreground uppercase text-[9px] tracking-wider text-center border-b border-border/60 flex items-center justify-center">
      {locale === "ar" ? "الحصة / الوقت" : "Period / Time"}
    </div>
    {#each weekdays as day (day.value)}
      <div class="bg-card/90 p-3.5 font-bold text-muted-foreground uppercase text-[9px] tracking-wider text-center border-b border-border/60 flex items-center justify-center">
        {day.label}
      </div>
    {/each}

    <!-- 2. Rows (Periods & Breaks) -->
    {#each timeSlots as slot (slot.id)}
      {@const isBreak = slot.is_break === 1 || slot.is_break === true}

      <!-- Row Period Info Column -->
      <div class="bg-card/40 p-3 font-medium text-center flex flex-col justify-center items-center gap-1.5 h-24 border-b border-border/40">
        <span class="font-bold text-foreground tracking-wide">
          {locale === "ar" ? (slot.name_ar || slot.name) : (slot.name || slot.name_ar)}
        </span>
        <span class="text-[9px] text-muted-foreground font-mono bg-background/80 px-1.5 py-0.5 rounded border border-border/80 flex items-center gap-1">
          <Clock size={8} class="text-muted-foreground/80" />
          <span>{slot.start_time} - {slot.end_time}</span>
        </span>
      </div>

      <!-- Rows Data Columns -->
      {#if isBreak}
        <!-- Break row spans across all remaining 5 weekday columns -->
        <div class="col-span-5 bg-muted/40 text-muted-foreground border-b border-border/40 font-bold uppercase text-[9px] tracking-widest flex items-center justify-center gap-2 h-24 select-none">
          <span>{locale === "ar" ? "فترة راحة" : "BREAK INTERVAL"}</span>
          <span class="text-[8px] font-mono font-medium opacity-75">({slot.start_time} - {slot.end_time})</span>
        </div>
      {:else}
        {#each weekdays as day (day.value)}
          {@const lesson = getLessonInCell(day.value, slot.id)}
          {@const isDragOver = dragOverCell?.day === day.value && dragOverCell?.slotId === slot.id}
          
          <!-- Cell Container -->
          <!-- svelte-ignore a11y_no_static_element_interactions -->
          <div 
            class="bg-card/15 p-2 text-center align-middle h-24 relative flex items-center justify-center transition-all duration-200 border-b border-border/40
              {isDragOver ? 'bg-primary/[0.08] ring-2 ring-primary/40 ring-inset border-dashed' : 'hover:bg-card/5'}"
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
                onDelete={onDeleteLesson}
                ondragstart={(e) => handleDragStart(e, lesson)}
              />
            {:else}
              <!-- Empty Cell Add Trigger (Class Mode only) -->
              <button 
                type="button"
                onclick={() => onCellClick(day.value, slot.id)}
                class="w-full h-full border border-dashed border-transparent hover:border-primary/30 hover:bg-primary/[0.03] rounded-lg flex items-center justify-center text-muted-foreground/0 hover:text-primary transition-all duration-200 cursor-pointer"
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
