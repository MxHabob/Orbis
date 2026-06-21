<script lang="ts">
  import { fade } from "svelte/transition";

  let {
    lesson,
    viewMode = "class",
    locale = "ar",
    getSubjectName,
    getTeacherName,
    getClassName,
    subjectColor = "#a855f7",
    ondragstart,
  }: {
    lesson: any;
    viewMode: "class" | "teacher";
    locale: string;
    getSubjectName: (id: string) => string;
    getTeacherName: (id: string) => string;
    getClassName: (id: string) => string;
    subjectColor?: string;
    ondragstart?: (e: DragEvent) => void;
  } = $props();

  const isDraft = $derived(!!lesson.isDraft);
  const subjectName = $derived(getSubjectName(lesson.subject_id));
  
  // Format times nicely
  const timeStr = $derived(lesson.time_slot_start && lesson.time_slot_end 
    ? `${lesson.time_slot_start} - ${lesson.time_slot_end}` 
    : "");
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div 
  draggable={!isDraft}
  {ondragstart}
  style="--subject-color: {subjectColor};"
  class="relative overflow-hidden w-full h-full p-2.5 flex flex-row items-stretch justify-between gap-2.5 select-none transition-all duration-150 rounded-lg border
    {isDraft ? 'timetable-card-draft' : 'timetable-card-tech-premium cursor-grab active:cursor-grabbing'}"
  in:fade={{ duration: 150 }}
>
  
  <!-- Flex layout places children according to language direction -->
  {#if locale === "ar"}
    <!-- RTL Layout: Text on Right, Thin Stripe on Left -->
    <div class="flex-1 flex flex-col justify-center items-start text-start gap-0.5 min-w-0">
      <span class="font-bold text-[11px] md:text-[11.5px] leading-tight text-[var(--text-color)] tracking-wide line-clamp-2">
        {subjectName}
      </span>
      {#if timeStr}
        <span class="text-[9px] md:text-[9.5px] font-semibold text-[var(--text-color)] opacity-80 leading-none font-mono">
          {timeStr}
        </span>
      {/if}
      <span class="text-[8px] md:text-[8.5px] text-[var(--text-color)] opacity-60 line-clamp-1 mt-0.5">
        {viewMode === "class" ? getTeacherName(lesson.teacher_id) : getClassName(lesson.class_id)}
        {#if lesson.room_name || lesson.room}
          • {lesson.room_name || lesson.room}
        {/if}
      </span>
    </div>
    <div class="timetable-color-block"></div>
  {:else}
    <!-- LTR Layout: Thin Stripe on Left, Text on Right -->
    <div class="timetable-color-block"></div>
    <div class="flex-1 flex flex-col justify-center items-start text-start gap-0.5 min-w-0">
      <span class="font-bold text-[11px] md:text-[11.5px] leading-tight text-[var(--text-color)] tracking-wide line-clamp-2">
        {subjectName}
      </span>
      {#if timeStr}
        <span class="text-[9px] md:text-[9.5px] font-semibold text-[var(--text-color)] opacity-80 leading-none font-mono">
          {timeStr}
        </span>
      {/if}
      <span class="text-[8px] md:text-[8.5px] text-[var(--text-color)] opacity-60 line-clamp-1 mt-0.5">
        {viewMode === "class" ? getTeacherName(lesson.teacher_id) : getClassName(lesson.class_id)}
        {#if lesson.room_name || lesson.room}
          • {lesson.room_name || lesson.room}
        {/if}
      </span>
    </div>
  {/if}

</div>

<style>
  /* Premium Flat Card styling */
  .timetable-card-tech-premium {
    background-color: #ffffff;
    border-color: #e5e7eb;
    --text-color: #1f2937;
    box-shadow: none; /* No shadow/glow */
  }

  .timetable-card-tech-premium:hover {
    background-color: #f9fafb;
    border-color: #d1d5db;
    box-shadow: none;
  }

  /* Support dark-mode correctly */
  :global(.dark) .timetable-card-tech-premium {
    background-color: #18181b;
    border-color: #27272a;
    --text-color: #f4f4f5;
    box-shadow: none;
  }

  :global(.dark) .timetable-card-tech-premium:hover {
    background-color: #1c1c1f;
    border-color: #3f3f46;
    box-shadow: none;
  }

  /* Flat Color Block (the thin vertical line) - static, clean, no shadows/glows */
  .timetable-color-block {
    width: 4px;
    border-radius: 9999px;
    flex-shrink: 0;
    background-color: var(--subject-color);
  }

  /* Draft styling */
  .timetable-card-draft {
    background-color: #f9fafb;
    border-color: #d1d5db;
    border-style: dashed;
    opacity: 0.8;
    --text-color: #4b5563;
    animation: draft-pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;
  }

  .timetable-card-draft .timetable-color-block {
    opacity: 0.5;
    background-color: var(--subject-color);
    box-shadow: none;
    border: 1px dashed var(--subject-color);
  }

  @keyframes draft-pulse {
    0%, 100% { opacity: 0.8; }
    50% { opacity: 0.4; }
  }

  :global(.dark) .timetable-card-draft {
    background-color: #18181b;
    border-color: #3f3f46;
    --text-color: #a1a1aa;
  }
</style>
