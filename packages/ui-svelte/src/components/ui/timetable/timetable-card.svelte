<script lang="ts">
import { X } from "lucide-svelte";
import { fade } from "svelte/transition";

let {
  lesson,
  viewMode = "class",
  locale = "ar",
  getSubjectName,
  getTeacherName,
  getClassName,
  subjectColor = "#a855f7", // Purple is default to match Tech Conference
  onDelete,
  ondragstart,
}: {
  lesson: any;
  viewMode: "class" | "teacher";
  locale: string;
  getSubjectName: (id: string) => string;
  getTeacherName: (id: string) => string;
  getClassName: (id: string) => string;
  subjectColor?: string;
  onDelete: (id: string) => void;
  ondragstart?: (e: DragEvent) => void;
} = $props();

const isDraft = $derived(!!lesson.isDraft);
const subjectName = $derived(getSubjectName(lesson.subject_id));

// Format times nicely
const timeStr = $derived(
  lesson.time_slot_start && lesson.time_slot_end
    ? `${lesson.time_slot_start} - ${lesson.time_slot_end}`
    : "",
);
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div 
  draggable={!isDraft}
  {ondragstart}
  style="--subject-color: {subjectColor};"
  class="relative overflow-hidden w-full h-full p-2.5 flex flex-col justify-between items-start text-start select-none group transition-all duration-150 rounded-lg border
    {isDraft ? 'timetable-card-draft' : 'timetable-card-tech cursor-grab active:cursor-grabbing'}"
  in:fade={{ duration: 150 }}
>
  <!-- Delete Quick-Action Button -->
  {#if !isDraft}
    <button 
      type="button"
      onclick={(e) => { e.stopPropagation(); onDelete(lesson.id); }}
      class="absolute top-1.5 end-1.5 opacity-0 group-hover:opacity-100 h-4.5 w-4.5 text-[var(--text-color)] hover:text-red-600 hover:bg-red-50 dark:hover:bg-red-950/30 rounded flex items-center justify-center transition-all duration-150 cursor-pointer border border-transparent"
      title={locale === "ar" ? "حذف الحصة" : "Delete lesson"}
    >
      <X size={10} />
    </button>
  {/if}

  <!-- Tech-Conference layout: Header row with small colored dot and title -->
  <div class="flex flex-col gap-1 w-full pr-3">
    <div class="flex items-center gap-1.5 w-full">
      <!-- Colored circle dot on the left -->
      <div class="w-1.5 h-1.5 rounded-full bg-[var(--subject-color)] shrink-0"></div>
      
      <!-- Event Name -->
      <span class="font-bold text-[11px] md:text-[11.5px] leading-tight text-[var(--text-color)] tracking-wide line-clamp-1">
        {subjectName}
      </span>
    </div>
    
    <!-- Time range -->
    {#if timeStr}
      <span class="text-[9px] md:text-[9.5px] font-semibold text-[var(--text-color)] opacity-85 leading-none font-mono pl-3">
        {timeStr}
      </span>
    {/if}

    <!-- Auxiliary Info (Teacher/Class) -->
    <span class="text-[8px] md:text-[8.5px] text-[var(--text-color)] opacity-60 line-clamp-1 mt-0.5 pl-3">
      {viewMode === "class" ? getTeacherName(lesson.teacher_id) : getClassName(lesson.class_id)}
      {#if lesson.room_name || lesson.room}
        • {lesson.room_name || lesson.room}
      {/if}
    </span>
  </div>
</div>

<style>
  /* Styled exactly to match the Tech Conference pill layout */
  .timetable-card-tech {
    background-color: var(--bg-color);
    border-color: var(--border-color);
    --bg-color: color-mix(in srgb, var(--subject-color) 10%, #ffffff 90%);
    --border-color: color-mix(in srgb, var(--subject-color) 12%, #ffffff 88%);
    --text-color: color-mix(in srgb, var(--subject-color) 85%, #000000 15%);
    box-shadow: 0 1px 2px 0 rgba(0, 0, 0, 0.02);
  }

  .timetable-card-tech:hover {
    --bg-color: color-mix(in srgb, var(--subject-color) 14%, #ffffff 86%);
    --border-color: color-mix(in srgb, var(--subject-color) 20%, #ffffff 80%);
    box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.04);
    transform: translateY(-0.5px);
  }

  /* Support dark-mode */
  :global(.dark) .timetable-card-tech {
    --bg-color: color-mix(in srgb, var(--subject-color) 14%, #1e1e2e 86%);
    --border-color: color-mix(in srgb, var(--subject-color) 25%, #1e1e2e 75%);
    --text-color: color-mix(in srgb, var(--subject-color) 90%, #ffffff 10%);
    box-shadow: none;
  }

  :global(.dark) .timetable-card-tech:hover {
    --bg-color: color-mix(in srgb, var(--subject-color) 20%, #1e1e2e 80%);
  }

  /* Draft styling */
  .timetable-card-draft {
    background-color: var(--bg-color);
    border-color: var(--border-color);
    border-style: dashed;
    opacity: 0.8;
    --bg-color: color-mix(in srgb, var(--subject-color) 5%, #ffffff 95%);
    --border-color: color-mix(in srgb, var(--subject-color) 25%, #ffffff 75%);
    --text-color: color-mix(in srgb, var(--subject-color) 70%, #000000 30%);
    animation: draft-pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;
  }

  @keyframes draft-pulse {
    0%, 100% { opacity: 0.8; }
    50% { opacity: 0.4; }
  }

  :global(.dark) .timetable-card-draft {
    --bg-color: color-mix(in srgb, var(--subject-color) 8%, #1e1e2e 92%);
    --border-color: color-mix(in srgb, var(--subject-color) 35%, #1e1e2e 65%);
    --text-color: color-mix(in srgb, var(--subject-color) 85%, #ffffff 15%);
  }
</style>
